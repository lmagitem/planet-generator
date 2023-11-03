use crate::prelude::*;
use crate::internal::*;

/// A list of settings used to configure the Belts generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct CelestialBeltSettings {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CelestialBeltType {
    // Metallics
    Dust,
    Meteoroid,
    Ore,
    // Rocky
    Debris,
    Asteroid,
    Ash,
    // Icy
    Frost,
    Comet,
    // Gaseous
    GasClouds,
    GasBelt,
}
