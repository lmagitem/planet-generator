use crate::internal::*;
use crate::prelude::*;

/// A list of settings used to configure the [StarSystem] generation.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct SystemSettings {
    /// Skip the system generation and just uses a copy of ours.
    pub use_ours: bool,
    /// Makes sure that only interesting systems are generated.
    pub only_interesting: bool,
}

/// The population of stars in this system.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub enum StellarEvolution {
    /// Population III: The first generation of stars, formed from primordial gas with virtually no
    /// metals, likely massive and short-lived. Given the near absence of metals, Population III
    /// stars couldn't probably host rocky planets, if planets they have, they would only be gas
    /// giants or unusual compositions not seen in today's universe. Any protoplanetary disks around
    /// these stars would be primarily composed of hydrogen and helium, with very few heavier elements.
    Paleodwarf,
    /// Population II: Older stars with low metallicity, typically found in globular clusters and
    /// the halo of galaxies. Their planetary systems, if they exist, might have fewer rocky planets
    /// and more gas giants. The terrestrial planets that do form might be smaller and less diverse
    /// in composition. Asteroid belts and Kuiper belt-like structures might be less dense and less
    /// varied in composition due to the scarcity of heavier elements.
    Subdwarf,
    /// Early Population I: Younger, metal-rich stars found in the spiral arms and disks of
    /// galaxies, associated with ongoing star formation. Sol is a Early Population I star.
    #[default]
    Dwarf,
    /// Late Population I: Stars that are metal-rich and found in galactic disks, but older than
    /// early Population I stars, representing a more mature phase of galactic evolution. Could have
    /// increased chances of finding life on habitable planets.
    Superdwarf,
    /// Population 0: Extremely old stars from a universe nearing its end, having witnessed multiple
    /// generations of stellar evolution and potentially having very high metallicity due to the
    /// cumulative effects of countless supernovae and stellar processes over time. They might host
    /// planets with exotic compositions, enriched with heavy elements.
    Hyperdwarf,
}

impl Display for StellarEvolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StellarEvolution::Paleodwarf => "Population III",
                StellarEvolution::Subdwarf => "Population II",
                StellarEvolution::Dwarf => "Early Population I",
                StellarEvolution::Superdwarf => "Late Population I",
                StellarEvolution::Hyperdwarf => "Population 0",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub enum SystemPeculiarity {
    /// Carbon-rich systems are ones where the protoplanetary disk was made with a higher
    /// carbon/oxygen ratio during formation, producing carbon planets instead of rocky ones, and
    /// where ice planets would be more likely to be made with ammonia, methane or carbon monoxide
    /// than water.
    CarbonRich,
    /// A very unusual and destructive event drastically affected the system in its past.
    Cataclysm(CataclysmSeverity),
    /// In this system is found an unusual mass level of ice, dust and planetesimals.
    UnusualDebrisDensity(DebrisDensity),
    /// This system is located within or in close proximity to a nebula, a vast region of
    /// interstellar gas and dust. Nebulae can be remnants of dead stars, birthplaces of new stars,
    /// or simply cold, dark clouds in space. The system might experience a diffuse glow from the
    /// illuminated gas and dust, creating a visually stunning backdrop. If the nebula is a region
    /// of active star formation, the system could be exposed to higher levels of radiation from
    /// nearby young, massive stars.
    Nebulae(NebulaeApparentSize),
    /// The system seems perfectly standard.
    #[default]
    NoPeculiarity,
}

impl Display for SystemPeculiarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemPeculiarity::CarbonRich => write!(f, "Carbon Rich"),
            SystemPeculiarity::Cataclysm(severity) => write!(f, "{} Cataclysm", severity),
            SystemPeculiarity::UnusualDebrisDensity(density) => {
                write!(f, "{} Debris Density", density)
            }
            SystemPeculiarity::Nebulae(size) => write!(f, "{} Nebula Visible", size),
            SystemPeculiarity::NoPeculiarity => write!(f, "No Peculiarity"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum CataclysmSeverity {
    Minor,
    #[default]
    Major,
    Extreme,
    Ultimate,
}

impl Display for CataclysmSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CataclysmSeverity::Minor => write!(f, "Minor"),
            CataclysmSeverity::Major => write!(f, "Major"),
            CataclysmSeverity::Extreme => write!(f, "Extreme"),
            CataclysmSeverity::Ultimate => write!(f, "Ultimate"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum DebrisDensity {
    MuchLower,
    #[default]
    Lower,
    Higher,
    MuchHigher,
}

impl Display for DebrisDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebrisDensity::MuchLower => write!(f, "Much Lower"),
            DebrisDensity::Lower => write!(f, "Lower"),
            DebrisDensity::Higher => write!(f, "Higher"),
            DebrisDensity::MuchHigher => write!(f, "Much Higher"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum NebulaeApparentSize {
    Tiny,
    #[default]
    Small,
    Large,
    Dominant,
}

impl Display for NebulaeApparentSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NebulaeApparentSize::Tiny => write!(f, "Tiny"),
            NebulaeApparentSize::Small => write!(f, "Small"),
            NebulaeApparentSize::Large => write!(f, "Large"),
            NebulaeApparentSize::Dominant => write!(f, "Dominant"),
        }
    }
}
