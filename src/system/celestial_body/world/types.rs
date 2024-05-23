use crate::internal::*;
use std::fmt;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum CelestialBodyWorldType {
    /// Worlds mainly made of ices and cold enough to have water ice and similar frozen volatiles on its surface. May have liquid oceans under the surface if conditions are ok.
    Ice,
    /// Worlds mainly made of rocks and cold enough to have water ice and similar frozen volatiles on its surface.
    DirtySnowball,
    /// Worlds orbiting gas giants that experience tremendous amount of volcanic activity because of their proximity to the giant and other moons.
    Sulfur,
    /// Worlds not large enough to retain water vapor, and too hot to have much ice without atmosphere.
    #[default]
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
                CelestialBodyWorldType::Ice => "Ice Ball",
                CelestialBodyWorldType::DirtySnowball => "Dirty Snowball",
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

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum CelestialBodyCoreHeat {
    /// The core is largely inactive, leading to minimal geological activity, and often results
    /// in a lack of magnetic field and tectonic movement.
    #[default]
    FrozenCore,
    /// The core retains some residual heat, providing limited geological and possibly volcanic
    /// activity, but with a weaker impact on the planet's magnetic field and surface.
    WarmCore,
    /// A significantly heated core driving robust geological processes, including volcanism and
    /// tectonics, often accompanied by a stronger magnetic field.
    ActiveCore,
    /// The core is extremely hot, fueling vigorous geological activity, potentially including
    /// powerful volcanism and dynamic tectonics, and usually results in a strong magnetic field.
    IntenseCore,
}

impl Display for CelestialBodyCoreHeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CelestialBodyCoreHeat::FrozenCore => write!(f, "Frozen Core"),
            CelestialBodyCoreHeat::WarmCore => write!(f, "Warm Core"),
            CelestialBodyCoreHeat::ActiveCore => write!(f, "Active Core"),
            CelestialBodyCoreHeat::IntenseCore => write!(f, "Intense Core"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum MagneticFieldStrength {
    /// No magnetic field (0 Gauss), leading to increased surface radiation and faster atmospheric
    /// loss. Potentially more hostile to life.
    #[default]
    None,
    /// Weak magnetic field (0.1 - 0.3 Gauss, similar to Mars), offering minimal protection against
    /// solar wind and some atmospheric erosion. Limited shielding from cosmic radiation.
    Weak,
    /// Moderate magnetic field (0.3 - 1 Gauss, similar to the Moon or Mercury), providing moderate
    /// protection and can deflect some solar wind. May sustain a thin atmosphere with reduced
    /// atmospheric erosion.
    Moderate,
    /// Strong magnetic field (1 - 100 Gauss, similar to Earth), ensuring strong protection against
    /// solar and cosmic radiation. Supports preservation of atmosphere and potential for complex life.
    Strong,
    /// Very strong magnetic field (100 - 10,000 Gauss, similar to Gas Giants), providing extremely
    /// effective atmospheric retention and strong auroras. Can create radiation belts that might be
    /// hazardous for life or technology.
    VeryStrong,
    /// Extremely strong magnetic field (>10,000 Gauss, akin to Neutron Stars and Magnetars), found
    /// in exotic astrophysical objects. Likely uninhabitable due to extreme phenomena and strong
    /// radiation emissions.
    Extreme,
}

impl std::fmt::Display for MagneticFieldStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagneticFieldStrength::None => write!(f, "No Magnetic Field"),
            MagneticFieldStrength::Weak => write!(f, "Weak Magnetic Field"),
            MagneticFieldStrength::Moderate => write!(f, "Moderate Magnetic Field"),
            MagneticFieldStrength::Strong => write!(f, "Strong Magnetic Field"),
            MagneticFieldStrength::VeryStrong => write!(f, "Very Strong Magnetic Field"),
            MagneticFieldStrength::Extreme => write!(f, "Extreme Magnetic Field"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum VolcanicActivity {
    /// No volcanic activity. Geologically dead world with potentially solid core. Trace elements
    /// and useful minerals are absent or very rare. Examples: Mercury, Mars, the Moon.
    #[default]
    None,
    /// Light volcanic activity. Quiet world with a hot core and sporadic volcanism. Trace elements
    /// are uncommon. New volcanoes appear over thousands or millions of years.
    Light,
    /// Moderate volcanic activity. Active regions with occasional eruptions. Trace elements are
    /// well-circulated. New volcanoes appear over decades or centuries. Example: Earth.
    Moderate,
    /// Heavy volcanic activity. Frequent and widespread volcanism. Magma can emerge through
    /// large crustal wounds. Trace elements are abundant. New volcanoes appear every few years.
    Heavy,
    /// Extreme volcanic activity. Dominant and continuous volcanism making the surface highly
    /// unstable. Atmosphere likely non-breathable with high sulfur compounds or pollutants.
    /// Example: Io.
    Extreme,
}

impl std::fmt::Display for VolcanicActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolcanicActivity::None => write!(f, "No Volcanic Activity"),
            VolcanicActivity::Light => write!(f, "Light Volcanic Activity"),
            VolcanicActivity::Moderate => write!(f, "Moderate Volcanic Activity"),
            VolcanicActivity::Heavy => write!(f, "Heavy Volcanic Activity"),
            VolcanicActivity::Extreme => write!(f, "Extreme Volcanic Activity"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum TectonicActivity {
    /// No tectonic activity. Tectonically dead world with a thick, immobile crust. Weak and rare
    /// crustal quakes. Massive shield volcanoes if any volcanic activity. No mountain-building.
    #[default]
    None,
    /// Light tectonic activity. Limited movement of a few crustal plates. Powerful but uncommon
    /// quakes. Shield volcanoes and some smaller volcanic chains. Infrequent mountain-building.
    Light,
    /// Moderate tectonic activity. Freely moving number of plates. Common and powerful quakes,
    /// including mid-plate regions. Few shield volcanoes, mostly smaller chain volcanoes. Regular mountain-building cycles.
    Moderate,
    /// Heavy tectonic activity. Many crustal plates with frequent movement. Very powerful quakes
    /// almost anywhere. Small chain or arc volcanoes. Constant mountain-building with young, high ranges.
    Heavy,
    /// Extreme tectonic activity. Fragmented, unstable crust. Powerful quakes at any time and place.
    /// Chaotic landforms, including numerous young, high mountains. Highly dynamic geological environment.
    Extreme,
}

impl std::fmt::Display for crate::prelude::TectonicActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::prelude::TectonicActivity::None => write!(f, "No Tectonic Activity"),
            crate::prelude::TectonicActivity::Light => write!(f, "Light Tectonic Activity"),
            crate::prelude::TectonicActivity::Moderate => write!(f, "Moderate Tectonic Activity"),
            crate::prelude::TectonicActivity::Heavy => write!(f, "Heavy Tectonic Activity"),
            crate::prelude::TectonicActivity::Extreme => write!(f, "Extreme Tectonic Activity"),
        }
    }
}

/// A descriptive name for the world’s over-all surface climate.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum WorldClimateType {
    /// Temperature below 244 K (-29.15°C).
    #[default]
    Frozen,
    /// Temperature between 244 K (-29.15°C) and 255 K (-18.15°C).
    VeryCold,
    /// Temperature between 255 K (-18.15°C) and 266 K (-7.15°C).
    Cold,
    /// Temperature between 266 K (-7.15°C) and 278 K (4.85°C).
    Chilly,
    /// Temperature between 278 K (4.85°C) and 289 K (15.85°C).
    Cool,
    /// Temperature between 289 K (15.85°C) and 300 K (26.85°C).
    Ideal,
    /// Temperature between 300 K (26.85°C) and 311 K (37.85°C).
    Warm,
    /// Temperature between 311 K (37.85°C) and 322 K (48.85°C).
    Tropical,
    /// Temperature between 322 K (48.85°C) and 333 K (59.85°C).
    Hot,
    /// Temperature between 333 K (59.85°C) and 344 K (70.85°C).
    VeryHot,
    /// Temperature above 344 K (70.85°C).
    Infernal,
}

impl std::fmt::Display for crate::prelude::WorldClimateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::prelude::WorldClimateType::Frozen => write!(f, "Frozen"),
            crate::prelude::WorldClimateType::VeryCold => write!(f, "Very Cold"),
            crate::prelude::WorldClimateType::Cold => write!(f, "Cold"),
            crate::prelude::WorldClimateType::Chilly => write!(f, "Chilly"),
            crate::prelude::WorldClimateType::Cool => write!(f, "Cool"),
            crate::prelude::WorldClimateType::Ideal => write!(f, "Ideal"),
            crate::prelude::WorldClimateType::Warm => write!(f, "Warm"),
            crate::prelude::WorldClimateType::Tropical => write!(f, "Tropical"),
            crate::prelude::WorldClimateType::Hot => write!(f, "Hot"),
            crate::prelude::WorldClimateType::VeryHot => write!(f, "VeryHot"),
            crate::prelude::WorldClimateType::Infernal => write!(f, "Infernal"),
        }
    }
}
