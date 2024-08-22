use rand::thread_rng;
use rand::Rng;

use std::collections::HashSet;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Struct to generate secure random passwords with customizable options.
pub struct PasswordGenerator {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
    pub use_similar_characters: bool,
    pub use_duplicate_characters: bool,
    pub use_sequential_characters: bool,
    pub similar_characters: HashSet<char>,
    pub generated_password: String,
    pub quantity: usize,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        PasswordGenerator {
            length: 12,
            use_uppercase: true,
            use_lowercase: true,
            use_numbers: true,
            use_symbols: true,
            use_similar_characters: false,
            use_duplicate_characters: false,
            use_sequential_characters: false,
            similar_characters: "ilLo0O".chars().collect(),
            generated_password: String::new(),
            quantity: 1,
        }
    }
}

impl PasswordGenerator {
    /// Creates a new instance of `PasswordGenerator` with default settings.
    pub fn new() -> Self {
        Default::default()
    }

    /// Generates a single password based on the current settings.
    pub fn generate_password(&mut self) -> Result<(), String> {
        let mut rng = thread_rng();
        let mut charset: Vec<char> = Vec::new();

        if self.use_uppercase {
            charset.extend("ABCDEFGHIJKLMNPQRSTUVWXYZ".chars());
        }
        if self.use_lowercase {
            charset.extend("abcdefghjkmnpqrstuvwxyz".chars());
        }
        if self.use_numbers {
            charset.extend("23456789".chars());
        }
        if self.use_symbols {
            charset.extend("!@#$%^&*()_+-=[]{}|;:,.<>?".chars());
        }

        if charset.is_empty() {
            return Err("Charset is empty".to_string());
        }

        // Generate password
        self.generated_password = (0..self.length)
            .map(|_| {
                // Generate random index within the charset
                let mut idx = rng.gen_range(0..charset.len());
                let mut char_to_add = charset[idx];

                // If duplicate characters are not allowed, loop until a unique character is found
                if !self.use_duplicate_characters {
                    while self.generated_password.contains(char_to_add) {
                        idx = rng.gen_range(0..charset.len());
                        char_to_add = charset[idx];
                    }
                }

                // If sequential characters are not allowed, loop until a non-sequential character is found
                if !self.use_sequential_characters && !self.generated_password.is_empty() {
                    let last_char = self.generated_password.chars().last().unwrap();
                    while (char_to_add as i32 == last_char as i32 + 1)
                        || (char_to_add as i32 == last_char as i32 - 1)
                    {
                        idx = rng.gen_range(0..charset.len());
                        char_to_add = charset[idx];
                    }
                }

                // If similar characters are not allowed, loop until a dissimilar character is found
                if !self.use_similar_characters {
                    while self.similar_characters.contains(&char_to_add) {
                        idx = rng.gen_range(0..charset.len());
                        char_to_add = charset[idx];
                    }
                }
                // Return the chosen character
                char_to_add
            })
            .collect();

        Ok(())
    }

    pub fn increase_length(&mut self) {
        self.length += 1;
    }

    pub fn decrease_length(&mut self) {
        if self.length > 1 {
            self.length -= 1;
        }
    }

    pub fn clear_password(&mut self) {
        self.generated_password.clear();
    }

    pub fn toggle_uppercase(&mut self) {
        self.use_uppercase = !self.use_uppercase;
    }

    pub fn toggle_lowercase(&mut self) {
        self.use_lowercase = !self.use_lowercase;
    }

    pub fn toggle_numbers(&mut self) {
        self.use_numbers = !self.use_numbers;
    }

    pub fn toggle_symbols(&mut self) {
        self.use_symbols = !self.use_symbols;
    }

    pub fn toggle_similar_characters(&mut self) {
        self.use_similar_characters = !self.use_similar_characters;
    }

    pub fn toggle_duplicate_characters(&mut self) {
        self.use_duplicate_characters = !self.use_duplicate_characters;
    }

    pub fn toggle_sequential_characters(&mut self) {
        self.use_sequential_characters = !self.use_sequential_characters;
    }

    pub fn generate_multiple_passwords(&mut self) -> Result<Vec<String>, String> {
        let mut passwords = Vec::with_capacity(self.quantity);

        for _ in 0..self.quantity {
            self.clear_password();
            self.generate_password()?;
            passwords.push(format!("{}\n", self.generated_password));
        }

        Ok(passwords)
    }

    pub fn increase_quantity(&mut self) {
        self.quantity = self.quantity.saturating_add(1);
    }

    pub fn decrease_quantity(&mut self) {
        self.quantity = self.quantity.saturating_sub(1).max(1);
    }

    /// Exports the generated password to a file.
    pub fn write_to_file(&self) -> std::io::Result<()> {
        let file_path = Path::new("export/password.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;
        writeln!(file, "Password:")?;
        writeln!(file, "{}", self.generated_password)?;
        Ok(())
    }
}
