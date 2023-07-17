use std::sync::Arc;

use flatbuffers::Follow;

use crate::idl;
use crate::server::{Handles, Shared};

pub struct FrameHandles {
    shared: Arc<Shared>,
    handles: Arc<Handles>,
}

impl FrameHandles {
    pub fn new(shared: Arc<Shared>) -> Self {
        FrameHandles {
            shared,
            handles: Arc::new(Handles::new()),
        }
    }

    // 由于函数会使用多线程等，所以参数不能使用引用
    pub fn handle(&self, bytes: Vec<u8>) {
        let temp = self.handles.clone();
        tokio::spawn(async move {
            FrameHandles::handle_frame(bytes, temp);
        });
    }

    fn handle_frame(bytes: Vec<u8>, handles: Arc<Handles>) {
        let header = unsafe { idl::Header::follow(&bytes, 0) };
        let handle = handles.get(header.type_());
        if handle.is_none() {
            log::warn!("can not find the handle_impl for {:?}", header.type_());
            return;
        }
        let handle = handle.expect("");
        handle.handle(bytes);
    }
}