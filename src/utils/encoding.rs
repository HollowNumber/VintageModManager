use crate::utils::{LogLevel, Logger};
use base85::{decode, encode};
use brotli::{CompressorWriter, Decompressor};
use std::io::{Read, Write};
use std::{io, str};
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EncoderData {
    pub mod_id: String,
    pub mod_version: String,
}

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Decoding error: {0}")]
    Decode(String),
    #[error("Decompression error: {0}")]
    Decompress(String),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl From<io::Error> for EncodingError {
    fn from(error: io::Error) -> Self {
        EncodingError::Decompress(error.to_string())
    }
}

/// Struct to handle encoding and decoding operations.
pub struct Encoder {
    /// Logger instance for logging encoding operations.
    logger: Logger,
}

impl Encoder {
    /// Creates a new `Encoder` instance.
    ///
    /// # Returns
    ///
    /// A new `Encoder` instance with a default logger.
    pub fn new(verbose: bool) -> Self {
        Self {
            logger: Logger::new("Encoder".to_string(), LogLevel::Info, None, verbose),
        }
    }

    /// Encodes the given data to a base85 string.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice representing the data to encode.
    ///
    /// # Returns
    ///
    /// A `String` containing the base85 encoded data.
    pub fn encode(&self, data: &[u8]) -> String {
        let encoded = encode(data);
        self.logger
            .log_default(&format!("Encoding using `encode` function: {encoded}"));
        encoded
    }

    /// Decodes the given base85 string to a vector of bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - A `&str` representing the base85 encoded data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of bytes or a `EncodingError`.
    pub fn decode(&self, data: &str) -> Result<Vec<u8>, EncodingError> {
        self.logger
            .log_default(&format!("Decoding using `decode` function: {data}"));

        decode(data).map_err(|e| EncodingError::Decode(e.to_string()))
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
        let mod_string = self.format_encoder_data(mods);
        self.logger
            .log_default(&format!("Mod string before encoding: {mod_string}"));

        // Compress
        let compressed = self.compress(&mod_string).unwrap();
        let encoded = self.encode(&compressed);
        self.logger
            .log_default(&format!("Encoded mod string: {encoded}"));
        encoded
    }

    /// Formats a list of `EncoderData` to a compact string.
    ///
    /// The format is `mod_id|mod_version;mod_id|mod_version;...`.
    ///
    /// Example: `foo|1.10;bar|4.0;foobar|1.0.1`.
    ///
    /// # Arguments
    ///
    /// * `mods` - A slice of `EncoderData` representing the mods to encode.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted data.
    ///
    /// # Example
    ///
    /// ```
    /// use utils::encoding::Encoder;
    /// use utils::encoding::EncoderData;
    ///
    /// let encoder = Encoder::new(false);
    ///
    /// let mod_: EncoderData = EncoderData {
    ///     mod_id: "foo".to_string(),
    ///     mod_version: "1.10".to_string(),
    ///    };
    ///
    /// let formatted = encoder.format_encoder_data(&[mod_]);
    /// assert_eq!(formatted, "foo|1.10");
    ///```
    ///
    fn format_encoder_data(&self, mods: &[EncoderData]) -> String {
        mods.iter()
            .map(|mod_info| format!("{}|{}", mod_info.mod_id, mod_info.mod_version))
            .collect::<Vec<String>>()
            .join(";")
    }

    /// Decodes a compact string to a list of `EncoderData`.
    ///
    /// # Arguments
    ///
    /// * `data` - A `String` representing the compact encoded data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `EncoderData` or an EncodingError.
    pub fn decode_mod_string(&self, data: String) -> Result<Vec<EncoderData>, EncodingError> {
        let binary_data = self.decode(&data)?;
        let decompressed = self.decompress(&binary_data)?;

        let mods: Result<Vec<EncoderData>, EncodingError> = decompressed
            .split(';')
            .map(|mod_info| {
                let parts: Vec<&str> = mod_info.split('|').collect();
                if parts.len() != 2 {
                    return Err(EncodingError::Decode(
                        "Invalid mod string format".to_string(),
                    ));
                }
                Ok(EncoderData {
                    mod_id: parts[0].to_string(),
                    mod_version: parts[1].to_string(),
                })
            })
            .collect();

        self.logger
            .log_default(&format!("Decoded mod string: {mods:?}"));
        mods
    }

