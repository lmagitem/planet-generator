use crate::system::celestial_body::types::CelestialBodySize;
use crate::system::orbital_point::types::Orbit;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

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

/// Data of the body whose moons are being generated. Plain owned struct
/// (not a context): values are produced mid-call by the parent body generator.
#[derive(Clone, PartialEq, Debug)]
pub struct ParentBodyInfo {
    pub parent_orbit: Orbit,
    pub planet_id: u32,
    pub planet_size: CelestialBodySize,
    pub planet_mass: f64,
    pub planet_density: f32,
    pub planet_radius: f64,
    pub planet_orbital_period: f32,
    pub blackbody_temperature: u32,
}
