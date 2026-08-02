use crate::prelude::{AstronomicalObject, CelestialBodyDetails, StarSpectralType};
use crate::utils::conversion::ConversionUtils;
use crate::utils::math::MathUtils;
use crate::utils::string::StringUtils;
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
                    "[{}], {} {} {}, mass: {} M⊕, rds: {} R⊕ ({} km of diam.), dsity: {} g/cm³, grvty: {} g, temp: {} K ({}° C), tidal: {}, atm: {} atm{}, {}, {}, hydro: {}%, cryo: {}%, volcanism: {}, tectonics: {}, humidity: {}, traits: [{}]",
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
                    ConversionUtils::kelvin_to_celsius(body.blackbody_temperature),
                    body.tidal_heating,
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) =>
                            StringUtils::to_significant_decimals(details.atmospheric_pressure as f64),
                        _ => "WRONG-TYPE".to_string(),
                    },
                    match &body.details {
                        CelestialBodyDetails::Telluric(details) => {
                            if details.atmospheric_composition.len() > 0 {
                                let composition_string = details.atmospheric_composition.iter().map(|(percentage, component)| {
                                    format!("{} ({}%)", component, MathUtils::round_f32_to_precision(*percentage, 2))
                                }).collect::<Vec<_>>().join(", ");
                                format!(" (of {})", composition_string)
                            } else {
                                String::from("")
                            }
                        },
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
                            // Humidity has no level vocabulary of its own (unlike
                            // volcanism/tectonics), so it prints as a plain
                            // percentage like hydro/cryo — NOT the tectonics level.
                            format!("{}%", StringUtils::to_significant_decimals(details.humidity as f64)),
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
                    ConversionUtils::kelvin_to_celsius( body.blackbody_temperature),
                    body.tidal_heating,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::celestial_body::icy::IcyBodyDetails;
    use crate::system::celestial_body::telluric::TelluricBodyDetails;
    use crate::system::celestial_body::types::CelestialBodySize;
    use crate::system::celestial_body::world::types::CelestialBodyWorldType;
    use crate::system::celestial_body::CelestialBody;
    use std::rc::Rc;

    /// K14: the IcyBody arm used to pass (temperature, tidal, celsius) into a
    /// "temp: {} K ({}° C), tidal: {}" format string, so the tidal value landed
    /// in the °C slot and the celsius value in the tidal slot.
    #[test]
    fn icy_body_display_puts_celsius_and_tidal_in_their_own_slots() {
        let body = CelestialBody::new(
            None,
            0,
            Rc::from("test-ice"),
            1.0,
            1.0,
            1.0,
            0.1,
            300,
            45,
            CelestialBodySize::default(),
            CelestialBodyDetails::Icy(IcyBodyDetails::new(CelestialBodyWorldType::Ice, vec![])),
        );
        let displayed = AstronomicalObject::IcyBody(body).to_string();
        assert!(
            displayed.contains("temp: 300 K (26° C), tidal: 45"),
            "celsius should sit in the °C slot and tidal in the tidal slot: {displayed}"
        );
    }

    /// K14: the humidity figure used to be labeled with the TECTONICS level
    /// (e.g. "humidity: 55.00% (Extreme)"); humidity has no level vocabulary of
    /// its own and prints as a plain percentage, like hydro/cryo.
    #[test]
    fn telluric_body_display_does_not_label_humidity_with_the_tectonics_level() {
        let details = TelluricBodyDetails {
            humidity: 55.0,
            // Extreme tectonics: any level leaking into the humidity slot shows up.
            tectonic_activity: 90.0,
            ..Default::default()
        };
        let body = CelestialBody::new(
            None,
            0,
            Rc::from("test-rock"),
            1.0,
            1.0,
            5.5,
            1.0,
            300,
            12,
            CelestialBodySize::default(),
            CelestialBodyDetails::Telluric(details),
        );
        let displayed = AstronomicalObject::TelluricBody(body).to_string();
        assert!(
            displayed.contains("humidity: 55.00%"),
            "humidity should print its own value: {displayed}"
        );
        assert!(
            !displayed.contains("humidity: 55.00% ("),
            "humidity must not carry the tectonics level label: {displayed}"
        );
    }
}
