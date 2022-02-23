use std::sync::Arc;

use flatbuffers::Follow;

use crate::idl;
use crate::idl::FrameType;
use crate::shared::Shared;

pub struct FrameHandle {
    shared: Arc<Shared>,
}

impl FrameHandle {
    pub fn new(shared: Arc<Shared>) -> Self {
        FrameHandle {
            shared,
        }
    }

    // 由于函数会使用多线程等，所以参数不能使用引用
    pub fn handle(&self, bytes: Vec<u8>) {
        tokio::spawn(async move {
            FrameHandle::handle_frame(bytes);
        });
    }

    fn handle_frame(bytes: Vec<u8>) {
        let header = idl::Header::follow(&bytes, 0);
        match header.type_() {
            FrameType::Message => {}
            FrameType::MessageAck => {}
            not_type @ _ => {
                log::error!("'{:?}' is not frame type", not_type);
            }
        }
    }
}