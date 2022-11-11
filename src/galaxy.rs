use crate::prelude::*;

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
    pub fn generate(
        neighborhood: GalacticNeighborhood,
        seed: &String,
        settings: GenerationSettings,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }
}
