use crate::diagnostics::types::{Diagnostics, GenerationWarning};
use crate::galaxy::constants::{
    default_layers_for, get_opposite_traits, OppositeTraits, LOCAL_GROUP_GALAXIES, NO_SPECIAL_TRAIT,
};
use crate::galaxy::map::division_level::GalacticMapDivisionLevel;
#[cfg(test)]
use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::region_map::types::GalacticRegionMap;
#[cfg(test)]
use crate::galaxy::map::types::GalacticRegion;
use crate::galaxy::neighborhood::types::GalacticNeighborhoodDensity;
use crate::galaxy::neighborhood::GalacticNeighborhood;
use crate::galaxy::types::GalaxyCategory;
use crate::galaxy::types::GalaxySatellites;
#[cfg(test)]
use crate::galaxy::types::GalaxySettings;
use crate::galaxy::types::GalaxySpecialTrait;
use crate::galaxy::types::GalaxySubCategory;
use crate::galaxy::Galaxy;
use crate::generator::types::GenerationSettings;
use crate::universe::types::StelliferousEra;
#[cfg(test)]
use crate::universe::types::UniverseSettings;
#[cfg(test)]
use crate::universe::Universe;
use log::debug;
use seeded_dice_roller::{
    CopyableRollToProcess, CopyableWeightedResult, RollMethod, SeededDiceRoller,
};
use std::collections::BTreeMap;
use std::mem::discriminant;
use std::rc::Rc;

impl Galaxy {
    /// Generates a brand new [Galaxy] using the given seed and [GenerationSettings].
    ///
    /// The given settings are expected to be valid (see
    /// [`GenerationSettings::validate`]); [`Generator::generate`] performs that
    /// check for you, direct callers of this method should do it first.
    pub fn generate(
        neighborhood: GalacticNeighborhood,
        index: u16,
        settings: &GenerationSettings,
    ) -> Self {
        let seed = &settings.seed.clone();
        debug!(
            "generating new galaxy (seed: {}, settings: {})",
            seed, settings.galaxy
        );
        let name;
        let is_dominant;
        let is_major;
        let age;
        let mut category;
        let sub_category;
        let special_traits;

        if settings.galaxy.use_ours && (index as usize) < LOCAL_GROUP_GALAXIES.len() {
            // Our universe and galaxy
            let model = LOCAL_GROUP_GALAXIES[index as usize].clone();
            name = model.name.into();
            is_dominant = model.is_dominant;
            is_major = model.is_major;
            age = if model.age > 0.0 {
                model.age
            } else {
                generate_age(neighborhood, index, seed, settings)
            };
            category = model.category;
            sub_category = model.sub_category;
            special_traits = if model.third_trait != NO_SPECIAL_TRAIT {
                vec![model.first_trait, model.second_trait, model.third_trait]
            } else if model.second_trait != NO_SPECIAL_TRAIT {
                vec![model.first_trait, model.second_trait]
            } else {
                vec![model.first_trait]
            };
        } else {
            // Generated galaxy
            name = "Galaxy".into();
            is_dominant = is_galaxy_dominant(neighborhood, index);
            is_major = is_galaxy_major(neighborhood, index);
            age = generate_age(neighborhood, index, seed, settings);
            category = generate_category(
                neighborhood,
                index,
                age,
                is_dominant,
                is_major,
                seed,
                settings,
            );
            sub_category = generate_sub_category(category, index, age, is_major, seed, settings);
            if settings.galaxy.fixed_category.is_none() {
                category = get_category_with_size(category, sub_category, index, seed);
            }
            special_traits = generate_special_traits(
                neighborhood,
                category,
                sub_category,
                index,
                seed,
                settings,
            );
        }
        let division_levels = GalacticMapDivisionLevel::generate_division_levels(settings);
        let galactic_map_layers = if settings.galaxy.galactic_map_layers.is_empty() {
            default_layers_for(category)
        } else {
            settings.galaxy.galactic_map_layers.clone()
        };

        let mut galaxy = Self {
            settings: settings.clone(),
            neighborhood,
            index,
            name,
            age,
            is_dominant,
            is_major,
            category,
            sub_category,
            special_traits,
            division_levels,
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers,
            region_map: None,
            diagnostics: Diagnostics::default(),
        };
        // The bounds helpers are methods on Galaxy, so the region map is built once the struct exists.
        let start = galaxy.get_galactic_start();
        let end = galaxy.get_galactic_end();
        galaxy.region_map = GalacticRegionMap::build_with_diagnostics(
            &galaxy.galactic_map_layers,
            start,
            end,
            &mut galaxy.diagnostics,
        );
        if galaxy.region_map.is_none() && !settings.galaxy.galactic_map_layers.is_empty() {
            // The user's SVG failed to parse (the warning was logged inside build): fall back to the
            // category's default map, which becomes the authoritative set of layers.
            galaxy
                .diagnostics
                .push_warning(GenerationWarning::UserLayersReplacedWithDefault);
            galaxy.galactic_map_layers = default_layers_for(category);
            galaxy.region_map = GalacticRegionMap::build_with_diagnostics(
                &galaxy.galactic_map_layers,
                start,
                end,
                &mut galaxy.diagnostics,
            );
        }
        debug!("generated: {}", galaxy);
        galaxy
    }
}

/// Generates an age to use in a [Galaxy] while following the given [GenerationSettings].
fn generate_age(
    neighborhood: GalacticNeighborhood,
    index: u16,
    seed: &Rc<str>,
    settings: &GenerationSettings,
) -> f32 {
    let mut age_rng = SeededDiceRoller::new(seed.as_ref(), &format!("gal_{}_age", index));
    let age;
    let fixed_age = get_fixed_age(settings);

    // A fixed age is honored when it is valid: non-negative (-1.0 is the "no
    // fixed age" sentinel from `get_fixed_age`) and not older than the
    // universe. Anything else falls back to the rolled age.
    age = if fixed_age >= 0.0 && fixed_age <= neighborhood.universe.age {
        fixed_age
    } else {
        neighborhood.universe.age
            - (if neighborhood.universe.era != StelliferousEra::AncientStelliferous {
                (age_rng.roll(1, 36, 24) as f32) / 100.0
            } else {
                (age_rng.roll(1, 16, 19) as f32) / 100.0
            })
    };
    (age * 100.0).round() / 100.0
}

