use crate::prelude::*;

impl GaseousDetails {
    pub(crate) fn generate_gas_giant(
        id: u32,
        orbit: Option<Orbit>,
        orbital_point_id: u32,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        coord: SpaceCoordinates,
        seed: String,
        settings: CelestialBodySettings,
    ) -> CelestialBody {
        CelestialBody {
            orbit,
            orbital_point_id,
            details: CelestialBodyDetails::Gaseous(GaseousDetails {}),
        }
    }
}
