use crate::prelude::*;
pub mod generator;
pub mod types;

/// A more or less coherent neighborhood of stars.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct StellarNeighborhood {
    /// The age of this neighborhood.
    pub age: StellarNeighborhoodAge,
}

impl StellarNeighborhood {
    /// Creates a new instance of [StellarNeighborhood].
    pub fn new(age: StellarNeighborhoodAge) -> Self {
        Self { age }
    }
}

impl Display for StellarNeighborhood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} stellar neighborhood", self.age,)
    }
}
