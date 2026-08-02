use crate::diagnostics::Diagnostics;
use crate::galaxy::map::division::GalacticMapDivision;
use crate::galaxy::map::division_level::GalacticMapDivisionLevel;
use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::hex::GalacticHex;
use crate::galaxy::map::region_map::types::GalacticRegionMap;
use crate::galaxy::neighborhood::GalacticNeighborhood;
use crate::galaxy::types::GalaxyCategory;
use crate::galaxy::types::GalaxySpecialTrait;
use crate::galaxy::types::GalaxySubCategory;
use crate::generator::types::GenerationSettings;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::rc::Rc;
pub(crate) mod constants;
pub use constants::{DEFAULT_REGION_MAP_SPIRAL, EXAMPLE_REGION_MAP_SVG};
use constants::{
    NO_SPECIAL_TRAIT, OUR_GALAXYS_AGE, OUR_GALAXYS_CATEGORY, OUR_GALAXYS_NAME,
    OUR_GALAXYS_SUB_CATEGORY,
};
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
    pub name: Rc<str>,
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
    /// This galaxy's already generated divisions, keyed by `(level, index)`.
    /// Serialized as `(key, value)` pairs: JSON maps require string keys, which coordinates are not.
    #[serde(with = "crate::galaxy::map::serde_helpers")]
    pub divisions: BTreeMap<(u8, SpaceCoordinates), GalacticMapDivision>,
    /// This galaxy's already generated hexagons, keyed by hex index.
    #[serde(with = "crate::galaxy::map::serde_helpers")]
    pub hexes: BTreeMap<SpaceCoordinates, GalacticHex>,
    /// The SVG layers effectively used for this galaxy's region map (the user's, or the category's default map).
    /// This field is the authoritative source when the region map is rebuilt.
    #[serde(default)]
    pub galactic_map_layers: Vec<String>,
    /// The parsed and rasterized region-map cache. Rebuilt automatically from `galactic_map_layers` when missing.
    /// Note that `Debug` output includes every rasterized cell, and that a deserialized galaxy (whose cache is
    /// `None`) compares unequal to a healed one until the first map query rebuilds it.
    #[serde(skip)]
    pub region_map: Option<GalacticRegionMap>,
    /// The non-fatal issues raised while generating this galaxy (e.g. unusable
    /// user-provided SVG map layers). Transient: deliberately not serialized.
    /// Note that a deserialized galaxy (whose diagnostics are rebuilt empty)
    /// compares unequal to the original whenever the original's diagnostics
    /// were non-empty.
    #[serde(skip)]
    pub diagnostics: Diagnostics,
}

impl Default for Galaxy {
    /// Returns a bare-bones Milky Way look-alike, intended as a convenient
    /// starting point for tests and demos.
    ///
    /// **Warning:** this galaxy is NOT ready for map queries. `division_levels`
    /// and `galactic_map_layers` are empty, so [`Galaxy::get_hex`],
    /// [`Galaxy::get_division_at_level`] and [`Galaxy::get_divisions_for_coord`]
    /// will not behave like they do on a generated galaxy (level 0 is missing,
    /// and no region map can be built). Fill `division_levels` via
    /// [`GalacticMapDivisionLevel::generate_division_levels`] and set
    /// `galactic_map_layers` (e.g. to `DEFAULT_REGION_MAP_SPIRAL`) first — or
    /// just use [`Galaxy::generate`].
    fn default() -> Self {
        Self {
            settings: GenerationSettings {
                ..Default::default()
            },
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            index: 0,
            name: OUR_GALAXYS_NAME.into(),
            age: OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: OUR_GALAXYS_CATEGORY,
            sub_category: OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![NO_SPECIAL_TRAIT],
            division_levels: vec![],
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers: vec![],
            region_map: None,
            diagnostics: Diagnostics::default(),
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
