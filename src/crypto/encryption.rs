use crate::error::PolyCryptError;
use crate::Logger;
use aes::Aes256;
use base64;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cipher::block_padding::Pkcs7;
use log::debug;
use rand::Rng;
use serde_json::{json, Value};

const AES_BLOCK_SIZE: usize = 16;

// comment out info logging for encrypt/decrypt since it is called a lot and logging is expensive and not useful for production
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "encryption"}));
    // logger.info("Starting encryption", Some(json!({"plaintext_length": plaintext.len()})));

    let mut rng = rand::thread_rng();
    let mut iv = [0u8; AES_BLOCK_SIZE];
    rng.fill(&mut iv);
    // logger.info("IV generated", None);

    let cipher = cbc::Encryptor::<Aes256>::new(key.into(), &iv.into());
    let mut buffer = vec![0u8; plaintext.len() + AES_BLOCK_SIZE];
    let ciphertext_len = cipher
        .encrypt_padded_b2b_mut::<Pkcs7>(plaintext, &mut buffer)
        .map_err(|e| {
            logger.error("Encryption failed", Some(json!({"error": e.to_string()})));
            PolyCryptError::EncryptionError(e.to_string())
        })?
        .len();

    let mut result = Vec::with_capacity(iv.len() + ciphertext_len);
    result.extend_from_slice(&iv);
    result.extend_from_slice(&buffer[..ciphertext_len]);

    /*
    logger.info("Encryption completed", Some(json!({
        "plaintext_length": plaintext.len(),
        "ciphertext_length": result.len()
    })));
    */

    // If using zeroize: buffer.zeroize();

    Ok(result)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "decryption"}));
    // logger.info("Starting decryption", Some(json!({"ciphertext_length": ciphertext.len()})));

    if ciphertext.len() < AES_BLOCK_SIZE {
        return Err(PolyCryptError::DecryptionError(
            "Ciphertext too short".to_string(),
        ));
    }

    let (iv, ciphertext) = ciphertext.split_at(AES_BLOCK_SIZE);
    let cipher = cbc::Decryptor::<Aes256>::new(key.into(), iv.into());
    let mut buffer = ciphertext.to_vec();
    let plaintext_len = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|e| {
            logger.error("Decryption failed", Some(json!({"error": e.to_string()})));
            PolyCryptError::DecryptionError(e.to_string())
        })?
        .len();

    buffer.truncate(plaintext_len);

    /*
    logger.info("Decryption completed", Some(json!({
        "ciphertext_length": ciphertext.len(),
        "plaintext_length": buffer.len()
    })));
    */

    Ok(buffer)
}

pub fn decrypt_fields(
    record: &Value,
    fields_to_decrypt: &[String],
    key: &[u8; 32],
) -> Result<Value, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "decrypt_fields"}));
    logger.info(
        "Starting field decryption",
        Some(json!({"fields": fields_to_decrypt})),
    );

    let mut decrypted_record = record.clone();

    for field in fields_to_decrypt {
        if let Some(encrypted_value) = record.get(field) {
            debug!("Decrypting field: {}", field);
            let decrypted_value = if encrypted_value.is_array() {
                let array = encrypted_value.as_array().unwrap();
                let decrypted_array: Result<Vec<String>, PolyCryptError> = array
                    .iter()
                    .map(|item| {
                        let ciphertext = base64::decode(item.as_str().unwrap())?;
                        let decrypted = decrypt(&ciphertext, key)?;
                        Ok(String::from_utf8(decrypted)?)
                    })
                    .collect();
                Value::Array(decrypted_array?.into_iter().map(Value::String).collect())
            } else {
                let ciphertext = base64::decode(encrypted_value.as_str().unwrap())?;
                let decrypted = decrypt(&ciphertext, key)?;
                Value::String(String::from_utf8(decrypted)?)
            };
            decrypted_record[field] = decrypted_value;
        }
    }

    logger.info("Field decryption completed", None);
    Ok(decrypted_record)
}

