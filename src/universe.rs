use crate::prelude::*;
/// The stelliferous era we currently live in.
const OUR_UNIVERSES_ERA: StelliferousEra = StelliferousEra::MiddleStelliferous;
/// The current age of the universe.
const OUR_UNIVERSES_AGE: f32 = 13.8;
/// The time in billions of years when the Ancient Stelliferous Era starts.
const MIN_ANCIENT_STELLIFEROUS: f32 = 0.4;
/// The time in billions of years when the Early Stelliferous Era starts.
const MIN_EARLY_STELLIFEROUS: f32 = 0.5;
/// The time in billions of years when the Middle Stelliferous Era starts.
const MIN_MIDDLE_STELLIFEROUS: f32 = 5.0;
/// The time in billions of years when the Late Stelliferous Era starts.
const MIN_LATE_STELLIFEROUS: f32 = 50.0;
/// The time in billions of years when the End Stelliferous Era starts.
const MIN_END_STELLIFEROUS: f32 = 2000.0;
/// The time in billions of years when the End Stelliferous Era ends.
const MAX_END_STELLIFEROUS: f32 = 100000.0;
/// An array containing the data used to calculate a universe's age.
const POSSIBLE_ERAS: [PossibleEra; 5] = [
    PossibleEra {
        era: StelliferousEra::AncientStelliferous,
        min: MIN_ANCIENT_STELLIFEROUS,
        max: MIN_EARLY_STELLIFEROUS,
        weight: 1,
    },
    PossibleEra {
        era: StelliferousEra::EarlyStelliferous,
        min: MIN_EARLY_STELLIFEROUS,
        max: MIN_MIDDLE_STELLIFEROUS,
        weight: 40,
    },
    PossibleEra {
        era: StelliferousEra::MiddleStelliferous,
        min: MIN_MIDDLE_STELLIFEROUS,
        max: MIN_LATE_STELLIFEROUS,
        weight: 218,
    },
    PossibleEra {
        era: StelliferousEra::LateStelliferous,
        min: MIN_LATE_STELLIFEROUS,
        max: MIN_END_STELLIFEROUS,
        weight: 40,
    },
    PossibleEra {
        era: StelliferousEra::EndStelliferous,
        min: MIN_END_STELLIFEROUS,
        max: MAX_END_STELLIFEROUS,
        weight: 1,
    },
];

/// A list of settings used to configure the [Universe] generation.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct UniverseSettings {
    /// The specific universe [StelliferousEra] to use if any. Will be overwritten if the age or **use_ours** is set.
    pub fixed_era: Option<StelliferousEra>,
    /// Asks to generate a universe's [StelliferousEra] randomly, but only using eras that are older than the given one. Will be overwritten
    /// if the age or **use_ours** is set.
    pub era_before: Option<StelliferousEra>,
    /// Asks to generate a universe's [StelliferousEra] randomly, but only using eras that are younger than the given one. Will be
    /// overwritten if the age or **use_ours** is set.
    pub era_after: Option<StelliferousEra>,
    /// The specific universe age to use if any, in billions of years. **Must be higher or equal to 0.4 and lower than 100000.**
    /// Will overwrite the era if set, and be overwritten if **use_ours** is set.
    pub fixed_age: Option<f32>,
    /// Asks to generate a universe's age randomly, but with an age at least older than the given one. **Must be higher or equal to 0.4
    /// and lower than 100000.** Will overwrite the era if set, and be overwritten if **use_ours** is set.
    pub age_before: Option<f32>,
    /// Asks to generate a universe's age randomly, but with an age at least younger than the given one. **Must be higher or equal to 0.4
    /// and lower than 100000.** Will overwrite the era if set, and be overwritten if **use_ours** is set.
    pub age_after: Option<f32>,
    /// Skip the universe generation and just uses a copy of ours. Will overwrite **fixed_era** and **fixed_age** if set.
    pub use_ours: bool,
}

/// The Stelliferous Era is the span of time after the Big Bang and the Primordial Era in which matter is arranged in the form of stars,
/// galaxies, and galaxy clusters, and most energy is produced in stars. Stars are the most dominant objects of the universe in this era.
/// Massive stars use up their fuel very rapidly, in as little as a few million years. Eventually, the only luminous stars remaining will
/// be white dwarf stars. By the end of this era, bright stars as we know them will be gone, their nuclear fuel exhausted, and only white
/// dwarfs, brown dwarfs, neutron stars and black holes will remain.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
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
    #[default]
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

impl Display for StelliferousEra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = String::new();
        match self {
            StelliferousEra::AncientStelliferous => name.push_str("Ancient Stelliferous"),
            StelliferousEra::EarlyStelliferous => name.push_str("Early Stelliferous"),
            StelliferousEra::MiddleStelliferous => name.push_str("Middle Stelliferous"),
            StelliferousEra::LateStelliferous => name.push_str("Late Stelliferous"),
            StelliferousEra::EndStelliferous => name.push_str("End Stelliferous"),
        }
        write!(f, "{}", name)
    }
}

