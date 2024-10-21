use aes::Aes256;
use cbc::cipher::{KeyIvInit, BlockEncryptMut, BlockDecryptMut};
use cipher::block_padding::Pkcs7;
use rand::Rng;
use crate::error::PolyCryptError;
use crate::Logger;
use serde_json::{json, Value};
use base64;
use log::debug;

const AES_BLOCK_SIZE: usize = 16;

pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "encryption"}));
    logger.info("Starting encryption", Some(json!({"plaintext_length": plaintext.len()})));

    let mut rng = rand::thread_rng();
    let mut iv = [0u8; AES_BLOCK_SIZE];
    rng.fill(&mut iv);
    logger.info("IV generated", None);

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

    logger.info("Encryption completed", Some(json!({
        "plaintext_length": plaintext.len(),
        "ciphertext_length": result.len()
    })));

    // If using zeroize: buffer.zeroize();

    Ok(result)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "decryption"}));
    logger.info("Starting decryption", Some(json!({"ciphertext_length": ciphertext.len()})));

    if ciphertext.len() < AES_BLOCK_SIZE {
        return Err(PolyCryptError::DecryptionError("Ciphertext too short".to_string()));
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

    logger.info("Decryption completed", Some(json!({
        "ciphertext_length": ciphertext.len(),
        "plaintext_length": buffer.len()
    })));

    Ok(buffer)
}

pub fn decrypt_fields(record: &Value, fields_to_decrypt: &[String], key: &[u8; 32]) -> Result<Value, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "decrypt_fields"}));
    logger.info("Starting field decryption", Some(json!({"fields": fields_to_decrypt})));

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

pub fn encrypt_fields(record: &Value, fields_to_encrypt: &[String], key: &[u8; 32]) -> Result<Value, PolyCryptError> {
    let logger = Logger::new(json!({"operation": "encrypt_fields"}));
    logger.info("Starting field encryption", Some(json!({"fields": fields_to_encrypt})));

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
