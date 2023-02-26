#![warn(clippy::all, clippy::pedantic)]
mod galaxy;
mod generator;
mod system;
mod universe;
mod utils;

pub mod prelude {
    pub use crate::galaxy::map::division::*;
    pub use crate::galaxy::map::division_level::*;
    pub use crate::galaxy::map::hex::types::*;
    pub use crate::galaxy::map::hex::*;
    pub use crate::galaxy::map::types::*;
    pub use crate::galaxy::neighborhood::types::*;
    pub use crate::galaxy::neighborhood::*;
    pub use crate::galaxy::types::*;
    pub use crate::galaxy::*;
    pub use crate::generator::types::*;
    pub use crate::generator::utils::*;
    pub use crate::generator::*;
    pub use crate::system::neighborhood::types::*;
    pub use crate::system::neighborhood::*;
    pub use crate::system::planet::types::*;
    pub use crate::system::planet::*;
    pub use crate::system::star::types::*;
    pub use crate::system::star::*;
    pub use crate::system::orbital_point::types::*;
    pub use crate::system::orbital_point::*;
    pub use crate::system::types::*;
    pub use crate::system::*;
    pub use crate::universe::types::*;
    pub use crate::universe::*;
    pub use crate::utils::*;
    pub use log::*;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
    pub use std::rc::Rc;
}
