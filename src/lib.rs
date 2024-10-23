pub mod bindings;
pub mod crypto;
pub mod error;
pub mod logger;

pub use bindings::ffi::{decrypt, encrypt, free_ffi_result, ByteArray, FFIResult};
pub use error::PolyCryptError;
pub use logger::Logger;

pub struct PolyCrypt {
    logger: Logger,
}

impl PolyCrypt {
    pub fn new(context: serde_json::Value) -> Self {
        Self {
            logger: Logger::new(context),
        }
    }

    pub fn log_info(&self, message: &str) {
        self.logger.info(message, None);
    }
}
