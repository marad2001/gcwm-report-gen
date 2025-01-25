use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NameString(String);

impl NameString {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for NameString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for NameString {
    type Error = &'static str;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        let trimmed = name.trim();

        if trimmed.is_empty() {
            return Err("A name string cannot be empty");
        }

        if trimmed.len() < 2 || trimmed.len() > 50 {
            return Err("A name must be between 2 and 50 characters long");
        }

        if !trimmed.chars().all(|c| c.is_alphabetic() || c == '-' || c == ' ') {
            return Err("A name can only contain alphabetic characters, hyphens, or spaces");
        }

        // Capitalise the first letter and lowercase the rest
        let mut chars = trimmed.chars();
        let formatted_name = match chars.next() {
            Some(first) => first.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
            None => return Err("Unexpected error while formatting the name"),
        };

        Ok(Self(formatted_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert_eq!(
            NameString::try_from("john".to_string()).unwrap().value(),
            "John"
        );
        assert_eq!(
            NameString::try_from("ANNE-MARIE".to_string()).unwrap().value(),
            "Anne-marie"
        );
        assert_eq!(
            NameString::try_from("o'connor".to_string()).unwrap().value(),
            "O'connor"
        );
    }

    #[test]
    fn test_invalid_names() {
        assert!(NameString::try_from("".to_string()).is_err());
        assert!(NameString::try_from("J".to_string()).is_err());
        assert!(NameString::try_from("John123".to_string()).is_err());
        assert!(NameString::try_from("John$#".to_string()).is_err());
        assert!(NameString::try_from("a".repeat(51)).is_err());
    }
}
