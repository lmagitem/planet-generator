use crate::internal::ConversionUtils;

/// Returns a value in Kelvin
pub(crate) fn calculate_blackbody_temperature(luminosity: f32, orbital_radius: f64) -> u32 {
    if orbital_radius <= 0.0 {
        panic!("Orbital radius should be greater than 0");
    }

    let b = 278.0 * ((luminosity as f64).powf(0.25)) / (orbital_radius).sqrt();
    b.round() as u32
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
pub(crate) fn calculate_surface_gravity(density_g_cm3: f32, radius_earth_radii: f32) -> f32 {
    (density_g_cm3 / 5.513) * radius_earth_radii
}

/// Calculates the Roche limit based on the masses of the primary and the satellite.
/// The radius of the satellite is in Earth radii, mass can be any shared unit, and the return value is in AU.
pub fn calculate_roche_limit(radius_satellite: f64, mass_primary: f64, mass_satellite: f64) -> f64 {
    let earth_radius_to_au: f64 = 1.0 / 215.032;
    let radius_satellite_au = radius_satellite * earth_radius_to_au;

    radius_satellite_au * (2.0 * mass_primary / mass_satellite).powf(1.0 / 3.0)
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
        let gravity = calculate_surface_gravity(earth_density as f32, earth_radius as f32);
        assert!(within_error_margin(gravity as f64, 1.0));
    }

    #[test]
    fn test_calculate_surface_gravity_mars() {
        let mars_density = 3.93;
        let mars_radius = 0.532;
        let gravity = calculate_surface_gravity(mars_density as f32, mars_radius as f32);
        assert!(within_error_margin(gravity as f64, 0.38));
    }

    #[test]
    fn test_calculate_surface_gravity_jupiter() {
        let jupiter_density = 1.33;
        let jupiter_radius = 11.2;
        let gravity = calculate_surface_gravity(jupiter_density as f32, jupiter_radius as f32);
        assert!(within_error_margin(gravity as f64, 2.528));
    }

    #[test]
    fn test_calculate_surface_gravity_saturn() {
        let saturn_density = 0.69;
        let saturn_radius = 9.45;
        let gravity = calculate_surface_gravity(saturn_density as f32, saturn_radius as f32);
        assert!(within_error_margin(gravity as f64, 1.065));
    }

    #[test]
    fn test_calculate_surface_gravity_ganymede() {
        let ganymede_density = 1.942;
        let ganymede_radius = 0.413;
        let gravity = calculate_surface_gravity(ganymede_density as f32, ganymede_radius as f32);
        assert!(within_error_margin(gravity as f64, 0.146));
    }

    #[test]
    fn test_calculate_surface_gravity_moon() {
        let moon_density = 3.344;
        let moon_radius = 0.273;
        let gravity = calculate_surface_gravity(moon_density as f32, moon_radius as f32);
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
        let titan_radius: f64 = 0.00040356612;
        let saturn_mass: f64 = 95.16;
        let titan_mass: f64 = 2.30424618e-22;
        let expected_roche_limit_au =
            titan_radius * (2.0 * saturn_mass / titan_mass).powf(1.0 / 3.0) / 215.032;

        let roche_limit = calculate_roche_limit(titan_radius, saturn_mass, titan_mass);
        assert!((roche_limit - expected_roche_limit_au).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_roche_limit_mass_star_planet() {
        let earth_radius: f64 = 1.0;
        let sun_mass: f64 = 333000.0;
        let earth_mass: f64 = 1.0;
        let expected_roche_limit_au =
            earth_radius * (2.0 * sun_mass / earth_mass).powf(1.0 / 3.0) / 215.032;

        let roche_limit = calculate_roche_limit(earth_radius, sun_mass, earth_mass);
        assert!((roche_limit - expected_roche_limit_au).abs() < EPSILON);
    }
}
