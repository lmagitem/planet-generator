use crate::prelude::*;

impl StellarNeighborhood {
    /// Generates a [StellarNeighborhood] using the given parameters.
    pub fn generate(coord: SpaceCoordinates, galaxy: &mut Galaxy) -> Self {
        Self {
            age: generate_age(coord, galaxy),
        }
    }
}

/// Uses data about the [GalacticMapDivision]s a neighborhood belongs to to generate its age.
fn generate_age(coord: SpaceCoordinates, galaxy: &mut Galaxy) -> StellarNeighborhoodAge {
    let divisions = galaxy
        .get_divisions_for_coord(coord)
        .expect("Should have returned divisions.");
    let mut regions = Vec::new();
    divisions.iter().for_each(|div| {
        if regions.iter().find(|r| **r == div.region).is_none() {
            regions.push(div.region.clone());
        }
    });
    let sub_sector = divisions
        .iter()
        .find(|div| div.level == 1)
        .expect("Should have found a subsector.");
    let mut modifier = 0;
    regions.iter().for_each(|region| match region {
        GalacticRegion::GlobularCluster
        | GalacticRegion::Halo
        | GalacticRegion::Aura
        | GalacticRegion::Void => modifier += 5,
        GalacticRegion::Stream => modifier += 3,
        GalacticRegion::Core | GalacticRegion::Bulge => modifier += 2,
        GalacticRegion::Disk | GalacticRegion::Ellipse => modifier += 1,
        GalacticRegion::Nucleus => modifier -= 1,
        GalacticRegion::Bar | GalacticRegion::Arm => modifier -= 2,
        GalacticRegion::Association => modifier -= 3,
        GalacticRegion::OpenCluster => modifier -= 5,
        _ => (),
    });

    let mut rng = SeededDiceRoller::new(
        &galaxy.settings.seed,
        &format!("ste_nei_{}_age", sub_sector.index),
    );
    let mut age = rng
        .get_result(&CopyableRollToProcess {
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
        })
        .expect("Should return a proper neighborhood age.");

    let universe_age = (galaxy.neighborhood.universe.age * 1000.0) as u64 - 200;
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
                (universe_age - 200)
                    .min((galaxy.neighborhood.universe.age * 1000.0) as u64 / divide_by),
            );
        }
        StellarNeighborhoodAge::Ancient(_) => {
            age = StellarNeighborhoodAge::Ancient(
                (universe_age - 200).min(universe_age - rng.roll(1, 10, -1) as u64 * 1000),
            );
        }
    }

    age
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_star_neighborhood_with_proper_size() {
        let mut rng = SeededDiceRoller::new("seed", "step");
        for i in 0..10000 {
            let settings = &GenerationSettings {
                seed: String::from(&i.to_string()),
                galaxy: GalaxySettings {
                    ..Default::default()
                },
                ..Default::default()
            };
            let neighborhood =
                GalacticNeighborhood::generate(Universe::generate(&settings), &settings);
            let mut galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &settings);
            let gal_end = galaxy.get_galactic_end();
            let x = rng.gen_u32() as i64 % gal_end.x;
            let y = rng.gen_u32() as i64 % gal_end.y;
            let z = rng.gen_u32() as i64 % gal_end.z;
            let hex = galaxy
                .get_hex(SpaceCoordinates::new(x, y, z).rel(galaxy.get_galactic_start()))
                .expect("Should have generated a hex.");
            match hex.neighborhood.age {
                StellarNeighborhoodAge::Young(a) => {
                    assert!(a >= 1 && a <= (galaxy.neighborhood.universe.age * 1000.0) as u64 - 200)
                }
                StellarNeighborhoodAge::Mature => (),
                StellarNeighborhoodAge::Old(a) => {
                    assert!(a >= 1 && a <= (galaxy.neighborhood.universe.age * 1000.0) as u64 - 200)
                }
                StellarNeighborhoodAge::Ancient(a) => {
                    assert!(a >= 1 && a <= (galaxy.neighborhood.universe.age * 1000.0) as u64 - 200)
                }
            }
        }
    }
}