/// Generates a [Galaxy] category while following the given [GenerationSettings].
fn generate_category(
    neighborhood: GalacticNeighborhood,
    index: u16,
    age: f32,
    is_dominant: bool,
    is_major: bool,
    seed: &Rc<str>,
    settings: &GenerationSettings,
) -> GalaxyCategory {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_cat", index));
    let category;
    if let Some(fixed_category) = settings.galaxy.fixed_category {
        category = fixed_category;
    } else {
        match neighborhood.density {
            GalacticNeighborhoodDensity::Void(_, _) => {
                if is_major {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxyCategory::Intergalactic(0, 0, 0),
                                weight: 1,
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Irregular(0, 0, 0),
                                weight: if age < 1.0 {
                                    13
                                } else if age < 5.0 {
                                    4
                                } else {
                                    1
                                },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Spiral(0, 0),
                                weight: if age < 50.0 { 8 } else { 3 },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Lenticular(0, 0),
                                weight: if age < 50.0 { 4 } else { 7 },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Elliptical(0),
                                weight: if age < 50.0 {
                                    1
                                } else if age < 750.0 {
                                    4
                                } else {
                                    12
                                },
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                } else {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxyCategory::Intergalactic(0, 0, 0),
                                weight: 1,
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Irregular(0, 0, 0),
                                weight: 6,
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                }
            }
            GalacticNeighborhoodDensity::Group(_, _) => {
                if is_major {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxyCategory::Intergalactic(0, 0, 0),
                                weight: if age < 5.0 { 3 } else { 1 },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Irregular(0, 0, 0),
                                weight: if age < 1.0 {
                                    13
                                } else if age < 5.0 {
                                    5
                                } else {
                                    2
                                },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Spiral(0, 0),
                                weight: if age < 50.0 { 9 } else { 3 },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Lenticular(0, 0),
                                weight: if age < 50.0 { 5 } else { 8 },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Elliptical(0),
                                weight: if age < 5.0 {
                                    1
                                } else if age < 50.0 {
                                    3
                                } else if age < 750.0 {
                                    9
                                } else {
                                    12
                                },
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                } else {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxyCategory::Intergalactic(0, 0, 0),
                                weight: 1,
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Irregular(0, 0, 0),
                                weight: 11,
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                }
            }
            GalacticNeighborhoodDensity::Cluster(_, _, _) => {
                if is_dominant {
                    category = GalaxyCategory::DominantElliptical(0);
                } else if is_major {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxyCategory::Intracluster(0, 0, 0),
                                weight: 2,
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Irregular(0, 0, 0),
                                weight: if age < 1.0 {
                                    20
                                } else if age < 5.0 {
                                    5
                                } else {
                                    2
                                },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Spiral(0, 0),
                                weight: if age < 50.0 { 4 } else { 2 },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Lenticular(0, 0),
                                weight: if age < 5.0 { 5 } else { 11 },
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Elliptical(0),
                                weight: if age < 5.0 {
                                    1
                                } else if age < 50.0 {
                                    4
                                } else if age < 750.0 {
                                    10
                                } else {
                                    18
                                },
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                } else {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxyCategory::Intracluster(0, 0, 0),
                                weight: 1,
                            },
                            CopyableWeightedResult {
                                result: GalaxyCategory::Irregular(0, 0, 0),
                                weight: 6,
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                }
            }
        }
    }

    category
}

/// Fills the size parameters of a [Galaxy] category while following the given [GenerationSettings].
fn get_category_with_size(
    category: GalaxyCategory,
    sub_category: GalaxySubCategory,
    index: u16,
    seed: &Rc<str>,
) -> GalaxyCategory {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_cws", index));
    let category_with_size;

    match sub_category {
        GalaxySubCategory::DwarfAmorphous => {
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    rng.roll(1, 390, 9) as u32 * 10,
                    rng.roll(1, 390, 9) as u32 * 10,
                    rng.roll(1, 2925, 74) as u32,
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    rng.roll(1, 390, 9) as u32 * 10,
                    rng.roll(1, 390, 9) as u32 * 10,
                    rng.roll(1, 2925, 74) as u32,
                );
            } else {
                category_with_size = GalaxyCategory::Intracluster(
                    rng.roll(1, 390, 9) as u32 * 10,
                    rng.roll(1, 390, 9) as u32 * 10,
                    rng.roll(1, 2925, 74) as u32,
                );
            }
        }
        GalaxySubCategory::Amorphous => {
            let min_size: u64 = 4000 * 4000 * 3000;
            let mut l = 0;
            let mut w = 0;
            let mut h = 0;
            let mut current_size: u64 = 0;
            // Defensive bound against a pathological seed looping forever:
            // each attempt clears min_size with overwhelming probability, so
            // the cap is unreachable in practice. If it ever fires, the
            // largest roll seen so far is kept as a deterministic fallback.
            // Whenever the cap does not fire, behavior is exactly as before.
            let mut iterations = 0;
            let mut best_size: u64 = 0;
            let mut best_roll = (0, 0, 0);
            while current_size < min_size {
                l = rng.roll(1, 1125, 124) as u32 * 10;
                w = rng.roll(1, 1125, 124) as u32 * 10;
                h = rng.roll(1, 900, 99) as u32 * 10;
                current_size = (l as u64) * (w as u64) * (h as u64);
                if current_size > best_size {
                    best_size = current_size;
                    best_roll = (l, w, h);
                }
                iterations += 1;
                if iterations >= 1000 {
                    (l, w, h) = best_roll;
                    break;
                }
            }
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(l, w, h);
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(l, w, h);
            } else {
                category_with_size = GalaxyCategory::Intracluster(l, w, h);
            }
        }
        GalaxySubCategory::DwarfSpiral => {
            let radius = rng.roll(1, 475, 24) as u32 * 10;
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    radius * 2,
                    radius * 2,
                    (radius * rng.roll(1, 3, 0) as u32 / 100).max(10),
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    radius * 2,
                    radius * 2,
                    (radius * rng.roll(1, 3, 0) as u32 / 100).max(10),
                );
            } else if category == GalaxyCategory::Intracluster(0, 0, 0) {
                category_with_size = GalaxyCategory::Intracluster(
                    radius * 2,
                    radius * 2,
                    (radius * rng.roll(1, 3, 0) as u32 / 100).max(10),
                );
            } else {
                category_with_size = GalaxyCategory::Spiral(
                    radius,
                    (radius * rng.roll(1, 3, 0) as u32 / 100).max(10),
                );
            }
        }
        GalaxySubCategory::FlatSpiral
        | GalaxySubCategory::BarredSpiral
        | GalaxySubCategory::ClassicSpiral => {
            let radius = rng.roll(5, 4, 0) as u32 * 1000;
            category_with_size = GalaxyCategory::Spiral(radius, (radius / 100).max(10));
        }
        GalaxySubCategory::DwarfLenticular => {
            let radius = rng.roll(1, 475, 24) as u32 * 10;
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    radius * 2,
                    radius * 2,
                    (radius * rng.roll(1, 6, 0) as u32 / 100).max(10),
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    radius * 2,
                    radius * 2,
                    (radius * rng.roll(1, 6, 0) as u32 / 100).max(10),
                );
            } else if category == GalaxyCategory::Intracluster(0, 0, 0) {
                category_with_size = GalaxyCategory::Intracluster(
                    radius * 2,
                    radius * 2,
                    (radius * rng.roll(1, 6, 0) as u32 / 100).max(10),
                );
            } else {
                category_with_size = GalaxyCategory::Lenticular(
                    radius,
                    (radius * rng.roll(1, 6, 0) as u32 / 100).max(10),
                );
            }
        }
        GalaxySubCategory::CommonLenticular => {
            let radius = rng.roll(5, 6, 0) as u32 * 1000;
            category_with_size = GalaxyCategory::Lenticular(
                radius,
                (radius * rng.roll(2, 6, 0) as u32 / 100).max(10),
            );
        }
        GalaxySubCategory::GiantLenticular => {
            let radius = rng.roll(1, 31, 29) as u32 * 1000;
            category_with_size = GalaxyCategory::Lenticular(
                radius,
                (radius * rng.roll(3, 6, 0) as u32 / 100).max(10),
            );
        }
        GalaxySubCategory::DwarfElliptical => {
            let radius = rng.roll(1, 170, 4) as u32 * 10;
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    radius * 2,
                    radius * 2,
                    (radius * (rng.roll(10, 3, 0) / 10) as u32).max(5),
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    radius * 2,
                    radius * 2,
                    (radius * (rng.roll(10, 3, 0) / 10) as u32).max(5),
                );
            } else if category == GalaxyCategory::Intracluster(0, 0, 0) {
                category_with_size = GalaxyCategory::Intracluster(
                    radius * 2,
                    radius * 2,
                    (radius * (rng.roll(10, 4, 0) / 20) as u32).max(5),
                );
            } else {
                category_with_size = GalaxyCategory::Elliptical(radius);
            }
        }
        GalaxySubCategory::CommonElliptical => {
            if category == GalaxyCategory::DominantElliptical(0) {
                category_with_size =
                    GalaxyCategory::DominantElliptical(rng.roll(5, 31, 45) as u32 * 1000);
            } else {
                category_with_size = GalaxyCategory::Elliptical(rng.roll(10, 20, 0) as u32 * 100);
            }
        }
        GalaxySubCategory::GiantElliptical => {
            if category == GalaxyCategory::DominantElliptical(0) {
                category_with_size =
                    GalaxyCategory::DominantElliptical(rng.roll(5, 61, 195) as u32 * 1000);
            } else {
                category_with_size = GalaxyCategory::Elliptical(rng.roll(5, 61, 195) as u32 * 100);
            }
        }
    }

    category_with_size
}

