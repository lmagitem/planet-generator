use super::StarSystem;
use crate::prelude::*;
#[path = "./constants.rs"]
mod constants;
use constants::*;

impl StarSystem {
    /// Generates a brand new star system at the given coordinates
    pub fn generate(
        system_index: u16,
        coord: SpaceCoordinates,
        hex: &GalacticHex,
        sub_sector: &GalacticMapDivision,
        galaxy: &mut Galaxy,
    ) -> Self {
        let center_id: u32;
        let main_star_id: u32;
        let mut all_objects: Vec<OrbitalPoint> = vec![];

        let name = get_system_name(system_index, coord, galaxy);

        let number_of_stars = generate_number_of_stars_in_system(system_index, coord, galaxy);
        let mut stars = generate_stars(
            number_of_stars,
            system_index,
            name.clone(),
            coord,
            hex,
            sub_sector,
            galaxy,
        );

        if stars.len() > 1 {
            let result = generate_binary_relations(
                &mut stars,
                &mut all_objects,
                system_index,
                coord,
                galaxy,
            );
            center_id = result.0;
            main_star_id = result.1;
        } else {
            let center =
                OrbitalPoint::new(0, None, AstronomicalObject::Star(stars.remove(0)), vec![]);
            center_id = 0;
            main_star_id = 0;
            all_objects.push(center);
        }

        // Assign star own orbit so they can work with it in zone calculations
        all_objects.iter_mut().for_each(|o| o.update_object_own_orbit());

        Self::new(name, center_id, main_star_id, all_objects)
    }
}

/// Temporary name generation
fn get_system_name(system_index: u16, coord: SpaceCoordinates, galaxy: &Galaxy) -> String {
    let settings = &galaxy.settings;
    if settings.star.use_ours {
        "Sol".to_string()
    } else {
        let mut rng = SeededDiceRoller::new(
            &galaxy.settings.seed,
            &format!("sys_{}_{}_ste_evo", coord, system_index),
        );
        let random_names = get_random_names();
        String::from(random_names[rng.gen_usize() % random_names.len()])
    }
}

fn generate_number_of_stars_in_system(
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> u16 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("sys_{}_{}_ste_evo", coord, system_index),
    );
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(1, 400),
            CopyableWeightedResult::new(2, 280),
            CopyableWeightedResult::new(3, 120),
            CopyableWeightedResult::new(4, 32),
            CopyableWeightedResult::new(5, 20),
            CopyableWeightedResult::new(6, 12),
            CopyableWeightedResult::new(7, 4),
            CopyableWeightedResult::new(8, 2),
            CopyableWeightedResult::new(9, 1),
        ],
        RollMethod::SimpleRoll,
    ))
    .unwrap()
}

fn generate_stars(
    number_of_stars: u16,
    system_index: u16,
    system_name: String,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    galaxy: &mut Galaxy,
) -> Vec<Star> {
    let mut stars = Vec::new();
    for star_index in 0..number_of_stars {
        let evolution =
            generate_stellar_evolution(star_index, system_index, coord, hex, sub_sector, galaxy);

        stars.push(Star::generate(
            star_index,
            system_index,
            system_name.clone(),
            coord,
            evolution,
            hex,
            galaxy,
            &galaxy.settings,
        ));
    }
    stars
}

