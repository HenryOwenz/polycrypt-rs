use polycrypt_rs::crypto::encryption;
use polycrypt_rs::PolyCryptError;
use serde_json::json;

#[test]
fn test_encryption() {
    let plaintext = b"Hello, world!";
    let key = [0u8; 32]; // Use a proper key in real scenarios

    let encrypted = encryption::encrypt(plaintext, &key).unwrap();

    assert_ne!(encrypted, plaintext);

    // Check that the encrypted data is at least as long as the plaintext + IV
    assert!(encrypted.len() >= plaintext.len() + 16);

    // Check that the encrypted data is a multiple of the block size (16 bytes for AES)
    assert_eq!(encrypted.len() % 16, 0);

    let decrypted = encryption::decrypt(&encrypted, &key).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_field_encryption() {
    let key = [0u8; 32]; // Use a proper key in real scenarios
    let record = json!({
        "id": "1234",
        "name": "John Doe",
        "sensitive_data": "This is sensitive information",
        "array_field": ["item1", "item2", "item3"]
    });
    let fields_to_encrypt = vec!["sensitive_data".to_string(), "array_field".to_string()];

    let encrypted_record = encryption::encrypt_fields(&record, &fields_to_encrypt, &key).unwrap();

    // Check that specified fields are encrypted
    assert_ne!(encrypted_record["sensitive_data"], record["sensitive_data"]);
    assert_ne!(encrypted_record["array_field"], record["array_field"]);

    // Check that non-specified fields remain unchanged
    assert_eq!(encrypted_record["id"], record["id"]);
    assert_eq!(encrypted_record["name"], record["name"]);

    let decrypted_record = encryption::decrypt_fields(&encrypted_record, &fields_to_encrypt, &key).unwrap();
    assert_eq!(decrypted_record, record);
}

#[test]
fn test_encryption_error_handling() {
    let plaintext = b"Hello, world!";
    let mut key = [0u8; 32];
    key[..31].copy_from_slice(&[1u8; 31]); // Fill first 31 bytes with 1s

    // Create a wrapper function that accepts &[u8] instead of &[u8; 32]
    fn encrypt_wrapper(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, PolyCryptError> {
        if key.len() != 32 {
            return Err(PolyCryptError::InvalidKeyError("Key must be 32 bytes long".to_string()));
        }
        let key_array: [u8; 32] = key.try_into().unwrap();
        encryption::encrypt(plaintext, &key_array)
    }

    // Test with invalid key length
    let result = encrypt_wrapper(plaintext, &key[..31]);
    assert!(result.is_err());

    // Test with valid key length (should succeed)
    let result = encrypt_wrapper(plaintext, &key);
    assert!(result.is_ok());
}
