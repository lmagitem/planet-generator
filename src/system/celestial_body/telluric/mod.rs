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
    pub special_traits: Vec<CelestialBodySpecialTrait>,
    /// The degree of heat this body's core still has.
    pub core_heat: CelestialBodyCoreHeat,
    /// The strength of this object's magnetic field.
    pub magnetic_field: MagneticFieldStrength,
    /// This body's atmospheric pressure, in atm, with 1 atm being equal to the average sea-level air pressure on Earth.
    pub atmospheric_pressure: f32,
    // pub atmospheric_density: AtmosphericDensityType,
    /// Percentage of this world that is covered by some kind of liquid.
    pub hydrosphere: f32,
    // pub cryosphere: f32,
    ///
    pub volcanism: f32,
    ///
    pub tectonic_activity: f32,
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
        special_traits: Vec<CelestialBodySpecialTrait>,
        core_heat: CelestialBodyCoreHeat,
        magnetic_field: MagneticFieldStrength,
        atmospheric_pressure: f32,
        hydrosphere: f32,
        volcanism: f32,
        tectonic_activity: f32,
    ) -> Self {
        Self {
            body_type,
            world_type,
            special_traits,
            core_heat,
            magnetic_field,
            atmospheric_pressure,
            hydrosphere,
            volcanism,
            tectonic_activity,
        }
    }

    ///
    pub fn get_volcanism_level(self) -> VolcanicActivity {
        if self.volcanism <= 0.01 {
            VolcanicActivity::None
        } else if self.volcanism <= 4.0 {
            VolcanicActivity::Light
        } else if self.volcanism <= 19.0 {
            VolcanicActivity::Moderate
        } else if self.volcanism <= 54.0 {
            VolcanicActivity::Heavy
        } else {
            VolcanicActivity::Extreme
        }
    }

    ///
    pub fn get_tectonics_level(self) -> TectonicActivity {
        if self.tectonic_activity <= 0.01 {
            TectonicActivity::None
        } else if self.tectonic_activity <= 16.0 {
            TectonicActivity::Light
        } else if self.tectonic_activity <= 32.0 {
            TectonicActivity::Moderate
        } else if self.tectonic_activity <= 48.0 {
            TectonicActivity::Heavy
        } else {
            TectonicActivity::Extreme
        }
    }
}
