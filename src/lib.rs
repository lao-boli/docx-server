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

// 新增结构体用于FFI传输 (必须repr(C))
#[repr(C)]
pub struct ByteResult {
    data_ptr: *mut u8,
    data_len: usize,
    error_ptr: *mut c_char,
}

// 生成DOCX并返回字节流
#[no_mangle]
pub extern "C" fn generate_docx_bytes(
    json_config: *const c_char,
    error_ptr: *mut *mut c_char
) -> ByteResult {
    let mut result = ByteResult {
        data_ptr: ptr::null_mut(),
        data_len: 0,
        error_ptr: ptr::null_mut(),
    };

    // 安全包装
    let parse_result = unsafe {
        if json_config.is_null() {
            return result_with_error(result, "Null pointer received".into(), error_ptr);
        }

        // 转换C字符串
        let c_config_str = match CStr::from_ptr(json_config).to_str() {
            Ok(s) => s,
            Err(e) => {
                return result_with_error(
                    result,
                    format!("Invalid UTF-8: {}", e),
                    error_ptr
                );
            }
        };

        // 解析JSON
        match serde_json::from_str::<TableConfig>(c_config_str) {
            Ok(config) => {
                match core::generate_docx(&config) {
                    Ok(docx) => {
                        let mut buf = Cursor::new(Vec::new());
                        if let Err(e) = docx.build().pack(&mut buf) {
                            return result_with_error(
                                result,
                                format!("DOCX build failed: {}", e),
                                error_ptr
                            );
                        }
                        let bytes = buf.into_inner();
                        bytes
                    },
                    Err(e) => {
                        return result_with_error(
                            result,
                            format!("Generation error: {}", e),
                            error_ptr
                        );
                    }
                }
            },
            Err(e) => {
                return result_with_error(
                    result,
                    format!("JSON parse error: {}", e),
                    error_ptr
                );
            }
        }
    };

    // 处理成功结果
    let mut bytes = parse_result;
    let len = bytes.len();
    let ptr = bytes.as_mut_ptr();

    // 防止Rust释放内存
    std::mem::forget(bytes);

    ByteResult {
        data_ptr: ptr,
        data_len: len,
        error_ptr: ptr::null_mut(),
    }
}

// 统一错误处理
fn result_with_error(
    mut result: ByteResult,
    msg: String,
    error_ptr: *mut *mut c_char
) -> ByteResult {
    let c_error = match CString::new(msg) {
        Ok(s) => s,
        Err(_) => CString::new("Error message conversion failed").unwrap(),
    };

    unsafe {
        if !error_ptr.is_null() {
            *error_ptr = c_error.into_raw();
        } else {
            result.error_ptr = c_error.into_raw();
        }
    }

    result
}

// 内存释放函数
#[no_mangle]
pub extern "C" fn free_rust_bytes(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, len);
        }
    }
}
