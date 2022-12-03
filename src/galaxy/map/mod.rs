use crate::prelude::*;
#[path = "../constants.rs"]
mod constants;
pub mod division;
pub mod division_level;
pub mod hex;
pub mod types;
use constants::*;

impl Galaxy {
    /// Returns the [GalacticHex] whose coordinates have been given in parameters.
    /// TODO: Add a boolean "populate" parameter that genetates life in the hex if needed.
    pub fn get_hex(&self, coord: SpaceCoordinates) -> Result<GalacticHex, String> {
        if !self.are_coord_valid(coord) {
            return Err(String::from("Invalid coordinates."));
        }

        let starting_point = self.get_galactic_start();
        let abs_coord = coord.abs(starting_point);
        let hex_size = self.settings.sector.hex_size;
        let hex_size_as_coord =
            SpaceCoordinates::new(hex_size.0 as i64, hex_size.1 as i64, hex_size.2 as i64);
        let abs_hex_coordinates = abs_coord / hex_size_as_coord;
        let rel_hex_coordinates = abs_hex_coordinates.rel(starting_point);
        let possible_hex = self
            .hexes
            .iter()
            .find(|hex| hex.first_vertex == rel_hex_coordinates);

        if let Some(hex) = possible_hex {
            Ok(hex.clone())
        } else {
            // TODO: Generate the hex and return it
            Ok(GalacticHex::default())
        }
    }

    /// TODO: Returns the list of [GalacticMapDivision] the given coordinates are a part of.
    pub fn get_division_at_level(
        &self,
        coord: SpaceCoordinates,
        level: u8,
    ) -> Result<GalacticMapDivision, String> {
        if !self.are_coord_valid(coord) {
            return Err(String::from("Invalid coordinates."));
        }

        if let Some(division) = self
            .get_divisions(coord)?
            .iter()
            .find(|div| div.level == level)
        {
            Ok(division.clone())
        } else {
            // TODO: Generate the division and return it
            Ok(GalacticMapDivision::default())
        }
    }

    /// TODO: Returns the list of [GalacticMapDivision] the given coordinates are a part of.
    pub fn get_divisions(
        &self,
        coord: SpaceCoordinates,
    ) -> Result<Vec<GalacticMapDivision>, String> {
        if !self.are_coord_valid(coord) {
            return Err(String::from("Invalid coordinates."));
        }

        Ok(vec![])
    }

    /// Returns the starting point of a galactic 3D map.
    pub fn get_galactic_start(&self) -> SpaceCoordinates {
        return match self.category {
            GalaxyCategory::Intergalactic(l, w, h)
            | GalaxyCategory::Irregular(l, w, h)
            | GalaxyCategory::Intracluster(l, w, h) => {
                let x: i64 = if l % 2 == 0 {
                    1 - (l as i64 / 2)
                } else {
                    -(l as i64 / 2)
                };
                let y: i64 = if w % 2 == 0 {
                    1 - (w as i64 / 2)
                } else {
                    -(w as i64 / 2)
                };
                let z: i64 = if h % 2 == 0 {
                    1 - (h as i64 / 2)
                } else {
                    -(h as i64 / 2)
                };
                SpaceCoordinates::new(x, y, z)
            }
            GalaxyCategory::Spiral(r, d) | GalaxyCategory::Lenticular(r, d) => {
                let x: i64 = 1 - (r as i64);
                let z: i64 = if d % 2 == 0 {
                    1 - (d as i64 / 2)
                } else {
                    -(d as i64 / 2)
                };
                SpaceCoordinates::new(x, x, z)
            }
            GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => {
                let x: i64 = 1 - (r as i64);
                SpaceCoordinates::new(x, x, x)
            }
        };
    }

    /// Returns the center point of a galactic 3D map.
    pub fn get_galactic_center(&self) -> SpaceCoordinates {
        SpaceCoordinates::new(0, 0, 0)
    }

    /// Returns the point of a galactic 3D map that is the farthest from (0, 0, 0).
    pub fn get_galactic_end(&self) -> SpaceCoordinates {
        return match self.category {
            GalaxyCategory::Intergalactic(l, w, h)
            | GalaxyCategory::Irregular(l, w, h)
            | GalaxyCategory::Intracluster(l, w, h) => {
                SpaceCoordinates::new(l as i64 / 2, w as i64 / 2, h as i64 / 2)
            }
            GalaxyCategory::Spiral(r, d) | GalaxyCategory::Lenticular(r, d) => {
                SpaceCoordinates::new(r as i64, r as i64, d as i64 / 2)
            }
            GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => {
                SpaceCoordinates::new(r as i64, r as i64, r as i64)
            }
        };
    }

