use super::StarSystem;
use crate::prelude::*;

impl StarSystem {
    /// Generates a brand new star system at the given coordinates
    pub fn generate(
        system_index: u16,
        coord: SpaceCoordinates,
        hex: &GalacticHex,
        sub_sector: &GalacticMapDivision,
        galaxy: &mut Galaxy,
    ) -> Self {
        let number_of_stars = generate_number_of_stars_in_system(system_index, coord, galaxy);
        let mut stars = generate_stars(number_of_stars, system_index, coord, hex, sub_sector, galaxy);

        let mut center_id: u32;
        let main_star_id: u32;
        let mut all_objects: Vec<OrbitalPoint> = vec![];
        if stars.len() > 1 {
            let result = generate_binary_relations(&mut stars, &mut all_objects, system_index, coord, galaxy);
            center_id = result.0;
            main_star_id = result.1;
        } else {
            let center = OrbitalPoint::new(
                0,
                None,
                None,
                vec![],
                AstronomicalObject::Star(stars.remove(0)),
            );
            center_id = 0;
            main_star_id = 0;
            all_objects.push(center);
        }

        Self::new(all_objects, center_id, main_star_id)
    }
}

fn generate_number_of_stars_in_system(
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> u16 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.seed,
        &format!("sys_{}_{}_ste_evo", coord, system_index),
    );
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(1, 100),
            CopyableWeightedResult::new(2, 70),
            CopyableWeightedResult::new(3, 30),
            CopyableWeightedResult::new(4, 8),
            CopyableWeightedResult::new(5, 5),
            CopyableWeightedResult::new(6, 3),
            CopyableWeightedResult::new(7, 1),
        ],
        RollMethod::SimpleRoll,
    ))
    .unwrap()
}

