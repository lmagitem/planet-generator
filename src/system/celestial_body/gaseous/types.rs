use crate::internal::*;
use crate::prelude::*;

/// A list of settings used to configure the Gaseous Bodies (like gas giants) generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GaseousBodySettings {
    /// A list of specific [CelestialBodySpecialTrait]s to use, if any.
    pub fixed_special_traits: Option<Vec<CelestialBodySpecialTrait>>,
    /// A list of [CelestialBodySpecialTrait]s forbidden to use in Gas Giant generation.
    pub forbidden_special_traits: Option<Vec<CelestialBodySpecialTrait>>,
}
