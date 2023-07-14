use crate::prelude::*;
use std::ops::*;

/// Coordinates of a point in a galactic map, in parsecs.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct SpaceCoordinates {
    /// The x coordinate of the point in parsecs relative to the galactic center.
    pub x: i64,
    /// The y coordinate of the point in parsecs relative to the galactic center.
    pub y: i64,
    /// The z coordinate of the point in parsecs relative to the galactic center.
    pub z: i64,
}

impl SpaceCoordinates {
    /// Creates a new [SpaceCoordinates] instance.
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        SpaceCoordinates { x, y, z }
    }

    /// Returns the absolute value of the coordinates in the galaxy. Uses a starting point which corresponds to the coordinates of the first
    /// parsec in the galactic map.
    pub fn abs(self, starting_point: SpaceCoordinates) -> Self {
        self - starting_point
    }

    /// Returns the value of the coordinates relative to the center of the galaxy. Uses a starting point, which corresponds to the
    /// coordinates of the first parsec in the galactic map.
    pub fn rel(self, starting_point: SpaceCoordinates) -> Self {
        self + starting_point
    }
}

impl Display for SpaceCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl Add for SpaceCoordinates {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl Sub for SpaceCoordinates {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl Mul for SpaceCoordinates {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Self {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        }
    }
}

impl Div for SpaceCoordinates {
    type Output = Self;
    fn div(self, o: Self) -> Self {
        Self {
            x: self.x / o.x,
            y: self.y / o.y,
            z: self.z / o.z,
        }
    }
}
