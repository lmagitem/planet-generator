use crate::prelude::*;
pub mod generator;
pub mod neighborhood;
pub mod planet;
pub mod star;
pub mod types;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub struct StarSystem {}
