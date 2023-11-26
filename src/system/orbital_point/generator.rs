use crate::internal::*;
use crate::prelude::*;

/// Calculates the orbital period between two bodies.
///
/// # Arguments
///
/// * `orbital_radius` - The orbital radius between the two bodies in astronomical units (AU).
/// * `mass1` - The mass of the first body in solar masses.
/// * `mass2` - The mass of the second body in solar masses.
///
/// # Returns
///
/// The orbital period in days.
pub fn calculate_orbital_period(orbital_radius: f64, mass1: f64, mass2: f64) -> f64 {
    let combined_mass = mass1 + mass2;
    let period_in_years = (orbital_radius.powf(3.0) / combined_mass).sqrt();
    period_in_years * 365.256
}

/// Calculates the orbital period between two bodies with masses given in Earth masses.
///
/// # Arguments
///
/// * `orbital_radius` - The orbital radius between the two bodies in astronomical units (AU).
/// * `mass1_earth_masses` - The mass of the first body in Earth masses.
/// * `mass2_earth_masses` - The mass of the second body in Earth masses.
///
/// # Returns
///
/// The orbital period in days.
pub fn calculate_orbital_period_from_earth_masses(
    orbital_radius: f64,
    mass1_earth_masses: f64,
    mass2_earth_masses: f64,
) -> f64 {
    calculate_orbital_period(
        orbital_radius,
        ConversionUtils::earth_mass_to_solar_mass(mass1_earth_masses),
        ConversionUtils::earth_mass_to_solar_mass(mass2_earth_masses),
    )
}

pub fn calculate_planet_orbit_eccentricity(
    coord: &SpaceCoordinates,
    system_index: u16,
    star_id: u32,
    gas_giant_arrangement: GasGiantArrangement,
    orbital_point_id: u32,
    orbit_distance: f64,
    settings: &GenerationSettings,
    blackbody_temp: u32,
    body_type: CelestialBodyComposition,
) -> (f64, f64, f64) {
    let mut rng = SeededDiceRoller::new(
        &settings.seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_ect",
            coord, system_index, star_id, orbital_point_id
        ),
    );
    let eccentricity_modifier =
        if gas_giant_arrangement == GasGiantArrangement::ConventionalGasGiant {
            -6
        } else if gas_giant_arrangement == GasGiantArrangement::EccentricGasGiant
            && blackbody_temp < 170
            && body_type == CelestialBodyComposition::Gaseous
        {
            -4
        } else {
            0
        };
    let roll = rng.roll(3, 6, eccentricity_modifier);
    let eccentricity = ((if roll <= 3 {
        0.0
    } else if roll <= 6 {
        0.05
    } else if roll <= 9 {
        0.1
    } else if roll <= 11 {
        0.15
    } else if roll <= 12 {
        0.2
    } else if roll <= 13 {
        0.3
    } else if roll <= 14 {
        0.4
    } else if roll <= 15 {
        0.5
    } else if roll <= 16 {
        0.6
    } else if roll <= 17 {
        0.7
    } else {
        0.8
    }) + (rng.roll(1, 11, -6) as f64 * 0.01))
        .max(0.0)
        .min(0.8);
    let min_separation = (1.0 - eccentricity) * orbit_distance;
    let max_separation = (1.0 + eccentricity) * orbit_distance;
    (eccentricity, min_separation, max_separation)
}
