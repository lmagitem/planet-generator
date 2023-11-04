use crate::internal::*;
use crate::prelude::*;
use std::fmt;

/// A list of settings used to configure the the Telluric Bodies (like rocky planets) generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct TelluricBodySettings {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum TelluricBodyComposition {
    Metallic,
    Rocky,
}

impl Display for TelluricBodyComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TelluricBodyComposition::Metallic => "Metallic",
                TelluricBodyComposition::Rocky => "Rocky",
            }
        )
    }
}
