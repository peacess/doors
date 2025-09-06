use crate::{idl::FrameType, server::Handle};

pub(crate) struct HandleMessage {}

impl Handle for HandleMessage {
    fn handle(&self, _bytes: Vec<u8>) {
        todo!()
    }

    fn frame_type(&self) -> FrameType {
        FrameType::Message
    }
}
