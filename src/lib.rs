#![warn(clippy::all, clippy::pedantic)]
mod galactic_map;
mod galactic_neighborhood;
mod galaxy;
mod generator;
mod planet;
mod system;
mod universe;

pub mod prelude {
    pub use crate::galactic_map::division::*;
    pub use crate::galactic_map::hex::*;
    pub use crate::galactic_map::types::*;
    pub use crate::galactic_neighborhood::types::*;
    pub use crate::galactic_neighborhood::*;
    pub use crate::galaxy::types::*;
    pub use crate::galaxy::*;
    pub use crate::generator::types::*;
    pub use crate::generator::*;
    pub use crate::planet::types::*;
    pub use crate::planet::*;
    pub use crate::system::types::*;
    pub use crate::system::*;
    pub use crate::universe::types::*;
    pub use crate::universe::*;
    pub use log::*;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
}
