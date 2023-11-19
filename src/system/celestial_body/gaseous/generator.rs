use crate::internal::*;
use crate::prelude::*;
use crate::system::celestial_body::gaseous::constants::MASS_TO_DENSITY_DATASET;
use crate::system::contents::utils::calculate_blackbody_temperature;

impl GaseousBodyDetails {
    /// Returns the generated gas giant and its list of moons.
    pub(crate) fn generate_gas_giant(
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_name: Rc<str>,
        star_mass: f32,
        star_luminosity: f32,
        star_type: &StarSpectralType,
        star_traits: &Vec<StarPeculiarity>,
        orbit_distance: f64,
        orbit_index: u32,
        populated_orbit_index: u32,
        mut next_id: u32,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> (AstronomicalObject, Vec<AstronomicalObject>) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_gas_bdy{}",
                coord, system_index, star_id, orbital_point_id
            ),
        );
        let mut to_return = AstronomicalObject::Void;
        let special_traits: Vec<GasGiantSpecialTrait>;
        let is_proto_giant = if let Some(traits) = settings
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

        let mut mass: f32 = 0.0;
        let mut size = CelestialBodySize::Puny;
        let blackbody_temp = calculate_blackbody_temperature(star_luminosity, orbit_distance);
        if !is_proto_giant && roll_result <= 2 {
            // TODO: Gas cloud
            to_return = AstronomicalObject::GaseousBody(CelestialBody::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                0.0,
                0.0,
                0.0,
                blackbody_temp,
                CelestialBodySize::Large,
                CelestialBodyDetails::Cloud(CelestialBodyComposition::Gaseous),
            ));
        } else if !is_proto_giant && roll_result <= 6 {
            // TODO: Gas belt
            to_return = AstronomicalObject::GaseousDisk(CelestialDisk::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                CelestialDiskType::Belt(CelestialBeltDetails::new(CelestialBeltType::GasBelt)),
            ));
        } else if roll_result <= 106 {
            // Gas planet (2 to 16 masses)
            mass = rng.roll(1, 1600 - (200 - 1), 200 - 1) as f32 / 100.0;
            size = CelestialBodySize::Large;
        } else if roll_result <= 326 {
            // Gas giant (16.001 to 162)
            mass = rng.roll(1, 16200 - (1600 - 1), 1600 - 1) as f32 / 100.0;
            size = CelestialBodySize::Giant;
        } else if roll_result <= 386 {
            // Gas supergiant (162.001 to 2000)
            mass = rng.roll(1, 200000 - (16200 - 1), 16200 - 1) as f32 / 100.0;
            size = CelestialBodySize::Supergiant;
        } else if roll_result > 396 && roll_result <= 400 {
            // TODO: Brown dwarf
            mass = rng.roll(1, 800000 - (413140 - 1), 413140 - 1) as f32 / 100.0;
            size = CelestialBodySize::Hypergiant;
        } else {
            // Gas hypergiant (2000.001 to 4131)
            mass = rng.roll(1, 413100 - (200000 - 1), 200000 - 1) as f32 / 100.0;
            size = CelestialBodySize::Hypergiant;
        }

        let density = MathUtils::round_f32_to_precision(
            interpolate_density(mass) + (rng.roll(1, 61, -31) as f32 / 100.0),
            4,
        );
        let radius = (mass / density).cbrt();
        // TODO: Atmospheric composition

        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_gas_bdy{}_moons",
                coord, system_index, star_id, orbital_point_id
            ),
        );
        let moons: Vec<AstronomicalObject> = Vec::new();
        let moonlets: i8 = rng.roll(
            2,
            6,
            if orbit_distance < 0.1 {
                -10
            } else if orbit_distance < 0.5 {
                -8
            } else if orbit_distance < 0.75 {
                -6
            } else if orbit_distance < 1.5 {
                -3
            } else {
                0
            },
        ) as i8;
        let composition = rng
            .get_result(&CopyableRollToProcess::new(
                vec![
                    CopyableWeightedResult::new(
                        CelestialRingComposition::Ice,
                        if blackbody_temp < 241 {
                            12
                        } else if blackbody_temp < 300 {
                            1
                        } else {
                            0
                        },
                    ),
                    CopyableWeightedResult::new(
                        CelestialRingComposition::Rock,
                        if blackbody_temp < 241 { 5 } else { 12 },
                    ),
                    CopyableWeightedResult::new(CelestialRingComposition::Metal, 1),
                ],
                RollMethod::SimpleRoll,
            ))
            .expect("Should have picked a ring composition.");
        next_id += 1;
        let rings: CelestialDisk = if moonlets < 4 {
            CelestialDisk::new(
                None, // TODO
                next_id,
                format!(
                    "{}{} A Ring",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Unnoticeable,
                    CelestialRingComposition::Dust,
                )),
            )
        } else if moonlets < 6 {
            CelestialDisk::new(
                None,
                next_id,
                format!(
                    "{}{} A Ring",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Noticeable,
                    composition,
                )),
            )
        } else if moonlets < 10 {
            CelestialDisk::new(
                None,
                next_id,
                format!(
                    "{}{} A Ring",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Visible,
                    composition,
                )),
            )
        } else {
            CelestialDisk::new(
                None,
                next_id,
                format!(
                    "{}{} A Ring",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8)
                )
                .into(),
                CelestialDiskType::Ring(CelestialRingDetails::new(
                    CelestialRingLevel::Spectacular,
                    composition,
                )),
            )
        };
        let major_moons: i8 = rng.roll(
            1,
            6,
            if orbit_distance < 0.1 {
                -6
            } else if orbit_distance < 0.5 {
                -5
            } else if orbit_distance < 0.75 {
                -4
            } else if orbit_distance < 1.5 {
                -1
            } else {
                0
            },
        ) as i8; // TODO: Determine major moon sizes
        let other_moonlets: i8 = rng.roll(
            1,
            6,
            if orbit_distance < 0.5 {
                -6
            } else if orbit_distance < 0.75 {
                -5
            } else if orbit_distance < 1.5 {
                -4
            } else if orbit_distance < 3.0 {
                -1
            } else {
                0
            },
        ) as i8;

        if discriminant(&to_return) == discriminant(&AstronomicalObject::Void) {
            to_return = AstronomicalObject::GaseousBody(CelestialBody::new(
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
                CelestialBodyDetails::Gaseous(GaseousBodyDetails::new(special_traits)),
            ));
        }

        (to_return, moons)
    }

    fn get_gas_body_size_modifier(
        star_mass: f32,
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

pub fn interpolate_density(mass: f32) -> f32 {
    let mut prev_point = MASS_TO_DENSITY_DATASET[0];
    for &point in MASS_TO_DENSITY_DATASET.iter().skip(1) {
        if mass <= prev_point.0 && mass >= point.0 {
            if prev_point.0 == point.0 {
                return point.1;
            }
            let fraction = (mass - point.0) as f32 / (prev_point.0 - point.0) as f32;
            return point.1 + fraction * (prev_point.1 - point.1);
        }
        prev_point = point;
    }
    panic!("Mass value is out of range!");
}
