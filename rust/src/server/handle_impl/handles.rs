use std::collections::BTreeMap;

use crate::idl::FrameType;

use super::handle_message::HandleMessage;
use super::handle_message_ack::HandleMessageAck;

pub trait Handle {
    fn handle(&self, bytes: Vec<u8>);
    fn frame_type(&self) -> FrameType;
}

type BoxHandle = Box<dyn Handle + Send + Sync + 'static>;
type MapHandles = BTreeMap<FrameType, BoxHandle>;

pub struct Handles {
    map_handles: MapHandles,
}

impl Handles {
    pub fn new() -> Self {
        let mut hs = Handles {
            map_handles: BTreeMap::new(),
        };

        hs.register_handles();
        hs
    }
    pub fn register_handles(&mut self) {
        let h = HandleMessage {};
        self.map_handles.insert(h.frame_type(), Box::new(h));

        let h = HandleMessageAck {};
        self.map_handles.insert(h.frame_type(), Box::new(h));
    }

    pub fn get(&self, frame_type: FrameType) -> Option<&BoxHandle> {
        self.map_handles.get(&frame_type)
    }
}