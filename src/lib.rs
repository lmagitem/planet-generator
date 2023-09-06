#![warn(clippy::all, clippy::pedantic)]
mod galaxy;
mod generator;
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
    pub use crate::generator::utils::GeneratorUtils;
    pub use crate::generator::Generator;
    pub use crate::system::celestial_body::gaseous::types::*;
    pub use crate::system::celestial_body::gaseous::GaseousDetails;
    pub use crate::system::celestial_body::icy::types::*;
    pub use crate::system::celestial_body::icy::IcyDetails;
    pub use crate::system::celestial_body::telluric::types::*;
    pub use crate::system::celestial_body::telluric::TelluricDetails;
    pub use crate::system::celestial_body::types::*;
    pub use crate::system::celestial_body::CelestialBody;
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
    pub use crate::utils::ConversionUtils;
    pub use log::*;
    pub use ordered_float::OrderedFloat;
    pub use seeded_dice_roller::*;
    pub use serde::{Deserialize, Serialize};
    pub use smart_default::SmartDefault;
    pub use std::fmt::Display;
    pub use std::mem::discriminant;
    pub use std::rc::Rc;
}

lazy_static! {
    static ref LOGGER_INITIALIZED: Once = Once::new();
}

#[cfg(test)]
fn init_logger() {
    LOGGER_INITIALIZED.call_once(|| {
        simple_logger::SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .init()
            .unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::*;

    // #[test]
    fn add_logs_to_run() {
        init_logger();
    }

    // #[test]
    fn generate_example_systems() {
        init_logger();
        for i in 0..50 {
            let settings = &GenerationSettings {
                seed: String::from(&i.to_string()),
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
            println!("\n{:#?}", system);
        }
    }

    // #[test]
    fn generate_interesting_example_systems() {
        init_logger();
        let mut highest_distance;
        for i in 0..50 {
            let settings = &GenerationSettings {
                seed: String::from(&i.to_string()),
                ..Default::default()
            };
            let universe = Universe::generate(&settings);
            let neighborhood = GalacticNeighborhood::generate(universe, &settings);
            let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
            let coord = SpaceCoordinates::new(0, 0, 0);
            let sub_sector = galaxy
                .get_division_at_level(coord, 1)
                .expect("Should have returned a sub-sector.");
            let hex = galaxy.get_hex(coord).expect("Should have returned an hex.");
            let system = StarSystem::generate(0, coord, &hex, &sub_sector, &mut galaxy);
            // Find in objects the one with the highest distance from primary body.
            let higher_distance = system
                .all_objects
                .iter()
                .map(|o| {
                    o.get_own_orbit()
                        .unwrap_or(Orbit {
                            ..Default::default()
                        })
                        .average_distance
                })
                .max_by(|a, b| a.total_cmp(b))
                .unwrap();
            if
            /* higher_distance > highest_distance */
            i % 500 == 0
                || system.center_id >= 13
                    && (system
                        .all_objects
                        .iter()
                        .filter(|o| {
                            let mut result = false;
                            if let AstronomicalObject::Star(star) = &o.object {
                                match star.spectral_type {
                                    StarSpectralType::WR(_)
                                    | StarSpectralType::O(_)
                                    | StarSpectralType::B(_)
                                    | StarSpectralType::A(_)
                                    | StarSpectralType::F(_)
                                    | StarSpectralType::G(_)
                                    | StarSpectralType::Y(_)
                                    | StarSpectralType::DA
                                    | StarSpectralType::DB
                                    | StarSpectralType::DC
                                    | StarSpectralType::DO
                                    | StarSpectralType::DZ
                                    | StarSpectralType::DQ
                                    | StarSpectralType::DX
                                    | StarSpectralType::XNS
                                    | StarSpectralType::XBH => {
                                        result = true;
                                    }
                                    _ => (),
                                }
                                match star.luminosity_class {
                                    StarLuminosityClass::O
                                    | StarLuminosityClass::Ia
                                    | StarLuminosityClass::Ib
                                    | StarLuminosityClass::II
                                    | StarLuminosityClass::III
                                    | StarLuminosityClass::IV
                                    | StarLuminosityClass::VII
                                    | StarLuminosityClass::XNS
                                    | StarLuminosityClass::XBH => {
                                        result = true;
                                    }
                                    _ => (),
                                }
                            }
                            result
                        })
                        .count()
                        > 4
                        || (system
                            .all_objects
                            .iter()
                            .filter(|o| {
                                let mut result = false;
                                if let AstronomicalObject::Star(star) = &o.object {
                                    match star.spectral_type {
                                        StarSpectralType::WR(_)
                                        | StarSpectralType::O(_)
                                        | StarSpectralType::B(_)
                                        | StarSpectralType::A(_)
                                        | StarSpectralType::F(_)
                                        | StarSpectralType::G(_)
                                        | StarSpectralType::Y(_)
                                        | StarSpectralType::DA
                                        | StarSpectralType::DB
                                        | StarSpectralType::DC
                                        | StarSpectralType::DO
                                        | StarSpectralType::DZ
                                        | StarSpectralType::DQ
                                        | StarSpectralType::DX
                                        | StarSpectralType::XNS
                                        | StarSpectralType::XBH => {
                                            result = true;
                                        }
                                        _ => (),
                                    }
                                    match star.luminosity_class {
                                        StarLuminosityClass::O
                                        | StarLuminosityClass::Ia
                                        | StarLuminosityClass::Ib
                                        | StarLuminosityClass::II
                                        | StarLuminosityClass::III
                                        | StarLuminosityClass::IV
                                        | StarLuminosityClass::VII
                                        | StarLuminosityClass::XNS
                                        | StarLuminosityClass::XBH => {
                                            result = true;
                                        }
                                        _ => (),
                                    }
                                }
                                result
                            })
                            .count()
                            > 1
                            && system
                                .all_objects
                                .iter()
                                .filter(|o| {
                                    let mut result = false;
                                    if let AstronomicalObject::Star(star) = &o.object {
                                        match star.spectral_type {
                                            StarSpectralType::WR(_)
                                            | StarSpectralType::O(_)
                                            | StarSpectralType::B(_)
                                            | StarSpectralType::A(_)
                                            | StarSpectralType::XNS
                                            | StarSpectralType::XBH => {
                                                result = true;
                                            }
                                            _ => (),
                                        }
                                    }
                                    result
                                })
                                .count()
                                > 0))
            {
                highest_distance = higher_distance;
                println!("\nseed: {}, distance: {}", settings.seed, highest_distance);
                println!("\n{:#?}", system);
            };
        }
    }
}
