use crate::prelude::*;

/// Data allowing us to model a galaxy Neighborhood (a section of the universe containing multiple galaxies).
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalacticNeighborhood {
    /// The universe this neighborhood belongs to.
    pub universe: Universe,
    /// How dense is this neighborhood, with the number of galaxies present.
    pub density: GalacticNeighborhoodDensity,
}

impl Display for GalacticNeighborhood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Galactic {}", self.density)
    }
}

impl GalacticNeighborhood {
    /// Returns a new [GalacticNeighborhood] using the given arguments.
    pub fn new(universe: Universe, density: GalacticNeighborhoodDensity) -> Self {
        Self { universe, density }
    }

    /// Generates a brand new [GalacticNeighborhood] using the given seed and [GenerationSettings].
    pub fn generate(universe: Universe, seed: &String, settings: &GenerationSettings) -> Self {
        let density;

        if let Some(fixed_neighborhood) = settings.galaxy.fixed_neighborhood {
            density = fixed_neighborhood;
        } else if settings.galaxy.use_ours {
            density = GalacticNeighborhoodDensity::Group(2, 36);
        } else {
            let mut rng = SeededDiceRoller::new(seed.as_str(), "gal_den");
            let is_group = rng.roll(1, 4, 0) != 4;

            if is_group {
                let galaxies = rng.roll(1, 6, -1) as u8;
                if galaxies == 0 {
                    density = GalacticNeighborhoodDensity::Void(
                        // Major galaxies
                        if universe.era == StelliferousEra::EndStelliferous
                            || universe.era == StelliferousEra::LateStelliferous
                        {
                            rng.roll(1, 2, 0) as u8
                        } else {
                            rng.roll(1, 4, -1) as u8
                        },
                        // Minor galaxies
                        if universe.era == StelliferousEra::EndStelliferous {
                            0
                        } else if universe.era == StelliferousEra::LateStelliferous {
                            rng.roll(1, 5, 0) as u16
                        } else {
                            rng.roll(1, 16, 4) as u16
                        },
                    );
                } else {
                    density = GalacticNeighborhoodDensity::Group(
                        // Major galaxies
                        if universe.era == StelliferousEra::EndStelliferous {
                            rng.roll(1, 2, 0) as u8
                        } else if universe.era == StelliferousEra::LateStelliferous {
                            rng.roll(1, 3, 0) as u8
                        } else {
                            galaxies
                        },
                        // Minor galaxies
                        if universe.era == StelliferousEra::EndStelliferous {
                            0
                        } else if universe.era == StelliferousEra::LateStelliferous {
                            rng.roll(1, 22, 3) as u16
                        } else {
                            rng.roll(1, 70, 9) as u16
                        },
                    );
                }
            } else {
                let mut galaxies = 0 as u8;
                let mut dominant = 0;
                let mut roll = 0;
                let mut turn = 0;

                while roll == 10 || turn < 2 {
                    roll = rng.roll(1, 10, 0) as u8;
                    galaxies += if roll == 1 { 0 } else { roll };
                    dominant += if roll == 1 { 1 } else { 0 };
                    turn += 1;
                }

                density = GalacticNeighborhoodDensity::Cluster(
                    // Dominant galaxies
                    if universe.era == StelliferousEra::EndStelliferous {
                        1
                    } else if universe.era == StelliferousEra::LateStelliferous {
                        1.max(dominant) as u8
                    } else {
                        dominant
                    },
                    // Major galaxies
                    if universe.era == StelliferousEra::EndStelliferous {
                        0.max(rng.roll(1, 2, 0) as u8) as u8
                    } else if universe.era == StelliferousEra::LateStelliferous {
                        1.max(galaxies / 2) as u8
                    } else {
                        galaxies
                    },
                    // Minor galaxies
                    if universe.era == StelliferousEra::EndStelliferous {
                        0
                    } else if universe.era == StelliferousEra::LateStelliferous {
                        rng.roll(1, 200, 0) as u16
                    } else {
                        rng.roll(1, 950, 50) as u16
                    },
                );
            }
        };

        Self { universe, density }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_a_galactic_neighborhood() {
        for i in 0..10000 {
            let settings = GenerationSettings {
                ..Default::default()
            };
            let seed = String::from(&i.to_string());
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&seed, &settings),
                &seed,
                &settings,
            );
            match neighborhood.density {
                GalacticNeighborhoodDensity::Void(galaxies, _) => assert!(galaxies < 4),
                GalacticNeighborhoodDensity::Group(galaxies, _) => {
                    assert!(galaxies > 0 && galaxies < 6)
                }
                GalacticNeighborhoodDensity::Cluster(dominant, galaxies, _) => {
                    assert!(galaxies > 0 || dominant > 0)
                }
            }
        }
    }

    #[test]
    fn generate_our_galactic_neighborhood() {
        for i in 0..100 {
            let settings = GenerationSettings {
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
            let seed = String::from(&i.to_string());
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&seed, &settings),
                &seed,
                &settings,
            );
            assert_eq!(
                neighborhood.density,
                GalacticNeighborhoodDensity::Group(2, 36)
            );
        }
    }

    #[test]
    fn generate_a_galactic_neighborhood_with_specific_density() {
        for i in 0..1000 {
            let mut rng = SeededDiceRoller::new(&String::from(i.to_string()), "t");
            let fixed_neighborhood = rng
                .get_result(&CopyableRollToProcess {
                    possible_results: SeededDiceRoller::to_copyable_possible_results(vec![
                        GalacticNeighborhoodDensity::Void(0, 5),
                        GalacticNeighborhoodDensity::Void(1, 5),
                        GalacticNeighborhoodDensity::Void(2, 5),
                        GalacticNeighborhoodDensity::Void(3, 5),
                        GalacticNeighborhoodDensity::Group(1, 5),
                        GalacticNeighborhoodDensity::Group(2, 5),
                        GalacticNeighborhoodDensity::Group(3, 5),
                        GalacticNeighborhoodDensity::Group(4, 5),
                        GalacticNeighborhoodDensity::Group(5, 5),
                        GalacticNeighborhoodDensity::Cluster(0, 1, 5),
                        GalacticNeighborhoodDensity::Cluster(0, 2, 5),
                        GalacticNeighborhoodDensity::Cluster(0, 3, 5),
                        GalacticNeighborhoodDensity::Cluster(0, 4, 5),
                        GalacticNeighborhoodDensity::Cluster(0, 5, 5),
                        GalacticNeighborhoodDensity::Cluster(0, 6, 5),
                        GalacticNeighborhoodDensity::Cluster(0, 7, 5),
                        GalacticNeighborhoodDensity::Cluster(1, 1, 5),
                        GalacticNeighborhoodDensity::Cluster(1, 2, 5),
                        GalacticNeighborhoodDensity::Cluster(1, 3, 5),
                    ]),
                    roll_method: RollMethod::SimpleRoll,
                })
                .expect("Should return a density as result");

            let settings = &GenerationSettings {
                galaxy: GalaxySettings {
                    fixed_neighborhood: Some(fixed_neighborhood),
                    ..Default::default()
                },
                ..Default::default()
            };
            let seed = String::from(&i.to_string());
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&seed, &settings),
                &seed,
                &settings,
            );
            assert_eq!(neighborhood.density, fixed_neighborhood);
        }
    }
}
