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
    /// The orbits that revolve around this point.
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

    /// Returns this orbital point's own orbit.
    pub fn get_own_orbit(&self) -> Option<Orbit> {
        self.own_orbit.clone()
    }

    /// Replaces this orbital point's own orbit, and the reference the object found at the orbital
    /// point might have to that orbit.
    pub fn set_own_orbit(&mut self, orbit: Orbit) {
        self.own_orbit = Some(orbit.clone());
        match &mut self.object {
            AstronomicalObject::Void => {}
            AstronomicalObject::Star(ref mut star) => star.orbit = Some(orbit),
            AstronomicalObject::TelluricBody(ref mut body) => body.orbit = Some(orbit),
            AstronomicalObject::GaseousBody(ref mut body) => body.orbit = Some(orbit),
            AstronomicalObject::IcyBody(ref mut body) => body.orbit = Some(orbit),
            AstronomicalObject::Ring => {}
            AstronomicalObject::Spacecraft => {}
        }
    }

    /// Updates the reference the object found at the orbital point might have to its own orbit.
    pub fn update_object_own_orbit(&mut self) {
        let orbit = self.get_own_orbit();
        match &mut self.object {
            AstronomicalObject::Void => {}
            AstronomicalObject::Star(star) => {
                star.orbit = orbit;
                star.orbital_point_id = self.id;
            }
            AstronomicalObject::TelluricBody(body) => {
                body.orbit = orbit;
                body.orbital_point_id = self.id;
            }
            AstronomicalObject::GaseousBody(body) => {
                body.orbit = orbit;
                body.orbital_point_id = self.id;
            }
            AstronomicalObject::IcyBody(body) => {
                body.orbit = orbit;
                body.orbital_point_id = self.id;
            }
            AstronomicalObject::Ring => {}
            AstronomicalObject::Spacecraft => {}
        }
    }
}
