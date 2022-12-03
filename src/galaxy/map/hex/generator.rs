use crate::prelude::*;

impl GalacticHex {
    pub fn generate(coord: SpaceCoordinates, index: SpaceCoordinates, galaxy: &Galaxy) -> Self {
        Self {
            index,
            contents: vec![],
        }
    }
}
