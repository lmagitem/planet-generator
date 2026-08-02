#[cfg(test)]
use crate::diagnostics::types::Diagnostics;
use crate::diagnostics::types::{GenerationError, GenerationWarning};
use crate::galaxy::map::division::GalacticMapDivision;
use crate::galaxy::map::division_level::GalacticMapDivisionLevel;
use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::hex::GalacticHex;
use crate::galaxy::map::region_map::types::GalacticRegionMap;
#[cfg(test)]
use crate::galaxy::map::types::GalacticRegion;
#[cfg(test)]
use crate::galaxy::map::types::SectorSettings;
#[cfg(test)]
use crate::galaxy::neighborhood::GalacticNeighborhood;
use crate::galaxy::types::GalaxyCategory;
#[cfg(test)]
use crate::galaxy::types::GalaxySettings;
use crate::galaxy::Galaxy;
#[cfg(test)]
use crate::generator::types::GenerationSettings;
#[cfg(test)]
use crate::universe::types::UniverseSettings;
use log::error;
#[path = "../constants.rs"]
mod constants;
pub mod division;
pub mod division_level;
pub mod hex;
pub mod region_map;
pub mod serde_helpers;
pub mod types;

impl Galaxy {
    /// Returns the [GalacticHex] whose coordinates have been given in parameters.
    /// TODO: Add a boolean "populate" parameter that generates life in the hex if needed.
    ///
    /// # Errors
    /// Returns [`GenerationError::InvalidCoordinates`] if the coordinates are outside the bounds of
    /// the galaxy, or propagates any error from the hex's generation.
    pub fn get_hex(&mut self, coord: SpaceCoordinates) -> Result<&GalacticHex, GenerationError> {
        if !self.are_coord_valid(coord) {
            return Err(GenerationError::InvalidCoordinates(coord));
        }
        if self.region_map.is_none() && !self.galactic_map_layers.is_empty() {
            self.rebuild_region_map();
        }

        let starting_point = self.get_galactic_start();
        let abs_coord = coord.abs(starting_point);
        let hex_size = self
            .division_levels
            .iter()
            .find(|l| l.level == 0)
            .ok_or(GenerationError::InternalInvariant(
                "division level 0 (hex size) must be set",
            ))?
            .as_coord();
        let index = abs_coord / hex_size;

        if !self.hexes.contains_key(&index) {
            let new_hex = GalacticHex::generate(coord, index, self)?;
            self.hexes.insert(index, new_hex);
        }
        self.hexes
            .get(&index)
            .ok_or(GenerationError::InternalInvariant(
                "hex must exist right after being found or generated",
            ))
    }

    /// Returns the [GalacticMapDivision] at the level and coordinates given in parameters. 0 being the hex level and 9 being the highest
    /// possible division level.
    ///
    /// # Errors
    /// Returns [`GenerationError::InvalidCoordinates`] if the coordinates are outside the bounds of
    /// the galaxy, or [`GenerationError::InvalidDivisionLevel`] if the level is not in 1..=9.
    pub fn get_division_at_level(
        &mut self,
        coord: SpaceCoordinates,
        level: u8,
    ) -> Result<&GalacticMapDivision, GenerationError> {
        if !self.are_coord_valid(coord) {
            return Err(GenerationError::InvalidCoordinates(coord));
        }
        if level <= 0 || level >= 10 {
            return Err(GenerationError::InvalidDivisionLevel(level));
        }

        let divisions = self.get_divisions_for_coord(coord)?;
        // `level` was validated above and `get_divisions_for_coord` returns one
        // division per level 0..=9, so this lookup cannot actually fail.
        let division = divisions
            .into_iter()
            .find(|div| div.level == level)
            .ok_or(GenerationError::InvalidDivisionLevel(level))?;
        Ok(division)
    }

