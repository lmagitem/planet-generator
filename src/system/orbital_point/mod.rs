use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct OrbitalPoint {
    /// The id of this orbital point.
    pub id: u32,
    /// The id of an eventual primary body the object placed at this point is orbiting around.
    pub primary_body_id: Option<u32>,
    /// The distance there is between the object placed at this point and its primary body. In Astronomical Units.
    pub distance_from_primary: Option<f64>,
    /// The eccentricity of this orbital point's orbit around its primary body.
    pub orbital_eccentricity: Option<f32>,
    /// The list of ids of any satellites this point might have.
    pub satellite_ids: Vec<u32>,
    /// The object placed at this point.
    pub object: AstronomicalObject,
}

impl OrbitalPoint {
    /// Creates a new [OrbitalPoint].
    pub fn new(
        id: u32,
        primary_body_id: Option<u32>,
        distance_from_primary: Option<f64>,
        orbital_eccentricity: Option<f32>,
        satellite_ids: Vec<u32>,
        object: AstronomicalObject,
    ) -> Self {
        Self {
            id,
            primary_body_id,
            distance_from_primary,
            orbital_eccentricity,
            satellite_ids,
            object,
        }
    }
}