pub fn encrypt_fields(
    record: &Value,
    fields_to_encrypt: &[String],
    key: &[u8; 32],
) -> Result<Value, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "encrypt_fields"}));
    logger.info(
        "Starting field encryption",
        Some(json!({"fields": fields_to_encrypt})),
    );

    let mut encrypted_record = record.clone();

    for field in fields_to_encrypt {
        if let Some(plaintext_value) = record.get(field) {
            let encrypted_value = if plaintext_value.is_array() {
                let array = plaintext_value.as_array().unwrap();
                let encrypted_array: Result<Vec<String>, PolyCryptError> = array
                    .iter()
                    .map(|item| {
                        let plaintext = item.as_str().unwrap().as_bytes();
                        let encrypted = encrypt(plaintext, key)?;
                        Ok(base64::encode(encrypted))
                    })
                    .collect();
                Value::Array(encrypted_array?.into_iter().map(Value::String).collect())
            } else {
                let plaintext = plaintext_value.as_str().unwrap().as_bytes();
                let encrypted = encrypt(plaintext, key)?;
                Value::String(base64::encode(encrypted))
            };
            encrypted_record[field] = encrypted_value;
        }
    }

    logger.info("Field encryption completed", None);
    Ok(encrypted_record)
}

pub fn decrypt_fields_in_batch(
    records: &[Value],
    fields_to_decrypt: &[String],
    key: &[u8; 32],
) -> Result<Vec<Value>, PolyCryptError> {
    records
        .iter()
        .map(|record| decrypt_fields(record, fields_to_decrypt, key))
        .collect()
}

pub fn encrypt_fields_in_batch(
    records: &[Value],
    fields_to_encrypt: &[String],
    key: &[u8; 32],
) -> Result<Vec<Value>, PolyCryptError> {
    records
        .iter()
        .map(|record| encrypt_fields(record, fields_to_encrypt, key))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = b"Hello, world!";
        let key = [0u8; 32]; // Use a fixed key for testing

        let encrypted = encrypt(plaintext, &key).unwrap();
        assert_ne!(encrypted, plaintext);

        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_fields() {
        let key = [0u8; 32]; // Use a fixed key for testing
        let record = json!({
            "id": "1234",
            "name": "John Doe",
            "sensitive_data": "This is sensitive information",
            "array_field": ["item1", "item2", "item3"]
        });
        let fields_to_encrypt = vec!["sensitive_data".to_string(), "array_field".to_string()];

        let encrypted_record = encrypt_fields(&record, &fields_to_encrypt, &key).unwrap();
        assert_ne!(encrypted_record["sensitive_data"], record["sensitive_data"]);
        assert_ne!(encrypted_record["array_field"], record["array_field"]);
        assert_eq!(encrypted_record["id"], record["id"]);
        assert_eq!(encrypted_record["name"], record["name"]);

        let decrypted_record = decrypt_fields(&encrypted_record, &fields_to_encrypt, &key).unwrap();
        assert_eq!(decrypted_record, record);
    }

    #[test]
    fn test_encryption_error() {
        let plaintext = b"Hello, world!";
        let mut key = [0u8; 32];
        key[..31].copy_from_slice(&[1u8; 31]); // Fill first 31 bytes with 1s

        // Create a wrapper function that accepts &[u8] instead of &[u8; 32]
        fn encrypt_wrapper(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, PolyCryptError> {
            if key.len() != 32 {
                return Err(PolyCryptError::InvalidKeyError(
                    "Key must be 32 bytes long".to_string(),
                ));
            }
            let key_array: [u8; 32] = key.try_into().unwrap();
            encrypt(plaintext, &key_array)
        }

        // Test with invalid key length
        let result = encrypt_wrapper(plaintext, &key[..31]);
        assert!(result.is_err());

        // Test with valid key length (should succeed)
        let result = encrypt_wrapper(plaintext, &key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decryption_error() {
        let invalid_ciphertext = vec![0u8; 15]; // Too short for valid ciphertext
        let key = [0u8; 32];

        let result = decrypt(&invalid_ciphertext, &key);
        assert!(result.is_err());
    }
}
