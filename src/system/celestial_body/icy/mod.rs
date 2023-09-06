use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct IcyDetails {
    // pub special_features: Vec<Information>,
}

impl IcyDetails {
    /// Creates a new [IcyDetails].
    pub fn new() -> Self {
        Self {}
    }
}
