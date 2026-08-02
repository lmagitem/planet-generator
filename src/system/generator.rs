use super::StarSystem;
use crate::diagnostics::types::GenerationError;
use crate::diagnostics::Diagnostics;
use crate::galaxy::map::division::GalacticMapDivision;
use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::hex::GalacticHex;
use crate::galaxy::map::types::GalacticRegion;
use crate::galaxy::types::GalaxyCategory;
use crate::galaxy::types::GalaxySpecialTrait;
use crate::galaxy::types::GalaxySubCategory;
use crate::galaxy::Galaxy;
use crate::system::celestial_body::types::CelestialBodyDetails;
use crate::system::celestial_body::world::types::CelestialBodyWorldType;
use crate::system::orbital_point::types::AstronomicalObject;
use crate::system::orbital_point::types::Orbit;
use crate::system::orbital_point::OrbitalPoint;
use crate::system::star::types::ZoneType;
use crate::system::star::Star;
use crate::system::types::StellarEvolution;
use crate::system::types::SystemGenContext;
use crate::system::types::SystemPeculiarity;
use crate::universe::types::StelliferousEra;
use crate::utils::conversion::ConversionUtils;
use seeded_dice_roller::{
    CopyableRollToProcess, CopyableWeightedResult, PreparedRoll, RollMethod, SeededDiceRoller,
};
use std::collections::HashSet;
use std::rc::Rc;

#[path = "./constants.rs"]
mod constants;
use crate::system::contents::generator::generate_stars_systems;
use crate::system::contents::zones::generate_star_zones;
use constants::get_random_names;

