#[path = "./constants.rs"]
mod universe_constants;
use crate::prelude::*;
use universe_constants::*;

/// Data allowing us to model the universe.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct Universe {
    /// In which part of the Stelliferous Era the universe is currently.
    pub era: StelliferousEra,
    /// The time passed since the big bang, in billions of years.
    #[default = 13.8]
    pub age: f32,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A {} billion years old Universe in the {} era",
            self.age, self.era
        )
    }
}

impl Universe {
    /// Generates a brand new [Universe] using the given seed and [GenerationSettings]. If an appropriate age or era cannot be generated from
    /// the given settings, our own universe's age and/or era will be used.
    pub fn generate(seed: &String, settings: &GenerationSettings) -> Self {
        trace!("seed: {}, settings: {}", seed, settings.universe);
        let mut age = generate_age(settings, seed);
        let mut era = get_era_from_age(age);
        if !are_age_and_era_valid(era, age) {
            age = OUR_UNIVERSES_AGE;
            era = OUR_UNIVERSES_ERA;
        }
        let universe = Universe { era, age };
        trace!("generated: {}", universe);
        universe
    }
}

/// Generates an age to use in a [Universe] while following the given [GenerationSettings].
fn generate_age(settings: &GenerationSettings, seed: &String) -> f32 {
    let age;
    if settings.universe.use_ours {
        age = OUR_UNIVERSES_AGE;
    } else if settings.universe.fixed_age.is_some() {
        age = settings
            .universe
            .fixed_age
            .expect("Fixed age should have been set.");
    } else {
        age = calculate_age(settings, seed);
    }
    age
}

/// Generates the age of a [Universe] using the given [GenerationSettings] and **seed**.
fn calculate_age(settings: &GenerationSettings, seed: &String) -> f32 {
    let age: f32;
    let mut rng = SeededDiceRoller::new(seed.as_str(), "uni_age");
    let (mut min, mut max) = get_min_and_max_age(settings);
    let possible_eras = filter_unwanted_eras(min, max);

    if let Some(era) = generate_era(&mut rng, possible_eras) {
        min = min.max(era.min);
        max = max.min(era.max);

        max = max - min;
        if ((max * 100.0) as u32) == 0 {
            age = (max + min * 100.0).round() / 100.0;
        } else {
            age = (rng.roll(1, (max * 100.0) as u32, (min * 100.0) as i32) as f32).round() / 100.0;
        }
    } else {
        age = OUR_UNIVERSES_AGE;
    }
    age
}

