use image::Luma;
use qrcode::render::unicode;
use qrcode::QrCode;
use std::path::Path;

/// Struct to generate QR codes from an input string.
pub struct QRCodeGenerator {
    /// The input string to be encoded into a QR code.
    pub input: String,
    /// The generated QR code, if available.
    pub qr_code: Option<QrCode>,
    /// A potential message for tools export.
    pub tools_export_message: Option<String>,
}

impl QRCodeGenerator {
    /// Creates a new instance of `QRCodeGenerator`.
    pub fn new() -> Self {
        QRCodeGenerator {
            input: String::new(),
            qr_code: None,
            tools_export_message: None,
        }
    }

    /// Generates a QR code from the input string.
    pub fn generate_qr_code(&mut self) {
        // Attempts to create a QR code from the input string and stores it in `qr_code` if successful.
        self.qr_code = QrCode::new(self.input.as_bytes()).ok();
    }

    /// Exports the generated QR code as a PNG image.
    pub fn export_qr_code(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Checks if a QR code has been generated.
        if let Some(qr_code) = &self.qr_code {
            let export_dir = "export/";

            // Creates the "export" directory if it doesn't exist.
            std::fs::create_dir_all(export_dir)?;

            // Creates a filename using the first 10 characters of the input string (truncated, sanitized).
            let truncated_input = self.input.chars().take(10).collect::<String>();
            let filename = format!("{}.png", truncated_input.replace(" ", "_").to_lowercase());

            // Creates the full file path by joining the export directory and the filename.
            let filepath = Path::new(export_dir).join(filename);

            // Renders the QR code as a PNG image.
            let image = qr_code.render::<Luma<u8>>().build();

            // Saves the QR code image to the specified file path.
            image.save(filepath)?;

            Ok(())
        } else {
            // Returns an error if a QR code has not been generated yet.
            Err("QR code has not been generated yet".into())
        }
    }

    /// Returns the QR code as a string of unicode characters, suitable for rendering in a terminal.
    pub fn get_qr_string(&self) -> String {
        match &self.qr_code {
            Some(qr) => {
                // Renders the QR code as a string of Unicode characters using the Dense1x2 style.
                qr.render::<unicode::Dense1x2>().quiet_zone(false).build()
            }
            _none => String::from(
                "No QR code generated yet, Please enter the data in the input field to create one.",
            ),
        }
    }
}