    /// Decompresses the data using Brotli decompression.
    pub fn decompress(&self, data: &[u8]) -> io::Result<String> {
        let mut decoder = Decompressor::new(data, 4096);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        let decompressed_string = String::from_utf8(decompressed_data).unwrap();
        Ok(decompressed_string)
    }

    /// Compresses the data using Brotli compression.
    pub fn compress(&self, data: &str) -> io::Result<Vec<u8>> {
        let mut encoder = CompressorWriter::new(Vec::new(), 4096, 11, 22);
        encoder.write_all(data.as_bytes())?;
        let compressed_data = encoder.into_inner();
        self.logger
            .log_default(&format!("Compressed data: {compressed_data:?}"));
        Ok(compressed_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_base85() {
        let encoder = Encoder::new(false);
        let data = b"hello";
        let encoded = encoder.encode(data);
        assert_eq!(encoded, "Xk~0{Zv");
    }

    #[test]
    fn decode_base85() {
        let encoder = Encoder::new(false);
        let data = "Xk~0{Zv";
        let decoded = encoder.decode(data);
        assert!(decoded.is_ok(), "Decoding failed: {:?}", decoded.err());
        assert_eq!(decoded.unwrap(), b"hello");
    }

    #[test]
    fn encode_mod_string() {
        let encoder = Encoder::new(false);
        let mods = vec![
            EncoderData {
                mod_id: "foo".to_string(),
                mod_version: "1.10".to_string(),
            },
            EncoderData {
                mod_id: "bar".to_string(),
                mod_version: "2.0".to_string(),
            },
        ];
        let encoded = encoder.encode_mod_string(&mods);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn decode_mod_string() {
        let encoder = Encoder::new(false);
        let data = encoder.encode_mod_string(&[
            EncoderData {
                mod_id: "foo".to_string(),
                mod_version: "1.10".to_string(),
            },
            EncoderData {
                mod_id: "bar".to_string(),
                mod_version: "2.0".to_string(),
            },
        ]);
        let decoded = encoder.decode_mod_string(data).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].mod_id, "foo");
        assert_eq!(decoded[0].mod_version, "1.10");
        assert_eq!(decoded[1].mod_id, "bar");
        assert_eq!(decoded[1].mod_version, "2.0");
    }

    #[test]
    fn format_encoder_data() {
        let encoder = Encoder::new(false);
        let mods = vec![
            EncoderData {
                mod_id: "foo".to_string(),
                mod_version: "1.10".to_string(),
            },
            EncoderData {
                mod_id: "bar".to_string(),
                mod_version: "2.0".to_string(),
            },
        ];
        let formatted = encoder.format_encoder_data(&mods);
        assert_eq!(formatted, "foo|1.10;bar|2.0");
    }

    #[test]
    fn decompress_data() {
        let encoder = Encoder::new(false);
        let data = encoder.compress("hello").unwrap();
        let decompressed = encoder.decompress(&data).unwrap();
        assert_eq!(decompressed, "hello");
    }

    #[test]
    fn compress_data() {
        let encoder = Encoder::new(false);
        let data = "hello";
        let compressed = encoder.compress(data).unwrap();
        assert!(!compressed.is_empty());
    }

    #[test]
    fn decode_mod_string_with_invalid_data() {
        let encoder = Encoder::new(false);
        let data = "invalid_data";
        let result = encoder.decode_mod_string(data.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn format_empty_encoder_data() {
        let encoder = Encoder::new(false);
        let mods: Vec<EncoderData> = vec![];
        let formatted = encoder.format_encoder_data(&mods);
        assert_eq!(formatted, "");
    }
}
