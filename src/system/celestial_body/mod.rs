use crate::internal::*;
use crate::prelude::*;
pub mod gaseous;
pub mod generator;
pub mod icy;
pub mod telluric;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialBody {
    /// Is this body a simple stub to be redesigned later?
    stub: bool,
    /// The body's own orbit, along which it revolves.
    pub orbit: Option<Orbit>,
    /// The id of the orbital point this body inhabits.
    pub orbital_point_id: u32,
    /// This body's mass, in Earth's masses.
    pub mass: f32,
    /// This body's radius, in Earth's radii.
    pub radii: f32,
    /// This body's density, in g/cm³.
    pub density: f32,

    // General Properties
    // pub id: u32,
    // pub name: Rc<str>,
    // pub distance_from_orbited: f64,
    // pub exploitable_resources: Vec<ExploitableResource>,
    // pub diameter: f64,
    // pub gravity: f64,
    // pub axial_tilt: f32,
    // pub hours_in_a_day: f32,
    // pub terran_years_in_a_year: f32,
    /// The size class in which this body falls.
    pub size: CelestialBodySize,
    /// Specific body details.
    pub details: CelestialBodyDetails,
}

impl CelestialBody {
    /// Creates a new [CelestialBody].
    pub fn new(
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        mass: f32,
        radii: f32,
        density: f32,
        size: CelestialBodySize,
        details: CelestialBodyDetails,
    ) -> Self {
        Self {
            stub: false,
            orbit,
            orbital_point_id,
            mass,
            radii,
            density,
            size,
            details,
        }
    }

    /// Indicates whether this body is a stub (a temporary body placed here before proper generation).
    pub fn is_stub(self) -> bool {
        self.stub
    }
}