/// Data allowing us to model the universe.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Universe {
    /// In which part of the Stelliferous Era the universe is currently.
    pub era: StelliferousEra,
    /// The time passed since the big bang, in billions of years.
    pub age: f32,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Universe {{ age: {} billion years, era: {} }}",
            self.age, self.era
        )
    }
}

/// Data used to calculate a universe's age.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
struct PossibleEra {
    /// The era this object represents.
    era: StelliferousEra,
    /// When the era begins.
    min: f32,
    /// When the era ends.
    max: f32,
    /// How often should we get this era as result during generation?
    weight: u32,
}

impl Universe {
    /// Generates a brand new universe using the given seed and [GenerationSettings]. If an appropriate age or era cannot be generated from
    /// the given settings, our own universe's age and/or era will be used.
    pub fn generate(seed: String, settings: GenerationSettings) -> Self {
        let mut age = Self::generate_age(settings, seed);
        let mut era = Self::get_era_from_age(age);
        if !Self::are_age_and_era_valid(era, age) {
            age = OUR_UNIVERSES_AGE;
            era = OUR_UNIVERSES_ERA;
        }
        let universe = Universe { era, age };
        trace!("generated the following {}", universe);
        universe
    }

    /// Generates an age to use in a [Universe] while following the given [GenerationSettings].
    fn generate_age(settings: GenerationSettings, seed: String) -> f32 {
        let age;
        match settings.universe {
            Some(sub_set) => {
                if sub_set.use_ours {
                    age = OUR_UNIVERSES_AGE;
                } else if sub_set.fixed_age.is_some() {
                    age = sub_set.fixed_age.expect("Fixed age should have been set.");
                } else {
                    age = Self::calculate_age(settings, seed);
                }
            }
            None => age = Self::calculate_age(settings, seed),
        };
        age
    }

    /// Generates the age of a [Universe] using the given [GenerationSettings] and **seed**.
    fn calculate_age(settings: GenerationSettings, seed: String) -> f32 {
        let age: f32;
        let mut rng = SeededDiceRoller::new(seed.as_str(), "uni_age");
        let (mut min, mut max) = Self::get_min_and_max_age(settings);
        let possible_eras = Self::filter_unwanted_eras(min, max);

        if let Some(era) = Self::generate_era(&mut rng, possible_eras) {
            min = min.max(era.min);
            max = max.min(era.max);

            max = max - min;
            age = (rng.roll(1, (max * 100.0) as u32, (min * 100.0) as i32) as f32) / 100.0;
        } else {
            age = OUR_UNIVERSES_AGE;
        }
        age
    }

    /// Uses the given [GenerationSettings] to get the min and max universe age to use.
    fn get_min_and_max_age(settings: GenerationSettings) -> (f32, f32) {
        let mut min: f32 = MIN_ANCIENT_STELLIFEROUS;
        let mut max: f32 = MAX_END_STELLIFEROUS;
        if let Some(sub_set) = settings.universe {
            if let Some(era_after) = sub_set.era_after {
                match era_after {
                    StelliferousEra::AncientStelliferous => {
                        min = min.max(MIN_EARLY_STELLIFEROUS);
                    }
                    StelliferousEra::EarlyStelliferous => {
                        min = min.max(MIN_MIDDLE_STELLIFEROUS);
                    }
                    StelliferousEra::MiddleStelliferous => {
                        min = min.max(MIN_LATE_STELLIFEROUS);
                    }
                    StelliferousEra::LateStelliferous => {
                        min = min.max(MIN_END_STELLIFEROUS);
                    }
                    StelliferousEra::EndStelliferous => {
                        min = min.max(MAX_END_STELLIFEROUS);
                    }
                }
            }
            if let Some(era_before) = sub_set.era_before {
                match era_before {
                    StelliferousEra::AncientStelliferous => {
                        max = max.min(MIN_ANCIENT_STELLIFEROUS);
                    }
                    StelliferousEra::EarlyStelliferous => {
                        max = max.min(MIN_EARLY_STELLIFEROUS);
                    }
                    StelliferousEra::MiddleStelliferous => {
                        max = max.min(MIN_MIDDLE_STELLIFEROUS);
                    }
                    StelliferousEra::LateStelliferous => {
                        max = max.min(MIN_LATE_STELLIFEROUS);
                    }
                    StelliferousEra::EndStelliferous => {
                        max = max.min(MIN_END_STELLIFEROUS);
                    }
                }
            }
            if let Some(fixed_era) = sub_set.fixed_era {
                match fixed_era {
                    StelliferousEra::AncientStelliferous => {
                        min = min.max(MIN_ANCIENT_STELLIFEROUS);
                        max = max.min(MIN_EARLY_STELLIFEROUS);
                    }
                    StelliferousEra::EarlyStelliferous => {
                        min = min.max(MIN_EARLY_STELLIFEROUS);
                        max = max.min(MIN_MIDDLE_STELLIFEROUS);
                    }
                    StelliferousEra::MiddleStelliferous => {
                        min = min.max(MIN_MIDDLE_STELLIFEROUS);
                        max = max.min(MIN_LATE_STELLIFEROUS);
                    }
                    StelliferousEra::LateStelliferous => {
                        min = min.max(MIN_LATE_STELLIFEROUS);
                        max = max.min(MIN_END_STELLIFEROUS);
                    }
                    StelliferousEra::EndStelliferous => {
                        min = min.max(MIN_END_STELLIFEROUS);
                        max = max.min(MAX_END_STELLIFEROUS);
                    }
                }
            }
            if let Some(age_after) = sub_set.age_after {
                min = min.max(age_after);
            }
            if let Some(age_before) = sub_set.age_before {
                max = max.min(age_before);
            }
        }
        (min, max)
    }

