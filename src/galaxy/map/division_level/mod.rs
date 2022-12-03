use crate::prelude::*;
pub mod generator;

/// Data pertaining how space is divided on a galactic scale.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct GalacticMapDivisionLevel {
    /// The divisions' "level" this object represents. The higher the level, the bigger the division. 0 is a single hex, 1 a subsector,
    /// 2 a sector, and so on...
    pub level: u8,
    /// How many inferior level divisions this level spans on the x axis, or how many parsecs for level 0.
    pub x_subdivisions: u8,
    /// How many inferior level divisions this level spans on the y axis, or how many parsecs for level 0.
    pub y_subdivisions: u8,
    /// How many inferior level divisions this level spans on the z axis, or how many parsecs for level 0.
    pub z_subdivisions: u8,
}
