use criterion::{black_box, criterion_group, criterion_main, Criterion};

use polycrypt_rs::crypto::encryption;

use serde_json::json;

fn bench_encrypt(c: &mut Criterion) {
    let plaintext = b"Hello, world! This is a test message for benchmarking.";
    let key = [0u8; 32]; // Use a fixed key for benchmarking

    c.bench_function("encrypt", |b| {
        b.iter(|| encryption::encrypt(black_box(plaintext), black_box(&key)))
    });
}

fn bench_decrypt(c: &mut Criterion) {
    let plaintext = b"Hello, world! This is a test message for benchmarking.";
    let key = [0u8; 32]; // Use a fixed key for benchmarking
    let ciphertext = encryption::encrypt(plaintext, &key).unwrap();

    c.bench_function("decrypt", |b| {
        b.iter(|| encryption::decrypt(black_box(&ciphertext), black_box(&key)))
    });
}

fn bench_encrypt_fields(c: &mut Criterion) {
    let key = [0u8; 32]; // Use a fixed key for benchmarking
    let record = json!({
        "id": "1234",
        "name": "John Doe",
        "sensitive_data": "This is sensitive information",
        "array_field": ["item1", "item2", "item3"]
    });
    let fields_to_encrypt = vec!["sensitive_data".to_string(), "array_field".to_string()];

    c.bench_function("encrypt_fields", |b| {
        b.iter(|| {
            encryption::encrypt_fields(
                black_box(&record),
                black_box(&fields_to_encrypt),
                black_box(&key),
            )
        })
    });
}

fn bench_decrypt_fields(c: &mut Criterion) {
    let key = [0u8; 32]; // Use a fixed key for benchmarking
    let record = json!({
        "id": "1234",
        "name": "John Doe",
        "sensitive_data": "This is sensitive information",
        "array_field": ["item1", "item2", "item3"]
    });
    let fields_to_encrypt = vec!["sensitive_data".to_string(), "array_field".to_string()];
    let encrypted_record = encryption::encrypt_fields(&record, &fields_to_encrypt, &key).unwrap();

    c.bench_function("decrypt_fields", |b| {
        b.iter(|| {
            encryption::decrypt_fields(
                black_box(&encrypted_record),
                black_box(&fields_to_encrypt),
                black_box(&key),
            )
        })
    });
}

criterion_group!(
    benches,
    bench_encrypt,
    bench_decrypt,
    bench_encrypt_fields,
    bench_decrypt_fields
);

criterion_main!(benches);
