use crate::prelude::*;

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
}
