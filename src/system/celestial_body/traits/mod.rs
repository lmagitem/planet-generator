use crate::internal::*;
use crate::prelude::*;
use std::fmt::Display;
pub mod types;

/// Peculiarities a celestial body might have.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum CelestialBodySpecialTrait {
    /// This body has the exact traits that one might expect for a member of its type and subtype.
    #[default]
    NoPeculiarity,
    /// This Gas Giant was the first to arise from its star proto-planetary disk.
    ProtoGiant,
    RetrogradeOrbit,
    SpecificGeologicActivity(TelluricGeologicActivity),
    SpecificTerrainRelief(TelluricTerrainRelief),
    TideLocked(TideLockTarget),
    UnusualVolatileDensity(TelluricVolatileDensityDifference),
    UnusualMagneticField(TelluricMagneticFieldDifference),
    UnusualAxialTilt(TelluricAxialTiltDifference),
    UnusualRotation(TelluricRotationDifference),
    UnusualCore(TelluricCoreDifference),
}

impl Display for CelestialBodySpecialTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CelestialBodySpecialTrait::NoPeculiarity => write!(f, "No Peculiarity"),
            CelestialBodySpecialTrait::ProtoGiant => write!(f, "Proto-Giant"),
            CelestialBodySpecialTrait::RetrogradeOrbit => write!(f, "Retrograde Orbit"),
            CelestialBodySpecialTrait::SpecificGeologicActivity(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::SpecificTerrainRelief(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualVolatileDensity(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualMagneticField(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualAxialTilt(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualRotation(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::UnusualCore(s) => write!(f, "{}", s),
            CelestialBodySpecialTrait::TideLocked(s) => write!(f, "Tide-Locked {}", s),
        }
    }
}
