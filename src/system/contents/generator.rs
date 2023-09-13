use crate::prelude::AstronomicalObject::TelluricBody;
use crate::prelude::*;
use crate::system::contents::get_next_id;
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

    let mut number_of_bodies_per_star =
        collect_number_of_bodies_per_star(all_objects, &system_index, &coord, galaxy);

    let mut new_objects = Vec::new();
    number_of_bodies_per_star
        .iter_mut()
        .for_each(|(major_bodies_left, star_index)| {
            generate_orbits_and_bodies(
                all_objects,
                system_traits,
                system_index,
                coord,
                galaxy,
                seed.clone(),
                &all_zones,
                &mut new_objects,
                major_bodies_left,
                star_index,
            );
        });

    all_objects.extend(new_objects);
}

fn collect_number_of_bodies_per_star(
    all_objects: &mut Vec<OrbitalPoint>,
    system_index: &u16,
    coord: &SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> Vec<(i32, usize)> {
    all_objects
        .iter_mut()
        .enumerate()
        .filter_map(|(index, o)| {
            if let AstronomicalObject::Star(ref mut star) = o.object {
                Some((
                    generate_number_of_bodies(system_index, coord, galaxy, star),
                    index,
                ))
            } else {
                None
            }
        })
        .collect()
}

fn generate_orbits_and_bodies(
    all_objects: &mut Vec<OrbitalPoint>,
    system_traits: &Vec<SystemPeculiarity>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
    seed: Rc<str>,
    all_zones: &Vec<StarZone>,
    mut new_objects: &mut Vec<OrbitalPoint>,
    major_bodies_left: &mut i32,
    star_index: &mut usize,
) {
    let mut next_id = get_next_id(&all_objects);
    let initial_number_of_bodies = major_bodies_left.clone();
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

        let mut reference_orbit_radius =
            generate_reference_orbit_radius(system_index, coord, galaxy, major_bodies_left, &star);

        if let Some(orbit_radius) = generate_proto_gas_giant_position(
            &gas_giant_arrangement,
            star,
            system_index,
            coord,
            galaxy,
        ) {
            let (new_next_id, orbit_radius, orbit, point) = handle_proto_gas_giant_placement(
                &all_zones,
                system_traits,
                system_index,
                coord,
                &galaxy,
                seed.clone(),
                major_bodies_left,
                next_id,
                star_orbital_point.id,
                star.orbit.clone(),
                gas_giant_arrangement,
                reference_orbit_radius,
                orbit_radius,
            );
            next_id = new_next_id;
            reference_orbit_radius = orbit_radius;

            if let Some(o) = orbit {
                star_orbital_point.orbits.push(o);
            }
            if let Some(p) = point {
                new_objects.push(p);
            }
        }

        // Generate all possible orbits
        let mut new_orbits = generate_orbits(
            all_zones,
            system_index,
            coord,
            galaxy,
            &star_orbital_point,
            star,
            &mut reference_orbit_radius,
        );
        star_orbital_point.orbits.append(&mut new_orbits);

        // Sort all zones by increasing distance
        star_orbital_point
            .orbits
            .sort_by(|a, b| a.average_distance.partial_cmp(&b.average_distance).unwrap());

        let spawn_chances = if star_orbital_point.orbits.len() == 0 || initial_number_of_bodies == 0
        {
            0
        } else {
            (initial_number_of_bodies as f32 / star_orbital_point.orbits.len() as f32 * 100.0)
                as u32
        };
        let outwards_orbits_with_gas_giants_data =
            get_outwards_orbits_with_gas_giants(new_objects, star_orbital_point);
        let mut orbit_contents: Vec<(usize, f64, Option<u32>)> = vec![];
        generate_gas_giants(
            system_traits,
            system_index,
            coord,
            galaxy,
            seed,
            new_objects,
            major_bodies_left,
            next_id,
            star_orbital_point,
            gas_giant_arrangement,
            spawn_chances,
            outwards_orbits_with_gas_giants_data,
            orbit_contents,
        );
    }
}

fn get_outwards_orbits_with_gas_giants(
    mut new_objects: &mut Vec<OrbitalPoint>,
    star_orbital_point: &mut OrbitalPoint,
) -> Vec<(usize, f64)> {
    let outwards_orbits_with_gas_giants_data: Vec<(usize, f64)> = star_orbital_point
        .orbits
        .iter()
        .enumerate()
        .filter_map(|(index, orbit)| {
            if new_objects.iter().any(|object| {
                object.id == *orbit.satellite_ids.first().unwrap_or(&0)
                    && matches!(object.object, AstronomicalObject::GaseousBody(_))
            }) {
                Some((index, orbit.average_distance))
            } else {
                None
            }
        })
        .collect();
    outwards_orbits_with_gas_giants_data
}

