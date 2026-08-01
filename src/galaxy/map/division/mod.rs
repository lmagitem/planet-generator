use crate::internal::*;
use crate::prelude::*;
pub mod generator;

/// Represents a specific part of the [Galaxy].
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub struct GalacticMapDivision {
    /// The denomination by which this particular partition of space is known.
    #[default("default")]
    pub name: Rc<str>,
    /// Which region of space makes the most of the division.
    pub region: GalacticRegion,
    /// The level of division this partition belongs to. See [SpaceDivisionLevel].
    pub level: u8,
    /// Which cell this division occupies on the X axis in its parent's grid.
    pub x: u8,
    /// Which cell this division occupies on the Y axis in its parent's grid.
    pub y: u8,
    /// Which cell this division occupies on the Z axis in its parent's grid.
    pub z: u8,
    /// The index of this division on the x, y and z axis.
    pub index: SpaceCoordinates,
    /// This division's size in parsecs
    size: SpaceCoordinates,
}

impl GalacticMapDivision {
    /// Creates a new [GalacticMapDivision].
    pub fn new(
        name: Rc<str>,
        region: GalacticRegion,
        level: u8,
        x: u8,
        y: u8,
        z: u8,
        index: SpaceCoordinates,
    ) -> Self {
        Self {
            name,
            region,
            level,
            x,
            y,
            z,
            index,
            size: SpaceCoordinates::new(-1, -1, -1),
        }
    }

    /// Returns the top up left coordinates of this division.
    pub fn get_top_left_up(&mut self, galaxy: &Galaxy) -> SpaceCoordinates {
        galaxy.get_galactic_start() + self.index * self.get_size(galaxy)
    }

    /// Returns the center coordinates of this division.
    pub fn get_center(&mut self, galaxy: &Galaxy) -> SpaceCoordinates {
        self.get_top_left_up(galaxy) + self.get_size(galaxy) / SpaceCoordinates::new(2, 2, 2)
    }

    /// Returns the bottom down right coordinates of this division.
    pub fn get_bottom_right_down(&mut self, galaxy: &Galaxy) -> SpaceCoordinates {
        self.get_top_left_up(galaxy) + self.get_size(galaxy)
    }

    /// Returns the full width of this division in parsecs.
    pub fn get_width(&mut self, galaxy: &Galaxy) -> i64 {
        self.get_size(galaxy).x
    }

    /// Returns the full height of this division in parsecs.
    pub fn get_height(&mut self, galaxy: &Galaxy) -> i64 {
        self.get_size(galaxy).y
    }

    /// Returns the full depth of this division in parsecs.
    pub fn get_depth(&mut self, galaxy: &Galaxy) -> i64 {
        self.get_size(galaxy).z
    }

