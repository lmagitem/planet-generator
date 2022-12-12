use crate::prelude::*;

impl GalacticMapDivision {
    pub fn generate(
        coord: SpaceCoordinates,
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
    let mut region = GalacticRegion::Multiple;
    let start = division.get_top_left_up(galaxy);
    let size = division.get_size(galaxy);
    let half_size = size / SpaceCoordinates::new(2, 2, 2);

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
                region = generate_region(SpaceCoordinates::new(x, y, z), galaxy);
                // TODO: for each of these points, checks in which region it is
                // TODO: If all points are in the same region, select that region, otherwise set region to multiple
            }
        }
    }
    region
}

// TODO
fn generate_region(coord: SpaceCoordinates, galaxy: &Galaxy) -> GalacticRegion {
    GalacticRegion::Multiple
}

///
fn is_within_sphere_in_non_equal_planes(
    coord: SpaceCoordinates,
    sizes: SpaceCoordinates,
    galaxy: &Galaxy,
) {
    // Find biggest size
    // Produit en croix
    // ? = coord.? x biggest_size / size
    // plus qu'à utiliser ? comme **point** dans is_within_sphere, et biggest_size comme radius
}

/// returns true if the given point is within the area of the sphere whose center and radius are given in parameters.
fn is_within_sphere(point: SpaceCoordinates, center: SpaceCoordinates, radius: i64) -> bool {
    i64::pow(point.x - center.x, 2)
        + i64::pow(point.y - center.y, 2)
        + i64::pow(point.z - center.z, 2)
        <= i64::pow(radius, 2)
}
