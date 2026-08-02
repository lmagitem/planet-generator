use crate::diagnostics::types::{Diagnostics, GenerationError};
use crate::galaxy::neighborhood::types::GalacticNeighborhoodDensity;
use crate::galaxy::neighborhood::GalacticNeighborhood;
use crate::galaxy::Galaxy;
use crate::generator::types::GeneratedUniverse;
use crate::generator::types::GenerationSettings;
use crate::universe::Universe;
use serde::{Deserialize, Serialize};
pub mod types;

/// The generator itself, depending on the given settings, can generate a full blown universe with multiple galaxies, sectors, systems,
/// planets and the species living in those.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Generator {}

impl Generator {
    /// Generates a full blown universe with multiple galaxies, sectors, systems, planets and the species living in those following the
    /// given [`GenerationSettings`], in a deterministic way thanks to the given **seed**. Validates the settings first and returns
    /// [`GenerationError::InvalidSettings`] listing every violation found if any.
    ///
    /// # Errors
    /// Returns [`GenerationError::InvalidSettings`] if the given settings fail validation.
    pub fn generate(settings: GenerationSettings) -> Result<GeneratedUniverse, GenerationError> {
        if let Err(violations) = settings.validate() {
            return Err(GenerationError::InvalidSettings(violations));
        }
        let mut diagnostics = Diagnostics::default();
        let universe = Universe::generate(&settings, &mut diagnostics);
        let galactic_neighborhood =
            GalacticNeighborhood::generate(universe, &settings, &mut diagnostics);
        let galaxies: Vec<Galaxy> = generate_galaxies(galactic_neighborhood, settings);

        Ok(GeneratedUniverse {
            universe,
            galactic_neighborhood,
            galaxies,
            diagnostics,
        })
    }
}

/// Generates a list of [Galaxy] in the given **galactic_neighborhood** using the given **seed** and **settings**.
fn generate_galaxies(
    galactic_neighborhood: GalacticNeighborhood,
    settings: GenerationSettings,
) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];
    let to_generate: u16;
    match galactic_neighborhood.density {
        GalacticNeighborhoodDensity::Void(g, m) | GalacticNeighborhoodDensity::Group(g, m) => {
            to_generate = (g as u16) + m
        }
        GalacticNeighborhoodDensity::Cluster(d, g, m) => to_generate = (d as u16) + (g as u16) + m,
    }
    for i in 0..to_generate {
        galaxies.push(Galaxy::generate(galactic_neighborhood, i, &settings));
    }
    galaxies
}
