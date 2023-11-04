use crate::internal::*;
use crate::prelude::*;
use crate::system::celestial_body::generator::get_size_constraint;
use crate::system::contents::utils::calculate_blackbody_temperature;

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
            size: CelestialBodySize::Moonlet,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                CelestialBodySubType::Rocky,
            )),
        }
    }

    /// Generates a fully fledged telluric body.
    pub fn generate_rocky_body(
        coord: SpaceCoordinates,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_luminosity: f32,
        star_traits: &Vec<StarPeculiarity>,
        orbit_index: u32,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: Option<Orbit>,
        orbit_distance: f64,
        orbits: Vec<Orbit>,
        seed: Rc<str>,
        settings: GenerationSettings,
        size_modifier: i32,
    ) -> (AstronomicalObject, Vec<AstronomicalObject>) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_orbit{}_bdy{}",
                coord, system_index, star_id, orbit_index, orbital_point_id
            ),
        );
        let rolled_size = rng.roll(1, 400, size_modifier);
        let mut to_return = AstronomicalObject::Void;
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut density = 0.0;
        let mut size = CelestialBodySize::Moonlet;
        if rolled_size <= 21 {
            // TODO: Debris disk
            to_return = AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Debris)),
            ));
        } else if rolled_size <= 86 {
            // TODO: Asteroid belt
            to_return = AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Asteroid)),
            ));
        } else if rolled_size <= 96 {
            // TODO: Ash belt
            to_return = AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Ash)),
            ));
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
        } else if rolled_size <= 240 {
            // Coreless rock planet
            min_density = 3.0;
            max_density = 4.5;
            size = CelestialBodySize::Standard;
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
        density = rng.roll(
            1,
            ((max_density * 1000.0) as u32 - (min_density * 1000.0) as u32) + 1,
            (min_density * 1000.0) as i32 - 1,
        ) as f32
            / 1000.0;
        let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
        let size_constraint = get_size_constraint(size, &mut rng);
        let radius = size_constraint * (blackbody_temp as f32 / (density / 5.513)).sqrt(); // in Earth radii
        let surface_gravity = density * radius;
        let mass = density * radius.powf(3.0);

        let moons: Vec<AstronomicalObject> =
            TelluricBodyDetails::generate_moons_for_telluric_body();

        if discriminant(&to_return) == discriminant(&AstronomicalObject::Void) {
            to_return = AstronomicalObject::TelluricBody(CelestialBody::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                mass,
                radius,
                density,
                size,
                CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                    CelestialBodySubType::Rocky,
                )),
            ));
        }

        (to_return, moons)
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
            size: CelestialBodySize::Moonlet,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                CelestialBodySubType::Metallic,
            )),
        }
    }

    /// Generates a fully fledged metallic body.
    pub fn generate_metallic_body(
        coord: SpaceCoordinates,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_luminosity: f32,
        star_traits: &Vec<StarPeculiarity>,
        orbit_index: u32,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: Option<Orbit>,
        orbit_distance: f64,
        orbits: Vec<Orbit>,
        seed: Rc<str>,
        settings: GenerationSettings,
        size_modifier: i32,
    ) -> (AstronomicalObject, Vec<AstronomicalObject>) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_orbit{}_bdy{}",
                coord, system_index, star_id, orbit_index, orbital_point_id
            ),
        );
        let rolled_size = rng.roll(1, 400, size_modifier);
        let mut to_return = AstronomicalObject::Void;
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut density = 0.0;
        let mut size = CelestialBodySize::Moonlet;
        if rolled_size <= 61 {
            // TODO: Dust belt
            to_return = AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Dust)),
            ));
        } else if rolled_size <= 131 {
            // TODO: Meteoroid belt
            to_return = AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Meteoroid)),
            ));
        } else if rolled_size <= 141 {
            // TODO: Ore belt
            to_return = AstronomicalObject::TelluricDisk(CelestialDisk::new(
                own_orbit.clone(),
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Ore)),
            ));
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
            size = CelestialBodySize::Giant;
        }
        density = rng.roll(
            1,
            ((max_density * 1000.0) as u32 - (min_density * 1000.0) as u32) + 1,
            (min_density * 1000.0) as i32 - 1,
        ) as f32
            / 1000.0;
        let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
        let size_constraint = get_size_constraint(size, &mut rng);
        let radius = size_constraint * (blackbody_temp as f32 / (density / 5.513)).sqrt(); // in Earth radii
        let surface_gravity = density * radius;
        let mass = density * radius.powf(3.0);

        let moons: Vec<AstronomicalObject> =
            TelluricBodyDetails::generate_moons_for_telluric_body();

        if discriminant(&to_return) == discriminant(&AstronomicalObject::Void) {
            to_return = AstronomicalObject::TelluricBody(CelestialBody::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                mass,
                radius,
                density,
                size,
                CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                    CelestialBodySubType::Metallic,
                )),
            ));
        }

        (to_return, moons)
    }

    pub(crate) fn generate_moons_for_telluric_body() -> Vec<AstronomicalObject> {
        Vec::new()
    }
}
