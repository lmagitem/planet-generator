use crate::prelude::*;
pub mod types;
pub mod utils;

/// The generator itself, depending on the given settings, can generate a full blown universe with multiple galaxies, sectors, systems,
/// planets and the species living in those.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Generator {}

impl Generator {
    /// Generates a full blown universe with multiple galaxies, sectors, systems, planets and the species living in those following the
    /// given [GenerationSettings], in a deterministic way thanks to the given **seed**.
    pub fn generate(seed: &String, settings: GenerationSettings) -> GeneratedUniverse {
        let universe = Universe::generate(seed, &settings);
        let galactic_neighborhood = GalacticNeighborhood::generate(universe, seed, &settings);
        let galaxies: Vec<Galaxy> = generate_galaxies(galactic_neighborhood, seed, settings);

        GeneratedUniverse {
            universe,
            galactic_neighborhood,
            galaxies,
        }
    }
}

/// Generates a list of [Galaxy] in the given **galactic_neighborhood** using the given **seed** and **settings**.
fn generate_galaxies(
    galactic_neighborhood: GalacticNeighborhood,
    seed: &String,
    settings: GenerationSettings,
) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];
    let to_generate: u16;
    match galactic_neighborhood.density {
        GalacticNeighborhoodDensity::Void(g, m) | GalacticNeighborhoodDensity::Group(g, m) => {
            to_generate = (g as u16) + m
        }
        GalacticNeighborhoodDensity::Cluster(d, g, m) => to_generate = (d as u16) + (g as u16) + m,
    }
    for i in 0..to_generate {
        galaxies.push(Galaxy::generate(galactic_neighborhood, i, seed, &settings));
    }
    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn system_test() {
        let settings = &GenerationSettings {
            universe: UniverseSettings {
                use_ours: true,
                ..Default::default()
            },
            galaxy: GalaxySettings {
                use_ours: true,
                ..Default::default()
            },
            ..Default::default()
        };
        for i in 0..50 {
            let seed = String::from(&i.to_string());
            let universe = Universe::generate(&seed, &settings);
            let neighborhood = GalacticNeighborhood::generate(universe, &seed, &settings);
            let mut galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &seed, &settings);
            let coord = SpaceCoordinates::new(0, 0, 0);
            let sub_sector = galaxy
                .get_division_at_level(coord, 1)
                .expect("Should have returned a sub-sector.");
            let hex = galaxy.get_hex(coord).expect("Should have returned an hex.");
            let system = StarSystem::generate(i as u16, coord, &hex, &sub_sector, &mut galaxy);
            println!("\n{:#?}", system);
        }
    }
}
