use crate::internal::*;
use crate::prelude::*;

/// How is placed the first gas giant of a system.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum GasGiantArrangement {
    NoGasGiant,
    #[default]
    ConventionalGasGiant,
    EccentricGasGiant,
    EpistellarGasGiant,
}

impl Display for GasGiantArrangement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GasGiantArrangement::NoGasGiant => write!(f, "No Gas Giant"),
            GasGiantArrangement::ConventionalGasGiant => write!(f, "Conventional Gas Giant"),
            GasGiantArrangement::EccentricGasGiant => write!(f, "Eccentric Gas Giant"),
            GasGiantArrangement::EpistellarGasGiant => write!(f, "Epistellar Gas Giant"),
        }
    }
}