/// Generates a [Galaxy] sub-category while following the given [GenerationSettings].
fn generate_sub_category(
    category: GalaxyCategory,
    index: u16,
    age: f32,
    is_major: bool,
    seed: &Rc<str>,
    settings: &GenerationSettings,
) -> GalaxySubCategory {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_sbc", index));
    let sub_category;

    if let Some(fixed_sub_category) = settings.galaxy.fixed_sub_category {
        sub_category = fixed_sub_category;
    } else {
        match category {
            GalaxyCategory::Intergalactic(_, _, _)
            | GalaxyCategory::Intracluster(_, _, _)
            | GalaxyCategory::Irregular(_, _, _) => {
                if is_major {
                    sub_category = GalaxySubCategory::Amorphous;
                } else {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxySubCategory::DwarfAmorphous,
                                weight: if age < 5.0 { 10 } else { 4 },
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::DwarfSpiral,
                                weight: if age < 5.0 {
                                    1
                                } else if age < 50.0 {
                                    4
                                } else {
                                    2
                                },
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::DwarfLenticular,
                                weight: if age < 5.0 {
                                    1
                                } else if age < 50.0 {
                                    2
                                } else {
                                    4
                                },
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::DwarfElliptical,
                                weight: if age < 5.0 {
                                    1
                                } else if age < 50.0 {
                                    2
                                } else {
                                    6
                                },
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    sub_category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                }
            }
            GalaxyCategory::Spiral(_, _) => {
                if is_major {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxySubCategory::FlatSpiral,
                                weight: 2,
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::BarredSpiral,
                                weight: if age > 5.0 && age < 50.0 { 7 } else { 3 },
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::ClassicSpiral,
                                weight: 3,
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    sub_category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                } else {
                    sub_category = GalaxySubCategory::DwarfSpiral;
                }
            }
            GalaxyCategory::Lenticular(_, _) => {
                if is_major {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxySubCategory::CommonLenticular,
                                weight: 7,
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::GiantLenticular,
                                weight: if age < 500.0 { 3 } else { 10 },
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    sub_category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                } else {
                    sub_category = GalaxySubCategory::DwarfLenticular;
                }
            }
            GalaxyCategory::Elliptical(_) => {
                if is_major {
                    let table = CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxySubCategory::CommonElliptical,
                                weight: 5,
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::GiantElliptical,
                                weight: if age < 500.0 { 3 } else { 8 },
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    };
                    // Unreachable fallback (hardcoded non-empty table): first entry.
                    sub_category = rng
                        .get_result(&table)
                        .unwrap_or(table.possible_results[0].result);
                } else {
                    sub_category = GalaxySubCategory::DwarfElliptical;
                }
            }
            GalaxyCategory::DominantElliptical(_) => {
                let table = CopyableRollToProcess {
                    possible_results: vec![
                        CopyableWeightedResult {
                            result: GalaxySubCategory::CommonElliptical,
                            weight: if age < 50.0 { 6 } else { 3 },
                        },
                        CopyableWeightedResult {
                            result: GalaxySubCategory::GiantElliptical,
                            weight: if age < 50.0 {
                                3
                            } else if age < 500.0 {
                                8
                            } else {
                                12
                            },
                        },
                    ],
                    roll_method: RollMethod::SimpleRoll,
                };
                // Unreachable fallback (hardcoded non-empty table): first entry.
                sub_category = rng
                    .get_result(&table)
                    .unwrap_or(table.possible_results[0].result);
            }
        }
    }

    sub_category
}

