use crate::internal::types::MoonDistance;
use crate::internal::*;
use crate::prelude::*;
use crate::system::celestial_body::generator::{
    generate_acceptable_telluric_parameters, get_world_type,
};
use crate::system::contents::utils::{calculate_blackbody_temperature, calculate_surface_gravity};
use crate::system::orbital_point::generator::{
    complete_orbit_with_orbital_period, complete_orbit_with_rotation_and_axis,
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
            tidal_heating: 0,
            size: CelestialBodySize::Puny,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                TelluricBodyComposition::Rocky,
                CelestialBodyWorldType::Rock,
                Vec::new(),
                CelestialBodyCoreHeat::FrozenCore,
                MagneticFieldStrength::None,
                0.0,
                0.0,
                0.0,
                0.0,
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
        star_mass: f64,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_luminosity: f32,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f64,
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
            let surface_gravity = calculate_surface_gravity(density, radius);
            let world_type = get_world_type(
                size,
                CelestialBodyComposition::Rocky,
                blackbody_temp,
                primary_star_mass,
                &mut rng,
            );

            let this_orbit = if !is_moon {
                complete_orbit_with_orbital_period(
                    coord,
                    system_index,
                    star_id,
                    ConversionUtils::solar_mass_to_earth_mass(star_mass),
                    gas_giant_arrangement,
                    body_id,
                    &own_orbit,
                    orbit_distance,
                    own_orbit
                        .clone()
                        .unwrap_or_default()
                        .average_distance_from_system_center,
                    body_type == CelestialBodyComposition::Gaseous,
                    blackbody_temp,
                    mass,
                    size,
                    is_moon,
                    &settings,
                )
            } else {
                own_orbit.clone().unwrap_or_default()
            };

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
                own_orbit.clone().unwrap(),
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
                this_orbit.orbital_period,
                blackbody_temp,
                &settings,
                is_moon,
            );

            let this_orbit = if !is_moon {
                complete_orbit_with_rotation_and_axis(
                    coord,
                    system_index,
                    star_id,
                    star_age,
                    ConversionUtils::solar_mass_to_earth_mass(star_mass),
                    None,
                    gas_giant_arrangement,
                    system_traits,
                    body_id,
                    &Some(this_orbit),
                    orbit_distance,
                    body_type == CelestialBodyComposition::Gaseous,
                    blackbody_temp,
                    mass,
                    radius,
                    size,
                    &mut special_traits,
                    &moons,
                    is_moon,
                    MoonDistance::Any,
                    &settings,
                )
            } else {
                this_orbit
            };

            to_return = WorldGenerator::bundle_world_first_pass(
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
        special_traits: &mut Vec<CelestialBodySpecialTrait>,
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
            special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                TelluricCoreDifference::Coreless,
            ));
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
            special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                TelluricCoreDifference::Coreless,
            ));
        } else if rolled_size <= 240 {
            // Coreless rock planet
            min_density = 3.0;
            max_density = 4.5;
            size = CelestialBodySize::Standard;
            special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                TelluricCoreDifference::Coreless,
            ));
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
            tidal_heating: 0,
            size: CelestialBodySize::Puny,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                TelluricBodyComposition::Metallic,
                CelestialBodyWorldType::Rock,
                Vec::new(),
                CelestialBodyCoreHeat::FrozenCore,
                MagneticFieldStrength::None,
                0.0,
                0.0,
                0.0,
                0.0,
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
        star_mass: f64,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_luminosity: f32,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f64,
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
            let surface_gravity = calculate_surface_gravity(density, radius);
            let world_type = get_world_type(
                size,
                CelestialBodyComposition::Metallic,
                blackbody_temp,
                primary_star_mass,
                &mut rng,
            );

            let this_orbit = if !is_moon {
                complete_orbit_with_orbital_period(
                    coord,
                    system_index,
                    star_id,
                    ConversionUtils::solar_mass_to_earth_mass(star_mass),
                    gas_giant_arrangement,
                    body_id,
                    &own_orbit,
                    orbit_distance,
                    own_orbit
                        .clone()
                        .unwrap_or_default()
                        .average_distance_from_system_center,
                    body_type == CelestialBodyComposition::Gaseous,
                    blackbody_temp,
                    mass,
                    size,
                    is_moon,
                    &settings,
                )
            } else {
                own_orbit.clone().unwrap_or_default()
            };

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
                own_orbit.clone().unwrap(),
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
                this_orbit.orbital_period,
                blackbody_temp,
                &settings,
                is_moon,
            );

            let this_orbit = if !is_moon {
                complete_orbit_with_rotation_and_axis(
                    coord,
                    system_index,
                    star_id,
                    star_age,
                    ConversionUtils::solar_mass_to_earth_mass(star_mass),
                    None,
                    gas_giant_arrangement,
                    system_traits,
                    body_id,
                    &Some(this_orbit),
                    orbit_distance,
                    body_type == CelestialBodyComposition::Gaseous,
                    blackbody_temp,
                    mass,
                    radius,
                    size,
                    &mut special_traits,
                    &moons,
                    is_moon,
                    MoonDistance::Any,
                    &settings,
                )
            } else {
                this_orbit
            };

            to_return = WorldGenerator::bundle_world_first_pass(
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
    special_traits: &mut Vec<CelestialBodySpecialTrait>,
) {
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
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Poor,
                    ));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Rich,
                    ));
                } else if current_roll <= 32
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Slow,
                    ));
                } else if current_roll <= 37
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Fast,
                    ));
                } else if current_roll <= 40
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Minimal,
                    ));
                } else if current_roll <= 49
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Extreme,
                    ));
                } else if current_roll <= 53
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                        TelluricCoreDifference::Smaller,
                    ));
                } else if current_roll <= 55
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                        TelluricCoreDifference::Larger,
                    ));
                } else if current_roll <= 59
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead,
                    ));
                } else if current_roll <= 69
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct,
                    ));
                } else if current_roll <= 74
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive,
                    ));
                } else if current_roll <= 79
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(CelestialBodySpecialTrait::UnusualMagneticField(
                        magnetic_field,
                    ));
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
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 98
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::EquatorialRidge,
                    ));
                } else if current_roll <= 99
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
            CelestialBodySize::Standard | CelestialBodySize::Small => {
                if current_roll <= 4
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Poor,
                    ));
                } else if current_roll <= 14
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Rich,
                    ));
                } else if current_roll <= 19
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Slow,
                    ));
                } else if current_roll <= 24
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Fast,
                    ));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Minimal,
                    ));
                } else if current_roll <= 34
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Extreme,
                    ));
                } else if current_roll <= 39
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                        TelluricCoreDifference::Smaller,
                    ));
                } else if current_roll <= 42
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                        TelluricCoreDifference::Larger,
                    ));
                } else if current_roll <= 44
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead,
                    ));
                } else if current_roll <= 49
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct,
                    ));
                } else if current_roll <= 64
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive,
                    ));
                } else if current_roll <= 76
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(CelestialBodySpecialTrait::UnusualMagneticField(
                        magnetic_field,
                    ));
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
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 98
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::EquatorialRidge,
                    ));
                } else if current_roll <= 99
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
            CelestialBodySize::Tiny => {
                if current_roll <= 9
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Poor,
                    ));
                } else if current_roll <= 14
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualVolatileDensity(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualVolatileDensity(
                        TelluricVolatileDensityDifference::Rich,
                    ));
                } else if current_roll <= 24
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Slow,
                    ));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Fast,
                    ));
                } else if current_roll <= 39
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Minimal,
                    ));
                } else if current_roll <= 44
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Extreme,
                    ));
                } else if current_roll <= 49
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                        TelluricCoreDifference::Smaller,
                    ));
                } else if current_roll <= 54
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualCore(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                        TelluricCoreDifference::Larger,
                    ));
                } else if current_roll <= 59
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead,
                    ));
                } else if current_roll <= 67
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct,
                    ));
                } else if current_roll <= 69
                    && !special_traits.iter().any(|x| {
                        matches!(x, CelestialBodySpecialTrait::SpecificGeologicActivity(_))
                    })
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive,
                    ));
                } else if current_roll <= 74
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(CelestialBodySpecialTrait::UnusualMagneticField(
                        magnetic_field,
                    ));
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
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 98
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::EquatorialRidge,
                    ));
                } else if current_roll <= 99
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
            CelestialBodySize::Puny => {
                if current_roll <= 10
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Slow,
                    ));
                } else if current_roll <= 19
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualRotation(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                        TelluricRotationDifference::Fast,
                    ));
                } else if current_roll <= 29
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Minimal,
                    ));
                } else if current_roll <= 34
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualAxialTilt(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::UnusualAxialTilt(
                        TelluricAxialTiltDifference::Extreme,
                    ));
                } else if current_roll <= 44
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::UnusualMagneticField(_)))
                {
                    let magnetic_field = generate_magnetic_field_difference(&mut rng);
                    special_traits.push(CelestialBodySpecialTrait::UnusualMagneticField(
                        magnetic_field,
                    ));
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
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::FlatTerrain,
                    ));
                } else if current_roll <= 68
                    && !special_traits
                        .iter()
                        .any(|x| matches!(x, CelestialBodySpecialTrait::SpecificTerrainRelief(_)))
                {
                    special_traits.push(CelestialBodySpecialTrait::SpecificTerrainRelief(
                        TelluricTerrainRelief::VariedTerrain,
                    ));
                }
            }
        }
    }
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
