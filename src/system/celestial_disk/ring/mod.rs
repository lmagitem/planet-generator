use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialRingDetails {
    /// Specific ring details
    pub level: CelestialRingLevel,
    /// What the ring is made of
    pub composition: CelestialRingComposition,
}

impl CelestialRingDetails {
    /// Creates a new [CelestialRingDetails].
    pub fn new(level: CelestialRingLevel, composition: CelestialRingComposition) -> Self {
        Self { level, composition }
    }
}
