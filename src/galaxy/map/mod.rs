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
    /// TODO: Add a boolean "populate" parameter that generates life in the hex if needed.
    pub fn get_hex(&mut self, coord: SpaceCoordinates) -> Result<GalacticHex, String> {
        if !self.are_coord_valid(coord) {
            return Err(String::from("Invalid coordinates."));
        }

        let starting_point = self.get_galactic_start();
        let abs_coord = coord.abs(starting_point);
        let hex_size = self
            .division_levels
            .iter()
            .find(|l| l.level == 0)
            .expect("The division levels should be set")
            .as_coord();
        let index = abs_coord / hex_size;
        let possible_hex = self.hexes.iter().find(|hex| hex.index == index);

        if let Some(hex) = possible_hex {
            Ok(hex.clone())
        } else {
            let new_hex = GalacticHex::generate(coord, index, self);
            self.hexes.push(new_hex.clone());
            Ok(new_hex)
        }
    }

    /// Returns the [GalacticMapDivision] at the level and coordinates given in parameters. 0 being the hex level and 9 being the highest
    /// possible division level.
    pub fn get_division_at_level(
        &mut self,
        coord: SpaceCoordinates,
        level: u8,
    ) -> Result<GalacticMapDivision, String> {
        if !self.are_coord_valid(coord) {
            return Err(String::from("Invalid coordinates."));
        }
        if level <= 0 || level >= 10 {
            return Err(String::from(
                "Level must be higher than 0 and less than 10.",
            ));
        }

        let divisions = self
            .get_divisions_for_coord(coord)
            .expect("Divisions should have been found or generated.");
        let division = divisions
            .iter()
            .find(|div| div.level == level)
            .expect("A division should have been found or generated.");
        Ok(division.clone())
    }

    /// Returns the list of [GalacticMapDivision] the given coordinates are a part of.
    pub fn get_divisions_for_coord(
        &mut self,
        coord: SpaceCoordinates,
    ) -> Result<Vec<GalacticMapDivision>, String> {
        if !self.are_coord_valid(coord) {
            return Err(String::from("Invalid coordinates."));
        }

        let mut result = Vec::new();
        let starting_point = self.get_galactic_start();
        let abs_coord = coord.abs(starting_point);

        let mut index = abs_coord;
        for i in 0..=9 {
            index = calculate_next_index(self, i, index);
            let possible_division = self
                .divisions
                .iter()
                .filter(|div| div.level == i)
                .find(|div| div.index == index);

            if let Some(division) = possible_division {
                result.push(division.clone())
            } else {
                let new_division = GalacticMapDivision::generate(
                    coord,
                    index,
                    i,
                    &self
                        .division_levels
                        .iter()
                        .find(|lvl| lvl.level == i + 1)
                        .unwrap_or(&GalacticMapDivisionLevel::new(10, 255, 255, 255)),
                    self,
                );
                self.divisions.push(new_division.clone());
                result.push(new_division)
            }
        }

        Ok(result)
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

/// Calculates the index of a [GalacticMapDivision] when iterating over division levels to determine the index of higher levels.
fn calculate_next_index(
    galaxy: &mut Galaxy,
    level: u8,
    index: SpaceCoordinates,
) -> SpaceCoordinates {
    let size = galaxy
        .division_levels
        .iter()
        .find(|l| l.level == level)
        .expect("The division levels should be set.")
        .as_coord();
    index / size
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
    fn hexes_and_divs_are_at_expected_coordinates() {
        let settings = GenerationSettings {
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
                flat_map: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut galaxy = Galaxy {
            seed: String::from("default"),
            settings: settings.clone(),
            neighborhood: GalacticNeighborhood {
                ..Default::default()
            },
            index: 0,
            name: String::from(OUR_GALAXYS_NAME),
            age: OUR_GALAXYS_AGE,
            is_dominant: false,
            is_major: true,
            category: GalaxyCategory::Irregular(100, 5, 1),
            sub_category: OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![NO_SPECIAL_TRAIT],
            division_levels: GalacticMapDivisionLevel::generate_division_levels(&settings),
            divisions: vec![],
            hexes: vec![],
        };
        let first_hex = galaxy
            .get_hex(SpaceCoordinates::new(-49, -2, 0))
            .expect("Should return a hex.");
        assert!(first_hex.index == SpaceCoordinates::new(0, 0, 0));
        let first_hex_but_second_parsec = galaxy
            .get_hex(SpaceCoordinates::new(-48, -2, 0))
            .expect("Should return a hex.");
        assert!(first_hex_but_second_parsec.index == SpaceCoordinates::new(0, 0, 0));
        let another_hex = galaxy
            .get_hex(SpaceCoordinates::new(-10, -2, 0))
            .expect("Should return a hex.");
        assert!(another_hex.index == SpaceCoordinates::new(9, 0, 0));
        let another_hex_with_different_y = galaxy
            .get_hex(SpaceCoordinates::new(-10, 0, 0))
            .expect("Should return a hex.");
        assert!(another_hex_with_different_y.index == SpaceCoordinates::new(9, 1, 0));
        let last_hex = galaxy
            .get_hex(SpaceCoordinates::new(50, 2, 0))
            .expect("Should return a hex.");
        assert!(last_hex.index == SpaceCoordinates::new(24, 2, 0));

        let first_div = galaxy
            .get_division_at_level(SpaceCoordinates::new(-49, -2, 0), 1)
            .expect("Should return a div.");
        assert!(first_div.index == SpaceCoordinates::new(0, 0, 0));
        let first_div_but_fourth_parsec = galaxy
            .get_division_at_level(SpaceCoordinates::new(-46, -2, 0), 1)
            .expect("Should return a div.");
        assert!(first_div_but_fourth_parsec.index == SpaceCoordinates::new(0, 0, 0));
        let another_div = galaxy
            .get_division_at_level(SpaceCoordinates::new(-10, -2, 0), 1)
            .expect("Should return a div.");
        assert!(another_div.index == SpaceCoordinates::new(4, 0, 0));
        let another_div_with_different_y = galaxy
            .get_division_at_level(SpaceCoordinates::new(-10, 0, 0), 1)
            .expect("Should return a div.");
        assert!(another_div_with_different_y.index == SpaceCoordinates::new(4, 0, 0));
        let last_div = galaxy
            .get_division_at_level(SpaceCoordinates::new(50, 2, 0), 1)
            .expect("Should return a div.");
        assert!(last_div.index == SpaceCoordinates::new(12, 1, 0));

        let first_second_level_div = galaxy
            .get_division_at_level(SpaceCoordinates::new(-49, -2, 0), 2)
            .expect("Should return a div.");
        assert!(first_second_level_div.index == SpaceCoordinates::new(0, 0, 0));
        let first_second_level_div_but_fourth_parsec = galaxy
            .get_division_at_level(SpaceCoordinates::new(-46, -2, 0), 2)
            .expect("Should return a div.");
        assert!(first_second_level_div_but_fourth_parsec.index == SpaceCoordinates::new(0, 0, 0));
        let another_second_level_div = galaxy
            .get_division_at_level(SpaceCoordinates::new(-10, -2, 0), 2)
            .expect("Should return a div.");
        assert!(another_second_level_div.index == SpaceCoordinates::new(1, 0, 0));
        let another_second_level_div_with_different_y = galaxy
            .get_division_at_level(SpaceCoordinates::new(-10, 0, 0), 2)
            .expect("Should return a div.");
        assert!(another_second_level_div_with_different_y.index == SpaceCoordinates::new(1, 0, 0));
        let last_second_level_div = galaxy
            .get_division_at_level(SpaceCoordinates::new(50, 2, 0), 2)
            .expect("Should return a div.");
        assert!(last_second_level_div.index == SpaceCoordinates::new(4, 0, 0));
    }
}