/// Generates the Population of stars in a system.
fn generate_stellar_evolution(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    galaxy: &mut Galaxy,
) -> StellarEvolution {
    let mut subsector_rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("sys_{}_ste_evo", sub_sector.index),
    );
    let mut hex_rng =
        SeededDiceRoller::new(&galaxy.settings.seed, &format!("sys_{}_ste_evo", hex.index));
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("sys_{}_{}_ste_evo", coord, system_index),
    );
    let mut coord_rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("sys_{}_ste_evo", star_index),
    );

    let mut modifier = 0;
    match galaxy.neighborhood.universe.era {
        StelliferousEra::AncientStelliferous => modifier -= 10,
        StelliferousEra::EarlyStelliferous => modifier -= 5,
        StelliferousEra::LateStelliferous => modifier += 2,
        StelliferousEra::EndStelliferous => modifier += 5,
        _ => (),
    }
    modifier += if galaxy.is_dominant {
        2
    } else if !galaxy.is_major {
        -2
    } else {
        0
    };
    match galaxy.category {
        GalaxyCategory::Intergalactic(_, _, _) => modifier -= 10,
        _ => (),
    }
    match galaxy.sub_category {
        GalaxySubCategory::DwarfAmorphous
        | GalaxySubCategory::DwarfSpiral
        | GalaxySubCategory::DwarfElliptical
        | GalaxySubCategory::DwarfLenticular => modifier -= 2,
        GalaxySubCategory::GiantLenticular | GalaxySubCategory::GiantElliptical => modifier += 1,
        _ => (),
    }
    galaxy.special_traits.iter().for_each(|t| match t {
        GalaxySpecialTrait::MetalPoor => modifier -= 5,
        GalaxySpecialTrait::Younger => modifier -= 2,
        GalaxySpecialTrait::SubSize(_) => modifier -= 1,
        GalaxySpecialTrait::Dusty | GalaxySpecialTrait::SuperSize(_) => modifier += 1,
        GalaxySpecialTrait::Starburst => modifier += 2,
        _ => (),
    });
    let divisions = galaxy
        .get_divisions_for_coord(coord)
        .expect("Should have returned divisions.");
    let mut regions = Vec::new();
    divisions.iter().for_each(|div| {
        if regions.iter().find(|r| **r == div.region).is_none() {
            regions.push(div.region.clone());
        }
    });
    regions.iter().for_each(|region| match region {
        GalacticRegion::Nucleus => modifier += 2,
        GalacticRegion::Core | GalacticRegion::Bar | GalacticRegion::Arm => modifier += 1,
        GalacticRegion::Disk => modifier -= 1,
        GalacticRegion::Ellipse => modifier -= 2,
        GalacticRegion::Halo | GalacticRegion::Void | GalacticRegion::Stream => modifier -= 5,
        GalacticRegion::Aura => modifier -= 10,
        _ => (),
    });

    let roll = subsector_rng.roll(1, 4, -1)
        + hex_rng.roll(1, 3, -1)
        + coord_rng.roll(1, 3, -1)
        + rng.roll(1, 4, -1)
        + modifier;
    let result = if roll < -10 {
        StellarEvolution::PopulationIII
    } else if roll < 3 {
        StellarEvolution::PopulationII
    } else if roll < 21 {
        StellarEvolution::PopulationI
    } else {
        StellarEvolution::Population0
    };
    result
}

/// For a given list of stars, generates binary pairs and makes them dance together. Returns the id of the system's
/// center point of gravity (res.0), the id of the system's main star (res.1) and the last id used for an object (res.2).
fn generate_binary_relations(
    stars_left: &mut Vec<Star>,
    all_objects: &mut Vec<OrbitalPoint>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> (u32, u32, u32) {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("sys_{}_{}_bin_rel", coord, system_index),
    );
    let mut center_id = 0;
    let mut last_id = 0;
    let main_star_id = last_id;
    let number_of_stars = stars_left.len();

    // Extract the biggest star of the bunch
    let biggest_mass_in_vec = stars_left
        .iter()
        .map(|star| star.mass)
        .max_by(|a, b| {
            a.partial_cmp(b)
                .expect("There should be at least two stars to compare.")
        })
        .expect("There should be at least one star with some mass.");
    let star_index = stars_left
        .iter()
        .position(|star| star.mass == biggest_mass_in_vec)
        .expect("I should be able to find the index of a star.");
    let most_massive = stars_left.remove(star_index);

    // Use it as our first primary member in pairs
    let mut most_massive_mass = most_massive.mass;
    let mut most_massive_radius = most_massive.radius;
    let mut most_massive_point = OrbitalPoint::new(
        last_id,
        None,
        AstronomicalObject::Star(most_massive),
        vec![],
    );

    let mut first_turn = true;
    let mut previous_actual_distance = 0.005;
    // Then make binary pairs as long as you have stars
    while stars_left.len() > 0 {
        // If at least two stars left, with a random chance of 1 in 4 or more
        if stars_left.len() > 1
            && (rng.gen_u8() % 7 != 0
                || (!first_turn && number_of_stars % 2 == 0 && rng.gen_u8() % 5 != 0))
        {
            // Make a pair and have it dance with the biggest mass/last pair.
            last_id += 1;
            let first_of_pair = stars_left.remove(0);
            let first_of_pair_mass = first_of_pair.mass;
            let first_of_pair_radius = first_of_pair.radius;
            let first_of_pair_point = OrbitalPoint::new(
                last_id,
                None,
                AstronomicalObject::Star(first_of_pair),
                vec![],
            );
            last_id += 1;
            let second_of_pair = stars_left.remove(0);
            let second_of_pair_mass = second_of_pair.mass;
            let second_of_pair_radius = second_of_pair.radius;
            let second_of_pair_point = OrbitalPoint::new(
                last_id,
                None,
                AstronomicalObject::Star(second_of_pair),
                vec![],
            );

            // Generate our new pair
            let result = make_binary_pair(
                last_id,
                first_of_pair_mass,
                first_of_pair_radius,
                first_of_pair_point,
                second_of_pair_mass,
                second_of_pair_radius,
                second_of_pair_point,
                minimum_distance(first_of_pair_radius as f64, second_of_pair_radius as f64),
                star_index,
                system_index,
                coord,
                galaxy,
                all_objects,
            );
            last_id = result.0;
            let less_massive_point = result.1;
            let less_massive_mass = result.2;
            let less_massive_radius = result.3;

            // Use the newly generated pair as the less massive member to generate the next pair
            let result = make_binary_pair(
                last_id,
                most_massive_mass,
                most_massive_radius,
                most_massive_point,
                less_massive_mass,
                less_massive_radius,
                less_massive_point,
                previous_actual_distance,
                star_index,
                system_index,
                coord,
                galaxy,
                all_objects,
            );
            last_id = result.0;

            // Then update what values to use in the next turn
            most_massive_point = result.1;
            most_massive_mass = result.2;
            most_massive_radius = result.3;
            previous_actual_distance = result.4;
            center_id = most_massive_point.id;
        } else {
            // Take a single star and have it dance with the biggest mass/last pair.
            last_id += 1;
            let less_massive = stars_left.remove(0);
            let less_massive_mass = less_massive.mass;
            let less_massive_radius = less_massive.radius;
            let less_massive_point = OrbitalPoint::new(
                last_id,
                None,
                AstronomicalObject::Star(less_massive),
                vec![],
            );

            let result = make_binary_pair(
                last_id,
                most_massive_mass,
                most_massive_radius,
                most_massive_point,
                less_massive_mass,
                less_massive_radius,
                less_massive_point,
                previous_actual_distance,
                star_index,
                system_index,
                coord,
                galaxy,
                all_objects,
            );
            last_id = result.0;

            // Then update what values to use in the next turn
            most_massive_point = result.1;
            most_massive_mass = result.2;
            most_massive_radius = result.3;
            previous_actual_distance = result.4;
            center_id = most_massive_point.id;
        }
        first_turn = false;
    }
    // Finally, add the center point to the system's list of objects
    all_objects.push(most_massive_point);

    (center_id, main_star_id, last_id)
}

