use crate::prelude::*;
#[path = "./constants.rs"]
mod constants;
use constants::*;

impl Star {
    /// Generates a new star.
    pub fn generate(
        star_index: u16,
        system_index: u16,
        coord: SpaceCoordinates,
        evolution: StellarEvolution,
        system: &StarSystem,
        hex: &GalacticHex,
        galaxy: &Galaxy,
    ) -> Self {
        let mut mass = generate_mass(star_index, system_index, coord, galaxy);
        let mut luminosity = calculate_main_sequence_luminosity(mass);
        let main_lifespan = calculate_lifespan(mass, luminosity);
        let subgiant_lifespan = main_lifespan * 0.15;
        let giant_lifespan = main_lifespan * 0.0917;
        let age = generate_age(
            star_index,
            system_index,
            coord,
            hex,
            &galaxy,
            &galaxy.neighborhood.universe,
        );
        mass = simulate_mass_loss_over_the_years(mass, age);

        // Main sequence values
        let mut radius = calculate_radius(
            mass,
            0.0,
            main_lifespan,
            subgiant_lifespan,
            giant_lifespan,
            star_index,
            system_index,
            coord,
            galaxy,
        );
        let mut temperature = calculate_temperature_using_luminosity(luminosity, radius) as u32;
        let mut spectral_type = calculate_spectral_type(temperature);

        // Actual values
        radius = calculate_radius(
            mass,
            age,
            main_lifespan,
            subgiant_lifespan,
            giant_lifespan,
            star_index,
            system_index,
            coord,
            galaxy,
        );
        temperature = calculate_temperature_using_luminosity(luminosity, radius) as u32;
        spectral_type = calculate_spectral_type(temperature);
        let luminosity_class = StarLuminosityClass::Ia;

        Self {
            name: "Sun".to_string(),
            mass,
            luminosity,
            radius,
            age: age / 1000.0,
            temperature: temperature,
            spectral_type,
            luminosity_class,
        }
    }
}

/// Reduces mass towards 150 solar masses if higher, as a star that is bigger than that blows off its mass as solar wind until it gets to 150.
fn simulate_mass_loss_over_the_years(mass: f32, age: f32) -> f32 {
    if mass > 150.0 {
        150.0_f32.max(mass - age)
    } else {
        mass
    }
}

fn generate_mass(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> f32 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.seed,
        &format!("star_{}_{}_{}_mass", coord, system_index, star_index),
    );
    let range = rng
        .get_result(&CopyableRollToProcess::new(
            vec![
                // Brown dwarf
                CopyableWeightedResult {
                    result: (0.015, 0.039),
                    weight: 920,
                },
                // Red dwarf Pop 0
                CopyableWeightedResult {
                    result: (0.04, 0.07),
                    weight: 1078,
                },
                // Red dwarf Pop I
                CopyableWeightedResult {
                    result: (0.071, 0.124),
                    weight: 1013,
                },
                // Red dwarf Pop II
                CopyableWeightedResult {
                    result: (0.125, 0.25),
                    weight: 2252,
                },
                CopyableWeightedResult {
                    result: (0.251, 0.399),
                    weight: 1344,
                },
                // Red dwarf Pop III
                CopyableWeightedResult {
                    result: (0.4, 0.5),
                    weight: 896,
                },
                // Orange
                CopyableWeightedResult {
                    result: (0.501, 1.0),
                    weight: 1520,
                },
                // Yellow
                CopyableWeightedResult {
                    result: (1.001, 2.0),
                    weight: 640,
                },
                // White
                CopyableWeightedResult {
                    result: (2.001, 4.0),
                    weight: 240,
                },
                // Giants
                CopyableWeightedResult {
                    result: (4.001, 8.0),
                    weight: 64,
                },
                // Blue giants
                CopyableWeightedResult {
                    result: (8.001, 20.0),
                    weight: 24,
                },
                CopyableWeightedResult {
                    result: (20.001, 50.0),
                    weight: 2,
                },
                // Pop I
                CopyableWeightedResult {
                    result: (50.001, 100.0),
                    weight: 1,
                },
                // Pop II
                CopyableWeightedResult {
                    result: (100.001, 200.0),
                    weight: 1,
                },
                // Pop III
                CopyableWeightedResult {
                    result: (200.001, 500.0),
                    weight: 1,
                },
            ],
            RollMethod::SimpleRoll,
        ))
        .expect("Should return a range to generate a star's mass.");
    rng.gen_f32() % (range.1 - range.0) + range.0
}

