use qrcode::render::unicode;
use qrcode::QrCode;

pub struct QRCodeGenerator {
    pub input: String,
    pub qr_code: String,
}

impl QRCodeGenerator {
    pub fn new() -> Self {
        QRCodeGenerator {
            input: String::new(),
            qr_code: String::new(),
        }
    }

    pub fn generate_qr_code(&mut self) {
        if let Ok(code) = QrCode::new(self.input.as_bytes()) {
            self.qr_code = code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
        } else {
            self.qr_code = "Error generating QR code".to_string();
        }
    }
}
