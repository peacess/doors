use std::sync::Arc;

use idl::{ErrorInfo, ErrorInfoArgs, FrameError, FrameErrorArgs, Header, UlidBytes};

use super::ffi_bytes::FfiBytes;
use crate::lib_app::LibApp;

#[unsafe(no_mangle)]
pub extern "C" fn init(callback: CallBack) -> FfiBytes {
    env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).init();
    log::info!("init rust lib rpc ffi");
    if let Err(e) = LibApp::init(Arc::new(Some(callback))) {
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
    FfiBytes::zero()
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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn call(bytes: *mut u8, length: u64) -> FfiBytes {
    //todo
    let raw_bytes = unsafe { core::slice::from_raw_parts(bytes, length as usize) };

    let err = if let Some(app) = LibApp::app() {
        match app.call(raw_bytes) {
            Ok(bytes) => return bytes,
            Err(e) => {
                log::error!("Error calling app: {}", e);
                e
            }
        }
    } else {
        log::error!("lib app is none");
        anyhow::anyhow!("lib app is none")
    };
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let message = Some(builder.create_string(err.to_string().as_str()));
    let error_info = ErrorInfo::create(
        &mut builder,
        &ErrorInfoArgs {
            id: Some(&UlidBytes::from(idl::ids::generate_ulid())),
            req_id: None,
            code: 1,
            message,
        },
    );

    let header = Header::zero_error(error_info.value() as u32);
    let data_frame = FrameError::create(
        &mut builder,
        &FrameErrorArgs {
            header: Some(&header),
            error_info: Some(error_info),
        },
    );
    builder.finish(data_frame, None);
    // todo how to dont copy it
    FfiBytes::from(builder.finished_data().to_vec())
}

#[unsafe(no_mangle)]
pub extern "C" fn bytes_free(mut data: FfiBytes) {
    if !data.bytes.is_null() {
        let _ = unsafe { Vec::from_raw_parts(data.bytes, data.len as usize, data.capacity as usize) };
        //maybe it don't work, but it is a good code
        data.bytes = core::ptr::null_mut();
        data.len = 0;
        data.capacity = 0;
        data.offset = 0;
    }
}

/// 回调用函数的返回值在dart中并不支持，所以没有返回值
pub type CallBack = extern "C" fn(FfiBytes);

/// remove the length for performance
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn generate_ulid(bytes: *mut u8) {
    let bs = ulid::Ulid::new().to_bytes();
    // debug_assert_eq!(bs.len(), length as usize);
    unsafe {
        core::ptr::copy_nonoverlapping(bs.as_ptr(), bytes, bs.len());
    }
}

/// remove the length for performance
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn generate_uuid_v7(bytes: *mut u8) {
    let v7 = uuid::Uuid::now_v7();
    let bs = v7.as_bytes();
    // debug_assert_eq!(bs.len(), length as usize);
    unsafe {
        core::ptr::copy_nonoverlapping(bs.as_ptr(), bytes, bs.len());
    }
}
