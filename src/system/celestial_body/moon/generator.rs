use crate::internal::types::MoonDistance;
use crate::internal::*;
use crate::prelude::*;
use crate::system::contents::generator::{
    generate_body_from_type, generate_inner_body_type, generate_outer_body_type,
};
use crate::system::contents::utils::{calculate_hill_sphere_radius, calculate_roche_limit};
use crate::system::orbital_point::generator::{
    complete_orbit_with_orbital_period, complete_orbit_with_rotation_and_axis,
};
use crate::system::orbital_point::utils::sort_orbital_points_by_average_distance;
use std::iter::Filter;
use std::slice::Iter;

impl MoonGenerator {
    pub(crate) fn generate_planets_moons(
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_age: f32,
        star_mass: f64,
        star_luminosity: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f64,
        parent_orbit: Orbit,
        coord: SpaceCoordinates,
        seed: &Rc<str>,
        next_id: &mut u32,
        gas_giant_arrangement: GasGiantArrangement,
        mut populated_orbit_index: u32,
        planet_id: u32,
        planet_size: CelestialBodySize,
        planet_mass: f64,
        planet_density: f32,
        planet_radius: f64,
        planet_orbital_period: f32,
        blackbody_temperature: u32,
        settings: &GenerationSettings,
        is_moon: bool,
    ) -> Vec<OrbitalPoint> {
        let mut result = Vec::new();
        if is_moon {
            return result;
        }

        let (
            number_of_major_moons,
            number_of_moonlets,
            number_of_inner_moonlets,
            number_of_outer_moonlets,
        ) = Self::get_planet_number_of_moons(
            coord,
            system_index,
            star_id,
            planet_id,
            *&parent_orbit.average_distance,
            planet_size,
            &settings,
        );

        Self::generate_moons(
            system_traits,
            system_index,
            star_id,
            star_name,
            star_age,
            star_mass,
            star_luminosity,
            star_type,
            star_class,
            star_traits,
            primary_star_mass,
            parent_orbit,
            coord,
            seed,
            next_id,
            gas_giant_arrangement,
            populated_orbit_index,
            planet_id,
            planet_size,
            planet_mass,
            planet_density,
            planet_radius,
            planet_orbital_period,
            blackbody_temperature,
            &settings,
            &mut result,
            number_of_major_moons,
            number_of_moonlets,
            number_of_inner_moonlets,
            number_of_outer_moonlets,
        );

        result
    }

    pub(crate) fn generate_giants_moons(
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_age: f32,
        star_mass: f64,
        star_luminosity: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f64,
        parent_orbit: Orbit,
        coord: SpaceCoordinates,
        seed: &Rc<str>,
        next_id: &mut u32,
        gas_giant_arrangement: GasGiantArrangement,
        mut populated_orbit_index: u32,
        planet_id: u32,
        planet_size: CelestialBodySize,
        planet_mass: f64,
        planet_density: f32,
        planet_radius: f64,
        planet_orbital_period: f32,
        blackbody_temperature: u32,
        settings: GenerationSettings,
        is_moon: bool,
    ) -> Vec<OrbitalPoint> {
        let mut result = Vec::new();
        if is_moon {
            return result;
        }

        let (
            number_of_major_moons,
            number_of_moonlets,
            number_of_inner_moonlets,
            number_of_outer_moonlets,
        ) = Self::get_giant_number_of_moons(
            &system_index,
            &star_id,
            *&parent_orbit.average_distance,
            &coord,
            &planet_id,
            planet_size,
            &settings,
        );
        if planet_size == CelestialBodySize::Giant
            || planet_size == CelestialBodySize::Supergiant
            || planet_size == CelestialBodySize::Hypergiant
        {
            Self::add_giants_ring(
                system_index,
                star_id,
                star_name.clone(),
                star_mass,
                parent_orbit.clone(),
                coord,
                next_id,
                populated_orbit_index,
                planet_id,
                planet_mass,
                planet_density,
                planet_radius,
                blackbody_temperature,
                &settings,
                &mut result,
                number_of_inner_moonlets,
            );
        }

        Self::generate_moons(
            system_traits,
            system_index,
            star_id,
            star_name,
            star_age,
            star_mass,
            star_luminosity,
            star_type,
            star_class,
            star_traits,
            primary_star_mass,
            parent_orbit,
            coord,
            seed,
            next_id,
            gas_giant_arrangement,
            populated_orbit_index,
            planet_id,
            planet_size,
            planet_mass,
            planet_density,
            planet_radius,
            planet_orbital_period,
            blackbody_temperature,
            &settings,
            &mut result,
            number_of_major_moons,
            number_of_moonlets,
            number_of_inner_moonlets,
            number_of_outer_moonlets,
        );

        result
    }

