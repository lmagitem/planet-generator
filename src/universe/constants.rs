use crate::prelude::*;

/// The stelliferous era we currently live in.
pub const OUR_UNIVERSES_ERA: StelliferousEra = StelliferousEra::MiddleStelliferous;
/// The current age of the universe.
pub const OUR_UNIVERSES_AGE: f32 = 13.8;
/// The time in billions of years when the Ancient Stelliferous Era starts.
pub const MIN_ANCIENT_STELLIFEROUS: f32 = 0.4;
/// The time in billions of years when the Early Stelliferous Era starts.
pub const MIN_EARLY_STELLIFEROUS: f32 = 0.5;
/// The time in billions of years when the Middle Stelliferous Era starts.
pub const MIN_MIDDLE_STELLIFEROUS: f32 = 5.0;
/// The time in billions of years when the Late Stelliferous Era starts.
pub const MIN_LATE_STELLIFEROUS: f32 = 50.0;
/// The time in billions of years when the End Stelliferous Era starts.
pub const MIN_END_STELLIFEROUS: f32 = 2000.0;
/// The time in billions of years when the End Stelliferous Era ends.
pub const MAX_END_STELLIFEROUS: f32 = 100000.0;
/// An array containing the data used to calculate a universe's age.
pub const POSSIBLE_ERAS: [PossibleEra; 5] = [
    PossibleEra {
        era: StelliferousEra::AncientStelliferous,
        min: MIN_ANCIENT_STELLIFEROUS,
        max: MIN_EARLY_STELLIFEROUS,
        weight: 1,
    },
    PossibleEra {
        era: StelliferousEra::EarlyStelliferous,
        min: MIN_EARLY_STELLIFEROUS,
        max: MIN_MIDDLE_STELLIFEROUS,
        weight: 40,
    },
    PossibleEra {
        era: StelliferousEra::MiddleStelliferous,
        min: MIN_MIDDLE_STELLIFEROUS,
        max: MIN_LATE_STELLIFEROUS,
        weight: 218,
    },
    PossibleEra {
        era: StelliferousEra::LateStelliferous,
        min: MIN_LATE_STELLIFEROUS,
        max: MIN_END_STELLIFEROUS,
        weight: 40,
    },
    PossibleEra {
        era: StelliferousEra::EndStelliferous,
        min: MIN_END_STELLIFEROUS,
        max: MAX_END_STELLIFEROUS,
        weight: 1,
    },
];

/// Data used to calculate a universe's age.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PossibleEra {
    /// The era this object represents.
    pub era: StelliferousEra,
    /// When the era begins.
    pub min: f32,
    /// When the era ends.
    pub max: f32,
    /// How often should we get this era as result during generation?
    pub weight: u32,
}
