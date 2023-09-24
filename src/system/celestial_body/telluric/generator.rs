use crate::prelude::*;

impl TelluricBodyDetails {
    /// Generates a barebone rocky body to use in system generation.
    pub(crate) fn generate_rocky_body_stub(orbital_point_id: u32) -> CelestialBody {
        CelestialBody {
            stub: true,
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                CelestialBodySubType::Rocky,
            )),
        }
    }

    /// Generates a fully fledged rocky body.
    pub(crate) fn generate_rocky_body(
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_traits: &Vec<StarPeculiarity>,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> CelestialBody {
        CelestialBody {
            stub: false,
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                CelestialBodySubType::Rocky,
            )),
        }
    }

    /// Generates a barebone metallic body to use in system generation.
    pub(crate) fn generate_metallic_body_stub(orbital_point_id: u32) -> CelestialBody {
        CelestialBody {
            stub: true,
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                CelestialBodySubType::Metallic,
            )),
        }
    }

    /// Generates a fully fledged metallic body.
    pub(crate) fn generate_metallic_body(
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_traits: &Vec<StarPeculiarity>,
        coord: SpaceCoordinates,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> CelestialBody {
        CelestialBody {
            stub: false,
            orbit: None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
            orbital_point_id,
            details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                CelestialBodySubType::Metallic,
            )),
        }
    }
}
