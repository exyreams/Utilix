use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use std::i32;

/// Struct for converting dates between different formats.
pub struct DateConverter {
    pub input: String,
    pub rfc3339: String,
    pub rfc2822: String,
    pub iso8601: String,
    pub unix_timestamp: String,
    pub human_readable: String,
    pub short_date: String,
    pub time_only: String,
}

impl DateConverter {
    /// Creates a new instance of `DateConverter`.
    pub fn new() -> Self {
        DateConverter {
            input: String::new(),
            rfc3339: String::new(),
            rfc2822: String::new(),
            iso8601: String::new(),
            unix_timestamp: String::new(),
            human_readable: String::new(),
            short_date: String::new(),
            time_only: String::new(),
        }
    }

    /// Convert the `input` date string to all supported formats.
    pub fn convert_all(&mut self) {
        let parsed_datetime = self.parse_input();
        match parsed_datetime {
            Ok(datetime) => self.convert_from_datetime(datetime),
            Err(err) => self.set_all_to(&err),
        }
    }

    /// Parses the `input` string into a `DateTime<Utc>` object.
    /// Returns an error message if parsing fails.
    fn parse_input(&self) -> Result<DateTime<Utc>, String> {
        if let Ok(timestamp) = self.input.parse::<i64>() {
            if timestamp < i32::MIN as i64 || timestamp > i32::MAX as i64 {
                return Err("Timestamp out of supported range".to_string());
            }
            return Utc
                .timestamp_opt(timestamp, 0)
                .single()
                .ok_or_else(|| "Invalid timestamp".to_string());
        }

        let formats = [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%dT%H:%M:%S%:z",
            "%d/%m/%Y %H:%M:%S",
            "%Y-%m-%d",
            "%d/%m/%Y",
        ];

        for format in &formats {
            if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(&self.input, format) {
                let year = naive_datetime.year();
                if year < 1 || year > 9999 {
                    return Err("Year out of supported range (1-9999)".to_string());
                }
                return Ok(Utc.from_utc_datetime(&naive_datetime));
            }
        }

        // If parsing as a full date-time fails, try parsing as a date-only
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(&self.input, "%Y-%m-%d") {
            let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
            let year = naive_datetime.year();
            if year < 1 || year > 9999 {
                return Err("Year out of supported range (1-9999)".to_string());
            }
            return Ok(Utc.from_utc_datetime(&naive_datetime));
        }

        Err("Unrecognized date-time format".to_string())
    }

    /// Converts `DateTime<Utc>` object to all supported formats.
    fn convert_from_datetime(&mut self, datetime: DateTime<Utc>) {
        self.convert_to_rfc3339(datetime);
        self.convert_to_rfc2822(datetime);
        self.convert_to_iso8601(datetime);
        self.convert_to_unixtimestamp(datetime);
        self.convert_to_humanreadable(datetime);
        self.convert_to_shortdate(datetime);
        self.convert_to_timeonly(datetime);
    }

    /// Converts the given datetime to RFC3339 format.
    fn convert_to_rfc3339(&mut self, datetime: DateTime<Utc>) {
        self.rfc3339 = datetime.to_rfc3339();
    }

    /// Converts the given datetime to RFC2822 format.
    fn convert_to_rfc2822(&mut self, datetime: DateTime<Utc>) {
        self.rfc2822 = datetime.to_rfc2822();
    }

    /// Converts the given datetime to ISO8601 format.
    fn convert_to_iso8601(&mut self, datetime: DateTime<Utc>) {
        self.iso8601 = datetime.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    }

    /// Converts the given datetime to Unix timestamp (integer).
    fn convert_to_unixtimestamp(&mut self, datetime: DateTime<Utc>) {
        self.unix_timestamp = datetime.timestamp().to_string();
    }

    /// Converts the given datetime to a human-readable format.
    fn convert_to_humanreadable(&mut self, datetime: DateTime<Utc>) {
        self.human_readable = datetime.format("%A, %B %d, %Y %I:%M:%S %p").to_string();
    }

    /// Converts the given datetime to a short date format.
    fn convert_to_shortdate(&mut self, datetime: DateTime<Utc>) {
        self.short_date = datetime.format("%d/%m/%Y").to_string();
    }

    /// Converts the given datetime to a time-only format.
    fn convert_to_timeonly(&mut self, datetime: DateTime<Utc>) {
        self.time_only = datetime.format("%H:%M:%S").to_string();
    }

    /// Sets all output fields to the provided error message.
    fn set_all_to(&mut self, message: &str) {
        self.rfc3339 = message.to_string();
        self.rfc2822 = message.to_string();
        self.iso8601 = message.to_string();
        self.unix_timestamp = message.to_string();
        self.human_readable = message.to_string();
        self.short_date = message.to_string();
        self.time_only = message.to_string();
    }
}
