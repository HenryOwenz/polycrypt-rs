use polycrypt_rs::crypto::encryption;

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
}
