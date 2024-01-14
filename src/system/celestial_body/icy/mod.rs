use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct IcyBodyDetails {
    /// The type of this world.
    pub world_type: CelestialBodyWorldType,
    /// What are the pecularities of this icy body.
    pub special_traits: Vec<CelestialBodySpecialTrait>,
}

impl IcyBodyDetails {
    /// Creates a new [IcyBodyDetails].
    pub fn new(
        world_type: CelestialBodyWorldType,
        special_traits: Vec<CelestialBodySpecialTrait>,
    ) -> Self {
        Self {
            world_type,
            special_traits,
        }
    }
}
