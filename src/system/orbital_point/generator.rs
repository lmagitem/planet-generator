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

pub fn complete_orbit_with_period_and_eccentricity(
    coord: SpaceCoordinates,
    system_index: u16,
    star_id: u32,
    orbited_object_mass: f64,
    gas_giant_arrangement: GasGiantArrangement,
    orbital_point_id: u32,
    own_orbit: &Option<Orbit>,
    orbit_distance: f64,
    is_gas_giant: bool,
    blackbody_temp: u32,
    mass: f32,
    is_moon: bool,
    settings: &GenerationSettings,
) -> Orbit {
    let mut this_orbit = own_orbit.clone().unwrap_or_default();
    let orbital_period = calculate_orbital_period_from_earth_masses(
        orbit_distance,
        orbited_object_mass,
        mass as f64,
    );
    this_orbit.orbital_period = orbital_period as f32;
    let (eccentricity, min_separation, max_separation) = if is_moon {
        // TODO: Change this
        calculate_planet_orbit_eccentricity(
            &coord,
            system_index,
            star_id,
            gas_giant_arrangement,
            orbital_point_id,
            orbit_distance,
            &settings,
            blackbody_temp,
            is_gas_giant,
        )
    } else {
        calculate_planet_orbit_eccentricity(
            &coord,
            system_index,
            star_id,
            gas_giant_arrangement,
            orbital_point_id,
            orbit_distance,
            &settings,
            blackbody_temp,
            is_gas_giant,
        )
    };
    this_orbit.eccentricity = eccentricity as f32;
    this_orbit.min_separation = min_separation;
    this_orbit.max_separation = max_separation;
    this_orbit
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
    is_gas_giant: bool,
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
            && is_gas_giant
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon_orbital_period() {
        let radius = 0.00257; // in AU
        let earth_mass = 1.0; // Earth mass
        let moon_mass = 0.0123; // Moon's mass in Earth masses
        let period = calculate_orbital_period_from_earth_masses(radius, earth_mass, moon_mass);
        assert!(
            (period - 27.32).abs() < 1.0,
            "The calculated period for the Moon is incorrect"
        );
    }

    #[test]
    fn test_ganymede_orbital_period() {
        let radius = 0.00716; // in AU
        let jupiter_mass = 317.8; // Jupiter's mass in Earth masses
        let ganymede_mass = 0.0248; // Ganymede's mass in Earth masses
        let period =
            calculate_orbital_period_from_earth_masses(radius, jupiter_mass, ganymede_mass);
        assert!(
            (period - 7.15).abs() < 0.1,
            "The calculated period for Ganymede is incorrect"
        );
    }

    #[test]
    fn test_titan_orbital_period() {
        let radius = 0.00817; // in AU
        let saturn_mass = 95.16; // Saturn's mass in Earth masses
        let titan_mass = 0.0225; // Titan's mass in Earth masses
        let period = calculate_orbital_period_from_earth_masses(radius, saturn_mass, titan_mass);
        assert!(
            (period - 15.94).abs() < 0.1,
            "The calculated period for Titan is incorrect"
        );
    }
}