impl StarSystem {
    /// Generates a brand new star system at the given coordinates
    ///
    /// # Errors
    /// Returns [`GenerationError::InvalidCoordinates`] if the coordinates are outside the galaxy,
    /// or [`GenerationError::InterestingSystemNotFound`] if `only_interesting` is set and no
    /// interesting system was found within 5000 tries.
    ///
    /// The galaxy's settings are expected to be valid (see [`GenerationSettings::validate`]).
    pub fn generate(
        system_index: u16,
        coord: SpaceCoordinates,
        hex: &GalacticHex,
        sub_sector: &GalacticMapDivision,
        galaxy: &mut Galaxy,
    ) -> Result<Self, GenerationError> {
        let mut center_id: u32 = 0;
        let mut main_star_id: u32 = 0;
        let mut all_objects: Vec<OrbitalPoint> = Vec::new();
        // TODO: Did you add the special traits or is it just an empty array?
        let mut special_traits: Vec<SystemPeculiarity> = Vec::new();

        let name = get_system_name(system_index, coord, galaxy);

        let mut accept_system = false;
        let mut diagnostics = Diagnostics::default();
        let mut i = 0;
        while !accept_system {
            all_objects = Vec::new();
            special_traits = Vec::new();

            // `get_divisions_for_coord` is `&mut self` (lazy region-map rebuild +
            // division cache): it must run BEFORE `ctx` immutably borrows
            // `galaxy.settings`. Stays inside the loop (same per-try call
            // frequency as today's lookup inside `generate_stars`). The refs it
            // returns are cloned into an owned Vec for the same reason.
            let divisions = galaxy
                .get_divisions_for_coord(coord)?
                .into_iter()
                .cloned()
                .collect::<Vec<_>>();
            let ctx = SystemGenContext::new(
                i,
                &galaxy.settings,
                coord,
                system_index,
                &special_traits,
                hex,
                galaxy.neighborhood.universe,
            );

            let number_of_stars =
                generate_number_of_stars_in_system(i, system_index, coord, galaxy);
            let mut stars = generate_stars(
                &ctx,
                number_of_stars,
                name.clone(),
                sub_sector,
                &divisions,
                galaxy,
            );

            if stars.len() > 1 {
                let result = generate_binary_relations(
                    i,
                    &mut stars,
                    &mut all_objects,
                    system_index,
                    coord,
                    galaxy,
                );
                center_id = result.0;
                main_star_id = result.1;

                let mut calculated_ids = HashSet::new();
                for id in all_objects.iter().map(|op| op.id).collect::<Vec<u32>>() {
                    calculate_distance_from_system_center(
                        id,
                        &mut all_objects,
                        &mut calculated_ids,
                    );
                }
            } else {
                let center =
                    OrbitalPoint::new(0, None, AstronomicalObject::Star(stars.remove(0)), vec![]);
                center_id = 0;
                main_star_id = 0;
                all_objects.push(center);
            }

            // TODO: Generate dynamic parameters for star orbits
            update_existing_orbits(&mut all_objects);
            generate_star_zones(&mut all_objects);
            generate_stars_systems(&ctx, &mut all_objects);
            update_existing_orbits(&mut all_objects);

            accept_system = if galaxy.settings.system.only_interesting {
                let is_interesting = all_objects
                    .iter()
                    .find(|o| {
                        if let AstronomicalObject::TelluricBody(body) = &o.object {
                            if let CelestialBodyDetails::Telluric(details) = &body.details {
                                details.world_type == CelestialBodyWorldType::Terrestrial
                                    || details.world_type == CelestialBodyWorldType::Ocean
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .is_some();

                is_interesting
            } else {
                true
            };
            if accept_system {
                // The accepted try's diagnostics ride along with the returned
                // system; rejected tries' diagnostics are dropped (intended).
                diagnostics = ctx.diagnostics.into_inner();
            }
            i += 1;
            if i > 5000 {
                return Err(GenerationError::InterestingSystemNotFound { tries: 5000 });
            }
        }
        let mut system = Self::new(name, center_id, main_star_id, all_objects, special_traits);
        system.diagnostics = diagnostics;
        Ok(system)
    }
}

/// Temporary name generation
fn get_system_name(system_index: u16, coord: SpaceCoordinates, galaxy: &Galaxy) -> Rc<str> {
    let settings = &galaxy.settings;
    if settings.star.use_ours {
        "Sol".into()
    } else {
        let mut rng = SeededDiceRoller::new(
            &galaxy.settings.seed,
            &format!("sys_{}_{}_ste_evo", coord, system_index),
        );
        let random_names = get_random_names();
        (random_names[rng.gen_usize() % random_names.len()]).into()
    }
}

fn generate_number_of_stars_in_system(
    system_gen_try: u32,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> u16 {
    let mut rng = SeededDiceRoller::new(
        &*format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_{}_ste_evo", coord, system_index),
    );
    let table = CopyableRollToProcess::new(
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
    );
    // Unreachable fallback (hardcoded non-empty table): first entry.
    rng.get_result(&table)
        .unwrap_or(table.possible_results[0].result)
}

fn generate_stars(
    ctx: &SystemGenContext,
    number_of_stars: u16,
    system_name: Rc<str>,
    sub_sector: &GalacticMapDivision,
    divisions: &[GalacticMapDivision],
    galaxy: &Galaxy, // identity fields for generate_stellar_evolution
) -> Vec<Star> {
    let mut stars = Vec::new();
    for star_index in 0..number_of_stars {
        let evolution = generate_stellar_evolution(
            &ctx.seed,
            star_index,
            ctx.system_index,
            ctx.coord,
            ctx.hex,
            sub_sector,
            divisions,
            galaxy,
        );

        stars.push(Star::generate(
            ctx,
            star_index,
            system_name.clone(),
            evolution,
        ));
    }
    stars
}

/// Generates the Population of stars in a system.
#[allow(clippy::too_many_arguments)]
fn generate_stellar_evolution(
    seed: &str, // try-seed: `format!("{}{}", system_gen_try, settings.seed)`
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    sub_sector: &GalacticMapDivision,
    divisions: &[GalacticMapDivision],
    galaxy: &Galaxy,
) -> StellarEvolution {
    let mut subsector_rng =
        SeededDiceRoller::new(seed, &format!("sys_{}_ste_evo", sub_sector.index));
    let mut hex_rng = SeededDiceRoller::new(seed, &format!("sys_{}_ste_evo", hex.index));
    let mut rng = SeededDiceRoller::new(seed, &format!("sys_{}_{}_ste_evo", coord, system_index));
    let mut coord_rng = SeededDiceRoller::new(seed, &format!("sys_{}_ste_evo", star_index));

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
        StellarEvolution::Paleodwarf
    } else if roll < 3 {
        StellarEvolution::Subdwarf
    } else if roll < 10 {
        StellarEvolution::Dwarf
    } else if roll < 20 {
        StellarEvolution::Superdwarf
    } else {
        StellarEvolution::Hyperdwarf
    };
    result
}

/// For a given list of stars, generates binary pairs and makes them dance together. Returns the id of the system's
/// center point of gravity (res.0), the id of the system's main star (res.1) and the last id used for an object (res.2).
fn generate_binary_relations(
    system_gen_try: u32,
    stars_left: &mut Vec<Star>,
    all_objects: &mut Vec<OrbitalPoint>,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> (u32, u32, u32) {
    let mut rng = SeededDiceRoller::new(
        &*format!("{}{}", system_gen_try, &galaxy.settings.seed),
        &format!("sys_{}_{}_bin_rel", coord, system_index),
    );
    let mut center_id = 0;
    let mut last_id = 0;
    let main_star_id = last_id;
    let number_of_stars = stars_left.len();

    // Extract the biggest star of the bunch (`stars_left` is never empty here: the
    // number of stars rolled is at least 1; the fallbacks are unreachable).
    let biggest_mass_in_vec = stars_left
        .iter()
        .map(|star| star.mass)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);
    let star_index = stars_left
        .iter()
        .position(|star| star.mass == biggest_mass_in_vec)
        .unwrap_or(0);
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
                calculate_stars_minimum_distance(
                    first_of_pair_radius as f64,
                    second_of_pair_radius as f64,
                ),
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
    mut most_massive_mass: f64,
    mut most_massive_radius: f64,
    mut most_massive_point: OrbitalPoint,
    mut less_massive_mass: f64,
    mut less_massive_radius: f64,
    mut less_massive_point: OrbitalPoint,
    min_distance_between_bodies: f64,
    star_index: usize,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
    all_objects: &mut Vec<OrbitalPoint>,
) -> (u32, OrbitalPoint, f64, f64, f64) {
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
    most_massive_mass: f64,
    most_massive_radius: f64,
    less_massive_point: &mut OrbitalPoint,
    less_massive_mass: f64,
    less_massive_radius: f64,
    min_distance: f64,
    next_id: u32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> (OrbitalPoint, f64, f64, f64) {
    let mut center = OrbitalPoint::new(next_id, None, AstronomicalObject::Void, vec![]);

    let actual_distance =
        generate_distance_between_stars(star_index, system_index, min_distance, 0, coord, galaxy);
    let barycentre_distance_from_most_massive =
        calculate_barycentre(actual_distance, most_massive_mass, less_massive_mass);

    let most_massive_orbit = Orbit::new(
        next_id,
        Some(most_massive_point.id),
        ZoneType::ForbiddenZone,
        barycentre_distance_from_most_massive,
        0.0,
        0.0,
        barycentre_distance_from_most_massive,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        f32::INFINITY,
    );
    let less_massive_orbit = Orbit::new(
        next_id,
        Some(less_massive_point.id),
        ZoneType::ForbiddenZone,
        actual_distance - barycentre_distance_from_most_massive,
        0.0,
        0.0,
        actual_distance - barycentre_distance_from_most_massive,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        f32::INFINITY,
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
        most_massive_distance_and_radius
    } else {
        less_massive_distance_and_radius
    };

    (
        center,
        most_massive_mass + less_massive_mass,
        radius,
        most_massive_distance_and_radius + less_massive_distance_and_radius,
    )
}

/// Calculates the minimum distance there can be between two stars.
/// The radius are in solar radii, but the return of the function is in AU.
fn calculate_stars_minimum_distance(radius_first_star: f64, radius_second_star: f64) -> f64 {
    ConversionUtils::solar_radii_to_astronomical_units(radius_first_star)
        + ConversionUtils::solar_radii_to_astronomical_units(radius_second_star)
}

fn calculate_distance_from_system_center(
    orbital_point_id: u32,
    all_objects: &mut [OrbitalPoint],
    calculated_ids: &mut HashSet<u32>,
) {
    // Check if the distance for this orbital point has already been calculated
    if !calculated_ids.insert(orbital_point_id) {
        return;
    }

    // Find the orbital point in the list of all objects. Every id reaching this
    // function comes from an orbit stored on one of those objects, so the lookup
    // cannot fail; if the invariant is ever broken, skip instead of panicking.
    let Some(orbital_point_index) = all_objects.iter().position(|op| op.id == orbital_point_id)
    else {
        return;
    };

    let (orbit_option, orbital_point) = {
        let orbital_point = &all_objects[orbital_point_index];
        (orbital_point.own_orbit.clone(), orbital_point.clone())
    };

    // Calculate the distance for the primary orbit if it exists
    if let Some(orbit) = orbit_option {
        calculate_distance_from_system_center(orbit.primary_body_id, all_objects, calculated_ids);

        let primary_orbit_distance = get_primary_orbit_distance(&orbit, all_objects);

        // Update the distance from the system center
        if let Some(own_orbit) = &mut all_objects[orbital_point_index].own_orbit {
            own_orbit.average_distance_from_system_center += primary_orbit_distance;
        }
    }

    // Calculate the distances for any orbits around this orbital point
    for orbit in orbital_point.orbits.clone() {
        calculate_distance_from_system_center(orbit.primary_body_id, all_objects, calculated_ids);

        let primary_orbit_distance = get_primary_orbit_distance(&orbit, all_objects);

        if let Some(orbit) = all_objects[orbital_point_index]
            .orbits
            .iter_mut()
            .find(|o| o.id.unwrap_or(u32::MAX) == orbit.id.unwrap_or(u32::MAX))
        {
            orbit.average_distance_from_system_center += primary_orbit_distance;
        }
    }
}

fn get_primary_orbit_distance(orbit: &Orbit, all_objects: &[OrbitalPoint]) -> f64 {
    // The primary body's id always refers to an existing orbital point; a missing
    // one (broken invariant) deterministically yields the same 0.0 as no orbit.
    let primary_orbit_option = all_objects
        .iter()
        .find(|op| op.id == orbit.primary_body_id)
        .and_then(|op| op.own_orbit.as_ref());

    match primary_orbit_option {
        Some(primary_orbit) => primary_orbit.average_distance_from_system_center,
        None => 0.0,
    }
}

fn update_existing_orbits(all_objects: &mut Vec<OrbitalPoint>) {
    all_objects
        .iter_mut()
        .for_each(|o| o.update_object_own_orbit());
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
    let table = CopyableRollToProcess::new(
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
    );
    // Unreachable fallback (hardcoded non-empty table): first entry.
    let range = rng
        .get_result(&table)
        .unwrap_or(table.possible_results[0].result);
    let generated = rng.gen_f64() % (range.1 - range.0) + range.0;
    generated
}

/// Finds where the barycentre between two stars or centers of mass is.
fn calculate_barycentre(distance_between: f64, heaviest_mass: f64, lowest_mass: f64) -> f64 {
    distance_between * (lowest_mass as f64 / (heaviest_mass as f64 + lowest_mass as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::galaxy::neighborhood::GalacticNeighborhood;
    use crate::generator::types::GenerationSettings;
    use crate::universe::Universe;

    #[test]
    fn a_generated_system_carries_its_diagnostics() -> Result<(), GenerationError> {
        let settings = GenerationSettings {
            seed: Rc::from("system-diagnostics"),
            ..Default::default()
        };
        let mut diagnostics = Diagnostics::default();
        let universe = Universe::generate(&settings, &mut diagnostics);
        let neighborhood = GalacticNeighborhood::generate(universe, &settings, &mut diagnostics);
        let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
        let coord = SpaceCoordinates::new(0, 0, 0);
        let sub_sector = galaxy.get_division_at_level(coord, 1)?.clone();
        let hex = galaxy.get_hex(coord)?.clone();

        let system = StarSystem::generate(0, coord, &hex, &sub_sector, &mut galaxy)?;

        // The field rides along with the accepted generation try; nothing in the
        // pipeline pushes diagnostics yet, so it is present but empty.
        assert!(system.diagnostics.is_empty());
        Ok(())
    }
}
