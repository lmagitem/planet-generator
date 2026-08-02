use crate::diagnostics::types::{Diagnostics, SettingsError};
use crate::galaxy::map::types::SectorSettings;
use crate::galaxy::neighborhood::GalacticNeighborhood;
use crate::galaxy::types::GalaxySettings;
use crate::galaxy::Galaxy;
use crate::system::celestial_body::types::CelestialBodySettings;
use crate::system::star::types::StarSettings;
use crate::system::types::SystemSettings;
use crate::universe::types::UniverseSettings;
use crate::universe::Universe;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::Display;
use std::rc::Rc;

/// A list of settings used to configure generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct GenerationSettings {
    /// The seed to use to generate everything.
    #[default("default")]
    pub seed: Rc<str>,
    /// A list of settings used to configure the [Universe] generation.
    pub universe: UniverseSettings,
    /// A list of settings used to configure the [Galaxy] generation.
    pub galaxy: GalaxySettings,
    /// A list of settings used to configure the [GalacticMapDivisionLevel], [GalacticMapDivision]s and [GalacticHex]es generation.
    pub sector: SectorSettings,
    /// A list of settings used to configure the [StarSystem] generation.
    pub system: SystemSettings,
    /// A list of settings used to configure the [Star] generation.
    pub star: StarSettings,
    /// A list of settings used to configure the [CelestialBody] generation.
    pub celestial_body: CelestialBodySettings,
}

