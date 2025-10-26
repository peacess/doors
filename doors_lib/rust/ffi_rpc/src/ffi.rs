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
    env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).init();
    log::info!("init rust lib rpc ffi");
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
pub extern "C" fn call(bytes: *mut u8, length: u64) -> FfiBytes {
    //todo
    let c = {
        let temp = unsafe { Vec::from_raw_parts(bytes, length as usize, length as usize) };
        let copy = temp.clone();
        core::mem::forget(temp);
        copy
    };
    if let Some(app) = LibApp::app() {
        let mut data = app.call(&c);
        let bytes = data.as_mut_ptr();
        let len = data.len() as u64;
        let capacity = data.capacity() as u64;
        std::mem::forget(data);
        return FfiBytes {
            len,
            capacity,
            offset: 0,
            bytes,
        };
    } else {
        log::error!("Error calling lib app");
    }
    FfiBytes {
        len: 0,
        capacity: 0,
        offset: 0,
        bytes: core::ptr::null_mut(),
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

/// remove the length for performance
#[unsafe(no_mangle)]
pub extern "C" fn generate_ulid(bytes: *mut u8) {
    let bs = ulid::Ulid::new().to_bytes();
    // debug_assert_eq!(bs.len(), length as usize);
    unsafe {
        core::ptr::copy_nonoverlapping(bs.as_ptr(), bytes, bs.len());
    }
}

/// remove the length for performance
#[unsafe(no_mangle)]
pub extern "C" fn generate_uuid_v7(bytes: *mut u8) {
    let v7 = uuid::Uuid::now_v7();
    let bs = v7.as_bytes();
    // debug_assert_eq!(bs.len(), length as usize);
    unsafe {
        core::ptr::copy_nonoverlapping(bs.as_ptr(), bytes, bs.len());
    }
}

impl FfiBytes {
    #[inline]
    pub fn from(mut bytes: Vec<u8>) -> FfiBytes {
        let b = FfiBytes {
            len: bytes.len() as u64,
            capacity: bytes.capacity() as u64,
            offset: 0,
            bytes: bytes.as_mut_ptr(),
        };
        std::mem::forget(bytes);
        b
    }
}
