use crate::internal::*;
use crate::prelude::*;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum MoonDistance {
    Any,
    Ring,
    BeforeMajor,
    #[default]
    Close,
    MajorPlanetClose,
    MajorGiantClose,
    Medium,
    MediumOrFar,
    Far,
}
