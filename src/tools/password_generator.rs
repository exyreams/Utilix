use rand::thread_rng;
use rand::Rng;

use std::collections::HashSet;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Struct to generate secure random passwords with customizable options.
pub struct PasswordGenerator {
    /// The desired length of the generated password.
    pub length: usize,
    /// Whether to include uppercase letters.
    pub use_uppercase: bool,
    /// Whether to include lowercase letters.
    pub use_lowercase: bool,
    /// Whether to include numbers.
    pub use_numbers: bool,
    /// Whether to include symbols.
    pub use_symbols: bool,
    /// Whether to exclude similar characters (e.g., 'i', 'l', '1').
    pub use_similar_characters: bool,
    /// Whether to allow duplicate characters.
    pub use_duplicate_characters: bool,
    /// Whether to allow sequential characters (e.g., 'abc').
    pub use_sequential_characters: bool,
    /// A set of characters considered similar and excluded by default.
    pub similar_characters: HashSet<char>,
    /// The generated password string.
    pub generated_password: String,
    /// The number of passwords to generate (for multiple passwords).
    pub quantity: usize,
    /// A potential message for tools export.
    pub tools_export_message: Option<String>,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        PasswordGenerator {
            length: 12,                                     // Default password length is 12
            use_uppercase: true,                            // Include uppercase by default
            use_lowercase: true,                            // Include lowercase by default
            use_numbers: true,                              // Include numbers by default
            use_symbols: true,                              // Include symbols by default
            use_similar_characters: false,                  // Exclude similar characters by default
            use_duplicate_characters: false, // Exclude duplicate characters by default
            use_sequential_characters: false, // Exclude sequential characters by default
            similar_characters: "ilLo0O".chars().collect(), // Set of characters considered similar
            generated_password: String::new(), // The generated password is initially empty
            quantity: 1,                     // Generate only one password by default
            tools_export_message: None,      // No export message initially
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

        // Build the charset based on user settings.
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

        // Check if the charset is empty. If so, return an error.
        if charset.is_empty() {
            return Err("Charset is empty".to_string());
        }

        // Generate the password.
        self.generated_password = (0..self.length)
            .map(|_| {
                // Generate a random index within the charset.
                let mut idx = rng.gen_range(0..charset.len());
                let mut char_to_add = charset[idx];

                // If duplicate characters are not allowed, loop until a unique character is found.
                if !self.use_duplicate_characters {
                    while self.generated_password.contains(char_to_add) {
                        idx = rng.gen_range(0..charset.len());
                        char_to_add = charset[idx];
                    }
                }

                // If sequential characters are not allowed, loop until a non-sequential character is found.
                if !self.use_sequential_characters && !self.generated_password.is_empty() {
                    let last_char = self.generated_password.chars().last().unwrap();
                    while (char_to_add as i32 == last_char as i32 + 1)
                        || (char_to_add as i32 == last_char as i32 - 1)
                    {
                        idx = rng.gen_range(0..charset.len());
                        char_to_add = charset[idx];
                    }
                }

                // If similar characters are not allowed, loop until a dissimilar character is found.
                if !self.use_similar_characters {
                    while self.similar_characters.contains(&char_to_add) {
                        idx = rng.gen_range(0..charset.len());
                        char_to_add = charset[idx];
                    }
                }

                // Return the chosen character.
                char_to_add
            })
            .collect();

        Ok(())
    }

    /// Increases the password length by one.
    pub fn increase_length(&mut self) {
        self.length += 1;
    }

    /// Decreases the password length by one (minimum length is 1).
    pub fn decrease_length(&mut self) {
        if self.length > 1 {
            self.length -= 1;
        }
    }

    /// Clears the generated password.
    pub fn clear_password(&mut self) {
        self.generated_password.clear();
    }

    /// Toggles the use of uppercase letters in the password.
    pub fn toggle_uppercase(&mut self) {
        self.use_uppercase = !self.use_uppercase;
    }

    /// Toggles the use of lowercase letters in the password.
    pub fn toggle_lowercase(&mut self) {
        self.use_lowercase = !self.use_lowercase;
    }

    /// Toggles the use of numbers in the password.
    pub fn toggle_numbers(&mut self) {
        self.use_numbers = !self.use_numbers;
    }

    /// Toggles the use of symbols in the password.
    pub fn toggle_symbols(&mut self) {
        self.use_symbols = !self.use_symbols;
    }

    /// Toggles the exclusion of similar characters (e.g., 'i', 'l', '1').
    pub fn toggle_similar_characters(&mut self) {
        self.use_similar_characters = !self.use_similar_characters;
    }

    /// Toggles the allowance of duplicate characters in the password.
    pub fn toggle_duplicate_characters(&mut self) {
        self.use_duplicate_characters = !self.use_duplicate_characters;
    }

    /// Toggles the allowance of sequential characters (e.g., 'abc').
    pub fn toggle_sequential_characters(&mut self) {
        self.use_sequential_characters = !self.use_sequential_characters;
    }

    /// Generates multiple passwords based on the current settings.
    pub fn generate_multiple_passwords(&mut self) -> Result<Vec<String>, String> {
        let mut passwords = Vec::with_capacity(self.quantity);

        for _ in 0..self.quantity {
            self.clear_password();
            self.generate_password()?;
            passwords.push(format!("{}\n", self.generated_password));
        }

        Ok(passwords)
    }

    /// Increases the quantity of passwords to generate.
    pub fn increase_quantity(&mut self) {
        self.quantity = self.quantity.saturating_add(1);
    }

    /// Decreases the quantity of passwords to generate (minimum quantity is 1).
    pub fn decrease_quantity(&mut self) {
        self.quantity = self.quantity.saturating_sub(1).max(1);
    }

    /// Exports the generated password to a file.
    pub fn write_to_file(&self) -> std::io::Result<()> {
        // Create the "export" directory if it doesn't exist.
        let file_path = Path::new("export/password.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }

        // Open the file for writing.
        let mut file = File::create(file_path)?;

        // Write the generated password to the file.
        writeln!(file, "Password:")?;
        writeln!(file, "{}", self.generated_password)?;

        // Return Ok(()) to indicate success.
        Ok(())
    }
}
