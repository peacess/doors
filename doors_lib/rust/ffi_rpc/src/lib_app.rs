use tokio::runtime::Handle;

pub struct LibApp {
    handle: Handle,
    shutdown_sender: tokio::sync::broadcast::Sender<()>,
    // shutdown_receiver: tokio::sync::broadcast::Receiver<()>,
}

static LIB_APP: std::sync::OnceLock<LibApp> = std::sync::OnceLock::new();

impl LibApp {
    pub fn make_app() -> Result<Self, anyhow::Error> {
        let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build()?;
        let handle = rt.handle().clone();
        let (shutdown_sender, _shutdown_receiver) = tokio::sync::broadcast::channel::<()>(1);
        Ok(Self {
            handle,
            shutdown_sender,
            // shutdown_receiver,
        })
    }

    pub fn init() -> Result<(), anyhow::Error> {
        let result = std::panic::catch_unwind(|| {
            LIB_APP.get_or_init(|| {
                match Self::make_app() {
                    Ok(app) => app,
                    Err(e) => {
                        log::error!("Failed to initialize Door's library: {}", e);
                        panic!("Failed to initialize Door's library: {}", e);
                    }
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
}
