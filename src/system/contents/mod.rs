use crate::prelude::*;
pub mod generator;
mod types;
pub mod zones;

/// Returns the next unused identifier that can be assigned to an [OrbitalPoint].
fn get_next_id(existing_objects: &Vec<OrbitalPoint>) -> u32 {
    existing_objects.iter().map(|o| o.id).max().unwrap_or(0) + 1
}
