#![warn(clippy::all, clippy::pedantic)]
#[path = "./galactic-neighborhood/galactic_neighborhood.rs"]
mod galactic_neighborhood;
#[path = "./galactic-neighborhood/types.rs"]
mod galactic_neighborhood_types;
#[path = "./galaxy/galaxy.rs"]
mod galaxy;
#[path = "./galaxy/types.rs"]
mod galaxy_types;
#[path = "./generator/generator.rs"]
mod generator;
#[path = "./generator/types.rs"]
mod generator_types;
#[path = "./planet/planet.rs"]
mod planet;
#[path = "./planet/types.rs"]
mod planet_types;
#[path = "./sector/sector.rs"]
mod sector;
#[path = "./sector/types.rs"]
mod sector_types;
#[path = "./system/system.rs"]
mod system;
#[path = "./system/types.rs"]
mod system_types;
#[path = "./universe/universe.rs"]
mod universe;
#[path = "./universe/types.rs"]
mod universe_types;

pub mod prelude {
    pub use crate::galactic_neighborhood::*;
    pub use crate::galactic_neighborhood_types::*;
    pub use crate::galaxy::*;
    pub use crate::galaxy_types::*;
    pub use crate::generator::*;
    pub use crate::generator_types::*;
    pub use crate::planet::*;
    pub use crate::planet_types::*;
    pub use crate::sector::*;
    pub use crate::sector_types::*;
    pub use crate::system::*;
    pub use crate::system_types::*;
    pub use crate::universe::*;
    pub use crate::universe_types::*;
    pub use log::*;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
}
