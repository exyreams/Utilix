use image::Luma;
use qrcode::render::unicode;
use qrcode::QrCode;
use std::path::Path;

pub struct QRCodeGenerator {
    pub input: String,
    pub qr_code: Option<QrCode>,
}

impl QRCodeGenerator {
    pub fn new() -> Self {
        QRCodeGenerator {
            input: String::new(),
            qr_code: None,
        }
    }

    pub fn generate_qr_code(&mut self) {
        self.qr_code = QrCode::new(self.input.as_bytes()).ok();
    }

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

    pub fn get_qr_string(&self) -> String {
        match &self.qr_code {
            Some(qr) => qr.render::<unicode::Dense1x2>().quiet_zone(false).build(),
            None => String::from("No QR code generated yet"),
        }
    }
}
