use digest::Digest;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Struct to generate various hash values for an input string.
pub struct HashGenerator {
    /// The input string for which the hashes will be generated.
    input: String,
    /// The SHA-1 hash of the input string.
    sha1_hash: String,
    /// The SHA-256 hash of the input string.
    sha256_hash: String,
    /// The SHA-384 hash of the input string.
    sha384_hash: String,
    /// The SHA-512 hash of the input string.
    sha512_hash: String,
    /// A potential message for tools export.
    pub tools_export_message: Option<String>,
}

impl HashGenerator {
    /// Creates a new instance of `HashGenerator`.
    pub fn new() -> Self {
        HashGenerator {
            input: String::new(),
            sha1_hash: String::new(),
            sha256_hash: String::new(),
            sha384_hash: String::new(),
            sha512_hash: String::new(),
            tools_export_message: None,
        }
    }

    /// Updates the input string and calculates the hashes.
    pub fn update_input(&mut self, new_input: &str) {
        // Updates the input string.
        self.input = new_input.to_string();

        // Calculates all hash values.
        self.calculate_hashes();
    }

    /// Calculates all the hash values based on the current `input`.
    fn calculate_hashes(&mut self) {
        // Calculates and updates each hash.
        self.sha1_hash = self.generate_sha1();
        self.sha256_hash = self.generate_sha256();
        self.sha384_hash = self.generate_sha384();
        self.sha512_hash = self.generate_sha512();
    }

    /// Returns the SHA-1 hash of the input string.
    pub fn get_sha1(&self) -> &str {
        &self.sha1_hash
    }

    /// Returns the SHA-256 hash of the input string.
    pub fn get_sha256(&self) -> &str {
        &self.sha256_hash
    }

    /// Returns the SHA-384 hash of the input string.
    pub fn get_sha384(&self) -> &str {
        &self.sha384_hash
    }

    /// Returns the SHA-512 hash of the input string.
    pub fn get_sha512(&self) -> &str {
        &self.sha512_hash
    }

    /// Generates the SHA-1 hash of the input string.
    pub fn generate_sha1(&self) -> String {
        // Create a new SHA-1 hasher.
        let mut hasher = Sha1::new();

        // Update the hasher with the input string bytes.
        Digest::update(&mut hasher, self.input.as_bytes());

        // Finalize the hash calculation and return the hex representation.
        format!("{:x}", hasher.finalize())
    }

    /// Generates the SHA-256 hash of the input string.
    pub fn generate_sha256(&self) -> String {
        // Create a new SHA-256 hasher.
        let mut hasher = Sha256::new();

        // Update the hasher with the input string bytes.
        Digest::update(&mut hasher, self.input.as_bytes());

        // Finalize the hash calculation and return the hex representation.
        format!("{:x}", hasher.finalize())
    }

    /// Generates the SHA-384 hash of the input string.
    pub fn generate_sha384(&self) -> String {
        // Create a new SHA-384 hasher.
        let mut hasher = Sha384::new();

        // Update the hasher with the input string bytes.
        Digest::update(&mut hasher, self.input.as_bytes());

        // Finalize the hash calculation and return the hex representation.
        format!("{:x}", hasher.finalize())
    }

    /// Generates the SHA-512 hash of the input string.
    pub fn generate_sha512(&self) -> String {
        // Create a new SHA-512 hasher.
        let mut hasher = Sha512::new();

        // Update the hasher with the input string bytes.
        Digest::update(&mut hasher, self.input.as_bytes());

        // Finalize the hash calculation and return the hex representation.
        format!("{:x}", hasher.finalize())
    }

    /// Exports the generated hashes to a file.
    pub fn write_to_file(&mut self) -> std::io::Result<()> {
        // Create the "export" directory if it doesn't exist.
        let file_path = Path::new("export/hash.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }

        // Open the file for writing.
        let mut file = File::create(file_path)?;

        // Write the input and hash values to the file.
        writeln!(file, "Input: {}", self.input)?;
        writeln!(file, "SHA1: {}", self.sha1_hash)?;
        writeln!(file, "SHA256: {}", self.sha256_hash)?;
        writeln!(file, "SHA384: {}", self.sha384_hash)?;
        writeln!(file, "SHA512: {}", self.sha512_hash)?;

        // Return Ok(()) to indicate success.
        Ok(())
    }
}
