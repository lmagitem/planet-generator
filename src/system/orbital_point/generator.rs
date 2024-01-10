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

pub fn complete_orbit_with_dynamic_parameters(
    coord: SpaceCoordinates,
    system_index: u16,
    star_id: u32,
    star_age: f32,
    orbited_object_mass: f64,
    gas_giant_arrangement: GasGiantArrangement,
    orbital_point_id: u32,
    own_orbit: &Option<Orbit>,
    orbit_distance: f64,
    is_gas_giant: bool,
    blackbody_temp: u32,
    mass: f64,
    radius: f64,
    size: CelestialBodySize,
    special_traits: &mut Vec<TelluricSpecialTrait>,
    moons: &Vec<OrbitalPoint>,
    is_moon: bool,
    settings: &GenerationSettings,
) -> Orbit {
    let mut this_orbit = own_orbit.clone().unwrap_or_default();
    this_orbit.orbital_period = calculate_orbital_period_from_earth_masses(
        orbit_distance,
        orbited_object_mass,
        mass as f64,
    ) as f32;
    let (eccentricity, min_separation, max_separation) = calculate_planet_orbit_eccentricity(
        &coord,
        system_index,
        star_id,
        gas_giant_arrangement,
        orbital_point_id,
        orbit_distance,
        &settings,
        blackbody_temp,
        size,
        is_gas_giant,
        is_moon,
    );
    this_orbit.eccentricity = eccentricity as f32;
    this_orbit.min_separation = min_separation;
    this_orbit.max_separation = max_separation;

    let tidal_braking = if is_moon {
        calculate_moon_tidal_braking(
            radius,
            orbited_object_mass,
            orbit_distance,
            star_age as f64,
            mass,
        )
    } else {
        let moons_masses_and_radii: Vec<(f64, f64)> = moons
            .iter()
            .map(|o| {
                if let AstronomicalObject::TelluricBody(moon)
                | AstronomicalObject::IcyBody(moon)
                | AstronomicalObject::GaseousBody(moon) = o.object.clone()
                {
                    return (moon.mass, moon.radius);
                }
                (0.0, 0.0)
            })
            .collect::<Vec<(f64, f64)>>();
        calculate_planet_tidal_braking(
            radius,
            orbited_object_mass,
            orbit_distance,
            star_age as f64,
            mass,
            moons_masses_and_radii,
        )
    };
    this_orbit.rotation = generate_rotation_period(
        coord,
        system_index,
        star_id,
        orbital_point_id,
        &mut this_orbit,
        size,
        special_traits,
        eccentricity,
        tidal_braking,
        moons,
        is_moon,
        settings,
    );

    // TODO: - Local Calendar
    // TODO: - Axial Tilt
    this_orbit
}