/// Fills the size parameters of a [Galaxy] category while following the given [GenerationSettings].
fn generate_special_traits(
    neighborhood: GalacticNeighborhood,
    category: GalaxyCategory,
    sub_category: GalaxySubCategory,
    index: u16,
    seed: &Rc<str>,
    settings: &GenerationSettings,
) -> Vec<GalaxySpecialTrait> {
    let mut special_traits = vec![];

    if let Some(fixed_traits) = settings.clone().galaxy.fixed_special_traits {
        special_traits = fixed_traits;
    } else {
        let number_of_random_traits = get_number_of_random_traits(index, seed);

        let mut all_special_traits =
            get_full_list_of_traits(neighborhood, category, sub_category, index, seed);
        all_special_traits =
            remove_forbidden_traits(category, sub_category, settings, &all_special_traits);

        special_traits = add_age_related_traits(neighborhood, &mut special_traits, index, seed);
        special_traits = add_random_traits(
            number_of_random_traits,
            all_special_traits,
            &mut special_traits,
            index,
            seed,
        );
    }

    clean_special_traits(&mut special_traits)
}

/// Returns the complete list of traits a galaxy might have.
fn get_full_list_of_traits(
    neighborhood: GalacticNeighborhood,
    category: GalaxyCategory,
    sub_category: GalaxySubCategory,
    index: u16,
    seed: &Rc<str>,
) -> Vec<CopyableWeightedResult<GalaxySpecialTrait>> {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_gsp", index));
    let all_special_traits: Vec<CopyableWeightedResult<GalaxySpecialTrait>> = vec![
        CopyableWeightedResult {
            result: GalaxySpecialTrait::NoPeculiarity,
            weight: if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                1
            } else {
                4
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::ActiveNucleus,
            weight: if discriminant(&category) == discriminant(&GalaxyCategory::Elliptical(0))
                || discriminant(&category) == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                10
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::DoubleNuclei,
            weight: 1,
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Compact(
                // Unreachable fallback (hardcoded non-empty table): first entry.
                rng.get_result(&CopyableRollToProcess {
                    possible_results: vec![
                        CopyableWeightedResult {
                            result: 200,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 150,
                            weight: 3,
                        },
                        CopyableWeightedResult {
                            result: 120,
                            weight: 6,
                        },
                    ],
                    roll_method: RollMethod::SimpleRoll,
                })
                .unwrap_or(200),
            ),
            weight: if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                20
            } else if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                3
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Dusty,
            weight: if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                2
            } else if discriminant(&category) == discriminant(&GalaxyCategory::Lenticular(0, 0))
                || discriminant(&category) == discriminant(&GalaxyCategory::Spiral(0, 0))
            {
                20
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Expansive(
                // Unreachable fallback (hardcoded non-empty table): first entry.
                rng.get_result(&CopyableRollToProcess {
                    possible_results: vec![
                        CopyableWeightedResult {
                            result: 20,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 50,
                            weight: 3,
                        },
                        CopyableWeightedResult {
                            result: 75,
                            weight: 6,
                        },
                    ],
                    roll_method: RollMethod::SimpleRoll,
                })
                .unwrap_or(20),
            ),
            weight: if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                2
            } else if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                7
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::ExtendedHalo,
            weight: if discriminant(&neighborhood.density)
                == discriminant(&GalacticNeighborhoodDensity::Cluster(0, 0, 0))
                || discriminant(&category) == discriminant(&GalaxyCategory::Lenticular(0, 0))
                || discriminant(&category) == discriminant(&GalaxyCategory::Spiral(0, 0))
            {
                10
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::GasPoor,
            weight: if discriminant(&category) == discriminant(&GalaxyCategory::Lenticular(0, 0)) {
                10
            } else if discriminant(&category) == discriminant(&GalaxyCategory::Elliptical(0)) {
                15
            } else if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                3
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::GasRich,
            weight: if discriminant(&category) == discriminant(&GalaxyCategory::Irregular(0, 0, 0))
            {
                20
            } else if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                10
            } else if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                2
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Interacting,
            weight: if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                7
            } else if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
                || sub_category == GalaxySubCategory::GiantLenticular
                || sub_category == GalaxySubCategory::GiantElliptical
                || discriminant(&category) == discriminant(&GalaxyCategory::Irregular(0, 0, 0))
            {
                10
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::MetalPoor,
            weight: if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                20
            } else if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                3
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Older,
            weight: if discriminant(&category) == discriminant(&GalaxyCategory::Irregular(0, 0, 0))
                || discriminant(&category) == discriminant(&GalaxyCategory::Intergalactic(0, 0, 0))
                || discriminant(&category) == discriminant(&GalaxyCategory::Intracluster(0, 0, 0))
                || sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                20
            } else {
                10
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Satellites(
                // Unreachable fallback (hardcoded non-empty table): first entry.
                rng.get_result(&CopyableRollToProcess {
                    possible_results: vec![
                        CopyableWeightedResult {
                            result: GalaxySatellites::MuchMore,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: GalaxySatellites::More,
                            weight: 3,
                        },
                        CopyableWeightedResult {
                            result: GalaxySatellites::Less,
                            weight: 3,
                        },
                        CopyableWeightedResult {
                            result: GalaxySatellites::MuchLess,
                            weight: 2,
                        },
                        CopyableWeightedResult {
                            result: GalaxySatellites::None,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: GalaxySatellites::Special,
                            weight: 1,
                        },
                    ],
                    roll_method: RollMethod::SimpleRoll,
                })
                .unwrap_or(GalaxySatellites::MuchMore),
            ),
            weight: if discriminant(&category) == discriminant(&GalaxyCategory::Spiral(0, 0))
                && sub_category != GalaxySubCategory::DwarfSpiral
            {
                10
            } else if discriminant(&category) == discriminant(&GalaxyCategory::Elliptical(0))
                && sub_category != GalaxySubCategory::GiantElliptical
            {
                8
            } else if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                10
            } else if sub_category != GalaxySubCategory::GiantElliptical
                && sub_category != GalaxySubCategory::GiantLenticular
            {
                23
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Starburst,
            weight: if sub_category == GalaxySubCategory::Amorphous {
                20
            } else if discriminant(&category) == discriminant(&GalaxyCategory::Spiral(0, 0))
                || sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                5
            } else {
                2
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::SubSize(
                // Unreachable fallback (hardcoded non-empty table): first entry.
                rng.get_result(&CopyableRollToProcess {
                    possible_results: vec![
                        CopyableWeightedResult {
                            result: 20,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 30,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 50,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 75,
                            weight: 1,
                        },
                    ],
                    roll_method: RollMethod::SimpleRoll,
                })
                .unwrap_or(20),
            ),
            weight: if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                10
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::SuperSize(
                // Unreachable fallback (hardcoded non-empty table): first entry.
                rng.get_result(&CopyableRollToProcess {
                    possible_results: vec![
                        CopyableWeightedResult {
                            result: 150,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 200,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 300,
                            weight: 1,
                        },
                        CopyableWeightedResult {
                            result: 500,
                            weight: 1,
                        },
                    ],
                    roll_method: RollMethod::SimpleRoll,
                })
                .unwrap_or(150),
            ),
            weight: if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
            {
                25
            } else if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::Amorphous
            {
                2
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Younger,
            weight: if discriminant(&category)
                == discriminant(&GalaxyCategory::DominantElliptical(0))
                || (sub_category != GalaxySubCategory::GiantElliptical
                    && sub_category != GalaxySubCategory::GiantLenticular)
            {
                2
            } else {
                5
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Dead,
            weight: 1,
        },
        // This galaxy has lost too much of its gas while interacting with other galaxies and is no longer able to produce new stars.
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Dormant,
            weight: if discriminant(&neighborhood.density)
                == discriminant(&GalacticNeighborhoodDensity::Cluster(0, 0, 0))
            {
                3
            } else if discriminant(&neighborhood.density)
                == discriminant(&GalacticNeighborhoodDensity::Group(0, 0))
            {
                2
            } else {
                1
            },
        },
        CopyableWeightedResult {
            result: GalaxySpecialTrait::Tail,
            weight: if sub_category == GalaxySubCategory::DwarfAmorphous
                || sub_category == GalaxySubCategory::DwarfElliptical
                || sub_category == GalaxySubCategory::DwarfLenticular
                || sub_category == GalaxySubCategory::DwarfSpiral
            {
                3
            } else {
                1
            },
        },
    ];
    all_special_traits
}

