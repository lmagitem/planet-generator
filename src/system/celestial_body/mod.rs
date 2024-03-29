use crate::internal::*;
use crate::prelude::*;
pub mod gaseous;
pub mod generator;
pub mod icy;
pub mod moon;
pub mod telluric;
pub mod traits;
pub mod types;
pub mod world;

#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct CelestialBody {
    /// Is this body a simple stub to be redesigned later?
    stub: bool,
    /// This body's name.
    #[default("default")]
    pub name: Rc<str>,
    /// The body's own orbit, along which it revolves.
    pub orbit: Option<Orbit>,
    /// The id of the orbital point this body inhabits.
    pub orbital_point_id: u32,
    /// This body's mass, in Earth's masses.
    pub mass: f64,
    /// This body's radius, in Earth's radii.
    pub radius: f64,
    /// This body's density, in g/cm³.
    pub density: f32,
    /// This body's surface gravity, in Gs.
    pub gravity: f32,
    /// This body's blackbody temperature, in Kelvins.
    pub blackbody_temperature: u32,
    /// The size class in which this body falls.
    pub size: CelestialBodySize,
    /// Specific body details.
    pub details: CelestialBodyDetails,
    /// A measure of the tidal friction caused on this body by the resonance of its orbit with its neighbors orbits.
    pub tidal_heating: u32,
}

impl CelestialBody {
    /// Creates a new [CelestialBody].
    pub fn new(
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        name: Rc<str>,
        mass: f64,
        radius: f64,
        density: f32,
        gravity: f32,
        blackbody_temperature: u32,
        tidal_heating: u32,
        size: CelestialBodySize,
        details: CelestialBodyDetails,
    ) -> Self {
        Self {
            stub: false,
            orbit,
            orbital_point_id,
            name,
            mass,
            radius,
            density,
            gravity,
            blackbody_temperature,
            tidal_heating,
            size,
            details,
        }
    }

    /// Indicates whether this body is a stub (a temporary body placed here before proper generation).
    pub fn is_stub(self) -> bool {
        self.stub
    }
}