fn generate_reference_orbit_radius(
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
    major_bodies_left: &mut i32,
    star: &Star,
) -> f64 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_loc",
            coord, system_index, star.orbital_point_id, major_bodies_left
        ),
    );
    let mut reference_orbit_radius = star
        .zones
        .iter()
        .filter(|&o| o.zone_type != ZoneType::ForbiddenZone)
        .max_by(|a, b| a.end.partial_cmp(&b.end).unwrap())
        .unwrap_or(&StarZone::new(0.0, 0.0, ZoneType::ForbiddenZone))
        .end
        / (rng.roll(1, 6, 0) as f64 * 0.05 + 1.0);
    reference_orbit_radius
}

fn generate_orbits(
    all_zones: &Vec<StarZone>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
    star_orbital_point: &OrbitalPoint,
    star: &Star,
    reference_orbit_radius: &mut f64,
) -> Vec<Orbit> {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!(
            "sys_{}_{}_str_{}_orbt_loc",
            coord, system_index, star.orbital_point_id
        ),
    );
    let mut orbits = vec![];
    let mut last_orbit = &mut *reference_orbit_radius;
    let star_id = star_orbital_point.id;
    let star_orbit = star.orbit.clone().unwrap_or_default();
    generate_inner_orbits(
        all_zones,
        &mut orbits,
        star_id,
        &star_orbit,
        &mut rng,
        &mut last_orbit,
    );
    last_orbit = &mut *reference_orbit_radius;
    generate_outer_orbits(
        all_zones,
        &mut orbits,
        star_id,
        &star_orbit,
        &mut rng,
        &mut last_orbit,
    );
    orbits
}

fn generate_inner_orbits(
    all_zones: &Vec<StarZone>,
    orbits: &mut Vec<Orbit>,
    star_id: u32,
    star_orbit: &Orbit,
    rng: &mut SeededDiceRoller,
    last_orbit: &mut f64,
) {
    let mut inner_orbits_done = false;
    while !inner_orbits_done {
        let multiplier = rng
            .get_result(&CopyableRollToProcess::new(
                vec![
                    CopyableWeightedResult::new(1.4, 1),
                    CopyableWeightedResult::new(1.5, 7),
                    CopyableWeightedResult::new(1.6, 16),
                    CopyableWeightedResult::new(1.7, 48),
                    CopyableWeightedResult::new(1.8, 16),
                    CopyableWeightedResult::new(1.9, 7),
                    CopyableWeightedResult::new(2.0, 1),
                ],
                RollMethod::SimpleRoll,
            ))
            .expect("A multiplier should have been picked.");
        let mut next_orbit = *last_orbit / multiplier;
        if *last_orbit - next_orbit < 0.15 {
            next_orbit = *last_orbit - 0.15 + rng.roll(1, 301, -151) as f64 / 10000.0;
        }
        if next_orbit <= 0.0 {
            inner_orbits_done = true;
            break;
        }
        *last_orbit = next_orbit;

        let next_orbit_from_center = next_orbit + star_orbit.average_distance_from_system_center;
        if let Some(zone) = all_zones
            .iter()
            .find(|o| next_orbit_from_center >= o.start && next_orbit_from_center <= o.end)
        {
            match zone.zone_type {
                ZoneType::InnerZone | ZoneType::BioZone | ZoneType::OuterZone => {
                    let orbit = Orbit::new(
                        star_id,
                        vec![],
                        zone.zone_type,
                        next_orbit,
                        next_orbit_from_center,
                        0.0,
                        0.0,
                    );
                    orbits.push(orbit);
                }
                ZoneType::ForbiddenZone => (),
                _ => {
                    inner_orbits_done = true;
                }
            }
        } else {
            inner_orbits_done = true;
        }
    }
}