/// Removes any eventual traits marked as forbidden in the [GenerationSettings].
fn remove_forbidden_traits(
    category: GalaxyCategory,
    sub_category: GalaxySubCategory,
    settings: &GenerationSettings,
    all_special_traits: &Vec<CopyableWeightedResult<GalaxySpecialTrait>>,
) -> Vec<CopyableWeightedResult<GalaxySpecialTrait>> {
    let mut special_traits =
        if let Some(traits_to_remove) = settings.clone().galaxy.forbidden_special_traits {
            let mut temp_special_traits = vec![];
            all_special_traits.iter().for_each(|t| {
                if traits_to_remove
                    .iter()
                    .find(|to_remove| discriminant(*to_remove) == discriminant(&t.result))
                    .is_none()
                {
                    temp_special_traits.push(*t);
                }
            });
            temp_special_traits
        } else {
            all_special_traits.to_vec()
        };

    if sub_category == GalaxySubCategory::DwarfAmorphous
        || sub_category == GalaxySubCategory::DwarfElliptical
        || sub_category == GalaxySubCategory::DwarfLenticular
        || sub_category == GalaxySubCategory::DwarfSpiral
    {
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::ActiveNucleus);
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::DoubleNuclei);
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::ExtendedHalo);
        remove_specific_possible_trait(
            &mut special_traits,
            GalaxySpecialTrait::Satellites(GalaxySatellites::None),
        );
    } else if sub_category == GalaxySubCategory::GiantElliptical
        || sub_category == GalaxySubCategory::GiantLenticular
        || discriminant(&category) == discriminant(&GalaxyCategory::DominantElliptical(0))
    {
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::SubSize(0));
    } else if discriminant(&category) == discriminant(&GalaxyCategory::Elliptical(0)) {
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::GasRich);
    } else if discriminant(&category) == discriminant(&GalaxyCategory::Lenticular(0, 0)) {
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::GasRich);
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::MetalPoor);
    } else if sub_category == GalaxySubCategory::Amorphous {
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::ActiveNucleus);
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::ExtendedHalo);
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::GasPoor);
        remove_specific_possible_trait(&mut special_traits, GalaxySpecialTrait::MetalPoor);
    }

    special_traits
}

/// Adds [GalaxySpecialTrait]s to the given **list_to_fill** according to the universe's age.
fn add_age_related_traits(
    neighborhood: GalacticNeighborhood,
    list_to_fill: &mut Vec<GalaxySpecialTrait>,
    index: u16,
    seed: &Rc<str>,
) -> Vec<GalaxySpecialTrait> {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_spa", index));
    match neighborhood.universe.era {
        StelliferousEra::AncientStelliferous | StelliferousEra::EarlyStelliferous => {
            if neighborhood.universe.age < 1.5 || rng.roll(1, 3, 0) == 1 {
                list_to_fill.push(GalaxySpecialTrait::Younger);
            }
        }
        StelliferousEra::MiddleStelliferous => (),
        StelliferousEra::LateStelliferous | StelliferousEra::EndStelliferous => {
            if neighborhood.universe.age > 1500.0 || rng.roll(1, 3, 0) == 1 {
                list_to_fill.push(GalaxySpecialTrait::Older);
            }
            if neighborhood.universe.age > 50000.0 {
                list_to_fill.push(GalaxySpecialTrait::Dead);
            }
        }
    }
    list_to_fill.to_vec()
}

/// Adds **to_add** traits from the given list of **possible_traits** to an existing **list_to_fill**.
fn add_random_traits(
    to_add: i32,
    mut possible_traits: Vec<CopyableWeightedResult<GalaxySpecialTrait>>,
    list_to_fill: &mut Vec<GalaxySpecialTrait>,
    index: u16,
    seed: &Rc<str>,
) -> Vec<GalaxySpecialTrait> {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_art", index));
    let opposite_traits = get_opposite_traits();
    let mut turn = 0;
    while turn < to_add {
        // Termination guards (pure scans, no RNG consumed): stop when no
        // candidate remains, or when every remaining candidate is already in
        // the list — in both cases no further roll could ever be added.
        if possible_traits.is_empty()
            || possible_traits.iter().all(|possible_trait| {
                list_to_fill.iter().any(|current_trait| {
                    discriminant(&possible_trait.result) == discriminant(current_trait)
                })
            })
        {
            break;
        }
        let entry_found = rng.get_result(&CopyableRollToProcess {
            possible_results: possible_traits.clone(),
            roll_method: RollMethod::SimpleRoll,
        });
        if let Some(possible_trait) = entry_found {
            if list_to_fill
                .iter()
                .find(|current_trait| discriminant(&possible_trait) == discriminant(current_trait))
                .is_none()
            {
                turn += 1;
                remove_opposite_traits(possible_trait, &opposite_traits, list_to_fill);
                list_to_fill.push(possible_trait);
                remove_specific_possible_trait(&mut possible_traits, possible_trait);
            }
        }
    }
    list_to_fill.to_vec()
}