fn generate_rotation_period(
    coord: SpaceCoordinates,
    system_index: u16,
    star_id: u32,
    orbital_point_id: u32,
    mut this_orbit: &mut Orbit,
    size: CelestialBodySize,
    special_traits: &mut Vec<TelluricSpecialTrait>,
    eccentricity: f64,
    tidal_braking: u32,
    moons: &Vec<OrbitalPoint>,
    is_moon: bool,
    settings: &GenerationSettings,
) -> f32 {
    let mut rng = SeededDiceRoller::new(
        &settings.seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_rot",
            coord, system_index, star_id, orbital_point_id
        ),
    );

    let mut rotation = 0.0;
    if tidal_braking >= 50 {
        tide_lock_given_body(
            special_traits,
            moons,
            this_orbit.orbital_period,
            eccentricity,
            &mut rng,
            &mut rotation,
        );
    } else {
        let modifier = if size == CelestialBodySize::Giant || size == CelestialBodySize::Large {
            6
        } else if size == CelestialBodySize::Standard {
            10
        } else if size == CelestialBodySize::Small {
            14
        } else if size == CelestialBodySize::Tiny {
            18
        } else if size == CelestialBodySize::Puny {
            22
        } else {
            0
        };

        let mut unusually_slow_rotation = false;
        let mut roll = rng.roll(3, 500, 297) as f32 / 100.0;

        roll = if special_traits.contains(&TelluricSpecialTrait::UnusualRotation(
            TelluricRotationDifference::Fast,
        )) {
            special_traits.retain(|t| {
                *t != TelluricSpecialTrait::UnusualRotation(TelluricRotationDifference::Fast)
            });
            roll / 2.0
        } else {
            roll
        };
        if roll >= 16.0
            || special_traits.contains(&TelluricSpecialTrait::UnusualRotation(
                TelluricRotationDifference::Slow,
            ))
        {
            special_traits.retain(|t| {
                *t != TelluricSpecialTrait::UnusualRotation(TelluricRotationDifference::Slow)
            });
            unusually_slow_rotation = true
        }
        roll += modifier as f32 + tidal_braking as f32;
        if roll >= 36.0 {
            unusually_slow_rotation = true
        }

        rotation = roll / 24.0;

        if unusually_slow_rotation {
            let slow_roll = rng.roll(2, 6, 0);

            if slow_roll == 7 {
                rotation = rng.roll(1, 6, 0) as f32 * 2.0
            } else if slow_roll == 8 {
                rotation = rng.roll(1, 6, 0) as f32 * 5.0
            } else if slow_roll == 9 {
                rotation = rng.roll(1, 6, 0) as f32 * 10.0
            } else if slow_roll == 10 {
                rotation = rng.roll(1, 6, 0) as f32 * 20.0
            } else if slow_roll == 11 {
                rotation = rng.roll(1, 6, 0) as f32 * 50.0
            } else if slow_roll == 12 {
                rotation = rng.roll(1, 6, 0) as f32 * 100.0
            }
            if slow_roll > 6 {
                rotation += rng.roll(1, 101, -1) as f32 - 50.0;
            }
        }

        if rotation >= this_orbit.orbital_period {
            tide_lock_given_body(
                special_traits,
                moons,
                this_orbit.orbital_period,
                eccentricity,
                &mut rng,
                &mut rotation,
            );
        }
    }

    let retrograde_roll = rng.roll(3, 6, 0);
    rotation = if !is_moon && retrograde_roll >= 13 || retrograde_roll >= 17 {
        special_traits.push(TelluricSpecialTrait::RetrogradeRotation);
        -rotation
    } else {
        rotation
    };

    if special_traits.is_empty() {
        special_traits.push(TelluricSpecialTrait::NoPeculiarity);
    } else if special_traits.len() > 1 {
        special_traits.retain(|t| *t != TelluricSpecialTrait::NoPeculiarity);
    }

    rotation
}

/// Tide-lock the given body, to its closest major moon if it's a planet that has one, to the body it orbits instead if not..
fn tide_lock_given_body(
    special_traits: &mut Vec<TelluricSpecialTrait>,
    moons: &Vec<OrbitalPoint>,
    orbital_period: f32,
    eccentricity: f64,
    rng: &mut SeededDiceRoller,
    rotation: &mut f32,
) {
    let closest_major_moon = moons
        .iter()
        .filter(|moon_point| {
            if let AstronomicalObject::TelluricBody(moon) = moon_point.object.clone() {
                moon.size != CelestialBodySize::Puny
            } else {
                false
            }
        })
        .min_by(|a, b| {
            a.own_orbit
                .clone()
                .unwrap_or_default()
                .average_distance
                .partial_cmp(&b.own_orbit.clone().unwrap_or_default().average_distance)
                .expect("Should have been able to compare two distances.")
        });
    let moon_period;

    let mut to_add = if let Some(major_moon) = closest_major_moon {
        moon_period = major_moon
            .own_orbit
            .clone()
            .unwrap_or_default()
            .orbital_period;
        TelluricSpecialTrait::TideLocked(TideLockTarget::Satellite)
    } else {
        moon_period = 0.0;
        TelluricSpecialTrait::TideLocked(TideLockTarget::Orbited)
    };

    if eccentricity >= 0.1 {
        if rng.roll(3, 6, 0) >= 16 {
            to_add = TelluricSpecialTrait::UnusualRotation(TelluricRotationDifference::Resonant);
            *rotation = 2.0 / 3.0 * orbital_period;
        }
    }

    if to_add == TelluricSpecialTrait::TideLocked(TideLockTarget::Satellite) {
        *rotation = moon_period;
    } else if to_add == TelluricSpecialTrait::TideLocked(TideLockTarget::Orbited) {
        *rotation = orbital_period;
    }

    if !special_traits.contains(&to_add) {
        special_traits.push(to_add);
    }
}

