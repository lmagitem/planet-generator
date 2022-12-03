use crate::prelude::*;

impl GalacticMapDivision {
    pub fn generate(
        coord: SpaceCoordinates,
        index: SpaceCoordinates,
        level: u8,
        parent_division_level: &GalacticMapDivisionLevel,
        galaxy: &Galaxy,
    ) -> Self {
        // TODO: Get beginning, center and end coordinates in parsecs of the division
        // TODO: For each of these points, checks in which region it is
        // TODO: If all points are in the same region, select that region, otherwise set region to multiple
        let region = GalacticRegion::Multiple;

        Self {
            name: String::from("GalaxyDivision"),
            region,
            level,
            x: (index.x % parent_division_level.x_subdivisions as i64) as u8,
            y: (index.y % parent_division_level.y_subdivisions as i64) as u8,
            z: (index.z % parent_division_level.z_subdivisions as i64) as u8,
            index,
        }
    }
}

/// returns true if the given point is within the area of the sphere whose center and radius are given in parameters.
fn is_within_sphere(point: SpaceCoordinates, center: SpaceCoordinates, radius: i64) -> bool {
    i64::pow(point.x - center.x, 2)
        + i64::pow(point.y - center.y, 2)
        + i64::pow(point.z - center.z, 2)
        <= i64::pow(radius, 2)
}
