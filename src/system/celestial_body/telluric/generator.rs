use crate::internal::*;
use crate::prelude::TelluricSpecialTrait::*;
use crate::prelude::*;
use crate::system::celestial_body::generator::*;
use crate::system::contents::generator::{
    generate_body_from_type, generate_inner_body_type, generate_outer_body_type,
};
use crate::system::contents::utils::{
    calculate_blackbody_temperature, calculate_hill_sphere_radius, calculate_surface_gravity,
};
use crate::system::orbital_point::generator::{
    calculate_planet_orbit_eccentricity, complete_orbit_with_period_and_eccentricity,
};

impl TelluricBodyDetails {
    /// Generates a barebone rocky body to use in system generation.
    pub(crate) fn generate_rocky_body_stub(orbital_point_id: u32) -> CelestialBody {
        CelestialBody {
            stub: true,
            name: "Temporary name".into(),
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            mass: 0.0,
            radius: 0.0,
            density: 0.0,
            gravity: 0.0,
            blackbody_temperature: 0,
            size: CelestialBodySize::Puny,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                0.0,
                TelluricBodyComposition::Rocky,
                CelestialBodyWorldType::Rock,
                Vec::new(),
                CelestialBodyCoreHeat::FrozenCore,
            )),
        }
    }

    /// Generates a fully fledged telluric body.
    pub fn generate_rocky_body(
        body_id: u32,
        coord: SpaceCoordinates,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_age: f32,
        star_mass: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_luminosity: f32,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f32,
        gas_giant_arrangement: GasGiantArrangement,
        next_id: &mut u32,
        populated_orbit_index: u32,
        own_orbit: Option<Orbit>,
        orbit_distance: f64,
        mut orbits: Vec<Orbit>,
        seed: Rc<str>,
        settings: GenerationSettings,
        size_modifier: i32,
        is_moon: bool,
        fixed_size: Option<CelestialBodySize>,
    ) -> (OrbitalPoint, Vec<OrbitalPoint>) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}",
                coord, system_index, star_id, body_id
            ),
        );
        let rolled_size = rng.roll(1, 400, size_modifier);

        let mut moons = Vec::new();
        let mut special_traits = Vec::new();
        let size_parameters = Self::determine_rocky_body_size(
            &star_name,
            populated_orbit_index,
            body_id,
            &own_orbit,
            &orbits,
            is_moon,
            rolled_size,
            &mut special_traits,
        );
        let mut to_return = size_parameters.0;
        let mut min_density = size_parameters.1;
        let mut max_density = size_parameters.2;
        let mut size = if let Some(s) = fixed_size {
            s
        } else {
            size_parameters.3
        };
        let mut mass = 0.0;
        let mut density = 0.0;
        let mut radius = 0.0;

        if discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void) {
            generate_peculiarities(
                &coord,
                &system_index,
                &star_id,
                &body_id,
                &settings,
                &mut min_density,
                &mut max_density,
                &mut size,
                &mut special_traits,
            );

            let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
            let (density, new_size, radius, mass) = generate_acceptable_telluric_parameters(
                size_modifier,
                &mut rng,
                min_density,
                max_density,
                size,
                blackbody_temp,
                "rocky".into(),
            );
            size = new_size;

            let body_type = CelestialBodyComposition::Rocky;
            let this_orbit = complete_orbit_with_period_and_eccentricity(
                coord,
                system_index,
                star_id,
                ConversionUtils::solar_mass_to_earth_mass(star_mass as f64),
                gas_giant_arrangement,
                body_id,
                &own_orbit,
                orbit_distance,
                body_type == CelestialBodyComposition::Gaseous,
                blackbody_temp,
                mass,
                false,
                &settings,
            );

            let surface_gravity = calculate_surface_gravity(density, radius);
            let world_type = get_world_type(
                size,
                CelestialBodyComposition::Rocky,
                blackbody_temp,
                primary_star_mass,
                &mut rng,
            );

            moons = MoonGenerator::generate_planets_moons(
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
                orbit_distance,
                coord,
                &seed.clone(),
                next_id,
                gas_giant_arrangement,
                populated_orbit_index,
                body_id,
                size,
                mass,
                density,
                radius,
                blackbody_temp,
                settings,
                is_moon,
            );

            to_return = Self::bundle_world_first_pass(
                star_name,
                populated_orbit_index,
                body_id,
                this_orbit,
                orbits,
                size,
                blackbody_temp,
                density,
                radius,
                mass,
                surface_gravity,
                TelluricBodyComposition::Rocky,
                world_type,
                special_traits,
                &moons,
                is_moon,
            );
        }

        (to_return, moons)
    }

    fn determine_rocky_body_size(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
        is_moon: bool,
        rolled_size: i64,
        special_traits: &mut Vec<TelluricSpecialTrait>,
    ) -> (OrbitalPoint, f64, f64, CelestialBodySize) {
        let mut to_return = OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::Void,
            orbits.clone(),
        );
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut size = CelestialBodySize::Puny;

        if is_moon {
            min_density = 3.0;
            max_density = 5.5;
        } else if rolled_size <= 21 {
            // Debris disk
            to_return = Self::make_debris_disk(
                &star_name,
                populated_orbit_index,
                orbital_point_id,
                &own_orbit,
                &orbits,
            );
        } else if rolled_size <= 86 {
            // Asteroid belt
            to_return = Self::make_asteroid_belt(
                &star_name,
                populated_orbit_index,
                orbital_point_id,
                &own_orbit,
                &orbits,
            );
        } else if rolled_size <= 96 {
            // Ash belt
            to_return = Self::make_ash_belt(
                &star_name,
                populated_orbit_index,
                orbital_point_id,
                &own_orbit,
                &orbits,
            );
        } else if rolled_size <= 161 {
            // Rock dwarf
            min_density = 3.3;
            max_density = 5.5;
            size = CelestialBodySize::Tiny;
        } else if rolled_size <= 163 {
            // Coreless rock planet
            min_density = 3.0;
            max_density = 4.5;
            size = CelestialBodySize::Tiny;
            special_traits.push(UnusualCore(TelluricCoreDifference::Coreless));
        } else if rolled_size <= 235 {
            // Rock dwarf
            min_density = 3.3;
            max_density = 5.5;
            size = CelestialBodySize::Small;
        } else if rolled_size <= 237 {
            // Coreless rock planet
            min_density = 3.0;
            max_density = 4.5;
            size = CelestialBodySize::Small;
            special_traits.push(UnusualCore(TelluricCoreDifference::Coreless));
        } else if rolled_size <= 240 {
            // Coreless rock planet
            min_density = 3.0;
            max_density = 4.5;
            size = CelestialBodySize::Standard;
            special_traits.push(UnusualCore(TelluricCoreDifference::Coreless));
        } else if rolled_size <= 318 {
            // Rock planet
            min_density = 4.4;
            max_density = 6.2;
            size = CelestialBodySize::Standard;
        } else {
            // Rock giant
            min_density = 4.9;
            max_density = 7.0;
            size = CelestialBodySize::Large;
        }

        (to_return, min_density, max_density, size)
    }

    /// Generates a barebone metallic body to use in system generation.
    pub(crate) fn generate_metallic_body_stub(orbital_point_id: u32) -> CelestialBody {
        CelestialBody {
            stub: true,
            name: "Temporary name".into(),
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            mass: 0.0,
            radius: 0.0,
            density: 0.0,
            gravity: 0.0,
            blackbody_temperature: 0,
            size: CelestialBodySize::Puny,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                0.0,
                TelluricBodyComposition::Metallic,
                CelestialBodyWorldType::Rock,
                Vec::new(),
                CelestialBodyCoreHeat::FrozenCore,
            )),
        }
    }

    /// Generates a fully fledged metallic body.
    pub fn generate_metallic_body(
        body_id: u32,
        coord: SpaceCoordinates,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_age: f32,
        star_mass: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_luminosity: f32,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f32,
        gas_giant_arrangement: GasGiantArrangement,
        next_id: &mut u32,
        populated_orbit_index: u32,
        own_orbit: Option<Orbit>,
        orbit_distance: f64,
        mut orbits: Vec<Orbit>,
        seed: Rc<str>,
        settings: GenerationSettings,
        size_modifier: i32,
        is_moon: bool,
        fixed_size: Option<CelestialBodySize>,
    ) -> (OrbitalPoint, Vec<OrbitalPoint>) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}",
                coord, system_index, star_id, body_id
            ),
        );
        let rolled_size = rng.roll(1, 400, size_modifier);
        let mut moons = Vec::new();
        let mut special_traits = Vec::new();
        let size_parameters = Self::determine_metallic_body_size(
            &star_name,
            populated_orbit_index,
            body_id,
            &own_orbit,
            &orbits,
            is_moon,
            rolled_size,
        );
        let mut to_return = size_parameters.0;
        let mut min_density = size_parameters.1;
        let mut max_density = size_parameters.2;
        let mut size = if let Some(s) = fixed_size {
            s
        } else {
            size_parameters.3
        };
        let mut mass = 0.0;
        let mut density = 0.0;
        let mut radius = 0.0;

        if discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void) {
            generate_peculiarities(
                &coord,
                &system_index,
                &star_id,
                &body_id,
                &settings,
                &mut min_density,
                &mut max_density,
                &mut size,
                &mut special_traits,
            );

            let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
            let (density, new_size, radius, mass) = generate_acceptable_telluric_parameters(
                size_modifier,
                &mut rng,
                min_density,
                max_density,
                size,
                blackbody_temp,
                "metallic".into(),
            );
            size = new_size;

            let body_type = CelestialBodyComposition::Metallic;
            let this_orbit = if is_moon {
                own_orbit.clone().unwrap_or_default()
            } else {
                complete_orbit_with_period_and_eccentricity(
                    coord,
                    system_index,
                    star_id,
                    ConversionUtils::solar_mass_to_earth_mass(star_mass as f64),
                    gas_giant_arrangement,
                    body_id,
                    &own_orbit,
                    orbit_distance,
                    body_type == CelestialBodyComposition::Gaseous,
                    blackbody_temp,
                    mass,
                    false,
                    &settings,
                )
            };

            let surface_gravity = calculate_surface_gravity(density, radius);
            let world_type = get_world_type(
                size,
                CelestialBodyComposition::Metallic,
                blackbody_temp,
                primary_star_mass,
                &mut rng,
            );

            moons = MoonGenerator::generate_planets_moons(
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
                orbit_distance,
                coord,
                &seed.clone(),
                next_id,
                gas_giant_arrangement,
                populated_orbit_index,
                body_id,
                size,
                mass,
                density,
                radius,
                blackbody_temp,
                settings,
                is_moon,
            );

            to_return = Self::bundle_world_first_pass(
                star_name,
                populated_orbit_index,
                body_id,
                this_orbit,
                orbits,
                size,
                blackbody_temp,
                density,
                radius,
                mass,
                surface_gravity,
                TelluricBodyComposition::Metallic,
                world_type,
                special_traits,
                &moons,
                is_moon,
            );
        }

        (to_return, moons)
    }

    fn determine_metallic_body_size(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
        is_moon: bool,
        rolled_size: i64,
    ) -> (OrbitalPoint, f64, f64, CelestialBodySize) {
        let mut to_return = OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::Void,
            orbits.clone(),
        );
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut size = CelestialBodySize::Puny;

        if is_moon {
            min_density = 5.0;
            max_density = 9.0;
        } else if rolled_size <= 61 {
            // Dust belt
            to_return = Self::make_dust_belt(
                &star_name,
                populated_orbit_index,
                orbital_point_id,
                &own_orbit,
                &orbits,
            );
        } else if rolled_size <= 131 {
            // Meteoroid belt
            to_return = Self::make_meteoroid_belt(
                &star_name,
                populated_orbit_index,
                orbital_point_id,
                &own_orbit,
                &orbits,
            );
        } else if rolled_size <= 141 {
            // Ore belt
            to_return = Self::make_ore_belt(
                &star_name,
                populated_orbit_index,
                orbital_point_id,
                &own_orbit,
                &orbits,
            );
        } else if rolled_size <= 221 {
            // Metal tiny
            min_density = 5.0;
            max_density = 7.0;
            size = CelestialBodySize::Tiny;
        } else if rolled_size <= 301 {
            // Solid metal dwarf
            min_density = 7.0;
            max_density = 15.0;
            size = CelestialBodySize::Tiny;
        } else if rolled_size <= 311 {
            // Metal small
            min_density = 6.0;
            max_density = 8.0;
            size = CelestialBodySize::Small;
        } else if rolled_size <= 321 {
            // Solid metal dwarf
            min_density = 7.0;
            max_density = 15.0;
            size = CelestialBodySize::Small;
        } else if rolled_size <= 391 {
            // Metal planet
            min_density = 6.0;
            max_density = 8.0;
            size = CelestialBodySize::Standard;
        } else if rolled_size <= 393 {
            // Solid metal planet
            min_density = 7.0;
            max_density = 15.0;
            size = CelestialBodySize::Standard;
        } else {
            // Metal giant
            min_density = 6.0;
            max_density = 9.0;
            size = CelestialBodySize::Large;
        }

        (to_return, min_density, max_density, size)
    }

    pub(crate) fn bundle_world_first_pass(
        star_name: Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: Orbit,
        orbits: Vec<Orbit>,
        mut size: CelestialBodySize,
        blackbody_temperature: u32,
        density: f32,
        radius: f32,
        mass: f32,
        gravity: f32,
        body_type: TelluricBodyComposition,
        world_type: CelestialBodyWorldType,
        special_traits: Vec<TelluricSpecialTrait>,
        moons: &Vec<OrbitalPoint>,
        is_moon: bool,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            Some(own_orbit.clone()),
            AstronomicalObject::TelluricBody(CelestialBody {
                stub: true,
                name: format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                orbit: None,
                orbital_point_id,
                mass,
                radius,
                density,
                gravity,
                blackbody_temperature,
                size,
                details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                    0.0,
                    body_type,
                    world_type,
                    special_traits,
                    CelestialBodyCoreHeat::ActiveCore,
                )),
            }),
            moons
                .clone()
                .iter()
                .filter(|o| o.own_orbit.is_some())
                .map(|o| o.own_orbit.clone().unwrap_or_default())
                .collect::<Vec<Orbit>>(),
        )
    }

    pub(crate) fn generate_world(
        coord: SpaceCoordinates,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_age: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_traits: &Vec<StarPeculiarity>,
        distance_from_star: f64,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: Orbit,
        orbits: Vec<Orbit>,
        world: CelestialBody,
        is_moon: bool,
        moons: &Vec<OrbitalPoint>,
        tidal_heating: u32,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> OrbitalPoint {
        let CelestialBody {
            name,
            orbit,
            mass,
            radius,
            density,
            gravity,
            blackbody_temperature,
            size,
            details,
            ..
        } = world;
        let CelestialBodyDetails::Telluric(TelluricBodyDetails {
            body_type,
            world_type,
            special_traits,
            ..
        }) = details
        else {
            panic!("At this point, details should be telluric.")
        };

        // Core heat
        let core_heat: CelestialBodyCoreHeat = Self::generate_core_heat(
            &coord,
            &system_index,
            &star_id,
            star_age,
            &orbital_point_id,
            &own_orbit,
            size,
            density,
            body_type,
            &special_traits,
            tidal_heating,
            &settings,
            distance_from_star,
        );

        // TODO: Magnetic field
        // TODO: Volcanism
        // TODO: Tectonics

        let atmospheric_pressure = generate_atmosphere(
            &coord,
            &system_index,
            &star_id,
            star_age,
            star_type,
            star_class,
            star_traits,
            &orbital_point_id,
            &own_orbit,
            size,
            mass,
            body_type,
            world_type,
            is_moon,
            &settings,
        );
        // TODO: Atmospheric composition

        // TODO: Blackbody correction and definitive blackbody temperature

        // TODO: Hydrographics
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_hydr",
                coord, system_index, star_id, orbital_point_id
            ),
        );
        let hydrosphere = match world_type {
            CelestialBodyWorldType::Rock
            | CelestialBodyWorldType::Chthonian
            | CelestialBodyWorldType::Sulfur
            | CelestialBodyWorldType::Hadean => 0.0,
            CelestialBodyWorldType::Ice => {
                if size == CelestialBodySize::Small {
                    rng.roll(1, 6000, 2499) as f32 / 100.0
                } else if size == CelestialBodySize::Standard || size == CelestialBodySize::Large {
                    rng.roll(2, 6000, -10000).max(0) as f32 / 100.0
                } else {
                    0.0
                }
            }
            CelestialBodyWorldType::Hadean => {
                if size == CelestialBodySize::Standard || size == CelestialBodySize::Small {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Chthonian => 0.0,
            CelestialBodyWorldType::Greenhouse => rng.gen_range(1.5..300.0),
            _ => 0.0,
        };

        // TODO: Climate

        // TODO: Dynamic parameters
        // TODO: - Tidal Braking
        // TODO: - Rotation Period
        // TODO: - Local Calendar
        // TODO: - Axial Tilt

        OrbitalPoint::new(
            orbital_point_id,
            Some(own_orbit.clone()),
            AstronomicalObject::TelluricBody(CelestialBody::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                name,
                mass,
                radius,
                density,
                gravity,
                blackbody_temperature,
                size,
                CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                    atmospheric_pressure,
                    if body_type == TelluricBodyComposition::Icy && blackbody_temperature >= 170 {
                        TelluricBodyComposition::Rocky
                    } else {
                        body_type
                    },
                    world_type,
                    special_traits,
                    core_heat,
                )),
            )),
            orbits.clone(),
        )
    }

    fn generate_core_heat(
        coord: &SpaceCoordinates,
        system_index: &u16,
        star_id: &u32,
        star_age: f32,
        orbital_point_id: &u32,
        own_orbit: &Orbit,
        size: CelestialBodySize,
        density: f32,
        body_type: TelluricBodyComposition,
        special_traits: &Vec<TelluricSpecialTrait>,
        tidal_heating: u32,
        settings: &GenerationSettings,
        distance_from_star: f64,
    ) -> CelestialBodyCoreHeat {
        if size == CelestialBodySize::Tiny {
            CelestialBodyCoreHeat::FrozenCore
        } else {
            let mut rng = SeededDiceRoller::new(
                &settings.seed,
                &format!(
                    "sys_{}_{}_str_{}_bdy{}_core",
                    coord, system_index, star_id, orbital_point_id
                ),
            );
            let mut core_heat_modifier = 0;
            core_heat_modifier += if size == CelestialBodySize::Puny {
                -100
            } else if size == CelestialBodySize::Tiny {
                -5
            } else if size == CelestialBodySize::Small {
                -2
            } else if size == CelestialBodySize::Standard {
                2
            } else if size == CelestialBodySize::Large {
                3
            } else {
                5
            };
            core_heat_modifier += if special_traits.iter().any(|x| {
                matches!(
                    x,
                    TelluricSpecialTrait::UnusualCore(TelluricCoreDifference::Coreless)
                )
            }) {
                -100
            } else if special_traits.iter().any(|x| {
                matches!(
                    x,
                    TelluricSpecialTrait::UnusualCore(TelluricCoreDifference::Smaller)
                )
            }) {
                -2
            } else if special_traits.iter().any(|x| {
                matches!(
                    x,
                    TelluricSpecialTrait::UnusualCore(TelluricCoreDifference::Smaller)
                )
            }) {
                2
            } else {
                0
            };
            core_heat_modifier += if special_traits.iter().any(|x| {
                matches!(
                    x,
                    TelluricSpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct
                    ) | TelluricSpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead
                    )
                )
            }) {
                -100
            } else if special_traits.iter().any(|x| {
                matches!(
                    x,
                    TelluricSpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive
                    )
                )
            }) {
                5
            } else {
                0
            };
            core_heat_modifier += if star_age < 0.703 {
                5
            } else if star_age < 1.251 {
                3
            } else if star_age < 1.6 {
                1
            } else if star_age < 2.0 {
                0
            } else if star_age < 5.730 {
                -1
            } else if star_age < 7.0 {
                -2
            } else if star_age < 10.0 {
                -3
            } else if star_age < 14.05 {
                -4
            } else if star_age < 20.0 {
                -5
            } else if star_age < 25.0 {
                -6
            } else if star_age < 30.0 {
                -7
            } else if star_age < 35.0 {
                -8
            } else {
                -9
            };
            core_heat_modifier += if body_type == TelluricBodyComposition::Metallic {
                1
            } else if body_type == TelluricBodyComposition::Rocky {
                0
            } else {
                -1
            };
            core_heat_modifier += if density < 3.0 {
                -1
            } else if density > 5.0 {
                1
            } else {
                0
            };
            core_heat_modifier += if distance_from_star <= 0.1 {
                2
            } else if distance_from_star <= 0.5 {
                1
            } else if distance_from_star <= 1.5 {
                0
            } else if distance_from_star <= 5.0 {
                -1
            } else {
                -2
            };
            core_heat_modifier += if own_orbit.eccentricity > 0.3 {
                2
            } else if own_orbit.eccentricity >= 0.1 {
                1
            } else {
                0
            };
            core_heat_modifier += tidal_heating as i32;
            rng.get_result(&CopyableRollToProcess::new(
                vec![
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::FrozenCore, 1),
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::WarmCore, 4),
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::ActiveCore, 4),
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::IntenseCore, 1),
                ],
                RollMethod::PreparedRoll(PreparedRoll::new(2, 6, core_heat_modifier)),
            ))
            .expect("Should have generated a core heat value.")
        }
    }

    pub(crate) fn make_ash_belt(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Ash)),
            )),
            orbits.clone(),
        )
    }

    pub(crate) fn make_asteroid_belt(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Asteroid)),
            )),
            orbits.clone(),
        )
    }

    pub(crate) fn make_debris_disk(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Debris)),
            )),
            orbits.clone(),
        )
    }

    pub(crate) fn make_ore_belt(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Ore)),
            )),
            orbits.clone(),
        )
    }

    pub(crate) fn make_meteoroid_belt(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Meteoroid)),
            )),
            orbits.clone(),
        )
    }

    pub(crate) fn make_dust_belt(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Dust)),
            )),
            orbits.clone(),
        )
    }
}