fn calculate_planet_tidal_braking(
    planet_radius_in_earth_radii: f64,
    star_mass_in_earth_masses: f64,
    orbit_distance: f64,
    system_age_in_billion_years: f64,
    planet_mass_in_earth_masses: f64,
    moons_masses_and_radii: Vec<(f64, f64)>,
) -> u32 {
    let star_mass_in_solar_masses =
        ConversionUtils::earth_mass_to_solar_mass(star_mass_in_earth_masses);

    // Tidal Force by Star
    let tidal_force_star =
        0.46 * star_mass_in_solar_masses * planet_radius_in_earth_radii / orbit_distance.powi(3);

    // Tidal Force by Moons
    let tidal_force_moons: f64 = moons_masses_and_radii
        .iter()
        .map(|(moon_mass, moon_orbit_radius_in_earth_radii)| {
            if MathUtils::does_f64_equal_zero(*moon_mass)
                || MathUtils::does_f64_equal_zero(*moon_orbit_radius_in_earth_radii)
            {
                return 0.0;
            }
            let moon_orbit_radius_in_earth_diameter = moon_orbit_radius_in_earth_radii / 2.0;
            17.8e6 * moon_mass * planet_radius_in_earth_radii
                / moon_orbit_radius_in_earth_diameter.powi(3)
        })
        .sum();

    // Total Tidal Effect
    let total_tidal_effect = ((tidal_force_star + tidal_force_moons) * system_age_in_billion_years)
        / planet_mass_in_earth_masses;

    total_tidal_effect.round() as u32
}

fn calculate_moon_tidal_braking(
    moon_radius_in_earth_radii: f64,
    planet_mass_in_earth_masses: f64,
    moon_orbit_radius_in_astronomical_units: f64,
    system_age_in_billion_years: f64,
    moon_mass_in_earth_masses: f64,
) -> u32 {
    let moon_orbit_radius_in_earth_diameters =
        ConversionUtils::astronomical_units_to_earth_radii(moon_orbit_radius_in_astronomical_units)
            * 2.0;

    // Tidal Force by Planet on Moon
    let tidal_force_planet = 17.8e6 * planet_mass_in_earth_masses * moon_radius_in_earth_radii
        / moon_orbit_radius_in_earth_diameters.powi(3);

    // Total Tidal Effect on Moon
    let total_tidal_effect_moon =
        (tidal_force_planet * system_age_in_billion_years) / moon_mass_in_earth_masses;

    total_tidal_effect_moon.round() as u32
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
    size: CelestialBodySize,
    is_gas_giant: bool,
    is_moon: bool,
) -> (f64, f64, f64) {
    let mut rng = SeededDiceRoller::new(
        &settings.seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_ect",
            coord, system_index, star_id, orbital_point_id
        ),
    );
    let mut eccentricity_modifier =
        if !is_moon && gas_giant_arrangement == GasGiantArrangement::ConventionalGasGiant {
            -6
        } else if is_gas_giant
            && gas_giant_arrangement == GasGiantArrangement::EccentricGasGiant
            && blackbody_temp < 170
        {
            -4
        } else if is_moon && size != CelestialBodySize::Puny {
            -2
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
