use crate::internal::*;
use crate::prelude::*;

/// A list of settings used to configure generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct GenerationSettings {
    /// The seed to use to generate everything.
    #[default("default")]
    pub seed: Rc<str>,
    /// A list of settings used to configure the [Universe] generation.
    pub universe: UniverseSettings,
    /// A list of settings used to configure the [Galaxy] generation.
    pub galaxy: GalaxySettings,
    /// A list of settings used to configure the [GalacticMapDivisionLevel], [GalacticMapDivision]s and [GalacticHex]es generation.
    pub sector: SectorSettings,
    /// A list of settings used to configure the [StarSystem] generation.
    pub system: SystemSettings,
    /// A list of settings used to configure the [Star] generation.
    pub star: StarSettings,
    /// A list of settings used to configure the [CelestialBody] generation.
    pub celestial_body: CelestialBodySettings,
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
