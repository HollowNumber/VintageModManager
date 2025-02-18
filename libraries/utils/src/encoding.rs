use crate::{LogLevel, Logger};
use base85::{decode, encode};
use brotli::{CompressorWriter, Decompressor};
use std::io::{Read, Write};
use std::{io, str};
use thiserror::Error;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EncoderData {
    pub mod_id: String,
    pub mod_version: String,
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
        //let encoded = self.engine.encode(data);
        let encoded = encode(data);
        self.logger
            .log_default(&format!("Encoding using `encode` function: {}", encoded));
        encoded
    }

    /// Decodes the given Base64 string to a vector of bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - A `&str` representing the Base64 encoded data.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of bytes or a `String`.
    pub fn decode(&self, data: &str) -> Result<Vec<u8>, String> {
        self.logger
            .log_default(&format!("Decoding using `decode` function: {}", data));
        decode(data).map_err(|e| e.to_string())
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
            .log_default(&format!("Mod string before encoding: {}", mod_string));

        // Compress
        let compressed = self.compress(&mod_string).unwrap();
        let encoded = self.encode(&compressed);
        self.logger
            .log_default(&format!("Encoded mod string: {}", encoded));
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
    pub fn format_encoder_data(&self, mods: &[EncoderData]) -> String {
        let mod_string = mods
            .iter()
            .map(|mod_info| format!("{}|{}", mod_info.mod_id, mod_info.mod_version))
            .collect::<Vec<String>>()
            .join(";");

        mod_string
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
        let binary_data = self.decode(&data).expect("Unable to decode mod string");
        let decompressed = self.decompress(&binary_data).unwrap();

        let mods = decompressed
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
            .log_default(&format!("Compressed data: {:?}", compressed_data));
        Ok(compressed_data)
    }
}
