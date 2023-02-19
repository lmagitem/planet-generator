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
        population: StellarEvolution,
        system: &StarSystem,
        hex: &GalacticHex,
        galaxy: &Galaxy,
        settings: &GenerationSettings,
    ) -> Self {
        let age = if settings.star.fixed_age.is_some() {
            settings.star.fixed_age.unwrap() * 1000.0
        } else {
            generate_age(
                star_index,
                system_index,
                coord,
                hex,
                &galaxy,
                &galaxy.neighborhood.universe,
            )
        };

        let mut mass = if settings.star.fixed_mass.is_some() {
            settings.star.fixed_mass.unwrap()
        } else {
            let generated_mass = generate_mass(star_index, system_index, coord, galaxy);
            let adjusted_mass = adjust_mass_to_population(generated_mass, population);
            simulate_mass_loss_over_the_years(adjusted_mass, age)
        };

        // Main sequence estimations
        let ms_luminosity = calculate_main_sequence_luminosity(mass);
        let ms_radius = calculate_radius(mass, 0.0, 1.0, 0.0, 0.0, 0, 0, coord, galaxy);
        let ms_temperature =
            calculate_temperature_using_luminosity(ms_luminosity, ms_radius as f64) as u32;

        let main_lifespan = calculate_lifespan(mass, ms_luminosity);
        let subgiant_lifespan = calculate_subgiant_lifespan(mass, main_lifespan);
        let giant_lifespan = calculate_giant_lifespan(mass, main_lifespan);
        let full_lifespan = main_lifespan + subgiant_lifespan + giant_lifespan;

        let age_range = get_age_range_in_star_lifecycle_dataset(
            age,
            main_lifespan,
            subgiant_lifespan,
            giant_lifespan,
        );

        let radius: f32;
        let luminosity: f32;
        let temperature: u32;
        let spectral_type: StarSpectralType;
        let luminosity_class: StarLuminosityClass;

        if age_range > 6.0 {
            // If remnant
            mass = calculate_remnant_mass(mass, settings);

            if mass < 1.4 {
                // White dwarf
                radius = calculate_white_dwarf_radius(mass);
                let initial_luminosity = calculate_white_dwarf_initial_luminosity(mass);
                let initial_temperature =
                    calculate_temperature_using_luminosity(initial_luminosity, radius as f64);
                temperature =
                    calculate_white_dwarf_temperature(initial_temperature, full_lifespan, age);
                luminosity = calculate_luminosity_using_temperature(temperature, radius as f64);
                spectral_type =
                    generate_white_dwarf_spectral_type(star_index, system_index, coord, galaxy);
                luminosity_class = StarLuminosityClass::VII;
            } else if mass < 3.2 {
                // Neutron star
                let precise_radius = calculate_precise_radius_of_neutron_star_or_black_hole(mass);
                temperature = calculate_neutron_star_temperature(age, full_lifespan);
                luminosity = calculate_luminosity_using_temperature(temperature, precise_radius);
                radius = precise_radius as f32;
                spectral_type = StarSpectralType::XNS;
                luminosity_class = StarLuminosityClass::XNS;
            } else {
                // Black hole
                let precise_radius = calculate_precise_radius_of_neutron_star_or_black_hole(mass);
                temperature = 0;
                luminosity = 0.0;
                radius = precise_radius as f32;
                spectral_type = StarSpectralType::XBH;
                luminosity_class = StarLuminosityClass::XBH;
            }
        } else {
            // If main sequence, subgiant or giant
            let mass_range = get_mass_range_in_star_lifecycle_dataset(mass);
            let nearest_values = get_nearest_star_lifecycle_dataset_cells(age_range, mass_range);

            // Compute interpolated values
            let interpolated_temperature = get_interpolated_temperature(
                mass,
                ms_temperature,
                nearest_values,
                age_range,
                mass_range,
            );
            let interpolated_lum_factor =
                get_interpolated_luminosity_factor(nearest_values, age_range, mass_range);
            let interpolated_luminosity =
                get_interpolated_luminosity(mass, ms_luminosity, interpolated_lum_factor);
            let interpolated_radius = get_interpolated_radius(
                mass,
                ms_radius,
                interpolated_luminosity,
                interpolated_temperature,
            );

            // Then mix main sequence and interpolated values if applicable
            radius = mix_values(ms_radius, interpolated_radius, age, main_lifespan);
            luminosity = mix_values(ms_luminosity, interpolated_luminosity, age, main_lifespan);
            temperature = mix_values(
                ms_temperature as f32,
                interpolated_temperature as f32,
                age,
                main_lifespan,
            ) as u32;

            // TODO: Change temperatures ranges according to population
            spectral_type = calculate_spectral_type(temperature);
            luminosity_class = calculate_luminosity_class();
        }

        let name = get_star_name(
            star_index,
            system_index,
            coord,
            system,
            hex,
            galaxy,
            settings,
            spectral_type,
            luminosity_class,
        );
        Self {
            name,
            mass,
            luminosity,
            radius,
            age: age / 1000.0,
            temperature,
            spectral_type,
            luminosity_class,
        }
    }
}

