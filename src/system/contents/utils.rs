use crate::internal::ConversionUtils;

use super::elements::AtmosphereElement;

/// Returns a value in Kelvin
pub(crate) fn calculate_blackbody_temperature(luminosity: f32, orbital_radius: f64) -> u32 {
    if orbital_radius <= 0.0 {
        panic!("Orbital radius should be greater than 0");
    }

    let b = 278.0 * ((luminosity as f64).powf(0.25)) / (orbital_radius).sqrt();
    b.round() as u32
}

/// Returns a value in Astronomical Units.
pub(crate) fn calculate_distance_for_temperature(luminosity: f32, temperature: i32) -> f64 {
    let t_ratio = temperature as f64 / 278.0;
    (luminosity as f64 / t_ratio.powi(4)).sqrt()
}

/// Returns a value in Earth Radii
pub(crate) fn calculate_radius(mass_earth_masses: f64, density_g_cm3: f64) -> f64 {
    let earth_mass_kg: f64 = 5.972e24;
    let earth_radius_meters: f64 = 6.371e6;

    let mass_kg = mass_earth_masses * earth_mass_kg;
    let density_kg_m3 = density_g_cm3 * 1000.0;
    let volume_m3 = mass_kg / density_kg_m3;
    let radius_meters = ((3.0 * volume_m3) / (4.0 * std::f64::consts::PI)).cbrt();
    let radius_earth_radii = radius_meters / earth_radius_meters;

    radius_earth_radii
}

/// Returns a value in Gs
pub(crate) fn calculate_surface_gravity(density_g_cm3: f32, radius_earth_radii: f64) -> f32 {
    (density_g_cm3 / 5.513) * radius_earth_radii as f32
}

/// Calculates the Roche limit based on the densities of the primary and the satellite.
/// The radius of the primary is in Earth radii, density can be any shared unit, and the return value is in AU.
pub fn calculate_roche_limit(
    radius_primary: f64,
    density_primary: f64,
    density_satellite: f64,
) -> f64 {
    ConversionUtils::earth_radii_to_astronomical_units(
        2.44 * radius_primary * (density_primary / density_satellite).powf(1.0 / 3.0),
    )
}

/// Calculates the Hill sphere radius, aka the region around a planet where it can have stable satellites instead of them
/// being pulled out by the system's star.
/// The distance must be in AU and the masses in Solar Masses.
pub(crate) fn calculate_hill_sphere_radius(
    orbital_radius_planet: f64,
    mass_planet: f64,
    mass_star: f64,
) -> f64 {
    orbital_radius_planet * (mass_planet / (3.0 * mass_star)).powf(1.0 / 3.0)
}

/// Calculates the escape velocity for a planet.
///
/// The escape velocity is the minimum speed needed for an object to break free
/// from the gravitational attraction of the planet without further propulsion.
///
/// # Parameters
/// - `mass_earth`: Mass of the planet in Earth masses (1 Earth mass = 5.972e24 kg)
/// - `radius_earth`: Radius of the planet in Earth radii (1 Earth radius = 6.371e6 m)
///
/// # Returns
/// The escape velocity in meters per second (m/s).
///
/// # Example
/// ```
/// let ve_earth = escape_velocity(1.0, 1.0);
/// println!("Escape velocity of Earth: {:.2} m/s", ve_earth);
/// println!("Escape velocity of Earth: {:.2} km/s", ve_earth / 1000.0);
/// ```
fn escape_velocity(mass_earth: f64, radius_earth: f64) -> f64 {
    const G: f64 = 6.67430e-11; // Gravitational constant in m^3 kg^-1 s^-2
    const EARTH_MASS: f64 = 5.972e24; // Earth mass in kg
    const EARTH_RADIUS: f64 = 6.371e6; // Earth radius in meters

    let mass = mass_earth * EARTH_MASS; // Convert mass to kg
    let radius = radius_earth * EARTH_RADIUS; // Convert radius to meters

    ((2.0 * G * mass) / radius).sqrt() // Result in m/s
}

