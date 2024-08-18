use uuid::Uuid;

pub struct UuidGenerator {
    pub generated_uuid: String,
}

impl UuidGenerator {
    pub fn new() -> Self {
        UuidGenerator {
            generated_uuid: String::new(),
        }
    }

    pub fn generate_uuid(&mut self) {
        self.generated_uuid = Uuid::new_v4().to_string();
    }
}
