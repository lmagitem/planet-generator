use crate::prelude::*;

/// A list of settings used to configure the [StarSystem] generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct SystemSettings {
    /// Skip the system generation and just uses a copy of ours.
    pub use_ours: bool,
}

/// The population of stars in this system.
pub enum StellarEvolution {
    PopulationIII,
    PopulationII,
    PopulationI,
    Population0,
}
