use crate::prelude::*;

/// The average age of a [StellarNeighborhood].
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum StellarNeighborhoodAge {
    /// Young neighborhoods begin as open clusters whose stars drift apart over millions of years. They are often still surrounded by the
    /// nebula from which they formed. Most stars in these clusters are of the same age.
    Young,
    /// Mixed populations of mostly main sequence stars of varying ages and types.
    #[default]
    Mature,
    /// Star formation in old neighborhoods is minimal, and has been so for a long time.
    Old,
    /// Ancient stars were the first generations to be born. No star formation has occured in these neighborhoods since a very long time.
    Ancient,
}
