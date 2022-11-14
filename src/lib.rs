#![warn(clippy::all, clippy::pedantic)]
mod galaxy;
mod galaxy_types;
mod planet;
mod planet_types;
mod sector;
mod sector_types;
mod system;
mod system_types;
mod universe;
mod universe_types;

pub mod prelude {
    pub use crate::galaxy::*;
    pub use crate::galaxy_types::*;
    pub use crate::planet::*;
    pub use crate::planet_types::*;
    pub use crate::sector::*;
    pub use crate::sector_types::*;
    pub use crate::system::*;
    pub use crate::system_types::*;
    pub use crate::universe::*;
    pub use crate::universe_types::*;
    pub use crate::GeneratedUniverse;
    pub use crate::GenerationSettings;
    pub use crate::Generator;
    pub use log::*;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
}
use prelude::*;

/// A list of settings used to configure generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GenerationSettings {
    /// A list of settings used to configure the [Universe] generation.
    pub universe: UniverseSettings,
    /// A list of settings used to configure the [Galaxy] generation.
    pub galaxy: GalaxySettings,
}

/// Data object filled with the results of a generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GeneratedUniverse {
    /// The generated [Universe].
    pub universe: Universe,
    /// The generated [GalacticNeighborhood].
    pub galactic_neighborhood: GalacticNeighborhood,
    /// A list of generated [Galaxy] objects.
    pub galaxies: Vec<Galaxy>,
}

impl Display for GeneratedUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GeneratedUniverse {{ universe: {}, galactic_neighborhood: {}, galaxies: {} }}",
            self.universe,
            self.galactic_neighborhood,
            self.galaxies
                .iter()
                .map(|g| format!("{}", g))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// The generator itself, depending on the given settings, can generate a full blown universe with multiple galaxies, sectors, systems,
/// planets and the species living in those.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Generator {}

impl Generator {
    /// Generates a full blown universe with multiple galaxies, sectors, systems, planets and the species living in those following the
    /// given [GenerationSettings], in a deterministic way thanks to the given **seed**.
    pub fn generate(seed: &String, settings: GenerationSettings) -> GeneratedUniverse {
        let universe = Universe::generate(seed, &settings);
        let galactic_neighborhood = GalacticNeighborhood::generate(universe, seed, &settings);
        let mut galaxies: Vec<Galaxy> = Vec::new();

        galaxies.push(Galaxy::generate(galactic_neighborhood, 0, seed, &settings));

        GeneratedUniverse {
            universe,
            galactic_neighborhood,
            galaxies,
        }
    }
}
