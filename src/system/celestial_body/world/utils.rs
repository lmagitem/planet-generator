use crate::prelude::WorldClimateType;

pub fn get_climate_from_temperature(blackbody_temperature: u32) -> WorldClimateType {
    let climate = {
        if blackbody_temperature < 244 {
            WorldClimateType::Frozen
        } else if blackbody_temperature < 255 {
            WorldClimateType::VeryCold
        } else if blackbody_temperature < 266 {
            WorldClimateType::Cold
        } else if blackbody_temperature < 278 {
            WorldClimateType::Chilly
        } else if blackbody_temperature < 289 {
            WorldClimateType::Cool
        } else if blackbody_temperature < 300 {
            WorldClimateType::Ideal
        } else if blackbody_temperature < 311 {
            WorldClimateType::Warm
        } else if blackbody_temperature < 322 {
            WorldClimateType::Tropical
        } else if blackbody_temperature < 333 {
            WorldClimateType::Hot
        } else if blackbody_temperature < 344 {
            WorldClimateType::VeryHot
        } else {
            WorldClimateType::Infernal
        }
    };
    climate
}
