use crate::prelude::*;
#[path = "./constants.rs"]
mod constants;
use constants::*;
pub mod generator;
pub mod map;
pub mod neighborhood;
pub mod types;

/// Data allowing us to model a galaxy.
#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Galaxy {
    /// The settings to use when generating content.
    pub settings: GenerationSettings,
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
    /// The specific division levels used to map this galaxy's content.
    pub division_levels: Vec<GalacticMapDivisionLevel>,
    /// This galaxy's already generated divisions.
    pub divisions: Vec<GalacticMapDivision>,
    /// This galaxy's already generated hexagons.
    pub hexes: Vec<GalacticHex>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Self {
            settings: GenerationSettings {
                ..Default::default()
            },
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
            division_levels: vec![],
            divisions: vec![],
            hexes: vec![],
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
        settings: GenerationSettings,
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
            settings,
            neighborhood,
            index,
            name,
            age,
            is_dominant,
            is_major,
            category,
            sub_category,
            special_traits,
            division_levels: vec![],
            divisions: vec![],
            hexes: vec![],
        }
    }
}
