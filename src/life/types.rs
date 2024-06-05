use crate::internal::*;
use std::fmt;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum LifeLevel {
    #[default]
    None,
    UniCellular,
    PluriCellular,
    PlantLike,
    AnimalLike,
    Sentient,
}

impl Display for LifeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LifeLevel::None => "None",
                LifeLevel::UniCellular => "UniCellular",
                LifeLevel::PluriCellular => "PluriCellular",
                LifeLevel::PlantLike => "PlantLike",
                LifeLevel::AnimalLike => "AnimalLike",
                LifeLevel::Sentient => "Sentient",
            }
        )
    }
}

impl LifeLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            LifeLevel::None => 0,
            LifeLevel::UniCellular => 1,
            LifeLevel::PluriCellular => 2,
            LifeLevel::PlantLike => 3,
            LifeLevel::AnimalLike => 4,
            LifeLevel::Sentient => 5,
        }
    }
}
