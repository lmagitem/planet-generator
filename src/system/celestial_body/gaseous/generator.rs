use crate::generator::types::GenerationSettings;
use crate::prelude::types::{TelluricRotationDifference, TideLockTarget};
use crate::system::celestial_body::gaseous::constants::MASS_TO_DENSITY_DATASET;
use crate::system::celestial_body::gaseous::GaseousBodyDetails;
use crate::system::celestial_body::moon::types::{MoonDistance, ParentBodyInfo};
use crate::system::celestial_body::moon::MoonGenerator;
use crate::system::celestial_body::traits::CelestialBodySpecialTrait;
use crate::system::celestial_body::types::CelestialBodyComposition;
use crate::system::celestial_body::types::CelestialBodyDetails;
use crate::system::celestial_body::types::CelestialBodySize;
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
use crate::system::star::types::StarSpectralType;
use crate::utils::conversion::ConversionUtils;
use crate::utils::math::MathUtils;
use crate::utils::string::StringUtils;
use seeded_dice_roller::SeededDiceRoller;
use std::fmt::Write as _;
use std::mem::discriminant;

impl GaseousBodyDetails {
    /// Returns the generated gas giant and its list of moons.
    pub(crate) fn generate_gas_giant(
        ctx: &StarGenContext,
        settings: &GenerationSettings,
        body_id: u32,
        orbit: Orbit,
        orbit_distance: f64,
        populated_orbit_index: u32,
        next_id: &mut u32,
    ) -> (OrbitalPoint, Vec<OrbitalPoint>) {
        let mut rng = ctx.system.with_step(|step| {
            let _ = write!(
                step,
                "sys_{}_{}_str_{}_gas_bdy{}",
                ctx.system.coord, ctx.system.system_index, ctx.star_id, body_id
            );
            SeededDiceRoller::new(&settings.seed, step)
        });
        let mut moons: Vec<OrbitalPoint> = Vec::new();
        let mut object = AstronomicalObject::Void;
        let mut this_orbit = orbit.clone();
        let mut special_traits: Vec<CelestialBodySpecialTrait>;
        let is_proto_giant = if let Some(traits) = &settings
            .celestial_body
            .gaseous_body_settings
            .fixed_special_traits
        {
            special_traits = traits.clone();
            traits
                .iter()
                .find(|o| discriminant(*o) == discriminant(&CelestialBodySpecialTrait::ProtoGiant))
                .is_some()
        } else {
            special_traits = Vec::new();
            false
        };
        let size_modifier = Self::get_gas_body_size_modifier(
            ctx.star_mass,
            &ctx.star_type,
            &mut rng,
            is_proto_giant,
        );
        let roll_result = rng.roll(1, 400, size_modifier);

        let mut mass: f64 = 0.0;
        let mut size = CelestialBodySize::Puny;
        let blackbody_temp = calculate_blackbody_temperature(ctx.star_luminosity, orbit_distance);
        if !is_proto_giant && roll_result <= 2 {
            // TODO: Gas cloud
            object = AstronomicalObject::GaseousBody(CelestialBody::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                body_id,
                format!(
                    "{}{}",
                    ctx.star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                0.0,
                0.0,
                0.0,
                0.0,
                blackbody_temp,
                0,
                CelestialBodySize::Large,
                CelestialBodyDetails::Cloud(CelestialBodyComposition::Gaseous),
            ));
        } else if !is_proto_giant && roll_result <= 6 {
            // TODO: Gas belt
            object = AstronomicalObject::GaseousDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                body_id,
                format!(
                    "{}{}",
                    ctx.star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::GasBelt)),
            ));
        } else if roll_result <= 106 {
            // Gas planet (2 to 16 masses)
            mass = rng.roll(1, 1600 - (200 - 1), 200 - 1) as f64 / 100.0;
            size = CelestialBodySize::Large;
        } else if roll_result <= 326 {
            // Gas giant (16.001 to 162)
            mass = rng.roll(1, 16200 - (1600 - 1), 1600 - 1) as f64 / 100.0;
            size = CelestialBodySize::Giant;
        } else if roll_result <= 386 {
            // Gas supergiant (162.001 to 2000)
            mass = rng.roll(1, 200000 - (16200 - 1), 16200 - 1) as f64 / 100.0;
            size = CelestialBodySize::Supergiant;
        } else if roll_result > 396 && roll_result <= 400 {
            // TODO: Brown dwarf
            mass = rng.roll(1, 800000 - (413140 - 1), 413140 - 1) as f64 / 100.0;
            size = CelestialBodySize::Hypergiant;
        } else {
            // Gas hypergiant (2000.001 to 4131)
            mass = rng.roll(1, 413100 - (200000 - 1), 200000 - 1) as f64 / 100.0;
            size = CelestialBodySize::Hypergiant;
        }

        if discriminant(&object) == discriminant(&AstronomicalObject::Void) {
            let density = MathUtils::round_f32_to_precision(
                (interpolate_density(mass) + (rng.roll(1, 61, -31) as f64 / 100.0)) as f32,
                4,
            );
            let radius = calculate_radius(mass, density as f64);
            let surface_gravity = calculate_surface_gravity(density, radius);
            // TODO: Atmospheric composition

            this_orbit = complete_orbit_with_orbital_period(
                ctx.system.coord,
                ctx.system.system_index,
                ctx.star_id,
                ConversionUtils::solar_mass_to_earth_mass(ctx.star_mass),
                ctx.gas_giant_arrangement,
                body_id,
                &Some(orbit.clone()),
                orbit_distance,
                orbit.clone().average_distance_from_system_center,
                true,
                blackbody_temp,
                mass,
                size,
                false,
                settings,
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

            let mut telluric_special_traits = Vec::new();
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
                &mut telluric_special_traits,
                &moons,
                false,
                MoonDistance::Any,
                settings,
            );
            if telluric_special_traits.contains(&CelestialBodySpecialTrait::UnusualRotation(
                TelluricRotationDifference::Resonant,
            )) {
                special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                    TelluricRotationDifference::Resonant,
                ))
            }
            if telluric_special_traits.contains(&CelestialBodySpecialTrait::UnusualRotation(
                TelluricRotationDifference::Retrograde,
            )) {
                special_traits.push(CelestialBodySpecialTrait::UnusualRotation(
                    TelluricRotationDifference::Retrograde,
                ))
            }
            if telluric_special_traits.contains(&CelestialBodySpecialTrait::TideLocked(
                TideLockTarget::Orbited,
            )) {
                special_traits.push(CelestialBodySpecialTrait::TideLocked(
                    TideLockTarget::Orbited,
                ))
            }
            if telluric_special_traits.contains(&CelestialBodySpecialTrait::TideLocked(
                TideLockTarget::Satellite,
            )) {
                special_traits.push(CelestialBodySpecialTrait::TideLocked(
                    TideLockTarget::Satellite,
                ))
            }

            object = AstronomicalObject::GaseousBody(CelestialBody::new(
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
                CelestialBodyDetails::Gaseous(GaseousBodyDetails::new(special_traits)),
            ));
        }

        (
            OrbitalPoint::new(
                body_id,
                Some(this_orbit),
                object,
                moons
                    .clone()
                    .iter()
                    .filter(|o| o.own_orbit.is_some())
                    .map(|o| o.own_orbit.clone().unwrap_or_default())
                    .collect::<Vec<Orbit>>(),
            ),
            moons,
        )
    }

    fn get_gas_body_size_modifier(
        star_mass: f64,
        star_type: &StarSpectralType,
        rng: &mut SeededDiceRoller,
        is_proto_giant: bool,
    ) -> i32 {
        let mut size_modifier = 0;
        size_modifier += match star_type {
            StarSpectralType::WR(_)
            | StarSpectralType::O(_)
            | StarSpectralType::B(_)
            | StarSpectralType::A(_) => {
                if rng.roll(1, 50, 0) == 1 {
                    0
                } else {
                    -(star_mass * 10.0) as i32
                }
            }
            StarSpectralType::F(_) => 20,
            StarSpectralType::K(_) => -20,
            StarSpectralType::M(_) => -40,
            StarSpectralType::L(_) | StarSpectralType::T(_) | StarSpectralType::Y(_) => -100,
            _ => 0,
        };
        size_modifier += if is_proto_giant { 100 } else { 0 };
        size_modifier
    }
}

pub fn interpolate_density(mass: f64) -> f64 {
    let mut prev_point = MASS_TO_DENSITY_DATASET[0];
    for &point in MASS_TO_DENSITY_DATASET.iter().skip(1) {
        if mass <= prev_point.0 && mass >= point.0 {
            if prev_point.0 == point.0 {
                return point.1;
            }
            let fraction = (mass - point.0) / (prev_point.0 - point.0);
            return point.1 + fraction * (prev_point.1 - point.1);
        }
        prev_point = point;
    }
    // The dataset spans (f64::MAX, _) down to (0.0, _): only a negative (or NaN)
    // mass falls through, which used to panic. Clamp to the dataset's edge instead.
    if mass > MASS_TO_DENSITY_DATASET[0].0 {
        MASS_TO_DENSITY_DATASET[0].1
    } else {
        MASS_TO_DENSITY_DATASET[MASS_TO_DENSITY_DATASET.len() - 1].1
    }
}
