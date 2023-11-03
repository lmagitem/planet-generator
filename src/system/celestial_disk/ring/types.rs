use crate::internal::*;
use crate::prelude::*;

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
