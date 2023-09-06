use crate::prelude::*;
use crate::system::contents::get_next_id;
use crate::system::contents::types::GasGiantArrangement;
use crate::system::contents::zones::collect_all_zones;

pub fn generate_stars_systems(
    all_objects: &mut Vec<OrbitalPoint>,
    system_traits: &Vec<SystemPeculiarity>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) {
    let seed = galaxy.settings.seed.clone();
    let all_zones = collect_all_zones(all_objects);

    let mut number_of_bodies_per_star = vec![];
    for (index, o) in all_objects.iter_mut().enumerate() {
        if let AstronomicalObject::Star(ref mut star) = o.object {
            let number_of_bodies = generate_number_of_bodies(&system_index, &coord, galaxy, star);
            number_of_bodies_per_star.push((number_of_bodies, index));
        }
    }

    number_of_bodies_per_star
        .iter_mut()
        .for_each(|(major_bodies_left, star_index)| {
            let mut next_id = get_next_id(&all_objects);
            let star_orbital_point = &mut all_objects[*star_index];
            if let AstronomicalObject::Star(star) = &star_orbital_point.object {
                let gas_giant_arrangement = generate_gas_giant_arrangement(
                    *major_bodies_left,
                    star.orbital_point_id,
                    &star.special_traits,
                    &star.spectral_type,
                    &star.population,
                    system_traits,
                    &system_index,
                    &coord,
                    galaxy,
                );

                trace!(
                    "Major bodies left: {}, star index: {}, star id: {:#?}",
                    major_bodies_left,
                    star_index,
                    star_orbital_point.id
                );

                let mut rng = SeededDiceRoller::new(
                    &galaxy.settings.seed,
                    &format!(
                        "sys_{}_{}_str_{}_bdy{}_loc",
                        coord, system_index, star.orbital_point_id, major_bodies_left
                    ),
                );

                if let Some(orbit_radius) =
                    generate_proto_gas_giant_position(&gas_giant_arrangement, star, &mut rng)
                {
                    // If zone isn't forbidden
                    if star.zones.iter().all(|zone| {
                        orbit_radius < zone.start
                            || orbit_radius > zone.end
                            || zone.zone_type != ZoneType::ForbiddenZone
                    }) {
                        // Create an Orbit
                        let mut orbit = Orbit::new(
                            star_orbital_point.id,
                            vec![next_id],
                            orbit_radius,
                            star.orbit
                                .clone()
                                .unwrap_or(Orbit {
                                    ..Default::default()
                                })
                                .average_distance_from_system_center
                                + orbit_radius,
                            0.0,
                            0.0,
                        );

                        // Generate Gas Giant Settings
                        let celestial_body_settings = &galaxy.settings.celestial_body;
                        let gaseous_body_settings_clone =
                            celestial_body_settings.gaseous_body_settings.clone();
                        let mut fixed_special_traits = gaseous_body_settings_clone
                            .fixed_special_traits
                            .unwrap_or_else(Vec::new);
                        if !fixed_special_traits.contains(&GasGiantSpecialTrait::ProtoGiant) {
                            fixed_special_traits.push(GasGiantSpecialTrait::ProtoGiant);
                        }
                        let gaseous_settings = GaseousBodySettings {
                            fixed_special_traits: Some(fixed_special_traits),
                            ..gaseous_body_settings_clone
                        };
                        let settings = CelestialBodySettings {
                            gaseous_body_settings: gaseous_settings,
                            ..celestial_body_settings.clone()
                        };

                        // Generate the Gas Giant
                        let gas_giant = GaseousDetails::generate_gas_giant(
                            next_id,
                            None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                            next_id,
                            system_traits,
                            system_index,
                            coord,
                            seed.clone(),
                            settings,
                        );

                        // Create an Orbital Point for the Gas Giant
                        let mut object_orbital_point = OrbitalPoint::new(
                            next_id,
                            Some(orbit.clone()),
                            AstronomicalObject::GaseousBody(gas_giant),
                            vec![],
                        );

                        // Update the Star's Orbits
                        star_orbital_point.orbits.push(orbit);

                        // Add the New Orbital Point to All Objects
                        all_objects.push(object_orbital_point);
                    }
                }

                *major_bodies_left -= 1;
            }
        });
}

