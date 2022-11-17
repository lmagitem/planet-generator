use crate::prelude::*;

/// Defines the density of a galactic neighborhood. The first associated number indicates how many major galaxies we find in that
/// neighborhood, the second indicates how many galaxies are dominant ones. Major galaxies within 2 megaparsecs (or 5 to 10 megaparsecs for
/// giant and dominant galaxies) tend to be gravitationally bound to each others.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum GalacticNeighborhoodDensity {
    /// The emptiest parts of the universe, covers a diameter far greater than the other densities. Contains 0 to 3 major galaxies stored in
    /// the first associated value and a certain number of minor ones stored in the second value.
    Void(#[default = 1] u8, #[default = 4] u16),
    /// A zone filled with a "regular" amount of galaxies. Contains 1 to 5 major galaxies stored in the first associated value and a certain
    /// number of minor ones stored in the second value.
    #[default]
    Group(#[default = 2] u8, #[default = 23] u16),
    /// The most crowded parts of the universe. Galaxies within this neighborhood usualy revolve around a huge dominant one. Space between
    /// galaxies is filled with super-hot plasma and a large number of intergalactic stars. Contains 5 to 20+ major galaxies. Thje first
    /// associated value is the number of dominant, the second value the number of major and the third the number of minor galaxies.
    Cluster(#[default = 1] u8, #[default = 8] u8, #[default = 209] u16),
}

impl Display for GalacticNeighborhoodDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GalacticNeighborhoodDensity::Void(g, m) => write!(
                f,
                "Void with {}{}{} galax{}",
                if g > &0 {
                    format!("{} major", g)
                } else {
                    String::from("")
                },
                if g > &0 && m > &0 {
                    String::from(" and ")
                } else {
                    String::from("")
                },
                if m > &0 {
                    format!("{} minor", m)
                } else {
                    String::from("")
                },
                if *m == 1 { "y" } else { "ies" }
            ),
            GalacticNeighborhoodDensity::Group(g, m) => write!(
                f,
                "Group with {}{}{} galax{}",
                if g > &0 {
                    format!("{} major", g)
                } else {
                    String::from("")
                },
                if g > &0 && m > &0 {
                    String::from(" and ")
                } else {
                    String::from("")
                },
                if m > &0 {
                    format!("{} minor", m)
                } else {
                    String::from("")
                },
                if *m == 1 { "y" } else { "ies" }
            ),
            GalacticNeighborhoodDensity::Cluster(d, g, m) => write!(
                f,
                "Cluster with {}{}{}{}{} galax{}",
                if d > &0 {
                    format!("{} dominant", d)
                } else {
                    String::from("")
                },
                if d > &0 && g > &0 && m > &0 {
                    String::from(", ")
                } else if (d > &0 && m > &0) || (d > &0 && g > &0) {
                    String::from(" and ")
                } else {
                    String::from("")
                },
                if g > &0 {
                    format!("{} major", m)
                } else {
                    String::from("")
                },
                if (m > &0 && d > &0) || (m > &0 && g > &0) {
                    String::from(" and ")
                } else {
                    String::from("")
                },
                if m > &0 {
                    format!("{} minor", m)
                } else {
                    String::from("")
                },
                if *m == 1 { "y" } else { "ies" }
            ),
        }
    }
}
