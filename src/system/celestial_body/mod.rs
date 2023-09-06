use crate::prelude::*;
pub mod gaseous;
pub mod generator;
pub mod icy;
pub mod telluric;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CelestialBody {
    /// The body's own orbit, along which it revolves.
    pub orbit: Option<Orbit>,
    /// The id of the orbital point this body inhabits.
    pub orbital_point_id: u32,

    // General Properties
    // pub id: u32,
    // pub name: String,
    // pub distance_from_orbited: f64,
    // pub exploitable_resources: Vec<ExploitableResource>,
    // pub diameter: f64,
    // pub gravity: f64,
    // pub axial_tilt: f32,
    // pub hours_in_a_day: f32,
    // pub terran_years_in_a_year: f32,

    // Specific body details
    pub details: CelestialBodyDetails,
}

impl CelestialBody {
    /// Creates a new [CelestialBody].
    pub fn new(orbit: Option<Orbit>, orbital_point_id: u32, details: CelestialBodyDetails) -> Self {
        Self {
            orbit,
            orbital_point_id,
            details,
        }
    }
}