    /// Returns the size of the [Galaxy] in parsecs on the (x, y, z) axis.
    pub fn get_galaxy_size(&self) -> SpaceCoordinates {
        return match self.category {
            GalaxyCategory::Intergalactic(l, w, h)
            | GalaxyCategory::Irregular(l, w, h)
            | GalaxyCategory::Intracluster(l, w, h) => {
                SpaceCoordinates::new(l as i64, w as i64, h as i64)
            }
            GalaxyCategory::Spiral(r, d) | GalaxyCategory::Lenticular(r, d) => {
                SpaceCoordinates::new(r as i64 * 2, r as i64 * 2, d as i64)
            }
            GalaxyCategory::Elliptical(r) | GalaxyCategory::DominantElliptical(r) => {
                SpaceCoordinates::new(r as i64 * 2, r as i64 * 2, r as i64 * 2)
            }
        };
    }

    /// Checks whether the given coordinates are within the bounds of the galaxy.
    fn are_coord_valid(&self, coord: SpaceCoordinates) -> bool {
        let start = self.get_galactic_start();
        let end = self.get_galactic_end();
        coord.x >= start.x
            && coord.y >= start.y
            && coord.z >= start.z
            && coord.x <= end.x
            && coord.y <= end.y
            && coord.z <= end.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_start_center_and_end_points() {
        let galaxy = Galaxy {
            seed: String::from("default"),
            settings: GenerationSettings {
                ..Default::default()
            },
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            index: 0,
            name: String::from(OUR_GALAXYS_NAME),
            age: OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: GalaxyCategory::Irregular(5, 4, 1),
            sub_category: OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![NO_SPECIAL_TRAIT],
            division_levels: vec![],
            divisions: vec![],
            hexes: vec![],
        };
        let start = galaxy.get_galactic_start();
        let center = galaxy.get_galactic_center();
        let end = galaxy.get_galactic_end();
        assert_eq!(start, SpaceCoordinates::new(-2, -1, 0));
        assert_eq!(center, SpaceCoordinates::new(0, 0, 0));
        assert_eq!(end, SpaceCoordinates::new(2, 2, 0));
    }

    #[test]
    fn checks_coordinates_validity_properly() {
        let galaxy = Galaxy {
            seed: String::from("default"),
            settings: GenerationSettings {
                ..Default::default()
            },
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            index: 0,
            name: String::from(OUR_GALAXYS_NAME),
            age: OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: GalaxyCategory::Irregular(5, 4, 1),
            sub_category: OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![NO_SPECIAL_TRAIT],
            division_levels: vec![],
            divisions: vec![],
            hexes: vec![],
        };
        let start = galaxy.get_galactic_start();
        let center = galaxy.get_galactic_center();
        let end = galaxy.get_galactic_end();
        assert_eq!(start, SpaceCoordinates::new(-2, -1, 0));
        assert_eq!(center, SpaceCoordinates::new(0, 0, 0));
        assert_eq!(end, SpaceCoordinates::new(2, 2, 0));

        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(-2, -1, 0)));
        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(-1, -1, 0)));
        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(0, -1, 0)));
        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(1, -1, 0)));
        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(2, -1, 0)));
        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(2, 0, 0)));
        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(2, 1, 0)));
        assert!(galaxy.are_coord_valid(SpaceCoordinates::new(2, 2, 0)));

        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(-3, -2, -1)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(-3, -2, 0)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(-3, -1, 0)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(-2, -2, 0)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(-2, -1, -1)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(-2, -1, 1)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(2, 3, 0)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(3, 2, 0)));
        assert!(!galaxy.are_coord_valid(SpaceCoordinates::new(2, 2, 60)));
    }

    #[test]
    fn get_hex_returns_hexes_at_expected_coordinates() {
        let galaxy = Galaxy {
            seed: String::from("default"),
            settings: GenerationSettings {
                universe: UniverseSettings {
                    ..Default::default()
                },
                galaxy: GalaxySettings {
                    ..Default::default()
                },
                sector: SectorSettings {
                    hex_size: (2, 2, 2),
                    level_1_size: (2, 2, 2),
                    level_2_size: (3, 3, 3),
                    level_3_size: (4, 4, 4),
                    level_4_size: (62, 62, 62),
                    level_5_size: (62, 62, 62),
                    level_6_size: (62, 62, 62),
                    level_7_size: (62, 62, 62),
                    level_8_size: (62, 62, 62),
                    level_9_size: (62, 62, 62),
                    flat_map: true,
                },
            },
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            index: 0,
            name: String::from(OUR_GALAXYS_NAME),
            age: OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: GalaxyCategory::Irregular(100, 1, 1),
            sub_category: OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![NO_SPECIAL_TRAIT],
            division_levels: vec![],
            divisions: vec![],
            hexes: vec![],
        };
        let last_hex = galaxy
            .get_hex(SpaceCoordinates::new(50, 0, 0))
            .expect("Should return a hex");
        assert!(last_hex.first_vertex == SpaceCoordinates::new(49, 0, 0));
        assert!(last_hex.last_vertex == SpaceCoordinates::new(50, 1, 1));
    }
}
