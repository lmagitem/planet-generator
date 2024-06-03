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
    /// Percentage of water on this world that is covered by some kind of ice.
    pub ice_over_water: f32,
    /// Percentage of land on this world that is in the open, as in not recovered by water or ice.
    pub land_area_percentage: f32,
    /// Percentage of land on this world that is covered by some kind of ice.
    pub ice_over_land: f32,
    /// An indication of the levels of volcanism in this world, from 0 to 100 (check `get_volcanism_level` to know what the numbers correspond to).
    pub volcanism: f32,
    /// An indication of the levels of tectonic activity in this world, from 0 to 100 (check `get_tectonics_level` to know what the numbers correspond to).
    pub tectonic_activity: f32,
    /// An indication of the levels of average relative humidity in this world.
    pub humidity: f32,
    /// A descriptive name for the world’s over-all surface temperature.
    pub temperature_category: WorldTemperatureCategory,
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
        ice_over_water: f32,
        land_area_percentage: f32,
        ice_over_land: f32,
        volcanism: f32,
        tectonic_activity: f32,
        humidity: f32,
        climate: WorldTemperatureCategory,
    ) -> Self {
        Self {
            body_type,
            world_type,
            special_traits,
            core_heat,
            magnetic_field,
            atmospheric_pressure,
            hydrosphere,
            ice_over_water,
            land_area_percentage,
            ice_over_land,
            volcanism,
            tectonic_activity,
            humidity,
            temperature_category: climate,
        }
    }

    /// Returns an enum value giving a human-comprehensible value to this planet's volcanism levels.
    pub fn get_volcanism_level(self) -> VolcanicActivity {
        if self.volcanism <= 0.01 {
            VolcanicActivity::None
        } else if self.volcanism <= 6.0 {
            VolcanicActivity::Light
        } else if self.volcanism <= 25.0 {
            VolcanicActivity::Moderate
        } else if self.volcanism <= 55.0 {
            VolcanicActivity::Heavy
        } else {
            VolcanicActivity::Extreme
        }
    }

    /// Returns an enum value giving a human-comprehensible value to this planet's tectonic activity levels.
    pub fn get_tectonics_level(self) -> TectonicActivity {
        if self.tectonic_activity <= 0.01 {
            TectonicActivity::None
        } else if self.tectonic_activity <= 16.0 {
            TectonicActivity::Light
        } else if self.tectonic_activity <= 35.0 {
            TectonicActivity::Moderate
        } else if self.tectonic_activity <= 55.0 {
            TectonicActivity::Heavy
        } else {
            TectonicActivity::Extreme
        }
    }
}
