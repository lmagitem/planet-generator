#[cfg(test)]
use crate::diagnostics::types::Diagnostics;
use crate::diagnostics::types::GenerationError;
use crate::diagnostics::types::GenerationWarning;
use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::types::GalacticRegion;
#[cfg(test)]
use crate::galaxy::neighborhood::GalacticNeighborhood;
#[cfg(test)]
use crate::galaxy::types::GalaxySettings;
use crate::galaxy::Galaxy;
#[cfg(test)]
use crate::generator::types::GenerationSettings;
use crate::system::neighborhood::types::StellarNeighborhoodAge;
use crate::system::neighborhood::StellarNeighborhood;
#[cfg(test)]
use crate::universe::Universe;
use seeded_dice_roller::{
    CopyableRollToProcess, CopyableWeightedResult, PreparedRoll, RollMethod, SeededDiceRoller,
};
#[cfg(test)]
use std::rc::Rc;

impl StellarNeighborhood {
    /// Generates a [`StellarNeighborhood`] using the given parameters.
    ///
    /// # Errors
    /// Returns [`GenerationError::InvalidCoordinates`] if the coordinates are outside the galaxy.
    pub fn generate(coord: SpaceCoordinates, galaxy: &mut Galaxy) -> Result<Self, GenerationError> {
        Ok(Self {
            age: generate_age(coord, galaxy)?,
        })
    }
}

