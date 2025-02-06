use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use std::str;

pub fn encode_mod_string(mod_string: &str) -> String {
    general_purpose::STANDARD.encode(mod_string)
}

pub fn decode_mod_string(mod_string: &str) -> Option<String> {
    let binary_data = general_purpose::STANDARD.decode(mod_string).ok()?;

    match str::from_utf8(&binary_data) {
        Ok(string) => Some(string.to_string()),
        Err(_) => None,
    }
}
