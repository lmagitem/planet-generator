#![warn(clippy::all, clippy::pedantic)]
mod galaxy;
mod planet;
mod sector;
mod system;
mod universe;

pub mod prelude {
    pub use crate::galaxy::*;
    pub use crate::planet::*;
    pub use crate::sector::*;
    pub use crate::system::*;
    pub use crate::universe::*;
    pub use crate::GenerationSettings;
    pub use log::*;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
}
use prelude::*;

/// A list of settings used to configure generation.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GenerationSettings {
    /// A list of settings used to configure the [Universe] generation.
    pub universe: Option<UniverseSettings>,
}