/// Uses data about the [GalacticMapDivision]s a neighborhood belongs to to generate its age.
fn generate_age(
    coord: SpaceCoordinates,
    galaxy: &mut Galaxy,
) -> Result<StellarNeighborhoodAge, GenerationError> {
    let divisions = galaxy.get_divisions_for_coord(coord)?;
    let mut regions = Vec::new();
    divisions.iter().for_each(|div| {
        if regions.iter().find(|r| **r == div.region).is_none() {
            regions.push(div.region.clone());
        }
    });
    // Extracted before `divisions`'s borrow of `galaxy` ends, since the seed
    // below needs `&galaxy.settings`. A level-1 division always exists:
    // `get_divisions_for_coord` returns one division per level 0..=9.
    let sub_sector_index = divisions
        .iter()
        .find(|div| div.level == 1)
        .ok_or(GenerationError::InvalidDivisionLevel(1))?
        .index;
    let mut modifier = 0;
    regions.iter().for_each(|region| match region {
        GalacticRegion::GlobularCluster
        | GalacticRegion::Halo
        | GalacticRegion::Aura
        | GalacticRegion::Void
        // Exile joins the sparse-outskirts arm: stars lost into the void skew old,
        // like the Halo. The sign is positive here because a higher prepared roll
        // lands on OLDER entries in the weighted table below (unlike the
        // StellarEvolution table in system/generator.rs, where older outcomes sit
        // at the low end of the roll and this same arm is negative there).
        | GalacticRegion::Exile => modifier += 5,
        GalacticRegion::Stream => modifier += 3,
        GalacticRegion::Core | GalacticRegion::Bulge => modifier += 2,
        GalacticRegion::Disk | GalacticRegion::Ellipse => modifier += 1,
        GalacticRegion::Nucleus => modifier -= 1,
        GalacticRegion::Bar | GalacticRegion::Arm => modifier -= 2,
        GalacticRegion::Association => modifier -= 3,
        GalacticRegion::OpenCluster => modifier -= 5,
        // Multiple aggregates several heterogeneous regions, so it is neutral by design.
        GalacticRegion::Multiple => (),
    });

    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("ste_nei_{sub_sector_index}_age"),
    );
    let table = CopyableRollToProcess {
        possible_results: vec![
            CopyableWeightedResult {
                result: StellarNeighborhoodAge::Young(0),
                weight: 1,
            },
            CopyableWeightedResult {
                result: StellarNeighborhoodAge::Mature,
                weight: 6,
            },
            CopyableWeightedResult {
                result: StellarNeighborhoodAge::Old(0),
                weight: 4,
            },
            CopyableWeightedResult {
                result: StellarNeighborhoodAge::Ancient(0),
                weight: 1,
            },
        ],
        roll_method: RollMethod::PreparedRoll(PreparedRoll {
            dice: 1,
            die_type: 8,
            modifier,
        }),
    };
    // A region modifier outside (-8, 12) makes the prepared 1d8 roll land outside every
    // weighted range (reachable with aggressively painted custom region maps): fall back
    // to the first entry instead of panicking.
    let mut age = rng.get_result(&table).unwrap_or_else(|| {
        galaxy
            .diagnostics
            .push_warning(GenerationWarning::RngTableFallback(
                "stellar neighborhood age",
            ));
        table.possible_results[0].result
    });

    let universe_age = ((galaxy.neighborhood.universe.age * 1000.0) as u64).saturating_sub(200);
    match age {
        StellarNeighborhoodAge::Young(_) => {
            let mut years = 0;
            let mut roll = 0;
            let mut turn = 0;
            let mut divide_by = 1;
            while roll == 1 || roll == 10 || turn < 1 {
                roll = rng.roll(1, 10, 0) as u64;
                years += if turn == 0 || roll == 10 { roll } else { 0 };
                divide_by *= if roll == 1 { 10 } else { 1 };
                turn += 1;
            }
            years = years * 100 / divide_by;
            age = StellarNeighborhoodAge::Young(1.max(years.min(universe_age)));
        }
        StellarNeighborhoodAge::Mature => (),
        StellarNeighborhoodAge::Old(_) => {
            let mut roll = 0;
            let mut turn = 0;
            let mut divide_by = 1;
            while roll == 10 || turn < 1 {
                roll = rng.roll(1, 10, 0) as u64;
                divide_by += roll;
                turn += 1;
            }
            age = StellarNeighborhoodAge::Old(
                // No 1.max here: Old(0) is reachable output (universe age at the 0.4 Gyr
                // floor), so only the potential underflow is made saturating.
                universe_age
                    .saturating_sub(200)
                    .min((galaxy.neighborhood.universe.age * 1000.0) as u64 / divide_by),
            );
        }
        StellarNeighborhoodAge::Ancient(_) => {
            age = StellarNeighborhoodAge::Ancient(
                1.max(
                    universe_age
                        .saturating_sub(200)
                        .min(universe_age.saturating_sub(rng.roll(1, 10, -1) as u64 * 1000)),
                ),
            );
        }
    }

    Ok(age)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_star_neighborhood_with_proper_size() -> Result<(), GenerationError> {
        let mut rng = SeededDiceRoller::new("seed", "step");
        for i in 0..10000 {
            let settings = &GenerationSettings {
                seed: Rc::from(i.to_string()),
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
            let mut galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &settings);
            let gal_end = galaxy.get_galactic_end();
            let x = rng.gen_u32() as i64 % gal_end.x;
            let y = rng.gen_u32() as i64 % gal_end.y;
            let z = rng.gen_u32() as i64 % gal_end.z;
            let hex = galaxy
                .get_hex(SpaceCoordinates::new(x, y, z).rel(galaxy.get_galactic_start()))?
                .clone();
            match hex.neighborhood.age {
                StellarNeighborhoodAge::Young(a) => {
                    assert!(a >= 1 && a <= (galaxy.neighborhood.universe.age * 1000.0) as u64 - 200)
                }
                StellarNeighborhoodAge::Mature => (),
                StellarNeighborhoodAge::Old(a) => {
                    // Old(0) is reachable by design at the 0.4 Gyr universe-age
                    // floor (the generator only saturates the subtraction), so the
                    // lower bound is 0 here, unlike Young and Ancient.
                    assert!(a <= (galaxy.neighborhood.universe.age * 1000.0) as u64 - 200)
                }
                StellarNeighborhoodAge::Ancient(a) => {
                    assert!(a >= 1 && a <= (galaxy.neighborhood.universe.age * 1000.0) as u64 - 200)
                }
            }
        }
        Ok(())
    }
}
