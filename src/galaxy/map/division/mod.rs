use crate::prelude::*;
pub mod generator;

/// Represents a specific part of the [Galaxy].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct GalacticMapDivision {
    /// The denomination by which this particular partition of space is known.
    pub name: String,
    /// Which region of space makes the most of the division.
    pub region: GalacticRegion,
    /// The level of division this partition belongs to. See [SpaceDivisionLevel].
    pub level: u8,
    /// Which cell this division occupies on the X axis in its parent's grid.
    pub x: u8,
    /// Which cell this division occupies on the Y axis in its parent's grid.
    pub y: u8,
    /// Which cell this division occupies on the Z axis in its parent's grid.
    pub z: u8,
    /// The index of this division on the x, y and z axis.
    pub index: SpaceCoordinates,
}

impl GalacticMapDivision {
    /// Creates a new [GalacticMapDivision].
    pub fn new(
        name: String,
        region: GalacticRegion,
        level: u8,
        x: u8,
        y: u8,
        z: u8,
        index: SpaceCoordinates,
    ) -> Self {
        Self {
            name,
            region,
            level,
            x,
            y,
            z,
            index,
        }
    }
}
