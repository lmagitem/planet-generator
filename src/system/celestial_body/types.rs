use crate::prelude::*;

/// A list of settings used to configure the [CelestialBody] generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct CelestialBodySettings {
    /// A list of settings used to configure the Gaseous Bodies (like gas giants) generation.
    pub gaseous_body_settings: GaseousBodySettings,
    /// A list of settings used to configure the Icy Bodies (like ice giants) generation.
    pub icy_body_settings: IcyBodySettings,
    /// A list of settings used to configure the Telluric Bodies (like rocky planets) generation.
    pub telluric_body_settings: TelluricBodySettings,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum CelestialBodyDetails {
    Telluric(TelluricDetails),
    Gaseous(GaseousDetails),
    Icy(IcyDetails),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum CelestialBodySubtype {
    Metallic,
    Rocky,
    Gaseous,
    Icy,
}
