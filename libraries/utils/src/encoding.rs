use crate::{LogLevel, Logger};
use base64::{engine::general_purpose, DecodeError, Engine as _};
use std::str;
use thiserror::Error;

/// Enum representing possible encoding errors.
#[derive(Error, Debug)]
pub enum EncodingError {
    /// Error occurring during Base64 decoding.
    #[error("Base64 decode error")]
    DecodeError(#[from] base64::DecodeError),
    /// Error occurring during UTF-8 conversion.
    #[error("UTF-8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
}

/// Struct to handle encoding and decoding operations.
pub struct Encoder {
    /// Base64 encoding engine.
    engine: general_purpose::GeneralPurpose,
    /// Logger instance for logging encoding operations.
    logger: Logger,
}

impl Encoder {
    /// Creates a new `Encoder` instance.
    ///
    /// # Returns
    ///
    /// A new `Encoder` instance with a default logger.
    pub fn new() -> Self {
        Self {
            engine: general_purpose::STANDARD,
            logger: Logger::new("Encoder".to_string(), LogLevel::Info),
        }
    }

    /// Encodes the given data to a Base64 string.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice representing the data to encode.
    ///
    /// # Returns
    ///
    /// A `String` containing the Base64 encoded data.
    pub fn encode(&self, data: &[u8]) -> String {
        self.engine.encode(data)
    }

    /// Decodes the given Base64 string to a vector of bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - A `&str` representing the Base64 encoded data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of bytes or a `DecodeError`.
    pub fn decode(&self, data: &str) -> Result<Vec<u8>, DecodeError> {
        self.engine.decode(data)
    }

    /// Encodes a vector of `u16` values to a Base64 string.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of `u16` values to encode.
    ///
    /// # Returns
    ///
    /// A `String` containing the Base64 encoded data.
    pub fn encode_mod_string(&self, data: &[u16]) -> String {
        let mod_string = self.create_mod_string(data);
        let encoded = self.engine.encode(mod_string.as_bytes());
        self.logger
            .log_default(&format!("Encoded mod string: {}", encoded));
        encoded
    }

    /// Decodes a Base64 string to a vector of `u16` values.
    ///
    /// # Arguments
    ///
    /// * `data` - A `String` representing the Base64 encoded data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `u16` values or an `EncodingError`.
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

    /// Creates a mod string from a slice of `u16` values.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of `u16` values.
    ///
    /// # Returns
    ///
    /// A `String` containing the mod string.
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

/// Decodes a Base64 mod string to an optional `String`.
///
/// # Arguments
///
/// * `mod_string` - A `&str` representing the Base64 encoded mod string.
///
/// # Returns
///
/// An `Option` containing the decoded `String` or `None` if decoding fails.
pub fn decode_mod_string(mod_string: &str) -> Option<String> {
    let binary_data = general_purpose::STANDARD.decode(mod_string).ok()?;

    match str::from_utf8(&binary_data) {
        Ok(string) => Some(string.to_string()),
        Err(_) => None,
    }
}
