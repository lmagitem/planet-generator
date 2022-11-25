use crate::prelude::*;

/// Every content already generated for this galaxy, and everything necessary to generate more.
pub struct GalaxyInsides {
    /// The generation seed to use when generating content.
    pub seed: String,
    /// The settings to use when generating content.
    pub settings: GenerationSettings,
    /// The specific division levels used to map this galaxy's content.
    pub division_levels: Vec<SpaceDivisionLevel>,
    /// This galaxy's already generated divisions.
    pub divisions: Vec<SpaceDivision>,
    /// This galaxy's already generated hexagons.
    pub hexes: Vec<SpaceHex>,
}
