mod core;
mod rpc;
mod pb;
mod client;

pub use core::*;
pub use rpc::*;
pub use pb::*;

use docx_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// src/lib.rs
use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::io::Cursor;
use std::ptr;
use std::slice;
use tonic::Status;

#[no_mangle]
pub extern "C" fn generate_docx(
    json_config: *const c_char,
    output_path: *const c_char,
    error_ptr: *mut *mut c_char
) -> bool {
    let mut error_message = None;

    // 安全包装（处理空指针）
    let result = unsafe {
        if json_config.is_null() || output_path.is_null() {
            error_message = Some("Null pointer received".to_string());
            return false
        }

        // 转换C字符串为Rust类型
        let c_config_str = CStr::from_ptr(json_config);
        let output_path_str = CStr::from_ptr(output_path).to_string_lossy();

        // 解析JSON配置
        match serde_json::from_str::<TableConfig>(c_config_str.to_str().unwrap()) {
            Ok(config) => {
                // 生成文档
                match core::generate_docx(&config) {
                    Ok(docx) => {
                        let mut buf = Cursor::new(Vec::new());
                        docx.build().pack(&mut buf).unwrap();
                        match std::fs::write(&*output_path_str, buf.into_inner()) {
                            Ok(_) => true,
                            Err(e) => {
                                error_message = Some(format!("File write error: {}", e));
                                false
                            }
                        }
                    },
                    Err(e) => {
                        error_message = Some(format!("Generation error: {}", e));
                        false
                    }
                }
            },
            Err(e) => {
                error_message = Some(format!("JSON parse error: {}", e));
                false
            }
        }
    };

    // 错误处理（通过指针返回错误信息）
    if let Some(msg) = error_message {
        let c_error = CString::new(msg).unwrap();
        unsafe {
            *error_ptr = c_error.into_raw();
        }
    }

    result
}

#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    }
}
