use crate::internal::*;
use crate::prelude::*;

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
    #[default = true]
    pub flat_map: bool,
    /// If set to true, only one roll will occur to determine how much star systems there are per hex. If set to false, a roll will be made
    /// for each cubic parsec inside the hex.
    #[default = true]
    pub density_by_hex_instead_of_parsec: bool,
    /// If set to true, the maximum number of systems per hex is one.
    #[default = true]
    pub max_one_system_per_hex: bool,
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
    /// The core of a spiral or dwarf lenticular galaxy.
    Nucleus,
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
    /// Some stars lost into the void, pushed out of their normal course by gravity during their past.
    Exile,
}
