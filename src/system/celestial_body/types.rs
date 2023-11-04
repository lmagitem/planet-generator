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
    Cloud(CelestialBodyComposition),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum CelestialBodyComposition {
    // Exotic,
    Metallic,
    Rocky,
    Gaseous,
    Icy,
}

impl Display for CelestialBodyComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialBodyComposition::Metallic => "Metallic",
                CelestialBodyComposition::Rocky => "Rocky",
                CelestialBodyComposition::Gaseous => "Gaseous",
                CelestialBodyComposition::Icy => "Icy",
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum CelestialBodyWorldType {
    /// Worlds cold enough to have water ice and similar frozen volatiles on its surface. May have liquid oceans under the surface if conditions are ok.
    Ice,
    /// Worlds orbiting gas giants that experience tremendous amount of volcanic activity because of their proximity to the giant and other moons.
    Sulfur,
    /// Worlds not large enough to retain water vapor, and too hot to have much ice without atmosphere.
    Rock,
    /// Worlds large enough to retain gaseous nitrogen but so cold that their nitrogen atmosphere is frozen on the surface.
    Hadean,
    /// Worlds large enough to retain a thick atmosphere, but so cold that water is always frozen. Instead, the atmosphere is mainly composed of
    /// ammonia and methane, and oceans are made of liquid ammonia with substantial amounts of water. Very unlikely around stars brighter than red dwarfs,
    /// for ammonia breaks down quickly when exposed to ultraviolet light.
    Ammonia,
    /// Worlds large enough to retain a thick atmosphere and are almost or entirely covered by oceans.
    Ocean,
    /// Worlds large enough to retain a thick atmosphere and plenty of water.
    Terrestrial,
    /// Worlds large enough to retain a thick atmosphere and plenty of water, but which became too hot and experienced a greenhouse effect. Some still have
    /// oceans of surface water. The air is unbreathable and furnace-hot and the planet is extremely hostile.
    Greenhouse,
    /// Worlds that would be large enough to retain a thick atmosphere, but that are so close to their star that almost all their volatiles have been stripped
    /// away. There may still be a tenuous atmosphere left, but likely composed of vaporized metals.
    Chthonian,
    /// Worlds that are mostly made of an endless atmosphere of volatiles, like ice and gas giants.
    VolatilesGiant,
}

impl Display for CelestialBodyWorldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialBodyWorldType::Ice => "Ice",
                CelestialBodyWorldType::Sulfur => "Sulfur",
                CelestialBodyWorldType::Rock => "Rock",
                CelestialBodyWorldType::Hadean => "Hadean",
                CelestialBodyWorldType::Ammonia => "Ammonia",
                CelestialBodyWorldType::Ocean => "Ocean",
                CelestialBodyWorldType::Terrestrial => "Terrestrial",
                CelestialBodyWorldType::Greenhouse => "Greenhouse",
                CelestialBodyWorldType::Chthonian => "Chthonian",
                CelestialBodyWorldType::VolatilesGiant => "Volatiles Giant",
            }
        )
    }
}
