use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConstrainedString20(String);

impl ConstrainedString20 {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for ConstrainedString20 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ConstrainedString20 {
    type Error = &'static str;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.trim().is_empty() {
            return Err("A constrained string 20 cannot be empty");
        }

        if string.len() > 20 {
            return Err("A constrained string 20 must be shorter than 20 characters");
        }

        Ok(Self(string))
    }
}

impl TryFrom<&str> for ConstrainedString20 {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        ConstrainedString20::try_from(string.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_string() {
        let constrained = ConstrainedString20::try_from("Valid String".to_string());
        assert!(constrained.is_ok());
        assert_eq!(constrained.unwrap().value(), "Valid String");
    }

    #[test]
    fn test_valid_str() {
        let constrained = ConstrainedString20::try_from("Another Valid");
        assert!(constrained.is_ok());
        assert_eq!(constrained.unwrap().value(), "Another Valid");
    }

    #[test]
    fn test_empty_string() {
        let constrained = ConstrainedString20::try_from("".to_string());
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 20 cannot be empty"
        );
    }

    #[test]
    fn test_empty_str() {
        let constrained = ConstrainedString20::try_from("");
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 20 cannot be empty"
        );
    }

    #[test]
    fn test_too_long_string() {
        let constrained = ConstrainedString20::try_from("This string is way too long".to_string());
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 20 must be shorter than 20 characters"
        );
    }

    #[test]
    fn test_too_long_str() {
        let constrained = ConstrainedString20::try_from("Excessively Long String");
        assert!(constrained.is_err());
        assert_eq!(
            constrained.err().unwrap(),
            "A constrained string 20 must be shorter than 20 characters"
        );
    }
}
