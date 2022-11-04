use crate::prelude::*;
/// The stelliferous era we currently live in.
const OUR_UNIVERSES_ERA: StelliferousEra = StelliferousEra::MiddleStelliferous;
/// The current age of the universe.
const OUR_UNIVERSES_AGE: f32 = 13.8;

/// A list of settings used to configure the [Universe] generation.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct UniverseSettings {
    /// The specific universe [StelliferousEra] to use if any. Will be overwritten if **fixed_age** or **use_ours** is set.
    pub fixed_era: Option<StelliferousEra>,
    /// The specific universe age to use if any, in billions of years. **Must be higher or equal to 0.4 and lower than 100000.**
    /// Will overwrite **fixed_era** if set, and be overwritten if **use_ours** is set.
    pub fixed_age: Option<f32>,
    /// Skip the universe generation and just uses a copy of ours. Will overwrite **fixed_era** and **fixed_age** if set.
    pub use_ours: bool,
}

/// The Stelliferous Era is the span of time after the Big Bang and the Primordial Era in which matter is arranged in the form of stars,
/// galaxies, and galaxy clusters, and most energy is produced in stars. Stars are the most dominant objects of the universe in this era.
/// Massive stars use up their fuel very rapidly, in as little as a few million years. Eventually, the only luminous stars remaining will
/// be white dwarf stars. By the end of this era, bright stars as we know them will be gone, their nuclear fuel exhausted, and only white
/// dwarfs, brown dwarfs, neutron stars and black holes will remain.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum StelliferousEra {
    /// A period between the Primordial and Stelliferous eras. With the birth and death of the first Population III stars, the first
    /// mini-galaxies and Population II stars begin to appear alongside elements heavier than helium. Galaxies allow small bubbles of
    /// ionized gas to exist and expand, but most of the universe is still made of an extremely dense and opaque neutral hydrogen cloud
    /// which would make impossible to see outside of one's galaxy.
    AncientStelliferous,
    /// The majority of the universe reionizes more or less 1 billion years after the Big Bang, light can now travel freely across the
    /// whole universe. The mini-galaxies created previously begin to merge and form mature galaxies, which may interact with each others
    /// by colliding, being engulfed, etc. The first Population I stars appear, which contain heavier elements and are more likely to be
    /// accompanied by planets.
    EarlyStelliferous,
    /// The part of the Stelliferous era we live in. There is a wide range of galaxies consisting of a wide range of stars.
    MiddleStelliferous,
    /// Local galactic groups will have merged into single giant galaxies. All galaxies outside of one's own cluster will disappear below
    /// the cosmological horizon - the only things that a member of a galaxy will be able to see are the stars and objects within its own
    /// galaxy. As the Late Stelliferous progresses, the general luminosity of galaxies will also diminish as the less massive red dwarfs
    /// begin to die as white dwarfs.
    LateStelliferous,
    /// All other galaxies outside one's own will no longer be detectable by any means. As this era progresses, stars will exhaust their
    /// fuel and cool off. All stars not massive enough to become a neutron star or a black hole will turn into white dwarfs and slowly cool
    /// until they're black dwarfs. By the end of the End Stelliferous, all stars will have burned out and star formation will end.
    EndStelliferous,
}

/// Data allowing us to model the universe.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Universe {
    /// In which part of the Stelliferous Era the universe is currently.
    pub era: StelliferousEra,
    /// The time passed since the big bang, in billions of years.
    pub age: f32,
}

impl Universe {
    /// Generates a brand new universe using the given seed and [GenerationSettings].
    pub fn generate(seed: String, settings: GenerationSettings) -> Self {
        let era = Self::generate_era(settings, &seed);
        let age = Self::generate_age(settings, seed, era);
        let universe = Universe { era, age };
        trace!("generate - created the following {:?}", universe);
        universe
    }

    /// Returns an era to use in a [Universe], either using the one given in **settings** or generating one using the given **seed**.
    fn generate_era(settings: GenerationSettings, seed: &String) -> StelliferousEra {
        let era;
        match settings.universe {
            Some(sub_set) => {
                if sub_set.use_ours {
                    era = OUR_UNIVERSES_ERA;
                } else if sub_set.fixed_age.is_some() {
                    let age = sub_set.fixed_age.unwrap();
                    if age < 0.4 {
                        panic!("Received a fixed age lower than 0.4.")
                    } else if age < 0.5 {
                        era = StelliferousEra::AncientStelliferous;
                    } else if age < 5.0 {
                        era = StelliferousEra::EarlyStelliferous;
                    } else if age < 50.0 {
                        era = StelliferousEra::MiddleStelliferous;
                    } else if age < 2000.0 {
                        era = StelliferousEra::LateStelliferous;
                    } else {
                        era = StelliferousEra::EndStelliferous;
                    }
                } else if sub_set.fixed_era.is_some() {
                    era = sub_set.fixed_era.unwrap();
                } else {
                    era = Self::calculate_era(seed);
                }
            }
            None => era = Self::calculate_era(seed),
        };
        era
    }

