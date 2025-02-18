use std::fmt;
use chrono::{NaiveDate, ParseError, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MeetingDate(NaiveDate);

impl MeetingDate {
    /// Returns a reference to the inner date value.
    pub fn value(&self) -> &NaiveDate {
        &self.0
    }

    /// Returns the date formatted as for example "26th November"
    pub fn formatted_day_month(&self) -> String {
        let day = self.0.day();
        let suffix = match day {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        };
        let month = self.0.format("%B").to_string(); // Full month name
        format!("{}{} {}", day, suffix, month)
    }
}

impl fmt::Display for MeetingDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%d/%m/%Y"))
    }
}

/// Implements conversion from a UK formatted date string into `MeetingDate`
impl TryFrom<String> for MeetingDate {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Ensure the string is not empty
        if value.trim().is_empty() {
            return Err("Meeting date cannot be empty.".to_string());
        }

        // Attempt to parse the date in UK format (DD/MM/YYYY)
        let parsed_date = NaiveDate::parse_from_str(&value, "%d/%m/%Y")
            .map_err(|_| "Invalid date format. Expected DD/MM/YYYY.".to_string())?;

        // Validate the year must be >= 2020
        if parsed_date.year() < 2020 {
            return Err(format!(
                "Meeting date cannot be before 2020. Provided date: {}",
                parsed_date.format("%d/%m/%Y")
            ));
        }

        // If all validations pass, return the MeetingDate instance
        Ok(MeetingDate(parsed_date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_meeting_date() {
        let result = MeetingDate::try_from("15/05/2021".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_date_format() {
        let result = MeetingDate::try_from("2021-05-15".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Invalid date format. Expected DD/MM/YYYY."
        );
    }

    #[test]
    fn test_date_before_2020() {
        let result = MeetingDate::try_from("15/05/2019".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Meeting date cannot be before 2020. Provided date: 15/05/2019"
        );
    }

    #[test]
    fn test_empty_date_string() {
        let result = MeetingDate::try_from("".to_string());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Meeting date cannot be empty.");
    }
}