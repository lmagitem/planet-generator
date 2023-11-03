use crate::internal::*;
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

impl GalacticMapDivisionLevel {
    /// Creates a new instance of [GalacticMapDivisionLevel].
    pub fn new(level: u8, x_subdivisions: u8, y_subdivisions: u8, z_subdivisions: u8) -> Self {
        Self {
            level,
            x_subdivisions,
            y_subdivisions,
            z_subdivisions,
        }
    }

    /// Returns the number of subdivisions on the x, y and z axis in the form of a [SpaceCoordinates] object.
    pub fn as_coord(self) -> SpaceCoordinates {
        SpaceCoordinates {
            x: self.x_subdivisions as i64,
            y: self.y_subdivisions as i64,
            z: self.z_subdivisions as i64,
        }
    }
}
