use crate::idl::FrameType;
use crate::server::Handle;

pub(crate) struct HandleMessage {}

impl Handle for HandleMessage {
    fn handle(&self, bytes: Vec<u8>) {
        todo!()
    }

    fn frame_type(&self) -> FrameType {
        FrameType::Message
    }
}