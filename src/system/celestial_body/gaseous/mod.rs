use crate::internal::*;
use crate::prelude::*;
mod constants;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct GaseousBodyDetails {
    /// What are the pecularities of this gaseous body.
    pub special_traits: Vec<GasGiantSpecialTrait>,
    // pub gas_giant_type: GasGiantType,
    // pub color: Information,
    // pub atmospheric_composition: AtmosphericCompositionType,
    // pub special_features: Vec<Information>,
}

impl GaseousBodyDetails {
    /// Creates a new [GaseousBodyDetails].
    pub fn new(special_traits: Vec<GasGiantSpecialTrait>) -> Self {
        Self { special_traits }
    }
}
