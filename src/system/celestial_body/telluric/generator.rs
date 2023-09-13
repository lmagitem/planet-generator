use crate::prelude::*;

impl TelluricDetails {
    /// Generates a barebone rocky body to use in system generation.
    pub(crate) fn generate_rocky_body_stub(
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> CelestialBody {
        CelestialBody {
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricDetails::new(
                CelestialBodySubtype::Rocky,
            )),
        }
    }

    /// Generates a fully fledged rocky body.
    pub(crate) fn generate_rocky_body(
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> CelestialBody {
        CelestialBody {
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricDetails::new(
                CelestialBodySubtype::Rocky,
            )),
        }
    }

    /// Generates a barebone metallic body to use in system generation.
    pub(crate) fn generate_metallic_body_stub(
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> CelestialBody {
        CelestialBody {
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricDetails::new(
                CelestialBodySubtype::Metallic,
            )),
        }
    }

    /// Generates a fully fledged metallic body.
    pub(crate) fn generate_metallic_body(
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> CelestialBody {
        CelestialBody {
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricDetails::new(
                CelestialBodySubtype::Metallic,
            )),
        }
    }
}
