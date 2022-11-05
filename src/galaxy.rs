use crate::prelude::*;

/// Defines the density of a galactic neighborhood. The associated number indicates how many major galaxies we find in that neighborhood.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum GalacticNeighborhoodDensity {
    /// The emptiest parts of the universe, covers a diameter far greater than the other densities. Contains 0 to 3 major galaxies.
    VOID(u8),
    /// A zone filled with a "regular" amount of galaxies. Contains 1 to 5 major galaxies.
    #[default]
    GROUP(#[default = 2] u8),
    /// The most crowded parts of the universe. Galaxies within this neighborhood usualy revolve around a huge dominant one. Space between
    /// galaxies is filled with super-hot plasma and a large number of intergalactic stars. Contains 5 to 20+ major galaxies.
    CLUSTER(u8),
}

/// Data allowing us to model a galaxy Neighborhood (a section of the universe containing multiple galaxies).
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalacticNeighborhood {
    /// The universe this neighborhood belongs to.
    pub universe: Universe,
    /// How dense is this neighborhood, with the precise number of galaxies present.
    pub density: GalacticNeighborhoodDensity,
}

/// Data allowing us to model a galaxy.
pub struct Galaxy {
    /// The neighborhood this galaxy belongs to.
    pub neighborhood: GalacticNeighborhood,
}

impl Galaxy {
    pub fn generate(universe: Universe, settings: GenerationSettings, seed: String) -> Self {
        // todo
        Self {
            neighborhood: GalacticNeighborhood {
                universe,
                density: GalacticNeighborhoodDensity::GROUP(3),
            },
        }
    }
}
