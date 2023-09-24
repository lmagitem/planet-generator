use crate::prelude::*;

/// A list of settings used to configure the [CelestialBody] generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct CelestialBodySettings {
    /// A list of settings used to configure the Gaseous Bodies (like gas giants) generation.
    pub gaseous_body_settings: GaseousBodySettings,
    /// A list of settings used to configure the Icy Bodies (like ice giants) generation.
    pub icy_body_settings: IcyBodySettings,
    /// A list of settings used to configure the Telluric Bodies (like rocky planets) generation.
    pub telluric_body_settings: TelluricBodySettings,
    /// During the filling of an orbit in the system's contents generation, do not add gaseous bodies.
    #[default(false)]
    pub do_not_generate_gaseous: bool,
    /// During the filling of an orbit in the system's contents generation, do not add icy bodies.
    #[default(false)]
    pub do_not_generate_icy: bool,
    /// During the filling of an orbit in the system's contents generation, do not add rocky bodies.
    #[default(false)]
    pub do_not_generate_rocky: bool,
    /// During the filling of an orbit in the system's contents generation, do not add metallic bodies.
    #[default(false)]
    pub do_not_generate_metallic: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum CelestialBodyDetails {
    Telluric(TelluricBodyDetails),
    Gaseous(GaseousBodyDetails),
    Icy(IcyBodyDetails),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum CelestialBodySubType {
    Metallic,
    Rocky,
    Gaseous,
    Icy,
}
