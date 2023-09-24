use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct IcyRingDetails {}

impl IcyRingDetails {
    /// Creates a new [IcyRingDetails].
    pub fn new() -> Self {
        Self {}
    }
}
