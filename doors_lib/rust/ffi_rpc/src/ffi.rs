use crate::lib_app::LibApp;

#[repr(C)]
pub struct Bytes {
    bytes: *mut u8,
    len: u64,
    capacity: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn init() -> Bytes {
    if let Err(e)= LibApp::init() {
        log::error!("Error building tokio runtime: {}", e);
        let mut info = format!("{}", e).as_bytes().to_vec();
        let bytes = info.as_mut_ptr();
        let len = info.len() as u64;
        let capacity = info.capacity() as u64;
        std::mem::forget(info);
        return Bytes{
            bytes,
            len,
            capacity,
        };
    }
    Bytes{
        bytes: core::ptr::null_mut(),
        len: 0,
        capacity: 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn un_init() -> Bytes {
    Bytes{
        bytes: core::ptr::null_mut(),
        len: 0,
        capacity: 0,
    }
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub extern "C" fn bytes_free(bytes: &mut Bytes) {
    let _ = unsafe { Vec::from_raw_parts(bytes.bytes, bytes.len as usize, bytes.capacity as usize) };
    return;
}