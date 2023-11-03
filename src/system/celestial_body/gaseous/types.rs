use crate::internal::*;
use crate::prelude::*;

/// A list of settings used to configure the Gaseous Bodies (like gas giants) generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GaseousBodySettings {
    /// A list of specific [GasGiantSpecialTrait]s to use, if any.
    pub fixed_special_traits: Option<Vec<GasGiantSpecialTrait>>,
    /// A list of [GasGiantSpecialTrait]s forbidden to use in Gas Giant generation.
    pub forbidden_special_traits: Option<Vec<GasGiantSpecialTrait>>,
}

/// Peculiarities a Gas Giant might have.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum GasGiantSpecialTrait {
    /// This Gas Giant has the exact traits that one might expect for a member of its type and subtype.
    #[default]
    NoPeculiarity,
    /// This Gas Giant was the first to arise from its star proto-planetary disk.
    ProtoGiant,
}

impl Display for GasGiantSpecialTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GasGiantSpecialTrait::NoPeculiarity => write!(f, "No Peculiarity"),
            GasGiantSpecialTrait::ProtoGiant => write!(f, "Proto-Giant"),
        }
    }
}
