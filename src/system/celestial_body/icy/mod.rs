use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct IcyBodyDetails {
    /// The type of this world.
    pub world_type: CelestialBodyWorldType,
}

impl IcyBodyDetails {
    /// Creates a new [IcyBodyDetails].
    pub fn new(world_type: CelestialBodyWorldType) -> Self {
        Self { world_type }
    }
}
