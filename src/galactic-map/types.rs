/// Data pertaining how space is divided on a galactic scale.
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

/// Represents the different kind of places a star could be in the galaxy.
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

/// Coordinates of an hex in a galactic map.
pub struct SpaceCoordinates {
    /// The x coordinate of the point.
    pub x: u64,
    /// The y coordinate of the point.
    pub y: u64,
    /// The z coordinate of the point.
    pub z: u64,
}