pub(crate) fn generate_peculiarities(
    coord: &SpaceCoordinates,
    system_index: &u16,
    star_id: &u32,
    orbital_point_id: &u32,
    settings: &GenerationSettings,
    min_density: &mut f64,
    max_density: &mut f64,
    size: &mut CelestialBodySize,
    special_traits: &mut Vec<TelluricSpecialTrait>,
) -> SeededDiceRoller {
    let mut rng = SeededDiceRoller::new(
        &settings.seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_spec",
            coord, system_index, star_id, orbital_point_id
        ),
    );
    let mut current_roll;
    let mut reroll = true;
    while reroll {
        current_roll = rng.roll(1, 121, 0);
        reroll = if current_roll > 120 {
            current_roll = rng.roll(1, 120, 0);
            true
        } else {
            false
        };
        match size {
            CelestialBodySize::Hypergiant
            | CelestialBodySize::Supergiant
            | CelestialBodySize::Giant
            | CelestialBodySize::Large => {
                if current_roll <= 2
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Poor,
                    ));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Rich,
                    ));
                } else if current_roll <= 32
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Slow));
                } else if current_roll <= 37
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Fast));
                } else if current_roll <= 40
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Minimal));
                } else if current_roll <= 49
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Extreme));
                } else if current_roll <= 53
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(UnusualCore(TelluricCoreDifference::Smaller));
                } else if current_roll <= 55
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(UnusualCore(TelluricCoreDifference::Larger));
                } else if current_roll <= 59
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead,
                    ));
                } else if current_roll <= 69
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct,
                    ));
                } else if current_roll <= 74
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive,
                    ));
                } else if current_roll <= 79
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(UnusualMagneticField(magnetic_field));
                } else if current_roll <= 84 {
                    // Less dense
                    *min_density += -1.0;
                    *max_density += -1.0;
                } else if current_roll <= 87 {
                    // Denser
                    *min_density += 1.0;
                    *max_density += 1.0;
                } else if current_roll <= 96
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 98
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
            CelestialBodySize::Standard | CelestialBodySize::Small => {
                if current_roll <= 4
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Poor,
                    ));
                } else if current_roll <= 14
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Rich,
                    ));
                } else if current_roll <= 19
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Slow));
                } else if current_roll <= 24
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Fast));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Minimal));
                } else if current_roll <= 34
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Extreme));
                } else if current_roll <= 39
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(UnusualCore(TelluricCoreDifference::Smaller));
                } else if current_roll <= 42
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(UnusualCore(TelluricCoreDifference::Larger));
                } else if current_roll <= 44
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead,
                    ));
                } else if current_roll <= 49
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct,
                    ));
                } else if current_roll <= 64
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive,
                    ));
                } else if current_roll <= 76
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(UnusualMagneticField(magnetic_field));
                } else if current_roll <= 81 {
                    // Less dense
                    *min_density += -1.0;
                    *max_density += -1.0;
                } else if current_roll <= 86 {
                    // Denser
                    *min_density += 1.0;
                    *max_density += 1.0;
                } else if current_roll <= 86
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 98
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
            CelestialBodySize::Tiny => {
                if current_roll <= 9
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Poor,
                    ));
                } else if current_roll <= 14
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Rich,
                    ));
                } else if current_roll <= 24
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Slow));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Fast));
                } else if current_roll <= 39
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Minimal));
                } else if current_roll <= 44
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Extreme));
                } else if current_roll <= 49
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(UnusualCore(TelluricCoreDifference::Smaller));
                } else if current_roll <= 54
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(UnusualCore(TelluricCoreDifference::Larger));
                } else if current_roll <= 59
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead,
                    ));
                } else if current_roll <= 67
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct,
                    ));
                } else if current_roll <= 69
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificGeologicActivity(_)))
                {
                    special_traits.push(SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive,
                    ));
                } else if current_roll <= 74
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(UnusualMagneticField(magnetic_field));
                } else if current_roll <= 79 {
                    // Less dense
                    *min_density += -1.0;
                    *max_density += -1.0;
                } else if current_roll <= 84 {
                    // Denser
                    *min_density += 1.0;
                    *max_density += 1.0;
                } else if current_roll <= 89
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 98
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
            CelestialBodySize::Puny => {
                if current_roll <= 10
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Slow));
                } else if current_roll <= 19
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(UnusualRotation(TelluricRotationDifference::Fast));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Minimal));
                } else if current_roll <= 34
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(UnusualAxialTilt(TelluricAxialTiltDifference::Extreme));
                } else if current_roll <= 44
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(UnusualMagneticField(magnetic_field));
                } else if current_roll <= 59 {
                    // Less dense
                    *min_density += -1.0;
                    *max_density += -1.0;
                } else if current_roll <= 64 {
                    // Denser
                    *min_density += 1.0;
                    *max_density += 1.0;
                } else if current_roll <= 69
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 68
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, TelluricSpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(TelluricSpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
        }
    }
    rng
}