/// TODO: Have the following format:
/// [X in base62 numbers of each division]:[Y in base62 numbers of each division]:[Z in base62 numbers of each division]
/// [System number] - [Some made up words] [Star letter if more than one] ([Spectral type] [Luminosity Class])
fn get_star_name(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    system: &StarSystem,
    hex: &GalacticHex,
    galaxy: &Galaxy,
    settings: &GenerationSettings,
    spectral_type: StarSpectralType,
    luminosity_class: StarLuminosityClass,
) -> String {
    if settings.star.use_ours {
        "Sun".to_string()
    } else {
        format!(
            "{}:{}:{} {} - Star {} ({} {})",
            coord.x, coord.y, coord.z, system_index, star_index, spectral_type, luminosity_class
        )
    }
}

fn calculate_temperature_using_luminosity(luminosity: f32, radius: f64) -> f32 {
    let pi = std::f64::consts::PI;
    let area = 4.0 * pi * (radius as f64).powf(2.0);
    // If I understood properly, the real approximation of the constant is 5.670367x10^-8, I have no idea why but I need to put 10^-17
    // in order to get working results.
    let sigma = 5.670367 * f64::powf(10.0, -17.0);
    // Same there, I've added the "x0.94304315" because it gives me results that are, on average (and exactly in the case of Sun), closer
    // to the ones I found on existing stars.
    let mut result = (luminosity as f64 / (area * sigma)).powf(1.0 / 4.0) as f32;
    result = result * 0.94304315;
    result
}

fn calculate_luminosity_using_temperature(temperature: u32, radius: f64) -> f32 {
    let pi = std::f64::consts::PI;
    let area = 4.0 * pi * (radius as f64).powf(2.0);
    // If I understood properly, the real approximation of the constant is 5.670367x10^-8, I have no idea why but I need to put 10^-17
    // in order to get working results.
    let sigma = 5.670367 * f64::powf(10.0, -17.0);
    // Same there, I've added the "x1.2643679" because it gives me results that are, on average (and exactly in the case of Sun), closer
    // to the ones I found on existing stars.
    let mut result = (sigma * area * (temperature as f64).powf(4.0)) as f32;
    result = result * 1.2643679;
    result
}

fn calculate_radius_using_luminosity_and_temperature(luminosity: f32, temperature: u32) -> f32 {
    let pi = std::f64::consts::PI;
    // If I understood properly, the real approximation of the constant is 5.670367x10^-8, I have no idea why but I need to put 10^-17
    // in order to get working results.
    let sigma = 5.670367 * f64::powf(10.0, -17.0);
    // Same there, I've added the "x0.88937" because it gives me results that are, on average (and exactly in the case of Sun), closer
    // to the ones I found on existing stars.
    let mut result =
        f64::sqrt(luminosity as f64 / (4.0 * pi * sigma * (temperature as f64).powf(4.0))) as f32;
    result = result * 0.88937;
    result
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
                    result: (BROWN_DWARF_MIN_MASS, RED_DWARF_POP_0_MIN_MASS - 0.001),
                    weight: 920,
                },
                // Red dwarf Pop 0
                CopyableWeightedResult {
                    result: (RED_DWARF_POP_0_MIN_MASS, RED_DWARF_POP_I_MIN_MASS - 0.001),
                    weight: 1078,
                },
                // Red dwarf Pop I
                CopyableWeightedResult {
                    result: (RED_DWARF_POP_I_MIN_MASS, RED_DWARF_POP_II_MIN_MASS - 0.001),
                    weight: 1013,
                },
                // Red dwarf Pop II
                CopyableWeightedResult {
                    result: (RED_DWARF_POP_II_MIN_MASS, 0.25),
                    weight: 2252,
                },
                CopyableWeightedResult {
                    result: (0.251, RED_DWARF_POP_III_MIN_MASS - 0.001),
                    weight: 1344,
                },
                // Red dwarf Pop III
                CopyableWeightedResult {
                    result: (RED_DWARF_POP_III_MIN_MASS, ORANGE_DWARF_MIN_MASS - 0.001),
                    weight: 896,
                },
                // Orange
                CopyableWeightedResult {
                    result: (ORANGE_DWARF_MIN_MASS, YELLOW_DWARF_MIN_MASS - 0.001),
                    weight: 1520,
                },
                // Yellow
                CopyableWeightedResult {
                    result: (YELLOW_DWARF_MIN_MASS, WHITE_DWARF_MIN_MASS - 0.001),
                    weight: 640,
                },
                // White
                CopyableWeightedResult {
                    result: (WHITE_DWARF_MIN_MASS, WHITE_GIANT_MIN_MASS - 0.001),
                    weight: 240,
                },
                // Giants
                CopyableWeightedResult {
                    result: (WHITE_GIANT_MIN_MASS, BLUE_GIANT_MIN_MASS - 0.001),
                    weight: 64,
                },
                // Blue giants
                CopyableWeightedResult {
                    result: (BLUE_GIANT_MIN_MASS, 20.0),
                    weight: 24,
                },
                CopyableWeightedResult {
                    result: (20.001, BLUE_GIANT_POP_0_MAX_MASS),
                    weight: 2,
                },
                // Pop I
                CopyableWeightedResult {
                    result: (BLUE_GIANT_POP_0_MAX_MASS + 0.001, BLUE_GIANT_POP_I_MAX_MASS),
                    weight: 1,
                },
                // Pop II
                CopyableWeightedResult {
                    result: (
                        BLUE_GIANT_POP_I_MAX_MASS + 0.001,
                        BLUE_GIANT_POP_II_MAX_MASS,
                    ),
                    weight: 1,
                },
                // Pop III
                CopyableWeightedResult {
                    result: (
                        BLUE_GIANT_POP_II_MAX_MASS + 0.001,
                        BLUE_GIANT_POP_III_MAX_MASS,
                    ),
                    weight: 1,
                },
            ],
            RollMethod::SimpleRoll,
        ))
        .expect("Should return a range to generate a star's mass.");
    rng.gen_f32() % (range.1 - range.0) + range.0
}