fn generate_stars(number_of_stars: u16, system_index: u16, coord: SpaceCoordinates, hex: &GalacticHex, sub_sector: &GalacticMapDivision, galaxy: &mut Galaxy) -> Vec<Star> {
    let mut stars = Vec::new();
    for star_index in 0..number_of_stars {
        let evolution = generate_stellar_evolution(
            star_index,
            system_index,
            coord,
            hex,
            sub_sector,
            galaxy,
        );

        stars.push(Star::generate(
            star_index,
            system_index,
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
    let mut subsector_rng =
        SeededDiceRoller::new(&galaxy.seed, &format!("sys_{}_ste_evo", sub_sector.index));
    let mut hex_rng = SeededDiceRoller::new(&galaxy.seed, &format!("sys_{}_ste_evo", hex.index));
    let mut rng = SeededDiceRoller::new(
        &galaxy.seed,
        &format!("sys_{}_{}_ste_evo", coord, system_index),
    );
    let mut coord_rng = SeededDiceRoller::new(&galaxy.seed, &format!("sys_{}_ste_evo", star_index));

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

fn generate_binary_relations(stars: &mut Vec<Star>, all_objects: &mut Vec<OrbitalPoint>, system_index: u16, coord: SpaceCoordinates, galaxy: &mut Galaxy) -> (u32, u32) {
    let mut center_id;
    let main_star_id;
    let biggest_mass_in_vec = stars
        .iter()
        .map(|star| star.mass)
        .max_by(|a, b| {
            a.partial_cmp(b)
                .expect("There should be at least two stars to compare.")
        })
        .expect("There should be at least one star with some mass.");
    let star_index = stars
        .iter()
        .position(|star| star.mass == biggest_mass_in_vec)
        .expect("I should be able to find the index of a star.");

    let most_massive = stars.remove(star_index);
    let most_massive_mass = most_massive.mass;
    let most_massive_radius = most_massive.radius;
    let mut most_massive_point = OrbitalPoint::new(
        1,
        None,
        None,
        vec![],
        AstronomicalObject::Star(most_massive),
    );
    main_star_id = most_massive_point.id;

    let first_binary = stars.remove(0);
    let first_binary_mass = first_binary.mass;
    let first_binary_radius = first_binary.radius;
    let mut first_binary_point = OrbitalPoint::new(
        2,
        None,
        None,
        vec![],
        AstronomicalObject::Star(first_binary),
    );

    while stars.len() > 0 {
        if stars.len() == 1 {

        } else if stars.len() == 3 {

        } else {

        }
    }

    let result = make_binary_pair(
        &mut most_massive_point,
        most_massive_mass,
        most_massive_radius,
        &mut first_binary_point,
        first_binary_mass,
        first_binary_radius,
        0,
        star_index as u16,
        system_index,
        coord,
        galaxy,
    );

    let center = result.0;
    center_id = center.id;

    all_objects.push(center);
    all_objects.push(most_massive_point);
    all_objects.push(first_binary_point);

    (center_id, main_star_id)
}

/// Organizes two elements (either stars or binary pairs) into a binary system, updates them, and returns said binary system's barycentre
/// (an [OrbitalPoint], res.0), their mass ([f32], res.1) and radius ([f32], res.2) to use in further calculations.
fn make_binary_pair(
    most_massive_point: &mut OrbitalPoint,
    most_massive_mass: f32,
    most_massive_radius: f32,
    less_massive_point: &mut OrbitalPoint,
    less_massive_mass: f32,
    less_massive_radius: f32,
    next_id: u32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> (OrbitalPoint, f32, f32) {
    let mut center = OrbitalPoint::new(next_id, None, None, vec![], AstronomicalObject::Void);

    let min_distance =
        calculate_roche_limit(most_massive_radius, most_massive_mass, less_massive_mass);
    let actual_distance = generate_distance_between_stars(
        star_index as u16,
        system_index,
        min_distance,
        0,
        coord,
        galaxy,
    );
    let barycentre_distance_from_most_massive =
        calculate_barycentre(actual_distance, most_massive_mass, less_massive_mass);

    center.satellites.push(most_massive_point.id);
    center.satellites.push(less_massive_point.id);

    let most_massive_distance_and_radius =
        most_massive_radius as f64 + barycentre_distance_from_most_massive;
    let less_massive_distance_and_radius =
        less_massive_radius as f64 + actual_distance - barycentre_distance_from_most_massive;
    let radius = if most_massive_distance_and_radius > less_massive_distance_and_radius {
        most_massive_distance_and_radius as f32
    } else {
        less_massive_distance_and_radius as f32
    };

    most_massive_point.primary_body = Some(next_id);
    most_massive_point.distance_from_primary = Some(barycentre_distance_from_most_massive);
    less_massive_point.primary_body = Some(next_id);
    less_massive_point.distance_from_primary = Some(actual_distance - barycentre_distance_from_most_massive);

    (
        center,
        most_massive_mass + less_massive_mass,
        radius,
    )
}

/// Calculates the Roche limit, which is the minimum distance there can be between two stars for them to have a stable binary relation.
/// The radius of the heaviest star is in solar radii, but the return of the function is in AU.
fn calculate_roche_limit(
    radius_heaviest_star: f32,
    mass_heaviest_star: f32,
    mass_lighter_star: f32,
) -> f64 {
    ConversionUtils::solar_radii_to_astronomical_units(radius_heaviest_star as f64)
        * (2.0 * mass_heaviest_star as f64 / mass_lighter_star as f64).powf(1.0 / 3.0)
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
        &galaxy.seed,
        &format!("star_{}_{}_{}_mass", coord, system_index, star_index),
    );
    let range = rng
        .get_result(&CopyableRollToProcess::new(
            vec![
                // Very close
                CopyableWeightedResult {
                    result: (0.0, 0.05),
                    weight: 3,
                },
                // Close
                CopyableWeightedResult {
                    result: (0.051, 0.5),
                    weight: 3,
                },
                // Moderate
                CopyableWeightedResult {
                    result: (0.501, 2.0),
                    weight: 3,
                },
                // Wide
                CopyableWeightedResult {
                    result: (2.001, 10.0),
                    weight: 2,
                },
                // Distant
                CopyableWeightedResult {
                    result: (10.001, 50.0),
                    weight: 3,
                },
            ],
            RollMethod::PreparedRoll(PreparedRoll::new(3, 6, modifier)),
        ))
        .expect("Should return a range to generate a the distance between two stars.");
    let generated = rng.gen_f64() % (range.1 - range.0) + range.0;
    if generated < min_distance {
        min_distance
    } else {
        generated
    }
}

/// Finds where the barycentre between two stars or centers of mass is.
fn calculate_barycentre(distance_between: f64, heaviest_mass: f32, lowest_mass: f32) -> f64 {
    distance_between * (lowest_mass as f64 / (heaviest_mass as f64 + lowest_mass as f64))
}
