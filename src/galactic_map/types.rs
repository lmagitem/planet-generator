use crate::prelude::*;
use std::ops::*;

/// A list of settings used to configure the [GalacticMapDivisionLevel], [GalacticMapDivision]s and [GalacticHex]es generation.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub struct SectorSettings {
    /// How many parsecs the building block of a galactic map spans, on the (x, y, z) axis. Must be between 1 and 255 inclusive.
    #[default((1, 1, 1))]
    pub hex_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_1_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((4, 4, 4))]
    pub level_2_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_3_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_4_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_5_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_6_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_7_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_8_size: (u8, u8, u8),
    /// How many inferior level divisions this level spans, on the (x, y, z) axis. Must be between 4 and 62 inclusive. the z level might
    /// be overridden by the **flat_map** parameter.
    #[default((10, 10, 10))]
    pub level_9_size: (u8, u8, u8),
    /// If set to true, a single z level will be generated. For your map to still have some kind of height, you can set the **hex_size**
    /// z axis to a value different than 1, it will enable star systems to be generated "above" and "under" the map plane.
    #[default = false]
    pub flat_map: bool,
}

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
    pub fn generate_division_levels(settings: &GenerationSettings) -> Vec<Self> {
        let mut division_levels = Vec::new();
        // TODO: this
        division_levels
    }
}

/// Represents the different kind of places a star could be in the galaxy.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum GalacticRegion {
    /// This division spawns accross a number of different regions.
    Multiple,
    /// The core of an elliptical galaxy. Stars there are generally older, numerous and densely packed.
    Core,
    /// The bulge of a spiral or lenticular galaxy. Stars there are generally numerous and densely packed, but age and star generation rate
    /// depends on the type of bulge.
    Bulge,
    /// The central bar of barred spiral galaxies. It channels gas inwards from the spiral arms and acts as a kind of stellar nursery.
    Bar,
    /// The arms of a spiral galaxy. Stars there are usualy young, with numerous bright blue stars due to the high mass density and high
    /// rate of star formation.
    Arm,
    /// The disk of a lenticular galaxy, or the space between the arms in a spiral one. Stars there usualy follow a very regular and
    /// circular orbit.
    Disk,
    /// The majority of an elliptical galaxy's content. Star formation rate is usualy quite poor.
    Ellipse,
    /// The area in the outskirts of a galaxy. Stars there are generally much older and way less densely packed.
    Halo,
    /// The area in the outskirts of an irregular galaxy lost in the void or distorded by the influence of other members of a cluster. Stars
    /// there are generally older and way less densely packed.
    Aura,
    /// The void outside the limits of the galaxy proper. One can still find the occasional stray stars and satellite clusters around.
    #[default]
    Void,
    /// A spheroidal conglomeration of stars gound together by their gravity, with a higher concentration of stars towards the center. They
    /// tenbd to be older, denser and have less metallicity
    GlobularCluster,
    /// A cluster of stars made up from the same giant molecular cloud. They have roughly the same age and are loosely bound by mutual
    /// gravitational attraction.
    OpenCluster,
    /// A very loose cluster of stars that share a common origin. They no longer are bound by gravitational attraction, but still move
    /// together through space.
    Association,
    /// A number of stars which was previously a cluster and has been torn apart and stretched out along the orbit of a galaxy by tidal
    /// forces.
    Stream,
    /// A number of stars in the remnants of a supernova.
    Remnant,
    /// Some stars lost into the void, pushed out of their normal course by gravity during their past.
    Exile,
}

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

    /// Returns the absolute value of the coordinates using a given starting point.
    pub fn abs(self, starting_point: SpaceCoordinates) -> Self {
        self + starting_point
    }
}

impl Add for SpaceCoordinates {
    fn add(self, o: Self) -> Self {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
    type Output = Self;
}

impl Sub for SpaceCoordinates {
    fn sub(self, o: Self) -> Self {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
    type Output = Self;
}

impl Mul for SpaceCoordinates {
    fn mul(self, o: Self) -> Self {
        Self {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        }
    }
    type Output = Self;
}

impl Div for SpaceCoordinates {
    fn div(self, o: Self) -> Self {
        Self {
            x: self.x / o.x,
            y: self.y / o.y,
            z: self.z / o.z,
        }
    }
    type Output = Self;
}