fn generate_magnetic_field_difference(
    rng: &mut SeededDiceRoller,
) -> TelluricMagneticFieldDifference {
    let magnetic_field_roll = rng.roll(1, 4, 0);
    let magnetic_field = if magnetic_field_roll <= 1 {
        TelluricMagneticFieldDifference::MuchWeaker
    } else if magnetic_field_roll <= 2 {
        TelluricMagneticFieldDifference::Weaker
    } else if magnetic_field_roll <= 3 {
        TelluricMagneticFieldDifference::Stronger
    } else {
        TelluricMagneticFieldDifference::MuchStronger
    };
    magnetic_field
}

fn generate_atmosphere(
    coord: &SpaceCoordinates,
    system_index: &u16,
    star_id: &u32,
    star_age: f32,
    star_type: &StarSpectralType,
    star_class: &StarLuminosityClass,
    star_traits: &Vec<StarPeculiarity>,
    orbital_point_id: &u32,
    own_orbit: &Orbit,
    size: CelestialBodySize,
    mass: f32,
    body_type: TelluricBodyComposition,
    world_type: CelestialBodyWorldType,
    is_moon: bool,
    settings: &GenerationSettings,
) -> f32 {
    let mut atmospheric_mass_modifier = 0;
    atmospheric_mass_modifier += if own_orbit.average_distance <= 0.2 {
        -2
    } else if own_orbit.average_distance <= 2.0 {
        -1
    } else {
        0
    };
    atmospheric_mass_modifier += if mass >= 20.0 {
        5
    } else if mass >= 10.0 {
        4
    } else if mass >= 6.0 {
        3
    } else if mass >= 3.0 {
        2
    } else if mass >= 1.0 {
        1
    } else if mass < 0.1 {
        -1
    } else {
        0
    };
    atmospheric_mass_modifier += if star_age <= 0.1 {
        1
    } else if star_age >= 10.0 {
        -2
    } else if star_age >= 5.0 {
        -1
    } else {
        0
    };
    atmospheric_mass_modifier += if body_type == TelluricBodyComposition::Icy {
        2
    } else {
        0
    };
    atmospheric_mass_modifier += match star_type {
        StarSpectralType::WR(_) => -5,
        StarSpectralType::O(_) => -4,
        StarSpectralType::B(_) => -3,
        _ => 0,
    };
    atmospheric_mass_modifier += match star_class {
        StarLuminosityClass::O => -5,
        StarLuminosityClass::Ia => -5,
        StarLuminosityClass::Ib => -5,
        StarLuminosityClass::II => -4,
        StarLuminosityClass::III => -3,
        StarLuminosityClass::XNS => -5,
        _ => 0,
    };
    atmospheric_mass_modifier += if is_moon { -2 } else { 0 };
    for peculiarity in star_traits {
        if let StarPeculiarity::UnusualMetallicity(metallicity_difference) = peculiarity {
            if discriminant(metallicity_difference)
                == discriminant(&StarMetallicityDifference::MuchHigher)
            {
                2
            } else if discriminant(metallicity_difference)
                == discriminant(&StarMetallicityDifference::Higher)
            {
                1
            } else if discriminant(metallicity_difference)
                == discriminant(&StarMetallicityDifference::Lower)
            {
                -1
            } else {
                -2
            }
        } else if discriminant(peculiarity) == discriminant(&StarPeculiarity::ExcessiveRadiation)
            || discriminant(peculiarity) == discriminant(&StarPeculiarity::PowerfulStellarWinds)
            || discriminant(peculiarity) == discriminant(&StarPeculiarity::StrongMagneticField)
        {
            -2
        } else {
            0
        };
    }
    let mut rng = SeededDiceRoller::new(
        &settings.seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_atmo",
            coord, system_index, star_id, orbital_point_id
        ),
    );
    let mut atmospheric_pressure = if size == CelestialBodySize::Puny {
        0.0
    } else {
        match world_type {
            CelestialBodyWorldType::Ice
            | CelestialBodyWorldType::DirtySnowball
            | CelestialBodyWorldType::Sulfur => {
                if size == CelestialBodySize::Tiny {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Rock => {
                if size == CelestialBodySize::Tiny || size == CelestialBodySize::Small {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Hadean => {
                if size == CelestialBodySize::Tiny
                    || size == CelestialBodySize::Small
                    || size == CelestialBodySize::Standard
                {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Chthonian => 0.0,
            CelestialBodyWorldType::Greenhouse => rng.gen_range(1.5..300.0),
            _ => -1.0,
        }
    };
    if atmospheric_pressure < 0.0 {
        let random_pressure_table = [
            (0.0, 0.01),
            (0.0, 0.01),
            (0.01, 0.5),
            (0.01, 0.5),
            (0.01, 0.5),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.8, 1.2),
            (0.8, 1.2),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.5, 10.0),
            (1.5, 10.0),
            (10.0, 300.0),
        ];
        let generic_pressure_table = [
            (0.01, 0.5),
            (0.01, 0.5),
            (0.01, 0.5),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.5, 10.0),
            (1.5, 10.0),
        ];
        let terrestrial_pressure_table = [
            (0.5, 0.8),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
        ];
        let atmospheric_pressure_bracket: (f32, f32) = (match world_type {
            CelestialBodyWorldType::Sulfur | CelestialBodyWorldType::Ammonia => {
                generic_pressure_table
            }
            CelestialBodyWorldType::Ocean | CelestialBodyWorldType::Terrestrial => {
                terrestrial_pressure_table
            }
            _ => random_pressure_table,
        })[(rng.roll(1, 10, atmospheric_mass_modifier).min(14).max(0) as usize)];
        atmospheric_pressure =
            rng.gen_range(atmospheric_pressure_bracket.0..atmospheric_pressure_bracket.1);
    }
    atmospheric_pressure
}