fn adjust_mass_to_population(mass: f32, population: StellarEvolution) -> f32 {
    match population {
        StellarEvolution::Population0 => {
            if mass > BLUE_GIANT_POP_0_MAX_MASS {
                BLUE_GIANT_POP_0_MAX_MASS
            } else {
                mass
            }
        }
        StellarEvolution::PopulationI => {
            if mass > BLUE_GIANT_POP_I_MAX_MASS {
                BLUE_GIANT_POP_I_MAX_MASS
            } else {
                mass
            }
        }
        StellarEvolution::PopulationII => {
            if mass > BLUE_GIANT_POP_II_MAX_MASS {
                BLUE_GIANT_POP_II_MAX_MASS
            } else {
                mass
            }
        }
        _ => mass,
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
    // From what I found online, mass^.8 should return a good approximation, but I've found better results by doing the following:
    let mut radius = mass.powf(0.78);
    radius += radius * 2.5666;

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
    let mut age = if let StellarNeighborhoodAge::Ancient(years)
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
    };
    age = if age >= universe.age * 1000.0 - 40.0 {
        universe.age * 1000.0 - 40.0
    } else if age < 50.0 {
        50.0
    } else {
        age
    };
    age
}

/// In millions of years.
fn calculate_lifespan(mass: f32, luminosity: f32) -> f32 {
    f32::powi(10.0, 4) * mass as f32 / luminosity as f32
}

fn calculate_subgiant_lifespan(mass: f32, main_lifespan: f32) -> f32 {
    if mass > RED_DWARF_POP_III_MIN_MASS {
        main_lifespan * 0.15
    } else {
        0.0
    }
}

fn calculate_giant_lifespan(mass: f32, main_lifespan: f32) -> f32 {
    if mass > RED_DWARF_POP_III_MIN_MASS {
        main_lifespan * 0.0917
    } else {
        0.0
    }
}

fn get_interpolated_radius(
    mass: f32,
    ms_radius: f32,
    interpolated_luminosity: f32,
    interpolated_temperature: u32,
) -> f32 {
    if mass < 0.4 {
        ms_radius
    } else {
        calculate_radius_using_luminosity_and_temperature(
            interpolated_luminosity,
            interpolated_temperature as u32,
        )
    }
}

fn get_interpolated_luminosity(mass: f32, ms_luminosity: f32, interpolated_lum_factor: f32) -> f32 {
    if mass < 0.4 {
        ms_luminosity
    } else {
        f32::powf(10.0, interpolated_lum_factor)
    }
}

fn get_interpolated_luminosity_factor(
    nearest_values: [TemperatureAndLuminosity; 4],
    age_range: f32,
    mass_range: f32,
) -> f32 {
    interpolate_f32(
        nearest_values[0].1,
        nearest_values[1].1,
        nearest_values[2].1,
        nearest_values[3].1,
        age_range,
        mass_range,
    )
}

fn get_interpolated_temperature(
    mass: f32,
    ms_temperature: u32,
    nearest_values: [TemperatureAndLuminosity; 4],
    age_range: f32,
    mass_range: f32,
) -> u32 {
    if mass < 0.4 {
        ms_temperature
    } else {
        interpolate_f32(
            nearest_values[0].0,
            nearest_values[1].0,
            nearest_values[2].0,
            nearest_values[3].0,
            age_range,
            mass_range,
        ) as u32
    }
}