fn generate_outer_orbits(
    all_zones: &Vec<StarZone>,
    orbits: &mut Vec<Orbit>,
    star_id: u32,
    star_orbit: &Orbit,
    rng: &mut SeededDiceRoller,
    last_orbit: &mut f64,
) {
    let mut outer_orbits_done = false;
    while !outer_orbits_done {
        let multiplier = rng
            .get_result(&CopyableRollToProcess::new(
                vec![
                    CopyableWeightedResult::new(1.4, 1),
                    CopyableWeightedResult::new(1.5, 7),
                    CopyableWeightedResult::new(1.6, 16),
                    CopyableWeightedResult::new(1.7, 48),
                    CopyableWeightedResult::new(1.8, 16),
                    CopyableWeightedResult::new(1.9, 7),
                    CopyableWeightedResult::new(2.0, 1),
                ],
                RollMethod::SimpleRoll,
            ))
            .expect("A multiplier should have been picked.");
        let next_orbit = *last_orbit * multiplier;
        *last_orbit = next_orbit;

        let next_orbit_from_center = next_orbit + star_orbit.average_distance_from_system_center;
        if let Some(zone) = all_zones
            .iter()
            .find(|o| next_orbit_from_center >= o.start && next_orbit_from_center <= o.end)
        {
            match zone.zone_type {
                ZoneType::InnerZone | ZoneType::BioZone | ZoneType::OuterZone => {
                    let orbit = Orbit::new(
                        star_id,
                        vec![],
                        zone.zone_type,
                        next_orbit,
                        next_orbit_from_center,
                        0.0,
                        0.0,
                    );
                    orbits.push(orbit);
                }
                ZoneType::ForbiddenZone => (),
                _ => {
                    outer_orbits_done = true;
                }
            }
        } else {
            outer_orbits_done = true;
        }
    }
}