fn generate_number_of_bodies(
    system_index: &u16,
    coord: &SpaceCoordinates,
    galaxy: &mut Galaxy,
    star: &mut Star,
) -> i32 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!(
            "sys_{}_{}_str_{}_nbr_bdy",
            coord, system_index, star.orbital_point_id
        ),
    );

    let mut modifier = 0;
    modifier += if discriminant(&star.population) == discriminant(&StellarEvolution::Paleodwarf) {
        -10
    } else if discriminant(&star.population) == discriminant(&StellarEvolution::Subdwarf) {
        -5
    } else if discriminant(&star.population) == discriminant(&StellarEvolution::Superdwarf) {
        5
    } else if discriminant(&star.population) == discriminant(&StellarEvolution::Hyperdwarf) {
        10
    } else {
        0
    };
    modifier += (if star.age < 0.1 {
        -15.0 + star.age * 150.0
    } else {
        0.0
    }) as i32;
    modifier += (if star.mass > 4.0 {
        -star.mass * 0.2
    } else {
        0.0
    }) as i32;

    let mut possible_results = vec![
        // 0 - Empty system
        WeightedResult::new(0, 2),
        // 1 - Circumstellar disk
        WeightedResult::new(if rng.roll(1, 8, 0) == 1 { 0 } else { 1 } as i32, 2),
        // 2 - Small system
        WeightedResult::new(rng.roll(1, 4, 2) as i32, 4),
        // 3 - Standard system
        WeightedResult::new(rng.roll(1, 4, 5) as i32, 7),
        // 4 - Large system
        WeightedResult::new(rng.roll(1, 4, 10) as i32, 2),
    ];

    let number_of_bodies_index = rng
        .get_result_index(&RollToProcess::new(
            possible_results.clone(),
            RollMethod::PreparedRoll(PreparedRoll::new(2, 8, modifier)),
        ))
        .expect("The roll should return a result.");

    // Special cases
    if number_of_bodies_index == 1 {
        star.special_traits.push(StarPeculiarity::CircumstellarDisk)
    } else if number_of_bodies_index == 3 && possible_results[number_of_bodies_index].result == 9 {
        possible_results[number_of_bodies_index].result += rng.roll(1, 4, 0) as i32;
    } else if number_of_bodies_index == 4 && possible_results[number_of_bodies_index].result == 14 {
        possible_results[number_of_bodies_index].result += rng.roll(1, 4, 0) as i32;
    }

    possible_results[number_of_bodies_index].result
}

