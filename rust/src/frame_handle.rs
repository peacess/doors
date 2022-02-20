pub struct FrameHandle {}

impl FrameHandle {
    pub fn new() -> Self {
        FrameHandle {}
    }

    // 由于函数会使用多线程等，所以参数不能使用引用
    pub fn handle(&self, bytes: Vec<u8>) {}
}