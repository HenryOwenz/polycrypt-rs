use polycrypt_rs::bindings::ffi;
use serde_json::Value;
use std::ffi::CString;
use std::str;

#[test]
fn test_ffi_encrypt_decrypt() {
    let plaintext = b"Hello, world!";
    let key = [0u8; 32];

    let encrypted = ffi::encrypt(plaintext.as_ptr(), plaintext.len(), key.as_ptr());
    assert_eq!(encrypted.error_code, 0);
    assert_ne!(encrypted.data.len, 0);
    assert_ne!(encrypted.data.data, std::ptr::null_mut());

    let decrypted = ffi::decrypt(encrypted.data.data, encrypted.data.len, key.as_ptr());
    assert_eq!(decrypted.error_code, 0);
    assert_ne!(decrypted.data.len, 0);
    assert_ne!(decrypted.data.data, std::ptr::null_mut());

    let decrypted_text =
        unsafe { std::slice::from_raw_parts(decrypted.data.data, decrypted.data.len) };
    assert_eq!(decrypted_text, plaintext);

    ffi::free_ffi_result(encrypted);
    ffi::free_ffi_result(decrypted);
}

#[test]
fn test_ffi_encrypt_decrypt_fields() {
    let key = [0u8; 32];
    let record = r#"{"id":"1234","name":"John Doe","sensitive_data":"This is sensitive information","array_field":["item1","item2","item3"]}"#;
    let fields = r#"["sensitive_data","array_field"]"#;

    let record_cstring = CString::new(record).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    let encrypted = ffi::encrypt_fields(
        record_cstring.as_ptr(),
        fields_cstring.as_ptr(),
        key.as_ptr(),
    );
    assert_eq!(encrypted.error_code, 0);
    assert_ne!(encrypted.data.len, 0);
    assert_ne!(encrypted.data.data, std::ptr::null_mut());

    let decrypted = ffi::decrypt_fields(
        encrypted.data.data,
        encrypted.data.len,
        fields_cstring.as_ptr(),
        key.as_ptr(),
    );
    assert_eq!(decrypted.error_code, 0);
    assert_ne!(decrypted.data.len, 0);
    assert_ne!(decrypted.data.data, std::ptr::null_mut());

    let decrypted_str = unsafe {
        str::from_utf8_unchecked(std::slice::from_raw_parts(
            decrypted.data.data,
            decrypted.data.len,
        ))
    };

    // Parse both strings as JSON and compare
    let original_json: Value = serde_json::from_str(record).unwrap();
    let decrypted_json: Value = serde_json::from_str(decrypted_str).unwrap();
    assert_eq!(original_json, decrypted_json);

    ffi::free_ffi_result(encrypted);
    ffi::free_ffi_result(decrypted);
}

#[test]
fn test_ffi_encrypt_decrypt_fields_in_batch() {
    let key = [0u8; 32];
    let records = r#"[
        {"id":"1234","name":"John Doe","sensitive_data":"This is sensitive information","array_field":["item1","item2","item3"]},
        {"id":"5678","name":"Jane Smith","sensitive_data":"Another piece of sensitive data","array_field":["item4","item5","item6"]}
    ]"#;
    let fields = r#"["sensitive_data","array_field"]"#;

    let records_cstring = CString::new(records).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    let encrypted = ffi::encrypt_fields_in_batch(
        records_cstring.as_ptr(),
        fields_cstring.as_ptr(),
        key.as_ptr(),
    );
    assert_eq!(encrypted.error_code, 0);
    assert_ne!(encrypted.data.len, 0);
    assert_ne!(encrypted.data.data, std::ptr::null_mut());

    let decrypted = ffi::decrypt_fields_in_batch(
        encrypted.data.data,
        encrypted.data.len,
        fields_cstring.as_ptr(),
        key.as_ptr(),
    );
    assert_eq!(decrypted.error_code, 0);
    assert_ne!(decrypted.data.len, 0);
    assert_ne!(decrypted.data.data, std::ptr::null_mut());

    let decrypted_str = unsafe {
        str::from_utf8_unchecked(std::slice::from_raw_parts(
            decrypted.data.data,
            decrypted.data.len,
        ))
    };

    // Parse both strings as JSON and compare
    let original_json: Value = serde_json::from_str(records).unwrap();
    let decrypted_json: Value = serde_json::from_str(decrypted_str).unwrap();
    assert_eq!(original_json, decrypted_json);

    ffi::free_ffi_result(encrypted);
    ffi::free_ffi_result(decrypted);
}
