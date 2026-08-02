use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::Display;

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
        /// Builds one "<count> <kind> galaxy/galaxies" fragment; the plural
        /// follows THIS count alone.
        fn count_fragment(count: u16, kind: &str) -> String {
            format!(
                "{} {} galax{}",
                count,
                kind,
                if count == 1 { "y" } else { "ies" }
            )
        }

        /// Joins one to three fragments the way this type always has:
        /// "a", "a and b", or "a, b and c".
        fn join(fragments: &[String]) -> String {
            match fragments.len() {
                0 => "no galaxies".to_string(),
                1 => fragments[0].clone(),
                2 => format!("{} and {}", fragments[0], fragments[1]),
                _ => format!("{}, {} and {}", fragments[0], fragments[1], fragments[2]),
            }
        }

        match self {
            GalacticNeighborhoodDensity::Void(g, m) => {
                let mut fragments = Vec::new();
                if *g > 0 {
                    fragments.push(count_fragment(u16::from(*g), "major"));
                }
                if *m > 0 {
                    fragments.push(count_fragment(*m, "minor"));
                }
                write!(f, "Void with {}", join(&fragments))
            }
            GalacticNeighborhoodDensity::Group(g, m) => {
                let mut fragments = Vec::new();
                if *g > 0 {
                    fragments.push(count_fragment(u16::from(*g), "major"));
                }
                if *m > 0 {
                    fragments.push(count_fragment(*m, "minor"));
                }
                write!(f, "Group with {}", join(&fragments))
            }
            GalacticNeighborhoodDensity::Cluster(d, g, m) => {
                let mut fragments = Vec::new();
                if *d > 0 {
                    fragments.push(count_fragment(u16::from(*d), "dominant"));
                }
                if *g > 0 {
                    fragments.push(count_fragment(u16::from(*g), "major"));
                }
                if *m > 0 {
                    fragments.push(count_fragment(*m, "minor"));
                }
                write!(f, "Cluster with {}", join(&fragments))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use GalacticNeighborhoodDensity::{Cluster, Group, Void};

    /// K14: each count's plural must follow its own count, a zero count must not
    /// leave a double space or a dangling separator, and the Cluster arm must
    /// print the MAJOR count in the major slot (it used to print the minor one).
    #[test]
    fn density_display_plurals_follow_each_count() {
        assert_eq!("Void with 1 major galaxy", Void(1, 0).to_string());
        assert_eq!("Void with 1 minor galaxy", Void(0, 1).to_string());
        assert_eq!(
            "Void with 2 major galaxies and 1 minor galaxy",
            Void(2, 1).to_string()
        );
        assert_eq!(
            "Void with 1 major galaxy and 3 minor galaxies",
            Void(1, 3).to_string()
        );
        assert_eq!(
            "Group with 1 major galaxy and 1 minor galaxy",
            Group(1, 1).to_string()
        );
        assert_eq!(
            "Group with 2 major galaxies and 23 minor galaxies",
            Group(2, 23).to_string()
        );
    }

    #[test]
    fn density_display_cluster_uses_the_major_count_in_the_major_slot() {
        // Regression pin: the major slot used to print the MINOR count (5 here).
        assert_eq!(
            "Cluster with 1 dominant galaxy, 2 major galaxies and 5 minor galaxies",
            Cluster(1, 2, 5).to_string()
        );
        assert_eq!(
            "Cluster with 1 dominant galaxy and 1 major galaxy",
            Cluster(1, 1, 0).to_string()
        );
        assert_eq!(
            "Cluster with 5 minor galaxies",
            Cluster(0, 0, 5).to_string()
        );
        assert_eq!(
            "Cluster with 1 dominant galaxy",
            Cluster(1, 0, 0).to_string()
        );
        assert_eq!(
            "Cluster with 2 dominant galaxies and 3 minor galaxies",
            Cluster(2, 0, 3).to_string()
        );
    }
}
