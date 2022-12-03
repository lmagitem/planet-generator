use crate::prelude::*;

impl GalacticMapDivision {
    pub fn generate(
        coord: SpaceCoordinates,
        index: SpaceCoordinates,
        level: u8,
        parent_division_level: &GalacticMapDivisionLevel,
        galaxy: &Galaxy,
    ) -> Self {
        Self {
            name: String::from("GalaxyDivision"),
            region: GalacticRegion::Core,
            level,
            x: (index.x % parent_division_level.x_subdivisions as i64) as u8,
            y: (index.y % parent_division_level.y_subdivisions as i64) as u8,
            z: (index.z % parent_division_level.z_subdivisions as i64) as u8,
            index,
        }
    }
}
