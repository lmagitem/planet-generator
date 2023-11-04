use crate::internal::*;
use crate::prelude::*;
pub mod belt;
pub mod generator;
pub mod ring;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialDisk {
    /// Is this disk a simple stub to be redesigned later?
    stub: bool,
    /// This disk's name.
    pub name: Rc<str>,
    /// The disk's own orbit, along which it revolves.
    pub orbit: Option<Orbit>,
    /// The id of the orbital point this disk inhabits.
    pub orbital_point_id: u32,
    /// Specific disk details
    pub details: CelestialDiskType,
}

impl CelestialDisk {
    /// Creates a new [CelestialDisk].
    pub fn new(orbit: Option<Orbit>, orbital_point_id: u32, name: Rc<str>, details: CelestialDiskType) -> Self {
        Self {
            stub: false,
            orbit,
            orbital_point_id,
            name,
            details,
        }
    }

    /// Indicates whether this disk is a stub (a temporary body placed here before proper generation).
    pub fn is_stub(self) -> bool {
        self.stub
    }
}
