use crate::prelude::*;

impl GalacticHex {
    /// Generates the [GalacticHex] at the given coordinates.
    pub fn generate(coord: SpaceCoordinates, index: SpaceCoordinates, galaxy: &mut Galaxy) -> Self {
        let contents = Vec::new();
        let neighborhood = StellarNeighborhood::generate(coord, galaxy);
        let mut generated = Self {
            index,
            neighborhood,
            contents,
        };

        let number_of_systems_to_generate = get_number_of_systems_to_generate(galaxy, index, coord);
        for i in 0..number_of_systems_to_generate {
            generated.contents.push(StarSystem::generate(
                i,
                coord,
                &generated,
                &galaxy
                    .get_division_at_level(coord, 1)
                    .expect("Should return a subsector."),
                galaxy,
            ));
        }

        generated
    }
}

/// Calculates how many systems should be generated using the expected stellar distribution of the hex.
fn get_number_of_systems_to_generate(
    galaxy: &mut Galaxy,
    index: SpaceCoordinates,
    coord: SpaceCoordinates,
) -> u16 {
    let mut rng = SeededDiceRoller::new(&galaxy.settings.seed, &format!("hex_{}_nbr_sys", index));
    let mut number_of_systems_to_generate = 0;
    let success_on;
    let to_roll: PreparedRoll;

    let turns = if galaxy.settings.sector.density_by_hex_instead_of_parsec {
        1
    } else {
        // Number of cubir parsecs
        let hex_size = galaxy.settings.sector.hex_size;
        hex_size.0 * hex_size.1 * hex_size.2
    };

    let region = galaxy
        .get_division_at_level(coord, 1)
        .expect("Should return a subsector.")
        .region;
    match region {
        GalacticRegion::Void => {
            to_roll = PreparedRoll::new(1, 50, 0);
            success_on = 1;
        }
        GalacticRegion::Aura => {
            to_roll = PreparedRoll::new(1, 20, 0);
            success_on = 1;
        }
        GalacticRegion::Halo | GalacticRegion::Exile => {
            to_roll = PreparedRoll::new(1, 10, 0);
            success_on = 1;
        }
        GalacticRegion::Stream | GalacticRegion::Association => {
            to_roll = PreparedRoll::new(1, 5, 0);
            success_on = 1;
        }
        GalacticRegion::Ellipse | GalacticRegion::Disk | GalacticRegion::Multiple => {
            to_roll = PreparedRoll::new(1, 2, 0);
            success_on = 1;
        }
        GalacticRegion::Arm | GalacticRegion::OpenCluster => {
            to_roll = PreparedRoll::new(1, 4, 0);
            success_on = 4;
        }
        GalacticRegion::Bar => {
            to_roll = PreparedRoll::new(1, 20, 0);
            success_on = 20;
        }
        GalacticRegion::Bulge | GalacticRegion::GlobularCluster => {
            to_roll = PreparedRoll::new(1, 100, 0);
            success_on = 100;
        }
        GalacticRegion::Core | GalacticRegion::Nucleus => {
            to_roll = PreparedRoll::new(1, 500, 0);
            success_on = 500;
        }
    };

    for _ in 0..turns {
        let roll = rng.roll_prepared(&to_roll);
        if roll <= success_on {
            number_of_systems_to_generate += roll;
        }
    }

    // Add a number of brown dwarfs
    rng = SeededDiceRoller::new(&galaxy.settings.seed, &format!("hex_{}_nbr_brwn", index));
    let mut number_of_brown_dwarfs = 0;
    for _ in 0..turns {
        let roll = rng.roll_prepared(&to_roll);
        if roll <= success_on {
            number_of_brown_dwarfs += roll;
        }
    }
    number_of_systems_to_generate += number_of_brown_dwarfs / 5;

    if galaxy.settings.sector.max_one_system_per_hex && number_of_systems_to_generate > 1 {
        number_of_systems_to_generate = 1;
    }

    number_of_systems_to_generate as u16
}
