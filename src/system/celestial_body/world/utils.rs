use crate::prelude::WorldTemperatureCategory;

pub fn get_category_from_temperature(blackbody_temperature: u32) -> WorldTemperatureCategory {
    let climate = {
        if blackbody_temperature < 244 {
            WorldTemperatureCategory::Frozen
        } else if blackbody_temperature < 255 {
            WorldTemperatureCategory::VeryCold
        } else if blackbody_temperature < 266 {
            WorldTemperatureCategory::Cold
        } else if blackbody_temperature < 278 {
            WorldTemperatureCategory::Chilly
        } else if blackbody_temperature < 289 {
            WorldTemperatureCategory::Cool
        } else if blackbody_temperature < 300 {
            WorldTemperatureCategory::Temperate
        } else if blackbody_temperature < 311 {
            WorldTemperatureCategory::Warm
        } else if blackbody_temperature < 322 {
            WorldTemperatureCategory::Hot
        } else if blackbody_temperature < 333 {
            WorldTemperatureCategory::VeryHot
        } else if blackbody_temperature < 344 {
            WorldTemperatureCategory::Scorching
        } else {
            WorldTemperatureCategory::Infernal
        }
    };
    climate
}
