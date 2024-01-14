use crate::internal::*;
use crate::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Orbit {
    /// The id of the primary body at the center of this orbit.
    pub primary_body_id: u32,
    /// The id of the object in this orbit.
    pub id: Option<u32>,
    /// The zone type this orbit is in. Forbidden means planets cannot form there naturally.
    pub zone: ZoneType,
    /// The average distance from the primary body to the object in this orbit.
    pub average_distance: f64,
    /// The minimum possible distance from the primary body to the object in this orbit.
    pub min_separation: f64,
    /// The maximum possible distance from the primary body to the object in this orbit.
    pub max_separation: f64,
    /// The average distance from the barycentre of the solar system to the object in this orbit.
    pub average_distance_from_system_center: f64,
    /// A measure of the orbit's deviation from a perfect circle, ranging from 0 (circular) to
    /// values close to 1 (highly elongated).
    pub eccentricity: f32,
    /// Indicates an orbit in the reference plane, while 90° is perpendicular. Values over 90°
    /// suggest a retrograde orbit.
    pub inclination: f32,
    /// The axial tilt in degrees, indicating the angle between the object's rotational axis and
    /// its orbital plane, affecting seasonal variations.
    pub axial_tilt: f32,
    /// The time it takes in terran days for the object in this orbit to make a complete journey
    /// around what it orbits.
    pub orbital_period: f32,
    /// The time it takes in terran days for the object in this orbit to make a complete rotation
    /// around itself.
    pub rotation: f32,
    /// The time it takes in terran days between two sunrises on the object in this orbit.
    pub day_length: f32,
}

impl Orbit {
    /// Creates a new [Orbit].
    pub fn new(
        primary_body_id: u32,
        id: Option<u32>,
        zone: ZoneType,
        average_distance: f64,
        min_separation: f64,
        max_separation: f64,
        average_distance_from_system_center: f64,
        eccentricity: f32,
        inclination: f32,
        axial_tilt: f32,
        orbital_period: f32,
        rotation: f32,
        day_length: f32,
    ) -> Self {
        Self {
            primary_body_id,
            id,
            zone,
            average_distance,
            min_separation,
            max_separation,
            average_distance_from_system_center,
            eccentricity,
            inclination,
            axial_tilt,
            orbital_period,
            rotation,
            day_length,
        }
    }
}

impl Display for Orbit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} around {} - {} AU ({} AU), ecc: {}, min sep: {} AU, max sep: {} AU, incl: {}, tilt: {}°, period: {} day⊕, rota°: {} day⊕, day: {} day⊕",
            if self.id.is_some() { format!("{:03}", self.id.unwrap()) } else { String::from("EMPTY") },
            format!("{:03}", self.primary_body_id),
            StringUtils::to_significant_decimals(self.average_distance),
            StringUtils::to_significant_decimals(self.average_distance_from_system_center),
            StringUtils::to_significant_decimals(self.eccentricity as f64),
            StringUtils::to_significant_decimals(self.min_separation),
            StringUtils::to_significant_decimals(self.max_separation),
            StringUtils::to_significant_decimals(self.inclination as f64),
            self.axial_tilt,
            StringUtils::to_significant_decimals(self.orbital_period as f64),
            StringUtils::to_significant_decimals(self.rotation as f64),
            StringUtils::to_significant_decimals(self.day_length as f64)
        )
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
                    "[{}], a {} {}{} Star of age: {} BY, mass: {} M☉, radius: {} R☉ ({} km of diameter), temperature: {} K ({}° C), traits: [{}]",
                    star.name,
                    star.population,
                    star.spectral_type,
                    if discriminant(&star.spectral_type) != discriminant(&StarSpectralType::XNS) &&
                        discriminant(&star.spectral_type) != discriminant(&StarSpectralType::XBH)   {
                       format!("{}", star.luminosity_class)
                    } else { "".to_string() }, star.age,
                    StringUtils::to_significant_decimals(star.mass as f64),
                    StringUtils::to_significant_decimals(star.radius as f64),
                    StringUtils::to_significant_decimals((star.radius * 696340.0 * 2.0) as f64),
                    star.temperature,
                    ConversionUtils::kelvin_to_celsius( star.temperature),
                    &star.special_traits.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "),
                ),
                AstronomicalObject::TelluricBody(body) => format!(
                    "[{}], {} {}, mass: {} M⊕, rds: {} R⊕ ({} km of diam.), dsity: {} g/cm³, grvty: {} g, temp: {} K ({}° C), tidal: {}, atm: {} atm, core: {}, traits: [{}]",
                    body.name,
                    body.size,
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                          format!("{} ({})", details.world_type, details.body_type),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    StringUtils::to_significant_decimals(body.mass as f64),
                    StringUtils::to_significant_decimals(body.radius as f64),
                    StringUtils::to_significant_decimals((body.radius * 12742.0) as f64),
                    StringUtils::to_significant_decimals(body.density as f64),
                    StringUtils::to_significant_decimals(body.gravity as f64),
                    body.blackbody_temperature,
                    ConversionUtils::kelvin_to_celsius( body.blackbody_temperature),
                    body.tidal_heating,
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            StringUtils::to_significant_decimals(details.atmospheric_pressure as f64),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            format!("{}", details.core_heat),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            details.special_traits.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "),
                        _ => "WRONG-TYPE".to_string(),
                    },
                ),
                AstronomicalObject::IcyBody(body) => format!(
                    "[{}], Ice {}, mass: {} M⊕, rds: {} R⊕ ({} km of diam.), dsity: {} g/cm³, grvty: {} g, temp: {} K ({}° C), tidal: {}, traits: [{}]",
                    body.name,
                    body.size,
                    StringUtils::to_significant_decimals(body.mass as f64),
                    StringUtils::to_significant_decimals(body.radius as f64),
                    StringUtils::to_significant_decimals((body.radius * 12742.0) as f64),
                    StringUtils::to_significant_decimals(body.density as f64),
                    StringUtils::to_significant_decimals(body.gravity as f64),
                    body.blackbody_temperature,
                    body.tidal_heating,
                   ConversionUtils::kelvin_to_celsius( body.blackbody_temperature),
                    match &body.details {
                        CelestialBodyDetails::Icy(details) =>
                            details.special_traits.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "),
                        _ => "WRONG-TYPE".to_string(),
                    },
                ),
                AstronomicalObject::GaseousBody(body) => format!(
                    "[{}], Gas {}, mass: {} M⊕, rds: {} R⊕ ({} km of diam.), dsity: {} g/cm³, grvty: {} g, temp: {} K ({}° C) tidal: {}, traits: [{}]",
                    body.name,
                    body.size,
                    StringUtils::to_significant_decimals(body.mass as f64),
                    StringUtils::to_significant_decimals(body.radius as f64),
                    StringUtils::to_significant_decimals((body.radius * 12742.0) as f64),
                    StringUtils::to_significant_decimals(body.density as f64),
                    StringUtils::to_significant_decimals(body.gravity as f64),
                    body.blackbody_temperature,
                   ConversionUtils::kelvin_to_celsius( body.blackbody_temperature),
                    body.tidal_heating,
                    match &body.details {
                        CelestialBodyDetails::Gaseous(details) =>
                            details.special_traits.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "),
                        _ => "WRONG-TYPE".to_string(),
                    },
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