    fn generate_moons(
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_age: f32,
        star_mass: f64,
        star_luminosity: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f64,
        parent_orbit: Orbit,
        coord: SpaceCoordinates,
        seed: &Rc<str>,
        next_id: &mut u32,
        gas_giant_arrangement: GasGiantArrangement,
        mut populated_orbit_index: u32,
        planet_id: u32,
        planet_size: CelestialBodySize,
        planet_mass: f64,
        planet_density: f32,
        planet_radius: f64,
        planet_orbital_period: f32,
        blackbody_temperature: u32,
        settings: &GenerationSettings,
        result: &mut Vec<OrbitalPoint>,
        mut number_of_major_moons: i8,
        mut number_of_moonlets: i8,
        mut number_of_inner_moonlets: i8,
        mut number_of_outer_moonlets: i8,
    ) {
        let mut moon_stubs: Vec<OrbitalPoint> = Vec::new();
        let separate_inner_moonlets_and_major_moons = if SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_orbit",
                coord, system_index, star_id, planet_id
            ),
        )
        .roll(1, 10, 0)
            == 1
        {
            true
        } else {
            false
        };
        let initial_number_of_major_moons = number_of_major_moons;
        let orbits_to_generate = number_of_major_moons
            + number_of_moonlets
            + number_of_inner_moonlets
            + number_of_outer_moonlets;
        let mut closest_major_distance = f64::MAX;
        let mut record_closest_distance = false;

        for moon_orbit_index in 0..orbits_to_generate {
            let moon_id = *next_id;
            *next_id += 1;

            let mut rng = SeededDiceRoller::new(
                &settings.seed,
                &format!(
                    "sys_{}_{}_str_{}_bdy{}_type",
                    coord, system_index, star_id, moon_id
                ),
            );

            let orbit = Some(Orbit {
                primary_body_id: planet_id,
                id: Some(moon_id),
                average_distance_from_system_center: parent_orbit.average_distance_from_system_center,
                zone: parent_orbit.zone,
                ..Default::default()
            });
            let mut moon_distance;
            let fixed_size = if number_of_major_moons > 0 {
                record_closest_distance = true;
                number_of_major_moons += -1;
                moon_distance = if planet_size == CelestialBodySize::Giant
                    || planet_size == CelestialBodySize::Supergiant
                    || planet_size == CelestialBodySize::Hypergiant
                {
                    MoonDistance::MajorGiantClose
                } else {
                    MoonDistance::MajorPlanetClose
                };
                Some(Self::generate_moon_size(&mut rng, planet_size))
            } else if number_of_moonlets > 0 {
                record_closest_distance = false;
                number_of_moonlets += -1;
                moon_distance = MoonDistance::Close;
                Some(CelestialBodySize::Puny)
            } else if number_of_inner_moonlets > 0 {
                record_closest_distance = false;
                number_of_inner_moonlets += -1;
                moon_distance = if initial_number_of_major_moons > 0 {
                    MoonDistance::BeforeMajor
                } else {
                    MoonDistance::Close
                };
                Some(CelestialBodySize::Puny)
            } else {
                record_closest_distance = false;
                number_of_outer_moonlets += -1;
                moon_distance = MoonDistance::MediumOrFar;
                Some(CelestialBodySize::Puny)
            };
            let body_type = {
                let celestial_body_settings = &settings.celestial_body;
                let celestial_body_settings = CelestialBodySettings {
                    do_not_generate_gaseous: true,
                    ..celestial_body_settings.clone()
                };
                let settings = GenerationSettings {
                    celestial_body: celestial_body_settings,
                    ..settings.clone()
                };

                let moon_type = if blackbody_temperature >= 170 {
                    generate_inner_body_type(&mut rng, settings.clone())
                } else {
                    generate_outer_body_type(&mut rng, settings.clone())
                };

                if moon_type == CelestialBodyComposition::Metallic {
                    TelluricBodyComposition::Metallic
                } else if moon_type == CelestialBodyComposition::Icy {
                    TelluricBodyComposition::Icy
                } else {
                    TelluricBodyComposition::Rocky
                }
            };
            let mut moon_stub = generate_body_from_type(
                system_traits,
                system_index,
                star_id,
                star_name.clone(),
                star_age,
                star_mass,
                star_luminosity,
                star_type,
                star_class,
                star_traits,
                primary_star_mass,
                coord,
                seed,
                next_id,
                gas_giant_arrangement,
                populated_orbit_index,
                0,
                body_type,
                moon_id,
                orbit.clone(),
                *&parent_orbit.average_distance,
                Vec::new(),
                settings.clone(),
                true,
                fixed_size,
            )
            .0;
            if let AstronomicalObject::TelluricBody(ref mut body) = moon_stub.object {
                body.name = format!(
                    "{}{}",
                    body.name,
                    StringUtils::number_to_lowercase_letter(moon_orbit_index as u8 + 1)
                )
                .into();
            } else if let AstronomicalObject::IcyBody(ref mut body) = moon_stub.object {
                body.name = format!(
                    "{}{}",
                    body.name,
                    StringUtils::number_to_lowercase_letter(moon_orbit_index as u8 + 1)
                )
                .into();
            }

            let moon_clone = Self::clone_moon_body(&moon_stub);

            let mut rng = SeededDiceRoller::new(
                &settings.seed,
                &format!(
                    "sys_{}_{}_str_{}_bdy{}_orbit",
                    coord, system_index, star_id, moon_id
                ),
            );
            let max_attempts = 50;
            let mut attempt_count = 0;
            let mut found = false;
            let mut moon_orbit_distance = 0.0;
            while attempt_count <= max_attempts && !found {
                let ring_distance = result
                    .iter()
                    .find(|o| {
                        if let AstronomicalObject::TelluricDisk(ring) = o.object.clone() {
                            return true;
                        } else if let AstronomicalObject::IcyDisk(ring) = o.object.clone() {
                            return true;
                        } else {
                            return false;
                        }
                    })
                    .unwrap_or(&OrbitalPoint::new(
                        0,
                        Some(Orbit::default()),
                        AstronomicalObject::Void,
                        Vec::new(),
                    ))
                    .own_orbit
                    .clone()
                    .unwrap_or_default()
                    .average_distance;

                moon_orbit_distance = Self::generate_moon_orbit_distance(
                    &mut rng,
                    star_mass as f64,
                    *&parent_orbit.average_distance,
                    planet_mass as f64,
                    planet_density as f64,
                    planet_radius as f64,
                    moon_clone.mass as f64,
                    moon_clone.density as f64,
                    moon_clone.radius as f64,
                    moon_distance,
                    ring_distance,
                    closest_major_distance,
                );

                if moon_orbit_distance > 0.0 {
                    let mut conflict = false;
                    let mut highest_blocking_distance = 0.0;
                    for existing_moon_point in &moon_stubs {
                        let existing_moon = Self::clone_moon_body(existing_moon_point);
                        let existing_moon_distance = existing_moon_point
                            .own_orbit
                            .clone()
                            .unwrap_or_default()
                            .average_distance;
                        let roche_limit = calculate_roche_limit(
                            existing_moon.radius as f64,
                            existing_moon.density as f64,
                            moon_clone.density as f64,
                        );
                        let hill_sphere = calculate_hill_sphere_radius(
                            existing_moon_distance,
                            existing_moon.mass as f64,
                            planet_mass as f64,
                        );

                        highest_blocking_distance = if roche_limit > hill_sphere {
                            roche_limit
                        } else {
                            hill_sphere
                        };

                        if moon_orbit_distance >= existing_moon_distance - highest_blocking_distance
                            && moon_orbit_distance
                                <= existing_moon_distance + highest_blocking_distance
                        {
                            conflict = true;
                            break;
                        }
                    }

                    if !conflict {
                        let distance_minus_influence =
                            moon_orbit_distance - highest_blocking_distance;
                        if record_closest_distance
                            && closest_major_distance > distance_minus_influence
                        {
                            closest_major_distance = distance_minus_influence;
                        }
                        let orbit = Some(Orbit {
                            average_distance: moon_orbit_distance,
                            ..orbit.clone().unwrap_or_default()
                        });
                        let moon_clone = if let AstronomicalObject::TelluricBody(moon) =
                            moon_stub.clone().object
                        {
                            moon
                        } else {
                            CelestialBody::default()
                        };
                        let mut special_traits: Vec<CelestialBodySpecialTrait> =
                            if let CelestialBodyDetails::Telluric(moon_details) = moon_clone.details
                            {
                                moon_details.special_traits
                            } else {
                                Vec::new()
                            };
                        moon_stub.own_orbit = Some(complete_orbit_with_rotation_and_axis(
                            coord,
                            system_index,
                            star_id,
                            star_age,
                            ConversionUtils::solar_mass_to_earth_mass(star_mass),
                            Some(planet_orbital_period),
                            gas_giant_arrangement,
                            system_traits,
                            moon_id,
                            &Some(complete_orbit_with_orbital_period(
                                coord,
                                system_index,
                                star_id,
                                ConversionUtils::solar_mass_to_earth_mass(star_mass),
                                gas_giant_arrangement,
                                moon_id,
                                &moon_stub.own_orbit,
                                moon_orbit_distance,
                                *&parent_orbit.average_distance_from_system_center,
                                false,
                                blackbody_temperature,
                                moon_clone.mass,
                                moon_clone.size,
                                true,
                                &settings,
                            )),
                            moon_orbit_distance,
                            false,
                            blackbody_temperature,
                            moon_clone.mass,
                            moon_clone.radius,
                            moon_clone.size,
                            &mut special_traits,
                            &Vec::new(),
                            true,
                            moon_distance,
                            &settings,
                        ));

                        if let AstronomicalObject::TelluricBody(moon) = &mut moon_stub.object {
                            if let CelestialBodyDetails::Telluric(moon_details) = &mut moon.details
                            {
                                moon_details.special_traits = special_traits.clone();
                            }
                        }
                        moon_stubs.push(moon_stub.clone());
                        found = true;
                    } else if attempt_count == max_attempts {
                        if moon_distance == MoonDistance::Close {
                            moon_distance = MoonDistance::Medium;
                            attempt_count = 0;
                        } else if moon_distance == MoonDistance::Medium {
                            moon_distance = MoonDistance::Far;
                            attempt_count = 0;
                        }
                    }
                }
                attempt_count += 1;
            }
        }

        sort_orbital_points_by_average_distance(&mut moon_stubs);
        let tidal_heating_array = OrbitalHarmonicsUtils::calculate_gravitational_harmonics(
            &OrbitalHarmonicsUtils::prepare_harmonics_array(&moon_stubs, true),
            0.03,
        );

        for i in 0..moon_stubs.len() {
            let (stub_id, stub_orbit, stub_orbits, stub_body) = {
                let current_stub = &moon_stubs[i];
                (
                    current_stub.id,
                    current_stub.own_orbit.clone(),
                    current_stub.orbits.clone(),
                    current_stub.object.clone(),
                )
            };

            let tidal_heating = tidal_heating_array[i];
            match stub_body {
                AstronomicalObject::TelluricBody(stub_body) => {
                    let polished = WorldGenerator::generate_world(
                        coord,
                        system_traits,
                        system_index,
                        star_id,
                        star_age,
                        star_type,
                        star_class,
                        star_traits,
                        *&parent_orbit.average_distance,
                        populated_orbit_index,
                        stub_id,
                        stub_orbit.unwrap_or_default(),
                        stub_orbits,
                        stub_body,
                        true,
                        &Vec::new(),
                        tidal_heating,
                        seed.clone(),
                        settings.clone(),
                    );
                    result.push(polished);
                }
                _ => {}
            }
        }
    }

    fn clone_moon_body(existing_moon_point: &OrbitalPoint) -> CelestialBody {
        let existing_moon =
            if let AstronomicalObject::TelluricBody(body) = existing_moon_point.object.clone() {
                body
            } else if let AstronomicalObject::IcyBody(body) = existing_moon_point.object.clone() {
                body
            } else {
                CelestialBody::default()
            };
        existing_moon
    }

    pub(crate) fn generate_moon_orbit_distance(
        rng: &mut SeededDiceRoller,
        star_mass: f64,
        orbit_distance_from_star: f64,
        planet_mass: f64,
        planet_density: f64,
        planet_radius: f64,
        moon_mass: f64,
        moon_density: f64,
        moon_radius: f64,
        moon_distance: MoonDistance,
        ring_distance: f64,
        closest_major_distance: f64,
    ) -> f64 {
        let (min_distance, max_distance) = Self::get_min_and_max_moon_distance(
            star_mass,
            orbit_distance_from_star,
            planet_mass,
            planet_density,
            planet_radius,
            moon_mass,
            moon_density,
            moon_radius,
            moon_distance,
            ring_distance,
            closest_major_distance,
        );
        let moon_orbit_distance = if min_distance < max_distance {
            rng.gen_range(min_distance..max_distance)
        } else {
            -1.0
        };
        moon_orbit_distance
    }

    fn get_min_and_max_moon_distance(
        star_mass: f64,
        orbit_distance_from_star: f64,
        planet_mass: f64,
        planet_density: f64,
        planet_radius: f64,
        moon_mass: f64,
        moon_density: f64,
        moon_radius: f64,
        moon_distance: MoonDistance,
        ring_distance: f64,
        closest_major_distance: f64,
    ) -> (f64, f64) {
        let diameter = planet_radius * 2.0;
        let mut min_distance = calculate_roche_limit(planet_radius, planet_density, moon_density);
        if ring_distance > min_distance {
            min_distance = ring_distance;
        }
        let hill_sphere_radius = calculate_hill_sphere_radius(
            orbit_distance_from_star,
            ConversionUtils::earth_mass_to_solar_mass(planet_mass),
            star_mass,
        );
        let min_ring_distance = ConversionUtils::earth_radii_to_astronomical_units((diameter));
        let max_ring_distance =
            min_distance + ConversionUtils::earth_radii_to_astronomical_units((diameter * 0.2));
        let min_major_giant_close_distance = Self::get_distance_within_bounds(
            ConversionUtils::earth_radii_to_astronomical_units((diameter * 2.5)),
            min_distance,
            hill_sphere_radius,
        );
        let min_major_planet_close_distance = Self::get_distance_within_bounds(
            ConversionUtils::earth_radii_to_astronomical_units((diameter * 5.0)),
            min_distance,
            hill_sphere_radius,
        );
        let max_close_distance = Self::get_distance_within_bounds(
            ConversionUtils::earth_radii_to_astronomical_units((diameter * 15.0)),
            min_distance,
            hill_sphere_radius,
        );
        let max_medium_distance = Self::get_distance_within_bounds(
            ConversionUtils::earth_radii_to_astronomical_units((diameter * 60.0)),
            min_distance,
            hill_sphere_radius,
        );
        let max_far_distance = Self::get_distance_within_bounds(
            ConversionUtils::earth_radii_to_astronomical_units((diameter * 180.0)),
            min_distance,
            hill_sphere_radius,
        );

        match moon_distance {
            MoonDistance::Any => (min_distance, max_far_distance),
            MoonDistance::Ring => (
                Self::get_distance_within_bounds(
                    min_ring_distance,
                    min_ring_distance,
                    hill_sphere_radius,
                ),
                Self::get_distance_within_bounds(
                    max_ring_distance,
                    max_ring_distance,
                    hill_sphere_radius,
                ),
            ),
            MoonDistance::BeforeMajor => (min_distance, closest_major_distance),
            MoonDistance::Close => Self::get_appropriate_moon_distance_values(
                min_distance,
                &[
                    max_close_distance,
                    max_medium_distance,
                    max_far_distance,
                    hill_sphere_radius,
                ],
            ),
            MoonDistance::MajorGiantClose => Self::get_appropriate_moon_distance_values(
                Self::get_distance_within_bounds(
                    min_major_giant_close_distance,
                    min_distance,
                    hill_sphere_radius,
                ),
                &[
                    max_close_distance,
                    max_medium_distance,
                    max_far_distance,
                    hill_sphere_radius,
                ],
            ),
            MoonDistance::MajorPlanetClose => Self::get_appropriate_moon_distance_values(
                Self::get_distance_within_bounds(
                    min_major_planet_close_distance,
                    min_distance,
                    hill_sphere_radius,
                ),
                &[
                    max_close_distance,
                    max_medium_distance,
                    max_far_distance,
                    hill_sphere_radius,
                ],
            ),
            MoonDistance::Medium => Self::get_appropriate_moon_distance_values(
                max_close_distance,
                &[max_medium_distance, max_far_distance, hill_sphere_radius],
            ),
            MoonDistance::MediumOrFar => Self::get_appropriate_moon_distance_values(
                max_close_distance,
                &[max_far_distance, hill_sphere_radius],
            ),
            MoonDistance::Far => Self::get_appropriate_moon_distance_values(
                max_medium_distance,
                &[max_far_distance, hill_sphere_radius],
            ),
        }
    }

    fn get_appropriate_moon_distance_values(
        min_value: f64,
        potential_max_values: &[f64],
    ) -> (f64, f64) {
        for &max_value in potential_max_values {
            if max_value > min_value {
                return (min_value, max_value);
            }
        }
        // If all max values are less than or equal to the min value, return the last one
        (
            min_value,
            *potential_max_values.last().unwrap_or(&min_value),
        )
    }

    fn get_distance_within_bounds(planet_radius: f64, roche_limit: f64, hill_sphere: f64) -> f64 {
        if planet_radius < roche_limit {
            roche_limit
        } else if planet_radius > hill_sphere {
            hill_sphere
        } else {
            planet_radius
        }
    }

    fn generate_moon_size(
        mut rng: &mut SeededDiceRoller,
        size: CelestialBodySize,
    ) -> CelestialBodySize {
        let size_roll = rng.roll(3, 6, 0);
        if size_roll <= 11 {
            match size {
                CelestialBodySize::Hypergiant => CelestialBodySize::Large,
                CelestialBodySize::Supergiant => CelestialBodySize::Standard,
                CelestialBodySize::Giant => CelestialBodySize::Small,
                CelestialBodySize::Large => CelestialBodySize::Tiny,
                _ => CelestialBodySize::Puny,
            }
        } else if size_roll <= 14 {
            match size {
                CelestialBodySize::Hypergiant | CelestialBodySize::Supergiant => {
                    CelestialBodySize::Large
                }
                CelestialBodySize::Giant => CelestialBodySize::Standard,
                CelestialBodySize::Large => CelestialBodySize::Small,
                CelestialBodySize::Standard => CelestialBodySize::Tiny,
                _ => CelestialBodySize::Puny,
            }
        } else {
            match size {
                CelestialBodySize::Hypergiant
                | CelestialBodySize::Supergiant
                | CelestialBodySize::Giant => CelestialBodySize::Large,
                CelestialBodySize::Large => CelestialBodySize::Standard,
                CelestialBodySize::Standard => CelestialBodySize::Small,
                CelestialBodySize::Small => CelestialBodySize::Tiny,
                _ => CelestialBodySize::Puny,
            }
        }
    }

    fn get_planet_number_of_moons(
        coord: SpaceCoordinates,
        system_index: u16,
        star_id: u32,
        orbital_point_id: u32,
        orbit_distance: f64,
        size: CelestialBodySize,
        settings: &GenerationSettings,
    ) -> (i8, i8, i8, i8) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_moons",
                coord, system_index, star_id, orbital_point_id
            ),
        );
        let mut modifier = if orbit_distance < 0.5 {
            -6
        } else if orbit_distance < 0.75 {
            -3
        } else if orbit_distance < 1.5 {
            -1
        } else {
            0
        };
        modifier += if size == CelestialBodySize::Tiny {
            -2
        } else if size == CelestialBodySize::Small {
            -1
        } else if size == CelestialBodySize::Large {
            1
        } else {
            0
        };
        let major_moons: i8 = rng.roll(1, 6, -4 + modifier) as i8;
        let moonlets: i8 = if major_moons > 0 {
            0
        } else {
            rng.roll(1, 6, -2 + modifier) as i8
        };
        (major_moons, moonlets, 0, 0)
    }

    fn get_giant_number_of_moons(
        system_index: &u16,
        star_id: &u32,
        orbit_distance_from_star: f64,
        coord: &SpaceCoordinates,
        planet_id: &u32,
        planet_size: CelestialBodySize,
        settings: &GenerationSettings,
    ) -> (i8, i8, i8, i8) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_gas_bdy{}_moons",
                coord, system_index, star_id, planet_id
            ),
        );

        let size_modifier = if planet_size == CelestialBodySize::Hypergiant {
            0
        } else if planet_size == CelestialBodySize::Supergiant {
            -1
        } else if planet_size == CelestialBodySize::Giant {
            -2
        } else {
            -4
        };
        let inner_moonlets_modifier = if orbit_distance_from_star < 0.1 {
            -12
        } else if orbit_distance_from_star < 0.5 {
            -9
        } else if orbit_distance_from_star < 0.75 {
            -6
        } else if orbit_distance_from_star < 1.5 {
            -3
        } else {
            0
        };
        let major_moons_modifier = if orbit_distance_from_star < 0.1 {
            -6
        } else if orbit_distance_from_star < 0.5 {
            -5
        } else if orbit_distance_from_star < 0.75 {
            -4
        } else if orbit_distance_from_star < 1.5 {
            -1
        } else {
            0
        };
        let outer_moonlets_modifier = if orbit_distance_from_star < 0.5 {
            -6
        } else if orbit_distance_from_star < 0.75 {
            -5
        } else if orbit_distance_from_star < 1.5 {
            -4
        } else if orbit_distance_from_star < 3.0 {
            -1
        } else {
            0
        };

        let inner_moonlets: i8 = rng.roll(2, 8, inner_moonlets_modifier + size_modifier) as i8;
        let major_moons: i8 = rng.roll(1, 8, major_moons_modifier + size_modifier) as i8;
        let outer_moonlets: i8 = rng.roll(1, 10, outer_moonlets_modifier + size_modifier) as i8;

        (major_moons, 0, inner_moonlets, outer_moonlets)
    }

    fn add_giants_ring(
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_mass: f64,
        parent_orbit: Orbit,
        coord: SpaceCoordinates,
        next_id: &mut u32,
        mut populated_orbit_index: u32,
        planet_id: u32,
        planet_mass: f64,
        planet_density: f32,
        planet_radius: f64,
        blackbody_temperature: u32,
        settings: &GenerationSettings,
        moons: &mut Vec<OrbitalPoint>,
        moonlets: i8,
    ) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_gas_bdy{}_ring",
                coord, system_index, star_id, planet_id
            ),
        );
        let ring_composition = if moonlets < 4 {
            CelestialRingComposition::Dust
        } else {
            rng.get_result(&CopyableRollToProcess::new(
                vec![
                    CopyableWeightedResult::new(
                        CelestialRingComposition::Ice,
                        if blackbody_temperature < 241 {
                            12
                        } else if blackbody_temperature < 300 {
                            1
                        } else {
                            0
                        },
                    ),
                    CopyableWeightedResult::new(
                        CelestialRingComposition::Rock,
                        if blackbody_temperature < 241 { 5 } else { 12 },
                    ),
                    CopyableWeightedResult::new(CelestialRingComposition::Metal, 1),
                ],
                RollMethod::SimpleRoll,
            ))
            .expect("Should have picked a ring composition.")
        };

        let ring_id = *next_id;
        *next_id += 1;
        let ring_mass = (moonlets as f64) * 2.0 * 10.0f64.powf(-7.0);

        let ring_distance = MoonGenerator::generate_moon_orbit_distance(
            &mut SeededDiceRoller::new(
                &settings.seed,
                &format!(
                    "sys_{}_{}_str_{}_gas_bdy{}_ring",
                    coord, system_index, star_id, planet_id
                ),
            ),
            star_mass as f64,
            *&parent_orbit.average_distance,
            planet_mass as f64,
            planet_density as f64,
            planet_radius as f64,
            ring_mass,
            if ring_composition == CelestialRingComposition::Ice {
                1.1
            } else if ring_composition == CelestialRingComposition::Rock {
                3.0
            } else if ring_composition == CelestialRingComposition::Metal {
                7.0
            } else {
                2.5
            },
            1.0 * 10.0f64.powf(-10.0),
            MoonDistance::Ring,
            0.0,
            0.0,
        );
        let ring_name = format!(
            "{}{}'s ring",
            star_name,
            StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
        );
        let rings: CelestialDisk = if moonlets < 4 {
            CelestialDisk::new(
                None, // TODO
                ring_id,
                ring_name.into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Unnoticeable,
                    ring_composition,
                )),
            )
        } else if moonlets < 6 {
            CelestialDisk::new(
                None,
                ring_id,
                ring_name.into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Noticeable,
                    ring_composition,
                )),
            )
        } else if moonlets < 10 {
            CelestialDisk::new(
                None,
                ring_id,
                ring_name.into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Visible,
                    ring_composition,
                )),
            )
        } else {
            CelestialDisk::new(
                None,
                ring_id,
                ring_name.into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Spectacular,
                    ring_composition,
                )),
            )
        };
        if ring_distance > 0.0 {
            moons.push(OrbitalPoint::new(
                ring_id,
                Some(Orbit {
                    primary_body_id: planet_id,
                    id: Some(ring_id),
                    average_distance: ring_distance,
                    average_distance_from_system_center: parent_orbit.average_distance_from_system_center,
                    zone: parent_orbit.zone,
                    ..Default::default()
                }),
                match ring_composition {
                    CelestialRingComposition::Ice => AstronomicalObject::IcyDisk(rings),
                    _ => AstronomicalObject::TelluricDisk(rings),
                },
                Vec::new(),
            ));
        }
    }
}

pub(crate) fn get_major_moons(
    moons: &Vec<OrbitalPoint>,
) -> Filter<Iter<OrbitalPoint>, fn(&&OrbitalPoint) -> bool> {
    moons.iter().filter(|moon_point| {
        if let AstronomicalObject::TelluricBody(moon) = moon_point.object.clone() {
            moon.size != CelestialBodySize::Puny
        } else {
            false
        }
    })
}
