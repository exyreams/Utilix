use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Struct to perform number base conversions.
pub struct NumberBaseConverter {
    pub input: String,
    pub base_from: u32,
    pub base_to: u32,
    pub result: String,
    pub binary_to_decimal: String,
    pub binary_to_hexadecimal: String,
    pub decimal_to_binary: String,
    pub decimal_to_hexadecimal: String,
    pub hexadecimal_to_binary: String,
    pub hexadecimal_to_decimal: String,
}

impl NumberBaseConverter {
    /// Creates a new instance of `NumberBaseConverter`.
    pub fn new() -> Self {
        NumberBaseConverter {
            input: String::new(),
            base_from: 10,
            base_to: 2,
            result: String::new(),
            binary_to_decimal: String::new(),
            binary_to_hexadecimal: String::new(),
            decimal_to_binary: String::new(),
            decimal_to_hexadecimal: String::new(),
            hexadecimal_to_binary: String::new(),
            hexadecimal_to_decimal: String::new(),
        }
    }

    /// Performs the number base conversion based on `base_from` and `base_to`.
    pub fn convert(&mut self) {
        self.result = match (self.base_from, self.base_to) {
            (2, 10) => self.binary_to_decimal(),
            (2, 16) => self.binary_to_hexadecimal(),
            (10, 2) => self.decimal_to_binary(),
            (10, 16) => self.decimal_to_hexadecimal(),
            (16, 2) => self.hexadecimal_to_binary(),
            (16, 10) => self.hexadecimal_to_decimal(),
            _ => "Unsupported conversion".to_string(),
        };

        // Perform conversions between various number bases (regardless of the initial conversion).
        self.binary_to_decimal = self.binary_to_decimal();
        self.binary_to_hexadecimal = self.binary_to_hexadecimal();
        self.decimal_to_binary = self.decimal_to_binary();
        self.decimal_to_hexadecimal = self.decimal_to_hexadecimal();
        self.hexadecimal_to_binary = self.hexadecimal_to_binary();
        self.hexadecimal_to_decimal = self.hexadecimal_to_decimal();
    }

    /// Converts a binary number to decimal.
    fn binary_to_decimal(&self) -> String {
        u64::from_str_radix(&self.input, 2)
            .map(|n| n.to_string())
            .unwrap_or_else(|_| "Invalid binary number".to_string())
    }

    /// Converts a binary number to hexadecimal.
    fn binary_to_hexadecimal(&self) -> String {
        u64::from_str_radix(&self.input, 2)
            .map(|n| format!("{:X}", n))
            .unwrap_or_else(|_| "Invalid binary number".to_string())
    }

    /// Converts a decimal number to binary.
    fn decimal_to_binary(&self) -> String {
        self.input
            .parse::<u64>()
            .map(|n| format!("{:b}", n))
            .unwrap_or_else(|_| "Invalid decimal number".to_string())
    }

    /// Converts a decimal number to hexadecimal.
    fn decimal_to_hexadecimal(&self) -> String {
        self.input
            .parse::<u64>()
            .map(|n| format!("{:X}", n))
            .unwrap_or_else(|_| "Invalid decimal number".to_string())
    }

    /// Converts a hexadecimal number to binary.
    fn hexadecimal_to_binary(&self) -> String {
        u64::from_str_radix(&self.input, 16)
            .map(|n| format!("{:b}", n))
            .unwrap_or_else(|_| "Invalid hexadecimal number".to_string())
    }

    /// Converts a hexadecimal number to decimal.
    fn hexadecimal_to_decimal(&self) -> String {
        u64::from_str_radix(&self.input, 16)
            .map(|n| n.to_string())
            .unwrap_or_else(|_| "Invalid hexadecimal number".to_string())
    }

    /// Exports the conversion results and create a file.
    pub fn write_to_file(&self) -> std::io::Result<()> {
        let file_path = Path::new("export/number_conversion.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;
        writeln!(file, "Input: {}", self.input)?;
        writeln!(file, "From Base: {}", self.base_from)?;
        writeln!(file, "To Base: {}", self.base_to)?;
        writeln!(file, "Result: {}", self.result)?;
        writeln!(file, "Binary to Decimal: {}", self.binary_to_decimal)?;
        writeln!(
            file,
            "Binary to Hexadecimal: {}",
            self.binary_to_hexadecimal
        )?;
        writeln!(file, "Decimal to Binary: {}", self.decimal_to_binary)?;
        writeln!(
            file,
            "Decimal to Hexadecimal: {}",
            self.decimal_to_hexadecimal
        )?;
        writeln!(
            file,
            "Hexadecimal to Binary: {}",
            self.hexadecimal_to_binary
        )?;
        writeln!(
            file,
            "Hexadecimal to Decimal: {}",
            self.hexadecimal_to_decimal
        )?;
        Ok(())
    }
}
