use digest::Digest;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

pub struct HashGenerator {
    input: String,
    sha1_hash: String,
    sha256_hash: String,
    sha384_hash: String,
    sha512_hash: String,
}

impl HashGenerator {
    pub fn new() -> Self {
        HashGenerator {
            input: String::new(),
            sha1_hash: String::new(),
            sha256_hash: String::new(),
            sha384_hash: String::new(),
            sha512_hash: String::new(),
        }
    }

    pub fn update_input(&mut self, new_input: &str) {
        self.input = new_input.to_string();
        self.calculate_hashes();
    }

    fn calculate_hashes(&mut self) {
        self.sha1_hash = self.generate_sha1();
        self.sha256_hash = self.generate_sha256();
        self.sha384_hash = self.generate_sha384();
        self.sha512_hash = self.generate_sha512();
    }

    pub fn get_sha1(&self) -> &str {
        &self.sha1_hash
    }

    pub fn get_sha256(&self) -> &str {
        &self.sha256_hash
    }

    pub fn get_sha384(&self) -> &str {
        &self.sha384_hash
    }

    pub fn get_sha512(&self) -> &str {
        &self.sha512_hash
    }

    pub fn generate_sha1(&self) -> String {
        let mut hasher = Sha1::new();
        Digest::update(&mut hasher, self.input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn generate_sha256(&self) -> String {
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, self.input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn generate_sha384(&self) -> String {
        let mut hasher = Sha384::new();
        Digest::update(&mut hasher, self.input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn generate_sha512(&self) -> String {
        let mut hasher = Sha512::new();
        Digest::update(&mut hasher, self.input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