fn generate_gas_giants(
    system_traits: &Vec<SystemPeculiarity>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
    seed: Rc<str>,
    mut new_objects: &mut Vec<OrbitalPoint>,
    major_bodies_left: &mut i32,
    mut next_id: u32,
    star_orbital_point: &mut OrbitalPoint,
    gas_giant_arrangement: GasGiantArrangement,
    spawn_chances: u32,
    outwards_orbits_with_gas_giants_data: Vec<(usize, f64)>,
    mut orbit_contents: Vec<(usize, f64, Option<u32>)>,
) {
    star_orbital_point
        .orbits
        .iter_mut()
        .enumerate()
        .for_each(|(index, orbit)| {
            if orbit.satellite_ids.is_empty() {
                let mut rng = SeededDiceRoller::new(
                    &galaxy.settings.seed,
                    &format!(
                        "sys_{}_{}_str_{}_bdy{}_gen",
                        coord, system_index, star_orbital_point.id, major_bodies_left
                    ),
                );

                let inwards_gas_giant = orbit_contents.iter().rev().find(|&&(gi, _, _)| gi < index);
                let outwards_gas_giant = outwards_orbits_with_gas_giants_data
                    .iter()
                    .find(|&&(gi, _)| gi > index);
                let gas_giant_au_inwards_proximity =
                    inwards_gas_giant.map(|&(_, dist, _)| orbit.average_distance - dist);
                let gas_giant_au_outwards_proximity =
                    outwards_gas_giant.map(|&(_, distance)| distance - orbit.average_distance);
                let gas_giant_orbits_inwards_proximity =
                    inwards_gas_giant.map(|&(gi, _, _)| index - gi);
                let gas_giant_orbits_outwards_proximity =
                    outwards_gas_giant.map(|&(gi, _)| gi - index);

                trace!(
                    "Looking at orbit n°{}, distance: {}au",
                    index,
                    orbit.average_distance
                );
                if inwards_gas_giant.is_none()
                    || gas_giant_au_inwards_proximity.is_none()
                    || gas_giant_au_inwards_proximity.unwrap() >= 0.5
                {
                    let should_spawn =
                        should_spawn(&mut rng, spawn_chances) && major_bodies_left > &mut 0;
                    if should_spawn {
                        trace!(
                            "Should spawn a {} of {} bodies left in {}",
                            gas_giant_arrangement,
                            major_bodies_left,
                            orbit.zone
                        );
                        match gas_giant_arrangement {
                            GasGiantArrangement::NoGasGiant => (),
                            GasGiantArrangement::ConventionalGasGiant => {
                                if orbit.zone == ZoneType::OuterZone {
                                    next_id += 1;
                                    orbit.satellite_ids.push(next_id);
                                    let body_type = generate_outer_body_type(&mut rng);
                                    let body_orbital_point = generate_new_body(
                                        system_traits,
                                        system_index,
                                        coord,
                                        galaxy,
                                        seed.clone(),
                                        next_id,
                                        orbit,
                                        body_type,
                                    );
                                    new_objects.push(body_orbital_point);

                                    trace!(
                                        "{} - {} - Generate a {:?}",
                                        gas_giant_arrangement,
                                        orbit.zone,
                                        body_type
                                    );

                                    // Book-keeping
                                    *major_bodies_left -= 1;
                                }
                            }
                            GasGiantArrangement::EpistellarGasGiant
                            | GasGiantArrangement::EccentricGasGiant => {
                                match orbit.zone {
                                    ZoneType::InnerZone | ZoneType::BioZone => {
                                        if orbit.zone == ZoneType::OuterZone {
                                            next_id += 1;
                                            orbit.satellite_ids.push(next_id);
                                            let body_type = generate_outer_body_type(&mut rng);
                                            let body_orbital_point = generate_new_body(
                                                system_traits,
                                                system_index,
                                                coord,
                                                galaxy,
                                                seed.clone(),
                                                next_id,
                                                orbit,
                                                body_type,
                                            );
                                            new_objects.push(body_orbital_point);

                                            trace!(
                                                "{} - {} - Generate a {:?}",
                                                gas_giant_arrangement,
                                                orbit.zone,
                                                body_type
                                            );

                                            // Book-keeping
                                            *major_bodies_left -= 1;
                                            if body_type == CelestialBodySubtype::Gaseous {
                                                *major_bodies_left -= 1;
                                            }
                                        }
                                    }
                                    ZoneType::OuterZone => {
                                        if orbit.zone == ZoneType::OuterZone {
                                            next_id += 1;
                                            orbit.satellite_ids.push(next_id);
                                            let body_type = generate_outer_body_type(&mut rng);
                                            let body_orbital_point = generate_new_body(
                                                system_traits,
                                                system_index,
                                                coord,
                                                galaxy,
                                                seed.clone(),
                                                next_id,
                                                orbit,
                                                body_type,
                                            );
                                            new_objects.push(body_orbital_point);

                                            trace!(
                                                "{} - {} - Generate a {:?}",
                                                gas_giant_arrangement,
                                                orbit.zone,
                                                body_type
                                            );

                                            // Book-keeping
                                            *major_bodies_left -= 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                } else {
                    trace!("Skipping this orbit.")
                }
            }
            if let Some(id) = orbit.satellite_ids.first() {
                orbit_contents.push((
                    index,
                    orbit.average_distance,
                    new_objects.iter().find(|o| o.id == *id).map(|o| o.id),
                ));
            } else {
                orbit_contents.push((index, orbit.average_distance, None));
            }
        });
}

fn generate_new_body(
    system_traits: &Vec<SystemPeculiarity>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
    seed: Rc<str>,
    next_id: u32,
    orbit: &mut Orbit,
    body_type: CelestialBodySubtype,
) -> OrbitalPoint {
    let orbital_point;
    let body;
    match body_type {
        CelestialBodySubtype::Metallic => {
            body = TelluricDetails::generate_rocky_body_stub(
                next_id,
                system_traits,
                system_index,
                coord,
                seed.clone(),
                galaxy.settings.clone(),
            );

            orbital_point = OrbitalPoint::new(
                next_id,
                Some(orbit.clone()),
                AstronomicalObject::GaseousBody(body),
                vec![],
            );
        }
        CelestialBodySubtype::Rocky => {
            body = TelluricDetails::generate_rocky_body_stub(
                next_id,
                system_traits,
                system_index,
                coord,
                seed.clone(),
                galaxy.settings.clone(),
            );

            orbital_point = OrbitalPoint::new(
                next_id,
                Some(orbit.clone()),
                AstronomicalObject::GaseousBody(body),
                vec![],
            );
        }
        CelestialBodySubtype::Gaseous => {
            body = GaseousDetails::generate_gas_giant(
                next_id,
                system_traits,
                system_index,
                coord,
                seed.clone(),
                galaxy.settings.clone(),
            );

            orbital_point = OrbitalPoint::new(
                next_id,
                Some(orbit.clone()),
                AstronomicalObject::GaseousBody(body),
                vec![],
            );
        }
        CelestialBodySubtype::Icy => {
            body = IcyDetails::generate_icy_body_stub(
                next_id,
                system_traits,
                system_index,
                coord,
                seed.clone(),
                galaxy.settings.clone(),
            );

            orbital_point = OrbitalPoint::new(
                next_id,
                Some(orbit.clone()),
                AstronomicalObject::GaseousBody(body),
                vec![],
            );
        }
    }
    orbital_point
}

fn handle_proto_gas_giant_placement(
    all_zones: &Vec<StarZone>,
    system_traits: &Vec<SystemPeculiarity>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &&mut Galaxy,
    seed: Rc<str>,
    major_bodies_left: &mut i32,
    mut next_id: u32,
    star_id: u32,
    star_orbit: Option<Orbit>,
    gas_giant_arrangement: GasGiantArrangement,
    mut reference_orbit_radius: f64,
    orbit_radius: f64,
) -> (u32, f64, Option<Orbit>, Option<OrbitalPoint>) {
    let orbit_from_center = orbit_radius
        + star_orbit
            .unwrap_or_default()
            .average_distance_from_system_center;
    if let Some(zone) = all_zones
        .iter()
        .find(|zone| orbit_from_center >= zone.start && orbit_from_center <= zone.end)
    {
        if zone.zone_type != ZoneType::ForbiddenZone {
            reference_orbit_radius = orbit_radius;
            // Create an Orbit
            let mut orbit = Orbit::new(
                star_id,
                vec![next_id],
                zone.zone_type,
                orbit_radius,
                orbit_from_center,
                0.0,
                0.0,
            );

            // Generate Gas Giant Settings
            let settings = &galaxy.settings;
            let celestial_body_settings = &galaxy.settings.celestial_body;
            let gaseous_body_settings = celestial_body_settings.gaseous_body_settings.clone();
            let mut fixed_special_traits = gaseous_body_settings
                .fixed_special_traits
                .unwrap_or_else(Vec::new);
            if !fixed_special_traits.contains(&GasGiantSpecialTrait::ProtoGiant) {
                fixed_special_traits.push(GasGiantSpecialTrait::ProtoGiant);
            }
            let gaseous_body_settings = GaseousBodySettings {
                fixed_special_traits: Some(fixed_special_traits),
                ..gaseous_body_settings
            };
            let celestial_body_settings = CelestialBodySettings {
                gaseous_body_settings,
                ..celestial_body_settings.clone()
            };
            let settings = GenerationSettings {
                celestial_body: celestial_body_settings,
                ..settings.clone()
            };

            // Generate the Gas Giant
            let gas_giant = GaseousDetails::generate_gas_giant(
                next_id,
                system_traits,
                system_index,
                coord,
                seed.clone(),
                settings,
            );

            // Create an Orbital Point for the Gas Giant
            let object_orbital_point = OrbitalPoint::new(
                next_id,
                Some(orbit.clone()),
                AstronomicalObject::GaseousBody(gas_giant),
                vec![],
            );

            // Book-keeping
            *major_bodies_left -= 1;
            if gas_giant_arrangement == GasGiantArrangement::EpistellarGasGiant {
                *major_bodies_left -= 1;
            }

            return (
                next_id,
                reference_orbit_radius,
                Some(orbit),
                Some(object_orbital_point),
            );
        }
    }
    (next_id, reference_orbit_radius, None, None)
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
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> Option<f64> {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!(
            "sys_{}_{}_str_{}_gas_pos",
            coord, system_index, star.orbital_point_id
        ),
    );

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

fn should_spawn(mut rng: &mut SeededDiceRoller, spawn_chances: u32) -> bool {
    let mut spawn_chances = if spawn_chances > 100 {
        100
    } else if spawn_chances < 0 {
        0
    } else {
        spawn_chances
    };
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(false, 100 - spawn_chances),
            CopyableWeightedResult::new(true, spawn_chances),
        ],
        RollMethod::SimpleRoll,
    ))
    .expect("A boolean result should have been picked.")
}

fn generate_inner_body_type(mut rng: &mut SeededDiceRoller) -> CelestialBodySubtype {
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(CelestialBodySubtype::Metallic, 3),
            CopyableWeightedResult::new(CelestialBodySubtype::Rocky, 5),
            CopyableWeightedResult::new(CelestialBodySubtype::Icy, 2),
            CopyableWeightedResult::new(CelestialBodySubtype::Gaseous, 1),
        ],
        RollMethod::SimpleRoll,
    ))
    .expect("A body type should have been picked.")
}

fn generate_outer_body_type(mut rng: &mut SeededDiceRoller) -> CelestialBodySubtype {
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(CelestialBodySubtype::Metallic, 1),
            CopyableWeightedResult::new(CelestialBodySubtype::Rocky, 3),
            CopyableWeightedResult::new(CelestialBodySubtype::Icy, 6),
            CopyableWeightedResult::new(CelestialBodySubtype::Gaseous, 6),
        ],
        RollMethod::SimpleRoll,
    ))
    .expect("A body type should have been picked.")
}
