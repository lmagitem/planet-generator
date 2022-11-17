use crate::prelude::*;

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
