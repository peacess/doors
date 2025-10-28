use std::mem::ManuallyDrop;

#[repr(C)]
pub struct FfiBytes {
    pub(crate) len: u64,
    pub(crate) capacity: u64,
    pub(crate) offset: u64,
    pub(crate) bytes: *mut u8,
}

impl FfiBytes {
    #[inline]
    pub fn from(bytes: Vec<u8>) -> FfiBytes {
        let mut data = ManuallyDrop::new(bytes);
        FfiBytes {
            len: data.len() as u64,
            capacity: data.capacity() as u64,
            offset: 0,
            bytes: data.as_mut_ptr(),
        }
    }

    pub fn zero() -> Self {
        Self {
            len: 0,
            capacity: 0,
            offset: 0,
            bytes: core::ptr::null_mut(),
        }
    }
}
