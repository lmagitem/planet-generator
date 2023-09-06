use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct GaseousDetails {
    // pub gas_giant_type: GasGiantType,
    // pub color: Information,
    // pub atmospheric_composition: AtmosphericCompositionType,
    // pub special_features: Vec<Information>,
}

impl GaseousDetails {
    /// Creates a new [GaseousDetails].
    pub fn new() -> Self {
        Self {}
    }
}
