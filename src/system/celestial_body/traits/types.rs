use crate::internal::*;
use crate::prelude::*;
use std::fmt;
use std::fmt::Display;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricVolatileDensityDifference {
    Poor,
    #[default]
    Rich,
}

impl Display for TelluricVolatileDensityDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricVolatileDensityDifference::Poor => write!(f, "Volatile Poor"),
            TelluricVolatileDensityDifference::Rich => write!(f, "Volatile Rich"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricRotationDifference {
    Slow,
    #[default]
    Fast,
    Retrograde,
    /// The world rotates three times for every two orbits around its main body.
    Resonant,
}

impl Display for TelluricRotationDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricRotationDifference::Slow => write!(f, "Slow Rotation"),
            TelluricRotationDifference::Fast => write!(f, "Fast Rotation"),
            TelluricRotationDifference::Retrograde => write!(f, "Retrograde Rotation"),
            TelluricRotationDifference::Resonant => write!(f, "3:2 Spin-Orbit Resonance"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TideLockTarget {
    /// The object is tide-locked to what it's orbiting
    #[default]
    Orbited,
    /// The object is tide-locked to its primary satellite
    Satellite,
}

impl Display for TideLockTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TideLockTarget::Orbited => write!(f, "to orbited"),
            TideLockTarget::Satellite => write!(f, "to main satellite"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricAxialTiltDifference {
    Minimal,
    #[default]
    Extreme,
}

impl Display for TelluricAxialTiltDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricAxialTiltDifference::Minimal => write!(f, "Minimal Axial Tilt"),
            TelluricAxialTiltDifference::Extreme => write!(f, "Extreme Axial Tilt"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricCoreDifference {
    Coreless,
    #[default]
    Smaller,
    Larger,
}

impl Display for TelluricCoreDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricCoreDifference::Coreless => write!(f, "Coreless"),
            TelluricCoreDifference::Smaller => write!(f, "Smaller Core"),
            TelluricCoreDifference::Larger => write!(f, "Larger Core"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricGeologicActivity {
    GeologicallyDead,
    #[default]
    GeologicallyExtinct,
    GeologicallyActive,
}

impl Display for TelluricGeologicActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricGeologicActivity::GeologicallyDead => write!(f, "Geologically Dead"),
            TelluricGeologicActivity::GeologicallyExtinct => write!(f, "Geologically Extinct"),
            TelluricGeologicActivity::GeologicallyActive => write!(f, "Geologically Active"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricMagneticFieldDifference {
    MuchWeaker,
    #[default]
    Weaker,
    Stronger,
    MuchStronger,
}

impl Display for TelluricMagneticFieldDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricMagneticFieldDifference::MuchWeaker => write!(f, "Much Weaker Magnetic Field"),
            TelluricMagneticFieldDifference::Weaker => write!(f, "Weaker Magnetic Field"),
            TelluricMagneticFieldDifference::Stronger => write!(f, "Stronger Magnetic Field"),
            TelluricMagneticFieldDifference::MuchStronger => {
                write!(f, "Much Stronger Magnetic Field")
            }
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricTerrainRelief {
    #[default]
    FlatTerrain,
    VariedTerrain,
    /// This planet has an equatorial ridge that follows its equator almost perfectly,
    /// either in its entirety or spanning more than half of the length of the equator.
    EquatorialRidge,
}

impl Display for TelluricTerrainRelief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricTerrainRelief::FlatTerrain => write!(f, "Flat Terrain"),
            TelluricTerrainRelief::VariedTerrain => write!(f, "Varied Terrain"),
            TelluricTerrainRelief::EquatorialRidge => write!(f, "Equatorial Ridge"),
        }
    }
}
