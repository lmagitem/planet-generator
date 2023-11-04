use crate::internal::*;
use crate::prelude::*;
use std::fmt;

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
    /// A celestial body composed mainly of volatile ices (like water, methane, and ammonia) and rock, similar to Uranus or Neptune.
    IcyBody(CelestialBody),
    /// A celestial body with a significant atmosphere, but lacking a solid surface, similar in composition to Jupiter or Saturn.
    GaseousBody(CelestialBody),
    /// Disk mostly made of rock, metals or dust that orbit around planets, stars, or other celestial bodies.
    TelluricDisk(CelestialDisk),
    /// Disk mostly made of ices that orbit around planets, stars, or other celestial bodies.
    IcyDisk(CelestialDisk),
    /// Disk mostly made of gas that orbit around planets, stars, or other celestial bodies.
    GaseousDisk(CelestialDisk),
    /// A man-made vehicle or habitat designed for operation in outer space.
    Spacecraft,
}

impl Display for AstronomicalObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AstronomicalObject::Void => "Empty space".to_string(),
                AstronomicalObject::Star(star) => format!(
                    "[{}], a {} {}{} Star of age: {} BY, mass: {} M☉, radius: {} R☉, temperature: {} K",
                    star.name,
                    star.population,
                    star.spectral_type,
                    if discriminant(&star.spectral_type) != discriminant(&StarSpectralType::XNS) &&
                        discriminant(&star.spectral_type) != discriminant(&StarSpectralType::XBH)   {
                       format!("{}", star.luminosity_class)
                    } else { "".to_string() }, star.age,
                    MathUtils::round_f32_to_precision(star.mass, 4),
                    MathUtils::round_f32_to_precision(star.radius, 4),
                    star.temperature,
                ),
                AstronomicalObject::TelluricBody(body) => format!(
                    "[{}], a {} {} body of mass: {} M⊕, radius: {} R⊕ ({} km of diameter), density: {} g/cm³, temperature: {} K",
                    body.name,
                    body.size,
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                          format!("{} ({})", details.body_type, details.world_type),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    MathUtils::round_f32_to_precision(body.mass,4),
                    MathUtils::round_f32_to_precision(body.radius,4),
                    MathUtils::round_f32_to_precision(body.radius * 12742.0,4),
                    MathUtils::round_f32_to_precision( body.density,4),
                    body.blackbody_temperature
                ),
                AstronomicalObject::IcyBody(body) => format!(
                    "[{}], a {} Icy {} body of mass: {} M⊕, radius: {} R⊕ ({} km of diameter), density: {} g/cm³, temperature: {} K",
                    body.name,
                    body.size,
                    match &body.details {
                        CelestialBodyDetails::Icy(details) =>
                            format!("({})", details.world_type),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    MathUtils::round_f32_to_precision( body.mass,4),
                    MathUtils::round_f32_to_precision(body.radius,4),
                    MathUtils::round_f32_to_precision(body.radius * 12742.0,4),
                    MathUtils::round_f32_to_precision( body.density,4),
                    body.blackbody_temperature
                ),
                AstronomicalObject::GaseousBody(body) => format!(
                    "[{}], a {} Gaseous body of mass: {} M⊕, radius: {} R⊕ ({} km of diameter), density: {} g/cm³, temperature: {} K",
                    body.name,
                    body.size,
                    MathUtils::round_f32_to_precision( body.mass,4),
                    MathUtils::round_f32_to_precision( body.radius,4),
                    MathUtils::round_f32_to_precision( body.radius * 12742.0,4),
                    MathUtils::round_f32_to_precision(body.density,4),
                    body.blackbody_temperature
                ),
                AstronomicalObject::TelluricDisk(disk) => format!(
                    "[{}], a {}",
                    disk.name,
                    disk.details,
                ),
                AstronomicalObject::IcyDisk(disk) => format!(
                    "[{}], a {}",
                    disk.name,
                    disk.details,
                ),
                AstronomicalObject::GaseousDisk(disk) => format!(
                    "[{}], a {}",
                    disk.name,
                    disk.details,
                ),
                AstronomicalObject::Spacecraft => "Spacecraft".to_string(),
            }
        )
    }
}
