use uuid::Uuid;
pub struct UuidGenerator {
    pub generated_uuid: String,
    pub length: usize,
}

impl UuidGenerator {
    pub fn new() -> Self {
        UuidGenerator {
            generated_uuid: String::new(),
            length: 1,
        }
    }

    pub fn generate_v4_uuid(&mut self) {
        self.generated_uuid = Uuid::new_v4().to_string();
    }

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
}
