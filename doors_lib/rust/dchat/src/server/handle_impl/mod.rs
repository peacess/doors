pub use handles::{Handle, Handles};

mod handle_message;
mod handle_message_ack;
mod handles;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum FrameType {
    None,
}

impl From<u32> for FrameType {
    fn from(_value: u32) -> Self {
        //todo
        FrameType::None
    }
}
