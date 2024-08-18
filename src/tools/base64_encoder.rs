use base64::{engine::general_purpose, Engine as _};

pub struct Base64Encoder {
    pub input: String,
    pub encoded: String,
    pub decoded: String,
}

impl Base64Encoder {
    pub fn new() -> Self {
        Base64Encoder {
            input: String::new(),
            encoded: String::new(),
            decoded: String::new(),
        }
    }

    pub fn encode(&mut self) {
        self.encoded = general_purpose::STANDARD.encode(&self.input);
    }

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
}
