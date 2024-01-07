use crate::prelude::OrbitalPoint;

/// Sorts the Vec<OrbitalPoint> in place based on the average_distance in their own_orbit
pub fn sort_orbital_points_by_average_distance(orbital_points: &mut Vec<OrbitalPoint>) {
    orbital_points.sort_by(|a, b| {
        let distance_a = a
            .own_orbit
            .as_ref()
            .map_or(f64::INFINITY, |orbit| orbit.average_distance);
        let distance_b = b
            .own_orbit
            .as_ref()
            .map_or(f64::INFINITY, |orbit| orbit.average_distance);
        distance_a
            .partial_cmp(&distance_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}
