use crate::internal::*;
use crate::prelude::*;
use std::fmt;

/// A list of settings used to configure the the Telluric Bodies (like rocky planets) generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct TelluricBodySettings {}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricBodyComposition {
    Metallic,
    #[default]
    Rocky,
    Icy,
}

impl Display for TelluricBodyComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TelluricBodyComposition::Metallic => "Metallic",
                TelluricBodyComposition::Rocky => "Rocky",
                TelluricBodyComposition::Icy => "Icy",
            }
        )
    }
}

/// Peculiarities a Gas Giant might have.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TelluricSpecialTrait {
    /// This body has the exact traits that one might expect for a member of its type and subtype.
    #[default]
    NoPeculiarity,
    UnusualVolatileDensity(TelluricVolatileDensityDifference),
    UnusualRotation(TelluricRotationDifference),
    UnusualAxialTilt(TelluricAxialTiltDifference),
    UnusualCore(TelluricCoreDifference),
    SpecificGeologicActivity(TelluricGeologicActivity),
    UnusualMagneticField(TelluricMagneticFieldDifference),
    SpecificTerrainRelief(TelluricTerrainRelief),
}

impl Display for TelluricSpecialTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricSpecialTrait::NoPeculiarity => write!(f, "No Peculiarity"),
            TelluricSpecialTrait::UnusualVolatileDensity(s) => write!(f, "{}", s),
            TelluricSpecialTrait::UnusualRotation(s) => write!(f, "{}", s),
            TelluricSpecialTrait::UnusualAxialTilt(s) => write!(f, "{}", s),
            TelluricSpecialTrait::UnusualCore(s) => write!(f, "{}", s),
            TelluricSpecialTrait::SpecificGeologicActivity(s) => write!(f, "{}", s),
            TelluricSpecialTrait::UnusualMagneticField(s) => write!(f, "{}", s),
            TelluricSpecialTrait::SpecificTerrainRelief(s) => write!(f, "{}", s),
        }
    }
}

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
}

impl Display for TelluricRotationDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricRotationDifference::Slow => write!(f, "Slow Rotation"),
            TelluricRotationDifference::Fast => write!(f, "Fast Rotation"),
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
}

impl Display for TelluricTerrainRelief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelluricTerrainRelief::FlatTerrain => write!(f, "Flat Terrain"),
            TelluricTerrainRelief::VariedTerrain => write!(f, "Varied Terrain"),
        }
    }
}
