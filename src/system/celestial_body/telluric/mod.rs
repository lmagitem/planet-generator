use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct TelluricBodyDetails {
    pub body_type: CelestialBodySubType,
    // pub atmospheric_density: AtmosphericDensityType,
    // pub hydrosphere: f32,
    // pub cryosphere: f32,
    // pub volcanism: f32,
    // pub tectonic_activity: f32,
    // pub land_area_percentage: f32,
    // pub humidity: f32,
    // pub average_temperature: f32,
    // pub climate: ColonizableClimateType,
    // pub landmasses: u32,
    // pub territories: Vec<Territory>,
    // pub atmospheric_composition: AtmosphericCompositionType,
    // pub special_features: Vec<Information>,
}

impl TelluricBodyDetails {
    /// Creates a new [TelluricBodyDetails].
    pub fn new(body_type: CelestialBodySubType) -> Self {
        Self { body_type }
    }
}