fn calculate_spectral_type(temperature: u32) -> StarSpectralType {
    println!("calculate_spectral_type");

    // Find the two temperatures in the dataset that the given temperature is between
    let (lower_temp, lower_class) = dataset.iter().find(|&(t, _)| *t <= temperature).unwrap();
    let (upper_temp, upper_class) = dataset
        .iter()
        .rev()
        .find(|&(t, _)| *t > temperature)
        .unwrap();

    // Interpolate the class value between the two nearest temperatures
    let class_as_int: u32 = (*lower_class as f32
        + (temperature as f32 - *lower_temp as f32) * (*upper_class as f32 - *lower_class as f32)
            / (*upper_temp as f32 - *lower_temp as f32)) as u32;

    // Convert the class value to the spectral type
    let spectral_type = match class_as_int / 10 {
        0 => StarSpectralType::WR((class_as_int % 10) as u8),
        1 => StarSpectralType::O((class_as_int % 10) as u8),
        2 => StarSpectralType::B((class_as_int % 10) as u8),
        3 => StarSpectralType::A((class_as_int % 10) as u8),
        4 => StarSpectralType::F((class_as_int % 10) as u8),
        5 => StarSpectralType::G((class_as_int % 10) as u8),
        6 => StarSpectralType::K((class_as_int % 10) as u8),
        7 => StarSpectralType::M((class_as_int % 10) as u8),
        8 => StarSpectralType::L((class_as_int % 10) as u8),
        9 => StarSpectralType::T((class_as_int % 10) as u8),
        _ => StarSpectralType::Y((class_as_int % 10) as u8),
    };

    spectral_type
}

fn calculate_temperature_using_luminosity(luminosity: f32, radius: f32) -> f32 {
    let pi = std::f64::consts::PI;
    let area = 4.0 * pi * (radius as f64).powf(2.0);
    // If I understood properly, the real approximation of the constant is 5.670367x10^-8, I have no idea why but I need to put 10^-17
    // in order to get working results.
    let sigma = 5.670367 * f64::powf(10.0, -17.0);
    // Same there, I've added the "x0.94304315" because it gives me results that are, on average (and exactly in the case of Sun), closer
    // to the ones I found on existing stars.
    ((luminosity as f64 / (area * sigma)).powf(1.0 / 4.0) * 0.94304315) as f32
}

fn calculate_luminosity_using_temperature(temperature: u32, radius: f32) -> f32 {
    let pi = std::f64::consts::PI;
    let area = 4.0 * pi * (radius as f64).powf(2.0);
    // If I understood properly, the real approximation of the constant is 5.670367x10^-8, I have no idea why but I need to put 10^-17
    // in order to get working results.
    let sigma = 5.670367 * f64::powf(10.0, -17.0);
    // Same there, I've added the "x1.2643679" because it gives me results that are, on average (and exactly in the case of Sun), closer
    // to the ones I found on existing stars.
    ((sigma * area * (temperature as f64).powf(4.0)) * 1.2643679) as f32
}

fn calculate_radius(
    mass: f32,
    age: f32,
    main_lifespan: f32,
    subgiant_lifespan: f32,
    giant_lifespan: f32,
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> f32 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.seed,
        &format!("star_{}_{}_{}_radius", coord, system_index, star_index),
    );
    let mut radius = mass.powf(0.8);
    let rand_multiplier = rng.roll(1, 4666, 999) as f32 / 10000.0;
    if age < main_lifespan + subgiant_lifespan {
        // Subgiant
        radius = radius * rand_multiplier * 1.5;
    } else if age < main_lifespan + subgiant_lifespan + giant_lifespan {
        // Giant
        radius = radius * rand_multiplier * 3.0;
    } else {
        // Remnant
        if mass < 8.0 {
            // White dwarf
            radius = radius / 60.0;
        } else if mass < 50.0 {
            // Neutron star
            radius = 0.001_f32.max((mass / (mass - 6.0) + mass) / 20000.0);
        } else {
            // Black hole
            radius = mass / 33333.33333;
        }
    }
    (radius * 1000.0).round() / 1000.0
}

