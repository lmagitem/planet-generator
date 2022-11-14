use crate::prelude::*;

/// The current age of the universe.
const OUR_GALAXYS_AGE: f32 = 13.61;

/// Data allowing us to model a galaxy.
#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Galaxy {
    /// The neighborhood this galaxy belongs to.
    pub neighborhood: GalacticNeighborhood,
    /// The name of this galaxy.
    pub name: String,
    /// The age of this galaxy in billions of years.
    pub age: f32,
    /// Is this galaxy a dominant one in its cluster?
    pub is_dominant: bool,
    /// Is this galaxy a major one in its neighborhood?
    pub is_major: bool,
    /// In what category this galaxy belongs to.
    pub category: GalaxyCategory,
    /// In what sub-category this galaxy belongs to.
    pub sub_category: GalaxySubCategory,
    /// What are the pecularities of this galaxy.
    pub special_traits: Vec<GalaxySpecialTrait>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self {
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            name: String::from("Milky Way"),
            age: 13.61,
            is_dominant: false,
            is_major: true,
            category: GalaxyCategory::Spiral(16203, 160),
            sub_category: GalaxySubCategory::BarredSpiral,
            special_traits: vec![GalaxySpecialTrait::NoPeculiarity],
        }
    }
}

impl Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\", {}{}, of sub-type {}, aged {} billion years, with the following special traits: {}",
            self.name,
            if self.is_dominant { "" } else if self.is_major { "major " } else { "minor " },
            self.category,
            self.sub_category,
            self.age,
            self.special_traits
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Galaxy {
    /// Generates a brand new [Galaxy] using the given seed and [GenerationSettings].
    pub fn generate(
        neighborhood: GalacticNeighborhood,
        index: u8,
        seed: &String,
        settings: &GenerationSettings,
    ) -> Self {
        let name = String::from("Galaxy");
        let age = generate_age(neighborhood, index, seed, settings);
        let is_dominant = is_galaxy_dominant(neighborhood, index);
        let is_major = is_galaxy_major(neighborhood, index);
        let mut category = generate_category(
            neighborhood,
            index,
            age,
            is_dominant,
            is_major,
            seed,
            settings,
        );
        let sub_category = generate_sub_category(
            neighborhood,
            category,
            index,
            age,
            is_dominant,
            is_major,
            seed,
            settings,
        );
        if settings.galaxy.fixed_category.is_none() {
            category = get_category_with_size(category, sub_category, index, seed, settings);
        }
        let special_traits =
            generate_special_traits(neighborhood, category, sub_category, index, seed, settings);
        // !todo Think about "use_ours", our galaxy but during different era? which index "use_ours", what are the following galaxies?
        // !todo Generate a number of indexes according to the number of galaxies in a group
        Self {
            neighborhood,
            name,
            age,
            is_dominant,
            is_major,
            category,
            sub_category,
            special_traits,
        }
    }
}

/// Generates an age to use in a [Galaxy] while following the given [GenerationSettings].
fn generate_age(
    neighborhood: GalacticNeighborhood,
    index: u8,
    seed: &String,
    settings: &GenerationSettings,
) -> f32 {
    let mut age_rng = SeededDiceRoller::new(seed.as_str(), &format!("gal_{}_age", index));
    let age;
    let fixed_age = get_fixed_age(settings);

    age = if fixed_age > neighborhood.universe.age {
        fixed_age
    } else {
        neighborhood.universe.age
            - (if neighborhood.universe.era != StelliferousEra::AncientStelliferous {
                let randomize = (age_rng.roll(1, 36, 24) as f32) / 100.0;
                if age_rng.roll(1, 10, 0) == 0 {
                    neighborhood.universe.age / age_rng.roll(1, 9, 1) as f32 + randomize
                } else {
                    randomize
                }
            } else {
                (age_rng.roll(1, 16, 19) as f32) / 100.0
            })
    };
    (age * 100.0).round() / 100.0
}

