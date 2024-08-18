use sha2::{Digest, Sha256};

pub struct HashGenerator {
    pub input: String,
    pub hash: String,
}

impl HashGenerator {
    pub fn new() -> Self {
        HashGenerator {
            input: String::new(),
            hash: String::new(),
        }
    }

    pub fn generate_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.input.as_bytes());
        self.hash = format!("{:x}", hasher.finalize());
    }
}
