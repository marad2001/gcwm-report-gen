use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProtectedTaxFreeCashEntitlement(f32);

impl ProtectedTaxFreeCashEntitlement {
    /// Returns the value of the entitlement as a percentage.
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl fmt::Display for ProtectedTaxFreeCashEntitlement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}%", self.0) // Displays the value as "25%", "100%", etc.
    }
}

impl TryFrom<f32> for ProtectedTaxFreeCashEntitlement {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 25.0 {
            return Err("A ProtectedTaxFreeCashEntitlement cannot be less than 25%");
        }

        if value > 100.0 {
            return Err("A ProtectedTaxFreeCashEntitlement cannot exceed 100%");
        }

        // Ensure no decimal places by checking the fractional part
        if value.fract() != 0.0 {
            return Err("A ProtectedTaxFreeCashEntitlement cannot have decimal places");
        }

        Ok(Self(value))
    }
}

impl TryFrom<String> for ProtectedTaxFreeCashEntitlement {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed_value = value
            .trim()
            .replace("%", "")
            .parse::<f32>()
            .map_err(|_| "Invalid string format for ProtectedTaxFreeCashEntitlement")?;

        ProtectedTaxFreeCashEntitlement::try_from(parsed_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_entitlements() {
        let entitlement = ProtectedTaxFreeCashEntitlement::try_from(25.0).unwrap();
        assert_eq!(entitlement.value(), 25.0);
        assert_eq!(format!("{}", entitlement), "25%");

        let entitlement = ProtectedTaxFreeCashEntitlement::try_from(100.0).unwrap();
        assert_eq!(entitlement.value(), 100.0);
        assert_eq!(format!("{}", entitlement), "100%");
    }

    #[test]
    fn test_invalid_entitlements() {
        let entitlement = ProtectedTaxFreeCashEntitlement::try_from(24.9);
        assert!(entitlement.is_err());
        assert_eq!(
            entitlement.err().unwrap(),
            "A ProtectedTaxFreeCashEntitlement cannot be less than 25%"
        );

        let entitlement = ProtectedTaxFreeCashEntitlement::try_from(100.1);
        assert!(entitlement.is_err());
        assert_eq!(
            entitlement.err().unwrap(),
            "A ProtectedTaxFreeCashEntitlement cannot exceed 100%"
        );

        let entitlement = ProtectedTaxFreeCashEntitlement::try_from(50.5);
        assert!(entitlement.is_err());
        assert_eq!(
            entitlement.err().unwrap(),
            "A ProtectedTaxFreeCashEntitlement cannot have decimal places"
        );
    }

    #[test]
    fn test_valid_string_entitlements() {
        let entitlement = ProtectedTaxFreeCashEntitlement::try_from("25%".to_string()).unwrap();
        assert_eq!(entitlement.value(), 25.0);
        assert_eq!(format!("{}", entitlement), "25%");

        let entitlement = ProtectedTaxFreeCashEntitlement::try_from("100".to_string()).unwrap();
        assert_eq!(entitlement.value(), 100.0);
        assert_eq!(format!("{}", entitlement), "100%");
    }

    #[test]
    fn test_invalid_string_entitlements() {
        let entitlement = ProtectedTaxFreeCashEntitlement::try_from("24.9%".to_string());
        assert!(entitlement.is_err());
        assert_eq!(
            entitlement.err().unwrap(),
            "A ProtectedTaxFreeCashEntitlement cannot be less than 25%"
        );

        let entitlement = ProtectedTaxFreeCashEntitlement::try_from("101%".to_string());
        assert!(entitlement.is_err());
        assert_eq!(
            entitlement.err().unwrap(),
            "A ProtectedTaxFreeCashEntitlement cannot exceed 100%"
        );

        let entitlement = ProtectedTaxFreeCashEntitlement::try_from("50.5%".to_string());
        assert!(entitlement.is_err());
        assert_eq!(
            entitlement.err().unwrap(),
            "A ProtectedTaxFreeCashEntitlement cannot have decimal places"
        );
    }
}
