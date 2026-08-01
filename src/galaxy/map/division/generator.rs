use crate::internal::*;
use crate::prelude::*;
use std::collections::HashMap;

impl GalacticMapDivision {
    pub fn generate(
        index: SpaceCoordinates,
        level: u8,
        parent_division_level: &GalacticMapDivisionLevel,
        galaxy: &Galaxy,
    ) -> Self {
        let mut division = Self {
            name: "GalaxyDivision".into(),
            region: GalacticRegion::Multiple,
            level,
            x: (index.x % parent_division_level.x_subdivisions as i64) as u8,
            y: (index.y % parent_division_level.y_subdivisions as i64) as u8,
            z: (index.z % parent_division_level.z_subdivisions as i64) as u8,
            index,
            size: SpaceCoordinates::new(-1, -1, -1),
        };
        division.region = get_region(&mut division, galaxy);
        division
    }
}

fn get_region(division: &mut GalacticMapDivision, galaxy: &Galaxy) -> GalacticRegion {
    let start = division.get_top_left_up(galaxy);
    let size = division.get_size(galaxy);
    let half_size = size / SpaceCoordinates::new(2, 2, 2);
    let mut region_count = HashMap::new();

    for xi in 0..3 {
        let x = if xi == 0 {
            start.x
        } else if xi == 1 {
            start.x + half_size.x
        } else {
            start.x + size.x
        };
        for yi in 0..3 {
            let y = if yi == 0 {
                start.y
            } else if yi == 1 {
                start.y + half_size.y
            } else {
                start.y + size.y
            };
            for zi in 0..3 {
                let z = if zi == 0 {
                    start.z
                } else if zi == 1 {
                    start.z + half_size.z
                } else {
                    start.z + size.z
                };

                // Finds which region this point belongs to and remembers it
                let point_region = generate_region(SpaceCoordinates::new(x, y, z), galaxy);
                *region_count.entry(point_region).or_insert(0) += 1;
            }
        }
    }

    // If there was only one region in the whole division, set that division's region to it, otherwise use Multiple.
    if region_count.len() == 1 {
        let (region, _) = region_count.into_iter().next().unwrap();
        region
    } else {
        GalacticRegion::Multiple
    }
}

/// Returns the proper region for a given coordinate, using the galaxy's region map.
fn generate_region(coord: SpaceCoordinates, galaxy: &Galaxy) -> GalacticRegion {
    galaxy
        .region_map
        .as_ref()
        .map_or(GalacticRegion::Multiple, |m| m.region_at(coord))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_region_without_map_returns_multiple() {
        // Hand-built galaxies with no region map keep the status-quo catch-all region.
        let galaxy = Galaxy {
            category: GalaxyCategory::Irregular(100, 5, 1),
            ..Default::default()
        };
        assert_eq!(
            generate_region(SpaceCoordinates::new(0, 0, 0), &galaxy),
            GalacticRegion::Multiple
        );
    }
}