/// Returns the last id used ([u32], res.0), the [OrbitalPoint] at the center of the newly made binary pair (res.1), the mass ([f32], res.2)
/// and radius ([f32], res.3) of that pair, and the actual distance between the two elements ([f64], res.4) to use in further calculations.
fn make_binary_pair(
    mut last_id: u32,
    mut most_massive_mass: f32,
    mut most_massive_radius: f32,
    mut most_massive_point: OrbitalPoint,
    mut less_massive_mass: f32,
    mut less_massive_radius: f32,
    mut less_massive_point: OrbitalPoint,
    min_distance_between_bodies: f64,
    star_index: usize,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
    all_objects: &mut Vec<OrbitalPoint>,
) -> (u32, OrbitalPoint, f32, f32, f64) {
    // Switch stars if necessary so that most_massive_point is the most massive.
    if less_massive_mass > most_massive_mass {
        let temp_mass = less_massive_mass;
        let temp_radius = less_massive_radius;
        let temp_point = less_massive_point;
        less_massive_mass = most_massive_mass;
        less_massive_radius = most_massive_radius;
        less_massive_point = most_massive_point;
        most_massive_mass = temp_mass;
        most_massive_radius = temp_radius;
        most_massive_point = temp_point;
    }

    last_id += 1;
    let result = find_center_of_binary_pair(
        &mut most_massive_point,
        most_massive_mass,
        most_massive_radius,
        &mut less_massive_point,
        less_massive_mass,
        less_massive_radius,
        min_distance_between_bodies,
        last_id,
        star_index as u16,
        system_index,
        coord,
        galaxy,
    );
    all_objects.push(most_massive_point);
    all_objects.push(less_massive_point);

    (last_id, result.0, result.1, result.2, result.3)
}

