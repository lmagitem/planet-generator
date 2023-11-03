use crate::prelude::*;
use crate::internal::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialBeltDetails {
    /// What kind of belt it is
    pub composition: CelestialBeltType,
}

impl CelestialBeltDetails {
    /// Creates a new [CelestialBeltDetails].
    pub fn new(composition: CelestialBeltComposition) -> Self {
        Self { composition }
    }
}
