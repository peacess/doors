use crate::server::{Handle, handle_impl::FrameType};

pub(crate) struct HandleMessage {}

impl Handle for HandleMessage {
    fn handle(&self, _bytes: Vec<u8>) {
        todo!()
    }

    fn frame_type(&self) -> FrameType {
        FrameType::None
    }
}
