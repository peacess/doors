use crate::lib_app::LibApp;

#[repr(C)]
pub struct FfiBytes {
    len: u64,
    capacity: u64,
    offset: u64,
    bytes: *mut u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn init(callback: CallBack) -> FfiBytes {
    if let Err(e) = LibApp::init(callback) {
        log::error!("Error building tokio runtime: {}", e);
        let mut info = format!("{}", e).as_bytes().to_vec();
        let bytes = info.as_mut_ptr();
        let len = info.len() as u64;
        let capacity = info.capacity() as u64;
        std::mem::forget(info);
        return FfiBytes {
            len,
            capacity,
            offset: 0,
            bytes,
        };
    }
    FfiBytes {
        len: 0,
        capacity: 0,
        offset: 0,
        bytes: core::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn un_init() -> FfiBytes {
    LibApp::uninit();
    FfiBytes {
        len: 0,
        capacity: 0,
        offset: 0,
        bytes: core::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn call(data: FfiBytes) -> FfiBytes {
    //todo
    let c = vec![0u8];
    let mut v = core::mem::ManuallyDrop::new(c);
    FfiBytes {
        len: v.len() as u64,
        capacity: v.capacity() as u64,
        offset: 0,
        bytes: v.as_mut_ptr(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bytes_free(mut data: FfiBytes) {
    if !data.bytes.is_null() {
        let _ = unsafe { Vec::from_raw_parts(data.bytes, data.len as usize, data.capacity as usize) };
        //maybe it don't work, but it is a good code
        data.bytes = core::ptr::null_mut();
    }
}

/// 回调用函数的返回值在dart中并不支持，所以没有返回值
pub type CallBack = extern "C" fn(FfiBytes);

// /// if the parameter call_back is null, then cancel the callback
// #[unsafe(no_mangle)]
// pub extern "C" fn set_call_back(callback: Option<CallBack>) -> i32 {
//     let c = vec![0u8];
//     let _v = core::mem::ManuallyDrop::new(c);
//     0
// }
