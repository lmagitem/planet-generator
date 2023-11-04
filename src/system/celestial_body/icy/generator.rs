use crate::internal::*;
use crate::prelude::*;
use crate::system::celestial_body::generator::get_size_constraint;
use crate::system::contents::utils::calculate_blackbody_temperature;

impl IcyBodyDetails {
    pub(crate) fn generate_icy_body_stub(orbital_point_id: u32) -> CelestialBody {
        CelestialBody {
            stub: true,
            name: "Temporary name".into(),
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            mass: 0.0,
            radius: 0.0,
            density: 0.0,
            size: CelestialBodySize::Moonlet,
            details: CelestialBodyDetails::Icy(IcyBodyDetails {}),
        }
    }

    /// Generates a fully fledged icy body.
    pub fn generate_icy_body(
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
        let moons: Vec<AstronomicalObject>;
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut density = 0.0;
        let mut size = CelestialBodySize::Moonlet;
        if rolled_size <= 21 {
            // TODO: Frost belt
            to_return = AstronomicalObject::IcyDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Frost)),
            ));
        } else if rolled_size <= 61 {
            // TODO: Comet belt
            to_return = AstronomicalObject::IcyDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Comet)),
            ));
        } else if rolled_size <= 65 {
            // TODO: Comet cloud
            to_return = AstronomicalObject::IcyDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                ).into(),
                CelestialDiskType::Shell,
            ));
        } else if rolled_size <= 105 {
            // Coreless ice dwarf
            min_density = 1.0;
            max_density = 1.83;
            size = CelestialBodySize::Tiny;
        } else if rolled_size <= 135 {
            // Ice dwarf
            min_density = 1.63;
            max_density = 2.6;
            size = CelestialBodySize::Tiny;
        } else if rolled_size <= 140 {
            // Coreless ice dwarf
            min_density = 1.0;
            max_density = 1.5;
            size = CelestialBodySize::Small;
        } else if rolled_size <= 170 {
            // Ice dwarf
            min_density = 1.5;
            max_density = 3.9;
            size = CelestialBodySize::Small;
        } else if rolled_size <= 175 {
            // Coreless ice planet
            min_density = 1.0;
            max_density = 1.5;
            size = CelestialBodySize::Standard;
        } else if rolled_size <= 255 {
            // Ice planet
            min_density = 1.5;
            max_density = 5.5;
            size = CelestialBodySize::Standard;
        } else if Self::is_after_snow_line(own_orbit.clone()) && rolled_size <= 305 {
            // Ice small giant
            min_density = 1.2;
            max_density = 1.6;
            size = CelestialBodySize::Large;
        } else if Self::is_after_snow_line(own_orbit.clone()) && rolled_size <= 395 {
            // Ice giant
            min_density = 0.6;
            max_density = 1.3;
            size = CelestialBodySize::Giant;
        } else if Self::is_after_snow_line(own_orbit.clone()) {
            // Ice supergiant
            min_density = 0.9;
            max_density = 1.6;
            size = CelestialBodySize::Supergiant;
        } else {
            // Ice planet
            min_density = 1.5;
            max_density = 5.5;
            size = CelestialBodySize::Standard;
        }
        density = rng.roll(
            1,
            ((max_density * 1000.0) as u32 - (min_density * 1000.0) as u32) + 1,
            (min_density * 1000.0) as i32 - 1,
        ) as f32
            / 1000.0;
        let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);

        if size != CelestialBodySize::Giant
            && size != CelestialBodySize::Supergiant
            && size != CelestialBodySize::Hypergiant
        {
            let size_constraint = get_size_constraint(size, &mut rng);
            let radius = size_constraint * (blackbody_temp as f32 / (density / 5.513)).sqrt(); // in Earth radii
            let surface_gravity = density * radius;
            let mass = density * radius.powf(3.0);

            moons = TelluricBodyDetails::generate_moons_for_telluric_body();

            if discriminant(&to_return) == discriminant(&AstronomicalObject::Void) {
                to_return = AstronomicalObject::IcyBody(CelestialBody::new(
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
                    CelestialBodyDetails::Icy(IcyBodyDetails::new()),
                ));
            }
        } else {
            moons = Vec::new();
        }

        (to_return, moons)
    }

    fn is_after_snow_line(own_orbit: Option<Orbit>) -> bool {
        own_orbit.clone().unwrap_or_default().zone == ZoneType::OuterZone
    }
}