impl GenerationSettings {
    /// Checks every settings area against the ranges documented on its fields and collects ALL
    /// violations found, so callers can fix their settings in one pass.
    ///
    /// What is checked:
    /// - `sector.hex_size`: every component must be between 1 and 255 inclusive.
    /// - `sector.level_N_size` (N in 1..=9): components must be between 4 and 62 inclusive.
    ///   Only x and y are checked when `sector.flat_map` is set, because the z component is
    ///   then forced to 1 downstream (see `GalacticMapDivisionLevel::generate_division_levels`).
    /// - `universe.fixed_age`: must be higher or equal to 0.4 and lower than 100000.
    /// - `star.fixed_age` / `star.fixed_mass`: must not be negative.
    ///
    /// # Errors
    /// Returns the list of every violation found, if any.
    pub fn validate(&self) -> Result<(), Vec<SettingsError>> {
        let mut errors = Vec::new();

        let hex_size = self.sector.hex_size;
        if hex_size.0 == 0 || hex_size.1 == 0 || hex_size.2 == 0 {
            errors.push(SettingsError::InvalidHexSize(
                hex_size.0, hex_size.1, hex_size.2,
            ));
        }

        let level_sizes = [
            (1, self.sector.level_1_size),
            (2, self.sector.level_2_size),
            (3, self.sector.level_3_size),
            (4, self.sector.level_4_size),
            (5, self.sector.level_5_size),
            (6, self.sector.level_6_size),
            (7, self.sector.level_7_size),
            (8, self.sector.level_8_size),
            (9, self.sector.level_9_size),
        ];
        for (level, size) in level_sizes {
            let x_invalid = size.0 < 4 || size.0 > 62;
            let y_invalid = size.1 < 4 || size.1 > 62;
            // With flat_map the z component is forced to 1 downstream, so an
            // out-of-range z is harmless then and only invalid without flat_map.
            let z_invalid = !self.sector.flat_map && (size.2 < 4 || size.2 > 62);
            if x_invalid || y_invalid || z_invalid {
                errors.push(SettingsError::InvalidLevelSize { level, size });
            }
        }

        if let Some(age) = self.universe.fixed_age {
            // NaN is checked explicitly: it fails the range check anyway, but
            // spelling it out guards the intent — a NaN age used to slip past
            // generation and silently fall back to our universe's age.
            if age.is_nan() || !(0.4..100_000.0).contains(&age) {
                errors.push(SettingsError::InvalidUniverseAge(age));
            }
        }
        if let Some(age) = self.star.fixed_age {
            if age < 0.0 {
                errors.push(SettingsError::InvalidStarAge(age));
            }
        }
        if let Some(mass) = self.star.fixed_mass {
            if mass < 0.0 {
                errors.push(SettingsError::InvalidStarMass(mass));
            }
        }

        let body = &self.celestial_body;
        if body.do_not_generate_gaseous
            && body.do_not_generate_icy
            && body.do_not_generate_rocky
            && body.do_not_generate_metallic
        {
            errors.push(SettingsError::AllBodyTypesDisabled);
        }

        let star = &self.star;
        let total_star_chance = star.brown_dwarf_gen_chance
            + star.red_dwarf_one_gen_chance
            + star.red_dwarf_two_gen_chance
            + star.red_dwarf_three_gen_chance
            + star.red_dwarf_four_gen_chance
            + star.red_dwarf_five_gen_chance
            + star.orange_dwarf_gen_chance
            + star.yellow_dwarf_gen_chance
            + star.white_star_gen_chance
            + star.blue_star_one_gen_chance
            + star.blue_star_two_gen_chance
            + star.blue_star_three_gen_chance
            + star.violet_star_one_gen_chance
            + star.violet_star_two_gen_chance
            + star.violet_star_three_gen_chance;
        if total_star_chance == 0 {
            errors.push(SettingsError::AllStarChancesZero);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Data object filled with the results of a generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GeneratedUniverse {
    /// The generated [Universe].
    pub universe: Universe,
    /// The generated [GalacticNeighborhood].
    pub galactic_neighborhood: GalacticNeighborhood,
    /// A list of generated [Galaxy] objects.
    pub galaxies: Vec<Galaxy>,
    /// The non-fatal issues raised while generating the universe and its neighborhood.
    /// Each galaxy's own diagnostics live on its [Galaxy] instead of being aggregated
    /// here: they are per-galaxy concerns (e.g. that galaxy's SVG map layers), and
    /// duplicating them at the universe level would double the storage for no benefit.
    /// Transient: deliberately not serialized.
    #[serde(skip)]
    pub diagnostics: Diagnostics,
}

impl Display for GeneratedUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GeneratedUniverse {{\n\tuniverse: {},\n\tgalactic_neighborhood: {},\n\tgalaxies:\n\t\t{}\n}}",
            self.universe,
            self.galactic_neighborhood,
            self.galaxies
                .iter()
                .map(|g| format!("{}", g))
                .collect::<Vec<String>>()
                .join(",\n\t\t")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_are_valid() {
        assert_eq!(GenerationSettings::default().validate(), Ok(()));
    }

    #[test]
    fn catches_a_zero_hex_size_component() {
        for hex_size in [(0, 1, 1), (1, 0, 1), (1, 1, 0)] {
            let settings = GenerationSettings {
                sector: SectorSettings {
                    hex_size,
                    ..Default::default()
                },
                ..Default::default()
            };
            assert_eq!(
                settings.validate(),
                Err(vec![SettingsError::InvalidHexSize(
                    hex_size.0, hex_size.1, hex_size.2
                )])
            );
        }
    }

    #[test]
    fn catches_out_of_range_level_size_components() {
        for size in [(3, 10, 10), (63, 10, 10), (10, 3, 10), (10, 63, 10)] {
            let settings = GenerationSettings {
                sector: SectorSettings {
                    level_1_size: size,
                    ..Default::default()
                },
                ..Default::default()
            };
            assert_eq!(
                settings.validate(),
                Err(vec![SettingsError::InvalidLevelSize { level: 1, size }])
            );
        }
    }

    #[test]
    fn ignores_level_size_z_component_on_a_flat_map_but_not_otherwise() {
        // With flat_map the z component is forced to 1 downstream, so an
        // out-of-range z is not a violation.
        let flat = GenerationSettings {
            sector: SectorSettings {
                level_1_size: (10, 10, 1),
                flat_map: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(flat.validate(), Ok(()));

        let not_flat = GenerationSettings {
            sector: SectorSettings {
                level_1_size: (10, 10, 1),
                flat_map: false,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            not_flat.validate(),
            Err(vec![SettingsError::InvalidLevelSize {
                level: 1,
                size: (10, 10, 1),
            }])
        );
    }

    #[test]
    fn catches_out_of_range_universe_fixed_age() {
        for age in [0.39, 100_000.0, -1.0] {
            let settings = GenerationSettings {
                universe: UniverseSettings {
                    fixed_age: Some(age),
                    ..Default::default()
                },
                ..Default::default()
            };
            assert_eq!(
                settings.validate(),
                Err(vec![SettingsError::InvalidUniverseAge(age)])
            );
        }
        for age in [0.4, 13.8, 99999.9] {
            let settings = GenerationSettings {
                universe: UniverseSettings {
                    fixed_age: Some(age),
                    ..Default::default()
                },
                ..Default::default()
            };
            assert_eq!(settings.validate(), Ok(()));
        }
    }

    #[test]
    fn catches_a_nan_universe_fixed_age() {
        let settings = GenerationSettings {
            universe: UniverseSettings {
                fixed_age: Some(f32::NAN),
                ..Default::default()
            },
            ..Default::default()
        };
        // NaN never equals itself, so match on the shape instead of assert_eq.
        assert!(matches!(
            settings.validate(),
            Err(errors) if errors.len() == 1
                && matches!(errors[0], SettingsError::InvalidUniverseAge(age) if age.is_nan())
        ));
    }

    #[test]
    fn catches_negative_star_fixed_age_and_mass() {
        let settings = GenerationSettings {
            star: StarSettings {
                fixed_age: Some(-0.5),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            settings.validate(),
            Err(vec![SettingsError::InvalidStarAge(-0.5)])
        );

        let settings = GenerationSettings {
            star: StarSettings {
                fixed_mass: Some(-1.0),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            settings.validate(),
            Err(vec![SettingsError::InvalidStarMass(-1.0)])
        );
    }

    #[test]
    fn reports_every_violation_at_once() {
        let settings = GenerationSettings {
            universe: UniverseSettings {
                fixed_age: Some(0.1),
                ..Default::default()
            },
            sector: SectorSettings {
                hex_size: (1, 0, 1),
                level_3_size: (2, 2, 2),
                ..Default::default()
            },
            star: StarSettings {
                fixed_age: Some(-0.5),
                fixed_mass: Some(-1.0),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            settings.validate(),
            Err(vec![
                SettingsError::InvalidHexSize(1, 0, 1),
                SettingsError::InvalidLevelSize {
                    level: 3,
                    size: (2, 2, 2),
                },
                SettingsError::InvalidUniverseAge(0.1),
                SettingsError::InvalidStarAge(-0.5),
                SettingsError::InvalidStarMass(-1.0),
            ])
        );
    }

    #[test]
    fn catches_all_body_types_disabled() {
        let settings = GenerationSettings {
            celestial_body: CelestialBodySettings {
                do_not_generate_gaseous: true,
                do_not_generate_icy: true,
                do_not_generate_rocky: true,
                do_not_generate_metallic: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            settings.validate(),
            Err(vec![SettingsError::AllBodyTypesDisabled])
        );
    }

    #[test]
    fn catches_all_star_chances_at_zero() {
        let settings = GenerationSettings {
            star: StarSettings {
                brown_dwarf_gen_chance: 0,
                red_dwarf_one_gen_chance: 0,
                red_dwarf_two_gen_chance: 0,
                red_dwarf_three_gen_chance: 0,
                red_dwarf_four_gen_chance: 0,
                red_dwarf_five_gen_chance: 0,
                orange_dwarf_gen_chance: 0,
                yellow_dwarf_gen_chance: 0,
                white_star_gen_chance: 0,
                blue_star_one_gen_chance: 0,
                blue_star_two_gen_chance: 0,
                blue_star_three_gen_chance: 0,
                violet_star_one_gen_chance: 0,
                violet_star_two_gen_chance: 0,
                violet_star_three_gen_chance: 0,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(
            settings.validate(),
            Err(vec![SettingsError::AllStarChancesZero])
        );
    }
}
