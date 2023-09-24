use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct TelluricRingDetails {
    pub body_type: CelestialBodySubType,
}

impl TelluricRingDetails {
    /// Creates a new [TelluricRingDetails].
    pub fn new(body_type: CelestialBodySubType) -> Self {
        Self { body_type }
    }
}
