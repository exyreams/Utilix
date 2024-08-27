use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use std::i32;

/// Struct for converting dates between different formats.
pub struct DateConverter {
    /// The input date string.
    pub input: String,
    /// The converted date in RFC 3339 format.
    pub rfc3339: String,
    /// The converted date in RFC 2822 format.
    pub rfc2822: String,
    /// The converted date in ISO 8601 format.
    pub iso8601: String,
    /// The converted date as a Unix timestamp (integer).
    pub unix_timestamp: String,
    /// The converted date in a human-readable format.
    pub human_readable: String,
    /// The converted date in a short date format.
    pub short_date: String,
    /// The converted time in a time-only format.
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
        // Parse the input string into a DateTime<Utc> object.
        let parsed_datetime = self.parse_input();
        match parsed_datetime {
            Ok(datetime) => {
                // If parsing is successful, convert the datetime to all supported formats.
                self.convert_from_datetime(datetime);
            }
            Err(err) => {
                // If parsing fails, set all output fields to the error message.
                self.set_all_to(&err);
            }
        }
    }

    /// Parses the `input` string into a `DateTime<Utc>` object.
    /// Returns an error message if parsing fails.
    fn parse_input(&self) -> Result<DateTime<Utc>, String> {
        // Attempt to parse the input as a Unix timestamp (integer).
        if let Ok(timestamp) = self.input.parse::<i64>() {
            // Check if the timestamp is within the supported range (i32::MIN to i32::MAX).
            if timestamp < i32::MIN as i64 || timestamp > i32::MAX as i64 {
                return Err("Timestamp out of supported range".to_string());
            }

            // If the timestamp is valid, create a DateTime<Utc> from it.
            return Utc
                .timestamp_opt(timestamp, 0)
                .single()
                .ok_or_else(|| "Invalid timestamp".to_string());
        }

        // Define a list of supported date and time formats.
        let formats = [
            "%Y-%m-%d %H:%M:%S",    //  2024-03-22 10:00:00
            "%Y-%m-%dT%H:%M:%S%:z", // 2024-03-22T10:00:00-05:00
            "%d/%m/%Y %H:%M:%S",    // 22/03/2024 10:00:00
            "%Y-%m-%d",             // 2024-03-22
            "%d/%m/%Y",             // 22/03/2024
        ];

        // Attempt to parse the input using each supported format.
        for format in &formats {
            if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(&self.input, format) {
                let year = naive_datetime.year();
                // Check if the year is within the supported range (1-9999).
                if year < 1 || year > 9999 {
                    return Err("Year out of supported range (1-9999)".to_string());
                }

                // Create a DateTime<Utc> from the parsed NaiveDateTime.
                return Ok(Utc.from_utc_datetime(&naive_datetime));
            }
        }

        // If parsing as a full date-time fails, try parsing as a date-only format.
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(&self.input, "%Y-%m-%d") {
            let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
            let year = naive_datetime.year();
            // Check if the year is within the supported range.
            if year < 1 || year > 9999 {
                return Err("Year out of supported range (1-9999)".to_string());
            }
            return Ok(Utc.from_utc_datetime(&naive_datetime));
        }

        // If all parsing attempts fail, return an error message indicating an unrecognized format.
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
        self.human_readable = datetime.format("%A, %B %d, %Y, %I:%M:%S %p").to_string();
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
