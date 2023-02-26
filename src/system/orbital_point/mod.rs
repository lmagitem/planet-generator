use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct OrbitalPoint {
    /// The id of this orbital point.
    pub id: u32,
    /// The id of an eventual primary body the object placed at this point is orbiting around.
    pub primary_body: Option<u32>,
    /// The distance there is between the object placed at this point and its primary body. In Astronomical Units.
    pub distance_from_primary: Option<f64>,
    /// The list of ids of any satellites this point might have.
    pub satellites: Vec<u32>,
    /// The object placed at this point.
    pub object: AstronomicalObject,
}

impl OrbitalPoint {
    /// Creates a new [OrbitalPoint].
    pub fn new(
        id: u32,
        primary_body: Option<u32>,
        distance_from_primary: Option<f64>,
        satellites: Vec<u32>,
        object: AstronomicalObject,
    ) -> Self {
        Self {
            id,
            primary_body,
            distance_from_primary,
            satellites,
            object,
        }
    }
}
