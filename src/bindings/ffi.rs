use crate::crypto::encryption;
use crate::error::PolyCryptError;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[repr(C)]
pub struct ByteArray {
    pub data: *mut u8,
    pub len: usize,
}

#[repr(C)]
pub struct FFIResult {
    pub data: ByteArray,
    pub error_code: i32,
}

fn to_byte_array(vec: Vec<u8>) -> ByteArray {
    let mut boxed_slice = vec.into_boxed_slice();
    let ptr = boxed_slice.as_mut_ptr();
    let len = boxed_slice.len();
    std::mem::forget(boxed_slice);
    ByteArray { data: ptr, len }
}

fn to_ffi_result(result: Result<Vec<u8>, PolyCryptError>) -> FFIResult {
    match result {
        Ok(data) => FFIResult {
            data: to_byte_array(data),
            error_code: 0,
        },
        Err(e) => {
            eprintln!("Error: {:?}", e);
            FFIResult {
                data: ByteArray {
                    data: std::ptr::null_mut(),
                    len: 0,
                },
                error_code: -1,
            }
        }
    }
}

fn validate_key(key: *const u8) -> Result<[u8; 32], FFIResult> {
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    if key_slice.len() != 32 {
        return Err(FFIResult {
            data: ByteArray {
                data: std::ptr::null_mut(),
                len: 0,
            },
            error_code: -1,
        });
    }
    key_slice.try_into().map_err(|_| FFIResult {
        data: ByteArray {
            data: std::ptr::null_mut(),
            len: 0,
        },
        error_code: -1,
    })
}

#[no_mangle]
pub extern "C" fn encrypt(plaintext: *const u8, plaintext_len: usize, key: *const u8) -> FFIResult {
    let plaintext_slice = unsafe { slice::from_raw_parts(plaintext, plaintext_len) };

    let key_array = match validate_key(key) {
        Ok(k) => k,
        Err(e) => return e,
    };

    match encryption::encrypt(plaintext_slice, &key_array) {
        Ok(encrypted_data) => FFIResult {
            data: to_byte_array(encrypted_data),
            error_code: 0,
        },
        Err(_) => FFIResult {
            data: ByteArray {
                data: std::ptr::null_mut(),
                len: 0,
            },
            error_code: -1,
        },
    }
}

#[no_mangle]
pub extern "C" fn decrypt(
    ciphertext: *const u8,
    ciphertext_len: usize,
    key: *const u8,
) -> FFIResult {
    let ciphertext_slice = unsafe { slice::from_raw_parts(ciphertext, ciphertext_len) };

    let key_array = match validate_key(key) {
        Ok(k) => k,
        Err(e) => return e,
    };

    match encryption::decrypt(ciphertext_slice, &key_array) {
        Ok(decrypted_data) => FFIResult {
            data: to_byte_array(decrypted_data),
            error_code: 0,
        },
        Err(_) => FFIResult {
            data: ByteArray {
                data: std::ptr::null_mut(),
                len: 0,
            },
            error_code: -1,
        },
    }
}

#[no_mangle]
pub extern "C" fn encrypt_fields(
    record: *const c_char,
    fields_to_encrypt: *const c_char,
    key: *const u8,
) -> FFIResult {
    let key_array = match validate_key(key) {
        Ok(k) => k,
        Err(e) => return e,
    };

    let record_str = unsafe { CStr::from_ptr(record).to_str().unwrap() };
    let fields_str = unsafe { CStr::from_ptr(fields_to_encrypt).to_str().unwrap() };

    let record: Value = serde_json::from_str(record_str).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    let result = encryption::encrypt_fields(&record, &fields, &key_array)
        .map(|encrypted| serde_json::to_vec(&encrypted).unwrap());

    to_ffi_result(result)
}

#[no_mangle]
pub extern "C" fn decrypt_fields(
    encrypted: *const u8,
    encrypted_len: usize,
    fields_to_decrypt: *const c_char,
    key: *const u8,
) -> FFIResult {
    let key_array = match validate_key(key) {
        Ok(k) => k,
        Err(e) => return e,
    };

    let encrypted_slice = unsafe { slice::from_raw_parts(encrypted, encrypted_len) };
    let fields_str = unsafe { CStr::from_ptr(fields_to_decrypt).to_str().unwrap() };

    let encrypted_value: Value = serde_json::from_slice(encrypted_slice).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    let result = encryption::decrypt_fields(&encrypted_value, &fields, &key_array)
        .map(|decrypted| serde_json::to_vec(&decrypted).unwrap());

    to_ffi_result(result)
}

#[no_mangle]
pub extern "C" fn encrypt_fields_in_batch(
    records: *const c_char,
    fields_to_encrypt: *const c_char,
    key: *const u8,
) -> FFIResult {
    let key_array = match validate_key(key) {
        Ok(k) => k,
        Err(e) => return e,
    };

    let records_str = unsafe { CStr::from_ptr(records).to_str().unwrap() };
    let fields_str = unsafe { CStr::from_ptr(fields_to_encrypt).to_str().unwrap() };

    let records: Vec<Value> = serde_json::from_str(records_str).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    let result = encryption::encrypt_fields_in_batch(&records, &fields, &key_array)
        .map(|encrypted_records| serde_json::to_vec(&encrypted_records).unwrap());

    to_ffi_result(result)
}

#[no_mangle]
pub extern "C" fn decrypt_fields_in_batch(
    encrypted: *const u8,
    encrypted_len: usize,
    fields_to_decrypt: *const c_char,
    key: *const u8,
) -> FFIResult {
    let key_array = match validate_key(key) {
        Ok(k) => k,
        Err(e) => return e,
    };

    let encrypted_slice = unsafe { slice::from_raw_parts(encrypted, encrypted_len) };
    let fields_str = unsafe { CStr::from_ptr(fields_to_decrypt).to_str().unwrap() };

    let encrypted_records: Vec<Value> = serde_json::from_slice(encrypted_slice).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    let result = encryption::decrypt_fields_in_batch(&encrypted_records, &fields, &key_array)
        .map(|decrypted_records| serde_json::to_vec(&decrypted_records).unwrap());

    to_ffi_result(result)
}

#[no_mangle]
pub extern "C" fn free_ffi_result(result: FFIResult) {
    if !result.data.data.is_null() {
        unsafe {
            let _ = Box::from_raw(slice::from_raw_parts_mut(result.data.data, result.data.len));
        }
    }
}

// Add these new functions at the end of the file

#[no_mangle]
pub extern "C" fn free_byte_array(arr: ByteArray) {
    if !arr.data.is_null() {
        unsafe {
            let _ = Box::from_raw(slice::from_raw_parts_mut(arr.data, arr.len));
        }
    }
}

#[no_mangle]
pub extern "C" fn free_c_char(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn init_logger() {
    // Initialize the logger here
    // You may need to add a dependency on the `log` crate and implement this function
    // For now, we'll leave it as a no-op
}