fn generate_gas_giant_arrangement(
    number_of_major_bodies: i32,
    star_id: u32,
    star_traits: &Vec<StarPeculiarity>,
    star_type: &StarSpectralType,
    star_population: &StellarEvolution,
    system_traits: &Vec<SystemPeculiarity>,
    system_index: &u16,
    coord: &SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> GasGiantArrangement {
    if number_of_major_bodies == 0 || star_traits.contains(&StarPeculiarity::CircumstellarDisk) {
        GasGiantArrangement::NoGasGiant
    } else {
        let mut nothing_chances = 50;
        let mut conventional_chances = 30;
        let mut eccentric_chances = 15;
        let mut epistellar_chances = 5;
        let modifiers = apply_gas_giant_arrangement_modifiers(
            system_traits,
            star_traits,
            star_type,
            star_population,
        );
        nothing_chances += modifiers.0;
        conventional_chances += modifiers.1;
        eccentric_chances += modifiers.2;
        epistellar_chances += modifiers.3;
        nothing_chances = if nothing_chances < 0 {
            0
        } else {
            nothing_chances
        };
        conventional_chances = if conventional_chances < 0 {
            0
        } else {
            conventional_chances
        };
        eccentric_chances = if eccentric_chances < 0 {
            0
        } else {
            eccentric_chances
        };
        epistellar_chances = if epistellar_chances < 0 {
            0
        } else {
            epistellar_chances
        };

        SeededDiceRoller::new(
            &galaxy.settings.seed,
            &format!("sys_{}_{}_str_{}_gas_arr", coord, system_index, star_id),
        )
        .get_result(&CopyableRollToProcess::new(
            vec![
                CopyableWeightedResult::new(
                    GasGiantArrangement::NoGasGiant,
                    nothing_chances as u32,
                ),
                CopyableWeightedResult::new(
                    GasGiantArrangement::ConventionalGasGiant,
                    conventional_chances as u32,
                ),
                CopyableWeightedResult::new(
                    GasGiantArrangement::EccentricGasGiant,
                    eccentric_chances as u32,
                ),
                CopyableWeightedResult::new(
                    GasGiantArrangement::EpistellarGasGiant,
                    epistellar_chances as u32,
                ),
            ],
            RollMethod::SimpleRoll,
        ))
        .expect("The roll should return a result.")
    }
}

fn apply_gas_giant_arrangement_modifiers(
    system_traits: &Vec<SystemPeculiarity>,
    star_traits: &Vec<StarPeculiarity>,
    star_type: &StarSpectralType,
    star_population: &StellarEvolution,
) -> (i32, i32, i32, i32) {
    let mut nothing_chances = 0;
    let mut conventional_chances = 0;
    let mut eccentric_chances = 0;
    let mut epistellar_chances = 0;

    if star_traits.len() > 0 {
        star_traits.iter().for_each(|t| {
            if discriminant(t) == discriminant(&StarPeculiarity::ChaoticOrbits) {
                conventional_chances += -20;
                eccentric_chances += 20;
            } else if discriminant(t) == discriminant(&StarPeculiarity::ExcessiveRadiation) {
                nothing_chances += 5;
                epistellar_chances += -5;
            } else if discriminant(t)
                == discriminant(&StarPeculiarity::AgeDifference(StarAgeDifference::Younger))
            {
                match t {
                    StarPeculiarity::AgeDifference(StarAgeDifference::MuchYounger) => {
                        nothing_chances += 20;
                        conventional_chances += -10;
                        eccentric_chances += -6;
                        epistellar_chances += -4;
                    }
                    StarPeculiarity::AgeDifference(StarAgeDifference::Younger) => {
                        nothing_chances += 10;
                        conventional_chances += -5;
                        eccentric_chances += -2;
                        epistellar_chances += -3;
                    }
                    StarPeculiarity::AgeDifference(StarAgeDifference::Older) => {
                        nothing_chances += -10;
                        conventional_chances += 7;
                        eccentric_chances += 2;
                        epistellar_chances += 1;
                    }
                    StarPeculiarity::AgeDifference(StarAgeDifference::MuchOlder) => {
                        nothing_chances += -20;
                        conventional_chances += 15;
                        eccentric_chances += 4;
                        epistellar_chances += 1;
                    }
                    _ => (),
                }
            } else if discriminant(t)
                == discriminant(&StarPeculiarity::RotationAnomaly(
                    RotationAnomalySpeed::Faster,
                ))
            {
                match t {
                    StarPeculiarity::RotationAnomaly(RotationAnomalySpeed::MuchFaster) => {
                        nothing_chances += 20;
                        conventional_chances += -20;
                        eccentric_chances += 10;
                        epistellar_chances += -10;
                    }
                    StarPeculiarity::RotationAnomaly(RotationAnomalySpeed::Faster) => {
                        nothing_chances += 10;
                        conventional_chances += -10;
                        eccentric_chances += 5;
                        epistellar_chances += -5;
                    }
                    StarPeculiarity::RotationAnomaly(RotationAnomalySpeed::Slower) => {
                        nothing_chances += -5;
                        conventional_chances += 10;
                    }
                    StarPeculiarity::RotationAnomaly(RotationAnomalySpeed::MuchSlower) => {
                        nothing_chances += -10;
                        conventional_chances += 20;
                    }
                    _ => (),
                }
            } else if discriminant(t)
                == discriminant(&StarPeculiarity::UnusualMetallicity(
                    StarMetallicityDifference::Higher,
                ))
            {
                match t {
                    StarPeculiarity::UnusualMetallicity(StarMetallicityDifference::MuchHigher) => {
                        nothing_chances += -30;
                        conventional_chances += 20;
                        epistellar_chances += 10;
                    }
                    StarPeculiarity::UnusualMetallicity(StarMetallicityDifference::Higher) => {
                        nothing_chances += -20;
                        conventional_chances += 15;
                        epistellar_chances += 5;
                    }
                    StarPeculiarity::UnusualMetallicity(StarMetallicityDifference::Lower) => {
                        nothing_chances += 10;
                        conventional_chances += -15;
                        epistellar_chances += -5;
                    }
                    StarPeculiarity::UnusualMetallicity(StarMetallicityDifference::MuchLower) => {
                        nothing_chances += 20;
                        conventional_chances += -20;
                        epistellar_chances += -10;
                    }
                    _ => (),
                }
            } else if discriminant(t) == discriminant(&StarPeculiarity::PowerfulStellarWinds)
                || discriminant(t) == discriminant(&StarPeculiarity::StrongMagneticField)
            {
                nothing_chances += 10;
                conventional_chances += -10;
                eccentric_chances += 10;
                epistellar_chances += -10;
            } else if discriminant(t)
                == discriminant(&StarPeculiarity::VariableStar(VariableStarInterval::Days))
            {
                match t {
                    StarPeculiarity::VariableStar(VariableStarInterval::Minutes) => {
                        nothing_chances += 30;
                        conventional_chances += -25;
                        eccentric_chances += 20;
                        epistellar_chances += -10;
                    }
                    StarPeculiarity::VariableStar(VariableStarInterval::Hours) => {
                        nothing_chances += 25;
                        conventional_chances += -20;
                        eccentric_chances += 15;
                        epistellar_chances += -8;
                    }
                    StarPeculiarity::VariableStar(VariableStarInterval::Days) => {
                        nothing_chances += 15;
                        conventional_chances += -15;
                        eccentric_chances += 10;
                        epistellar_chances += -5;
                    }
                    StarPeculiarity::VariableStar(VariableStarInterval::Months) => {
                        nothing_chances += 5;
                        conventional_chances += -5;
                        eccentric_chances += 5;
                        epistellar_chances += -3;
                    }
                    _ => (),
                }
            } else if discriminant(t) == discriminant(&StarPeculiarity::CircumstellarDisk) {
                nothing_chances += 100;
                conventional_chances += -100;
                eccentric_chances += -100;
                epistellar_chances += -100;
            }
        })
    }
    if system_traits.len() > 0 {
        system_traits.iter().for_each(|t| {
            if discriminant(t)
                == discriminant(&SystemPeculiarity::Cataclysm(CataclysmSeverity::Major))
            {
                match t {
                    SystemPeculiarity::Cataclysm(CataclysmSeverity::Minor) => {
                        nothing_chances += 10;
                        conventional_chances += -10;
                        eccentric_chances += 12;
                        epistellar_chances += -5;
                    }
                    SystemPeculiarity::Cataclysm(CataclysmSeverity::Major) => {
                        nothing_chances += 20;
                        conventional_chances += -20;
                        eccentric_chances += 10;
                        epistellar_chances += -5;
                    }
                    SystemPeculiarity::Cataclysm(CataclysmSeverity::Extreme) => {
                        nothing_chances += 30;
                        conventional_chances += -25;
                        eccentric_chances += 5;
                        epistellar_chances += -5;
                    }
                    SystemPeculiarity::Cataclysm(CataclysmSeverity::Ultimate) => {
                        nothing_chances += 40;
                        conventional_chances += -30;
                        eccentric_chances += 0;
                        epistellar_chances += -10;
                    }
                    _ => (),
                }
            } else if discriminant(t)
                == discriminant(&SystemPeculiarity::Nebulae(NebulaeApparentSize::Small))
            {
                nothing_chances += 5;
            } else if discriminant(t)
                == discriminant(&SystemPeculiarity::UnusualDebrisDensity(
                    DebrisDensity::Lower,
                ))
            {
                match t {
                    SystemPeculiarity::UnusualDebrisDensity(DebrisDensity::MuchLower) => {
                        nothing_chances += 20;
                        conventional_chances += -10;
                        eccentric_chances += -10;
                    }
                    SystemPeculiarity::UnusualDebrisDensity(DebrisDensity::Lower) => {
                        nothing_chances += 10;
                        conventional_chances += -5;
                        eccentric_chances += -5;
                    }
                    SystemPeculiarity::UnusualDebrisDensity(DebrisDensity::Higher) => {
                        nothing_chances += -20;
                        conventional_chances += 10;
                        eccentric_chances += 10;
                    }
                    SystemPeculiarity::UnusualDebrisDensity(DebrisDensity::MuchHigher) => {
                        nothing_chances += -40;
                        conventional_chances += 20;
                        eccentric_chances += 20;
                    }
                    _ => (),
                }
            }
        })
    }
    if discriminant(star_type) == discriminant(&StarSpectralType::M(0)) {
        nothing_chances += 19;
        conventional_chances += -10;
        eccentric_chances += -5;
        epistellar_chances += -4;
    } else if discriminant(star_type) == discriminant(&StarSpectralType::G(0)) {
        nothing_chances += -20;
        conventional_chances += 20;
    } else if discriminant(star_type) == discriminant(&StarSpectralType::A(0))
        || discriminant(star_type) == discriminant(&StarSpectralType::B(0))
        || discriminant(star_type) == discriminant(&StarSpectralType::O(0))
        || discriminant(star_type) == discriminant(&StarSpectralType::WR(0))
    {
        nothing_chances += 20;
        conventional_chances += 0;
        eccentric_chances += 20;
        epistellar_chances += -20;
    } else if discriminant(star_type) == discriminant(&StarSpectralType::XBH)
        || discriminant(star_type) == discriminant(&StarSpectralType::XNS)
    {
        nothing_chances += 37;
        conventional_chances += -25;
        eccentric_chances += -12;
        epistellar_chances += -100;
    } else if discriminant(star_type) == discriminant(&StarSpectralType::DA)
        || discriminant(star_type) == discriminant(&StarSpectralType::DB)
        || discriminant(star_type) == discriminant(&StarSpectralType::DC)
        || discriminant(star_type) == discriminant(&StarSpectralType::DO)
        || discriminant(star_type) == discriminant(&StarSpectralType::DZ)
        || discriminant(star_type) == discriminant(&StarSpectralType::DQ)
        || discriminant(star_type) == discriminant(&StarSpectralType::DX)
    {
        nothing_chances += 10;
        conventional_chances += 0;
        eccentric_chances += 20;
        epistellar_chances += -10;
    } else if discriminant(star_type) == discriminant(&StarSpectralType::Y(0))
        || discriminant(star_type) == discriminant(&StarSpectralType::T(0))
        || discriminant(star_type) == discriminant(&StarSpectralType::L(0))
    {
        conventional_chances += 10;
        epistellar_chances += -5;
    }
    if discriminant(star_population) == discriminant(&StellarEvolution::Paleodwarf) {
        nothing_chances += 45;
        conventional_chances += -28;
        eccentric_chances += -13;
        epistellar_chances += -8;
    } else if discriminant(star_population) == discriminant(&StellarEvolution::Subdwarf) {
        nothing_chances += 14;
        conventional_chances += -17;
        eccentric_chances += 10;
        epistellar_chances += -7;
    } else if discriminant(star_population) == discriminant(&StellarEvolution::Dwarf)
        || discriminant(star_population) == discriminant(&StellarEvolution::Superdwarf)
    {
        conventional_chances += 10;
    } else if discriminant(star_population) == discriminant(&StellarEvolution::Hyperdwarf) {
        nothing_chances += -35;
        conventional_chances += 20;
        eccentric_chances += 10;
        epistellar_chances += 5;
    }
    (
        nothing_chances,
        conventional_chances,
        eccentric_chances,
        epistellar_chances,
    )
}

fn generate_proto_gas_giant_position(
    arrangement: &GasGiantArrangement,
    star: &Star,
    rng: &mut SeededDiceRoller,
) -> Option<f64> {
    match arrangement {
        GasGiantArrangement::ConventionalGasGiant => {
            let snow_line = star
                .zones
                .iter()
                .find(|z| z.zone_type == ZoneType::OuterZone)?
                .start;
            Some(rng.roll(2, 6, -2) as f64 * 0.05 + 1.0 * snow_line)
        }
        GasGiantArrangement::EccentricGasGiant => {
            let snow_line = star
                .zones
                .iter()
                .find(|z| z.zone_type == ZoneType::OuterZone)?
                .start;
            Some(rng.roll(2, 6, 0) as f64 * 0.125 * snow_line)
        }
        GasGiantArrangement::EpistellarGasGiant => {
            let outside_of_star = star
                .zones
                .iter()
                .find(|z| z.zone_type == ZoneType::Corona)?
                .end;
            Some(rng.roll(3, 6, 0) as f64 * 0.1 + outside_of_star)
        }
        _ => None,
    }
}