/// Returns true if the given **list_to_fill** contains traits incompatible with the given **possible_trait**, using the **opposite_traits_list** as a reference to determine what is compatible or not.
fn remove_opposite_traits(
    possible_trait: GalaxySpecialTrait,
    opposite_traits_list: &Vec<OppositeTraits>,
    list_to_fill: &mut Vec<GalaxySpecialTrait>,
) {
    // Find a list of opposite traits to the given **possible_trait**
    let mut possible_oppposites: Option<Vec<GalaxySpecialTrait>> = None;
    opposite_traits_list.iter().for_each(|pair| {
        if possible_oppposites.is_none() {
            if pair
                .0
                .iter()
                .find(|current_trait| discriminant(*current_trait) == discriminant(&possible_trait))
                .is_some()
            {
                possible_oppposites = Some(pair.1.clone());
            } else if pair
                .1
                .iter()
                .find(|current_trait| discriminant(*current_trait) == discriminant(&possible_trait))
                .is_some()
            {
                possible_oppposites = Some(pair.0.clone());
            }
        }
    });

    // For each opposite, if it is present in the **list_to_fill**, removes it
    if let Some(opposites) = possible_oppposites {
        opposites.iter().for_each(|opposite| {
            let possible_found = list_to_fill
                .iter()
                .find(|current_trait| discriminant(*current_trait) == discriminant(&opposite));
            if let Some(found) = possible_found {
                remove_specific_trait(list_to_fill, *found);
            }
        });
    }
}

/// If no traits, adds the "NoPeculiarity" one, otherwise checks if it's present and removes it.
fn clean_special_traits(special_traits: &mut Vec<GalaxySpecialTrait>) -> Vec<GalaxySpecialTrait> {
    if special_traits.len() < 1 {
        special_traits.push(GalaxySpecialTrait::NoPeculiarity);
    } else if special_traits.len() > 1
        && special_traits
            .iter()
            .find(|current_trait| {
                discriminant(*current_trait) == discriminant(&GalaxySpecialTrait::NoPeculiarity)
            })
            .is_some()
    {
        remove_specific_trait(special_traits, GalaxySpecialTrait::NoPeculiarity);
    }
    special_traits.to_vec()
}

/// Removes an entry from the given list of possible traits.
fn remove_specific_possible_trait(
    possible_traits: &mut Vec<CopyableWeightedResult<GalaxySpecialTrait>>,
    possible_trait: GalaxySpecialTrait,
) {
    let possible_index = possible_traits
        .iter()
        .position(|r| discriminant(&r.result) == discriminant(&possible_trait));
    if let Some(index) = possible_index {
        possible_traits.remove(index);
    }
}

/// Removes an entry from the given list of traits.
fn remove_specific_trait(traits: &mut Vec<GalaxySpecialTrait>, possible_trait: GalaxySpecialTrait) {
    let possible_index = traits
        .iter()
        .position(|r| discriminant(r) == discriminant(&possible_trait));
    if let Some(index) = possible_index {
        traits.remove(index);
    }
}

/// Calculates the number of random traits this galaxy will have.
fn get_number_of_random_traits(index: u16, seed: &Rc<str>) -> i32 {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_srt", index));
    let mut number_of_random_traits = 0;
    let mut roll = 0;
    let mut turn = 0;
    while roll == 50 || turn < 1 {
        roll = rng.roll(1, 50, 0) as u8;
        number_of_random_traits += if roll < 10 { 0 } else { 1 };
        turn += 1;
    }
    number_of_random_traits
}

/// Retreives an age to use in [Galaxy] generation from the given [GenerationSettings].
/// Returns -1.0 as a sentinel when no fixed age was set.
fn get_fixed_age(settings: &GenerationSettings) -> f32 {
    settings.galaxy.fixed_age.unwrap_or(-1.0)
}

/// Is the [Galaxy] to be generated a dominant one in its local cluster?
fn is_galaxy_dominant(neighborhood: GalacticNeighborhood, index: u16) -> bool {
    let is_dominant;
    match neighborhood.density {
        GalacticNeighborhoodDensity::Void(_, _) => is_dominant = false,
        GalacticNeighborhoodDensity::Group(_, _) => is_dominant = false,
        GalacticNeighborhoodDensity::Cluster(dominant, _, _) => {
            is_dominant = index < dominant as u16;
        }
    }
    is_dominant
}

