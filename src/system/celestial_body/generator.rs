use crate::prelude::{CelestialBodySize, CelestialBodyWorldType};
use seeded_dice_roller::SeededDiceRoller;

pub(crate) fn get_size_constraint(size: CelestialBodySize, rng: &mut SeededDiceRoller) -> f32 {
    let (min, max) = match size {
        CelestialBodySize::Large => (0.065, 0.0915),
        CelestialBodySize::Standard => (0.030, 0.065),
        CelestialBodySize::Small => (0.024, 0.030),
        CelestialBodySize::Tiny => (0.004, 0.024),
        CelestialBodySize::Moonlet => (0.000003, 0.004),
        _ => panic!("No giant or bigger planet should determine its size using this method"),
    };
    rng.gen_range(min..max)
}

pub(crate) fn get_world_type(
    size: CelestialBodySize,
    blackbody_temperature: u32,
    primary_star_mass: f32,
    rng: &mut SeededDiceRoller,
) -> CelestialBodyWorldType {
    match size {
        CelestialBodySize::Moonlet | CelestialBodySize::Tiny => {
            if blackbody_temperature <= 140 {
                CelestialBodyWorldType::Ice
            } else {
                CelestialBodyWorldType::Rock
            }
        }
        CelestialBodySize::Small => {
            if blackbody_temperature <= 80 {
                CelestialBodyWorldType::Hadean
            } else if blackbody_temperature <= 140 {
                CelestialBodyWorldType::Ice
            } else {
                CelestialBodyWorldType::Rock
            }
        }
        CelestialBodySize::Standard => {
            if blackbody_temperature <= 80 {
                CelestialBodyWorldType::Hadean
            } else if blackbody_temperature > 151
                && blackbody_temperature <= 230
                && primary_star_mass < 0.65
            {
                CelestialBodyWorldType::Ammonia
            } else if blackbody_temperature <= 240 {
                CelestialBodyWorldType::Ice
            } else if blackbody_temperature <= 320 {
                CelestialBodyWorldType::Terrestrial
            } else if blackbody_temperature <= 500 {
                CelestialBodyWorldType::Greenhouse
            } else {
                CelestialBodyWorldType::Chthonian
            }
        }
        CelestialBodySize::Large => {
            if blackbody_temperature > 151
                && blackbody_temperature <= 230
                && primary_star_mass < 0.65
            {
                CelestialBodyWorldType::Ammonia
            } else if blackbody_temperature <= 240 {
                CelestialBodyWorldType::Ice
            } else if blackbody_temperature <= 320 {
                CelestialBodyWorldType::Terrestrial
            } else if blackbody_temperature <= 500 {
                CelestialBodyWorldType::Greenhouse
            } else {
                CelestialBodyWorldType::Chthonian
            }
        }
        _ => CelestialBodyWorldType::VolatilesGiant,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celestial_body_generator_get_size_constraint_large() {
        let mut rng = SeededDiceRoller::new("seed_for_large", "test");
        for _ in 0..100 {
            let size = get_size_constraint(CelestialBodySize::Large, &mut rng);
            assert!(
                size >= 0.065 && size < 0.091,
                "Size was not within Large constraints: {}",
                size
            );
        }
    }

    #[test]
    fn celestial_body_generator_get_size_constraint_standard() {
        let mut rng = SeededDiceRoller::new("seed_for_standard", "test");
        for _ in 0..100 {
            let size = get_size_constraint(CelestialBodySize::Standard, &mut rng);
            assert!(
                size >= 0.030 && size < 0.065,
                "Size was not within Standard constraints: {}",
                size
            );
        }
    }

    #[test]
    fn celestial_body_generator_get_size_constraint_small() {
        let mut rng = SeededDiceRoller::new("seed_for_small", "test");
        for _ in 0..100 {
            let size = get_size_constraint(CelestialBodySize::Small, &mut rng);
            assert!(
                size >= 0.024 && size < 0.030,
                "Size was not within Small constraints: {}",
                size
            );
        }
    }

    #[test]
    fn celestial_body_generator_get_size_constraint_tiny() {
        let mut rng = SeededDiceRoller::new("seed_for_tiny", "test");
        for _ in 0..100 {
            let size = get_size_constraint(CelestialBodySize::Tiny, &mut rng);
            assert!(
                size >= 0.004 && size < 0.024,
                "Size was not within Tiny constraints: {}",
                size
            );
        }
    }

    #[test]
    fn celestial_body_generator_get_size_constraint_moonlet() {
        let mut rng = SeededDiceRoller::new("seed_for_moonlet", "test");
        for _ in 0..100 {
            let size = get_size_constraint(CelestialBodySize::Moonlet, &mut rng);
            assert!(
                size >= 0.000003 && size < 0.004,
                "Size was not within Moonlet constraints: {}",
                size
            );
        }
    }

    #[test]
    #[should_panic(
        expected = "No giant or bigger planet should determine its size using this method"
    )]
    fn celestial_body_generator_get_size_constraint_invalid() {
        let mut rng = SeededDiceRoller::new("seed_for_invalid", "test");
        get_size_constraint(CelestialBodySize::Giant, &mut rng);
    }
}