/// Uses the given [GenerationSettings] to get the min and max universe age to use.
fn get_min_and_max_age(settings: &GenerationSettings) -> (f32, f32) {
    let mut min: f32 = MIN_ANCIENT_STELLIFEROUS;
    let mut max: f32 = MAX_END_STELLIFEROUS;
    if let Some(era_after) = settings.universe.era_after {
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
    if let Some(era_before) = settings.universe.era_before {
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
    if let Some(fixed_era) = settings.universe.fixed_era {
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
    if let Some(age_after) = settings.universe.age_after {
        min = min.max(age_after);
    }
    if let Some(age_before) = settings.universe.age_before {
        max = max.min(age_before);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_a_universe() {
        for i in 0..10000 {
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    ..Default::default()
                },
            );
            let era = universe.era;
            let age = universe.age;
            assert!(are_age_and_era_valid(era, age));
        }
    }

    #[test]
    fn generate_our_universe() {
        for i in 0..100 {
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        use_ours: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert_eq!(universe.era, OUR_UNIVERSES_ERA);
            assert_eq!(universe.age, OUR_UNIVERSES_AGE);
            assert!(are_age_and_era_valid(universe.era, universe.age));
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
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        fixed_era: Some(era),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert_eq!(universe.era, era);
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }
    }

    #[test]
    fn generate_a_universe_with_specific_age() {
        for i in 0..100 {
            let age = SeededDiceRoller::new(&i.to_string(), "t").gen_f32() % 99999.6
                + MIN_ANCIENT_STELLIFEROUS;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        fixed_age: Some(age),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert_eq!(universe.age, age);
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }
    }

    #[test]
    fn generate_a_universe_with_an_era_greater_or_lower_than() {
        for i in 0..100 {
            let era = POSSIBLE_ERAS
                [SeededDiceRoller::new(&i.to_string(), "t").gen_usize() % POSSIBLE_ERAS.len()]
            .era;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        era_after: Some(era),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }

        for i in 0..100 {
            let era = POSSIBLE_ERAS
                [SeededDiceRoller::new(&i.to_string(), "t").gen_usize() % POSSIBLE_ERAS.len()]
            .era;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        era_before: Some(era),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }
    }

    #[test]
    fn generate_a_universe_with_an_age_greater_or_lower_than() {
        for i in 0..1000 {
            let age = (((SeededDiceRoller::new(&i.to_string(), "t").gen_u32() % 294960) as f32)
                + MIN_ANCIENT_STELLIFEROUS * 100.0)
                .round()
                / 100.0;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        age_after: Some(age),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert!(universe.age >= age);
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }

        for i in 0..1000 {
            let age = (((SeededDiceRoller::new(&i.to_string(), "t").gen_u32() % 294960) as f32)
                + MIN_ANCIENT_STELLIFEROUS * 100.0)
                .round()
                / 100.0;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        age_before: Some(age),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert!(universe.age < age);
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }
    }

    #[test]
    fn generate_a_universe_with_conflicting_settings() {
        for i in 0..1000 {
            let age_after = (((SeededDiceRoller::new(&i.to_string(), "after").gen_u32() % 294960)
                as f32)
                + MIN_ANCIENT_STELLIFEROUS * 100.0)
                .round()
                / 100.0;
            let age_before = (((SeededDiceRoller::new(&i.to_string(), "before").gen_u32() % 294960)
                as f32)
                + MIN_ANCIENT_STELLIFEROUS * 100.0)
                .round()
                / 100.0;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        age_before: Some(age_before),
                        age_after: Some(age_after),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            if age_after < age_before {
                assert!(universe.age < age_before);
                assert!(universe.age >= age_after);
            }
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }

        for i in 0..100 {
            let era_after = POSSIBLE_ERAS
                [SeededDiceRoller::new(&i.to_string(), "after").gen_usize() % POSSIBLE_ERAS.len()]
            .era;
            let era_before = POSSIBLE_ERAS
                [SeededDiceRoller::new(&i.to_string(), "before").gen_usize() % POSSIBLE_ERAS.len()]
            .era;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        era_after: Some(era_after),
                        era_before: Some(era_before),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }

        for i in 0..1000 {
            let age_after = (((SeededDiceRoller::new(&i.to_string(), "after").gen_u32() % 294960)
                as f32)
                + MIN_ANCIENT_STELLIFEROUS * 100.0)
                .round()
                / 100.0;
            let age_before = (((SeededDiceRoller::new(&i.to_string(), "before").gen_u32() % 294960)
                as f32)
                + MIN_ANCIENT_STELLIFEROUS * 100.0)
                .round()
                / 100.0;
            let era_after = POSSIBLE_ERAS
                [SeededDiceRoller::new(&i.to_string(), "after").gen_usize() % POSSIBLE_ERAS.len()]
            .era;
            let era_before = POSSIBLE_ERAS
                [SeededDiceRoller::new(&i.to_string(), "before").gen_usize() % POSSIBLE_ERAS.len()]
            .era;
            let universe = Universe::generate(
                &String::from(i.to_string()),
                &GenerationSettings {
                    universe: UniverseSettings {
                        era_after: Some(era_after),
                        era_before: Some(era_before),
                        age_before: Some(age_before),
                        age_after: Some(age_after),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            assert!(are_age_and_era_valid(universe.era, universe.age));
        }
    }
}
