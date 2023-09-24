use crate::prelude::*;
pub mod gaseous;
pub mod generator;
pub mod icy;
pub mod telluric;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialRing {
    /// Is this ring a simple stub to be redesigned later?
    stub: bool,
    /// The ring's own orbit, along which it revolves.
    pub orbit: Option<Orbit>,
    /// The id of the orbital point this ring inhabits.
    pub orbital_point_id: u32,
    /// Specific ring details
    pub details: CelestialRingDetails,
}

impl CelestialRing {
    /// Creates a new [CelestialRing].
    pub fn new(orbit: Option<Orbit>, orbital_point_id: u32, details: CelestialRingDetails) -> Self {
        Self {
            stub: false,
            orbit,
            orbital_point_id,
            details,
        }
    }

    /// Indicates whether this ring is a stub (a temporary body placed here before proper generation).
    pub fn is_stub(self) -> bool {
        self.stub
    }
}
