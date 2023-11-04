use crate::internal::*;
use crate::prelude::*;
use std::fmt;

/// A list of settings used to configure the Rings generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct CelestialRingSettings {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CelestialRingComposition {
    Ice,
    Rock,
    Metal,
    Dust,
}

impl Display for CelestialRingComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialRingComposition::Ice => "Icy",
                CelestialRingComposition::Rock => "Rocky",
                CelestialRingComposition::Metal => "Metallic",
                CelestialRingComposition::Dust => "Dusty",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CelestialRingLevel {
    /// A ring so thin or parse that one must search it in order to see it.
    Unnoticeable,
    /// A ring that is visible from up close or with a powerful telescope.
    Noticeable,
    /// A ring that is visible from anywhere in the star system when looking with an at least moderately powerful telescope.
    Visible,
    /// A ring that is easily visible, like Saturn's, even in small telescopes, from anywhere in the system.
    Spectacular,
}

impl Display for CelestialRingLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialRingLevel::Unnoticeable => "Unnoticeable",
                CelestialRingLevel::Noticeable => "Noticeable",
                CelestialRingLevel::Visible => "Visible",
                CelestialRingLevel::Spectacular => "Spectacular",
            }
        )
    }
}
