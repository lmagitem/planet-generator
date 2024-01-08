use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct TelluricBodyDetails {
    /// The main composition of this world.
    pub body_type: TelluricBodyComposition,
    /// The type of this world.
    pub world_type: CelestialBodyWorldType,
    /// What are the pecularities of this telluric body.
    pub special_traits: Vec<TelluricSpecialTrait>,
    /// The degree of heat this body's core still has.
    pub core_heat: CelestialBodyCoreHeat,
    /// The strength of this object's magnetic field.
    pub magnetic_field: MagneticFieldStrength,
    /// This body's atmospheric pressure, in atm, with 1 atm being equal to the average sea-level air pressure on Earth..
    pub atmospheric_pressure: f32,
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
    pub fn new(
        body_type: TelluricBodyComposition,
        world_type: CelestialBodyWorldType,
        special_traits: Vec<TelluricSpecialTrait>,
        core_heat: CelestialBodyCoreHeat,
        magnetic_field: MagneticFieldStrength,
        atmospheric_pressure: f32,
    ) -> Self {
        Self {
            body_type,
            world_type,
            special_traits,
            core_heat,
            magnetic_field,
            atmospheric_pressure,
        }
    }
}
