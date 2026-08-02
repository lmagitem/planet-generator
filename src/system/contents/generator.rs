use crate::diagnostics::types::{GenerationError, GenerationWarning};
use crate::generator::types::GenerationSettings;
use crate::system::celestial_body::gaseous::types::GaseousBodySettings;
use crate::system::celestial_body::gaseous::GaseousBodyDetails;
use crate::system::celestial_body::icy::IcyBodyDetails;
use crate::system::celestial_body::telluric::types::TelluricBodyComposition;
use crate::system::celestial_body::telluric::TelluricBodyDetails;
use crate::system::celestial_body::traits::CelestialBodySpecialTrait;
use crate::system::celestial_body::types::CelestialBodyComposition;
use crate::system::celestial_body::types::CelestialBodyDetails;
use crate::system::celestial_body::types::CelestialBodySettings;
use crate::system::celestial_body::types::CelestialBodySize;
use crate::system::celestial_body::world::WorldGenerator;
#[cfg(test)]
use crate::system::celestial_body::CelestialBody;
use crate::system::contents::get_next_id;
use crate::system::contents::types::GasGiantArrangement;
use crate::system::contents::zones::collect_all_zones;
use crate::system::orbital_point::types::AstronomicalObject;
use crate::system::orbital_point::types::Orbit;
use crate::system::orbital_point::utils::sort_orbital_points_by_average_distance;
use crate::system::orbital_point::OrbitalPoint;
use crate::system::star::types::RotationAnomalySpeed;
use crate::system::star::types::StarAgeDifference;
use crate::system::star::types::StarGenContext;
use crate::system::star::types::StarMetallicityDifference;
use crate::system::star::types::StarPeculiarity;
use crate::system::star::types::StarSpectralType;
use crate::system::star::types::StarZone;
use crate::system::star::types::VariableStarInterval;
use crate::system::star::types::ZoneType;
use crate::system::star::Star;
use crate::system::types::CataclysmSeverity;
use crate::system::types::DebrisDensity;
use crate::system::types::NebulaeApparentSize;
use crate::system::types::StellarEvolution;
use crate::system::types::SystemGenContext;
use crate::system::types::SystemPeculiarity;
use crate::utils::harmonics::OrbitalHarmonicsUtils;
use crate::utils::string::StringUtils;
use log::debug;
use seeded_dice_roller::{
    CopyableRollToProcess, CopyableWeightedResult, PreparedRoll, RollMethod, RollToProcess,
    SeededDiceRoller, WeightedResult,
};
use std::fmt::Write as _;
use std::mem::discriminant;

