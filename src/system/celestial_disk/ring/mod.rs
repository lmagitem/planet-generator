use crate::system::celestial_disk::ring::types::CelestialRingComposition;
use crate::system::celestial_disk::ring::types::CelestialRingLevel;
use serde::{Deserialize, Serialize};
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
