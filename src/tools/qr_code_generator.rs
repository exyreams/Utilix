use image::Luma;
use qrcode::render::unicode;
use qrcode::QrCode;
use std::path::Path;

/// Struct to generate QR codes from an input string.
pub struct QRCodeGenerator {
    pub input: String,
    pub qr_code: Option<QrCode>,
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
        self.qr_code = QrCode::new(self.input.as_bytes()).ok();
    }

    /// Exports the generated QR code as a PNG image.
    pub fn export_qr_code(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(qr_code) = &self.qr_code {
            let export_dir = "export/";

            std::fs::create_dir_all(export_dir)?;

            let truncated_input = self.input.chars().take(10).collect::<String>();
            let filename = format!("{}.png", truncated_input.replace(" ", "_").to_lowercase());

            let filepath = Path::new(export_dir).join(filename);

            let image = qr_code.render::<Luma<u8>>().build();

            image.save(filepath)?;

            Ok(())
        } else {
            Err("QR code has not been generated yet".into())
        }
    }

    /// Returns the QR code as a string of unicode characters, suitable for rendering in a terminal.
    pub fn get_qr_string(&self) -> String {
        match &self.qr_code {
            Some(qr) => qr.render::<unicode::Dense1x2>().quiet_zone(false).build(),
            None => String::from(
                "No QR code generated yet, Please enter the data in the input field to create one.",
            ),
        }
    }
}