pub fn generate_stars_systems(ctx: &SystemGenContext, all_objects: &mut Vec<OrbitalPoint>) {
    let mut next_id = get_next_id(all_objects);
    let all_zones = collect_all_zones(all_objects);

    let mut number_of_bodies_per_star = collect_number_of_bodies_per_star(ctx, all_objects);
    // A system always contains at least one star by construction; the fallback is
    // unreachable but recorded if the invariant is ever broken.
    let primary_star_mass = all_objects
        .iter()
        .filter_map(|o| {
            if let AstronomicalObject::Star(star) = &o.object {
                Some(star.mass)
            } else {
                None
            }
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_else(|| {
            ctx.fail(GenerationError::InternalInvariant(
                "a star system must contain at least one star",
            ));
            0.0
        });

    let mut new_objects = Vec::new();
    number_of_bodies_per_star
        .iter_mut()
        .for_each(|(major_bodies_left, star_index)| {
            new_objects.append(&mut generate_orbits_and_bodies(
                ctx,
                all_objects,
                &all_zones,
                &mut Vec::new(),
                major_bodies_left,
                star_index,
                primary_star_mass,
                &mut next_id,
            ));
        });

    // TODO: Check if planets/moons rotate too fast to exist. If they do, check if they are interesting ones.
    //       If so, change the rotations to more sensible values. Otherwise smash them into an asteroid belt.

    all_objects.extend(new_objects);
    ctx.report();
}

fn collect_number_of_bodies_per_star(
    ctx: &SystemGenContext,
    all_objects: &mut Vec<OrbitalPoint>,
) -> Vec<(i32, usize)> {
    all_objects
        .iter_mut()
        .enumerate()
        .filter_map(|(index, o)| {
            if let AstronomicalObject::Star(ref mut star) = o.object {
                Some((generate_number_of_bodies(ctx, star), index))
            } else {
                None
            }
        })
        .collect()
}

fn generate_orbits_and_bodies(
    ctx: &SystemGenContext,
    all_objects: &mut Vec<OrbitalPoint>,
    all_zones: &[StarZone],
    new_objects: &mut Vec<OrbitalPoint>,
    major_bodies_left: &mut i32,
    star_index: &mut usize,
    primary_star_mass: f64,
    mut next_id: &mut u32,
) -> Vec<OrbitalPoint> {
    let initial_number_of_bodies = major_bodies_left.clone();
    let star_orbital_point = &mut all_objects[*star_index];
    let mut result: Vec<OrbitalPoint> = Vec::new();
    if let AstronomicalObject::Star(star) = &star_orbital_point.object {
        let gas_giant_arrangement = generate_gas_giant_arrangement(ctx, *major_bodies_left, star);
        // Per-star snapshot, built AFTER `generate_number_of_bodies` has run (it may
        // push `CircumstellarDisk` into `star.special_traits`, §6.3) and after the
        // arrangement roll. Star fields are OWNED clones: the star lives inside
        // `star_orbital_point`, borrowed mutably throughout the orbit loops.
        let star_ctx = StarGenContext::new(
            ctx,
            star,
            star_orbital_point.id,
            primary_star_mass,
            gas_giant_arrangement,
        );

        debug!(
            "Major bodies left: {}, star index: {}, star id: {}",
            major_bodies_left, star_index, star_orbital_point.id
        );

        let mut reference_orbit_radius =
            generate_reference_orbit_radius(ctx, major_bodies_left, star);

        if let Some(orbit_radius) =
            generate_proto_gas_giant_position(ctx, &gas_giant_arrangement, star)
        {
            let (orbit_radius, orbit, point, mut moons) = handle_proto_gas_giant_placement(
                &star_ctx,
                all_zones,
                orbit_radius,
                major_bodies_left,
                &mut next_id,
                star.orbit.clone(),
                reference_orbit_radius,
                orbit_radius,
            );
            reference_orbit_radius = orbit_radius;

            if let Some(o) = orbit {
                star_orbital_point.orbits.push(o);
            }
            if let Some(p) = point {
                new_objects.push(p);
            }
            new_objects.append(&mut moons);
        }

        // Generate all possible orbits
        let mut new_orbits = generate_orbits(
            ctx,
            all_zones,
            &star_orbital_point,
            star,
            &mut reference_orbit_radius,
        );
        star_orbital_point.orbits.append(&mut new_orbits);

        // Sort all zones by increasing distance
        star_orbital_point.orbits.sort_by(|a, b| {
            a.average_distance
                .partial_cmp(&b.average_distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let spawn_chances = if star_orbital_point.orbits.len() == 0 || initial_number_of_bodies == 0
        {
            0
        } else {
            (initial_number_of_bodies as f32 / star_orbital_point.orbits.len() as f32 * 100.0)
                as i32
        };
        let orbits_with_gas_giants_data =
            get_orbits_with_gas_giants(new_objects, star_orbital_point);
        let orbit_contents: Vec<(usize, f64, Option<u32>)> = vec![];
        if spawn_chances > 0 {
            let orbit_contents = place_body_stubs(
                &star_ctx,
                new_objects,
                major_bodies_left,
                &mut next_id,
                star_orbital_point,
                spawn_chances,
                orbits_with_gas_giants_data,
                orbit_contents,
            );
            let orbits_with_gas_giants_data =
                get_orbits_with_gas_giants(new_objects, star_orbital_point);

            result.append(&mut replace_stubs(
                &star_ctx,
                new_objects,
                major_bodies_left,
                &mut next_id,
                star_orbital_point,
                orbits_with_gas_giants_data,
                orbit_contents,
            ));
        } else {
            debug!(
            "Spawn chances are 0% for star index: {}, star id: {:#?}, skipping bodies generation altogether",
            star_index, star_orbital_point.id
        );
            result.append(new_objects);
        }
    }
    result
}

fn get_orbits_with_gas_giants(
    new_objects: &mut Vec<OrbitalPoint>,
    star_orbital_point: &mut OrbitalPoint,
) -> Vec<(usize, f64)> {
    let orbits_with_gas_giants_data: Vec<(usize, f64)> = star_orbital_point
        .orbits
        .iter()
        .enumerate()
        .filter_map(|(index, orbit)| {
            if new_objects.iter().any(|object| {
                object.id == orbit.id.unwrap_or(u32::MAX)
                    && matches!(object.object, AstronomicalObject::GaseousBody(_))
            }) {
                Some((index, orbit.average_distance))
            } else {
                None
            }
        })
        .collect();
    orbits_with_gas_giants_data
}

fn generate_reference_orbit_radius(
    ctx: &SystemGenContext,
    major_bodies_left: &mut i32,
    star: &Star,
) -> f64 {
    let mut rng = ctx.with_step(|step| {
        let _ = write!(
            step,
            "sys_{}_{}_str_{}_bdy{}_loc",
            ctx.coord, ctx.system_index, star.orbital_point_id, major_bodies_left
        );
        SeededDiceRoller::new(&ctx.settings.seed, step)
    });
    let reference_orbit_radius = star
        .zones
        .iter()
        .filter(|&o| o.zone_type != ZoneType::ForbiddenZone)
        .max_by(|a, b| {
            a.end
                .partial_cmp(&b.end)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(&StarZone::new(0.0, 0.0, ZoneType::ForbiddenZone))
        .end
        / (rng.roll(1, 6, 0) as f64 * 0.05 + 1.0);
    reference_orbit_radius
}

fn generate_orbits(
    ctx: &SystemGenContext,
    all_zones: &[StarZone],
    star_orbital_point: &OrbitalPoint,
    star: &Star,
    reference_orbit_radius: &mut f64,
) -> Vec<Orbit> {
    let mut rng = ctx.with_step(|step| {
        let _ = write!(
            step,
            "sys_{}_{}_str_{}_orbt_loc",
            ctx.coord, ctx.system_index, star.orbital_point_id
        );
        SeededDiceRoller::new(&ctx.settings.seed, step)
    });
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
    all_zones: &[StarZone],
    orbits: &mut Vec<Orbit>,
    star_id: u32,
    star_orbit: &Orbit,
    rng: &mut SeededDiceRoller,
    last_orbit: &mut f64,
) {
    let mut inner_orbits_done = false;
    while !inner_orbits_done {
        let multiplier = get_orbit_multiplier(rng);
        let mut next_orbit = *last_orbit / multiplier;
        if *last_orbit - next_orbit < 0.15 {
            next_orbit = *last_orbit - 0.15 + rng.roll(1, 301, -151) as f64 / 10000.0;
        }
        if next_orbit <= 0.0 {
            break;
        }
        *last_orbit = next_orbit;

        let next_orbit_from_center = next_orbit + star_orbit.average_distance_from_system_center;
        place_orbit_if_possible(
            all_zones,
            orbits,
            star_id,
            &mut inner_orbits_done,
            next_orbit,
            next_orbit_from_center,
        );
    }
}

/// Pure-literal weighted table for orbit spacing; hoisted out of the orbit loops
/// (same entries, same order, same roll method as the former per-call table).
static ORBIT_MULTIPLIER_TABLE: std::sync::LazyLock<CopyableRollToProcess<f64>> =
    std::sync::LazyLock::new(|| {
        CopyableRollToProcess::new(
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
        )
    });

fn get_orbit_multiplier(rng: &mut SeededDiceRoller) -> f64 {
    // Unreachable fallback (hardcoded non-empty table): first entry.
    rng.get_result(&ORBIT_MULTIPLIER_TABLE)
        .unwrap_or(ORBIT_MULTIPLIER_TABLE.possible_results[0].result)
}

fn generate_outer_orbits(
    all_zones: &[StarZone],
    orbits: &mut Vec<Orbit>,
    star_id: u32,
    star_orbit: &Orbit,
    rng: &mut SeededDiceRoller,
    last_orbit: &mut f64,
) {
    let mut outer_orbits_done = false;
    while !outer_orbits_done {
        let multiplier = get_orbit_multiplier(rng);
        let next_orbit = *last_orbit * multiplier;
        *last_orbit = next_orbit;

        let next_orbit_from_center = next_orbit + star_orbit.average_distance_from_system_center;
        place_orbit_if_possible(
            all_zones,
            orbits,
            star_id,
            &mut outer_orbits_done,
            next_orbit,
            next_orbit_from_center,
        );
    }
}

fn place_orbit_if_possible(
    all_zones: &[StarZone],
    orbits: &mut Vec<Orbit>,
    star_id: u32,
    is_done: &mut bool,
    next_orbit: f64,
    next_orbit_from_center: f64,
) {
    if let Some(zone) = all_zones
        .iter()
        .find(|o| next_orbit_from_center >= o.start && next_orbit_from_center <= o.end)
    {
        match zone.zone_type {
            ZoneType::InnerZone | ZoneType::BioZone | ZoneType::OuterZone => {
                let orbit = Orbit::new(
                    star_id,
                    None,
                    zone.zone_type,
                    next_orbit,
                    0.0,
                    0.0,
                    next_orbit_from_center,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    f32::INFINITY,
                );
                orbits.push(orbit);
            }
            ZoneType::ForbiddenZone => (),
            _ => {
                *is_done = true;
            }
        }
    } else {
        *is_done = true;
    }
}

/// Iterates over all orbits and place all gas giants other than the first one, and body stubs for the other types of planets.
fn place_body_stubs(
    ctx: &StarGenContext,
    new_objects: &mut Vec<OrbitalPoint>,
    major_bodies_left: &mut i32,
    next_id: &mut u32,
    star_orbital_point: &mut OrbitalPoint,
    spawn_chances: i32,
    outwards_orbits_with_gas_giants_data: Vec<(usize, f64)>,
    mut orbit_contents: Vec<(usize, f64, Option<u32>)>,
) -> Vec<(usize, f64, Option<u32>)> {
    let mut populated_orbit_index = 0;

    // Settings variants and roll tables are constant for the whole star; build them
    // once instead of per orbit. The only per-orbit variation is the
    // `do_not_generate_gaseous` flag, which is either forced on or decided by
    // `should_skip_gaseous_body_gen` — i.e. always one of the same two variants.
    let settings = ctx.settings();
    let no_gaseous_settings = GenerationSettings {
        celestial_body: CelestialBodySettings {
            do_not_generate_gaseous: true,
            ..settings.celestial_body.clone()
        },
        ..settings.clone()
    };
    let gaseous_settings = GenerationSettings {
        celestial_body: CelestialBodySettings {
            do_not_generate_gaseous: false,
            ..settings.celestial_body.clone()
        },
        ..settings.clone()
    };
    let spawn_table = build_spawn_table(spawn_chances);
    let inner_body_type_table_no_gaseous = build_inner_body_type_table(&no_gaseous_settings);
    let inner_body_type_table_gaseous = build_inner_body_type_table(&gaseous_settings);
    let outer_body_type_table_no_gaseous = build_outer_body_type_table(&no_gaseous_settings);
    let outer_body_type_table_gaseous = build_outer_body_type_table(&gaseous_settings);

    star_orbital_point
        .orbits
        .iter_mut()
        .enumerate()
        .for_each(|(possible_orbit_index, orbit)| {
            if orbit.id.is_none() {
                let mut rng = ctx.system.with_step(|step| {
                    let _ = write!(
                        step,
                        "sys_{}_{}_str_{}_bdy{}_orbit{}_gen",
                        ctx.system.coord,
                        ctx.system.system_index,
                        star_orbital_point.id,
                        major_bodies_left,
                        possible_orbit_index
                    );
                    SeededDiceRoller::new(&ctx.settings().seed, step)
                });

                let inwards_gas_giant = orbit_contents
                    .iter()
                    .rev()
                    .find(|&&(gi, _, _)| gi < possible_orbit_index);
                let outwards_gas_giant = outwards_orbits_with_gas_giants_data
                    .iter()
                    .find(|&&(gi, _)| gi > possible_orbit_index);
                let gas_giant_au_inwards_proximity =
                    inwards_gas_giant.map(|&(_, dist, _)| orbit.average_distance - dist);
                let gas_giant_au_outwards_proximity =
                    outwards_gas_giant.map(|&(_, distance)| distance - orbit.average_distance);

                debug!(
                    "Looking at orbit n°{}, nextId is: {}, distance: {}au",
                    possible_orbit_index, next_id, orbit.average_distance
                );
                let should_spawn =
                    should_spawn(&mut rng, &spawn_table) && major_bodies_left > &mut 0;
                if should_spawn {
                    populated_orbit_index += 1;
                    debug!(
                        "Should spawn one of {} bodies left in {}, nextId is: {}",
                        *major_bodies_left, orbit.zone, next_id
                    );
                    match ctx.gas_giant_arrangement {
                        GasGiantArrangement::NoGasGiant => {
                            match orbit.zone {
                                ZoneType::InnerZone | ZoneType::BioZone => {
                                    let body_id = *next_id;
                                    *next_id += 1;
                                    orbit.id = Some(body_id);

                                    let body_type =
                                        roll_body_type(&mut rng, &inner_body_type_table_no_gaseous);
                                    let mut body_orbital_point = generate_new_body_and_moons(
                                        ctx,
                                        body_id,
                                        populated_orbit_index as u32,
                                        orbit.average_distance,
                                        next_id,
                                        orbit,
                                        body_type,
                                    );
                                    new_objects.push(body_orbital_point.0);
                                    new_objects.append(&mut body_orbital_point.1);

                                    debug!(
                                        "{} - {} - Generate a {:?}",
                                        ctx.gas_giant_arrangement, orbit.zone, body_type
                                    );

                                    // Book-keeping
                                    *major_bodies_left -= 1;
                                    if body_type == CelestialBodyComposition::Gaseous {
                                        *major_bodies_left -= 1;
                                    }
                                }
                                ZoneType::OuterZone => {
                                    let body_id = *next_id;
                                    *next_id += 1;
                                    orbit.id = Some(body_id);

                                    let body_type =
                                        roll_body_type(&mut rng, &outer_body_type_table_no_gaseous);
                                    let mut body_orbital_point = generate_new_body_and_moons(
                                        ctx,
                                        body_id,
                                        populated_orbit_index as u32,
                                        orbit.average_distance,
                                        next_id,
                                        orbit,
                                        body_type,
                                    );
                                    new_objects.push(body_orbital_point.0);
                                    new_objects.append(&mut body_orbital_point.1);

                                    debug!(
                                        "{} - {} - Generate a {:?}",
                                        ctx.gas_giant_arrangement, orbit.zone, body_type
                                    );

                                    // Book-keeping
                                    *major_bodies_left -= 1;
                                }
                                _ => {}
                            }
                        }
                        GasGiantArrangement::ConventionalGasGiant => {
                            match orbit.zone {
                                ZoneType::InnerZone | ZoneType::BioZone => {
                                    let body_id = *next_id;
                                    *next_id += 1;
                                    orbit.id = Some(body_id);

                                    let body_type =
                                        roll_body_type(&mut rng, &inner_body_type_table_no_gaseous);
                                    let mut body_orbital_point = generate_new_body_and_moons(
                                        ctx,
                                        body_id,
                                        populated_orbit_index as u32,
                                        orbit.average_distance,
                                        next_id,
                                        orbit,
                                        body_type,
                                    );
                                    new_objects.push(body_orbital_point.0);
                                    new_objects.append(&mut body_orbital_point.1);

                                    debug!(
                                        "{} - {} - Generate a {:?}",
                                        ctx.gas_giant_arrangement, orbit.zone, body_type
                                    );

                                    // Book-keeping
                                    *major_bodies_left -= 1;
                                    if body_type == CelestialBodyComposition::Gaseous {
                                        *major_bodies_left -= 1;
                                    }
                                }
                                ZoneType::OuterZone => {
                                    debug!("GasGiantArrangement::ConventionalGasGiant ZoneType::OuterZone");
                                    let body_id = *next_id;
                                    *next_id += 1;
                                    orbit.id = Some(body_id);

                                    let body_type = if should_skip_gaseous_body_gen(
                                        inwards_gas_giant,
                                        outwards_gas_giant,
                                        gas_giant_au_inwards_proximity,
                                        gas_giant_au_outwards_proximity,
                                    ) {
                                        roll_body_type(&mut rng, &outer_body_type_table_no_gaseous)
                                    } else {
                                        roll_body_type(&mut rng, &outer_body_type_table_gaseous)
                                    };
                                    let mut body_orbital_point = generate_new_body_and_moons(
                                        ctx,
                                        body_id,
                                        populated_orbit_index as u32,
                                        orbit.average_distance,
                                        next_id,
                                        orbit,
                                        body_type,
                                    );
                                    new_objects.push(body_orbital_point.0);
                                    new_objects.append(&mut body_orbital_point.1);

                                    debug!(
                                        "{} - {} - Generate a {:?}",
                                        ctx.gas_giant_arrangement, orbit.zone, body_type
                                    );

                                    // Book-keeping
                                    *major_bodies_left -= 1;
                                }
                                _ => {}
                            }
                        }
                        GasGiantArrangement::EpistellarGasGiant
                        | GasGiantArrangement::EccentricGasGiant => {
                            match orbit.zone {
                                ZoneType::InnerZone | ZoneType::BioZone => {
                                    let body_id = *next_id;
                                    *next_id += 1;
                                    orbit.id = Some(body_id);

                                    let body_type = if should_skip_gaseous_body_gen(
                                        inwards_gas_giant,
                                        outwards_gas_giant,
                                        gas_giant_au_inwards_proximity,
                                        gas_giant_au_outwards_proximity,
                                    ) {
                                        roll_body_type(&mut rng, &inner_body_type_table_no_gaseous)
                                    } else {
                                        roll_body_type(&mut rng, &inner_body_type_table_gaseous)
                                    };
                                    let mut body_orbital_point = generate_new_body_and_moons(
                                        ctx,
                                        body_id,
                                        populated_orbit_index as u32,
                                        orbit.average_distance,
                                        next_id,
                                        orbit,
                                        body_type,
                                    );
                                    new_objects.push(body_orbital_point.0);
                                    new_objects.append(&mut body_orbital_point.1);

                                    debug!(
                                        "{} - {} - Generate a {:?}",
                                        ctx.gas_giant_arrangement, orbit.zone, body_type
                                    );

                                    // Book-keeping
                                    *major_bodies_left -= 1;
                                    if body_type == CelestialBodyComposition::Gaseous {
                                        *major_bodies_left -= 1;
                                    }
                                }
                                ZoneType::OuterZone => {
                                    let body_id = *next_id;
                                    *next_id += 1;
                                    orbit.id = Some(body_id);

                                    let body_type = if should_skip_gaseous_body_gen(
                                        inwards_gas_giant,
                                        outwards_gas_giant,
                                        gas_giant_au_inwards_proximity,
                                        gas_giant_au_outwards_proximity,
                                    ) {
                                        roll_body_type(&mut rng, &outer_body_type_table_no_gaseous)
                                    } else {
                                        roll_body_type(&mut rng, &outer_body_type_table_gaseous)
                                    };
                                    let mut body_orbital_point = generate_new_body_and_moons(
                                        ctx,
                                        body_id,
                                        populated_orbit_index as u32,
                                        orbit.average_distance,
                                        next_id,
                                        orbit,
                                        body_type,
                                    );
                                    new_objects.push(body_orbital_point.0);
                                    new_objects.append(&mut body_orbital_point.1);

                                    debug!(
                                        "{} - {} - Generate a {:?}",
                                        ctx.gas_giant_arrangement, orbit.zone, body_type
                                    );

                                    // Book-keeping
                                    *major_bodies_left -= 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            } else {
                debug!(
                    "Skipping orbit n°{}, nextId is: {}, distance: {}au",
                    possible_orbit_index, next_id, orbit.average_distance
                );
            }
            if let Some(id) = orbit.id {
                orbit_contents.push((
                    possible_orbit_index,
                    orbit.average_distance,
                    new_objects.iter().find(|o| o.id == id).map(|o| o.id),
                ));
            } else {
                orbit_contents.push((possible_orbit_index, orbit.average_distance, None));
            }
        });

    orbit_contents
}

/// Iterates over all orbits replace stubs by proper worlds. The nature of each world is influenced
/// by nearby gas giants..
fn replace_stubs(
    ctx: &StarGenContext,
    new_objects: &mut Vec<OrbitalPoint>,
    major_bodies_left: &mut i32,
    next_id: &mut u32,
    star_orbital_point: &mut OrbitalPoint,
    orbits_with_gas_giants_data: Vec<(usize, f64)>,
    orbit_contents: Vec<(usize, f64, Option<u32>)>,
) -> Vec<OrbitalPoint> {
    let number_of_orbits = star_orbital_point.orbits.len();
    let orbits_before_outer_zone = star_orbital_point
        .orbits
        .iter()
        .take_while(|o| o.zone != ZoneType::OuterZone)
        .count();
    let mut populated_orbit_index = 0;
    star_orbital_point
        .orbits
        .iter_mut()
        .enumerate()
        .for_each(|(orbit_index, orbit)| {
            if orbit.id.is_some() {
                let (
                    gas_giant_orbits_inwards_proximity,
                    gas_giant_orbits_outwards_proximity,
                    nearest_forbidden_distance,
                    zone_change_orbits_proximity,
                ) = calculate_stub_orbital_relationships(
                    &star_orbital_point.object,
                    &orbits_with_gas_giants_data,
                    &orbit_contents,
                    orbits_before_outer_zone,
                    number_of_orbits,
                    orbit_index,
                    orbit,
                );
                // Unreachable: orbit_contents covers every orbit index; skip this
                // orbit instead of panicking if the invariant is ever broken.
                let Some((_, _, object_id)) = orbit_contents.iter().find(|o| o.0 == orbit_index)
                else {
                    return;
                };

                debug!(
                    "Looking at orbit n°{}, object_id: {:?}, distance: {}au",
                    orbit_index, object_id, orbit.average_distance
                );
                if let Some(id) = object_id {
                    populated_orbit_index += 1;
                    let index_to_replace = new_objects.iter().position(|o| o.id == *id);
                    if let Some(new_object_index) = index_to_replace {
                        let current_stub: &mut OrbitalPoint = &mut new_objects[new_object_index];
                        if match &current_stub.object {
                            AstronomicalObject::TelluricBody(body) => body.is_stub(),
                            AstronomicalObject::IcyBody(body) => body.is_stub(),
                            AstronomicalObject::GaseousBody(body) => body.is_stub(),
                            _ => false,
                        } {
                            let size_modifier = get_body_size_modifier(
                                ctx,
                                orbit_index,
                                major_bodies_left,
                                gas_giant_orbits_inwards_proximity,
                                gas_giant_orbits_outwards_proximity,
                                nearest_forbidden_distance,
                                zone_change_orbits_proximity,
                            );
                            let possibly_generated = generate_body_and_moons(
                                ctx,
                                next_id,
                                populated_orbit_index,
                                current_stub,
                                size_modifier,
                            );
                            if let Some(mut generated) = possibly_generated {
                                new_objects[new_object_index] = generated.0;
                                new_objects.append(&mut generated.1);
                            }
                        }
                    }
                }
            }
        });

    replace_telluric_stubs(ctx, new_objects, star_orbital_point, populated_orbit_index)
}

fn calculate_stub_orbital_relationships(
    star_object: &AstronomicalObject,
    orbits_with_gas_giants_data: &[(usize, f64)],
    orbit_contents: &[(usize, f64, Option<u32>)],
    orbits_before_outer_zone: usize,
    number_of_orbits: usize,
    orbit_index: usize,
    orbit: &mut Orbit,
) -> (Option<usize>, Option<usize>, f64, usize) {
    let inwards_gas_giant = orbit_contents
        .iter()
        .rev()
        .find(|&&(gi, _, _)| gi < orbit_index);
    let outwards_gas_giant = orbits_with_gas_giants_data
        .iter()
        .find(|&&(gi, _)| gi > orbit_index);
    let gas_giant_orbits_inwards_proximity = inwards_gas_giant.map(|&(gi, _, _)| orbit_index - gi);
    let gas_giant_orbits_outwards_proximity = outwards_gas_giant.map(|&(gi, _)| gi - orbit_index);
    let nearest_forbidden_distance = if let AstronomicalObject::Star(star) = star_object {
        star.zones
            .iter()
            .filter(|&zone| zone.zone_type == ZoneType::ForbiddenZone)
            .map(|zone| {
                let start_distance = (zone.start - orbit.average_distance).abs();
                let end_distance = (zone.end - orbit.average_distance).abs();
                if start_distance < end_distance {
                    start_distance
                } else {
                    end_distance
                }
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(f64::INFINITY)
    } else {
        f64::INFINITY
    };
    let zone_change_orbits_proximity = if orbit.zone != ZoneType::OuterZone {
        orbits_before_outer_zone
    } else {
        number_of_orbits - orbit_index
    };
    (
        gas_giant_orbits_inwards_proximity,
        gas_giant_orbits_outwards_proximity,
        nearest_forbidden_distance,
        zone_change_orbits_proximity,
    )
}

fn replace_telluric_stubs(
    ctx: &StarGenContext,
    new_objects: &mut Vec<OrbitalPoint>,
    _star_orbital_point: &mut OrbitalPoint,
    _populated_orbit_index: u32,
) -> Vec<OrbitalPoint> {
    let (mut finished_objects, mut non_finished_objects): (Vec<OrbitalPoint>, Vec<OrbitalPoint>) =
        new_objects.drain(..).partition(is_finished);

    let new_objects_ids = non_finished_objects
        .iter()
        .map(|o| o.id)
        .collect::<Vec<u32>>();

    sort_orbital_points_by_average_distance(&mut non_finished_objects);
    // TODO: Tidal heating should also happen when eccentric orbit near to parent
    let tidal_heating_array = OrbitalHarmonicsUtils::calculate_gravitational_harmonics(
        &OrbitalHarmonicsUtils::prepare_harmonics_array(&non_finished_objects, false),
        0.03,
    );

    new_objects_ids.iter().for_each(|orbit_id| {
        let index_to_replace = non_finished_objects.iter().position(|o| o.id == *orbit_id);
        if let Some(new_object_index) = index_to_replace {
            let is_telluric_stub = matches!(
                &non_finished_objects[new_object_index].object,
                AstronomicalObject::TelluricBody(body) if body.is_stub()
            );
            if is_telluric_stub {
                let current_stub = &mut non_finished_objects[new_object_index];
                let stub_id = current_stub.id;
                // The stub point is fully overwritten below; move its fields out
                // instead of cloning them.
                let stub_orbit = current_stub.own_orbit.take();
                let stub_orbits = std::mem::take(&mut current_stub.orbits);
                if let AstronomicalObject::TelluricBody(stub_body) =
                    std::mem::replace(&mut current_stub.object, AstronomicalObject::Void)
                {
                    let moons = finished_objects
                        .iter_mut()
                        .filter(|possible_moon_point| {
                            possible_moon_point
                                .own_orbit
                                .as_ref()
                                .map_or(false, |orbit| stub_id == orbit.primary_body_id)
                        })
                        .map(|o| o.clone())
                        .collect::<Vec<OrbitalPoint>>();

                    // TODO: Tidal heating should also happen when eccentric orbit near to parent
                    let tidal_heating = tidal_heating_array[new_object_index];
                    // A stub always has its own orbit; 0.0 is a deterministic,
                    // unreachable stand-in if the invariant is ever broken.
                    let distance_from_star = stub_orbit
                        .as_ref()
                        .map_or(0.0, |orbit| orbit.average_distance);
                    let generated = WorldGenerator::generate_world(
                        ctx,
                        stub_body,
                        stub_orbit.unwrap_or_default(),
                        stub_orbits,
                        &moons,
                        distance_from_star,
                        stub_id,
                        tidal_heating,
                        false,
                    );
                    non_finished_objects[new_object_index] = generated;
                }
            }
        }
    });

    finished_objects.append(&mut non_finished_objects);
    finished_objects
}

fn is_finished(o: &OrbitalPoint) -> bool {
    match &o.object {
        AstronomicalObject::TelluricBody(body)
        | AstronomicalObject::GaseousBody(body)
        | AstronomicalObject::IcyBody(body) => !body.is_stub(),
        _ => false,
    }
}

fn generate_body_and_moons(
    ctx: &StarGenContext,
    next_id: &mut u32,
    populated_orbit_index: u32,
    current_stub: &mut OrbitalPoint,
    size_modifier: i32,
) -> Option<(OrbitalPoint, Vec<OrbitalPoint>)> {
    let mut body_type = TelluricBodyComposition::Rocky;
    let body_id = current_stub.id;
    // The caller always overwrites the stub point with the returned one, so the
    // fields can be moved out instead of cloned.
    let orbit = current_stub.own_orbit.take();
    let orbit_distance = orbit.as_ref().map_or(0.0, |orbit| orbit.average_distance);
    let orbited_by = std::mem::take(&mut current_stub.orbits);
    let is_moon = false;
    let fixed_size = None;

    match std::mem::replace(&mut current_stub.object, AstronomicalObject::Void) {
        AstronomicalObject::GaseousBody(mut body) => {
            body.name = format!(
                "{}{}",
                ctx.star_name,
                StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
            )
            .into();

            Some((
                OrbitalPoint::new(
                    body_id,
                    orbit,
                    AstronomicalObject::GaseousBody(body),
                    orbited_by,
                ),
                vec![],
            ))
        }
        stub_object => {
            match stub_object {
                AstronomicalObject::TelluricBody(body) => {
                    if let CelestialBodyDetails::Telluric(details) = body.details {
                        body_type = details.body_type
                    }
                }
                AstronomicalObject::IcyBody(_body) => {
                    body_type = TelluricBodyComposition::Icy;
                }
                _ => {}
            }

            Some(generate_body_from_type(
                ctx,
                next_id,
                populated_orbit_index,
                size_modifier,
                body_type,
                body_id,
                orbit,
                orbit_distance,
                orbited_by,
                is_moon,
                fixed_size,
            ))
        }
    }
}

pub(crate) fn generate_body_from_type(
    ctx: &StarGenContext,
    next_id: &mut u32,
    populated_orbit_index: u32,
    size_modifier: i32,
    body_type: TelluricBodyComposition,
    body_id: u32,
    orbit: Option<Orbit>,
    orbit_distance: f64,
    orbited_by: Vec<Orbit>,
    is_moon: bool,
    fixed_size: Option<CelestialBodySize>,
) -> (OrbitalPoint, Vec<OrbitalPoint>) {
    if body_type == TelluricBodyComposition::Metallic {
        TelluricBodyDetails::generate_metallic_body(
            ctx,
            body_id,
            populated_orbit_index,
            orbit.clone(),
            orbit_distance,
            orbited_by.clone(),
            next_id,
            size_modifier,
            is_moon,
            fixed_size,
        )
    } else if body_type == TelluricBodyComposition::Rocky {
        TelluricBodyDetails::generate_rocky_body(
            ctx,
            body_id,
            populated_orbit_index,
            orbit.clone(),
            orbit_distance,
            orbited_by.clone(),
            next_id,
            size_modifier,
            is_moon,
            fixed_size,
        )
    } else if body_type == TelluricBodyComposition::Icy {
        IcyBodyDetails::generate_icy_body(
            ctx,
            body_id,
            populated_orbit_index,
            orbit,
            orbit_distance,
            orbited_by.clone(),
            next_id,
            size_modifier,
            is_moon,
            fixed_size,
        )
    } else {
        GaseousBodyDetails::generate_gas_giant(
            ctx,
            ctx.settings(),
            body_id,
            orbit.unwrap_or_default(),
            orbit_distance,
            populated_orbit_index,
            next_id,
        )
    }
}

fn get_body_size_modifier(
    ctx: &StarGenContext,
    orbit_index: usize,
    major_bodies_left: &mut i32,
    gas_giant_orbits_inwards_proximity: Option<usize>,
    gas_giant_orbits_outwards_proximity: Option<usize>,
    nearest_forbidden_distance: f64,
    zone_change_orbits_proximity: usize,
) -> i32 {
    let mut rng = ctx.system.with_step(|step| {
        let _ = write!(
            step,
            "sys_{}_{}_str_{}_bdy{}_orbit{}_rep",
            ctx.system.coord, ctx.system.system_index, ctx.star_id, major_bodies_left, orbit_index
        );
        SeededDiceRoller::new(&ctx.system.seed, step)
    });

    let mut size_modifier = 0;
    size_modifier += if nearest_forbidden_distance < 0.5 {
        -120
    } else {
        0
    };
    size_modifier += match gas_giant_orbits_outwards_proximity {
        Some(orbit_proximity) if orbit_proximity < 2 => -120,
        _ => 0,
    };
    size_modifier += match gas_giant_orbits_inwards_proximity {
        Some(orbit_proximity) if orbit_proximity < 2 => -60,
        _ => 0,
    };
    size_modifier += if zone_change_orbits_proximity < 2 {
        -60
    } else {
        0
    };
    size_modifier += match &ctx.star_type {
        StarSpectralType::WR(_)
        | StarSpectralType::O(_)
        | StarSpectralType::B(_)
        | StarSpectralType::A(_) => {
            if rng.roll(1, 50, 0) == 1 {
                0
            } else {
                -(ctx.star_mass * 5.0) as i32
            }
        }
        StarSpectralType::F(_) => 10,
        StarSpectralType::K(_) => -10,
        StarSpectralType::M(_) => -20,
        StarSpectralType::L(_) | StarSpectralType::T(_) | StarSpectralType::Y(_) => -50,
        _ => 0,
    };
    size_modifier
}

fn should_skip_gaseous_body_gen(
    inwards_gas_giant: Option<&(usize, f64, Option<u32>)>,
    outwards_gas_giant: Option<&(usize, f64)>,
    gas_giant_au_inwards_proximity: Option<f64>,
    gas_giant_au_outwards_proximity: Option<f64>,
) -> bool {
    (inwards_gas_giant.is_some() && outwards_gas_giant.is_some())
        || (gas_giant_au_inwards_proximity.map_or(false, |proximity| proximity < 0.5)
            || gas_giant_au_outwards_proximity.map_or(false, |proximity| proximity < 0.5))
}

fn generate_new_body_and_moons(
    ctx: &StarGenContext,
    body_id: u32,
    populated_orbit_index: u32,
    orbit_distance: f64,
    next_id: &mut u32,
    orbit: &mut Orbit,
    body_type: CelestialBodyComposition,
) -> (OrbitalPoint, Vec<OrbitalPoint>) {
    let mut moons: Vec<OrbitalPoint> = Vec::new();
    let orbital_point;
    let body;
    match body_type {
        CelestialBodyComposition::Metallic => {
            body = TelluricBodyDetails::generate_metallic_body_stub(body_id);

            orbital_point = OrbitalPoint::new(
                body_id,
                Some(orbit.clone()),
                AstronomicalObject::TelluricBody(body),
                vec![],
            );
        }
        CelestialBodyComposition::Rocky => {
            body = TelluricBodyDetails::generate_rocky_body_stub(body_id);

            orbital_point = OrbitalPoint::new(
                body_id,
                Some(orbit.clone()),
                AstronomicalObject::TelluricBody(body),
                vec![],
            );
        }
        CelestialBodyComposition::Gaseous => {
            let mut giant_and_moons = GaseousBodyDetails::generate_gas_giant(
                ctx,
                ctx.settings(),
                body_id,
                orbit.clone(),
                orbit_distance,
                populated_orbit_index,
                next_id,
            );
            orbital_point = giant_and_moons.0;
            moons.append(&mut giant_and_moons.1);
        }
        CelestialBodyComposition::Icy => {
            body = IcyBodyDetails::generate_icy_body_stub(body_id);

            orbital_point = OrbitalPoint::new(
                body_id,
                Some(orbit.clone()),
                AstronomicalObject::IcyBody(body),
                vec![],
            );
        }
    }
    (orbital_point, moons)
}

fn handle_proto_gas_giant_placement(
    ctx: &StarGenContext,
    all_zones: &[StarZone],
    orbit_distance: f64,
    major_bodies_left: &mut i32,
    next_id: &mut u32,
    star_orbit: Option<Orbit>,
    reference_orbit_radius: f64,
    orbit_radius: f64,
) -> (f64, Option<Orbit>, Option<OrbitalPoint>, Vec<OrbitalPoint>) {
    let mut reference_orbit_radius = reference_orbit_radius;
    let mut moons: Vec<OrbitalPoint> = Vec::new();
    let orbit_from_center = orbit_radius
        + star_orbit
            .unwrap_or_default()
            .average_distance_from_system_center;
    if let Some(zone) = all_zones
        .iter()
        .find(|zone| orbit_from_center >= zone.start && orbit_from_center <= zone.end)
    {
        if zone.zone_type != ZoneType::ForbiddenZone {
            let giant_id = *next_id;
            *next_id += 1;
            reference_orbit_radius = orbit_radius;
            // Create an Orbit
            let orbit = Orbit::new(
                ctx.star_id,
                Some(giant_id),
                zone.zone_type,
                orbit_radius,
                0.0,
                0.0,
                orbit_from_center,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                f32::INFINITY,
            );

            // Generate Gas Giant Settings
            let settings = ctx.settings();
            let celestial_body_settings = &settings.celestial_body;
            let gaseous_body_settings = celestial_body_settings.gaseous_body_settings.clone();
            let mut fixed_special_traits = gaseous_body_settings
                .fixed_special_traits
                .unwrap_or_else(Vec::new);
            if !fixed_special_traits.contains(&CelestialBodySpecialTrait::ProtoGiant) {
                fixed_special_traits.push(CelestialBodySpecialTrait::ProtoGiant);
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
            let mut giant_and_moons = GaseousBodyDetails::generate_gas_giant(
                ctx,
                &settings,
                giant_id,
                orbit,
                orbit_distance,
                0,
                next_id,
            );
            let gas_giant = giant_and_moons.0;
            moons.append(&mut giant_and_moons.1);

            // Book-keeping
            *major_bodies_left -= 1;
            if ctx.gas_giant_arrangement == GasGiantArrangement::EpistellarGasGiant {
                *major_bodies_left -= 1;
            }

            return (
                reference_orbit_radius,
                gas_giant.own_orbit.clone(),
                Some(gas_giant),
                moons,
            );
        }
    }
    (reference_orbit_radius, None, None, moons)
}

fn generate_number_of_bodies(ctx: &SystemGenContext, star: &mut Star) -> i32 {
    let mut rng = ctx.with_step(|step| {
        let _ = write!(
            step,
            "sys_{}_{}_str_{}_nbr_bdy",
            ctx.coord, ctx.system_index, star.orbital_point_id
        );
        SeededDiceRoller::new(&ctx.seed, step)
    });

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
    modifier += (if star.mass < 0.08 {
        -5.0
    } else if star.mass > 4.0 {
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

    // Unreachable (the table above is non-empty): index 0 is a deterministic stand-in.
    let number_of_bodies_index = rng
        .get_result_index(&RollToProcess::new(
            possible_results.clone(),
            RollMethod::PreparedRoll(PreparedRoll::new(2, 8, modifier)),
        ))
        .unwrap_or_else(|| {
            ctx.warn(GenerationWarning::RngTableFallback("number of bodies"));
            0
        });

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
    ctx: &SystemGenContext,
    number_of_major_bodies: i32,
    star: &Star,
) -> GasGiantArrangement {
    if number_of_major_bodies == 0
        || star
            .special_traits
            .contains(&StarPeculiarity::CircumstellarDisk)
    {
        GasGiantArrangement::NoGasGiant
    } else {
        let mut nothing_chances = 50;
        let mut conventional_chances = 30;
        let mut eccentric_chances = 15;
        let mut epistellar_chances = 5;
        let modifiers = apply_gas_giant_arrangement_modifiers(
            ctx.system_traits,
            &star.special_traits,
            &star.spectral_type,
            &star.population,
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

        let table = CopyableRollToProcess::new(
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
        );
        // Unreachable (the weights above always sum to at least 100): first entry.
        ctx.with_step(|step| {
            let _ = write!(
                step,
                "sys_{}_{}_str_{}_gas_arr",
                ctx.coord, ctx.system_index, star.orbital_point_id
            );
            SeededDiceRoller::new(&ctx.settings.seed, step)
        })
        .get_result(&table)
        .unwrap_or_else(|| {
            ctx.warn(GenerationWarning::RngTableFallback("gas giant arrangement"));
            table.possible_results[0].result
        })
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
    ctx: &SystemGenContext,
    arrangement: &GasGiantArrangement,
    star: &Star,
) -> Option<f64> {
    let mut rng = ctx.with_step(|step| {
        let _ = write!(
            step,
            "sys_{}_{}_str_{}_gas_pos",
            ctx.coord, ctx.system_index, star.orbital_point_id
        );
        SeededDiceRoller::new(&ctx.settings.seed, step)
    });

    match arrangement {
        GasGiantArrangement::ConventionalGasGiant => {
            let snow_line = star
                .zones
                .iter()
                .find(|z| z.zone_type == ZoneType::OuterZone)?
                .start;
            Some((1.0 + rng.roll(2, 6, -2) as f64 * 0.05) * snow_line)
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

fn build_spawn_table(spawn_chances: i32) -> CopyableRollToProcess<bool> {
    let spawn_chances = if spawn_chances > 100 {
        100
    } else if spawn_chances < 0 {
        0
    } else {
        10 + (spawn_chances as f32 * 0.90) as u32
    };
    CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult::new(false, 100 - spawn_chances),
            CopyableWeightedResult::new(true, spawn_chances),
        ],
        RollMethod::SimpleRoll,
    )
}

fn should_spawn(rng: &mut SeededDiceRoller, spawn_table: &CopyableRollToProcess<bool>) -> bool {
    // Unreachable fallback (weights always sum to 100): first entry.
    rng.get_result(spawn_table)
        .unwrap_or(spawn_table.possible_results[0].result)
}

// TODO: Add modifier according to star population and metallicity
fn build_inner_body_type_table(
    settings: &GenerationSettings,
) -> CopyableRollToProcess<CelestialBodyComposition> {
    CopyableRollToProcess::new(
        vec![
            // CopyableWeightedResult::new(
            //     CelestialBodySubType::Exotic,
            //     if settings.celestial_body.do_not_generate_exotic {
            //         0
            //     } else {
            //         1
            //     },
            // ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Metallic,
                if settings.celestial_body.do_not_generate_metallic {
                    0
                } else {
                    2
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Rocky,
                if settings.celestial_body.do_not_generate_rocky {
                    0
                } else {
                    6
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Icy,
                if settings.celestial_body.do_not_generate_icy {
                    0
                } else {
                    2
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Gaseous,
                if settings.celestial_body.do_not_generate_gaseous {
                    0
                } else {
                    1
                },
            ),
        ],
        RollMethod::SimpleRoll,
    )
}

pub(crate) fn generate_inner_body_type(
    rng: &mut SeededDiceRoller,
    settings: &GenerationSettings,
) -> CelestialBodyComposition {
    let table = build_inner_body_type_table(settings);
    roll_body_type(rng, &table)
}

// TODO: Add modifier according to star population and metallicity
fn build_outer_body_type_table(
    settings: &GenerationSettings,
) -> CopyableRollToProcess<CelestialBodyComposition> {
    CopyableRollToProcess::new(
        vec![
            // CopyableWeightedResult::new(
            //     CelestialBodySubType::Exotic,
            //     if settings.celestial_body.do_not_generate_exotic {
            //         0
            //     } else {
            //         1
            //     },
            // ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Metallic,
                if settings.celestial_body.do_not_generate_metallic {
                    0
                } else {
                    1
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Rocky,
                if settings.celestial_body.do_not_generate_rocky {
                    0
                } else {
                    3
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Icy,
                if settings.celestial_body.do_not_generate_icy {
                    0
                } else {
                    6
                },
            ),
            CopyableWeightedResult::new(
                CelestialBodyComposition::Gaseous,
                if settings.celestial_body.do_not_generate_gaseous {
                    0
                } else {
                    6
                },
            ),
        ],
        RollMethod::SimpleRoll,
    )
}

pub(crate) fn generate_outer_body_type(
    rng: &mut SeededDiceRoller,
    settings: &GenerationSettings,
) -> CelestialBodyComposition {
    let table = build_outer_body_type_table(settings);
    roll_body_type(rng, &table)
}

/// Rolls a body type in the given table. If every body type was disabled via the
/// `do_not_generate_*` settings the table's weights are all 0, which would make the
/// dice roller divide by zero; the first entry is then used as-is. (Rejecting that
/// settings combination is a `GenerationSettings::validate` concern.)
fn roll_body_type(
    rng: &mut SeededDiceRoller,
    table: &CopyableRollToProcess<CelestialBodyComposition>,
) -> CelestialBodyComposition {
    // Unreachable fallback otherwise (the table is hardcoded non-empty): first entry.
    let fallback = table.possible_results[0].result;
    if table.possible_results.iter().all(|r| r.weight == 0) {
        fallback
    } else {
        rng.get_result(table).unwrap_or(fallback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body_point(id: u32, stub: bool, primary_body_id: u32) -> OrbitalPoint {
        let body = if stub {
            TelluricBodyDetails::generate_rocky_body_stub(id)
        } else {
            CelestialBody::default()
        };
        OrbitalPoint::new(
            id,
            Some(Orbit {
                primary_body_id,
                id: Some(id),
                ..Default::default()
            }),
            AstronomicalObject::TelluricBody(body),
            Vec::new(),
        )
    }

    #[test]
    fn is_finished_is_true_for_polished_bodies() {
        // Moons are polished by `WorldGenerator::generate_world` as soon as they are
        // generated, so they must read as finished to be associable to their planet later.
        assert!(is_finished(&body_point(1, false, 0)));
        // Gas giants are generated in a single pass and are never stubs.
        let gas_giant = OrbitalPoint::new(
            2,
            None,
            AstronomicalObject::GaseousBody(CelestialBody::default()),
            Vec::new(),
        );
        assert!(is_finished(&gas_giant));
        let icy = OrbitalPoint::new(
            3,
            None,
            AstronomicalObject::IcyBody(CelestialBody::default()),
            Vec::new(),
        );
        assert!(is_finished(&icy));
    }

    #[test]
    fn is_finished_is_false_for_stubs_and_non_bodies() {
        // Planet stubs still waiting for their `generate_world` pass are not finished.
        assert!(!is_finished(&body_point(1, true, 0)));
        // Non-body objects (disks, voids, stars) are never candidate moons.
        let void = OrbitalPoint::new(2, None, AstronomicalObject::Void, Vec::new());
        assert!(!is_finished(&void));
    }

    #[test]
    fn partition_puts_a_polished_moon_among_finished_objects() {
        // Mirrors the fix in `replace_telluric_stubs`: a previously generated moon
        // (non-stub, orbiting the planet stub) must land in `finished_objects` so the
        // moon association filter can find it, while the planet stub must not.
        let planet_stub = body_point(10, true, 1);
        let moon = body_point(11, false, 10);
        let (finished, non_finished): (Vec<OrbitalPoint>, Vec<OrbitalPoint>) =
            vec![planet_stub, moon].into_iter().partition(is_finished);

        assert_eq!(finished.len(), 1);
        assert_eq!(finished[0].id, 11);
        assert_eq!(
            finished[0]
                .own_orbit
                .clone()
                .unwrap_or_default()
                .primary_body_id,
            10
        );
        assert_eq!(non_finished.len(), 1);
        assert_eq!(non_finished[0].id, 10);
    }
}
