use crate::system::celestial_body::generator::{
    generate_acceptable_telluric_parameters, get_world_type,
};
use crate::system::celestial_body::icy::IcyBodyDetails;
use crate::system::celestial_body::moon::types::{MoonDistance, ParentBodyInfo};
use crate::system::celestial_body::moon::MoonGenerator;
use crate::system::celestial_body::telluric::generator::generate_peculiarities;
use crate::system::celestial_body::telluric::types::TelluricBodyComposition;
use crate::system::celestial_body::telluric::TelluricBodyDetails;
use crate::system::celestial_body::traits::types::TelluricCoreDifference;
use crate::system::celestial_body::traits::CelestialBodySpecialTrait;
use crate::system::celestial_body::types::CelestialBodyComposition;
use crate::system::celestial_body::types::CelestialBodyDetails;
use crate::system::celestial_body::types::CelestialBodySize;
use crate::system::celestial_body::world::types::CelestialBodyWorldType;
use crate::system::celestial_body::world::WorldGenerator;
use crate::system::celestial_body::CelestialBody;
use crate::system::celestial_disk::belt::types::CelestialBeltType;
use crate::system::celestial_disk::belt::CelestialBeltDetails;
use crate::system::celestial_disk::types::CelestialDiskType;
use crate::system::celestial_disk::CelestialDisk;
use crate::system::contents::utils::{
    calculate_blackbody_temperature, calculate_radius, calculate_surface_gravity,
};
use crate::system::contents::zones::get_orbit_with_updated_zone;
use crate::system::orbital_point::generator::{
    complete_orbit_with_orbital_period, complete_orbit_with_rotation_and_axis,
};
use crate::system::orbital_point::types::AstronomicalObject;
use crate::system::orbital_point::types::Orbit;
use crate::system::orbital_point::OrbitalPoint;
use crate::system::star::types::StarGenContext;
use crate::utils::conversion::ConversionUtils;
use crate::utils::string::StringUtils;
use seeded_dice_roller::SeededDiceRoller;
use std::fmt::Write as _;
use std::mem::discriminant;
use std::rc::Rc;

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
            tidal_heating: 0,
            size: CelestialBodySize::Puny,
            details: CelestialBodyDetails::Icy(IcyBodyDetails {
                world_type: CelestialBodyWorldType::Ice,
                special_traits: Vec::new(),
            }),
        }
    }

    /// Generates a fully fledged icy body.
    pub fn generate_icy_body(
        ctx: &StarGenContext,
        body_id: u32,
        populated_orbit_index: u32,
        own_orbit: Option<Orbit>,
        orbit_distance: f64,
        orbits: Vec<Orbit>,
        next_id: &mut u32,
        size_modifier: i32,
        is_moon: bool,
        fixed_size: Option<CelestialBodySize>,
    ) -> (OrbitalPoint, Vec<OrbitalPoint>) {
        let mut rng = ctx.system.with_step(&ctx.system.settings.seed, |step| {
            let _ = write!(
                step,
                "sys_{}_{}_str_{}_bdy{}",
                ctx.system.coord, ctx.system.system_index, ctx.star_id, body_id
            );
        });
        let rolled_size = rng.roll(1, 400, size_modifier);
        let mut moons: Vec<OrbitalPoint> = Vec::new();
        let blackbody_temp = calculate_blackbody_temperature(ctx.star_luminosity, orbit_distance);
        let mut special_traits = Vec::new();
        let size_parameters = Self::determine_icy_size(
            &ctx.star_name,
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
        let density;
        let radius;

        if size != CelestialBodySize::Giant
            && size != CelestialBodySize::Supergiant
            && size != CelestialBodySize::Hypergiant
            && discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void)
        {
            if discriminant(&to_return.object) == discriminant(&AstronomicalObject::Void) {
                generate_peculiarities(
                    &ctx.system.coord,
                    &ctx.system.system_index,
                    &ctx.star_id,
                    &body_id,
                    ctx.settings(),
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
                let surface_gravity = calculate_surface_gravity(density, radius);
                let world_type = get_world_type(
                    size,
                    CelestialBodyComposition::Icy,
                    blackbody_temp,
                    ctx.primary_star_mass,
                    ctx.star_age,
                    &mut rng,
                );

                let mut this_orbit = if !is_moon {
                    complete_orbit_with_orbital_period(
                        ctx.system.coord,
                        ctx.system.system_index,
                        ctx.star_id,
                        ConversionUtils::solar_mass_to_earth_mass(ctx.star_mass),
                        ctx.gas_giant_arrangement,
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
                        ctx.settings(),
                    )
                } else {
                    own_orbit.clone().unwrap_or_default()
                };

                moons = MoonGenerator::generate_planets_moons(
                    ctx,
                    &ParentBodyInfo {
                        parent_orbit: own_orbit.clone().unwrap_or_default(),
                        planet_id: body_id,
                        planet_size: size,
                        planet_mass: mass,
                        planet_density: density,
                        planet_radius: radius,
                        planet_orbital_period: this_orbit.orbital_period,
                        blackbody_temperature: blackbody_temp,
                    },
                    populated_orbit_index,
                    is_moon,
                    next_id,
                );

                this_orbit = if !is_moon {
                    complete_orbit_with_rotation_and_axis(
                        ctx.system.coord,
                        ctx.system.system_index,
                        ctx.star_id,
                        ctx.star_age,
                        ConversionUtils::solar_mass_to_earth_mass(ctx.star_mass),
                        None,
                        ctx.gas_giant_arrangement,
                        ctx.system.system_traits,
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
                        ctx.settings(),
                    )
                } else {
                    this_orbit
                };

                to_return = WorldGenerator::bundle_world_first_pass(
                    ctx.star_name.clone(),
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
            let density = rng.roll(
                1,
                ((max_density * 1000.0) as u32 - (min_density * 1000.0) as u32) + 1,
                (min_density * 1000.0) as i32 - 1,
            ) as f32
                / 1000.0;
            let radius = calculate_radius(mass, density as f64);
            let surface_gravity = calculate_surface_gravity(density, radius);

            let mut this_orbit = complete_orbit_with_orbital_period(
                ctx.system.coord,
                ctx.system.system_index,
                ctx.star_id,
                ConversionUtils::solar_mass_to_earth_mass(ctx.star_mass),
                ctx.gas_giant_arrangement,
                body_id,
                &own_orbit,
                orbit_distance,
                own_orbit
                    .clone()
                    .unwrap_or_default()
                    .average_distance_from_system_center,
                true,
                blackbody_temp,
                mass,
                size,
                false,
                ctx.settings(),
            );
            this_orbit = get_orbit_with_updated_zone(this_orbit.clone(), blackbody_temp);

            moons = MoonGenerator::generate_giants_moons(
                ctx,
                &ParentBodyInfo {
                    parent_orbit: this_orbit.clone(),
                    planet_id: body_id,
                    planet_size: size,
                    planet_mass: mass,
                    planet_density: density,
                    planet_radius: radius,
                    planet_orbital_period: this_orbit.orbital_period,
                    blackbody_temperature: blackbody_temp,
                },
                populated_orbit_index,
                false,
                next_id,
            );

            this_orbit = complete_orbit_with_rotation_and_axis(
                ctx.system.coord,
                ctx.system.system_index,
                ctx.star_id,
                ctx.star_age,
                ConversionUtils::solar_mass_to_earth_mass(ctx.star_mass),
                None,
                ctx.gas_giant_arrangement,
                ctx.system.system_traits,
                body_id,
                &Some(this_orbit),
                orbit_distance,
                true,
                blackbody_temp,
                mass,
                radius,
                size,
                &mut special_traits,
                &moons,
                false,
                MoonDistance::Any,
                ctx.settings(),
            );

            to_return = OrbitalPoint::new(
                body_id,
                Some(this_orbit),
                AstronomicalObject::IcyBody(CelestialBody::new(
                    None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                    body_id,
                    format!(
                        "{}{}",
                        ctx.star_name,
                        StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                    )
                    .into(),
                    mass,
                    radius,
                    density,
                    surface_gravity,
                    blackbody_temp,
                    0,
                    size,
                    CelestialBodyDetails::Icy(IcyBodyDetails::new(
                        CelestialBodyWorldType::VolatilesGiant,
                        vec![CelestialBodySpecialTrait::NoPeculiarity],
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
        special_traits: &mut Vec<CelestialBodySpecialTrait>,
    ) -> (OrbitalPoint, f64, f64, CelestialBodySize, f64) {
        let mut to_return = OrbitalPoint::new(
            orbital_point_id,
            own_orbit.clone(),
            AstronomicalObject::Void,
            orbits.clone(),
        );
        let mut min_density = 0.0;
        let mut max_density = 5.0;
        let mut size = CelestialBodySize::Puny;
        let mut mass: f64 = 0.0;

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
            special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                TelluricCoreDifference::Coreless,
            ));
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
            special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                TelluricCoreDifference::Coreless,
            ));
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
            special_traits.push(CelestialBodySpecialTrait::UnusualCore(
                TelluricCoreDifference::Coreless,
            ));
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
            mass = rng.roll(1, 1400, 200 - 1) as f64 / 100.0;
            size = CelestialBodySize::Giant;
        } else if Self::is_temperature_low_enough_to_retain_water(blackbody_temp) {
            // Ice supergiant
            min_density = 0.9;
            max_density = 1.6;
            // `roll * roll`, NOT `powf(2.0)`: release builds inline `powf(x, 2.0)`
            // as a multiplication while debug builds call libm `pow`, and the two
            // do not always round identically — see tests/determinism.rs.
            let roll = rng.roll(1, 1000, 200 - 1) as f64;
            mass = roll * roll / 100.0;
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

    fn is_temperature_low_enough_to_retain_water(temperature: u32) -> bool {
        temperature < 321
    }
}
