use crate::{LogLevel, Logger};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    DecodeError, Engine as _,
};
use std::str;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Base64 decode error")]
    DecodeError(#[from] base64::DecodeError),
    #[error("UTF-8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
}

pub struct Encoder {
    engine: general_purpose::GeneralPurpose,
    logger: Logger,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            engine: general_purpose::STANDARD,
            logger: Logger::new("Encoder".to_string(), LogLevel::Info),
        }
    }

    pub fn encode(&self, data: &[u8]) -> String {
        self.engine.encode(data)
    }

    pub fn decode(&self, data: &str) -> Result<Vec<u8>, DecodeError> {
        self.engine.decode(data)
    }

    pub fn encode_mod_string(&self, data: &[u16]) -> String {
        let mod_string = self.create_mod_string(data);
        let encoded = self.engine.encode(mod_string.as_bytes());
        self.logger
            .log_default(&format!("Encoded mod string: {}", encoded));
        encoded
    }

    pub fn decode_mod_string(&self, data: String) -> Result<Vec<u16>, EncodingError> {
        let binary_data = self.engine.decode(data).map_err(|e| e)?;
        let decoded = str::from_utf8(&binary_data).map_err(|e| e)?;
        let decoded_data = decoded
            .split("&")
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        self.logger
            .log_default(&format!("Decoded mod string: {:?}", decoded_data));
        Ok(decoded_data)
    }

    fn create_mod_string(&self, data: &[u16]) -> String {
        // Assuming our data is a vector of u16 values
        // We need to take all these indices and append them to a string, with a delimiter, so we can later decode it
        let mod_string = data
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("&");
        self.logger
            .log_default(&format!("Created mod string: {}", mod_string));
        mod_string
    }
}

pub fn decode_mod_string(mod_string: &str) -> Option<String> {
    let binary_data = general_purpose::STANDARD.decode(mod_string).ok()?;

    match str::from_utf8(&binary_data) {
        Ok(string) => Some(string.to_string()),
        Err(_) => None,
    }
}