/// Generates a [Galaxy] category while following the given [GenerationSettings].
fn generate_category(
    neighborhood: GalacticNeighborhood,
    index: u8,
    age: f32,
    is_dominant: bool,
    is_major: bool,
    seed: &String,
    settings: &GenerationSettings,
) -> GalaxyCategory {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_cat", index));
    let category;
    if let Some(fixed_category) = settings.galaxy.fixed_category {
        category = fixed_category;
    } else {
        match neighborhood.density {
            GalacticNeighborhoodDensity::Void(_) => {
                if is_major {
                    category = rng
                        .get_result(&CopyableRollToProcess {
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
                                    weight: if age < 50.0 { 1 } else { 4 },
                                },
                            ],
                            roll_method: RollMethod::SimpleRoll,
                        })
                        .expect("Should return a category as result");
                } else {
                    category = rng
                        .get_result(&CopyableRollToProcess {
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
                        })
                        .expect("Should return a category as result");
                }
            }
            GalacticNeighborhoodDensity::Group(_) => {
                if is_major {
                    category = rng
                        .get_result(&CopyableRollToProcess {
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
                                    } else {
                                        9
                                    },
                                },
                            ],
                            roll_method: RollMethod::SimpleRoll,
                        })
                        .expect("Should return a category as result");
                } else {
                    category = rng
                        .get_result(&CopyableRollToProcess {
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
                        })
                        .expect("Should return a category as result");
                }
            }
            GalacticNeighborhoodDensity::Cluster(_, _) => {
                if is_dominant {
                    category = GalaxyCategory::DominantElliptical(0);
                } else if is_major {
                    category = rng
                        .get_result(&CopyableRollToProcess {
                            possible_results: vec![
                                CopyableWeightedResult {
                                    result: GalaxyCategory::Intracluster(0, 0, 0),
                                    weight: 2,
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
                                    } else {
                                        16
                                    },
                                },
                            ],
                            roll_method: RollMethod::SimpleRoll,
                        })
                        .expect("Should return a category as result");
                } else {
                    category = rng
                        .get_result(&CopyableRollToProcess {
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
                        })
                        .expect("Should return a category as result");
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
    index: u8,
    seed: &String,
    settings: &GenerationSettings,
) -> GalaxyCategory {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_cws", index));
    let category_with_size;

    match sub_category {
        GalaxySubCategory::DwarfAmorphous => {
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    rng.roll(1, 165, 34) as u32 * 10,
                    rng.roll(1, 165, 34) as u32 * 10,
                    rng.roll(1, 150, 24) as u32 * 10,
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    rng.roll(1, 165, 34) as u32 * 10,
                    rng.roll(1, 165, 34) as u32 * 10,
                    rng.roll(1, 150, 24) as u32 * 10,
                );
            } else {
                category_with_size = GalaxyCategory::Intracluster(
                    rng.roll(1, 165, 34) as u32 * 10,
                    rng.roll(1, 165, 34) as u32 * 10,
                    rng.roll(1, 150, 24) as u32 * 10,
                );
            }
        }
        GalaxySubCategory::Amorphous => {
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    rng.roll(1, 530, 69) as u32 * 10,
                    rng.roll(1, 530, 69) as u32 * 10,
                    rng.roll(1, 330, 69) as u32 * 10,
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    rng.roll(1, 530, 69) as u32 * 10,
                    rng.roll(1, 530, 69) as u32 * 10,
                    rng.roll(1, 330, 69) as u32 * 10,
                );
            } else {
                category_with_size = GalaxyCategory::Intracluster(
                    rng.roll(1, 530, 69) as u32 * 10,
                    rng.roll(1, 530, 69) as u32 * 10,
                    rng.roll(1, 330, 69) as u32 * 10,
                );
            }
        }
        GalaxySubCategory::DwarfSpiral => {
            let radius = rng.roll(1, 75, 24) as u32 * 10;
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
            let radius = rng.roll(1, 75, 24) as u32 * 10;
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
            let radius = rng.roll(1, 100, 0) as u32 * 10;
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    radius * 2,
                    radius * 2,
                    radius * (rng.roll(10, 3, 0) / 10) as u32,
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    radius * 2,
                    radius * 2,
                    radius * (rng.roll(10, 3, 0) / 10) as u32,
                );
            } else if category == GalaxyCategory::Intracluster(0, 0, 0) {
                category_with_size = GalaxyCategory::Intracluster(
                    radius * 2,
                    radius * 2,
                    radius * (rng.roll(10, 4, 0) / 20) as u32,
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
    neighborhood: GalacticNeighborhood,
    category: GalaxyCategory,
    index: u8,
    age: f32,
    is_dominant: bool,
    is_major: bool,
    seed: &String,
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
                    sub_category = rng
                        .get_result(&CopyableRollToProcess {
                            possible_results: vec![
                                CopyableWeightedResult {
                                    result: GalaxySubCategory::DwarfAmorphous,
                                    weight: if age < 5.0 { 7 } else { 4 },
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
                        })
                        .expect("Should return a sub-category as result");
                }
            }
            GalaxyCategory::Spiral(_, _) => {
                if is_major {
                    sub_category = rng
                        .get_result(&CopyableRollToProcess {
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
                        })
                        .expect("Should return a sub-category as result");
                } else {
                    sub_category = GalaxySubCategory::DwarfSpiral;
                }
            }
            GalaxyCategory::Lenticular(_, _) => {
                if is_major {
                    sub_category = rng
                        .get_result(&CopyableRollToProcess {
                            possible_results: vec![
                                CopyableWeightedResult {
                                    result: GalaxySubCategory::CommonLenticular,
                                    weight: 7,
                                },
                                CopyableWeightedResult {
                                    result: GalaxySubCategory::GiantLenticular,
                                    weight: 3,
                                },
                            ],
                            roll_method: RollMethod::SimpleRoll,
                        })
                        .expect("Should return a sub-category as result");
                } else {
                    sub_category = GalaxySubCategory::DwarfLenticular;
                }
            }
            GalaxyCategory::Elliptical(_) => {
                if is_major {
                    sub_category = rng
                        .get_result(&CopyableRollToProcess {
                            possible_results: vec![
                                CopyableWeightedResult {
                                    result: GalaxySubCategory::CommonElliptical,
                                    weight: 5,
                                },
                                CopyableWeightedResult {
                                    result: GalaxySubCategory::GiantElliptical,
                                    weight: 3,
                                },
                            ],
                            roll_method: RollMethod::SimpleRoll,
                        })
                        .expect("Should return a sub-category as result");
                } else {
                    sub_category = GalaxySubCategory::DwarfElliptical;
                }
            }
            GalaxyCategory::DominantElliptical(_) => {
                sub_category = rng
                    .get_result(&CopyableRollToProcess {
                        possible_results: vec![
                            CopyableWeightedResult {
                                result: GalaxySubCategory::CommonElliptical,
                                weight: if age < 50.0 { 6 } else { 3 },
                            },
                            CopyableWeightedResult {
                                result: GalaxySubCategory::GiantElliptical,
                                weight: if age < 50.0 { 3 } else { 6 },
                            },
                        ],
                        roll_method: RollMethod::SimpleRoll,
                    })
                    .expect("Should return a sub-category as result");
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
    index: u8,
    seed: &String,
    settings: &GenerationSettings,
) -> Vec<GalaxySpecialTrait> {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_spe", index));
    let mut special_traits = vec![];

    if let Some(fixed_traits) = settings.clone().galaxy.fixed_special_traits {
        special_traits = fixed_traits;
    } else {
        let number_of_random_traits = get_number_of_random_traits(index, seed);
        let mut all_special_traits = get_full_list_of_traits(neighborhood,category,sub_category,index,seed);
        all_special_traits = remove_forbidden_traits(settings, &all_special_traits);

        // !todo Add expected ones according to category/sub-category old/young

        special_traits = add_random_traits(
            number_of_random_traits,
            all_special_traits,
            &mut special_traits,
            index,
            seed,
        );
    }

    if special_traits.len() < 1 {
        special_traits.push(GalaxySpecialTrait::NoPeculiarity);
    }
    special_traits
}

/// Returns the complete list of traits a galaxy might have.
fn get_full_list_of_traits(
    neighborhood: GalacticNeighborhood,
    category: GalaxyCategory,
    sub_category: GalaxySubCategory,
    index: u8,
    seed: &String,
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
                .expect("Should return a density."),
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
                .expect("Should return a density."),
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
                == discriminant(&GalacticNeighborhoodDensity::Cluster(0, 0))
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
                .expect("Should return a galaxy satellites qualifier."),
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
                || sub_category != GalaxySubCategory::GiantLenticular
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
                .expect("Should return a percentage of mass."),
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
                .expect("Should return a percentage of mass."),
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
                || sub_category != GalaxySubCategory::GiantElliptical
                || sub_category != GalaxySubCategory::GiantLenticular
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
                == discriminant(&GalacticNeighborhoodDensity::Cluster(0, 0))
            {
                3
            } else if discriminant(&neighborhood.density)
                == discriminant(&GalacticNeighborhoodDensity::Group(0))
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
    settings: &GenerationSettings,
    all_special_traits: &Vec<CopyableWeightedResult<GalaxySpecialTrait>>,
) -> Vec<CopyableWeightedResult<GalaxySpecialTrait>> {
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
    }
}

/// Adds **number_of_random_traits** traits from the given **special_traits**
fn add_random_traits(
    to_add: i32,
    mut possible_traits: Vec<CopyableWeightedResult<GalaxySpecialTrait>>,
    list_to_fill: &mut Vec<GalaxySpecialTrait>,
    index: u8,
    seed: &String,
) -> Vec<GalaxySpecialTrait> {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_art", index));
    let mut turn = 0;
    while turn < to_add {
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
                // !todo If opposites (ex: gaspoor/gasrich) they cancel each others

                turn += 1;
                list_to_fill.push(possible_trait);
                let index = possible_traits
                    .iter()
                    .position(|r| r.result == possible_trait)
                    .expect("Should contain the trait we found earlier");
                possible_traits.remove(index);
            }
        }
    }
    list_to_fill.to_vec()
}

