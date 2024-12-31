use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polycrypt_rs::crypto::encryption;
use rusqlite::Connection;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::PathBuf;
use std::env;
use std::fs;
use std::ffi::CString;
use polycrypt_rs::bindings::ffi;

const BATCH_SIZES: [usize; 6] = [1, 100, 1_000, 10_000, 100_000, 1_000_000];
const BATCH_SIZE_INDEX: usize = 0; // Change this index to select different batch sizes
const BATCH_SIZE: usize = BATCH_SIZES[BATCH_SIZE_INDEX];

lazy_static! {
    static ref DB_PATH: String = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("benches");
        path.push("polycrypt_benchmark.db");
        path.to_str().unwrap().to_string()
    };
}

lazy_static! {
    static ref DB_CONN: Mutex<Connection> = Mutex::new(Connection::open(&*DB_PATH).unwrap());
}

// Fields to encrypt/decrypt, matching those in setup_db.py
const FIELDS_TO_ENCRYPT: [&str; 7] = ["conditions", "medications", "allergies", "notes", "sensitive_data", "name", "dob"];

fn sanity_check() {
    println!("Using database at path: {}", *DB_PATH);
    
    // Check if the file exists
    if fs::metadata(&*DB_PATH).is_ok() {
        println!("Database file exists.");
    } else {
        println!("Database file does not exist!");
    }

    // Try to open the file
    match Connection::open(&*DB_PATH) {
        Ok(_) => println!("Successfully opened the database connection."),
        Err(e) => println!("Failed to open database connection: {:?}", e),
    }

    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM plain_records").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    println!("Sanity check: Total records in plain_records: {}", count);
    
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM encrypted_records").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    println!("Sanity check: Total records in encrypted_records: {}", count);
}

fn bench_db_encrypt(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    c.bench_function("db_encrypt", |b| {
        b.iter(|| {
            let mut stmt = conn.prepare("SELECT data FROM plain_records ORDER BY RANDOM() LIMIT 1").unwrap();
            let record: String = stmt.query_row([], |row| row.get(0)).unwrap();
            let record: serde_json::Value = serde_json::from_str(&record).unwrap();
            let _encrypted = encryption::encrypt_fields(black_box(&record), black_box(&FIELDS_TO_ENCRYPT.iter().map(|&s| s.to_string()).collect::<Vec<String>>()), black_box(&key)).unwrap();
        })
    });
}

fn bench_db_decrypt(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    c.bench_function("db_decrypt", |b| {
        b.iter(|| {
            let mut stmt = conn.prepare("SELECT data FROM encrypted_records ORDER BY RANDOM() LIMIT 1").unwrap();
            let encrypted_record: String = stmt.query_row([], |row| row.get(0)).unwrap();
            let encrypted_record: serde_json::Value = serde_json::from_str(&encrypted_record).unwrap();
            let _decrypted = encryption::decrypt_fields(black_box(&encrypted_record), black_box(&FIELDS_TO_ENCRYPT.iter().map(|&s| s.to_string()).collect::<Vec<String>>()), black_box(&key));
        })
    });
}

fn bench_db_encrypt_fields(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    c.bench_function("db_encrypt_fields", |b| {
        b.iter(|| {
            let mut stmt = conn.prepare("SELECT data FROM plain_records ORDER BY RANDOM() LIMIT 1").unwrap();
            let record: String = stmt.query_row([], |row| row.get(0)).unwrap();
            let record: serde_json::Value = serde_json::from_str(&record).unwrap();
            let _encrypted = encryption::encrypt_fields(black_box(&record), black_box(&FIELDS_TO_ENCRYPT.iter().map(|&s| s.to_string()).collect::<Vec<String>>()), black_box(&key)).unwrap();
        })
    });
}

fn bench_db_decrypt_fields(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    c.bench_function("db_decrypt_fields", |b| {
        b.iter(|| {
            let mut stmt = conn.prepare("SELECT data FROM encrypted_records ORDER BY RANDOM() LIMIT 1").unwrap();
            let encrypted_record: String = stmt.query_row([], |row| row.get(0)).unwrap();
            let encrypted_record: serde_json::Value = serde_json::from_str(&encrypted_record).unwrap();
            let _decrypted = encryption::decrypt_fields(black_box(&encrypted_record), black_box(&FIELDS_TO_ENCRYPT.iter().map(|&s| s.to_string()).collect::<Vec<String>>()), black_box(&key));
        })
    });
}

fn bench_db_encrypt_fields_in_batch(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    let mut stmt = conn.prepare(&format!("SELECT data FROM plain_records LIMIT {}", BATCH_SIZE)).unwrap();
    let records: Vec<String> = stmt.query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    let records_count = records.len();
    let records: Vec<serde_json::Value> = records.iter().map(|r| serde_json::from_str(r).unwrap()).collect();

    c.bench_function(&format!("db_encrypt_fields_in_batch ({})", BATCH_SIZE), |b| {
        b.iter(|| {
            let _encrypted = encryption::encrypt_fields_in_batch(
                black_box(&records),
                black_box(&FIELDS_TO_ENCRYPT.iter().map(|&s| s.to_string()).collect::<Vec<String>>()),
                black_box(&key)
            ).unwrap();
        })
    });
    
    println!("Number of records processed for encryption: {}", records_count);
}

