use crate::server::{Handle, handle_impl::FrameType};

pub(crate) struct HandleMessageAck {}

impl Handle for HandleMessageAck {
    fn handle(&self, _bytes: Vec<u8>) {
        todo!()
    }

    fn frame_type(&self) -> FrameType {
        FrameType::None
    }
}
