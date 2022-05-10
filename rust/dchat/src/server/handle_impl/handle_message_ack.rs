use crate::idl::FrameType;
use crate::server::Handle;

pub(crate) struct HandleMessageAck {}

impl Handle for HandleMessageAck {
    fn handle(&self, bytes: Vec<u8>) {
        todo!()
    }

    fn frame_type(&self) -> FrameType {
        FrameType::MessageAck
    }
}