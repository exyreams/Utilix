use base64::{engine::general_purpose, Engine as _};

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Struct to encapsulate base64 encoding and decoding functionality.
pub struct Base64Encoder {
    pub input: String,
    pub encoded: String,
    pub decoded: String,
    pub tools_export_message: Option<String>,
}

impl Base64Encoder {
    /// Creates a new instance of `Base64Encoder`.
    pub fn new() -> Self {
        Base64Encoder {
            input: String::new(),
            encoded: String::new(),
            decoded: String::new(),
            tools_export_message: None,
        }
    }

    /// Encodes the `input` string using base64.
    pub fn encode(&mut self) {
        self.encoded = general_purpose::STANDARD.encode(&self.input);
    }

    /// Decodes the `input` string from base64.
    pub fn decode(&mut self) {
        match general_purpose::STANDARD.decode(&self.input) {
            Ok(decoded_bytes) => {
                self.decoded = String::from_utf8_lossy(&decoded_bytes).to_string();
            }
            Err(_) => {
                self.decoded = "provided input is not a valid base64 string.".to_string();
            }
        }
    }

    /// Export the encoded and decoded strings to a file.
    pub fn write_to_file(&mut self) -> std::io::Result<()> {
        let file_path = Path::new("export/base64.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;
        
        writeln!(file, "Input: {}", self.input)?;
        writeln!(file, "Encoded: {}", self.encoded)?;
        writeln!(file, "Decoded: {}", self.decoded)?;

        Ok(())
    }
}
