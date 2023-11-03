use crate::internal::*;
use crate::prelude::*;
pub mod generator;

/// Represents a specific part of the [Galaxy].
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub struct GalacticMapDivision {
    /// The denomination by which this particular partition of space is known.
    #[default("default")]
    pub name: Rc<str>,
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
    /// This division's size in parsecs
    size: SpaceCoordinates,
}

impl GalacticMapDivision {
    /// Creates a new [GalacticMapDivision].
    pub fn new(
        name: Rc<str>,
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
            size: SpaceCoordinates::new(-1, -1, -1),
        }
    }

    /// Returns the top up left coordinates of this division.
    pub fn get_top_left_up(&self, galaxy: &Galaxy) -> SpaceCoordinates {
        self.index.rel(galaxy.get_galactic_start())
    }

    /// Returns the center coordinates of this division.
    pub fn get_center(&mut self, galaxy: &Galaxy) -> SpaceCoordinates {
        self.get_top_left_up(galaxy)
            + (self.get_bottom_right_down(galaxy) - self.get_top_left_up(galaxy))
                .abs(galaxy.get_galactic_start())
                / SpaceCoordinates::new(2, 2, 2)
    }

    /// Returns the bottom down right coordinates of this division.
    pub fn get_bottom_right_down(&mut self, galaxy: &Galaxy) -> SpaceCoordinates {
        self.index.rel(galaxy.get_galactic_start()) + self.get_size(galaxy)
    }

    /// Returns the full width of this division in parsecs.
    pub fn get_width(&mut self, galaxy: &Galaxy) -> i64 {
        self.get_size(galaxy).x
    }

    /// Returns the full height of this division in parsecs.
    pub fn get_height(&mut self, galaxy: &Galaxy) -> i64 {
        self.get_size(galaxy).y
    }

    /// Returns the full depth of this division in parsecs.
    pub fn get_depth(&mut self, galaxy: &Galaxy) -> i64 {
        self.get_size(galaxy).z
    }

    /// Returns the full size of this division in parsecs.
    pub fn get_size(&mut self, galaxy: &Galaxy) -> SpaceCoordinates {
        if self.size.x < 1 || self.size.y < 1 || self.size.z < 1 {
            let mut size = SpaceCoordinates::new(1, 1, 1);
            galaxy
                .division_levels
                .clone()
                .iter()
                .filter(|l| l.level <= self.level)
                .for_each(|l| {
                    size = size
                        * SpaceCoordinates::new(
                            l.x_subdivisions as i64,
                            l.y_subdivisions as i64,
                            l.z_subdivisions as i64,
                        );
                });
            self.size = size;
        }
        self.size
    }
}
