use std::{sync::Arc, time::Duration};

use flatbuffers::EndianScalar;
use idl::{Frame, base_generated::base::HeaderType};
use tokio::runtime::Handle;
use tokio_util::sync::CancellationToken;

use crate::{
    discover::MulticastService,
    ffi_impl::{CallBack, FfiBytes},
};

pub struct LibApp {
    runtime_handle: Handle,
    cancel_token: CancellationToken,
    running_handle: Arc<std::sync::atomic::AtomicBool>,
    multicast_service: Arc<MulticastService>,
    call_back: Arc<Option<CallBack>>,
}

pub static LIB_APP: std::sync::OnceLock<LibApp> = std::sync::OnceLock::new();

impl LibApp {
    fn make_app(call_back: Arc<Option<CallBack>>) -> Result<Self, anyhow::Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build()?;
        let cancel_token = CancellationToken::new();
        let handle = runtime.handle().clone();
        let running_handle = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let running_handle_thread = running_handle.clone();
        let token_thread = cancel_token.clone();
        {
            log::info!("before run tokio runtime");
            let local_thread = std::thread::current();
            std::thread::spawn(move || {
                let _en = runtime.enter();
                runtime.block_on(async {
                    log::info!("running tokio runtime");
                    running_handle_thread.store(true, std::sync::atomic::Ordering::Relaxed);
                    local_thread.unpark();
                    token_thread.cancelled().await;
                    running_handle_thread.store(false, std::sync::atomic::Ordering::Relaxed);
                    log::info!("exit tokio runtime");
                });
            });
            // make sure the thead to started running
            std::thread::park_timeout(Duration::from_secs(1));
            log::info!("after run tokio runtime");
        }

        let multicast_service = MulticastService::new(handle.clone(), call_back.clone())?;

        Ok(Self {
            runtime_handle: handle,
            cancel_token,
            running_handle,
            multicast_service,
            call_back,
        })
    }

    pub fn init(call_back: Arc<Option<CallBack>>) -> Result<(), anyhow::Error> {
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

    pub fn call(&self, bytes: &[u8]) -> Result<FfiBytes, anyhow::Error> {
        let frame = flatbuffers::root::<Frame>(bytes)?;
        if let Some(header) = frame.header() {
            match HeaderType::from_little_endian(header.header_type()) {
                HeaderType::none => {
                    let err = anyhow::anyhow!("the header type is None");
                    log::error!("{}", err);
                    Err(err)
                }
                HeaderType::net_discovery => self.multicast_service.call_net_disovery(header, bytes),
                HeaderType::chat => {
                    unimplemented!()
                }
                HeaderType::ffi_rpc => {
                    unimplemented!()
                }
                HeaderType(a) => {
                    let err = anyhow::anyhow!("the header type {} is out of , ", a);
                    log::error!("{}", err);
                    Err(err)
                }
            }
        } else {
            let err = anyhow::anyhow!("the header of frame is None");
            log::error!("{}", err);
            Err(err)
        }
    }

    pub fn app() -> Option<&'static Self> {
        LIB_APP.get()
    }

    pub fn runtime_handle(&self) -> Option<&Handle> {
        if self.running_handle.load(std::sync::atomic::Ordering::Relaxed) {
            Some(&self.runtime_handle)
        } else {
            None
        }
    }

    // pub fn set_callback(&mut self, callback: Option<crate::ffi_impl::CallBack>) {
    //     self.call_back = callback;
    // }

    pub fn get_callback(&self) -> &Option<CallBack> {
        self.call_back.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{ffi_impl::FfiBytes, lib_app::LibApp};

    #[test]
    fn it_works() {
        extern "C" fn empty_call(_t: FfiBytes) {}
        LibApp::init(Arc::new(None)).unwrap();
        if let Some(app) = LibApp::app() {
            app.runtime_handle.spawn(async {});
        }
        LibApp::uninit();
    }
}