/// Calculates the number of random traits this galaxy will have.
fn get_number_of_random_traits(index: u8, seed: &String) -> i32 {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_srt", index));
    let mut number_of_random_traits = 0;
    let mut roll = 0;
    let mut turn = 0;
    while roll == 50 || turn < 1 {
        roll = rng.roll(1, 50, 0) as u8;
        number_of_random_traits += if roll == 1 { 0 } else { 1 };
        turn += 1;
    }
    number_of_random_traits
}

/// Retreives an age to use in [Galaxy] generation from the given [GenerationSettings].
fn get_fixed_age(settings: &GenerationSettings) -> f32 {
    let fixed_age = if settings.galaxy.fixed_age.is_some() {
        settings.galaxy.fixed_age.expect("Fixed age should be set")
    } else if settings.galaxy.use_ours {
        OUR_GALAXYS_AGE
    } else {
        -1.0
    };
    fixed_age
}

/// Is the [Galaxy] to be generated a dominant one in its local cluster?
fn is_galaxy_dominant(neighborhood: GalacticNeighborhood, index: u8) -> bool {
    let is_dominant;
    match neighborhood.density {
        GalacticNeighborhoodDensity::Void(_) => is_dominant = false,
        GalacticNeighborhoodDensity::Group(_) => is_dominant = false,
        GalacticNeighborhoodDensity::Cluster(_, dominant) => {
            is_dominant = index < dominant;
        }
    }
    is_dominant
}

