use crate::internal::*;
use crate::prelude::*;

impl GalacticMapDivisionLevel {
    /// Returns a list of [GalacticMapDivisionLevel], one for each level, using the given settings.
    pub fn generate_division_levels(settings: &GenerationSettings) -> Vec<Self> {
        let mut division_levels = Vec::new();
        let sector_settings = settings.sector;
        let flat_map = sector_settings.flat_map;
        division_levels.push(Self {
            level: 0,
            x_subdivisions: sector_settings.hex_size.0,
            y_subdivisions: sector_settings.hex_size.1,
            z_subdivisions: sector_settings.hex_size.2,
        });
        division_levels.push(Self {
            level: 1,
            x_subdivisions: sector_settings.level_1_size.0,
            y_subdivisions: sector_settings.level_1_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_1_size.2
            },
        });
        division_levels.push(Self {
            level: 2,
            x_subdivisions: sector_settings.level_2_size.0,
            y_subdivisions: sector_settings.level_2_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_2_size.2
            },
        });
        division_levels.push(Self {
            level: 3,
            x_subdivisions: sector_settings.level_3_size.0,
            y_subdivisions: sector_settings.level_3_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_3_size.2
            },
        });
        division_levels.push(Self {
            level: 4,
            x_subdivisions: sector_settings.level_4_size.0,
            y_subdivisions: sector_settings.level_4_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_4_size.2
            },
        });
        division_levels.push(Self {
            level: 5,
            x_subdivisions: sector_settings.level_5_size.0,
            y_subdivisions: sector_settings.level_5_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_5_size.2
            },
        });
        division_levels.push(Self {
            level: 6,
            x_subdivisions: sector_settings.level_6_size.0,
            y_subdivisions: sector_settings.level_6_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_6_size.2
            },
        });
        division_levels.push(Self {
            level: 7,
            x_subdivisions: sector_settings.level_7_size.0,
            y_subdivisions: sector_settings.level_7_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_7_size.2
            },
        });
        division_levels.push(Self {
            level: 8,
            x_subdivisions: sector_settings.level_8_size.0,
            y_subdivisions: sector_settings.level_8_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_8_size.2
            },
        });
        division_levels.push(Self {
            level: 9,
            x_subdivisions: sector_settings.level_9_size.0,
            y_subdivisions: sector_settings.level_9_size.1,
            z_subdivisions: if flat_map {
                1
            } else {
                sector_settings.level_9_size.2
            },
        });
        division_levels
    }
}
