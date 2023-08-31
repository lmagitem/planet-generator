use crate::prelude::*;

/// The average age of a [StellarNeighborhood].
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum StellarNeighborhoodAge {
    /// Young neighborhoods begin as open clusters whose stars drift apart over millions of years. They are often still surrounded by the
    /// nebula from which they formed. Most stars in these clusters are of the same age. The associated number is the actual age of the
    /// neighborhood in millions of years.
    Young(u64),
    /// Mixed populations of mostly main sequence stars of varying ages and types.
    #[default]
    Mature,
    /// Star formation in old neighborhoods is minimal, and has been so for a long time. The associated number is the actual age of the
    /// neighborhood in millions of years.
    Old(u64),
    /// Ancient stars were the first generations to be born. No star formation has occurred in these neighborhoods since a very long time.
    /// The associated number is the actual age of the neighborhood in millions of years.
    Ancient(u64),
}

impl Display for StellarNeighborhoodAge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StellarNeighborhoodAge::Young(difference) => {
                write!(f, "Young ({} million years)", difference)
            }
            StellarNeighborhoodAge::Mature => write!(f, "Mature"),
            StellarNeighborhoodAge::Old(difference) => {
                write!(f, "Old ({} million years)", difference)
            }
            StellarNeighborhoodAge::Ancient(difference) => {
                write!(f, "Ancient ({} million years)", difference)
            }
        }
    }
}
