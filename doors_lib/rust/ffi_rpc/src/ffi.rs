use crate::lib_app::LibApp;

#[repr(C)]
pub struct Bytes {
    len: u64,
    capacity: u64,
    offset: u64,
    bytes: *mut u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn init(callback: CallBack) -> Bytes {
    if let Err(e) = LibApp::init(callback) {
        log::error!("Error building tokio runtime: {}", e);
        let mut info = format!("{}", e).as_bytes().to_vec();
        let bytes = info.as_mut_ptr();
        let len = info.len() as u64;
        let capacity = info.capacity() as u64;
        std::mem::forget(info);
        return Bytes {
            len,
            capacity,
            offset: 0,
            bytes,
        };
    }
    Bytes {
        len: 0,
        capacity: 0,
        offset: 0,
        bytes: core::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn un_init() -> Bytes {
    LibApp::uninit();
    Bytes {
        len: 0,
        capacity: 0,
        offset: 0,
        bytes: core::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn call(method_idd: u64, in_parameter: &Bytes) -> Bytes {
    if method_idd < 10 {
        println!();
    }
    let c = vec![0u8];
    let mut v = core::mem::ManuallyDrop::new(c);
    Bytes {
        len: v.len() as u64,
        capacity: v.capacity() as u64,
        offset: 0,
        bytes: v.as_mut_ptr(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bytes_free(bytes: Bytes) -> i32 {
    let _ = unsafe { Vec::from_raw_parts(bytes.bytes, bytes.len as usize, bytes.capacity as usize) };
    0
}

/// 回调用函数的返回值在dart中并不支持，所以没有返回值
pub type CallBack = extern "C" fn(Bytes);

// /// if the parameter call_back is null, then cancel the callback
// #[unsafe(no_mangle)]
// pub extern "C" fn set_call_back(callback: Option<CallBack>) -> i32 {
//     let c = vec![0u8];
//     let _v = core::mem::ManuallyDrop::new(c);
//     0
// }
