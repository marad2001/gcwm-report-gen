use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ConstrainedString200(String);

impl ConstrainedString200 {
    pub fn value(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for ConstrainedString200 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ConstrainedString200 {
    type Error = &'static str;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.trim().is_empty() {
            return Err("A constrained string 200 cannot be empty");
        }

        if string.len() > 200 {
            return Err("A constrained string 200 must be shorter than 200 characters");
        }

        Ok(Self(string))
    }
}