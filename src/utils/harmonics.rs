use crate::prelude::{AstronomicalObject, CelestialBodySize, OrbitalPoint};

pub struct OrbitalHarmonicsUtils {}
impl OrbitalHarmonicsUtils {
    /// Calculates the gravitational harmonics for a set of orbital periods.
    ///
    /// This function computes the gravitational harmonic values for each orbit in a system,
    /// taking into account the resonance effects between each pair of orbits. The harmonic
    /// value is a measure of the gravitational influence and resonance between orbits.
    ///
    /// # Arguments
    ///
    /// * `orbital_periods_and_size_multipliers`: A slice of tuples where each tuple contains
    ///   the orbital period of a body (in arbitrary time units) and its size multiplier, which
    ///   represents the relative size or mass of the body.
    /// * `tolerance`: The tolerance level for considering two periods to be in resonance.
    ///   A smaller tolerance means only close matches to simple ratios are considered resonant.
    ///
    /// # Returns
    ///
    /// A vector of `u32`, where each element corresponds to the gravitational harmonic value
    /// of the corresponding orbit in the input slice.
    pub fn calculate_gravitational_harmonics(
        orbital_periods_and_size_multipliers: &[(f64, f64)],
        tolerance: f64,
    ) -> Vec<u32> {
        orbital_periods_and_size_multipliers
            .iter()
            .enumerate()
            .map(|(i, &orbit_period_and_multiplier)| {
                let current_period = orbit_period_and_multiplier.0;
                let forward_resonance = orbital_periods_and_size_multipliers
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .fold(0.0, |acc, (j, &next_orbit_period_and_multiplier)| {
                        let (next_period, multiplier) = next_orbit_period_and_multiplier;
                        acc + Self::evaluate_resonance(next_period, current_period, tolerance)
                            * calculate_modifier(j - i)
                            * multiplier
                    });

                let backward_resonance = orbital_periods_and_size_multipliers
                    .iter()
                    .enumerate()
                    .take(i)
                    .fold(0.0, |acc, (j, &previous_orbit_period_and_multiplier)| {
                        let (previous_period, multiplier) = previous_orbit_period_and_multiplier;
                        acc + Self::evaluate_resonance(previous_period, current_period, tolerance)
                            * 0.25
                            * calculate_modifier(i - j)
                            * multiplier
                    });

                f64::round(forward_resonance + backward_resonance) as u32
            })
            .collect()
    }

    /// Evaluates the resonance between two orbital periods.
    ///
    /// This function checks if two given orbital periods are in resonance within a specified
    /// tolerance. Resonance is determined based on simple integer ratios up to 7:7.
    ///
    /// # Arguments
    ///
    /// * `orbital_period1`: The orbital period of the first body.
    /// * `orbital_period2`: The orbital period of the second body.
    /// * `tolerance`: The tolerance for considering two periods to be in resonance.
    ///
    /// # Returns
    ///
    /// A floating-point value representing the strength of the resonance, or zero if no
    /// significant resonance is found.
    fn evaluate_resonance(orbital_period1: f64, orbital_period2: f64, tolerance: f64) -> f64 {
        let ratio = orbital_period1 / orbital_period2;

        let potential_ratios = (1..=7).flat_map(|x| (1..=7).map(move |y| (x, y)));

        for (num, denom) in potential_ratios {
            let target_ratio = num as f64 / denom as f64;

            if (ratio - target_ratio).abs() <= tolerance {
                return (12.0 - num as f64 - denom as f64) * 0.5;
            }
        }

        0.0
    }

    pub fn prepare_harmonics_array(
        orbital_points: &[OrbitalPoint],
        are_moons: bool,
    ) -> Vec<(f64, f64)> {
        orbital_points
            .iter()
            .filter_map(|orbital_point| {
                if let Some(own_orbit) = &orbital_point.own_orbit {
                    let distance_multiplier = if are_moons { 1.0 } else { 0.3 };
                    let size_multiplier = match orbital_point.object {
                        AstronomicalObject::TelluricBody(ref body)
                        | AstronomicalObject::IcyBody(ref body)
                        | AstronomicalObject::GaseousBody(ref body) => match body.size {
                            CelestialBodySize::Puny => 0.2,
                            CelestialBodySize::Tiny => 0.5,
                            CelestialBodySize::Small => 1.0,
                            CelestialBodySize::Standard => 1.5,
                            CelestialBodySize::Large => 3.0,
                            CelestialBodySize::Giant => 4.0,
                            CelestialBodySize::Supergiant => 5.0,
                            CelestialBodySize::Hypergiant => 6.0,
                        },
                        _ => 0.0,
                    };

                    Some((
                        own_orbit.orbital_period as f64,
                        distance_multiplier * size_multiplier,
                    ))
                } else {
                    Some((0.0, 0.0))
                }
            })
            .collect()
    }
}