fn calculate_spectral_type(temperature: u32) -> StarSpectralType {
    // Find the two temperatures in the dataset that the given temperature is between
    let (lower_temp, lower_class) = TEMPERATURE_TO_SPECTRAL_TYPE_DATASET
        .iter()
        .find(|&(t, _)| *t <= temperature)
        .unwrap();
    let (upper_temp, upper_class) = TEMPERATURE_TO_SPECTRAL_TYPE_DATASET
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

fn generate_white_dwarf_spectral_type(
    star_index: u16,
    system_index: u16,
    coord: SpaceCoordinates,
    galaxy: &Galaxy,
) -> StarSpectralType {
    let mut rng = SeededDiceRoller::new(
        &galaxy.seed,
        &format!("star_{}_{}_{}_wd_st", coord, system_index, star_index),
    );
    rng.get_result(&CopyableRollToProcess::new(
        vec![
            CopyableWeightedResult {
                result: StarSpectralType::DA,
                weight: 688,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DB,
                weight: 150,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DC,
                weight: 90,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DX,
                weight: 50,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DQ,
                weight: 15,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DZ,
                weight: 6,
            },
            CopyableWeightedResult {
                result: StarSpectralType::DO,
                weight: 1,
            },
        ],
        RollMethod::SimpleRoll,
    ))
    .expect("Should return a white dwarf spectral type.")
}

// TODO: Calculate luminosity class
fn calculate_luminosity_class() -> StarLuminosityClass {
    StarLuminosityClass::Ia
}

fn calculate_remnant_mass(mass: f32, settings: &GenerationSettings) -> f32 {
    if mass < 2.7 {
        0.096 * mass + 0.429
    } else {
        0.137 * mass + 0.318
    }
}

fn calculate_white_dwarf_temperature(
    initial_temperature: f32,
    full_lifespan: f32,
    age: f32,
) -> u32 {
    (initial_temperature * f32::powf(full_lifespan / age, 1.0 / 3.0)) as u32
}

fn calculate_white_dwarf_initial_luminosity(mass: f32) -> f32 {
    10.0_f32.powf(-2.15) * mass.powf(3.95)
}

fn calculate_white_dwarf_radius(mass: f32) -> f32 {
    0.0084 * mass.powf(-1.0 / 3.0)
}

/// TODO: That doesn't seem right at all, but at the moment I don't have anything better, so to be rewritten later.
fn calculate_neutron_star_temperature(age: f32, full_lifespan: f32) -> u32 {
    let neutron_star_age = age - full_lifespan * 1_000_000.0;
    let initial_temp = 1_000_000.0;
    let t_cool = 1.0 / (0.02 * (neutron_star_age / 10.0).powf(1.5)); // Cooling timescale in years
    let t_sec = 3.15e7 * t_cool; // Cooling timescale in seconds
    (initial_temp * ((t_sec / 1.0e6).ln() / (neutron_star_age / 10.0))) as u32
}

fn calculate_precise_radius_of_neutron_star_or_black_hole(mass: f32) -> f64 {
    let g: f64 = 6.674e-11;
    let c: f64 = 299_792_458.0;
    let sun_in_km = 696_340.0;
    2.0 * g * mass as f64 / c * 2.0 / sun_in_km
}

/// * 2.0 because my dataset has half-way points
fn get_age_range_in_star_lifecycle_dataset(
    age: f32,
    main_lifespan: f32,
    subgiant_lifespan: f32,
    giant_lifespan: f32,
) -> f32 {
    let to_subgiant_lifespan = main_lifespan + subgiant_lifespan;
    let to_giant_lifespan = to_subgiant_lifespan + giant_lifespan;
    if age >= 0.0 && age <= main_lifespan {
        return (age / main_lifespan) * 2.0;
    } else if age > main_lifespan && age <= to_subgiant_lifespan {
        return 2.0 + ((age - main_lifespan) / to_subgiant_lifespan) * 2.0;
    } else if age > to_subgiant_lifespan && age <= to_giant_lifespan {
        return 4.0 + ((age - to_subgiant_lifespan) / to_giant_lifespan) * 2.0;
    } else {
        return 7.0;
    }
}

fn get_mass_range_in_star_lifecycle_dataset(mass: f32) -> f32 {
    if mass < 0.4 {
        return 0.0;
    } else if mass >= 0.4 && mass <= 0.5 {
        return mass / 0.5;
    } else if mass > 0.5 && mass <= 1.0 {
        return 1.0 + ((mass - 0.5) / (1.0 - 0.5));
    } else if mass > 1.0 && mass <= 2.0 {
        return 2.0 + ((mass - 1.0) / (2.0 - 1.0));
    } else if mass > 2.0 && mass <= 5.0 {
        return 3.0 + ((mass - 2.0) / (5.0 - 2.0));
    } else if mass > 5.0 && mass <= 15.0 {
        return 4.0 + ((mass - 5.0) / (15.0 - 5.0));
    } else if mass > 15.0 && mass <= 60.0 {
        return 5.0 + ((mass - 15.0) / (60.0 - 15.0));
    } else if mass > 60.0 && mass <= 500.0 {
        return 6.0 + ((mass - 60.0) / (500.0 - 60.0));
    } else {
        return 8.0;
    }
}

fn get_nearest_star_lifecycle_dataset_cells(
    age_range: f32,
    mass_range: f32,
) -> [TemperatureAndLuminosity; 4] {
    if age_range < 0.0 || age_range > 6.0 || mass_range < 0.0 || mass_range > 7.0 {
        panic!(
            "{}",
            format!(
                "age_range ({}) or mass_range ({}) is out of bounds",
                age_range, mass_range
            )
        );
    }

    let x = age_range as usize;
    let x1 = if age_range.fract() != 0.0 { x + 1 } else { x };
    let y = mass_range as usize;
    let y1 = if mass_range.fract() != 0.0 { y + 1 } else { y };

    let a = STAR_LIFECYCLE_DATASET[y][x];
    let b = STAR_LIFECYCLE_DATASET[y][x1];
    let c = STAR_LIFECYCLE_DATASET[y1][x];
    let d = STAR_LIFECYCLE_DATASET[y1][x1];

    [a, b, c, d]
}

/// Returns true if the star is currently in the main sequence phase of its life.
fn is_star_a_main_sequence_dwarf(star: &Star) -> bool {
    star.luminosity_class == StarLuminosityClass::V
        && (discriminant(&star.spectral_type) == discriminant(&StarSpectralType::WR(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::O(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::B(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::A(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::F(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::G(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::K(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::M(0)))
}

/// Returns true if the star is currently in the main sequence, subgiant or giant phase of its life.
fn is_star_main_sequence_or_giant(star: &Star) -> bool {
    (star.luminosity_class == StarLuminosityClass::O
        || star.luminosity_class == StarLuminosityClass::Ia
        || star.luminosity_class == StarLuminosityClass::Ib
        || star.luminosity_class == StarLuminosityClass::II
        || star.luminosity_class == StarLuminosityClass::III
        || star.luminosity_class == StarLuminosityClass::IV
        || star.luminosity_class == StarLuminosityClass::V
        || star.luminosity_class == StarLuminosityClass::IV)
        && (discriminant(&star.spectral_type) == discriminant(&StarSpectralType::WR(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::O(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::B(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::A(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::F(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::G(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::K(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::M(0)))
}

fn interpolate_f32(x0_y0: f32, x1_y0: f32, x0_y1: f32, x1_y1: f32, x: f32, y: f32) -> f32 {
    let xf = x.fract();
    let yf = y.fract();
    let i1 = x0_y0 * (1.0 - yf) + x0_y1 * yf;
    let i2 = x1_y0 * (1.0 - yf) + x1_y1 * yf;
    i1 * (1.0 - xf) + i2 * xf
}

/// My "main sequence only" calculation is almost good, but it's useless for subgiants and giants, and my calculation that interpolates from
/// a dataset of temperatures and luminosity for mass/age steps wasn't a bad idea per se, but I don't have enough data to make it effective.
/// However, I need to go forward, so I'll mix the two for main sequence and use the dataset for giants.
/// If you're reading this, have knowledge in the field and an idea of how to improve my star generation sequence in a more realistic way,
/// feel free to reach out to me and contribute!
fn mix_values(a: f32, b: f32, age: f32, main_lifespan: f32) -> f32 {
    let result;
    let pond_a = 0.3 + age / main_lifespan;
    if pond_a >= 1.0 {
        result = b;
    } else {
        let pond_b = 1.0 - pond_a;
        result = a * pond_a + b * pond_b;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_main_sequence_values_approaching_reality() {
        let mut n = 0;
        let mut rad_ms_sum = 0.0;
        let mut lum_ms_sum = 0.0;
        let mut temp_ms_sum = 0.0;

        let coord = SpaceCoordinates {
            ..Default::default()
        };
        let galaxy = &Galaxy {
            ..Default::default()
        };

        for star in get_test_stars().iter() {
            if is_star_a_main_sequence_dwarf(star) {
                let mass = star.mass;
                let ms_luminosity = calculate_main_sequence_luminosity(mass);
                let ms_radius = calculate_radius(mass, 0.0, 1.0, 0.0, 0.0, 0, 0, coord, galaxy);
                let ms_temperature =
                    calculate_temperature_using_luminosity(ms_luminosity, ms_radius as f64) as u32;

                n += 1;
                rad_ms_sum += get_difference_percentage(ms_radius, star.radius);
                lum_ms_sum += get_difference_percentage(ms_luminosity, star.luminosity);
                temp_ms_sum +=
                    get_difference_percentage(ms_temperature as f32, star.temperature as f32);

                print_real_to_generated_star_comparison(
                    star,
                    mass,
                    ms_radius,
                    ms_luminosity,
                    ms_temperature,
                    calculate_spectral_type(ms_temperature),
                    calculate_luminosity_class(),
                    star.age,
                );

                if star.name.eq("Sun") {
                    // The results should be really close to reality for the Sun
                    assert!(
                        star.radius - star.radius * 0.025 <= ms_radius
                            && ms_radius <= star.radius + star.radius * 0.025
                    );
                    assert!(
                        star.luminosity - star.luminosity * 0.025 <= ms_luminosity
                            && ms_luminosity <= star.luminosity + star.luminosity * 0.025
                    );
                    assert!(
                        star.temperature as f32 - star.temperature as f32 * 0.025
                            <= ms_temperature as f32
                            && ms_temperature as f32
                                <= star.temperature as f32 + star.temperature as f32 * 0.025
                    );
                }
            }
        }

        rad_ms_sum /= n as f32;
        lum_ms_sum /= n as f32;
        temp_ms_sum /= n as f32;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_ms_sum, lum_ms_sum, temp_ms_sum);

        assert!(-0.2 <= rad_ms_sum && rad_ms_sum <= 0.2);
        assert!(-0.2 <= lum_ms_sum && lum_ms_sum <= 0.2);
        assert!(-0.2 <= temp_ms_sum && temp_ms_sum <= 0.2);
    }

    #[test]
    fn generate_interpolated_values_approaching_reality() {
        let mut n = 0;
        let mut rad_sum = 0.0;
        let mut lum_sum = 0.0;
        let mut temp_sum = 0.0;

        let coord = SpaceCoordinates {
            ..Default::default()
        };
        let galaxy = &Galaxy {
            ..Default::default()
        };

        for star in get_test_stars().iter() {
            if is_star_main_sequence_or_giant(star) {
                let age = star.age * 1000.0;
                let mass = star.mass;

                // Main sequence estimations
                let ms_luminosity = calculate_main_sequence_luminosity(mass);
                let ms_radius = calculate_radius(mass, 0.0, 1.0, 0.0, 0.0, 0, 0, coord, galaxy);
                let ms_temperature =
                    calculate_temperature_using_luminosity(ms_luminosity, ms_radius as f64) as u32;

                let main_lifespan = calculate_lifespan(mass, ms_luminosity);
                let age_range = get_age_range_in_star_lifecycle_dataset(
                    age,
                    main_lifespan,
                    calculate_subgiant_lifespan(mass, main_lifespan),
                    calculate_giant_lifespan(mass, main_lifespan),
                );
                let mass_range = get_mass_range_in_star_lifecycle_dataset(mass);
                if age_range < 7.0 && mass_range < 6.0 {
                    let nearest_values =
                        get_nearest_star_lifecycle_dataset_cells(age_range, mass_range);

                    // Compute interpolated values
                    let interpolated_temperature = get_interpolated_temperature(
                        mass,
                        ms_temperature,
                        nearest_values,
                        age_range,
                        mass_range,
                    );
                    let interpolated_lum_factor =
                        get_interpolated_luminosity_factor(nearest_values, age_range, mass_range);
                    let interpolated_luminosity =
                        get_interpolated_luminosity(mass, ms_luminosity, interpolated_lum_factor);
                    let interpolated_radius = get_interpolated_radius(
                        mass,
                        ms_radius,
                        interpolated_luminosity,
                        interpolated_temperature,
                    );

                    n += 1;
                    rad_sum += get_difference_percentage(interpolated_radius, star.radius);
                    lum_sum += get_difference_percentage(interpolated_luminosity, star.luminosity);
                    temp_sum += get_difference_percentage(
                        interpolated_temperature as f32,
                        star.temperature as f32,
                    );

                    print_real_to_generated_star_comparison(
                        star,
                        mass,
                        interpolated_radius,
                        interpolated_luminosity,
                        interpolated_temperature,
                        calculate_spectral_type(interpolated_temperature),
                        calculate_luminosity_class(),
                        age * 1000.0,
                    );

                    if star.name.eq("Sun") {
                        // The results should be really close to reality for the Sun
                        assert!(
                            star.radius - star.radius * 0.025 <= interpolated_radius
                                && interpolated_radius <= star.radius + star.radius * 0.025
                        );
                        assert!(
                            star.luminosity - star.luminosity * 0.025 <= interpolated_luminosity
                                && interpolated_luminosity
                                    <= star.luminosity + star.luminosity * 0.025
                        );
                        assert!(
                            star.temperature as f32 - star.temperature as f32 * 0.025
                                <= interpolated_temperature as f32
                                && interpolated_temperature as f32
                                    <= star.temperature as f32 + star.temperature as f32 * 0.025
                        );
                    }
                }
            }
        }

        rad_sum /= n as f32;
        lum_sum /= n as f32;
        temp_sum /= n as f32;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_sum, lum_sum, temp_sum);

        assert!(-0.2 <= rad_sum && rad_sum <= 0.2);
        assert!(-0.2 <= lum_sum && lum_sum <= 0.2);
        assert!(-0.2 <= temp_sum && temp_sum <= 0.2);
    }

    #[test]
    fn calculate_values_approaching_reality() {
        let mut n = 0;
        let mut rad_calc_sum = 0.0;
        let mut lum_calc_sum = 0.0;
        let mut temp_calc_sum = 0.0;

        for star in get_test_stars().iter() {
            if is_star_main_sequence_or_giant(star) {
                let calc_radius = calculate_radius_using_luminosity_and_temperature(
                    star.luminosity,
                    star.temperature,
                );
                let calc_luminosity =
                    calculate_luminosity_using_temperature(star.temperature, star.radius as f64);
                let calc_temperature =
                    calculate_temperature_using_luminosity(star.luminosity, star.radius as f64);

                n += 1;
                rad_calc_sum += get_difference_percentage(calc_radius, star.radius);
                lum_calc_sum += get_difference_percentage(calc_luminosity, star.luminosity);
                temp_calc_sum +=
                    get_difference_percentage(calc_temperature as f32, star.temperature as f32);

                print_real_to_generated_star_comparison(
                    star,
                    star.mass,
                    calc_radius,
                    calc_luminosity,
                    calc_temperature as u32,
                    calculate_spectral_type(calc_temperature as u32),
                    calculate_luminosity_class(),
                    star.age,
                );

                if star.name.eq("Sun") {
                    // The results should be really close to reality for the Sun
                    assert!(
                        star.radius - star.radius * 0.025 <= calc_radius
                            && calc_radius <= star.radius + star.radius * 0.025
                    );
                    assert!(
                        star.luminosity - star.luminosity * 0.025 <= calc_luminosity
                            && calc_luminosity <= star.luminosity + star.luminosity * 0.025
                    );
                    assert!(
                        star.temperature as f32 - star.temperature as f32 * 0.025
                            <= calc_temperature as f32
                            && calc_temperature as f32
                                <= star.temperature as f32 + star.temperature as f32 * 0.025
                    );
                }
            }
        }

        rad_calc_sum /= n as f32;
        lum_calc_sum /= n as f32;
        temp_calc_sum /= n as f32;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_calc_sum, lum_calc_sum, temp_calc_sum);

        assert!(-0.2 <= rad_calc_sum && rad_calc_sum <= 0.2);
        assert!(-0.2 <= lum_calc_sum && lum_calc_sum <= 0.2);
        assert!(-0.2 <= temp_calc_sum && temp_calc_sum <= 0.2);
    }

    #[test]
    fn generate_stars_looking_like_actual_stars() {
        let mut n = 0;
        let mut rad_sum = 0.0;
        let mut lum_sum = 0.0;
        let mut temp_sum = 0.0;

        let coord = SpaceCoordinates {
            ..Default::default()
        };
        let hex = &GalacticHex {
            ..Default::default()
        };
        let galaxy = &Galaxy {
            ..Default::default()
        };

        for star in get_test_stars().iter() {
            if is_star_main_sequence_or_giant(star) {
                let settings = GenerationSettings {
                    star: StarSettings {
                        fixed_age: Some(star.age),
                        fixed_mass: Some(star.mass),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let mut generated_star = Star::generate(
                    0,
                    0,
                    coord,
                    StellarEvolution::PopulationI,
                    &StarSystem {
                        ..Default::default()
                    },
                    hex,
                    galaxy,
                    &settings,
                );
                generated_star.name = star.name.clone();

                print_real_to_generated_star_comparison(
                    star,
                    generated_star.mass,
                    generated_star.radius,
                    generated_star.luminosity,
                    generated_star.temperature as u32,
                    generated_star.spectral_type,
                    generated_star.luminosity_class,
                    generated_star.age,
                );

                n += 1;
                rad_sum += get_difference_percentage(generated_star.radius, star.radius);
                lum_sum += get_difference_percentage(generated_star.luminosity, star.luminosity);
                temp_sum += get_difference_percentage(
                    generated_star.temperature as f32,
                    star.temperature as f32,
                );

                if star.name.eq("Sun") {
                    // The results should be really close to reality for the Sun
                    assert!(
                        star.radius - star.radius * 0.025 <= generated_star.radius
                            && generated_star.radius <= star.radius + star.radius * 0.025
                    );
                    assert!(
                        star.luminosity - star.luminosity * 0.025 <= generated_star.luminosity
                            && generated_star.luminosity
                                <= star.luminosity + star.luminosity * 0.025
                    );
                    assert!(
                        star.temperature as f32 - star.temperature as f32 * 0.025
                            <= generated_star.temperature as f32
                            && generated_star.temperature as f32
                                <= star.temperature as f32 + star.temperature as f32 * 0.025
                    );
                }
            }
        }

        rad_sum /= n as f32;
        lum_sum /= n as f32;
        temp_sum /= n as f32;

        // The results shouldn't have a variance higher than 10% in general
        print_real_to_generated_stars_comparison_results(rad_sum, lum_sum, temp_sum);

        assert!(-0.2 <= rad_sum && rad_sum <= 0.2);
        assert!(-0.2 <= lum_sum && lum_sum <= 0.2);
        assert!(-0.2 <= temp_sum && temp_sum <= 0.2);
    }

    // #[test]
    fn generate_fake_stars() {
        let mut rng = SeededDiceRoller::new("seed", "step");
        for i in 0..100 {
            let settings = GenerationSettings {
                star: StarSettings {
                    fixed_mass: Some(i as f32 / 50.0),
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
            let gal_end = galaxy.get_galactic_end();
            let x = rng.gen_u32() as i64 % gal_end.x;
            let y = rng.gen_u32() as i64 % gal_end.y;
            let z = rng.gen_u32() as i64 % gal_end.z;
            let coord = SpaceCoordinates::new(x, y, z);
            let hex = galaxy
                .get_hex(coord.rel(galaxy.get_galactic_start()))
                .expect("Should have generated a hex.");

            let generated_star = Star::generate(
                i,
                0,
                coord,
                StellarEvolution::PopulationI,
                &StarSystem {
                    ..Default::default()
                },
                &hex,
                &galaxy,
                &settings,
            );

            print_generated_star(&generated_star);
        }
    }

    #[test]
    fn calculate_proper_star_age() {
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
            let gal_end = galaxy.get_galactic_end();
            let x = rng.gen_u32() as i64 % gal_end.x;
            let y = rng.gen_u32() as i64 % gal_end.y;
            let z = rng.gen_u32() as i64 % gal_end.z;
            let coord = SpaceCoordinates::new(x, y, z);

            let age = generate_age(
                i as u16,
                i as u16 + 1,
                coord,
                &GalacticHex::generate(coord, coord, &mut galaxy),
                &galaxy,
                &galaxy.neighborhood.universe,
            ) / 1000.0;
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

    #[test]
    fn interpolate_temperature_properly() {
        let mut x0_y0 = 5000.0;
        let mut x1_y0 = 6000.0;
        let mut x0_y1 = 5500.0;
        let mut x1_y1 = 6500.0;
        let mut x = 2.5;
        let mut y = 1.5;
        let mut result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        let mut expected = 5750.0;
        assert!((result - expected).abs() < 0.001);

        x0_y0 = 0.0;
        x1_y0 = 1000.0;
        x0_y1 = 0.0;
        x1_y1 = 1000.0;
        x = 2.5;
        y = 1.5;
        result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        expected = 500.0;
        assert!((result - expected).abs() < 0.001);

        x0_y0 = 0.0;
        x1_y0 = 1000.0;
        x0_y1 = 500.0;
        x1_y1 = 1500.0;
        x = 2.5;
        y = 1.5;
        result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        expected = 750.0;
        assert!((result - expected).abs() < 0.001);

        x0_y0 = 0.0;
        x1_y0 = 1000.0;
        x0_y1 = 500.0;
        x1_y1 = 1500.0;
        x = 1.75;
        y = 1.5;
        result = interpolate_f32(x0_y0, x1_y0, x0_y1, x1_y1, x, y);
        expected = 1000.0;
        assert!((result - expected).abs() < 0.001);
    }

    fn print_real_to_generated_stars_comparison_results(rad_sum: f32, lum_sum: f32, temp_sum: f32) {
        println!(
        "\nVariance from generated values to real ones - radius: {}%, luminosity: {}%, temperature: {}%\n",
        format!("{}{}", if rad_sum > 0.0 {"+"} else {""}, rad_sum * 100.0),
        format!("{}{}", if lum_sum > 0.0 {"+"} else {""}, lum_sum * 100.0),
        format!("{}{}", if temp_sum > 0.0 {"+"} else {""}, temp_sum * 100.0),
    );
    }

    fn print_real_to_generated_star_comparison(
        star: &Star,
        mass: f32,
        radius: f32,
        luminosity: f32,
        temperature: u32,
        spectral_type: StarSpectralType,
        luminosity_class: StarLuminosityClass,
        age: f32,
    ) {
        println!(
            "     Real {} - mass: {}, rad: {}, lum: {}, temp: {}K, type: {} {}, age: {}",
            star.name,
            star.mass,
            star.radius,
            star.luminosity,
            star.temperature,
            star.spectral_type,
            star.luminosity_class,
            star.age
        );
        println!(
            "Generated {} - mass: {}, rad: {} ({}), lum: {} ({}), temp: {}K ({}), type: {} {}, age: {}\n",
            star.name,
            mass,
            radius,
            get_difference_percentage_str(radius, star.radius),
            luminosity,
            get_difference_percentage_str(luminosity, star.luminosity as f32),
            temperature,
            get_difference_percentage_str(temperature as f32, star.temperature as f32),
            spectral_type,
            luminosity_class,
            age
        );
    }

    fn print_generated_star(star: &Star) {
        println!(
            "{} - mass: {}, rad: {}, lum: {}, temp: {}K, type: {} {}, age: {}",
            star.name,
            star.mass,
            star.radius,
            star.luminosity,
            star.temperature,
            star.spectral_type,
            star.luminosity_class,
            star.age
        );
    }
}
