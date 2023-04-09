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
            name: String::from("GalaxyDivision"),
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
            half_size.x
        } else {
            size.x
        };
        for yi in 0..3 {
            let y = if yi == 0 {
                start.y
            } else if yi == 1 {
                half_size.y
            } else {
                size.y
            };
            for zi in 0..3 {
                let z = if zi == 0 {
                    start.z
                } else if zi == 1 {
                    half_size.z
                } else {
                    size.z
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

/// Returns the proper region for a given coordinate.
/// TODO: Generate regions properly
fn generate_region(coord: SpaceCoordinates, galaxy: &Galaxy) -> GalacticRegion {
    let spheroid_sizes = match galaxy.category {
        GalaxyCategory::Spiral(r, _) | GalaxyCategory::Lenticular(r, _) => {
            SpaceCoordinates::new(r as i64 * 2, r as i64 * 2, (r as i64 * 2) / 10)
        }
        GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => {
            SpaceCoordinates::new(r as i64 * 2, r as i64 * 2, r as i64 * 2)
        }
        _ => return GalacticRegion::Multiple,
    };

    let abs_coord = coord.abs(galaxy.get_galactic_start());
    if is_within_sphere_in_non_equal_planes(abs_coord, spheroid_sizes, galaxy) {
        GalacticRegion::Ellipse
    } else {
        GalacticRegion::Void
    }
}

/// Returns true it the given point is within the area the given galaxy (that must be a spheroid).
fn is_within_sphere_in_non_equal_planes(
    coord: SpaceCoordinates,
    sizes: SpaceCoordinates,
    galaxy: &Galaxy,
) -> bool {
    let biggest_size = sizes.x.max(sizes.y).max(sizes.z);
    let scaled_point = SpaceCoordinates {
        x: coord.x * biggest_size / sizes.x,
        y: coord.y * biggest_size / sizes.y,
        z: coord.z * biggest_size / sizes.z,
    };
    let center = galaxy.get_galactic_center();

    is_within_sphere(scaled_point, center, biggest_size)
}

/// Returns true if the given point is within the area of the sphere whose center and radius are given in parameters.
fn is_within_sphere(point: SpaceCoordinates, center: SpaceCoordinates, radius: i64) -> bool {
    i64::pow(point.x - center.x, 2)
        + i64::pow(point.y - center.y, 2)
        + i64::pow(point.z - center.z, 2)
        <= i64::pow(radius, 2)
}
