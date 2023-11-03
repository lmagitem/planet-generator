use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

/// The smaller division on a galactic map, might contain one or multiple star systems.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalacticHex {
    /// The index of this hex on the x, y and z axis.
    pub index: SpaceCoordinates,
    /// The neighborhood this hex belongs to.
    pub neighborhood: StellarNeighborhood,
    /// The star systems one can find in this hex, if any.
    pub contents: Vec<StarSystem>,
}

impl GalacticHex {
    /// Creates a new instance of [GalacticHex].
    pub fn new(
        index: SpaceCoordinates,
        neighborhood: StellarNeighborhood,
        contents: Vec<StarSystem>,
    ) -> Self {
        Self {
            index,
            neighborhood,
            contents,
        }
    }
}

impl Display for GalacticHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hex {} in {} containing {} star systems",
            self.index,
            self.neighborhood,
            self.contents.len(),
        )
    }
}