fn bench_db_decrypt_fields_in_batch(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    let mut stmt = conn.prepare(&format!("SELECT data FROM encrypted_records LIMIT {}", BATCH_SIZE)).unwrap();
    let records: Vec<String> = stmt.query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    let records_count = records.len();
    let records: Vec<serde_json::Value> = records.iter().map(|r| serde_json::from_str(r).unwrap()).collect();

    c.bench_function(&format!("db_decrypt_fields_in_batch ({})", BATCH_SIZE), |b| {
        b.iter(|| {
            let _decrypted = encryption::decrypt_fields_in_batch(
                black_box(&records),
                black_box(&FIELDS_TO_ENCRYPT.iter().map(|&s| s.to_string()).collect::<Vec<String>>()),
                black_box(&key)
            );
        })
    });
    
    println!("Number of records processed for decryption: {}", records_count);
}

fn bench_db_query(c: &mut Criterion) {
    sanity_check();
    let conn = DB_CONN.lock().unwrap();

    // Fetch records once, outside the benchmark loop
    let mut stmt = conn.prepare(&format!("SELECT data FROM plain_records ORDER BY RANDOM() LIMIT {}", BATCH_SIZE)).unwrap();
    let records: Vec<String> = stmt.query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    let record_count = records.len();
    println!("Number of records queried: {}", record_count);

    c.bench_function(&format!("db_query ({})", record_count), |b| {
        b.iter(|| {
            black_box(&records);
        })
    });
}

/*
fn bench_db_encrypt_fields_in_batches(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    c.bench_function("db_encrypt_fields_in_batches", |b| {
        b.iter(|| {
            let mut stmt = conn.prepare("SELECT data FROM plain_records ORDER BY RANDOM() LIMIT 1000000").unwrap();
            let mut records = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
            
            while let Some(batch) = records.by_ref().take(10000).collect::<Result<Vec<_>, _>>().ok() {
                let batch: Vec<serde_json::Value> = batch.iter().map(|r| serde_json::from_str(r).unwrap()).collect();
                let _encrypted = encryption::encrypt_fields_in_batch(black_box(&batch), black_box(&FIELDS_TO_ENCRYPT.iter().map(|&s| s.to_string()).collect::<Vec<String>>()), black_box(&key)).unwrap();
            }
        })
    });
}
*/

fn bench_db_ffi_encrypt_fields_in_batch(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    let mut stmt = conn.prepare(&format!("SELECT data FROM plain_records LIMIT {}", BATCH_SIZE)).unwrap();
    let records: Vec<String> = stmt.query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    let records_count = records.len();
    let records_json = serde_json::to_string(&records).unwrap();
    let records_cstring = CString::new(records_json).unwrap();
    let fields = serde_json::to_string(&FIELDS_TO_ENCRYPT).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    c.bench_function(&format!("db_ffi_encrypt_fields_in_batch ({})", BATCH_SIZE), |b| {
        b.iter(|| {
            let result = ffi::encrypt_fields_in_batch(
                black_box(records_cstring.as_ptr()),
                black_box(fields_cstring.as_ptr()),
                black_box(key.as_ptr()),
            );
            ffi::free_ffi_result(result);
        })
    });
    
    println!("Number of records processed for FFI encryption: {}", records_count);
}

fn bench_db_ffi_decrypt_fields_in_batch(c: &mut Criterion) {
    let conn = DB_CONN.lock().unwrap();
    let key = [0u8; 32];

    let mut stmt = conn.prepare(&format!("SELECT data FROM encrypted_records LIMIT {}", BATCH_SIZE)).unwrap();
    let records: Vec<String> = stmt.query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    let records_count = records.len();
    let records_json = serde_json::to_string(&records).unwrap();
    let records_cstring = CString::new(records_json).unwrap();
    let fields = serde_json::to_string(&FIELDS_TO_ENCRYPT).unwrap();
    let fields_cstring = CString::new(fields).unwrap();

    // First, encrypt the data
    let encrypted = ffi::encrypt_fields_in_batch(
        records_cstring.as_ptr(),
        fields_cstring.as_ptr(),
        key.as_ptr(),
    );

    c.bench_function(&format!("db_ffi_decrypt_fields_in_batch ({})", BATCH_SIZE), |b| {
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
    
    println!("Number of records processed for FFI decryption: {}", records_count);
}

// Modify the criterion_group! macro to include the new benchmarks
criterion_group!(
    benches,
    bench_db_query,
    bench_db_ffi_encrypt_fields_in_batch,
    bench_db_ffi_decrypt_fields_in_batch,
    bench_db_encrypt_fields_in_batch,
    bench_db_decrypt_fields_in_batch,
);

criterion_main!(benches);
