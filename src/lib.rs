#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code, unused_imports, unused)]
mod galaxy;
mod generator;
mod life;
mod system;
mod universe;
mod utils;

#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate simple_logger;

use log::LevelFilter;
use std::sync::Once;

pub mod prelude {
    pub use crate::galaxy::map::division::GalacticMapDivision;
    pub use crate::galaxy::map::division_level::GalacticMapDivisionLevel;
    pub use crate::galaxy::map::hex::types::*;
    pub use crate::galaxy::map::hex::GalacticHex;
    pub use crate::galaxy::map::types::*;
    pub use crate::galaxy::neighborhood::types::*;
    pub use crate::galaxy::neighborhood::GalacticNeighborhood;
    pub use crate::galaxy::types::*;
    pub use crate::galaxy::Galaxy;
    pub use crate::generator::types::*;
    pub use crate::generator::Generator;
    pub use crate::life::types::*;
    pub use crate::system::celestial_body::gaseous::types::*;
    pub use crate::system::celestial_body::gaseous::GaseousBodyDetails;
    pub use crate::system::celestial_body::icy::types::*;
    pub use crate::system::celestial_body::icy::IcyBodyDetails;
    pub use crate::system::celestial_body::telluric::types::*;
    pub use crate::system::celestial_body::telluric::TelluricBodyDetails;
    pub use crate::system::celestial_body::traits::types::*;
    pub use crate::system::celestial_body::traits::*;
    pub use crate::system::celestial_body::types::*;
    pub use crate::system::celestial_body::world::types::*;
    pub use crate::system::celestial_body::world::WorldGenerator;
    pub use crate::system::celestial_body::CelestialBody;
    pub use crate::system::celestial_disk::belt::types::*;
    pub use crate::system::celestial_disk::belt::CelestialBeltDetails;
    pub use crate::system::celestial_disk::ring::types::*;
    pub use crate::system::celestial_disk::ring::CelestialRingDetails;
    pub use crate::system::celestial_disk::types::*;
    pub use crate::system::celestial_disk::CelestialDisk;
    pub use crate::system::contents::elements::*;
    pub use crate::system::contents::types::*;
    pub use crate::system::neighborhood::types::*;
    pub use crate::system::neighborhood::StellarNeighborhood;
    pub use crate::system::orbital_point::types::*;
    pub use crate::system::orbital_point::OrbitalPoint;
    pub use crate::system::star::types::*;
    pub use crate::system::star::Star;
    pub use crate::system::types::*;
    pub use crate::system::StarSystem;
    pub use crate::universe::types::*;
    pub use crate::universe::Universe;
}

mod internal {
    pub use crate::system::celestial_body::moon::*;
    pub use crate::utils::conversion::ConversionUtils;
    pub use crate::utils::harmonics::OrbitalHarmonicsUtils;
    pub use crate::utils::math::MathUtils;
    pub use crate::utils::string::StringUtils;
    pub use log::*;
    pub use ordered_float::OrderedFloat;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
    pub use std::rc::Rc;
    pub use strum::IntoEnumIterator;
    pub use strum_macros::EnumIter;
}

lazy_static! {
    static ref LOGGER_INITIALIZED: Once = Once::new();
}