    /// Returns the list of [GalacticMapDivision] the given coordinates are a part of.
    ///
    /// # Errors
    /// Returns [`GenerationError::InvalidCoordinates`] if the coordinates are outside the bounds of
    /// the galaxy.
    pub fn get_divisions_for_coord(
        &mut self,
        coord: SpaceCoordinates,
    ) -> Result<Vec<&GalacticMapDivision>, GenerationError> {
        if !self.are_coord_valid(coord) {
            return Err(GenerationError::InvalidCoordinates(coord));
        }
        if self.region_map.is_none() && !self.galactic_map_layers.is_empty() {
            self.rebuild_region_map();
        }

        let starting_point = self.get_galactic_start();
        let abs_coord = coord.abs(starting_point);

        let mut index = abs_coord;
        for i in 0..=9 {
            index = calculate_next_index(&self.division_levels, i, index);
            if !self.divisions.contains_key(&(i, index)) {
                let new_division = GalacticMapDivision::generate(
                    index,
                    i,
                    &self
                        .division_levels
                        .iter()
                        .find(|lvl| lvl.level == i + 1)
                        .unwrap_or(&GalacticMapDivisionLevel::new(10, 255, 255, 255)),
                    self,
                );
                self.divisions.insert((i, index), new_division);
            }
        }

        let mut result = Vec::new();
        let mut index = abs_coord;
        for i in 0..=9 {
            index = calculate_next_index(&self.division_levels, i, index);
            result.push(self.divisions.get(&(i, index)).ok_or(
                GenerationError::InternalInvariant(
                    "division must exist right after being found or generated",
                ),
            )?);
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

    /// Rebuilds the region map from `galactic_map_layers` and clears already-generated
    /// divisions and hexes (their cached regions would otherwise go stale). On parse
    /// failure, falls back to the category's default map; if that also fails, clears
    /// `galactic_map_layers` (no map will be used) and logs an error.
    ///
    /// Note that this also runs automatically on the first map query of a galaxy whose
    /// region map is missing (e.g. freshly deserialized, since the cache is not
    /// serialized): the deserialized division and hex caches are cleared and their
    /// content is regenerated on demand. Regeneration is deterministic, but any
    /// consumer-side mutation of cached systems is discarded.
    pub fn rebuild_region_map(&mut self) {
        let start = self.get_galactic_start();
        let end = self.get_galactic_end();
        let region_map = GalacticRegionMap::build_with_diagnostics(
            &self.galactic_map_layers,
            start,
            end,
            &mut self.diagnostics,
        );
        self.region_map = region_map;
        if self.region_map.is_none() {
            self.galactic_map_layers = constants::default_layers_for(self.category);
            let region_map = GalacticRegionMap::build_with_diagnostics(
                &self.galactic_map_layers,
                start,
                end,
                &mut self.diagnostics,
            );
            self.region_map = region_map;
            if self.region_map.is_none() {
                error!(
                    "Failed to build any region map for galaxy {}; falling back to no map.",
                    self.name
                );
                self.galactic_map_layers.clear();
            } else {
                self.diagnostics
                    .push_warning(GenerationWarning::RegionMapRebuiltFromDefault);
            }
        }
        self.divisions.clear();
        self.hexes.clear();
    }
}

/// Calculates the index of a [GalacticMapDivision] when iterating over division levels to determine the index of higher levels.
fn calculate_next_index(
    division_levels: &[GalacticMapDivisionLevel],
    level: u8,
    index: SpaceCoordinates,
) -> SpaceCoordinates {
    // Unreachable fallback (division levels always cover 0..=9): the same dummy
    // level used by `get_divisions_for_coord` when a level is missing.
    let size = division_levels
        .iter()
        .find(|l| l.level == level)
        .map_or_else(
            || GalacticMapDivisionLevel::new(10, 255, 255, 255).as_coord(),
            |l| l.as_coord(),
        );
    index / size
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[test]
    fn returns_proper_start_center_and_end_points() {
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
            category: GalaxyCategory::Irregular(5, 4, 1),
            sub_category: constants::OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![constants::NO_SPECIAL_TRAIT],
            division_levels: vec![],
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers: vec![],
            region_map: None,
            diagnostics: Diagnostics::default(),
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
            category: GalaxyCategory::Irregular(5, 4, 1),
            sub_category: constants::OUR_GALAXYS_SUB_CATEGORY,
            special_traits: vec![constants::NO_SPECIAL_TRAIT],
            division_levels: vec![],
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers: vec![],
            region_map: None,
            diagnostics: Diagnostics::default(),
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
    fn hexes_and_divs_are_at_expected_coordinates() -> Result<(), GenerationError> {
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
                flat_map: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut galaxy = Galaxy {
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
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers: vec![],
            region_map: None,
            diagnostics: Diagnostics::default(),
        };
        let first_hex = galaxy.get_hex(SpaceCoordinates::new(-49, -2, 0))?;
        assert_eq!(first_hex.index, SpaceCoordinates::new(0, 0, 0));
        let first_hex_but_second_parsec = galaxy.get_hex(SpaceCoordinates::new(-48, -2, 0))?;
        assert_eq!(
            first_hex_but_second_parsec.index,
            SpaceCoordinates::new(0, 0, 0)
        );
        let another_hex = galaxy.get_hex(SpaceCoordinates::new(-10, -2, 0))?;
        assert_eq!(another_hex.index, SpaceCoordinates::new(9, 0, 0));
        let another_hex_with_different_y = galaxy.get_hex(SpaceCoordinates::new(-10, 0, 0))?;
        assert_eq!(
            another_hex_with_different_y.index,
            SpaceCoordinates::new(9, 1, 0)
        );
        let last_hex = galaxy.get_hex(SpaceCoordinates::new(50, 2, 0))?;
        assert_eq!(last_hex.index, SpaceCoordinates::new(24, 2, 0));

        let first_div = galaxy.get_division_at_level(SpaceCoordinates::new(-49, -2, 0), 1)?;
        assert_eq!(first_div.index, SpaceCoordinates::new(0, 0, 0));
        let first_div_but_fourth_parsec =
            galaxy.get_division_at_level(SpaceCoordinates::new(-46, -2, 0), 1)?;
        assert_eq!(
            first_div_but_fourth_parsec.index,
            SpaceCoordinates::new(0, 0, 0)
        );
        let another_div = galaxy.get_division_at_level(SpaceCoordinates::new(-10, -2, 0), 1)?;
        assert_eq!(another_div.index, SpaceCoordinates::new(4, 0, 0));
        let another_div_with_different_y =
            galaxy.get_division_at_level(SpaceCoordinates::new(-10, 0, 0), 1)?;
        assert_eq!(
            another_div_with_different_y.index,
            SpaceCoordinates::new(4, 0, 0)
        );
        let last_div = galaxy.get_division_at_level(SpaceCoordinates::new(50, 2, 0), 1)?;
        assert_eq!(last_div.index, SpaceCoordinates::new(12, 1, 0));

        let first_second_level_div =
            galaxy.get_division_at_level(SpaceCoordinates::new(-49, -2, 0), 2)?;
        assert_eq!(first_second_level_div.index, SpaceCoordinates::new(0, 0, 0));
        let first_second_level_div_but_fourth_parsec =
            galaxy.get_division_at_level(SpaceCoordinates::new(-46, -2, 0), 2)?;
        assert_eq!(
            first_second_level_div_but_fourth_parsec.index,
            SpaceCoordinates::new(0, 0, 0)
        );
        let another_second_level_div =
            galaxy.get_division_at_level(SpaceCoordinates::new(-10, -2, 0), 2)?;
        assert_eq!(
            another_second_level_div.index,
            SpaceCoordinates::new(1, 0, 0)
        );
        let another_second_level_div_with_different_y =
            galaxy.get_division_at_level(SpaceCoordinates::new(-10, 0, 0), 2)?;
        assert_eq!(
            another_second_level_div_with_different_y.index,
            SpaceCoordinates::new(1, 0, 0)
        );
        let last_second_level_div =
            galaxy.get_division_at_level(SpaceCoordinates::new(50, 2, 0), 2)?;
        assert_eq!(last_second_level_div.index, SpaceCoordinates::new(4, 0, 0));
        Ok(())
    }

    #[test]
    fn rebuild_clears_caches() -> Result<(), GenerationError> {
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
                flat_map: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut galaxy = Galaxy {
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
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers: vec![],
            region_map: None,
            diagnostics: Diagnostics::default(),
        };
        galaxy.get_hex(SpaceCoordinates::new(-10, -2, 0))?;
        assert!(
            !galaxy.divisions.is_empty() && !galaxy.hexes.is_empty(),
            "Generating a hex should have populated the division and hex caches."
        );

        galaxy.rebuild_region_map();

        assert!(
            galaxy.region_map.is_some(),
            "Rebuilding from empty layers should fall back to the category's default map."
        );
        assert!(
            galaxy.divisions.is_empty() && galaxy.hexes.is_empty(),
            "Rebuilding the region map should clear the stale division and hex caches."
        );
        Ok(())
    }

    #[test]
    fn caches_survive_a_serde_round_trip() -> Result<(), GenerationError> {
        #[derive(Serialize, Deserialize)]
        struct Caches {
            #[serde(with = "crate::galaxy::map::serde_helpers")]
            divisions: BTreeMap<(u8, SpaceCoordinates), GalacticMapDivision>,
            #[serde(with = "crate::galaxy::map::serde_helpers")]
            hexes: BTreeMap<SpaceCoordinates, GalacticHex>,
        }

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
                flat_map: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut galaxy = Galaxy {
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
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers: vec![],
            region_map: None,
            diagnostics: Diagnostics::default(),
        };
        galaxy.get_hex(SpaceCoordinates::new(-10, -2, 0))?;
        galaxy.get_hex(SpaceCoordinates::new(10, 1, 0))?;
        assert!(
            !galaxy.divisions.is_empty() && !galaxy.hexes.is_empty(),
            "The caches should be populated for the round-trip to be meaningful."
        );

        let caches = Caches {
            divisions: galaxy.divisions.clone(),
            hexes: galaxy.hexes.clone(),
        };
        let json = serde_json::to_string(&caches).expect("Should serialize.");
        let deserialized: Caches = serde_json::from_str(&json).expect("Should deserialize.");

        assert_eq!(deserialized.divisions, galaxy.divisions);
        assert_eq!(deserialized.hexes, galaxy.hexes);
        assert_eq!(
            serde_json::to_string(&deserialized).expect("Should serialize."),
            json,
            "Serializing the round-tripped caches should give the same JSON."
        );
        Ok(())
    }

    #[test]
    fn map_queries_heal_a_missing_region_map() -> Result<(), GenerationError> {
        // Simulates a freshly deserialized galaxy: layers are set but the rasterized cache is gone.
        let settings = GenerationSettings {
            seed: "default".into(),
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
        let full_arm_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="#0000FF"/></svg>"##;
        let mut galaxy = Galaxy {
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
            divisions: BTreeMap::new(),
            hexes: BTreeMap::new(),
            galactic_map_layers: vec![full_arm_svg.to_string()],
            region_map: None,
            diagnostics: Diagnostics::default(),
        };

        galaxy.get_hex(SpaceCoordinates::new(-10, -2, 0))?;
        assert!(
            galaxy.region_map.is_some(),
            "A map query should rebuild a missing region map from the galaxy's layers."
        );

        let division = galaxy.get_division_at_level(SpaceCoordinates::new(-10, -2, 0), 1)?;
        assert_eq!(division.region, GalacticRegion::Arm);
        Ok(())
    }
}
