use crate::internal::types::MoonDistance;
use crate::internal::*;
use crate::prelude::*;
use crate::system::celestial_body::gaseous::constants::MASS_TO_DENSITY_DATASET;
use crate::system::contents::utils::{
    calculate_blackbody_temperature, calculate_radius, calculate_surface_gravity,
};

impl GaseousBodyDetails {
    /// Returns the generated gas giant and its list of moons.
    pub(crate) fn generate_gas_giant(
        body_id: u32,
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
        orbit: Orbit,
        orbit_distance: f64,
        populated_orbit_index: u32,
        next_id: &mut u32,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> (OrbitalPoint, Vec<OrbitalPoint>) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_gas_bdy{}",
                coord, system_index, star_id, body_id
            ),
        );
        let mut moons: Vec<OrbitalPoint> = Vec::new();
        let mut object = AstronomicalObject::Void;
        let special_traits: Vec<GasGiantSpecialTrait>;
        let is_proto_giant = if let Some(traits) = settings
            .clone()
            .celestial_body
            .gaseous_body_settings
            .fixed_special_traits
        {
            special_traits = traits.clone();
            traits
                .iter()
                .find(|o| discriminant(*o) == discriminant(&GasGiantSpecialTrait::ProtoGiant))
                .is_some()
        } else {
            special_traits = Vec::new();
            false
        };
        let size_modifier =
            Self::get_gas_body_size_modifier(star_mass, star_type, &mut rng, is_proto_giant);
        let roll_result = rng.roll(1, 400, size_modifier);

        let mut mass: f64 = 0.0;
        let mut size = CelestialBodySize::Puny;
        let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
        if !is_proto_giant && roll_result <= 2 {
            // TODO: Gas cloud
            object = AstronomicalObject::GaseousBody(CelestialBody::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                body_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                0.0,
                0.0,
                0.0,
                0.0,
                blackbody_temp,
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
                    star_name,
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

            object = AstronomicalObject::GaseousBody(CelestialBody::new(
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
                CelestialBodyDetails::Gaseous(GaseousBodyDetails::new(special_traits)),
            ));
        }

        (
            OrbitalPoint::new(
                body_id,
                Some(orbit.clone()),
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
    panic!("Mass value is out of range!");
}