/// Organizes two elements (either stars or binary pairs) into a binary system, updates them, and returns said binary system's barycentre
/// (an [OrbitalPoint], res.0), their mass ([f32], res.1) and radius ([f32], res.2), and the actual distance between the two elements
/// ([f64], res.3) to use in further calculations.
fn find_center_of_binary_pair(
    most_massive_point: &mut OrbitalPoint,
    most_massive_mass: f32,
    most_massive_radius: f32,
    less_massive_point: &mut OrbitalPoint,
    less_massive_mass: f32,
    less_massive_radius: f32,
    min_distance: f64,
    next_id: u32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> (OrbitalPoint, f32, f32, f64) {
    let mut center = OrbitalPoint::new(next_id, None, AstronomicalObject::Void, vec![]);

    let actual_distance =
        generate_distance_between_stars(star_index, system_index, min_distance, 0, coord, galaxy);
    let barycentre_distance_from_most_massive =
        calculate_barycentre(actual_distance, most_massive_mass, less_massive_mass);

    let most_massive_orbit = Orbit::new(
        next_id,
        vec![most_massive_point.id],
        barycentre_distance_from_most_massive,
        0.0,
    );
    let less_massive_orbit = Orbit::new(
        next_id,
        vec![less_massive_point.id],
        actual_distance - barycentre_distance_from_most_massive,
        0.0,
    );

    center.orbits.push(most_massive_orbit.clone());
    center.orbits.push(less_massive_orbit.clone());

    most_massive_point.set_own_orbit(most_massive_orbit);
    less_massive_point.set_own_orbit(less_massive_orbit);

    let most_massive_distance_and_radius =
        most_massive_radius as f64 + barycentre_distance_from_most_massive;
    let less_massive_distance_and_radius =
        less_massive_radius as f64 + actual_distance - barycentre_distance_from_most_massive;
    let radius = if most_massive_distance_and_radius > less_massive_distance_and_radius {
        most_massive_distance_and_radius as f32
    } else {
        less_massive_distance_and_radius as f32
    };

    (
        center,
        most_massive_mass + less_massive_mass,
        radius,
        most_massive_distance_and_radius + less_massive_distance_and_radius,
    )
}

/// Calculates the Roche limit, which is the minimum distance there can be between two objects for them to have a stable binary relation.
/// The radius of the heaviest star is in solar radii, but the return of the function is in AU.
fn calculate_roche_limit(
    radius_heaviest_object: f32,
    mass_heaviest_object: f32,
    mass_lighter_object: f32,
) -> f64 {
    ConversionUtils::solar_radii_to_astronomical_units(radius_heaviest_object as f64)
        * (2.0 * mass_heaviest_object as f64 / mass_lighter_object as f64).powf(1.0 / 3.0)
}

/// Calculates the minimum distance there can be between two stars.
/// The radius are in solar radii, but the return of the function is in AU.
fn minimum_distance(radius_first_star: f64, radius_second_star: f64) -> f64 {
    ConversionUtils::solar_radii_to_astronomical_units(radius_first_star)
        + ConversionUtils::solar_radii_to_astronomical_units(radius_second_star)
}

fn generate_distance_between_stars(
    star_index: u16,
    system_index: u16,
    min_distance: f64,
    modifier: i32,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> f64 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("star_{}_{}_{}_mass", coord, system_index, star_index),
    );
    let min_distance_multiplied = if min_distance < 0.5 {
        min_distance * 6000.0
    } else if min_distance < 2.5 {
        min_distance * 600.0
    } else if min_distance < 10.0 {
        min_distance * 60.0
    } else if min_distance < 25.0 {
        min_distance * 10.0
    } else {
        min_distance * 2.0
    };
    let range = rng
        .get_result(&CopyableRollToProcess::new(
            vec![
                // Very close
                CopyableWeightedResult {
                    result: (
                        if min_distance < 15.0 {
                            min_distance
                        } else {
                            min_distance_multiplied
                        },
                        min_distance_multiplied + 0.48,
                    ),
                    weight: 3,
                },
                // Close
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 0.48,
                        min_distance_multiplied + 6.0,
                    ),
                    weight: 3,
                },
                // Moderate
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 6.0,
                        min_distance_multiplied + 72.0,
                    ),
                    weight: 3,
                },
                // Wide
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 72.0,
                        min_distance_multiplied + 120.0,
                    ),
                    weight: 2,
                },
                // Distant
                CopyableWeightedResult {
                    result: (
                        min_distance_multiplied + 120.0,
                        min_distance_multiplied + 600.0,
                    ),
                    weight: 3,
                },
            ],
            RollMethod::PreparedRoll(PreparedRoll::new(3, 6, modifier)),
        ))
        .expect("Should return a range to generate a the distance between two stars.");
    let generated = rng.gen_f64() % (range.1 - range.0) + range.0;
    generated
}

