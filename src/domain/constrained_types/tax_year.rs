use std::fmt;
use chrono::Datelike;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaxYear(String);

impl TaxYear {
    /// Returns the tax year string.
    pub fn value(&self) -> &String {
        &self.0
    }

    /// Validates the tax year format and rules.
    fn is_valid_tax_year(tax_year: &str) -> Result<(), String> {
        // Split the input into two parts.
        let parts: Vec<&str> = tax_year.split('/').collect();
        // Check that there are exactly two parts and that each part has exactly 4 characters.
        if parts.len() != 2 || parts[0].len() != 4 || parts[1].len() != 4 {
            return Err("Invalid format. Tax year must be in YYYY/YYYY format.".to_string());
        }

        let start_year = parts[0]
            .parse::<i32>()
            .map_err(|_| "Invalid start year in the tax year.".to_string())?;
        let end_year = parts[1]
            .parse::<i32>()
            .map_err(|_| "Invalid end year in the tax year.".to_string())?;

        // Ensure consecutive years.
        if end_year != start_year + 1 {
            return Err(format!(
                "Invalid tax year. The second year must be the first year + 1. Provided: {}/{}",
                start_year, end_year
            ));
        }

        // Get the current year.
        let current_year = chrono::Utc::now().year();

        // Ensure tax year is the present or future.
        if start_year < current_year {
            return Err(format!(
                "Invalid tax year. Tax year cannot be in the past. Provided: {}/{}",
                start_year, end_year
            ));
        }

        Ok(())
    }
}

impl fmt::Display for TaxYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for TaxYear {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        TaxYear::is_valid_tax_year(trimmed)?;
        Ok(TaxYear(trimmed.to_string()))
    }
}

