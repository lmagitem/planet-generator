use crate::prelude::*;

/// A list of settings used to configure the [StarSystem] generation.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct SystemSettings {
    /// Skip the system generation and just uses a copy of ours.
    pub use_ours: bool,
}

/// The population of stars in this system.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub enum StellarEvolution {
    Paleodwarf,
    Subdwarf,
    #[default]
    Dwarf,
    Superdwarf,
    Hyperdwarf,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub enum SystemPeculiarity {
    CarbonRich,
    Cataclysm(),
    UnusualDebrisDensity(),
    Nebulae(),
    #[default]
    NoPeculiarity,
}

impl Display for SystemPeculiarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemPeculiarity::CarbonRich => {}
            SystemPeculiarity::Cataclysm() => {}
            SystemPeculiarity::UnusualDebrisDensity() => {}
            SystemPeculiarity::Nebulae() => {}
            SystemPeculiarity::NoPeculiarity => {}
        }
    }
}
