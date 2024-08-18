use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

pub struct DateConverter {
    pub input: String,
    pub output: String,
}

impl DateConverter {
    pub fn new() -> Self {
        DateConverter {
            input: String::new(),
            output: String::new(),
        }
    }

    pub fn convert_date(&mut self) {
        if let Ok(timestamp) = self.input.parse::<i64>() {
            if let Some(datetime) = Utc.timestamp_opt(timestamp, 0).single() {
                self.output = datetime.to_rfc3339();
            } else {
                self.output = "Invalid timestamp".to_string();
            }
        } else if let Ok(naive_datetime) =
            NaiveDateTime::parse_from_str(&self.input, "%Y-%m-%d %H:%M:%S")
        {
            let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);
            self.output = datetime.timestamp().to_string();
        } else {
            self.output = "Invalid input format".to_string();
        }
    }
}
