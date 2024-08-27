use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

/// Struct to generate Universally Unique Identifiers (UUIDs) in version 4 and 7.
pub struct UuidGenerator {
    /// The generated version 4 UUID.
    pub generated_uuid_v4: String,
    /// The generated version 7 UUID.
    pub generated_uuid_v7: String,
    /// The number of UUIDs to generate.
    pub length: usize,
    /// A potential message for tools export.
    pub tools_export_message: Option<String>,
}

impl UuidGenerator {
    /// Creates a new instance of `UuidGenerator`.
    pub fn new() -> Self {
        UuidGenerator {
            generated_uuid_v4: String::new(),
            generated_uuid_v7: String::new(),
            length: 1,
            tools_export_message: None,
        }
    }

    /// Generates a single version 4 UUID.
    pub fn generate_v4_uuid(&mut self) {
        self.generated_uuid_v4 = Uuid::new_v4().to_string();
    }

    /// Generates multiple version 4 UUIDs.
    pub fn generate_multiple_v4_uuids(&mut self) {
        // Generate a specified number of v4 UUIDs and join them with newline separators.
        self.generated_uuid_v4 = (0..self.length)
            .map(|_| Uuid::new_v4().to_string())
            .collect::<Vec<_>>()
            .join("\n");
    }

    /// Generates a single version 7 UUID.
    pub fn generate_v7_uuid(&mut self) {
        self.generated_uuid_v7 = Uuid::now_v7().to_string();
    }

    /// Generates multiple version 7 UUIDs.
    pub fn generate_multiple_v7_uuids(&mut self) {
        // Generate a specified number of v7 UUIDs and join them with newline separators.
        self.generated_uuid_v7 = (0..self.length)
            .map(|_| Uuid::now_v7().to_string())
            .collect::<Vec<_>>()
            .join("\n");
    }

    /// Increases the number of UUIDs to generate.
    pub fn increase_length(&mut self) {
        self.length += 1;
    }

    /// Decreases the number of UUIDs to generate (minimum is 1).
    pub fn decrease_length(&mut self) {
        if self.length > 1 {
            self.length -= 1;
        }
    }

    /// Clears the generated UUIDs and resets the length to 1.
    pub fn clear(&mut self) {
        self.generated_uuid_v4.clear();
        self.generated_uuid_v7.clear();
        self.length = 1;
    }

    /// Exports the generated UUIDs to a file.
    pub fn write_to_file(&self) -> std::io::Result<()> {
        // Create the "export" directory if it doesn't exist.
        let file_path = Path::new("export/uuid.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }

        // Open the file for writing.
        let mut file = File::create(file_path)?;

        // Write the generated UUIDs to the file with headers.
        writeln!(file, "UUID v4:")?;
        writeln!(file, "{}", self.generated_uuid_v4)?;
        writeln!(file, "\nUUID v7:")?;
        writeln!(file, "{}", self.generated_uuid_v7)?;

        // Return Ok(()) to indicate success.
        Ok(())
    }
}
