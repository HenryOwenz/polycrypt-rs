use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polycrypt_rs::bindings::ffi;
use std::ffi::CString;

fn bench_ffi_encrypt(c: &mut Criterion) {
    let plaintext = b"Hello, world! This is a test message for benchmarking.";
    let key = [0u8; 32];
    c.bench_function("ffi_encrypt", |b| {
        b.iter(|| {
            let result = ffi::encrypt(
                black_box(plaintext.as_ptr()),
                black_box(plaintext.len()),
                black_box(key.as_ptr()),
            );
            ffi::free_ffi_result(result);
        })
    });
}

fn bench_ffi_decrypt(c: &mut Criterion) {
    let plaintext = b"Hello, world! This is a test message for benchmarking.";
    let key = [0u8; 32];
    let encrypted = ffi::encrypt(plaintext.as_ptr(), plaintext.len(), key.as_ptr());
    c.bench_function("ffi_decrypt", |b| {
        b.iter(|| {
            let result = ffi::decrypt(
                black_box(encrypted.data.data),
                black_box(encrypted.data.len),
                black_box(key.as_ptr()),
            );
            ffi::free_ffi_result(result);
        })
    });
    ffi::free_ffi_result(encrypted);
}

fn bench_ffi_encrypt_fields(c: &mut Criterion) {
    let key = [0u8; 32];
    let record = r#"{"id":"1234","name":"John Doe","sensitive_data":"This is sensitive information","array_field":["item1","item2","item3"]}"#;
    let fields = r#"["sensitive_data","array_field"]"#;
    let record_cstring = CString::new(record).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    c.bench_function("ffi_encrypt_fields", |b| {
        b.iter(|| {
            let result = ffi::encrypt_fields(
                black_box(record_cstring.as_ptr()),
                black_box(fields_cstring.as_ptr()),
                black_box(key.as_ptr()),
            );
            ffi::free_ffi_result(result);
        })
    });
}

fn bench_ffi_decrypt_fields(c: &mut Criterion) {
    let key = [0u8; 32];
    let record = r#"{"id":"1234","name":"John Doe","sensitive_data":"This is sensitive information","array_field":["item1","item2","item3"]}"#;
    let fields = r#"["sensitive_data","array_field"]"#;
    let record_cstring = CString::new(record).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    let encrypted = ffi::encrypt_fields(record_cstring.as_ptr(), fields_cstring.as_ptr(), key.as_ptr());

    c.bench_function("ffi_decrypt_fields", |b| {
        b.iter(|| {
            let result = ffi::decrypt_fields(
                black_box(encrypted.data.data),
                black_box(encrypted.data.len),
                black_box(fields_cstring.as_ptr()),
                black_box(key.as_ptr()),
            );
            ffi::free_ffi_result(result);
        })
    });

    ffi::free_ffi_result(encrypted);
}

fn bench_ffi_encrypt_fields_in_batch(c: &mut Criterion) {
    let key = [0u8; 32];
    let records = r#"[
        {"id":"1234","name":"John Doe","sensitive_data":"This is sensitive information","array_field":["item1","item2","item3"]},
        {"id":"5678","name":"Jane Smith","sensitive_data":"Another piece of sensitive data","array_field":["item4","item5","item6"]}
    ]"#;
    let fields = r#"["sensitive_data","array_field"]"#;
    let records_cstring = CString::new(records).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    c.bench_function("ffi_encrypt_fields_in_batch", |b| {
        b.iter(|| {
            let result = ffi::encrypt_fields_in_batch(
                black_box(records_cstring.as_ptr()),
                black_box(fields_cstring.as_ptr()),
                black_box(key.as_ptr()),
            );
            ffi::free_ffi_result(result);
        })
    });
}

fn bench_ffi_decrypt_fields_in_batch(c: &mut Criterion) {
    let key = [0u8; 32];
    let records = r#"[
        {"id":"1234","name":"John Doe","sensitive_data":"This is sensitive information","array_field":["item1","item2","item3"]},
        {"id":"5678","name":"Jane Smith","sensitive_data":"Another piece of sensitive data","array_field":["item4","item5","item6"]}
    ]"#;
    let fields = r#"["sensitive_data","array_field"]"#;
    let records_cstring = CString::new(records).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    let encrypted = ffi::encrypt_fields_in_batch(records_cstring.as_ptr(), fields_cstring.as_ptr(), key.as_ptr());

    c.bench_function("ffi_decrypt_fields_in_batch", |b| {
        b.iter(|| {
            let result = ffi::decrypt_fields_in_batch(
                black_box(encrypted.data.data),
                black_box(encrypted.data.len),
                black_box(fields_cstring.as_ptr()),
                black_box(key.as_ptr()),
            );
            ffi::free_ffi_result(result);
        })
    });

    ffi::free_ffi_result(encrypted);
}

criterion_group!(
    benches,
    bench_ffi_encrypt,
    bench_ffi_decrypt,
    bench_ffi_encrypt_fields,
    bench_ffi_decrypt_fields,
    bench_ffi_encrypt_fields_in_batch,
    bench_ffi_decrypt_fields_in_batch
);
criterion_main!(benches);