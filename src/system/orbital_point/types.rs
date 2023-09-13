use crate::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Orbit {
    /// The id of the primary body at the center of this orbit.
    pub primary_body_id: u32,
    /// The ids of the objects in this orbit.
    pub satellite_ids: Vec<u32>,
    /// The zone type this orbit is in. Forbidden means planets cannot form there naturally.
    pub zone: ZoneType,
    /// The average distance from the primary body to the objects in this orbit.
    pub average_distance: f64,
    /// The average distance from the barycentre of the solar system to the objects in this orbit.
    pub average_distance_from_system_center: f64,
    /// A measure of the orbit's deviation from a perfect circle, ranging from 0 (circular) to
    /// values close to 1 (highly elongated).
    pub eccentricity: f32,
    /// The tilt angle (in degrees) between the orbital plane and a reference plane. A value of 0°
    /// indicates an orbit in the reference plane, while 90° is perpendicular. Values over 90°
    /// suggest a retrograde orbit.
    pub inclination: f32,
}

impl Orbit {
    /// Creates a new [Orbit].
    pub fn new(
        primary_body_id: u32,
        satellite_ids: Vec<u32>,
        zone: ZoneType,
        average_distance: f64,
        average_distance_from_system_center: f64,
        eccentricity: f32,
        inclination: f32,
    ) -> Self {
        Self {
            primary_body_id,
            satellite_ids,
            zone,
            average_distance,
            average_distance_from_system_center,
            eccentricity,
            inclination,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub enum AstronomicalObject {
    /// Represents the absence of any significant object at a particular orbital point.
    #[default]
    Void,
    /// A celestial body emitting light and heat from nuclear reactions, like our Sun.
    Star(Star),
    /// A celestial body primarily composed of rock or metal, such as Mercury or Earth.
    TelluricBody(CelestialBody),
    /// A celestial body with a significant atmosphere, but lacking a solid surface, similar in composition to Jupiter or Saturn.
    GaseousBody(CelestialBody),
    /// A celestial body composed mainly of volatile ices (like water, methane, and ammonia) and rock, similar to Uranus or Neptune.
    IcyBody(CelestialBody),
    /// Thin disks of small particles that orbit around planets, stars, or other celestial bodies.
    Ring,
    /// A man-made vehicle or habitat designed for operation in outer space.
    Spacecraft,
}
