use crate::prelude::*;

/// A list of settings used to configure the [Universe] generation.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct UniverseSettings {
    /// The specific universe [StelliferousEra] to use if any. Will be overwritten if the age or **use_ours** is set.
    pub fixed_era: Option<StelliferousEra>,
    /// Asks to generate a universe's [StelliferousEra] randomly, but only using eras that are older than the given one. Will be overwritten
    /// if the age or **use_ours** is set.
    pub era_before: Option<StelliferousEra>,
    /// Asks to generate a universe's [StelliferousEra] randomly, but only using eras that are younger than the given one. Will be
    /// overwritten if the age or **use_ours** is set.
    pub era_after: Option<StelliferousEra>,
    /// The specific universe age to use if any, in billions of years. **Must be higher or equal to 0.4 and lower than 100000.**
    /// Will overwrite the era if set, and be overwritten if **use_ours** is set.
    pub fixed_age: Option<f32>,
    /// Asks to generate a universe's age randomly, but with an age at least older than the given one. **Must be higher or equal to 0.4
    /// and lower than 100000.** Will overwrite the era if set, and be overwritten if **use_ours** is set.
    pub age_before: Option<f32>,
    /// Asks to generate a universe's age randomly, but with an age at least younger than the given one. **Must be higher or equal to 0.4
    /// and lower than 100000.** Will overwrite the era if set, and be overwritten if **use_ours** is set.
    pub age_after: Option<f32>,
    /// Skip the universe generation and just uses a copy of ours. Will overwrite **fixed_era** and **fixed_age** if set.
    pub use_ours: bool,
}

impl Display for UniverseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ fixed_era: {}, era_before: {}, era_after: {}, fixed_age: {}, age_before: {}, age_after: {}, use_ours: {} }}",
        if self.fixed_era.is_some() { format!("{}", self.fixed_era.unwrap()) } else { String::from("None") },
        if self.era_before.is_some() { format!("{}", self.era_before.unwrap()) } else { String::from("None") },
        if self.era_after.is_some() { format!("{}", self.era_after.unwrap()) } else { String::from("None") },
        if self.fixed_age.is_some() { format!("{}", self.fixed_age.unwrap()) } else { String::from("None") },
        if self.age_before.is_some() { format!("{}", self.age_before.unwrap()) } else { String::from("None") },
        if self.age_after.is_some() { format!("{}", self.age_after.unwrap()) } else { String::from("None") },
        self.use_ours)
    }
}

/// The Stelliferous Era is the span of time after the Big Bang and the Primordial Era in which matter is arranged in the form of stars,
/// galaxies, and galaxy clusters, and most energy is produced in stars. Stars are the most dominant objects of the universe in this era.
/// Massive stars use up their fuel very rapidly, in as little as a few million years. Eventually, the only luminous stars remaining will
/// be white dwarf stars. By the end of this era, bright stars as we know them will be gone, their nuclear fuel exhausted, and only white
/// dwarfs, brown dwarfs, neutron stars and black holes will remain.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum StelliferousEra {
    /// A period between the Primordial and Stelliferous eras. With the birth and death of the first Population III stars, the first
    /// mini-galaxies and Population II stars begin to appear alongside elements heavier than helium. Galaxies allow small bubbles of
    /// ionized gas to exist and expand, but most of the universe is still made of an extremely dense and opaque neutral hydrogen cloud
    /// which would make impossible to see outside of one's galaxy.
    AncientStelliferous,
    /// The majority of the universe reionizes more or less 1 billion years after the Big Bang, light can now travel freely across the
    /// whole universe. The mini-galaxies created previously begin to merge and form mature galaxies, which may interact with each others
    /// by colliding, being engulfed, etc. The first Population I stars appear, which contain heavier elements and are more likely to be
    /// accompanied by planets.
    EarlyStelliferous,
    /// The part of the Stelliferous era we live in. There is a wide range of galaxies consisting of a wide range of stars.
    #[default]
    MiddleStelliferous,
    /// Local galactic groups will have merged into single giant galaxies. All galaxies outside of one's own cluster will disappear below
    /// the cosmological horizon - the only things that a member of a galaxy will be able to see are the stars and objects within its own
    /// galaxy. As the Late Stelliferous progresses, the general luminosity of galaxies will also diminish as the less massive red dwarfs
    /// begin to die as white dwarfs.
    LateStelliferous,
    /// All other galaxies outside one's own will no longer be detectable by any means. As this era progresses, stars will exhaust their
    /// fuel and cool off. All stars not massive enough to become a neutron star or a black hole will turn into white dwarfs and slowly cool
    /// until they're black dwarfs. By the end of the End Stelliferous, all stars will have burned out and star formation will end.
    EndStelliferous,
}

impl Display for StelliferousEra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StelliferousEra::AncientStelliferous => write!(f, "{}", "Ancient Stelliferous"),
            StelliferousEra::EarlyStelliferous => write!(f, "{}", "Early Stelliferous"),
            StelliferousEra::MiddleStelliferous => write!(f, "{}", "Middle Stelliferous"),
            StelliferousEra::LateStelliferous => write!(f, "{}", "Late Stelliferous"),
            StelliferousEra::EndStelliferous => write!(f, "{}", "End Stelliferous"),
        }
    }
}