/// Is the [Galaxy] to be generated a major one in its local neighborhood?
fn is_galaxy_major(neighborhood: GalacticNeighborhood, index: u16) -> bool {
    let is_major;
    match neighborhood.density {
        GalacticNeighborhoodDensity::Void(galaxies, _) => {
            is_major = index < galaxies as u16;
        }
        GalacticNeighborhoodDensity::Group(galaxies, _) => {
            is_major = index < galaxies as u16;
        }
        GalacticNeighborhoodDensity::Cluster(dominant, galaxies, _) => {
            is_major = index < (dominant + galaxies) as u16;
        }
    }
    is_major
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_a_galaxy_with_sensible_age() {
        for i in 0..10000 {
            let seed: Rc<str> = Rc::from(i.to_string());
            let settings = &GenerationSettings {
                seed,
                galaxy: GalaxySettings {
                    ..Default::default()
                },
                ..Default::default()
            };
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&settings, &mut Diagnostics::default()),
                &settings,
                &mut Diagnostics::default(),
            );
            let galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &settings);
            assert!(galaxy.age <= neighborhood.universe.age - 0.2)
        }
    }

    /// Returns the physically-plausible size envelope (in parsecs, inclusive)
    /// for every component of the given category. These bounds are deliberately
    /// much wider than the current dice tables: they catch unit slips (parsecs
    /// vs kiloparsecs), zero/negative sizes and impossible values, while leaving
    /// the tables free to be rebalanced without touching this test.
    fn size_envelope(category: &GalaxyCategory) -> (u32, u32) {
        match category {
            // From dwarf clouds/disks/ellipticals (below a kiloparsec) up to
            // supercluster-filament scale (~100 kpc): loose clouds, disks and
            // ordinary ellipticals all live inside this envelope.
            GalaxyCategory::Intergalactic(..)
            | GalaxyCategory::Irregular(..)
            | GalaxyCategory::Intracluster(..)
            | GalaxyCategory::Spiral(..)
            | GalaxyCategory::Lenticular(..)
            | GalaxyCategory::Elliptical(_) => (1, 100_000),
            // Dominant (cD) ellipticals are the largest galaxies known: never
            // below a few kiloparsecs, never beyond a megaparsec.
            GalaxyCategory::DominantElliptical(_) => (10_000, 1_000_000),
        }
    }

    #[test]
    fn generate_a_galaxy_with_proper_size() {
        for i in 0u16..10000 {
            let settings = &GenerationSettings {
                seed: Rc::from(i.to_string()),
                ..Default::default()
            };
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(settings, &mut Diagnostics::default()),
                settings,
                &mut Diagnostics::default(),
            );
            let galaxy = Galaxy::generate(neighborhood, i % 5, settings);

            // Every size component is positive and within the category's
            // physically-plausible envelope (see size_envelope).
            let (min, max) = size_envelope(&galaxy.category);
            let components: Vec<u32> = match galaxy.category {
                GalaxyCategory::Intergalactic(l, w, h)
                | GalaxyCategory::Irregular(l, w, h)
                | GalaxyCategory::Intracluster(l, w, h) => vec![l, w, h],
                GalaxyCategory::Spiral(r, t) | GalaxyCategory::Lenticular(r, t) => vec![r, t],
                GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => vec![r],
            };
            for component in components {
                assert!(
                    (min..=max).contains(&component),
                    "Size component {component} pc is outside the [{min}, {max}] pc envelope \
                     for category {} (sub-category {})",
                    galaxy.category,
                    galaxy.sub_category
                );
            }

            // Determinism: regenerating from the same seed and index must
            // produce the exact same category, sub-category and sizes.
            let regenerated = Galaxy::generate(neighborhood, i % 5, settings);
            assert_eq!(galaxy.category, regenerated.category);
            assert_eq!(galaxy.sub_category, regenerated.sub_category);
        }
    }

    #[test]
    fn generate_with_empty_layers_uses_category_default() {
        let settings = &GenerationSettings {
            seed: "map-default".into(),
            ..Default::default()
        };
        let neighborhood = GalacticNeighborhood::generate(
            Universe::generate(settings, &mut Diagnostics::default()),
            settings,
            &mut Diagnostics::default(),
        );
        let galaxy = Galaxy::generate(neighborhood, 0, settings);
        assert!(
            !galaxy.galactic_map_layers.is_empty(),
            "An empty layers setting should be replaced by the category's default map."
        );
        assert!(
            galaxy.region_map.is_some(),
            "The default map should have been built during generation."
        );
    }

    #[test]
    fn generate_with_user_layers_keeps_them() {
        // A one-layer map painting the whole galaxy in Arm blue.
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="#0000FF"/></svg>"##.to_string();
        let settings = &GenerationSettings {
            seed: "map-user".into(),
            galaxy: GalaxySettings {
                fixed_category: Some(GalaxyCategory::Irregular(500, 500, 100)),
                galactic_map_layers: vec![svg.clone()],
                ..Default::default()
            },
            ..Default::default()
        };
        let neighborhood = GalacticNeighborhood::generate(
            Universe::generate(settings, &mut Diagnostics::default()),
            settings,
            &mut Diagnostics::default(),
        );
        let galaxy = Galaxy::generate(neighborhood, 0, settings);
        assert_eq!(
            galaxy.galactic_map_layers,
            vec![svg],
            "Valid user layers should be kept as-is."
        );
        let map = galaxy
            .region_map
            .as_ref()
            .expect("The user's map should have been built during generation.");
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Arm
        );
    }

    #[test]
    fn generate_with_broken_layers_falls_back_to_default() {
        let settings = &GenerationSettings {
            seed: "map-broken".into(),
            galaxy: GalaxySettings {
                fixed_category: Some(GalaxyCategory::Spiral(10000, 100)),
                galactic_map_layers: vec!["not an svg".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let neighborhood = GalacticNeighborhood::generate(
            Universe::generate(settings, &mut Diagnostics::default()),
            settings,
            &mut Diagnostics::default(),
        );
        let galaxy = Galaxy::generate(neighborhood, 0, settings);
        assert_eq!(
            galaxy.galactic_map_layers,
            default_layers_for(galaxy.category),
            "Unparseable user layers should be replaced by the category's default map."
        );
        assert!(
            galaxy.region_map.is_some(),
            "The default map should have been built as a fallback."
        );
        assert!(
            galaxy
                .diagnostics
                .warnings
                .contains(&GenerationWarning::SvgLayerParseFailed { layer_index: 0 }),
            "The parse failure should be reported in the galaxy's diagnostics."
        );
        assert!(
            galaxy
                .diagnostics
                .warnings
                .contains(&GenerationWarning::UserLayersReplacedWithDefault),
            "The fallback to the default layers should be reported in the galaxy's diagnostics."
        );
    }

    /// Generates a galaxy with the given settings and returns it, building its
    /// neighborhood the standard way.
    fn generate_galaxy_for(settings: &GenerationSettings, index: u16) -> Galaxy {
        let neighborhood = GalacticNeighborhood::generate(
            Universe::generate(settings, &mut Diagnostics::default()),
            settings,
            &mut Diagnostics::default(),
        );
        Galaxy::generate(neighborhood, index, settings)
    }

    #[test]
    fn use_ours_local_group_sweep_has_real_named_sane_galaxies() {
        // The LOCAL_GROUP_GALAXIES table feeds every use_ours galaxy at
        // index < 38: each entry must carry a real, unique name (no leftover
        // "TODO" placeholders), flags consistent with the Group(2, 36) density
        // (indices 0-1 major, everything else minor, none dominant), and a sane
        // size in parsecs.
        let settings = &GenerationSettings {
            seed: Rc::from("local-group-sweep"),
            universe: UniverseSettings {
                use_ours: true,
                ..Default::default()
            },
            galaxy: GalaxySettings {
                use_ours: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut names = std::collections::BTreeSet::new();
        for index in 0..u16::try_from(LOCAL_GROUP_GALAXIES.len())
            .expect("The local-group table is far smaller than u16::MAX.")
        {
            let galaxy = generate_galaxy_for(settings, index);
            assert_ne!(
                "TODO",
                galaxy.name.as_ref(),
                "local-group entry #{index} is still a placeholder"
            );
            assert!(
                names.insert(galaxy.name.clone()),
                "duplicate local-group galaxy name: {}",
                galaxy.name
            );
            assert!(
                !galaxy.is_dominant,
                "{} should not be dominant in a Group",
                galaxy.name
            );
            assert_eq!(
                index < 2,
                galaxy.is_major,
                "{} (index {index}) should be major iff index < 2",
                galaxy.name
            );
            let largest_dimension = match galaxy.category {
                GalaxyCategory::Intergalactic(l, w, h)
                | GalaxyCategory::Irregular(l, w, h)
                | GalaxyCategory::Intracluster(l, w, h) => l.max(w).max(h),
                GalaxyCategory::Spiral(r, _) | GalaxyCategory::Lenticular(r, _) => r * 2,
                GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => r * 2,
            };
            assert!(
                largest_dimension > 0,
                "{} has a zero-sized category",
                galaxy.name
            );
            // No Local Group member exceeds Andromeda's ~47kpc span; 50kpc leaves slack.
            assert!(
                largest_dimension <= 50_000,
                "{} has an implausible size of {largest_dimension} pc",
                galaxy.name
            );
        }
    }

    #[test]
    // Exact equality is intended: the fixed age is assigned as-is.
    #[allow(clippy::float_cmp)]
    fn fixed_age_is_honored_when_valid() {
        // C6: a fixed age within [0, universe age] must be used as-is.
        for i in 0..100u16 {
            let settings = &GenerationSettings {
                seed: Rc::from(i.to_string()),
                universe: UniverseSettings {
                    use_ours: true,
                    ..Default::default()
                },
                galaxy: GalaxySettings {
                    fixed_age: Some(10.0),
                    ..Default::default()
                },
                ..Default::default()
            };
            let galaxy = generate_galaxy_for(settings, i % 5);
            assert_eq!(10.0, galaxy.age);
        }
    }

    #[test]
    fn fixed_age_falls_back_to_a_rolled_age_when_invalid() {
        // C6: fixed ages older than the universe or negative are rejected; the
        // usual roll applies (a rolled galaxy age is <= universe age - 0.2).
        for invalid in [20.0, -1.5] {
            for i in 0..100u16 {
                let settings = &GenerationSettings {
                    seed: Rc::from(i.to_string()),
                    universe: UniverseSettings {
                        use_ours: true,
                        ..Default::default()
                    },
                    galaxy: GalaxySettings {
                        fixed_age: Some(invalid),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let galaxy = generate_galaxy_for(settings, i % 5);
                assert!(galaxy.age <= 13.8 - 0.2);
            }
        }
    }

    #[test]
    fn generate_terminates_when_all_special_traits_are_forbidden() {
        // K9: forbidding every trait empties the candidate list; generation
        // must still terminate instead of spinning on un-addable rolls.
        let all_traits = vec![
            GalaxySpecialTrait::NoPeculiarity,
            GalaxySpecialTrait::ActiveNucleus,
            GalaxySpecialTrait::DoubleNuclei,
            GalaxySpecialTrait::Compact(0),
            GalaxySpecialTrait::Expansive(0),
            GalaxySpecialTrait::ExtendedHalo,
            GalaxySpecialTrait::MetalPoor,
            GalaxySpecialTrait::Dusty,
            GalaxySpecialTrait::GasPoor,
            GalaxySpecialTrait::GasRich,
            GalaxySpecialTrait::Starburst,
            GalaxySpecialTrait::Dead,
            GalaxySpecialTrait::Dormant,
            GalaxySpecialTrait::Satellites(GalaxySatellites::None),
            GalaxySpecialTrait::Interacting,
            GalaxySpecialTrait::Tail,
            GalaxySpecialTrait::Younger,
            GalaxySpecialTrait::Older,
            GalaxySpecialTrait::SubSize(0),
            GalaxySpecialTrait::SuperSize(0),
        ];
        let settings = &GenerationSettings {
            seed: "forbidden-all".into(),
            galaxy: GalaxySettings {
                forbidden_special_traits: Some(all_traits),
                ..Default::default()
            },
            ..Default::default()
        };
        let galaxy = generate_galaxy_for(settings, 0);
        assert_eq!(
            vec![GalaxySpecialTrait::NoPeculiarity],
            galaxy.special_traits
        );
    }

    /// Returns the weight of the given trait in the full candidate table for
    /// the given galaxy (sub-)category.
    fn trait_weight(
        category: GalaxyCategory,
        sub_category: GalaxySubCategory,
        reference: GalaxySpecialTrait,
    ) -> u32 {
        let neighborhood = GalacticNeighborhood {
            universe: Universe {
                era: StelliferousEra::MiddleStelliferous,
                age: 13.8,
            },
            density: GalacticNeighborhoodDensity::Group(2, 23),
        };
        get_full_list_of_traits(neighborhood, category, sub_category, 0, &Rc::from("k10"))
            .iter()
            .find(|t| discriminant(&t.result) == discriminant(&reference))
            .map(|t| t.weight)
            .expect("The trait should be in the full candidate list.")
    }

    #[test]
    fn giant_subcategories_use_the_giant_trait_weights() {
        // K10: the "not a giant" conditions used to be always-true
        // (`sub != A || sub != B`), so every galaxy got the non-giant weight.
        let satellites = GalaxySpecialTrait::Satellites(GalaxySatellites::None);
        // Non-dominant giants fall through to the giant arm.
        assert_eq!(
            5,
            trait_weight(
                GalaxyCategory::Elliptical(0),
                GalaxySubCategory::GiantElliptical,
                satellites
            )
        );
        assert_eq!(
            5,
            trait_weight(
                GalaxyCategory::Lenticular(0, 0),
                GalaxySubCategory::GiantLenticular,
                satellites
            )
        );
        // Non-giants and dominant ellipticals keep their existing weights.
        assert_eq!(
            8,
            trait_weight(
                GalaxyCategory::Elliptical(0),
                GalaxySubCategory::CommonElliptical,
                satellites
            )
        );
        assert_eq!(
            10,
            trait_weight(
                GalaxyCategory::DominantElliptical(0),
                GalaxySubCategory::GiantElliptical,
                satellites
            )
        );
        assert_eq!(
            23,
            trait_weight(
                GalaxyCategory::Lenticular(0, 0),
                GalaxySubCategory::CommonLenticular,
                satellites
            )
        );

        // Younger: dominant ellipticals and non-giants get 2, non-dominant
        // giants get 5.
        assert_eq!(
            2,
            trait_weight(
                GalaxyCategory::Elliptical(0),
                GalaxySubCategory::CommonElliptical,
                GalaxySpecialTrait::Younger
            )
        );
        assert_eq!(
            2,
            trait_weight(
                GalaxyCategory::DominantElliptical(0),
                GalaxySubCategory::GiantElliptical,
                GalaxySpecialTrait::Younger
            )
        );
        assert_eq!(
            5,
            trait_weight(
                GalaxyCategory::Elliptical(0),
                GalaxySubCategory::GiantElliptical,
                GalaxySpecialTrait::Younger
            )
        );
        assert_eq!(
            5,
            trait_weight(
                GalaxyCategory::Lenticular(0, 0),
                GalaxySubCategory::GiantLenticular,
                GalaxySpecialTrait::Younger
            )
        );
    }
}