    /// Returns the full size of this division in parsecs.
    pub fn get_size(&mut self, galaxy: &Galaxy) -> SpaceCoordinates {
        if self.size.x < 1 || self.size.y < 1 || self.size.z < 1 {
            let mut size = SpaceCoordinates::new(1, 1, 1);
            galaxy
                .division_levels
                .clone()
                .iter()
                .filter(|l| l.level <= self.level)
                .for_each(|l| {
                    size = size
                        * SpaceCoordinates::new(
                            l.x_subdivisions as i64,
                            l.y_subdivisions as i64,
                            l.z_subdivisions as i64,
                        );
                });
            self.size = size;
        }
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::super::constants;
    use super::*;

    /// Builds a galaxy shaped like the one used in the map tests: an
    /// Irregular(100, 5, 1) whose map starts at (-49, -2, 0) and ends at
    /// (50, 2, 0), with 4*2*4 parsecs hexes and 2*2*2 level 1 subdivisions.
    fn test_galaxy(flat_map: bool) -> Galaxy {
        let settings = GenerationSettings {
            seed: "default".into(),
            universe: UniverseSettings {
                ..Default::default()
            },
            galaxy: GalaxySettings {
                ..Default::default()
            },
            sector: SectorSettings {
                hex_size: (4, 2, 4),
                level_1_size: (2, 2, 2),
                level_2_size: (3, 3, 3),
                level_3_size: (2, 2, 2),
                flat_map,
                ..Default::default()
            },
            ..Default::default()
        };
        Galaxy {
            settings: settings.clone(),
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            index: 0,
            name: constants::OUR_GALAXYS_NAME.into(),
            age: constants::OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: GalaxyCategory::Irregular(100, 5, 1),
            sub_category: constants::OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![constants::NO_SPECIAL_TRAIT],
            division_levels: GalacticMapDivisionLevel::generate_division_levels(&settings),
            divisions: vec![],
            hexes: vec![],
            galactic_map_layers: vec![],
            region_map: None,
        }
    }

    #[test]
    fn computes_level_1_division_corners_and_center_on_a_flat_map() {
        let mut galaxy = test_galaxy(true);
        let mut division = galaxy
            .get_division_at_level(SpaceCoordinates::new(-10, -2, 0), 1)
            .expect("Should return a div.");
        assert_eq!(division.index, SpaceCoordinates::new(4, 0, 0));
        // On a flat map the z axis is not subdivided past the hex level, so
        // this division spans 8*4*4 parsecs.
        assert_eq!(division.get_size(&galaxy), SpaceCoordinates::new(8, 4, 4));
        assert_eq!(
            division.get_top_left_up(&galaxy),
            SpaceCoordinates::new(-17, -2, 0)
        );
        assert_eq!(
            division.get_bottom_right_down(&galaxy),
            SpaceCoordinates::new(-9, 2, 4)
        );
        assert_eq!(
            division.get_center(&galaxy),
            SpaceCoordinates::new(-13, 0, 2)
        );
    }

    #[test]
    fn computes_level_1_division_corners_and_center_without_flat_map() {
        let mut galaxy = test_galaxy(false);
        let mut division = galaxy
            .get_division_at_level(SpaceCoordinates::new(-10, -2, 0), 1)
            .expect("Should return a div.");
        assert_eq!(division.index, SpaceCoordinates::new(4, 0, 0));
        // Without a flat map the z axis is subdivided like the others, so
        // this division spans 8*4*8 parsecs.
        assert_eq!(division.get_size(&galaxy), SpaceCoordinates::new(8, 4, 8));
        assert_eq!(
            division.get_top_left_up(&galaxy),
            SpaceCoordinates::new(-17, -2, 0)
        );
        assert_eq!(
            division.get_bottom_right_down(&galaxy),
            SpaceCoordinates::new(-9, 2, 8)
        );
        assert_eq!(
            division.get_center(&galaxy),
            SpaceCoordinates::new(-13, 0, 4)
        );
    }

    #[test]
    fn computes_last_level_1_division_corner() {
        let mut galaxy = test_galaxy(true);
        let mut division = galaxy
            .get_division_at_level(SpaceCoordinates::new(50, 2, 0), 1)
            .expect("Should return a div.");
        assert_eq!(division.index, SpaceCoordinates::new(12, 1, 0));
        assert_eq!(
            division.get_top_left_up(&galaxy),
            SpaceCoordinates::new(47, 2, 0)
        );
        // The far corner overshoots the galactic end (50, 2, 0): divisions do
        // not tile the galaxy exactly, clamping is handled downstream.
        assert_eq!(
            division.get_bottom_right_down(&galaxy),
            SpaceCoordinates::new(55, 6, 4)
        );
    }

    #[test]
    fn computes_hex_corners_and_center() {
        let mut galaxy = test_galaxy(true);
        let divisions = galaxy
            .get_divisions_for_coord(SpaceCoordinates::new(-10, -2, 0))
            .expect("Should return divisions.");
        let mut hex = divisions
            .into_iter()
            .find(|d| d.level == 0)
            .expect("Should contain a hex-level division.");
        assert_eq!(hex.index, SpaceCoordinates::new(9, 0, 0));
        assert_eq!(hex.get_size(&galaxy), SpaceCoordinates::new(4, 2, 4));
        assert_eq!(
            hex.get_top_left_up(&galaxy),
            SpaceCoordinates::new(-13, -2, 0)
        );
        assert_eq!(
            hex.get_bottom_right_down(&galaxy),
            SpaceCoordinates::new(-9, 0, 4)
        );
        assert_eq!(hex.get_center(&galaxy), SpaceCoordinates::new(-11, -1, 2));
    }

    #[test]
    fn computes_corners_and_center_with_odd_sizes() {
        // Irregular(100, 50, 30) starts at (-49, -24, -14); the 3*5*7 hexes
        // exercise integer division truncation when halving the size.
        let galaxy = Galaxy {
            settings: GenerationSettings {
                seed: "default".into(),
                ..Default::default()
            },
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            index: 0,
            name: constants::OUR_GALAXYS_NAME.into(),
            age: constants::OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: GalaxyCategory::Irregular(100, 50, 30),
            sub_category: constants::OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![constants::NO_SPECIAL_TRAIT],
            division_levels: vec![GalacticMapDivisionLevel::new(0, 3, 5, 7)],
            divisions: vec![],
            hexes: vec![],
            galactic_map_layers: vec![],
            region_map: None,
        };
        let mut division = GalacticMapDivision::new(
            "default".into(),
            GalacticRegion::Multiple,
            0,
            2,
            1,
            3,
            SpaceCoordinates::new(2, 1, 3),
        );
        assert_eq!(division.get_size(&galaxy), SpaceCoordinates::new(3, 5, 7));
        assert_eq!(
            division.get_top_left_up(&galaxy),
            SpaceCoordinates::new(-43, -19, 7)
        );
        assert_eq!(
            division.get_bottom_right_down(&galaxy),
            SpaceCoordinates::new(-40, -14, 14)
        );
        assert_eq!(
            division.get_center(&galaxy),
            SpaceCoordinates::new(-42, -17, 10)
        );
    }
}
