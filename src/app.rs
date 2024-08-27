use crate::tools::{
    base64_encoder::Base64Encoder, color_converter::ColorConverter, date_converter::DateConverter,
    hash_generator::HashGenerator, number_base_converter::NumberBaseConverter,
    password_generator::PasswordGenerator, qr_code_generator::QRCodeGenerator,
    uuid_generator::UuidGenerator,
};

/// An enum representing the different tools available in the application.
#[derive(PartialEq)]
pub enum Tool {
    Base64Encoder,
    ColorConverter,
    DateConverter,
    HashGenerator,
    NumberBaseConverter,
    PasswordGenerator,
    QRCodeGenerator,
    UuidGenerator,
}

/// The main application struct.
pub struct App {
    /// The currently selected tool.
    pub current_tool: Tool,
    /// The Base64 encoder tool.
    pub base64_encoder: Base64Encoder,
    /// The color converter tool.
    pub color_converter: ColorConverter,
    /// The date converter tool.
    pub date_converter: DateConverter,
    /// The hash generator tool.
    pub hash_generator: HashGenerator,
    /// The number base converter tool.
    pub number_base_converter: NumberBaseConverter,
    /// The password generator tool.
    pub password_generator: PasswordGenerator,
    /// The QR code generator tool.
    pub qr_code_generator: QRCodeGenerator,
    /// The UUID generator tool.
    pub uuid_generator: UuidGenerator,
}

impl App {
    /// Creates a new instance of `App` with all tools initialized.
    pub fn new() -> App {
        App {
            current_tool: Tool::Base64Encoder, // Default to Base64Encoder tool
            base64_encoder: Base64Encoder::new(),
            color_converter: ColorConverter::new(),
            date_converter: DateConverter::new(),
            hash_generator: HashGenerator::new(),
            number_base_converter: NumberBaseConverter::new(),
            password_generator: PasswordGenerator::new(),
            qr_code_generator: QRCodeGenerator::new(),
            uuid_generator: UuidGenerator::new(),
        }
    }
}
