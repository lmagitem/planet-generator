use crate::internal::{ConversionUtils, StringUtils};
use crate::prelude::{AstronomicalObject, CelestialBodyDetails, StarSpectralType};
use std::fmt;
use std::fmt::Display;
use std::mem::discriminant;

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
                    "[{}], {} {} {}, mass: {} M⊕, rds: {} R⊕ ({} km of diam.), dsity: {} g/cm³, grvty: {} g, temp: {} K ({}° C), tidal: {}, atm: {} atm, {}, {}, hydro: {}%, cryo: {}%, volcanism: {}, tectonics: {}, humidity: {}, traits: [{}]",
                    body.name,
                    body.size,
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            format!("{}", details.temperature_category),
                        _ => "WRONG-TYPE".to_string(),
                    },
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
                            format!("{}", details.magnetic_field),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            StringUtils::to_significant_decimals(details.hydrosphere as f64),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            StringUtils::to_significant_decimals((details.ice_over_water + details.ice_over_land) as f64),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            format!("{}% ({})", StringUtils::to_significant_decimals(details.volcanism as f64), details.clone().get_volcanism_level()),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            format!("{}% ({})", StringUtils::to_significant_decimals(details.tectonic_activity as f64), details.clone().get_tectonics_level()),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            format!("{}% ({})", StringUtils::to_significant_decimals(details.humidity as f64), details.clone().get_tectonics_level()),
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
