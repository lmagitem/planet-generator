use crate::internal::*;
use crate::prelude::TelluricSpecialTrait::UnusualCore;
use crate::prelude::*;
use crate::system::celestial_body::generator::{
    downsize_world_by, generate_acceptable_telluric_parameters, get_size_constraint, get_world_type,
};
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
            blackbody_temperature: 0,
            size: CelestialBodySize::Puny,
            details: CelestialBodyDetails::Icy(IcyBodyDetails {
                world_type: CelestialBodyWorldType::Ice,
            }),
        }
    }

    /// Generates a fully fledged icy body.
    pub fn generate_icy_body(
        coord: SpaceCoordinates,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_age: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_luminosity: f32,
        star_traits: &Vec<StarPeculiarity>,
        primary_star_mass: f32,
        orbit_index: u32,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: Option<Orbit>,
        orbit_distance: f64,
        orbits: Vec<Orbit>,
        seed: Rc<str>,
        settings: GenerationSettings,
        size_modifier: i32,
    ) -> (OrbitalPoint, Vec<OrbitalPoint>) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_orbit{}_bdy{}",
                coord, system_index, star_id, orbit_index, orbital_point_id
            ),
        );
        let rolled_size = rng.roll(1, 400, size_modifier);
        let mut to_return = OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::Void,
            orbits.clone(),
        );
        let mut moons: Vec<OrbitalPoint> = Vec::new();
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut density = 0.0;
        let mut size = CelestialBodySize::Puny;
        let mut radius = 0.0;
        let mut mass = 0.0;
        let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
        let mut special_traits = Vec::new();
        if rolled_size <= 21 {
            // TODO: Frost belt
            to_return = OrbitalPoint::new(
                orbital_point_id,
                own_orbit.clone(),
                AstronomicalObject::IcyDisk(CelestialDisk::new(
                    None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                    orbital_point_id,
                    format!(
                        "{}{}",
                        star_name,
                        StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                    )
                    .into(),
                    CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Frost)),
                )),
                orbits.clone(),
            );
        } else if rolled_size <= 61 {
            // TODO: Comet belt
            to_return = OrbitalPoint::new(
                orbital_point_id,
                own_orbit.clone(),
                AstronomicalObject::IcyDisk(CelestialDisk::new(
                    None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                    orbital_point_id,
                    format!(
                        "{}{}",
                        star_name,
                        StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                    )
                    .into(),
                    CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Comet)),
                )),
                orbits.clone(),
            );
        } else if rolled_size <= 65 {
            // TODO: Comet cloud
            to_return = OrbitalPoint::new(
                orbital_point_id,
                own_orbit.clone(),
                AstronomicalObject::IcyDisk(CelestialDisk::new(
                    None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                    orbital_point_id,
                    format!(
                        "{}{}",
                        star_name,
                        StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                    )
                    .into(),
                    CelestialDiskType::Shell,
                )),
                orbits.clone(),
            );
        } else if rolled_size <= 105 {
            // Coreless ice dwarf
            min_density = 1.0;
            max_density = 1.83;
            size = CelestialBodySize::Tiny;
            special_traits.push(UnusualCore(TelluricCoreDifference::Coreless));
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
            special_traits.push(UnusualCore(TelluricCoreDifference::Coreless));
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
            special_traits.push(UnusualCore(TelluricCoreDifference::Coreless));
        } else if rolled_size <= 255 {
            // Ice planet
            min_density = 1.5;
            max_density = 5.5;
            size = CelestialBodySize::Standard;
        } else if Self::is_temperature_low_enough_to_retain_water(blackbody_temp)
            && rolled_size <= 305
        {
            // Ice small giant
            min_density = 1.2;
            max_density = 1.6;
            size = CelestialBodySize::Large;
        } else if Self::is_temperature_low_enough_to_retain_water(blackbody_temp)
            && rolled_size <= 395
        {
            // Ice giant
            min_density = 0.6;
            max_density = 1.3;
            mass = rng.roll(1, 1400, 200 - 1) as f32 / 100.0;
            size = CelestialBodySize::Giant;
        } else if Self::is_temperature_low_enough_to_retain_water(blackbody_temp) {
            // Ice supergiant
            min_density = 0.9;
            max_density = 1.6;
            mass = (rng.roll(1, 1000, 200 - 1) as f32).powf(2.0) / 100.0;
            size = CelestialBodySize::Supergiant;
        } else {
            // Ice planet
            min_density = 1.5;
            max_density = 5.5;
            size = CelestialBodySize::Standard;
        }

        if size != CelestialBodySize::Giant
            && size != CelestialBodySize::Supergiant
            && size != CelestialBodySize::Hypergiant
            && discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void)
        {
            if discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void) {
                let (new_density, new_size, new_radius, new_mass) =
                    generate_acceptable_telluric_parameters(
                        size_modifier,
                        &mut rng,
                        min_density,
                        max_density,
                        size,
                        blackbody_temp,
                        "icy".into(),
                    );
                density = new_density;
                size = new_size;
                radius = new_radius;
                mass = new_mass;
                let surface_gravity = density * radius;
                let mut world_type = get_world_type(
                    size,
                    CelestialBodyComposition::Icy,
                    blackbody_temp,
                    primary_star_mass,
                    &mut rng,
                );

                moons = TelluricBodyDetails::generate_moons_for_telluric_body(
                    orbit_distance,
                    size,
                    &mut rng,
                );

                to_return = TelluricBodyDetails::generate_world(
                    coord,
                    system_traits,
                    system_index,
                    star_id,
                    star_name,
                    star_age,
                    star_type,
                    star_class,
                    star_traits,
                    populated_orbit_index,
                    orbital_point_id,
                    own_orbit,
                    orbits,
                    size,
                    blackbody_temp,
                    density,
                    radius,
                    mass,
                    surface_gravity,
                    TelluricBodyComposition::Icy,
                    world_type,
                    special_traits,
                    false,
                    &moons,
                    seed,
                    settings,
                );
            }
        } else if discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void) {
            let density = rng.roll(
                1,
                ((max_density * 1000.0) as u32 - (min_density * 1000.0) as u32) + 1,
                (min_density * 1000.0) as i32 - 1,
            ) as f32
                / 1000.0;
            let radius = (mass / density).cbrt();

            to_return = OrbitalPoint::new(
                orbital_point_id,
                own_orbit.clone(),
                AstronomicalObject::IcyBody(CelestialBody::new(
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
                    blackbody_temp,
                    size,
                    CelestialBodyDetails::Icy(IcyBodyDetails::new(
                        CelestialBodyWorldType::VolatilesGiant,
                    )),
                )),
                orbits.clone(),
            );
        }

        (to_return, moons)
    }

    fn is_after_snow_line(own_orbit: Option<Orbit>) -> bool {
        own_orbit.clone().unwrap_or_default().zone == ZoneType::OuterZone
    }

    fn is_temperature_low_enough_to_retain_water(temperature: u32) -> bool {
        temperature < 321
    }
}
