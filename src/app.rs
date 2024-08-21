use crate::tools::{
    base64_encoder::Base64Encoder, date_converter::DateConverter, hash_generator::HashGenerator,
    number_base_converter::NumberBaseConverter, password_generator::PasswordGenerator,
    qr_code_generator::QRCodeGenerator, uuid_generator::UuidGenerator,
};

#[derive(PartialEq)]
pub enum Tool {
    Base64Encoder,
    DateConverter,
    HashGenerator,
    NumberBaseConverter,
    PasswordGenerator,
    QRCodeGenerator,
    UuidGenerator,
}

pub struct App {
    pub current_tool: Tool,
    pub base64_encoder: Base64Encoder,
    pub date_converter: DateConverter,
    pub hash_generator: HashGenerator,
    pub number_base_converter: NumberBaseConverter,
    pub password_generator: PasswordGenerator,
    pub qr_code_generator: QRCodeGenerator,
    pub uuid_generator: UuidGenerator,
}

impl App {
    pub fn new() -> App {
        App {
            current_tool: Tool::Base64Encoder,
            base64_encoder: Base64Encoder::new(),
            date_converter: DateConverter::new(),
            hash_generator: HashGenerator::new(),
            number_base_converter: NumberBaseConverter::new(),
            password_generator: PasswordGenerator::new(),
            qr_code_generator: QRCodeGenerator::new(),
            uuid_generator: UuidGenerator::new(),
        }
    }
}
