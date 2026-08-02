use crate::system::celestial_disk::belt::types::CelestialBeltType;
use serde::{Deserialize, Serialize};
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialBeltDetails {
    /// What kind of belt it is
    pub composition: CelestialBeltType,
}

impl CelestialBeltDetails {
    /// Creates a new [CelestialBeltDetails].
    pub fn new(composition: CelestialBeltType) -> Self {
        Self { composition }
    }
}
