use crate::{LogLevel, Logger};
use base64::engine::general_purpose;
use base64::{DecodeError, Engine as _};
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

pub trait ToStringVec {
    fn to_string_vec(&self) -> Vec<String>;
}

impl ToStringVec for [u16] {
    fn to_string_vec(&self) -> Vec<String> {
        self.iter().map(|x| x.to_string()).collect()
    }
}

impl ToStringVec for [String] {
    fn to_string_vec(&self) -> Vec<String> {
        self.to_vec()
    }
}

impl ToStringVec for i32 {
    fn to_string_vec(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl ToStringVec for String {
    fn to_string_vec(&self) -> Vec<String> {
        vec![self.clone()]
    }
}

pub trait FromStringVec {
    fn from_string_vec(data: Vec<String>) -> Vec<Self>
    where
        Self: Sized;
}

impl FromStringVec for u16 {
    fn from_string_vec(data: Vec<String>) -> Vec<Self> {
        data.into_iter()
            .map(|x| x.parse::<u16>().unwrap())
            .collect()
    }
}

impl FromStringVec for String {
    fn from_string_vec(data: Vec<String>) -> Vec<Self> {
        data
    }
}

#[derive(Debug, PartialEq)]
pub struct EncoderData {
    pub mod_id: String,
    pub mod_version: String,
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
            engine: general_purpose::STANDARD_NO_PAD,
            logger: Logger::new("Encoder".to_string(), LogLevel::Info, "logs/encoder.log"),
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

    /// Encodes a list of `EncoderData` to a compact string.
    ///
    /// # Arguments
    ///
    /// * `mods` - A slice of `EncoderData` representing the mods to encode.
    ///
    /// # Returns
    ///
    /// A `String` containing the compact encoded data.
    pub fn encode_mod_string(&self, mods: &[EncoderData]) -> String {
        let mod_string = mods
            .iter()
            .map(|mod_info| format!("{}|{}", mod_info.mod_id, mod_info.mod_version))
            .collect::<Vec<String>>()
            .join(";");
        let encoded = self.engine.encode(mod_string.as_bytes());
        self.logger
            .log_default(&format!("Encoded mod string: {}", encoded));
        encoded
    }

    /// Decodes a compact string to a list of `EncoderData`.
    ///
    /// # Arguments
    ///
    /// * `data` - A `String` representing the compact encoded data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `EncoderData` or an `EncodingError`.
    pub fn decode_mod_string(&self, data: String) -> Result<Vec<EncoderData>, EncodingError> {
        let binary_data = self.engine.decode(data).map_err(|e| e)?;
        let decoded = str::from_utf8(&binary_data).map_err(|e| e)?;
        let mods = decoded
            .split(';')
            .map(|mod_info| {
                let parts: Vec<&str> = mod_info.split('|').collect();
                EncoderData {
                    mod_id: parts[0].to_string(),
                    mod_version: parts[1].to_string(),
                }
            })
            .collect();
        self.logger
            .log_default(&format!("Decoded mod string: {:?}", mods));
        Ok(mods)
    }
}