/// Calculates the root mean square (rms) speed of gas molecules.
///
/// The rms speed is a measure of the average speed of gas molecules at a given temperature.
///
/// # Parameters
/// - `temperature`: Temperature in Kelvin (K)
/// - `molecular_mass`: Molecular mass of the gas in kilograms (kg)
///
/// # Returns
/// The rms speed in meters per second (m/s).
///
/// # Example
/// ```
/// let vrms_h2 = rms_speed(288, 2.0 * 1.6735575e-27);
/// println!("RMS speed of hydrogen molecules: {:.2} m/s", vrms_h2);
/// println!("RMS speed of hydrogen molecules: {:.2} km/s", vrms_h2 / 1000.0);
/// ```
fn rms_speed(temperature: i32, molecular_mass: f64) -> f64 {
    const K_B: f64 = 1.380649e-23; // Boltzmann constant in J/K
    ((3.0 * K_B * temperature as f64) / molecular_mass).sqrt()
}

/// Calculates the Jeans parameter for a given planet and gas.
///
/// The Jeans parameter indicates the ability of a planet to retain a particular gas.
/// Higher values indicate better retention capability.
///
/// # Parameters
/// - `mass_earth`: Mass of the planet in Earth masses (1 Earth mass = 5.972e24 kg)
/// - `radius_earth`: Radius of the planet in Earth radii (1 Earth radius = 6.371e6 m)
/// - `temperature`: Temperature in Kelvin (K)
/// - `element`: The atmospheric element as an enum
///
/// # Returns
/// The Jeans parameter (dimensionless).
///
/// # Example
/// ```
/// let jeans_param_h2 = jeans_parameter(1.0, 1.0, 288.0, AtmosphereElement::Hydrogen);
/// println!("Jeans parameter for hydrogen on Earth: {:.2}", jeans_param_h2);
/// ```
fn jeans_parameter(
    mass_earth: f64,
    radius_earth: f64,
    temperature: i32,
    element: AtmosphereElement,
) -> f64 {
    let v_e = escape_velocity(mass_earth, radius_earth);
    let v_rms = rms_speed(temperature, element.molecular_weight_kg());
    v_e / v_rms
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_MARGIN: f64 = 0.15;
    const EPSILON: f64 = 1e-5;

    fn within_error_margin(calculated: f64, expected: f64) -> bool {
        (calculated / expected - 1.0).abs() <= ERROR_MARGIN
    }

    #[test]
    fn test_calculate_radius_earth() {
        let earth_mass = 1.0;
        let earth_density = 5.513;
        let radius = calculate_radius(earth_mass, earth_density);
        assert!(within_error_margin(radius, 1.0));
    }

    #[test]
    fn test_calculate_radius_jupiter() {
        let jupiter_mass = 317.8; // Jupiter's mass in Earth masses
        let jupiter_density = 1.33; // Jupiter's density in g/cm³
        let radius = calculate_radius(jupiter_mass, jupiter_density);
        assert!(within_error_margin(radius, 11.208));
    }

    #[test]
    fn test_calculate_radius_saturn() {
        let saturn_mass = 95.2;
        let saturn_density = 0.69;
        let radius = calculate_radius(saturn_mass, saturn_density);
        assert!(within_error_margin(radius, 9.45));
    }

    #[test]
    fn test_calculate_radius_mars() {
        let mars_mass = 0.107;
        let mars_density = 3.93;
        let radius = calculate_radius(mars_mass, mars_density);
        assert!(within_error_margin(radius, 0.532));
    }

    #[test]
    fn test_calculate_radius_ganymede() {
        let ganymede_mass = 0.0248;
        let ganymede_density = 1.942;
        let radius = calculate_radius(ganymede_mass, ganymede_density);
        assert!(within_error_margin(radius, 0.413));
    }

    #[test]
    fn test_calculate_radius_moon() {
        let moon_mass = 0.0123;
        let moon_density = 3.344;
        let radius = calculate_radius(moon_mass, moon_density);
        assert!(within_error_margin(radius, 0.273));
    }

    #[test]
    fn test_calculate_surface_gravity_earth() {
        let earth_density = 5.513;
        let earth_radius = 1.0;
        let gravity = calculate_surface_gravity(earth_density, earth_radius);
        assert!(within_error_margin(gravity as f64, 1.0));
    }

    #[test]
    fn test_calculate_surface_gravity_mars() {
        let mars_density = 3.93;
        let mars_radius = 0.532;
        let gravity = calculate_surface_gravity(mars_density, mars_radius);
        assert!(within_error_margin(gravity as f64, 0.38));
    }

    #[test]
    fn test_calculate_surface_gravity_jupiter() {
        let jupiter_density = 1.33;
        let jupiter_radius = 11.2;
        let gravity = calculate_surface_gravity(jupiter_density, jupiter_radius);
        assert!(within_error_margin(gravity as f64, 2.528));
    }

    #[test]
    fn test_calculate_surface_gravity_saturn() {
        let saturn_density = 0.69;
        let saturn_radius = 9.45;
        let gravity = calculate_surface_gravity(saturn_density, saturn_radius);
        assert!(within_error_margin(gravity as f64, 1.065));
    }

    #[test]
    fn test_calculate_surface_gravity_ganymede() {
        let ganymede_density = 1.942;
        let ganymede_radius = 0.413;
        let gravity = calculate_surface_gravity(ganymede_density, ganymede_radius);
        assert!(within_error_margin(gravity as f64, 0.146));
    }

    #[test]
    fn test_calculate_surface_gravity_moon() {
        let moon_density = 3.344;
        let moon_radius = 0.273;
        let gravity = calculate_surface_gravity(moon_density, moon_radius);
        assert!(within_error_margin(gravity as f64, 0.165));
    }

    #[test]
    fn test_calculate_hill_sphere_radius_earth_sun() {
        let semi_major_axis_earth: f64 = 1.0;
        let earth_mass: f64 = 1.0 / 333000.0;
        let sun_mass: f64 = 1.0;
        let expected_hill_sphere_radius_au: f64 = 0.01;

        let hill_sphere_radius =
            calculate_hill_sphere_radius(semi_major_axis_earth, earth_mass, sun_mass);
        assert!((hill_sphere_radius - expected_hill_sphere_radius_au).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_roche_limit_mass_gas_giant_moon() {
        let saturn_radius: f64 = 9.14;
        let saturn_density: f64 = 0.687;
        let titan_density: f64 = 1.88;

        let expected_roche_limit_au = 0.0006790230406567097;
        let roche_limit = calculate_roche_limit(saturn_radius, saturn_density, titan_density);

        assert!((roche_limit - expected_roche_limit_au).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_roche_limit_mass_star_planet() {
        let sun_radius: f64 = 109.2;
        let sun_density: f64 = 1.41;
        let earth_density: f64 = 5.51;

        let expected_roche_limit_au = 0.00720416795141276;
        let roche_limit = calculate_roche_limit(sun_radius, sun_density, earth_density);

        assert!((roche_limit - expected_roche_limit_au).abs() < EPSILON);
    }

    #[test]
    fn test_escape_velocity_earth() {
        let ve_earth = escape_velocity(1.0, 1.0);
        assert!((ve_earth - 11186.0).abs() < 1.0); // Expect around 11186 m/s
    }

    #[test]
    fn test_rms_speed_hydrogen_earth() {
        let temperature_earth = 288; // Average temperature of Earth's surface in Kelvin
        let mass_h2 = 2.0 * 1.6735575e-27; // Mass of hydrogen molecule (H2) in kg
        let vrms_h2 = rms_speed(temperature_earth, mass_h2);
        assert!((vrms_h2 - 1930.0).abs() < 10.0); // Expect around 1930 m/s
    }

    #[test]
    fn test_jeans_parameter_hydrogen_earth() {
        let mass_earth = 1.0; // Mass of Earth in Earth masses
        let radius_earth = 1.0; // Radius of Earth in Earth radii
        let temperature_earth = 288; // Average temperature of Earth's surface in Kelvin
        let jeans_param_h2_earth = jeans_parameter(
            mass_earth,
            radius_earth,
            temperature_earth,
            AtmosphereElement::Hydrogen,
        );
        assert!((jeans_param_h2_earth - 5.8).abs() < 0.1); // Expect around 5.8
    }
}
