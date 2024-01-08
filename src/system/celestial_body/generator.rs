use crate::internal::*;
use crate::prelude::*;

pub(crate) fn get_size_constraint(size: CelestialBodySize, rng: &mut SeededDiceRoller) -> f32 {
    let (min, max) = match size {
        CelestialBodySize::Large => (0.065, 0.0915),
        CelestialBodySize::Standard => (0.030, 0.065),
        CelestialBodySize::Small => (0.024, 0.030),
        CelestialBodySize::Tiny => (0.004, 0.024),
        CelestialBodySize::Puny => (0.000003, 0.004),
        _ => panic!("No giant or bigger planet should determine its size using this method"),
    };
    rng.gen_range(min..max)
}

pub(crate) fn downsize_world_by(size: CelestialBodySize, number: u8) -> CelestialBodySize {
    let mut running_number = number;
    let mut new_size = size;
    loop {
        if size == CelestialBodySize::Hypergiant {
            new_size = CelestialBodySize::Supergiant;
        } else if size == CelestialBodySize::Supergiant {
            new_size = CelestialBodySize::Giant;
        } else if size == CelestialBodySize::Giant {
            new_size = CelestialBodySize::Large;
        } else if size == CelestialBodySize::Large {
            new_size = CelestialBodySize::Standard;
        } else if size == CelestialBodySize::Standard {
            new_size = CelestialBodySize::Small;
        } else if size == CelestialBodySize::Small {
            new_size = CelestialBodySize::Tiny;
        } else if size == CelestialBodySize::Tiny {
            new_size = CelestialBodySize::Puny;
        } else {
            new_size = CelestialBodySize::Puny;
        }

        if running_number < 1 {
            break;
        }
        running_number -= 1;
    }
    new_size
}

pub(crate) fn generate_acceptable_telluric_parameters(
    size_modifier: i32,
    mut rng: &mut SeededDiceRoller,
    mut min_density: f64,
    mut max_density: f64,
    size: CelestialBodySize,
    blackbody_temp: u32,
    planet_type: Rc<str>,
) -> (f32, CelestialBodySize, f64, f64) {
    let mut loop_number = 0;
    let mut density = 0.0;
    let mut size = size;
    let mut radius = 0.0;
    let mut mass = 0.0;
    loop {
        if min_density > max_density {
            let temp = max_density;
            max_density = min_density;
            min_density = temp;
        }
        density = (rng.roll(
            1,
            ((max_density * 1000.0) as u32 - (min_density * 1000.0) as u32) + 1,
            (min_density * 1000.0) as i32 - 1,
        ) as f32
            / 1000.0)
            .max(1.0);
        let size_constraint = get_size_constraint(size, &mut rng);
        radius = size_constraint as f64 * (blackbody_temp as f64 / (density as f64 / 5.513)).sqrt(); // in Earth radii
        mass = calculate_mass(density, radius);

        if mass < 10.0 {
            break;
        }

        loop_number += 1;
        if loop_number % 100 == 0 {
            size = downsize_world_by(size, 1);
        }
        if loop_number > 1000 {
            panic!("Infinite loop! Last turn was density: {}, size_constraint: {}, temp: {}, radius: {}, mass: {}, modifier: {}, for a {} {} planet.",
                   density, size_constraint, blackbody_temp, radius, mass, size_modifier, size, planet_type);
        }
    }
    (density, size, radius, mass)
}

fn calculate_mass(density: f32, radius: f64) -> f64 {
    // Earth's radius in centimeters
    let earth_radius_cm: f64 = 6.371e8;
    // Earth's mass in grams
    let earth_mass_g: f64 = 5.972e27;
    // Volume of the planet in cubic centimeters
    let volume_cm3: f64 =
        (4.0 / 3.0) * std::f64::consts::PI * (radius as f64 * earth_radius_cm).powi(3);
    // Mass of the planet in grams
    let mass_g: f64 = density as f64 * volume_cm3;
    // Convert mass from grams to Earth masses
    let mass_earth_masses: f64 = mass_g / earth_mass_g;

    mass_earth_masses
}

pub(crate) fn get_world_type(
    size: CelestialBodySize,
    body_type: CelestialBodyComposition,
    blackbody_temperature: u32,
    primary_star_mass: f64,
    rng: &mut SeededDiceRoller,
) -> CelestialBodyWorldType {
    match size {
        CelestialBodySize::Puny | CelestialBodySize::Tiny => {
            if blackbody_temperature <= 140 && body_type != CelestialBodyComposition::Icy {
                CelestialBodyWorldType::DirtySnowball
            } else if blackbody_temperature <= 140 {
                CelestialBodyWorldType::Ice
            } else {
                CelestialBodyWorldType::Rock
            }
        }
        CelestialBodySize::Small => {
            if blackbody_temperature <= 80 {
                CelestialBodyWorldType::Hadean
            } else if blackbody_temperature <= 140 && body_type != CelestialBodyComposition::Icy {
                CelestialBodyWorldType::DirtySnowball
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
            } else if blackbody_temperature <= 240 && body_type != CelestialBodyComposition::Icy {
                CelestialBodyWorldType::DirtySnowball
            } else if blackbody_temperature <= 240 {
                CelestialBodyWorldType::Ice
            } else if blackbody_temperature <= 320 && body_type != CelestialBodyComposition::Icy {
                CelestialBodyWorldType::Terrestrial
            } else if blackbody_temperature <= 320 {
                CelestialBodyWorldType::Ocean
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
            } else if blackbody_temperature <= 240 && body_type != CelestialBodyComposition::Icy {
                CelestialBodyWorldType::DirtySnowball
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
                size >= 0.065 && size < 0.092,
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
            let size = get_size_constraint(CelestialBodySize::Puny, &mut rng);
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