fn calculate_modifier(distance: usize) -> f64 {
    let mut modifier = 1.0;
    for i in 0..(distance - 1) {
        modifier = modifier * (1.0 / 3.0) * 2.0;
    }
    modifier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_gravitational_harmonics() {
        let orbits = vec![(1.0, 1.0), (2.0, 1.0), (4.0, 1.0)]; // Simple 1:2:4 resonance pattern
        let tolerance = 0.03;
        let expected_harmonics = vec![7, 6, 2];

        let calculated_harmonics =
            OrbitalHarmonicsUtils::calculate_gravitational_harmonics(&orbits, tolerance);

        for (calculated, expected) in calculated_harmonics.iter().zip(expected_harmonics.iter()) {
            assert!(
                (*calculated == *expected),
                "Calculated: {}, Expected: {}",
                calculated,
                expected
            );
        }
    }

    #[test]
    fn test_complex_gravitational_harmonics() {
        let orbits = vec![(1.0, 1.0), (2.0, 1.0), (5.6895, 1.0), (6.0, 1.0)]; // Simple 1:2:4 resonance pattern
        let tolerance = 0.03;
        let expected_harmonics = vec![6, 4, 2, 1];

        let calculated_harmonics =
            OrbitalHarmonicsUtils::calculate_gravitational_harmonics(&orbits, tolerance);

        for (calculated, expected) in calculated_harmonics.iter().zip(expected_harmonics.iter()) {
            assert!(
                (*calculated == *expected),
                "Calculated: {}, Expected: {}",
                calculated,
                expected
            );
        }
    }

    #[test]
    fn test_no_resonance_gravitational_harmonics() {
        let orbits: Vec<(f64, f64)> = vec![(1.0, 1.0), (10.0, 1.0), (100.0, 1.0), (1000.0, 1.0)];
        let tolerance = 0.03;
        let expected_harmonics = vec![0, 0, 0, 0];

        let calculated_harmonics =
            OrbitalHarmonicsUtils::calculate_gravitational_harmonics(&orbits, tolerance);

        for (calculated, expected) in calculated_harmonics.iter().zip(expected_harmonics.iter()) {
            assert!(
                (*calculated == *expected),
                "Calculated: {}, Expected: {}",
                calculated,
                expected
            );
        }
    }

    #[test]
    fn test_exact_resonance() {
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.0, 1.0, 0.03),
            4.5
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(3.0, 1.0, 0.03),
            4.0
        );
    }

    #[test]
    fn test_near_resonance() {
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.03, 1.0, 0.03),
            4.5
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.98, 1.0, 0.03),
            4.0
        );
    }

    #[test]
    fn test_simple_resonance() {
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(1.0, 2.0, 0.03),
            4.5
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(3.0, 2.0, 0.03),
            3.5
        );
    }

    #[test]
    fn test_complex_resonance() {
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.0, 5.0, 0.03),
            2.5
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(3.0, 5.0, 0.03),
            2.0
        );
    }

    #[test]
    fn test_near_complex_resonance() {
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.02, 5.0, 0.03),
            2.5
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.98, 5.0, 0.03),
            2.0
        );
    }

    #[test]
    fn test_no_resonance() {
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(0.563, 5.0, 0.03),
            0.0
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(3.55, 5.0, 0.03),
            0.0
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(7.66, 1.0, 0.03),
            0.0
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(1.8, 1.0, 0.03),
            0.0
        );
    }

    #[test]
    fn test_outside_tolerance() {
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.1, 1.0, 0.03),
            0.0
        );
        assert_eq!(
            OrbitalHarmonicsUtils::evaluate_resonance(2.92, 1.0, 0.03),
            0.0
        );
    }
}
