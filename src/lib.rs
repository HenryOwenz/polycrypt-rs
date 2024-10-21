pub mod error;



pub mod logger;



pub mod crypto;



pub mod bindings;



pub use error::PolyCryptError;



pub use logger::Logger;



pub use bindings::ffi::{ByteArray, encrypt, decrypt, free_byte_array};



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








