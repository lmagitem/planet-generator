#[path = "./constants.rs"]
mod galaxy_constants;
use crate::prelude::*;
use galaxy_constants::*;

/// Data allowing us to model a galaxy.
#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Galaxy {
    /// The neighborhood this galaxy belongs to.
    pub neighborhood: GalacticNeighborhood,
    /// The numeric identifier of this galaxy in its neighborhood.
    pub index: u16,
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
            index: 0,
            name: String::from(OUR_GALAXYS_NAME),
            age: OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: OUR_GALAXYS_CATEGORY,
            sub_category: OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![NO_SPECIAL_TRAIT],
        }
    }
}

impl Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04} - \"{}\" - {}{}, of sub-type {}, aged {} billion years, with the following special traits: {}",
            self.index,
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
    /// Returns a new [Galaxy] using the given arguments.
    pub fn new(
        neighborhood: GalacticNeighborhood,
        index: u16,
        name: String,
        age: f32,
        is_dominant: bool,
        is_major: bool,
        category: GalaxyCategory,
        sub_category: GalaxySubCategory,
        special_traits: Vec<GalaxySpecialTrait>,
    ) -> Self {
        Self {
            neighborhood,
            index,
            name,
            age,
            is_dominant,
            is_major,
            category,
            sub_category,
            special_traits,
        }
    }

    /// Generates a brand new [Galaxy] using the given seed and [GenerationSettings].
    pub fn generate(
        neighborhood: GalacticNeighborhood,
        index: u16,
        seed: &String,
        settings: &GenerationSettings,
    ) -> Self {
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
            name = String::from(model.name);
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
            name = String::from("Galaxy");
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

        Self {
            neighborhood,
            index,
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
    index: u16,
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
    seed: &String,
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
            GalacticNeighborhoodDensity::Group(_, _) => {
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
                                    } else if age < 750.0 {
                                        9
                                    } else {
                                        12
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
            GalacticNeighborhoodDensity::Cluster(_, _, _) => {
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
    index: u16,
    seed: &String,
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
            if category == GalaxyCategory::Intergalactic(0, 0, 0) {
                category_with_size = GalaxyCategory::Intergalactic(
                    rng.roll(1, 1125, 124) as u32 * 10,
                    rng.roll(1, 1125, 124) as u32 * 10,
                    rng.roll(1, 900, 99) as u32 * 10,
                );
            } else if category == GalaxyCategory::Irregular(0, 0, 0) {
                category_with_size = GalaxyCategory::Irregular(
                    rng.roll(1, 1125, 124) as u32 * 10,
                    rng.roll(1, 1125, 124) as u32 * 10,
                    rng.roll(1, 900, 99) as u32 * 10,
                );
            } else {
                category_with_size = GalaxyCategory::Intracluster(
                    rng.roll(1, 1125, 124) as u32 * 10,
                    rng.roll(1, 1125, 124) as u32 * 10,
                    rng.roll(1, 900, 99) as u32 * 10,
                );
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
            let radius = rng.roll(1, 500, 0) as u32 * 10;
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
                                    weight: if age < 500.0 { 3 } else { 10 },
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
                                    weight: if age < 500.0 { 3 } else { 8 },
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
    index: u16,
    seed: &String,
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
    seed: &String,
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
    seed: &String,
) -> Vec<GalaxySpecialTrait> {
    let mut rng = SeededDiceRoller::new(seed, &format!("gal_{}_art", index));
    let opposite_traits = get_opposite_traits();
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
fn get_number_of_random_traits(index: u16, seed: &String) -> i32 {
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
fn get_fixed_age(settings: &GenerationSettings) -> f32 {
    let fixed_age = if settings.galaxy.fixed_age.is_some() {
        settings.galaxy.fixed_age.expect("Fixed age should be set")
    } else {
        -1.0
    };
    fixed_age
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
            let galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &seed, &settings);
            println!("{}", galaxy);

            let category = galaxy.category;
            let sub_category = galaxy.sub_category;
            match sub_category {
                GalaxySubCategory::DwarfAmorphous => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 100 && l <= 4000 && w >= 100 && w <= 4000 && h >= 75 && h <= 3000
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 100 && l <= 4000 && w >= 100 && w <= 4000 && h >= 75 && h <= 3000
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 100 && l <= 4000 && w >= 100 && w <= 4000 && h >= 75 && h <= 3000
                        );
                    }
                }
                GalaxySubCategory::Amorphous => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 1250
                                && l <= 12500
                                && w >= 1250
                                && w <= 12500
                                && h >= 1000
                                && h <= 10000
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 1250
                                && l <= 12500
                                && w >= 1250
                                && w <= 12500
                                && h >= 1000
                                && h <= 10000
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 1250
                                && l <= 12500
                                && w >= 1250
                                && w <= 12500
                                && h >= 1000
                                && h <= 10000
                        );
                    }
                }
                GalaxySubCategory::DwarfSpiral => {
                    if let GalaxyCategory::Intergalactic(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 10000 && w >= 500 && w <= 10000 && h >= 2 && h <= 300
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 10000 && w >= 500 && w <= 10000 && h >= 2 && h <= 300
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 10000 && w >= 500 && w <= 10000 && h >= 2 && h <= 300
                        );
                    } else if let GalaxyCategory::Spiral(r, t) = category {
                        assert!(r >= 250 && r <= 5000 && t >= 2 && t <= 300);
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
                            l >= 500 && l <= 10000 && w >= 500 && w <= 10000 && h >= 2 && h <= 300
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 10000 && w >= 500 && w <= 10000 && h >= 2 && h <= 300
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 500 && l <= 10000 && w >= 500 && w <= 10000 && h >= 2 && h <= 300
                        );
                    } else if let GalaxyCategory::Lenticular(r, t) = category {
                        assert!(r >= 250 && r <= 5000 && t >= 2 && t <= 300);
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
                            l >= 20 && l <= 10000 && w >= 20 && w <= 10000 && h >= 5 && h <= 10000
                        );
                    } else if let GalaxyCategory::Irregular(l, w, h) = category {
                        assert!(
                            l >= 20 && l <= 10000 && w >= 20 && w <= 10000 && h >= 5 && h <= 10000
                        );
                    } else if let GalaxyCategory::Intracluster(l, w, h) = category {
                        assert!(
                            l >= 20 && l <= 10000 && w >= 20 && w <= 10000 && h >= 5 && h <= 10000
                        );
                    } else if let GalaxyCategory::Elliptical(r) = category {
                        assert!(r >= 10 && r <= 5000);
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
