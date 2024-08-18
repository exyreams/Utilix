use rand::Rng;

pub struct PasswordGenerator {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
    pub generated_password: String,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        PasswordGenerator {
            length: 12,
            use_uppercase: true,
            use_lowercase: true,
            use_numbers: true,
            use_symbols: true,
            generated_password: String::new(),
        }
    }

    pub fn generate_password(&mut self) {
        let mut rng = rand::thread_rng();
        let mut charset = String::new();

        if self.use_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.use_lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.use_numbers {
            charset.push_str("0123456789");
        }
        if self.use_symbols {
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }

        if charset.is_empty() {
            self.generated_password = String::new();
            return;
        }

        self.generated_password = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();
    }

    pub fn increase_length(&mut self) {
        self.length += 1;
    }

    pub fn decrease_length(&mut self) {
        if self.length > 1 {
            self.length -= 1;
        }
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
}
