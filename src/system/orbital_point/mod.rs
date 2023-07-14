use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct OrbitalPoint {
    /// The id of this orbital point.
    pub id: u32,
    /// This point's own orbit, around which it revolves.
    pub own_orbit: Option<Orbit>,
    /// The object placed at this point.
    pub object: AstronomicalObject,
    /// The orbits of this point.
    pub orbits: Vec<Orbit>,
}

impl OrbitalPoint {
    /// Creates a new [OrbitalPoint].
    pub fn new(
        id: u32,
        own_orbit: Option<Orbit>,
        object: AstronomicalObject,
        orbits: Vec<Orbit>,
    ) -> Self {
        Self {
            id,
            own_orbit,
            object,
            orbits,
        }
    }
}