#[cfg(test)]
fn init_logger(level: LevelFilter) {
    LOGGER_INITIALIZED.call_once(|| {
        simple_logger::SimpleLogger::new()
            .with_level(level)
            .init()
            .unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::internal::*;
    use super::prelude::*;
    use super::*;
    use crate::system::star::get_star_color_code;
    use std::collections::HashSet;

    // #[test]
    fn add_logs_to_run() {
        init_logger(LevelFilter::Debug);
    }

    #[test]
    fn generate_example_systems() {
        // init_logger(LevelFilter::Debug);
        for i in 0..50 {
            let settings = &GenerationSettings {
                seed: Rc::from(i.to_string()),
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
            let universe = Universe::generate(&settings);
            let neighborhood = GalacticNeighborhood::generate(universe, &settings);
            let mut galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &settings);
            let coord = SpaceCoordinates::new(0, 0, 0);
            let sub_sector = galaxy
                .get_division_at_level(coord, 1)
                .expect("Should have returned a sub-sector.");
            let hex = galaxy.get_hex(coord).expect("Should have returned an hex.");
            let system = StarSystem::generate(i as u16, coord, &hex, &sub_sector, &mut galaxy);
            let main_star = system
                .clone()
                .all_objects
                .iter()
                .find(|o| o.id == system.main_star_id)
                .cloned()
                .unwrap()
                .object;

            print_system_bodies(i, system);
        }
    }

    // #[test]
    fn generate_interesting_example_systems() {
        // init_logger(LevelFilter::Debug);
        for i in 0..50 {
            let settings = &GenerationSettings {
                seed: Rc::from(i.to_string()),
                system: SystemSettings {
                    only_interesting: true,
                    ..Default::default()
                },
                ..Default::default()
            };
            let universe = Universe::generate(&settings);
            let neighborhood = GalacticNeighborhood::generate(universe, &settings);
            let mut galaxy = Galaxy::generate(neighborhood, (i as u16) % 5, &settings);
            let coord = SpaceCoordinates::new(0, 0, 0);
            let sub_sector = galaxy
                .get_division_at_level(coord, 1)
                .expect("Should have returned a sub-sector.");
            let hex = galaxy.get_hex(coord).expect("Should have returned an hex.");
            let system = StarSystem::generate(i as u16, coord, &hex, &sub_sector, &mut galaxy);
            let main_star = system
                .clone()
                .all_objects
                .iter()
                .find(|o| o.id == system.main_star_id)
                .cloned()
                .unwrap()
                .object;

            print_system_bodies(i, system);
        }
    }

    fn print_system_bodies(i: usize, system: StarSystem) {
        println!(
            "\n>>>>> {} - {}, traits: [{}]",
            i,
            system.name,
            &system
                .special_traits
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut sorted_objects = Vec::new();
        let mut visited = HashSet::new();

        // Sort the collected objects at each depth level by their orbital distance
        sort_by_orbital_distance(&mut sorted_objects);

        // Start the sort with the object that orbits nothing (i.e., the central object)
        if let Some(central_object) = system.all_objects.iter().find(|o| o.own_orbit.is_none()) {
            orbits_depth_first_sort(
                central_object.id,
                &system.all_objects,
                &mut sorted_objects,
                &mut visited,
                0,
            );
        }

        sorted_objects.iter().for_each(|(o, depth)| {
            println!(
                "{}{} ({} AU)\n{}\x1b[37m{}\x1b[0m\n{}{}{}\x1b[0m",
                " ".repeat(*depth * 2),
                format!("{:03}", o.id),
                StringUtils::to_significant_decimals(
                    o.own_orbit.clone().unwrap_or_default().average_distance
                ),
                " ".repeat(*depth * 2 + 4),
                o.own_orbit.clone().unwrap_or_default(),
                " ".repeat(*depth * 2 + 4),
                if let AstronomicalObject::Star(star) = o.object.clone() {
                    format!("{}", get_star_color_code(&star))
                } else {
                    String::new()
                },
                o.object
            );
        });
    }

    fn orbits_depth_first_sort(
        point_id: u32,
        points: &Vec<OrbitalPoint>,
        sorted_points: &mut Vec<(OrbitalPoint, usize)>,
        visited: &mut HashSet<u32>,
        current_depth: usize,
    ) {
        if visited.contains(&point_id) {
            return;
        }

        visited.insert(point_id);

        if let Some(point) = points.iter().find(|p| p.id == point_id) {
            sorted_points.push((point.clone(), current_depth));

            // Iterate over all orbital points to find direct satellites of 'point'
            for satellite in points.iter().filter(|p| {
                p.own_orbit
                    .as_ref()
                    .map_or(false, |o| o.primary_body_id == point_id)
            }) {
                orbits_depth_first_sort(
                    satellite.id,
                    points,
                    sorted_points,
                    visited,
                    current_depth + 1,
                );
            }
        }
    }

    fn sort_by_orbital_distance(sorted_points: &mut Vec<(OrbitalPoint, usize)>) {
        sorted_points.sort_by(|a, b| {
            let depth_a = a.1;
            let depth_b = b.1;
            let distance_a = a.0.own_orbit.clone().unwrap_or_default().average_distance;
            let distance_b = b.0.own_orbit.clone().unwrap_or_default().average_distance;

            if depth_a == depth_b {
                distance_a
                    .partial_cmp(&distance_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                depth_a.cmp(&depth_b)
            }
        });
    }
}
