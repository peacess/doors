use std::sync::Arc;

use tokio::runtime::Handle;

pub struct LibApp {
    handle: Handle,
    shutdown_sender: tokio::sync::broadcast::Sender<()>,
    running_handle: Arc<std::sync::atomic::AtomicBool>,
}

static LIB_APP: std::sync::OnceLock<LibApp> = std::sync::OnceLock::new();

impl LibApp {
    fn make_app() -> Result<Self, anyhow::Error> {
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build()?;
        let (shutdown_sender, mut shutdown_receiver) = tokio::sync::broadcast::channel::<()>(1);
        let handle = runtime.handle().clone();
        let running_handle = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let running_handle_thread = running_handle.clone();
        std::thread::spawn(move || {
            let _en = runtime.enter();
            runtime.block_on(async {
                if let Err(e) = shutdown_receiver.recv().await {
                    log::error!("{}", e);
                }
                running_handle_thread.store(false, std::sync::atomic::Ordering::Relaxed);
            });
        });
        Ok(Self {
            handle,
            shutdown_sender,
            running_handle,
        })
    }

    pub fn init() -> Result<(), anyhow::Error> {
        let result = std::panic::catch_unwind(|| {
            let _ = LIB_APP.get_or_init(|| match Self::make_app() {
                Ok(app) => app,
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
            && let Err(e) = app.shutdown_sender.send(())
        {
            log::error!("{}", e);
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
}

#[cfg(test)]
mod tests {
    use crate::lib_app::LibApp;

    #[test]
    fn it_works() {
        LibApp::init().unwrap();
        if let Some(app) = LibApp::app() {
            app.handle.spawn(async {});
        }
        LibApp::uninit();
    }
}
