use crate::{prelude::*, system::generator::StellarEvolution};

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
        let mass = generate_mass(star_index, system_index, coord, galaxy);
        let luminosity = calculate_luminosity(mass);
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

        Self {}
    }
}

fn calculate_luminosity(mass: f32) -> f32 {
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

/// In billions of years.
fn calculate_lifespan(mass: f32, luminosity: f32) -> f32 {
    f32::powi(10.0, 10) * mass as f32 / luminosity as f32 / 10.0
}

/// In billions of years.
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
            .min((universe.age) as f32 - rng.roll(1, 10, -1) as f32)
            * 1000.0
    } else {
        rng.roll(1, 91, 9) as f32 / 10.0
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
