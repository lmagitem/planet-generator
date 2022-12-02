use crate::prelude::*;
pub mod generator;
pub mod types;

/// Data allowing us to model a galaxy Neighborhood (a section of the universe containing multiple galaxies).
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalacticNeighborhood {
    /// The universe this neighborhood belongs to.
    pub universe: Universe,
    /// How dense is this neighborhood, with the number of galaxies present.
    pub density: GalacticNeighborhoodDensity,
}

impl Display for GalacticNeighborhood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Galactic {}", self.density)
    }
}

impl GalacticNeighborhood {
    /// Returns a new [GalacticNeighborhood] using the given arguments.
    pub fn new(universe: Universe, density: GalacticNeighborhoodDensity) -> Self {
        Self { universe, density }
    }
}
