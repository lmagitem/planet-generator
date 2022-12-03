use crate::prelude::*;
pub mod generator;
pub mod types;

/// The smaller division on a galactic map, might contain one or multiple star systems.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub struct GalacticHex {
    /// The index of this hex on the x, y and z axis.
    pub index: SpaceCoordinates,
    /// The star systems one can find in this hex, if any.
    pub contents: Vec<StarSystem>,
}

impl GalacticHex {
    /// Creates a new instance of [GalacticHex].
    pub fn new(index: SpaceCoordinates, contents: Vec<StarSystem>) -> Self {
        Self { index, contents }
    }
}