/// Is the [Galaxy] to be generated a major one in its local neighborhood?
fn is_galaxy_major(neighborhood: GalacticNeighborhood, index: u8) -> bool {
    let is_major;
    match neighborhood.density {
        GalacticNeighborhoodDensity::Void(galaxies) => {
            is_major = index < galaxies;
        }
        GalacticNeighborhoodDensity::Group(galaxies) => {
            is_major = index < galaxies;
        }
        GalacticNeighborhoodDensity::Cluster(galaxies, dominant) => {
            is_major = index < dominant + galaxies;
        }
    }
    is_major
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_a_galaxy_with_proper_size() {
        for i in 0..10000 {
            let settings = &GenerationSettings {
                galaxy: GalaxySettings {
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
            let galaxy = Galaxy::generate(neighborhood, (i as u8) % 5, &seed, &settings);
            println!("{}", galaxy);

            let category = galaxy.category;
            let sub_category = galaxy.sub_category;
            match sub_category {
                GalaxySubCategory::DwarfAmorphous => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 350 && l <= 2000 && w >= 350 && w <= 2000 && h >= 250 && h <= 1750
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 350 && l <= 2000 && w >= 350 && w <= 2000 && h >= 250 && h <= 1750
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 350 && l <= 2000 && w >= 350 && w <= 2000 && h >= 250 && h <= 1750
                        );
                    }
                }
                GalaxySubCategory::Amorphous => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 700 && l <= 6000 && w >= 700 && w <= 6000 && h >= 700 && h <= 4000
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 700 && l <= 6000 && w >= 700 && w <= 6000 && h >= 700 && h <= 4000
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 700 && l <= 6000 && w >= 700 && w <= 6000 && h >= 700 && h <= 4000
                        );
                    }
                }
                GalaxySubCategory::DwarfSpiral => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 2000 && w >= 500 && w <= 2000 && h >= 2 && h <= 60
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 2000 && w >= 500 && w <= 2000 && h >= 2 && h <= 60
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 2000 && w >= 500 && w <= 2000 && h >= 2 && h <= 60
                        );
                    } else if let GalaxyCategory::Spiral(r, t) = category {
                        assert!(r >= 250 && r <= 1000 && t >= 2 && t <= 60);
                    }
                }
                GalaxySubCategory::FlatSpiral
                | GalaxySubCategory::BarredSpiral
                | GalaxySubCategory::ClassicSpiral => {
                    if let GalaxyCategory::Spiral(r, t) = category {
                        assert!(r >= 5000 && r <= 20000 && t >= 50 && t <= 400);
                    }
                }
                GalaxySubCategory::DwarfLenticular => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 2000 && w >= 500 && w <= 2000 && h >= 2 && h <= 60
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 2000 && w >= 500 && w <= 2000 && h >= 2 && h <= 60
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 2000 && w >= 500 && w <= 2000 && h >= 2 && h <= 60
                        );
                    } else if let GalaxyCategory::Lenticular(r, t) = category {
                        assert!(r >= 250 && r <= 1000 && t >= 2 && t <= 60);
                    }
                }
                GalaxySubCategory::CommonLenticular => {
                    if let GalaxyCategory::Lenticular(r, t) = category {
                        assert!(r >= 5000 && r <= 30000 && t >= 100 && t <= 3600);
                    }
                }
                GalaxySubCategory::GiantLenticular => {
                    if let GalaxyCategory::Lenticular(r, t) = category {
                        assert!(r >= 30000 && r <= 60000 && t >= 900 && t <= 10800);
                    }
                }
                GalaxySubCategory::DwarfElliptical => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 20 && l <= 2000 && w >= 20 && w <= 2000 && h >= 5 && h <= 2000
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 20 && l <= 2000 && w >= 20 && w <= 2000 && h >= 5 && h <= 2000
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 20 && l <= 2000 && w >= 20 && w <= 2000 && h >= 5 && h <= 2000
                        );
                    } else if let GalaxyCategory::Elliptical(r) = category {
                        assert!(r >= 10 && r <= 1000);
                    }
                }
                GalaxySubCategory::CommonElliptical => {
                    if let GalaxyCategory::DominantElliptical(r) = category {
                        assert!(r >= 50000 && r <= 200000);
                    } else if let GalaxyCategory::Elliptical(r) = category {
                        assert!(r >= 1000 && r <= 20000);
                    }
                }
                GalaxySubCategory::GiantElliptical => {
                    if let GalaxyCategory::DominantElliptical(r) = category {
                        assert!(r >= 200000 && r <= 500000);
                    } else if let GalaxyCategory::Elliptical(r) = category {
                        assert!(r >= 20000 && r <= 50000);
                    }
                }
            }
        }
    }
}
