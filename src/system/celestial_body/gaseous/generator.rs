use crate::prelude::*;

impl GaseousDetails {
    pub(crate) fn generate_gas_giant(
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
            details: CelestialBodyDetails::Gaseous(GaseousDetails {}),
        }
    }
}
