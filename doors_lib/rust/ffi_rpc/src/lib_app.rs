use std::{sync::Arc, time::Duration};

use tokio::runtime::Handle;
use tokio_util::sync::CancellationToken;

use crate::discover::MulticastService;

pub struct LibApp {
    handle: Handle,
    cancel_token: CancellationToken,
    running_handle: Arc<std::sync::atomic::AtomicBool>,
    multicast_service: Arc<MulticastService>,
    call_back: Option<crate::ffi::CallBack>,
}

static LIB_APP: std::sync::OnceLock<LibApp> = std::sync::OnceLock::new();

impl LibApp {
    fn make_app(call_back: crate::ffi::CallBack) -> Result<Self, anyhow::Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build()?;
        let cancel_token = CancellationToken::new();
        let handle = runtime.handle().clone();
        let running_handle = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let multicast_service = MulticastService::new()?;

        let running_handle_thread = running_handle.clone();
        let token_thread = cancel_token.clone();
        {
            let local_thread = std::thread::current();
            std::thread::spawn(move || {
                let _en = runtime.enter();
                runtime.block_on(async {
                    running_handle_thread.store(true, std::sync::atomic::Ordering::Relaxed);
                    local_thread.unpark();
                    token_thread.cancelled().await;
                    running_handle_thread.store(false, std::sync::atomic::Ordering::Relaxed);
                });
            });
            // make sure the thead to started running
            std::thread::park_timeout(Duration::from_secs(1));
        }

        Ok(Self {
            handle,
            cancel_token,
            running_handle,
            multicast_service,
            call_back: Some(call_back),
        })
    }

    pub fn init(call_back: crate::ffi::CallBack) -> Result<(), anyhow::Error> {
        let result = std::panic::catch_unwind(|| {
            let _ = LIB_APP.get_or_init(|| match Self::make_app(call_back) {
                Ok(app) => {
                    app.multicast_service.clone().init(app.cancel_token.clone());
                    app
                }
                Err(e) => {
                    log::error!("Failed to initialize Door's library: {}", e);
                    panic!("Failed to initialize Door's library: {}", e);
                }
            });
        });

        result.map_err(|e| {
            let message = {
                if let Some(err_str) = e.downcast_ref::<&str>() {
                    err_str.to_string()
                } else if let Some(err_str) = e.downcast_ref::<String>() {
                    err_str.to_string()
                } else {
                    "Unknown panic".into()
                }
            };
            anyhow::anyhow!("LibApp panicked: {}", message)
        })
    }

    pub fn uninit() {
        if let Some(app) = LIB_APP.get()
            && app.running_handle.load(std::sync::atomic::Ordering::Relaxed)
        {
            if let Err(e) = app.multicast_service.uninit() {
                log::error!("Failed to uninit Door's library: {}", e);
            }
            app.cancel_token.cancel();
        }
    }

    pub fn app() -> Option<&'static Self> {
        LIB_APP.get()
    }

    pub fn handle(&self) -> Option<&Handle> {
        if self.running_handle.load(std::sync::atomic::Ordering::Relaxed) {
            Some(&self.handle)
        } else {
            None
        }
    }

    pub fn set_callback(&mut self, callback: Option<crate::ffi::CallBack>) {
        self.call_back = callback;
    }

    pub fn get_callback(&self) -> Option<&crate::ffi::CallBack> {
        self.call_back.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ffi::FfiBytes, lib_app::LibApp};

    #[test]
    fn it_works() {
        extern "C" fn empty_call(_t: FfiBytes) {}
        LibApp::init(empty_call).unwrap();
        if let Some(app) = LibApp::app() {
            app.handle.spawn(async {});
        }
        LibApp::uninit();
    }
}
