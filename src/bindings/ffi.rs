use std::slice;
use std::os::raw::{c_char, c_uchar};
use std::ffi::{CStr, CString};
use crate::crypto::encryption;
use env_logger::{Builder, Env};
use serde_json::Value;
use std::ffi::c_void;

#[repr(C)]
pub struct ByteArray {
    pub data: *mut u8,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn encrypt(plaintext: *const c_uchar, plaintext_len: usize, key: *const c_uchar) -> ByteArray {
    let plaintext_slice = unsafe { slice::from_raw_parts(plaintext, plaintext_len) };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let key_array: [u8; 32] = key_slice.try_into().unwrap();

    match encryption::encrypt(plaintext_slice, &key_array) {
        Ok(encrypted) => {
            let mut boxed_slice = encrypted.into_boxed_slice();
            let ptr = boxed_slice.as_mut_ptr();
            let len = boxed_slice.len();
            std::mem::forget(boxed_slice);
            ByteArray { data: ptr, len }
        }
        Err(_) => ByteArray { data: std::ptr::null_mut(), len: 0 },
    }
}

#[no_mangle]
pub extern "C" fn free_byte_array(arr: ByteArray) {
    if !arr.data.is_null() {
        unsafe {
            let _ = Box::from_raw(slice::from_raw_parts_mut(arr.data, arr.len));
        }
    }
}

#[no_mangle]
pub extern "C" fn decrypt(ciphertext: *const c_uchar, ciphertext_len: usize, key: *const c_uchar) -> ByteArray {
    let ciphertext_slice = unsafe { slice::from_raw_parts(ciphertext, ciphertext_len) };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let key_array: [u8; 32] = key_slice.try_into().unwrap();

    match encryption::decrypt(ciphertext_slice, &key_array) {
        Ok(decrypted) => {
            let mut boxed_slice = decrypted.into_boxed_slice();
            let ptr = boxed_slice.as_mut_ptr();
            let len = boxed_slice.len();
            std::mem::forget(boxed_slice);
            ByteArray { data: ptr, len }
        }
        Err(_) => ByteArray { data: std::ptr::null_mut(), len: 0 },
    }
}

#[no_mangle]
pub extern "C" fn init_logger() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            use std::io::Write;
            writeln!(buf, "{}", record.args())
        })
        .init();
}

#[no_mangle]
pub extern "C" fn decrypt_fields(record: *const c_char, fields_to_decrypt: *const c_char, key: *const c_uchar) -> *mut c_char {
    let record_str = unsafe { CStr::from_ptr(record).to_str().unwrap() };
    let fields_str = unsafe { CStr::from_ptr(fields_to_decrypt).to_str().unwrap() };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let key_array: [u8; 32] = key_slice.try_into().unwrap();

    let record_value: Value = serde_json::from_str(record_str).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    match encryption::decrypt_fields(&record_value, &fields, &key_array) {
        Ok(decrypted) => {
            let json_string = serde_json::to_string(&decrypted).unwrap();
            CString::new(json_string).unwrap().into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn encrypt_fields(record: *const c_char, fields_to_encrypt: *const c_char, key: *const c_uchar) -> *mut c_char {
    let record_str = unsafe { CStr::from_ptr(record).to_str().unwrap() };
    let fields_str = unsafe { CStr::from_ptr(fields_to_encrypt).to_str().unwrap() };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let key_array: [u8; 32] = key_slice.try_into().unwrap();

    let record_value: Value = serde_json::from_str(record_str).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    match encryption::encrypt_fields(&record_value, &fields, &key_array) {
        Ok(encrypted) => {
            let json_string = serde_json::to_string(&encrypted).unwrap();
            CString::new(json_string).unwrap().into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn free_c_char(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn allocate_buffer(size: usize) -> *mut c_void {
    let buffer = vec![0u8; size].into_boxed_slice();
    let ptr = Box::into_raw(buffer) as *mut c_void;
    ptr
}

#[no_mangle]
pub extern "C" fn free_buffer(ptr: *mut c_void) {
    unsafe {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr as *mut u8);
        }
    }
}

#[no_mangle]
pub extern "C" fn decrypt_fields_in_batch(records: *const c_char, fields_to_decrypt: *const c_char, key: *const c_uchar) -> *mut c_char {
    let records_str = unsafe { CStr::from_ptr(records).to_str().unwrap() };
    let fields_str = unsafe { CStr::from_ptr(fields_to_decrypt).to_str().unwrap() };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let key_array: [u8; 32] = key_slice.try_into().unwrap();

    let records: Vec<Value> = serde_json::from_str(records_str).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    match encryption::decrypt_fields_in_batch(&records, &fields, &key_array) {
        Ok(decrypted_records) => {
            let result_json = serde_json::to_string(&decrypted_records).unwrap();
            CString::new(result_json).unwrap().into_raw()
        }
        Err(e) => {
            eprintln!("Error in decrypt_fields_in_batch: {:?}", e);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn encrypt_fields_in_batch(records: *const c_char, fields_to_encrypt: *const c_char, key: *const c_uchar) -> *mut c_char {
    let records_str = unsafe { CStr::from_ptr(records).to_str().unwrap() };
    let fields_str = unsafe { CStr::from_ptr(fields_to_encrypt).to_str().unwrap() };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let key_array: [u8; 32] = key_slice.try_into().unwrap();

    let records: Vec<Value> = serde_json::from_str(records_str).unwrap();
    let fields: Vec<String> = serde_json::from_str(fields_str).unwrap();

    match encryption::encrypt_fields_in_batch(&records, &fields, &key_array) {
        Ok(encrypted_records) => {
            let result_json = serde_json::to_string(&encrypted_records).unwrap();
            CString::new(result_json).unwrap().into_raw()
        }
        Err(e) => {
            eprintln!("Error in encrypt_fields_in_batch: {:?}", e);
            std::ptr::null_mut()
        }
    }
}
