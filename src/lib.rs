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
            "GeneratedUniverse {{\n\tuniverse: {},\n\tgalactic_neighborhood: {},\n\tgalaxies:\n\t\t{}\n}}",
            self.universe,
            self.galactic_neighborhood,
            self.galaxies
                .iter()
                .map(|g| format!("{}", g))
                .collect::<Vec<String>>()
                .join(",\n\t\t")
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
        let galaxies: Vec<Galaxy> =
            generate_galaxies(universe, galactic_neighborhood, seed, settings);

        GeneratedUniverse {
            universe,
            galactic_neighborhood,
            galaxies,
        }
    }
}

/// Generates a list of [Galaxy] in the given **galactic_neighborhood** using the given **seed** and **settings**.
fn generate_galaxies(
    universe: Universe,
    galactic_neighborhood: GalacticNeighborhood,
    seed: &String,
    settings: GenerationSettings,
) -> Vec<Galaxy> {
    let mut rng = SeededDiceRoller::new(seed, &format!("main_gal"));
    let mut galaxies: Vec<Galaxy> = vec![];
    let to_generate: u8;
    let minor_to_generate: u8 = if universe.era == StelliferousEra::EndStelliferous {
        0
    } else if universe.era == StelliferousEra::LateStelliferous {
        rng.roll(1, 3, 0) as u8
    } else {
        rng.roll(1, 6, 3) as u8
    };
    match galactic_neighborhood.density {
        GalacticNeighborhoodDensity::Void(g) | GalacticNeighborhoodDensity::Group(g) => {
            to_generate = minor_to_generate + g
        }
        GalacticNeighborhoodDensity::Cluster(g, d) => to_generate = minor_to_generate + g + d,
    }
    for i in 0..to_generate {
        galaxies.push(Galaxy::generate(galactic_neighborhood, i, seed, &settings));
    }
    galaxies
}
