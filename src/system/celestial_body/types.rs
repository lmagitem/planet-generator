use crate::internal::*;
use crate::prelude::*;
use std::fmt;

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
    // Exotic(ExoticBodyDetails),
    Telluric(TelluricBodyDetails),
    Gaseous(GaseousBodyDetails),
    Icy(IcyBodyDetails),
    Cloud(CelestialBodySubType),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum CelestialBodySubType {
    // Exotic,
    Metallic,
    Rocky,
    Gaseous,
    Icy,
}

impl Display for CelestialBodySubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialBodySubType::Metallic => "Metallic",
                CelestialBodySubType::Rocky => "Rocky",
                CelestialBodySubType::Gaseous => "Gaseous",
                CelestialBodySubType::Icy => "Icy",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum CelestialBodySize {
    /// Bodies at least halfway through being massive enough to sustain deuterium fusion in their core.
    Hypergiant,
    /// A world whose size is akin Jupiter's size.
    Supergiant,
    /// A world large enough to retain hydrogen.
    Giant,
    /// A world large enough to retain helium gas.
    Large,
    /// A world large enough to retain water vapor in its atmosphere.
    Standard,
    /// A world large enough to retain molecular nitrogen. Titan and Mars lie within this category.
    Small,
    /// A world too small to retain significant atmosphere, think of bodies like Mercury, the Moon, Callisto, Europa, Io...
    Tiny,
    /// A body that isn't big enough for its self-gravity to overcome rigid body forces and assume an ellipsoidal shape in equilibrium.
    Moonlet,
}

impl Display for CelestialBodySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialBodySize::Hypergiant => "Hypergiant",
                CelestialBodySize::Supergiant => "Supergiant",
                CelestialBodySize::Giant => "Giant",
                CelestialBodySize::Large => "Large",
                CelestialBodySize::Standard => "Standard",
                CelestialBodySize::Small => "Small",
                CelestialBodySize::Tiny => "Tiny",
                CelestialBodySize::Moonlet => "Asteroid-sized",
            }
        )
    }
}