fn calculate_main_sequence_luminosity(mass: f32) -> f32 {
    if mass <= 0.43 {
        0.23 * f32::powf(mass, 2.3)
    } else if mass <= 2.0 {
        f32::powf(mass, 4.0)
    } else if mass <= 55.0 {
        1.4 * f32::powf(mass, 3.5)
    } else {
        32000.0 * mass
    }
}

/// In millions of years.
fn calculate_lifespan(mass: f32, luminosity: f32) -> f32 {
    f32::powi(10.0, 10) * mass as f32 / luminosity as f32 * 100.0
}

/// In millions of years.
fn generate_age(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    hex: &GalacticHex,
    galaxy: &Galaxy,
    universe: &Universe,
) -> f32 {
    let mut rng = SeededDiceRoller::new(
        &galaxy.seed,
        &format!("star_{}_{}_{}_age", coord, system_index, star_index),
    );
    if let StellarNeighborhoodAge::Ancient(years)
    | StellarNeighborhoodAge::Old(years)
    | StellarNeighborhoodAge::Young(years) = hex.neighborhood.age
    {
        years as f32
    } else if universe.era == StelliferousEra::AncientStelliferous
        || universe.era == StelliferousEra::EarlyStelliferous
    {
        (((universe.age * 1000.0) as f32) - 300.0)
            .min(((universe.age) as f32 * 1000.0) - rng.roll(1, 9000, 0) as f32)
    } else {
        rng.roll(1, 9000, 999) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn print_out_test_stars() {
        for star in get_test_stars().iter() {
            print_tested_star(
                star,
                calculate_radius(
                    star.mass,
                    star.age,
                    99999.0,
                    99999.0,
                    99999.0,
                    1,
                    1,
                    SpaceCoordinates { x: 0, y: 0, z: 0 },
                    &Galaxy {
                        ..Default::default()
                    },
                ),
                calculate_main_sequence_luminosity(star.mass),
                calculate_luminosity_using_temperature(star.temperature, star.radius),
                calculate_temperature_using_luminosity(star.luminosity, star.radius),
            );
        }
    }

    #[test]
    fn calculate_proper_age() {
        for i in 0..1000 {
            let mut rng = SeededDiceRoller::new(&format!("{}", i), &"test_age");
            let settings = &GenerationSettings {
                galaxy: GalaxySettings {
                    ..Default::default()
                },
                ..Default::default()
            };
            let seed = String::from(&i.to_string());
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&seed, &settings),
                &seed,
                &settings,
            );
            let mut galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &seed, &settings);
            let coord = SpaceCoordinates::new(
                rng.gen_u16() as i64,
                rng.gen_u16() as i64,
                rng.gen_u16() as i64,
            );
            let age = generate_age(
                i as u16,
                i as u16 + 1,
                coord,
                &GalacticHex::generate(coord, coord, &mut galaxy),
                &galaxy,
                &galaxy.neighborhood.universe,
            );
            assert!(age > 0.0 && age < galaxy.neighborhood.universe.age);
        }
    }

    #[test]
    fn calculate_proper_spectral_type() {
        assert!(calculate_spectral_type(380000) == StarSpectralType::WR(2));
        assert!(calculate_spectral_type(170000) == StarSpectralType::WR(3));
        assert!(calculate_spectral_type(117000) == StarSpectralType::WR(4));
        assert!(calculate_spectral_type(54000) == StarSpectralType::O(2));
        assert!(calculate_spectral_type(45000) == StarSpectralType::O(3));
        assert!(calculate_spectral_type(43300) == StarSpectralType::O(4));
        assert!(calculate_spectral_type(40600) == StarSpectralType::O(5));
        assert!(calculate_spectral_type(39500) == StarSpectralType::O(6));
        assert!(calculate_spectral_type(37100) == StarSpectralType::O(7));
        assert!(calculate_spectral_type(35100) == StarSpectralType::O(8));
        assert!(calculate_spectral_type(33300) == StarSpectralType::O(9));
        assert!(calculate_spectral_type(29200) == StarSpectralType::B(0));
        assert!(calculate_spectral_type(23000) == StarSpectralType::B(1));
        assert!(calculate_spectral_type(21000) == StarSpectralType::B(2));
        assert!(calculate_spectral_type(17600) == StarSpectralType::B(3));
        assert!(calculate_spectral_type(15200) == StarSpectralType::B(5));
        assert!(calculate_spectral_type(14300) == StarSpectralType::B(6));
        assert!(calculate_spectral_type(13500) == StarSpectralType::B(7));
        assert!(calculate_spectral_type(12300) == StarSpectralType::B(8));
        assert!(calculate_spectral_type(11400) == StarSpectralType::B(9));
        assert!(calculate_spectral_type(9600) == StarSpectralType::A(0));
        assert!(calculate_spectral_type(9330) == StarSpectralType::A(1));
        assert!(calculate_spectral_type(9040) == StarSpectralType::A(2));
        assert!(calculate_spectral_type(8750) == StarSpectralType::A(3));
        assert!(calculate_spectral_type(8480) == StarSpectralType::A(4));
        assert!(calculate_spectral_type(8310) == StarSpectralType::A(5));
        assert!(calculate_spectral_type(7920) == StarSpectralType::A(7));
        assert!(calculate_spectral_type(7350) == StarSpectralType::F(0));
        assert!(calculate_spectral_type(7200) == StarSpectralType::F(1));
        assert!(calculate_spectral_type(7050) == StarSpectralType::F(2));
        assert!(calculate_spectral_type(6850) == StarSpectralType::F(3));
        assert!(calculate_spectral_type(6700) == StarSpectralType::F(5));
        assert!(calculate_spectral_type(6550) == StarSpectralType::F(6));
        assert!(calculate_spectral_type(6400) == StarSpectralType::F(7));
        assert!(calculate_spectral_type(6300) == StarSpectralType::F(8));
        assert!(calculate_spectral_type(6050) == StarSpectralType::G(0));
        assert!(calculate_spectral_type(5930) == StarSpectralType::G(1));
        assert!(calculate_spectral_type(5800) == StarSpectralType::G(2));
        assert!(calculate_spectral_type(5660) == StarSpectralType::G(5));
        assert!(calculate_spectral_type(5440) == StarSpectralType::G(8));
        assert!(calculate_spectral_type(5240) == StarSpectralType::K(0));
        assert!(calculate_spectral_type(5110) == StarSpectralType::K(1));
        assert!(calculate_spectral_type(4960) == StarSpectralType::K(2));
        assert!(calculate_spectral_type(4800) == StarSpectralType::K(3));
        assert!(calculate_spectral_type(4600) == StarSpectralType::K(4));
        assert!(calculate_spectral_type(4400) == StarSpectralType::K(5));
        assert!(calculate_spectral_type(4000) == StarSpectralType::K(7));
        assert!(calculate_spectral_type(3750) == StarSpectralType::M(0));
        assert!(calculate_spectral_type(3700) == StarSpectralType::M(1));
        assert!(calculate_spectral_type(3600) == StarSpectralType::M(2));
        assert!(calculate_spectral_type(3500) == StarSpectralType::M(3));
        assert!(calculate_spectral_type(3400) == StarSpectralType::M(4));
        assert!(calculate_spectral_type(3200) == StarSpectralType::M(5));
        assert!(calculate_spectral_type(3100) == StarSpectralType::M(6));
        assert!(calculate_spectral_type(2900) == StarSpectralType::M(7));
        assert!(calculate_spectral_type(2700) == StarSpectralType::M(8));
        assert!(calculate_spectral_type(2600) == StarSpectralType::L(0));
        assert!(calculate_spectral_type(2200) == StarSpectralType::L(3));
        assert!(calculate_spectral_type(1500) == StarSpectralType::L(8));
        assert!(calculate_spectral_type(1400) == StarSpectralType::T(2));
        assert!(calculate_spectral_type(1000) == StarSpectralType::T(6));
        assert!(calculate_spectral_type(800) == StarSpectralType::T(8));
        assert!(calculate_spectral_type(370) == StarSpectralType::Y(0));
        assert!(calculate_spectral_type(350) == StarSpectralType::Y(1));
        assert!(calculate_spectral_type(320) == StarSpectralType::Y(2));
        assert!(calculate_spectral_type(250) == StarSpectralType::Y(4));
    }
}
