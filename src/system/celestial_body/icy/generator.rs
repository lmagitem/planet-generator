use crate::internal::*;
use crate::prelude::TelluricSpecialTrait::UnusualCore;
use crate::prelude::*;
use crate::system::celestial_body::generator::{
    downsize_world_by, generate_acceptable_telluric_parameters, get_size_constraint, get_world_type,
};
use crate::system::celestial_body::telluric::generator::generate_peculiarities;
use crate::system::contents::utils::{
    calculate_blackbody_temperature, calculate_radius, calculate_surface_gravity,
};
use crate::system::orbital_point::generator::{
    calculate_orbital_period_from_earth_masses, calculate_planet_orbit_eccentricity,
    complete_orbit_with_period_and_eccentricity,
};

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
            gravity: 0.0,
            blackbody_temperature: 0,
            size: CelestialBodySize::Puny,
            details: CelestialBodyDetails::Icy(IcyBodyDetails {
                world_type: CelestialBodyWorldType::Ice,
            }),
        }
    }

    /// Generates a fully fledged icy body.
    pub fn generate_icy_body(
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
        let mut moons: Vec<OrbitalPoint> = Vec::new();
        let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
        let mut special_traits = Vec::new();
        let size_parameters = Self::determine_icy_size(
            &star_name,
            populated_orbit_index,
            body_id,
            &own_orbit,
            &orbits,
            &mut rng,
            rolled_size,
            blackbody_temp,
            is_moon,
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
        let mut mass = size_parameters.4;
        let mut density = 0.0;
        let mut radius = 0.0;

        if size != CelestialBodySize::Giant
            && size != CelestialBodySize::Supergiant
            && size != CelestialBodySize::Hypergiant
            && discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void)
        {
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

                let body_type = CelestialBodyComposition::Icy;
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
                let mut world_type = get_world_type(
                    size,
                    CelestialBodyComposition::Icy,
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

                to_return = TelluricBodyDetails::bundle_world_first_pass(
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
                    TelluricBodyComposition::Icy,
                    world_type,
                    special_traits,
                    &moons,
                    is_moon,
                );
            }
        } else if discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void) {
            let density = (rng.roll(
                1,
                ((max_density * 1000.0) as u32 - (min_density * 1000.0) as u32) + 1,
                (min_density * 1000.0) as i32 - 1,
            ) as f32
                / 1000.0);
            let radius = calculate_radius(mass as f64, density as f64) as f32;
            let surface_gravity = calculate_surface_gravity(density, radius);

            moons = MoonGenerator::generate_giants_moons(
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
                false,
            );

            to_return = OrbitalPoint::new(
                body_id,
                own_orbit.clone(),
                AstronomicalObject::IcyBody(CelestialBody::new(
                    None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                    body_id,
                    format!(
                        "{}{}",
                        star_name,
                        StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                    )
                    .into(),
                    mass,
                    radius,
                    density,
                    surface_gravity,
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

    fn determine_icy_size(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
        rng: &mut SeededDiceRoller,
        rolled_size: i64,
        blackbody_temp: u32,
        is_moon: bool,
        special_traits: &mut Vec<TelluricSpecialTrait>,
    ) -> (OrbitalPoint, f64, f64, CelestialBodySize, f32) {
        let mut to_return = OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::Void,
            orbits.clone(),
        );
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut size = CelestialBodySize::Puny;
        let mut mass = 0.0;

        if is_moon {
            min_density = 0.6;
            max_density = 3.0;
        } else if rolled_size <= 21 {
            // Frost belt
            to_return = if blackbody_temp >= 170 {
                TelluricBodyDetails::make_dust_belt(
                    &star_name,
                    populated_orbit_index,
                    orbital_point_id,
                    &own_orbit,
                    &orbits,
                )
            } else {
                Self::make_frost_belt(
                    &star_name,
                    populated_orbit_index,
                    orbital_point_id,
                    &own_orbit,
                    &orbits,
                )
            };
        } else if rolled_size <= 61 {
            // Comet belt
            to_return = if blackbody_temp >= 170 {
                TelluricBodyDetails::make_debris_disk(
                    &star_name,
                    populated_orbit_index,
                    orbital_point_id,
                    &own_orbit,
                    &orbits,
                )
            } else {
                Self::make_comet_belt(
                    &star_name,
                    populated_orbit_index,
                    orbital_point_id,
                    &own_orbit,
                    &orbits,
                )
            };
        } else if rolled_size <= 65 {
            // Comet cloud
            to_return = if blackbody_temp >= 170 {
                TelluricBodyDetails::make_asteroid_belt(
                    &star_name,
                    populated_orbit_index,
                    orbital_point_id,
                    &own_orbit,
                    &orbits,
                )
            } else {
                Self::make_comet_cloud(
                    &star_name,
                    populated_orbit_index,
                    orbital_point_id,
                    &own_orbit,
                    &orbits,
                )
            };
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

        (to_return, min_density, max_density, size, mass)
    }

    fn make_comet_cloud(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::IcyDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Shell,
            )),
            orbits.clone(),
        )
    }

    fn make_comet_belt(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::IcyDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Comet)),
            )),
            orbits.clone(),
        )
    }

    fn make_frost_belt(
        star_name: &Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: &Option<Orbit>,
        orbits: &Vec<Orbit>,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::IcyDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::Frost)),
            )),
            orbits.clone(),
        )
    }

    fn is_after_snow_line(own_orbit: Option<Orbit>) -> bool {
        own_orbit.clone().unwrap_or_default().zone == ZoneType::OuterZone
    }

    fn is_temperature_low_enough_to_retain_water(temperature: u32) -> bool {
        temperature < 321
    }
}
