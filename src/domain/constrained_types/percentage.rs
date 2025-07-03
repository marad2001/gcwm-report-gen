use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Percentage(f32);

impl Percentage {
    /// Expect a fractional input in [0.0, 1.0].
    pub fn try_from_fraction(frac: f32) -> Result<Self, &'static str> {
        if frac < 0.0 {
            return Err("Percentage cannot be negative");
        }
        if frac > 1.0 {
            return Err("Percentage cannot exceed 1.0 (i.e. 100%)");
        }
        // round to 2dp internally if you like:
        let rounded = (frac * 100.0).round() / 100.0;
        Ok(Self(rounded))
    }

    /// Expect a percentage‐point input in [0.0, 100.0].
    pub fn try_from_percent(percent: f32) -> Result<Self, &'static str> {
        if percent < 0.0 {
            return Err("Percentage cannot be negative");
        }
        if percent > 100.0 {
            return Err("Percentage cannot exceed 100.0");
        }
        Self::try_from_fraction(percent / 100.0)
    }

    /// Get the raw **fractional** value (e.g. 0.35 for “35%”).
    pub fn as_fraction(&self) -> f32 {
        self.0
    }

    /// Get the **percent-point** value (e.g. 35.0 for “35%”).
    pub fn as_percent(&self) -> f32 {
        self.0 * 100.0
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // show as percent with two decimals
        write!(f, "{:.2}%", self.as_percent())
    }
}

/// If you really want `TryFrom<f32>` still, pick one semantic:
/// here I choose it to mean “fractional”:
impl TryFrom<f32> for Percentage {
    type Error = &'static str;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Percentage::try_from_fraction(value)
    }
}

impl TryFrom<String> for Percentage {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        // strip any trailing “%”, parse as f32 in percent-points, then delegate
        let p = s.trim()
                 .trim_end_matches('%')
                 .parse::<f32>()
                 .map_err(|_| "Invalid percentage string")?;
        Percentage::try_from_percent(p)
    }
}