    /// Removes every [PossibleEra] that is before or after the **min** and **max** times.
    fn filter_unwanted_eras(min: f32, max: f32) -> Vec<PossibleEra> {
        let mut possible_eras: Vec<PossibleEra> = Vec::new();
        POSSIBLE_ERAS.iter().for_each(|era| {
            if min < era.max && max > era.min {
                possible_eras.push(*era)
            }
        });
        possible_eras
    }

    /// Rolls for a [PossibleEra] between the give possible choices.
    fn generate_era(
        rng: &mut SeededDiceRoller,
        possible_eras: Vec<PossibleEra>,
    ) -> Option<PossibleEra> {
        rng.get_result(&CopyableRollToProcess {
            possible_results: possible_eras
                .iter()
                .map(|era| CopyableWeightedResult {
                    result: *era,
                    weight: era.weight,
                })
                .collect(),
            roll_method: RollMethod::SimpleRoll,
        })
    }

    /// Checks if the given **age** and **era** match each other. Panics if the age isn't valid.
    fn are_age_and_era_valid(era: StelliferousEra, age: f32) -> bool {
        let mut result = false;
        assert!(age >= MIN_ANCIENT_STELLIFEROUS && age < MAX_END_STELLIFEROUS);
        match era {
            StelliferousEra::AncientStelliferous => {
                if age >= MIN_ANCIENT_STELLIFEROUS && age < MIN_EARLY_STELLIFEROUS {
                    result = true
                }
            }
            StelliferousEra::EarlyStelliferous => {
                if age >= MIN_EARLY_STELLIFEROUS && age < MIN_MIDDLE_STELLIFEROUS {
                    result = true
                }
            }
            StelliferousEra::MiddleStelliferous => {
                if age >= MIN_MIDDLE_STELLIFEROUS && age < MIN_LATE_STELLIFEROUS {
                    result = true
                }
            }
            StelliferousEra::LateStelliferous => {
                if age >= MIN_LATE_STELLIFEROUS && age < MIN_END_STELLIFEROUS {
                    result = true
                }
            }
            StelliferousEra::EndStelliferous => {
                if age >= MIN_END_STELLIFEROUS && age < MAX_END_STELLIFEROUS {
                    result = true
                }
            }
        }
        result
    }

    /// Returns the [StelliferousEra] matching the given age.
    fn get_era_from_age(age: f32) -> StelliferousEra {
        let mut result = StelliferousEra::MiddleStelliferous;
        if age >= MIN_ANCIENT_STELLIFEROUS && age < MIN_EARLY_STELLIFEROUS {
            result = StelliferousEra::AncientStelliferous
        } else if age >= MIN_EARLY_STELLIFEROUS && age < MIN_MIDDLE_STELLIFEROUS {
            result = StelliferousEra::EarlyStelliferous
        } else if age >= MIN_MIDDLE_STELLIFEROUS && age < MIN_LATE_STELLIFEROUS {
            result = StelliferousEra::MiddleStelliferous
        } else if age >= MIN_LATE_STELLIFEROUS && age < MIN_END_STELLIFEROUS {
            result = StelliferousEra::LateStelliferous
        } else if age >= MIN_END_STELLIFEROUS && age < MAX_END_STELLIFEROUS {
            result = StelliferousEra::EndStelliferous
        }
        result
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
            assert_eq!(Universe::are_age_and_era_valid(era, age), true);
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
            assert_eq!(
                Universe::are_age_and_era_valid(universe.era, universe.age),
                true
            );
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
            assert_eq!(
                Universe::are_age_and_era_valid(universe.era, universe.age),
                true
            );
        }
    }

    #[test]
    fn generate_a_universe_with_specific_age() {
        for i in 0..100 {
            let age = SeededDiceRoller::new(&i.to_string(), "test").gen_f32() % 99999.6
                + MIN_ANCIENT_STELLIFEROUS;
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
            assert_eq!(
                Universe::are_age_and_era_valid(universe.era, universe.age),
                true
            );
        }
    }

    #[test]
    fn generate_a_universe_with_an_era_greater_or_lower_than() {
        // todo
    }

    #[test]
    fn generate_a_universe_with_an_age_greater_or_lower_than() {
        // todo
    }

    #[test]
    fn generate_a_universe_with_conflicting_settings() {
        // todo
    }
}
