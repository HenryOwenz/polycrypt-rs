use base64::DecodeError;
use std::string::FromUtf8Error;

#[derive(Debug, thiserror::Error)]
pub enum PolyCryptError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] DecodeError),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("Invalid key: {0}")]
    InvalidKeyError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}
