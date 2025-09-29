
#[repr(C)]
pub struct Bytes {
    bytes: *mut u8,
    len: u64,
    capacity: u64,
}

#[no_mangle]
pub extern "C" fn call(method_idd: u64, _in_parameter: & Bytes) -> Bytes {
    if method_idd < 10 {
        println!("");
    }
    let c = vec![0u8];
    let mut v = core::mem::ManuallyDrop::new(c);
    return Bytes{
        bytes: v.as_mut_ptr(),
        len: v.len() as u64,
        capacity: v.capacity() as u64,
    };
}

#[no_mangle]
pub extern "C" fn bytes_free(bytes: &mut Bytes) {
    let _ = unsafe { Vec::from_raw_parts(bytes.bytes, bytes.len as usize, bytes.capacity as usize) };
    return;
}