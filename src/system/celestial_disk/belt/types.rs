use crate::internal::*;
use crate::prelude::*;
use std::fmt;

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
    GasBelt,
}

impl Display for CelestialBeltType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialBeltType::Dust => "Dust",
                CelestialBeltType::Meteoroid => "Meteoroid",
                CelestialBeltType::Ore => "Ore",
                CelestialBeltType::Debris => "Debris",
                CelestialBeltType::Asteroid => "Asteroid",
                CelestialBeltType::Ash => "Ash",
                CelestialBeltType::Frost => "Frost",
                CelestialBeltType::Comet => "Comet",
                CelestialBeltType::GasBelt => "Gas",
            }
        )
    }
}
