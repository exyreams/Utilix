use uuid::Uuid;

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Struct to generate Universally Unique Identifiers (UUIDs) in version 4.
pub struct UuidGenerator {
    pub generated_uuid: String,
    pub length: usize,
    pub tools_export_message: Option<String>,
}

impl UuidGenerator {
    /// Creates a new instance of `UuidGenerator`.
    pub fn new() -> Self {
        UuidGenerator {
            generated_uuid: String::new(),
            length: 1,
            tools_export_message: None,
        }
    }

    /// Generates a single version 4 UUID.
    pub fn generate_v4_uuid(&mut self) {
        self.generated_uuid = Uuid::new_v4().to_string();
    }

    /// Generates multiple version 4 UUIDs.
    pub fn generate_multiple_v4_uuids(&mut self) {
        self.generated_uuid.clear();
        for i in 0..self.length {
            self.generated_uuid.push_str(&Uuid::new_v4().to_string());
            if i < self.length - 1 {
                self.generated_uuid.push('\n');
            }
        }
    }

    pub fn increase_length(&mut self) {
        self.length += 1;
    }

    pub fn decrease_length(&mut self) {
        if self.length > 1 {
            self.length -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.generated_uuid.clear();
        self.length = 0;
    }

    /// Exports the generated UUIDs to a file.
    pub fn write_to_file(&self) -> std::io::Result<()> {
        let file_path = Path::new("export/uuid.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;
        writeln!(file, "UUIDs:")?;
        writeln!(file, "{}", self.generated_uuid)?;
        Ok(())
    }
}
