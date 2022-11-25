use crate::prelude::*;

/// The smaller division on a galactic map, might contain one or multiple star systems.
pub struct GalacticHex {
    /// The coordinates of this hex on the galactic map.
    pub coord: SpaceCoordinates,
    /// The star systems one can find in this hex, if any.
    pub contents: Vec<StarSystem>,
}
