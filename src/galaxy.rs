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
            "\"{}\", {}, of sub-type {}, aged {} billion years, with the following special traits: {}",
            self.name,
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
        seed: &String,
        settings: &GenerationSettings,
    ) -> Self {
        let name = String::from("Galaxy");
        let age = Self::generate_age(neighborhood, seed, settings);
        let category = Self::generate_category(neighborhood, seed, settings);
        let sub_category = GalaxySubCategory::BarredSpiral;
        let special_traits = vec![];
        Self {
            neighborhood,
            name,
            age,
            category,
            sub_category,
            special_traits,
        }
    }

    /// Generates an age to use in a [Galaxy] while following the given [GenerationSettings].
    fn generate_age(
        neighborhood: GalacticNeighborhood,
        seed: &String,
        settings: &GenerationSettings,
    ) -> f32 {
        let age;
        let mut age_rng = SeededDiceRoller::new(seed.as_str(), "gal_age");
        let fixed_age = Self::get_fixed_age(settings);

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
        seed: &String,
        settings: &GenerationSettings,
    ) -> GalaxyCategory {
        GalaxyCategory::DominantElliptical(3)
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
}
