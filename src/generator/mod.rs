use crate::diagnostics::types::{Diagnostics, GenerationError};
use crate::galaxy::neighborhood::types::GalacticNeighborhoodDensity;
use crate::galaxy::neighborhood::GalacticNeighborhood;
use crate::galaxy::Galaxy;
use crate::generator::types::GeneratedUniverse;
use crate::generator::types::GenerationSettings;
use crate::universe::Universe;
pub mod types;

/// The generator itself, depending on the given settings, can generate a full blown universe with multiple galaxies, sectors, systems,
/// planets and the species living in those.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Generator {}

impl Generator {
    /// Generates a full blown universe with multiple galaxies, sectors, systems, planets and the species living in those following the
    /// given [`GenerationSettings`], in a deterministic way thanks to the given **seed**. Validates the settings first and returns
    /// [`GenerationError::InvalidSettings`] listing every violation found if any.
    ///
    /// # Errors
    /// Returns [`GenerationError::InvalidSettings`] if the given settings fail validation.
    pub fn generate(settings: GenerationSettings) -> Result<GeneratedUniverse, GenerationError> {
        if let Err(violations) = settings.validate() {
            return Err(GenerationError::InvalidSettings(violations));
        }
        let mut diagnostics = Diagnostics::default();
        let universe = Universe::generate(&settings, &mut diagnostics);
        let galactic_neighborhood =
            GalacticNeighborhood::generate(universe, &settings, &mut diagnostics);
        let galaxies: Vec<Galaxy> = generate_galaxies(galactic_neighborhood, settings);

        Ok(GeneratedUniverse {
            universe,
            galactic_neighborhood,
            galaxies,
            diagnostics,
        })
    }
}

/// Generates a list of [Galaxy] in the given **galactic_neighborhood** using the given **seed** and **settings**.
fn generate_galaxies(
    galactic_neighborhood: GalacticNeighborhood,
    settings: GenerationSettings,
) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];
    let to_generate: u16;
    match galactic_neighborhood.density {
        GalacticNeighborhoodDensity::Void(g, m) | GalacticNeighborhoodDensity::Group(g, m) => {
            to_generate = (g as u16) + m
        }
        GalacticNeighborhoodDensity::Cluster(d, g, m) => to_generate = (d as u16) + (g as u16) + m,
    }
    for i in 0..to_generate {
        galaxies.push(Galaxy::generate(galactic_neighborhood, i, &settings));
    }
    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::galaxy::types::GalaxySettings;
    use std::rc::Rc;

    fn fixed_cluster_settings() -> GenerationSettings {
        GenerationSettings {
            seed: Rc::from("fixed-cluster"),
            galaxy: GalaxySettings {
                fixed_neighborhood: Some(GalacticNeighborhoodDensity::Cluster(1, 2, 5)),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn generate_a_fixed_cluster_with_proper_dominant_and_major_flags() {
        let generated = Generator::generate(fixed_cluster_settings())
            .expect("The fixed cluster settings should be valid.");
        let galaxies = &generated.galaxies;

        // The galaxy count is dominant + major + minor (see generate_galaxies).
        assert_eq!(
            galaxies.len(),
            8,
            "Cluster(1, 2, 5) should yield 1 dominant + 2 major + 5 minor = 8 galaxies."
        );

        // Flag semantics (see is_galaxy_dominant / is_galaxy_major in
        // galaxy/generator.rs): flags are assigned by galaxy index. Index 0 is
        // the dominant galaxy; is_major covers index < dominant + major, so the
        // dominant galaxy is ALSO major — 3 majors in total, 2 of which are
        // major-but-not-dominant.
        assert_eq!(
            galaxies.iter().filter(|g| g.is_dominant).count(),
            1,
            "Cluster(1, _, _) should have exactly one dominant galaxy."
        );
        assert_eq!(
            galaxies.iter().filter(|g| g.is_major).count(),
            3,
            "is_major includes the dominant galaxy: 1 dominant + 2 major."
        );
        assert_eq!(
            galaxies
                .iter()
                .filter(|g| g.is_major && !g.is_dominant)
                .count(),
            2,
            "Exactly 2 major-but-not-dominant galaxies."
        );

        // Boundary check per index: 0 = dominant, 1..=2 = major, 3..=7 = minor.
        assert!(galaxies[0].is_dominant && galaxies[0].is_major);
        for galaxy in &galaxies[1..3] {
            assert!(!galaxy.is_dominant && galaxy.is_major);
        }
        for galaxy in &galaxies[3..] {
            assert!(!galaxy.is_dominant && !galaxy.is_major);
        }
    }

    #[test]
    fn generate_a_fixed_cluster_deterministically() {
        let first = Generator::generate(fixed_cluster_settings())
            .expect("The fixed cluster settings should be valid.");
        let second = Generator::generate(fixed_cluster_settings())
            .expect("The fixed cluster settings should be valid.");

        assert_eq!(first.galaxies.len(), second.galaxies.len());
        for (a, b) in first.galaxies.iter().zip(second.galaxies.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.category, b.category);
            assert_eq!(a.sub_category, b.sub_category);
            assert_eq!(a.is_dominant, b.is_dominant);
            assert_eq!(a.is_major, b.is_major);
            assert_eq!(a.special_traits, b.special_traits);
        }
    }
}