/// Finds where the barycentre between two stars or centers of mass is.
fn calculate_barycentre(distance_between: f64, heaviest_mass: f32, lowest_mass: f32) -> f64 {
    distance_between * (lowest_mass as f64 / (heaviest_mass as f64 + lowest_mass as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn generate_interesting_example_systems() {
        let mut highest_distance = 0.0;
        for i in 0..10000 {
            let settings = &GenerationSettings {
                seed: String::from(&i.to_string()),
                ..Default::default()
            };
            let universe = Universe::generate(&settings);
            let neighborhood = GalacticNeighborhood::generate(universe, &settings);
            let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
            let coord = SpaceCoordinates::new(0, 0, 0);
            let sub_sector = galaxy
                .get_division_at_level(coord, 1)
                .expect("Should have returned a sub-sector.");
            let hex = galaxy.get_hex(coord).expect("Should have returned an hex.");
            let system = StarSystem::generate(0, coord, &hex, &sub_sector, &mut galaxy);
            // Find in objects the one with the highest distance from primary body.
            let higher_distance = system
                .all_objects
                .iter()
                .map(|o| {
                    o.get_own_orbit()
                        .unwrap_or(Orbit {
                            ..Default::default()
                        })
                        .average_distance
                })
                .max_by(|a, b| a.total_cmp(b))
                .unwrap();
            if
            /* higher_distance > highest_distance */
            system.center_id >= 13
                && (system
                    .all_objects
                    .iter()
                    .filter(|o| {
                        let mut result = false;
                        if let AstronomicalObject::Star(star) = &o.object {
                            match star.spectral_type {
                                StarSpectralType::WR(_)
                                | StarSpectralType::O(_)
                                | StarSpectralType::B(_)
                                | StarSpectralType::A(_)
                                | StarSpectralType::F(_)
                                | StarSpectralType::G(_)
                                | StarSpectralType::Y(_)
                                | StarSpectralType::DA
                                | StarSpectralType::DB
                                | StarSpectralType::DC
                                | StarSpectralType::DO
                                | StarSpectralType::DZ
                                | StarSpectralType::DQ
                                | StarSpectralType::DX
                                | StarSpectralType::XNS
                                | StarSpectralType::XBH => {
                                    result = true;
                                }
                                _ => (),
                            }
                            match star.luminosity_class {
                                StarLuminosityClass::O
                                | StarLuminosityClass::Ia
                                | StarLuminosityClass::Ib
                                | StarLuminosityClass::II
                                | StarLuminosityClass::III
                                | StarLuminosityClass::IV
                                | StarLuminosityClass::VII
                                | StarLuminosityClass::XNS
                                | StarLuminosityClass::XBH => {
                                    result = true;
                                }
                                _ => (),
                            }
                        }
                        result
                    })
                    .count()
                    > 4
                    || (system
                        .all_objects
                        .iter()
                        .filter(|o| {
                            let mut result = false;
                            if let AstronomicalObject::Star(star) = &o.object {
                                match star.spectral_type {
                                    StarSpectralType::WR(_)
                                    | StarSpectralType::O(_)
                                    | StarSpectralType::B(_)
                                    | StarSpectralType::A(_)
                                    | StarSpectralType::F(_)
                                    | StarSpectralType::G(_)
                                    | StarSpectralType::Y(_)
                                    | StarSpectralType::DA
                                    | StarSpectralType::DB
                                    | StarSpectralType::DC
                                    | StarSpectralType::DO
                                    | StarSpectralType::DZ
                                    | StarSpectralType::DQ
                                    | StarSpectralType::DX
                                    | StarSpectralType::XNS
                                    | StarSpectralType::XBH => {
                                        result = true;
                                    }
                                    _ => (),
                                }
                                match star.luminosity_class {
                                    StarLuminosityClass::O
                                    | StarLuminosityClass::Ia
                                    | StarLuminosityClass::Ib
                                    | StarLuminosityClass::II
                                    | StarLuminosityClass::III
                                    | StarLuminosityClass::IV
                                    | StarLuminosityClass::VII
                                    | StarLuminosityClass::XNS
                                    | StarLuminosityClass::XBH => {
                                        result = true;
                                    }
                                    _ => (),
                                }
                            }
                            result
                        })
                        .count()
                        > 1
                        && system
                            .all_objects
                            .iter()
                            .filter(|o| {
                                let mut result = false;
                                if let AstronomicalObject::Star(star) = &o.object {
                                    match star.spectral_type {
                                        StarSpectralType::WR(_)
                                        | StarSpectralType::O(_)
                                        | StarSpectralType::B(_)
                                        | StarSpectralType::A(_)
                                        | StarSpectralType::XNS
                                        | StarSpectralType::XBH => {
                                            result = true;
                                        }
                                        _ => (),
                                    }
                                }
                                result
                            })
                            .count()
                            > 0))
            {
                highest_distance = higher_distance;
                println!("\nseed: {}, distance: {}", settings.seed, highest_distance);
                println!("\n{:#?}", system);
            };
        }
    }
}
