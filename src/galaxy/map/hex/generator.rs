use crate::prelude::*;

impl GalacticHex {
    pub fn generate(coord: SpaceCoordinates, index: SpaceCoordinates, galaxy: &mut Galaxy) -> Self {
        let contents = Vec::new();
        let neighborhood = StellarNeighborhood::generate(coord, galaxy);

        Self {
            index,
            neighborhood,
            contents,
        }
    }
}
