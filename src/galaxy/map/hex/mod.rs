use crate::prelude::*;
pub mod generator;
pub mod types;

/// The smaller division on a galactic map, might contain one or multiple star systems.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub struct GalacticHex {
    /// The coordinates of the first parsec covered by that hex on the galactic map, that is the one whose x, y and z values are the lowest.
    pub first_vertex: SpaceCoordinates,
    /// The coordinates of the last parsec covered by that hex on the galactic map, that is the one whose x, y and z values are the highest.
    pub last_vertex: SpaceCoordinates,
    /// The star systems one can find in this hex, if any.
    pub contents: Vec<StarSystem>,
}
