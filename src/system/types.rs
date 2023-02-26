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
    PopulationIII,
    PopulationII,
    #[default]
    PopulationI,
    Population0,
}
