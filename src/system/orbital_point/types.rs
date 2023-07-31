use crate::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Orbit {
    /// The id of the primary body at the center of this orbit.
    pub primary_body_id: u32,
    /// The ids of the objects in this orbit.
    pub satellite_ids: Vec<u32>,
    /// The average distance from the primary body to the objects in this orbit.
    pub average_distance: f64,
    /// The average distance from the barycentre of the solar system to the objects in this orbit.
    pub average_distance_from_system_center: f64,
    /// The eccentricity of this orbit.
    pub eccentricity: f32,
}

impl Orbit {
    /// Creates a new [Orbit].
    pub fn new(
        primary_body_id: u32,
        satellite_ids: Vec<u32>,
        average_distance: f64,
        average_distance_from_system_center: f64,
        eccentricity: f32,
    ) -> Self {
        Self {
            primary_body_id,
            satellite_ids,
            average_distance,
            average_distance_from_system_center,
            eccentricity,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub enum AstronomicalObject {
    #[default]
    Void,
    Star(Star),
    TelluricPlanet,
    GasGiant,
    Ring,
    Station,
    Ship,
}
