use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct IcyBodyDetails {
    // pub special_features: Vec<Information>,
}

impl IcyBodyDetails {
    /// Creates a new [IcyBodyDetails].
    pub fn new() -> Self {
        Self {}
    }
}
