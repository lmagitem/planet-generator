use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct GaseousRingDetails {}

impl GaseousRingDetails {
    /// Creates a new [GaseousRingDetails].
    pub fn new() -> Self {
        Self {}
    }
}