    /// Generates a random era using the given **seed**.
    fn calculate_era(seed: &String) -> StelliferousEra {
        trace!("calculate_era - about to generate an era");
        let era = SeededDiceRoller::new(seed.as_str(), "uni_era")
            .get_result(&CopyableRollToProcess {
                possible_results: SeededDiceRoller::to_copyable_possible_results(vec![
                    StelliferousEra::AncientStelliferous,
                    StelliferousEra::EarlyStelliferous,
                    StelliferousEra::MiddleStelliferous,
                    StelliferousEra::LateStelliferous,
                    StelliferousEra::EndStelliferous,
                ]),
                roll_method: RollMethod::GaussianRoll(7),
            })
            .expect("Should return a proper era.");
        era
    }

    /// Returns an age to use in a [Universe], either using the one given in **settings** or generating one using the given **seed**.
    fn generate_age(settings: GenerationSettings, seed: String, era: StelliferousEra) -> f32 {
        let age;
        match settings.universe {
            Some(sub_set) => {
                if sub_set.use_ours {
                    age = OUR_UNIVERSES_AGE;
                } else if sub_set.fixed_age.is_some() {
                    age = sub_set.fixed_age.unwrap();
                } else {
                    age = Self::calculate_age(seed, era);
                }
            }
            None => age = Self::calculate_age(seed, era),
        };
        age
    }

    /// Generates the age of a [Universe] using the given **era** and **seed**.
    fn calculate_age(seed: String, era: StelliferousEra) -> f32 {
        trace!("calculate_era - about to generate an age for {:#?}", era);
        let age: f32;
        let mut rng = SeededDiceRoller::new(seed.as_str(), "uni_age");
        match era {
            StelliferousEra::AncientStelliferous => {
                age = (rng.roll(1, 10, 39) as f32) / 100.0;
            }
            StelliferousEra::EarlyStelliferous => {
                age = (rng.roll(1, 450, 49) as f32) / 100.0;
            }
            StelliferousEra::MiddleStelliferous => {
                age = (rng.roll(1, 4500, 499) as f32) / 100.0;
            }
            StelliferousEra::LateStelliferous => {
                age = (rng.roll(1, 19500, 499) as f32) / 10.0;
            }
            StelliferousEra::EndStelliferous => {
                age = (rng.roll(2, 50, 0) * 1000) as f32;
            }
        }
        age
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_a_universe() {
        for i in 0..10000 {
            let universe = Universe::generate(
                String::from(i.to_string()),
                GenerationSettings {
                    ..Default::default()
                },
            );
            let era = universe.era;
            let age = universe.age;
            match era {
                StelliferousEra::AncientStelliferous => assert_eq!(age >= 0.4 && age < 0.5, true),
                StelliferousEra::EarlyStelliferous => assert_eq!(age >= 0.5 && age < 5.0, true),
                StelliferousEra::MiddleStelliferous => assert_eq!(age >= 5.0 && age < 50.0, true),
                StelliferousEra::LateStelliferous => assert_eq!(age >= 50.0 && age < 2000.0, true),
                StelliferousEra::EndStelliferous => {
                    assert_eq!(age >= 2000.0 && age < 100000.0, true)
                }
            }
        }
    }

    #[test]
    fn generate_our_universe() {
        for i in 0..100 {
            let universe = Universe::generate(
                String::from(i.to_string()),
                GenerationSettings {
                    universe: Some(UniverseSettings {
                        use_ours: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            );
            assert_eq!(universe.era, OUR_UNIVERSES_ERA);
            assert_eq!(universe.age, OUR_UNIVERSES_AGE);
        }
    }

    #[test]
    fn generate_a_universe_with_specific_era() {
        for i in 0..100 {
            let modulo = i % 5;
            let era = match modulo {
                0 => StelliferousEra::AncientStelliferous,
                1 => StelliferousEra::EarlyStelliferous,
                2 => StelliferousEra::MiddleStelliferous,
                3 => StelliferousEra::LateStelliferous,
                _ => StelliferousEra::EndStelliferous,
            };
            let universe = Universe::generate(
                String::from(i.to_string()),
                GenerationSettings {
                    universe: Some(UniverseSettings {
                        fixed_era: Some(era),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            );
            assert_eq!(universe.era, era);
        }
    }

    #[test]
    fn generate_a_universe_with_specific_age() {
        for i in 0..100 {
            let age = SeededDiceRoller::new(&i.to_string(), "test").gen_f32() % 99999.6 + 0.4;
            let universe = Universe::generate(
                String::from(i.to_string()),
                GenerationSettings {
                    universe: Some(UniverseSettings {
                        fixed_age: Some(age),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            );
            assert_eq!(universe.age, age);
        }
    }
}
