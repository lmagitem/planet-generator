use crate::internal::generator::get_major_moons;
use crate::internal::*;
use crate::prelude::*;
use crate::system::celestial_body::world::utils::get_category_from_temperature;
use crate::system::contents::elements::ALL_ELEMENTS;
use crate::system::contents::elements::MOST_COMMON_ELEMENTS;
use crate::system::contents::zones::get_orbit_with_updated_zone;

impl WorldGenerator {
    pub(crate) fn bundle_world_first_pass(
        star_name: Rc<str>,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        orbit: Orbit,
        orbits: Vec<Orbit>,
        mut size: CelestialBodySize,
        blackbody_temperature: u32,
        density: f32,
        radius: f64,
        mass: f64,
        gravity: f32,
        body_type: TelluricBodyComposition,
        world_type: CelestialBodyWorldType,
        special_traits: Vec<CelestialBodySpecialTrait>,
        moons: &Vec<OrbitalPoint>,
        is_moon: bool,
    ) -> OrbitalPoint {
        OrbitalPoint::new(
            orbital_point_id,
            Some(orbit),
            AstronomicalObject::TelluricBody(CelestialBody {
                stub: true,
                name: format!(
                    "{}{}",
                    star_name,
                    StringUtils::number_to_lowercase_letter(populated_orbit_index as u8 + 1)
                )
                .into(),
                orbit: None,
                orbital_point_id,
                mass,
                radius,
                density,
                gravity,
                blackbody_temperature,
                tidal_heating: 0,
                size,
                details: CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                    body_type,
                    world_type,
                    special_traits,
                    CelestialBodyCoreHeat::ActiveCore,
                    MagneticFieldStrength::None,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    WorldTemperatureCategory::Frozen,
                    WorldClimateType::Dead,
                )),
            }),
            moons
                .clone()
                .iter()
                .filter(|o| o.own_orbit.is_some())
                .map(|o| o.own_orbit.clone().unwrap_or_default())
                .collect::<Vec<Orbit>>(),
        )
    }

    pub(crate) fn generate_world(
        coord: SpaceCoordinates,
        system_traits: &Vec<SystemPeculiarity>,
        system_index: u16,
        star_id: u32,
        star_age: f32,
        star_type: &StarSpectralType,
        star_class: &StarLuminosityClass,
        star_traits: &Vec<StarPeculiarity>,
        distance_from_star: f64,
        populated_orbit_index: u32,
        orbital_point_id: u32,
        own_orbit: Orbit,
        orbits: Vec<Orbit>,
        world: CelestialBody,
        is_moon: bool,
        moons: &Vec<OrbitalPoint>,
        tidal_heating: u32,
        seed: Rc<str>,
        settings: GenerationSettings,
    ) -> OrbitalPoint {
        let CelestialBody {
            name,
            orbit,
            mass,
            radius,
            density,
            gravity,
            mut blackbody_temperature,
            size,
            details,
            ..
        } = world;
        let CelestialBodyDetails::Telluric(TelluricBodyDetails {
            body_type,
            mut world_type,
            mut special_traits,
            ..
        }) = details
        else {
            panic!("At this point, the CelestialBodyDetails should be telluric.")
        };

        let core_heat: CelestialBodyCoreHeat = Self::generate_core_heat(
            coord,
            system_index,
            star_id,
            star_age,
            orbital_point_id,
            &own_orbit,
            size,
            density,
            body_type,
            world_type,
            &special_traits,
            tidal_heating,
            orbit.clone().unwrap_or_default().rotation,
            &settings,
            distance_from_star,
        );

        let magnetic_field = Self::generate_magnetic_field(
            coord,
            system_index,
            star_id,
            orbital_point_id,
            orbit,
            density,
            size,
            core_heat,
            &special_traits,
            &settings,
        );

        let mut hydrosphere = Self::generate_hydrosphere(
            &coord,
            &system_index,
            &star_id,
            &orbital_point_id,
            &settings,
            size,
            world_type,
            &special_traits,
            tidal_heating,
            magnetic_field,
            density,
        );
        // Here, we might have generated a world with "too much water", so it becomes an ocean world
        if hydrosphere >= 90.0 && world_type == CelestialBodyWorldType::Terrestrial {
            world_type = CelestialBodyWorldType::Ocean;
        }

        let volcanism = Self::generate_volcanism(
            &coord,
            &system_index,
            &star_id,
            star_age,
            &orbital_point_id,
            moons,
            tidal_heating,
            &settings,
            gravity,
            size,
            world_type,
            &special_traits,
            core_heat,
        );

        let tectonics = Self::generate_tectonic_activity(
            &coord,
            &system_index,
            &star_id,
            &orbital_point_id,
            moons,
            &settings,
            size,
            world_type,
            &special_traits,
            core_heat,
            hydrosphere,
            volcanism,
        );
        // TODO: Change to GeoActive for those with very high volcanism?

        let mut atmospheric_pressure = generate_atmosphere(
            coord,
            system_index,
            star_id,
            star_age,
            star_type,
            star_class,
            star_traits,
            orbital_point_id,
            &own_orbit,
            size,
            mass,
            body_type,
            world_type,
            volcanism + tectonics,
            hydrosphere,
            is_moon,
            &settings,
        );

        blackbody_temperature = Self::adjust_blackbody_temperature(
            &coord,
            &system_index,
            &star_id,
            &orbital_point_id,
            &settings,
            blackbody_temperature,
            size,
            world_type,
            hydrosphere,
            atmospheric_pressure,
        );
        let temperature_category = get_category_from_temperature(blackbody_temperature);

        let changed_hydrosphere_and_pressure = Self::compute_oceans(
            coord,
            system_index,
            star_id,
            star_traits,
            orbital_point_id,
            &settings,
            world_type,
            blackbody_temperature,
            &mut special_traits,
            hydrosphere,
            atmospheric_pressure,
        );
        hydrosphere = changed_hydrosphere_and_pressure.0;
        atmospheric_pressure = changed_hydrosphere_and_pressure.1;
        // Here hydrosphere might have been reduced to zero if no water can exist on the planet
        if hydrosphere < 87.5 && world_type == CelestialBodyWorldType::Ocean {
            world_type = CelestialBodyWorldType::Terrestrial;
        }

        Self::compute_subsurface_oceans(
            coord,
            system_index,
            star_id,
            star_traits,
            orbital_point_id,
            &settings,
            &mut special_traits,
            world_type,
            core_heat,
            tidal_heating,
            density,
        );

        let hydrosphere_and_cryosphere = Self::generate_cryosphere_and_adjust_hydrosphere(
            coord,
            system_index,
            star_id,
            orbital_point_id,
            &settings,
            world_type,
            special_traits.clone(),
            hydrosphere,
            temperature_category,
        );
        hydrosphere = hydrosphere_and_cryosphere.0;
        let ice_over_water = hydrosphere_and_cryosphere.1;
        let ice_over_land = hydrosphere_and_cryosphere.2;
        let cryosphere = ice_over_water + ice_over_land;
        let land_area_percentage = 100.0 - hydrosphere - cryosphere;

        let humidity = Self::generate_relative_humidity(
            coord,
            system_index,
            star_id,
            orbital_point_id,
            &settings,
            blackbody_temperature,
            &special_traits,
            hydrosphere,
            atmospheric_pressure,
            cryosphere,
            land_area_percentage,
        );

        // TODO: Atmospheric composition
        let present_volatiles: Vec<ChemicalComponent> = Vec::new();
        let atmospheric_composition = {
            let system_wide_elements_abundance: Vec<ChemicalComponent> = {
                let mut rng = SeededDiceRoller::new(
                    &settings.seed,
                    &format!("sys_{}_{}_elem_abnd", coord, system_index),
                );
                let mut elements = Vec::new();
                let mut roll = rng.gen_u8();
                while roll >= 7 {
                    if rng.gen_u8() >= 7 {
                        &elements.push(ALL_ELEMENTS[rng.gen_range(0..ALL_ELEMENTS.len())]);
                    } else {
                        elements.push(
                            MOST_COMMON_ELEMENTS[rng.gen_range(0..MOST_COMMON_ELEMENTS.len())],
                        )
                    }
                    roll = rng.gen_u8();
                }
                elements
            };
            let system_wide_elements_lack: Vec<ChemicalComponent> = {
                let mut rng = SeededDiceRoller::new(
                    &settings.seed,
                    &format!("sys_{}_{}_elem_lack", coord, system_index),
                );
                let mut elements = Vec::new();
                let mut roll = rng.gen_u8();
                while roll >= 7 {
                    if rng.gen_u8() >= 7 {
                        let element = ALL_ELEMENTS[rng.gen_range(0..ALL_ELEMENTS.len())];
                        if !system_wide_elements_abundance.contains(&element) {
                            elements.push(element);
                        }
                    } else {
                        let element =
                            MOST_COMMON_ELEMENTS[rng.gen_range(0..MOST_COMMON_ELEMENTS.len())];
                        if !system_wide_elements_abundance.contains(&element) {
                            elements.push(element);
                        }
                    }
                    roll = rng.gen_u8();
                }
                elements
            };

            let composition: Vec<ChemicalComponent>;
        };

        // TODO: Life
        let mut has_life = true;

        // TODO: Climate
        let climate = {
            let mut climate = None;

            if world_type != CelestialBodyWorldType::Terrestrial
                && world_type != CelestialBodyWorldType::Ocean
            {
                climate = Some(WorldClimateType::Dead)
            }

            if climate.is_none() {
                has_life = SeededDiceRoller::new(
                    &settings.seed,
                    &format!("sys_{}_{}_elem_lack", coord, system_index),
                )
                .gen_bool();
                for special_trait in &special_traits {
                    if let CelestialBodySpecialTrait::TideLocked(TideLockTarget::Orbited) =
                        special_trait
                    {
                        climate = Some(WorldClimateType::Ribbon);
                    }
                }
            }

            if climate.is_none() {
                let humidity_rating = if humidity < 35.0 {
                    0 // Low
                } else if humidity < 75.0 {
                    2 // Moderate
                } else {
                    3 // High
                };
                let hydrosphere_rating = if hydrosphere < 35.0 {
                    0 // Low
                } else if hydrosphere < 50.0 {
                    1 // Moderate-
                } else if hydrosphere < 75.0 {
                    2 // Moderate+
                } else if hydrosphere < 90.0 {
                    3 // High
                } else {
                    4 // Ocean
                };
                let cryosphere_rating = if cryosphere < 10.0 {
                    0 // Low
                } else if cryosphere < 30.0 {
                    2 // Moderate
                } else {
                    3 // High
                };

                if climate.is_none() && humidity_rating <= 0 {
                    // Low humidity
                    if hydrosphere_rating <= 0 && cryosphere_rating <= 1 {
                        climate = Some(WorldClimateType::Desert);
                    }
                }
                if climate.is_none() && humidity_rating <= 2 {
                    // Moderate humidity
                    if climate.is_none() && hydrosphere_rating <= 0 && cryosphere_rating <= 2 {
                        if blackbody_temperature < 291 && has_life {
                            climate = Some(WorldClimateType::Steppe);
                        } else if has_life {
                            climate = Some(WorldClimateType::Savanna);
                        }
                    }
                    if climate.is_none() && hydrosphere_rating <= 2 {
                        if cryosphere_rating <= 2 {
                            climate = Some(WorldClimateType::Terrestrial);
                        } else if blackbody_temperature <= 278 && has_life {
                            climate = Some(WorldClimateType::Taiga);
                        }
                    }
                }
                if climate.is_none() && humidity_rating <= 10 {
                    // High humidity
                    if climate.is_none() && hydrosphere_rating <= 0 {
                        if blackbody_temperature <= 267 {
                            climate = Some(WorldClimateType::Tundra);
                        } else {
                            climate = Some(WorldClimateType::MudBall);
                        }
                    }
                    if climate.is_none() && hydrosphere_rating <= 1 {
                        if cryosphere_rating <= 0 && has_life {
                            climate = Some(WorldClimateType::Jungle);
                        }
                    }
                    if climate.is_none() && hydrosphere_rating <= 2 {
                        if cryosphere_rating <= 0 && has_life {
                            climate = Some(WorldClimateType::Tropical);
                        }
                    }
                    if climate.is_none() && hydrosphere_rating <= 3 {
                        if cryosphere_rating <= 1 && has_life {
                            climate = Some(WorldClimateType::Rainforest);
                        }
                    }
                    if climate.is_none() && blackbody_temperature <= 263 {
                        climate = Some(WorldClimateType::Arctic);
                    }
                    if climate.is_none() && hydrosphere_rating >= 4 {
                        climate = Some(WorldClimateType::Ocean);
                    }
                    if climate.is_none() && hydrosphere_rating <= 1 {
                        climate = Some(WorldClimateType::MudBall);
                    }
                }
            }

            climate.unwrap_or_default()
        };

        OrbitalPoint::new(
            orbital_point_id,
            Some(get_orbit_with_updated_zone(
                own_orbit.clone(),
                blackbody_temperature,
            )),
            AstronomicalObject::TelluricBody(CelestialBody::new(
                None, // No need to fill it inside the object, a call to update_existing_orbits will be made at the end of the generation
                orbital_point_id,
                name,
                mass,
                radius,
                density,
                gravity,
                blackbody_temperature,
                tidal_heating,
                size,
                CelestialBodyDetails::Telluric(TelluricBodyDetails::new(
                    if body_type == TelluricBodyComposition::Icy && blackbody_temperature >= 170 {
                        TelluricBodyComposition::Rocky
                    } else {
                        body_type
                    },
                    world_type,
                    special_traits,
                    core_heat,
                    magnetic_field,
                    atmospheric_pressure,
                    hydrosphere,
                    ice_over_water,
                    land_area_percentage,
                    ice_over_land,
                    volcanism,
                    tectonics,
                    humidity,
                    temperature_category,
                    climate,
                )),
            )),
            orbits.clone(),
        )
    }

    fn generate_relative_humidity(
        coord: SpaceCoordinates,
        system_index: u16,
        star_id: u32,
        orbital_point_id: u32,
        settings: &GenerationSettings,
        blackbody_temperature: u32,
        special_traits: &Vec<CelestialBodySpecialTrait>,
        hydrosphere: f32,
        atmospheric_pressure: f32,
        cryosphere: f32,
        land_area_percentage: f32,
    ) -> f32 {
        if blackbody_temperature > 223 && atmospheric_pressure > 0.01 {
            let mut rng = SeededDiceRoller::new(
                &settings.seed,
                &format!(
                    "sys_{}_{}_str_{}_bdy{}_hmdt",
                    coord, system_index, star_id, orbital_point_id
                ),
            );

            let ocean_humidity = {
                if blackbody_temperature <= 223 {
                    80.0
                } else if blackbody_temperature <= 243 {
                    75.0
                } else if blackbody_temperature <= 273 {
                    80.0
                } else if blackbody_temperature <= 303 {
                    75.0
                } else if blackbody_temperature <= 313 {
                    70.0
                } else if blackbody_temperature <= 323 {
                    65.0
                } else if blackbody_temperature <= 333 {
                    60.0
                } else if blackbody_temperature <= 343 {
                    55.0
                } else if blackbody_temperature <= 363 {
                    45.0
                } else {
                    35.0
                }
            };
            let ice_humidity = {
                if blackbody_temperature <= 163 {
                    100.0
                } else if blackbody_temperature <= 173 {
                    95.0
                } else if blackbody_temperature <= 183 {
                    90.0
                } else if blackbody_temperature <= 193 {
                    85.0
                } else if blackbody_temperature <= 203 {
                    80.0
                } else if blackbody_temperature <= 213 {
                    85.0
                } else if blackbody_temperature <= 223 {
                    82.5
                } else if blackbody_temperature <= 253 {
                    80.0
                } else if blackbody_temperature <= 263 {
                    77.5
                } else if blackbody_temperature <= 273 {
                    72.5
                } else if blackbody_temperature <= 283 {
                    67.5
                } else if blackbody_temperature <= 293 {
                    62.5
                } else if blackbody_temperature <= 303 {
                    57.5
                } else if blackbody_temperature <= 313 {
                    52.5
                } else if blackbody_temperature <= 323 {
                    47.5
                } else if blackbody_temperature <= 333 {
                    42.5
                } else if blackbody_temperature <= 343 {
                    37.5
                } else {
                    32.5
                }
            };

            let mut is_there_water = false;
            let mut is_there_something_else_than_water = false;
            for special_trait in special_traits {
                if let CelestialBodySpecialTrait::Oceans(peculiar_component)
                | CelestialBodySpecialTrait::Lakes(peculiar_component) = special_trait
                {
                    if *peculiar_component == ChemicalComponent::Water {
                        is_there_water = true;
                    } else {
                        is_there_something_else_than_water = true;
                    }
                }
            }

            let land_humidity = if hydrosphere >= 0.01 && is_there_water {
                (((rng.roll(1, 1000, 0) as f32 / 100.0) + hydrosphere) / 2.0)
                    .min(100.0)
                    .max(0.0)
            } else {
                (rng.roll(1, 15000, -10000) as f32 / 10000.0)
                    .min(10.0)
                    .max(0.0)
            };

            if is_there_water || cryosphere <= 0.01 {
                // The sum of relative humidity everywhere
                ((hydrosphere / 100.0) * ocean_humidity)
                    + ((cryosphere / 100.0) * ice_humidity)
                    + (land_area_percentage / 100.0) * land_humidity
            } else if is_there_something_else_than_water || cryosphere <= 0.01 {
                let ocean_water_percentage = rng.roll(1, 1000, -500) as f32 / 100.0;
                let ice_water_percentage = rng.roll(1, 1000, 0) as f32 / 100.0;

                // The sum of relative humidity everywhere, taking into account that oceans aren't primarily made of water, and ice might not be either
                ((ocean_water_percentage / 100.0) * ice_humidity)
                    + ((ice_water_percentage / 100.0) * ice_humidity)
                    + (land_area_percentage / 100.0) * land_humidity
            } else {
                let ice_water_percentage = rng.roll(1, 1000, 0) as f32 / 100.0;

                // The sum of relative humidity everywhere, taking into account that ices aren't primarily made of water
                ((ice_water_percentage / 100.0) * ice_humidity)
                    + (land_area_percentage / 100.0) * 0.0
            }
        } else {
            -1.0
        }
    }

    fn generate_cryosphere_and_adjust_hydrosphere(
        coord: SpaceCoordinates,
        system_index: u16,
        star_id: u32,
        orbital_point_id: u32,
        settings: &GenerationSettings,
        world_type: CelestialBodyWorldType,
        special_traits: Vec<CelestialBodySpecialTrait>,
        hydrosphere: f32,
        temperature_category: WorldTemperatureCategory,
    ) -> (f32, f32, f32) {
        let mut cryosphere = 0.0;
        let mut ice_over_water = 0.0;
        let mut ice_over_land = 0.0;
        let mut hydrosphere = hydrosphere;
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_cryo",
                coord, system_index, star_id, orbital_point_id
            ),
        );

        let to_roll = match world_type {
            CelestialBodyWorldType::Greenhouse
            | CelestialBodyWorldType::Chthonian
            | CelestialBodyWorldType::VolatilesGiant
            | CelestialBodyWorldType::ProtoWorld => (0, 0, 0, 0),
            CelestialBodyWorldType::Ice | CelestialBodyWorldType::Hadean => {
                (10000, 7000, 6500, 10000)
            }
            CelestialBodyWorldType::DirtySnowball => (10000, 3000, 2000, 10000),
            CelestialBodyWorldType::GeoActive => match temperature_category {
                WorldTemperatureCategory::Frozen
                | WorldTemperatureCategory::VeryCold
                | WorldTemperatureCategory::Cold
                | WorldTemperatureCategory::Chilly => (20000, -10000, 0, 10000),
                _ => (0, 0, 0, 0),
            },
            CelestialBodyWorldType::Ammonia => match temperature_category {
                WorldTemperatureCategory::Frozen
                | WorldTemperatureCategory::VeryCold
                | WorldTemperatureCategory::Cold
                | WorldTemperatureCategory::Chilly
                | WorldTemperatureCategory::Cool
                | WorldTemperatureCategory::Temperate
                | WorldTemperatureCategory::Warm => (10000, 1000, 0, 10000),
                _ => (0, 0, 0, 0),
            },
            CelestialBodyWorldType::Rock => match temperature_category {
                WorldTemperatureCategory::Frozen
                | WorldTemperatureCategory::VeryCold
                | WorldTemperatureCategory::Cold
                | WorldTemperatureCategory::Chilly => (10000, -5000, 0, 6000),
                _ => (0, 0, 0, 0),
            },
            CelestialBodyWorldType::Ocean | CelestialBodyWorldType::Terrestrial => {
                match temperature_category {
                    WorldTemperatureCategory::Frozen => (7000, 7000, 6000, 10000),
                    WorldTemperatureCategory::VeryCold => (7500, 3500, 3000, 10000),
                    WorldTemperatureCategory::Cold => (5000, 2500, 2000, 9000),
                    WorldTemperatureCategory::Chilly => (4000, 1500, 1000, 7000),
                    WorldTemperatureCategory::Cool => (3000, 1000, 500, 5000),
                    WorldTemperatureCategory::Temperate => (2500, 450, 300, 3000),
                    WorldTemperatureCategory::Warm => (1500, 350, 0, 2000),
                    WorldTemperatureCategory::Hot => (500, 250, 0, 1000),
                    WorldTemperatureCategory::VeryHot => (0, 0, 0, 0),
                    WorldTemperatureCategory::Scorching => (0, 0, 0, 0),
                    WorldTemperatureCategory::Infernal => (0, 0, 0, 0),
                }
            }
        };
        cryosphere = if to_roll.0 == 0 {
            0.0
        } else {
            rng.roll(1, to_roll.0, to_roll.1)
                .max(to_roll.2)
                .min(to_roll.3) as f32
                / 100.0
        };

        // Add some digits if round number
        if cryosphere > 0.01 {
            cryosphere += (rng.roll(1, 101, -51) as f32 / 100.0);
        }
        cryosphere = cryosphere.min(100.0).max(0.0);

        // If oceans of other elements than water were generated, let them be over the ices
        let mut was_there_something_else_than_water = false;
        for special_trait in &special_traits {
            if let CelestialBodySpecialTrait::Oceans(peculiar_component)
            | CelestialBodySpecialTrait::Lakes(peculiar_component) = special_trait
            {
                if peculiar_component != &ChemicalComponent::Water {
                    was_there_something_else_than_water = true;
                    if cryosphere + hydrosphere >= 100.0 {
                        cryosphere = 100.0 - hydrosphere;
                    }
                }
            }
        }

        if was_there_something_else_than_water {
            ice_over_land = cryosphere;
        } else {
            // Remove proportionally from land and oceans to make room for ices
            let land_prop = (100.0 - hydrosphere) / 100.0;
            let water_prop = hydrosphere / 100.0;
            ice_over_land = cryosphere * land_prop;
            ice_over_water = cryosphere * water_prop;
            hydrosphere = hydrosphere - ice_over_water;
        }

        (hydrosphere, ice_over_water, ice_over_land)
    }

    fn compute_oceans(
        coord: SpaceCoordinates,
        system_index: u16,
        star_id: u32,
        star_traits: &Vec<StarPeculiarity>,
        orbital_point_id: u32,
        settings: &GenerationSettings,
        world_type: CelestialBodyWorldType,
        blackbody_temperature: u32,
        special_traits: &mut Vec<CelestialBodySpecialTrait>,
        mut hydrosphere: f32,
        mut atmospheric_pressure: f32,
    ) -> (f32, f32) {
        if hydrosphere > 0.001 {
            let mut chosen_component = None;

            if world_type == CelestialBodyWorldType::Terrestrial
                || world_type == CelestialBodyWorldType::Ocean
            {
                chosen_component = Some(ChemicalComponent::Water);
            } else if let Some(components) =
                ChemicalComponent::components_liquid_at(blackbody_temperature, atmospheric_pressure)
            {
                if !components.is_empty() {
                    // Pick the most likely component to be the majority of liquid
                    let mut candidates: Vec<(ChemicalComponent, f64)> = vec![];
                    for &component in &components {
                        let score = liquid_majority_composition_likelihood(component, star_traits);
                        if score > 0.0 {
                            candidates.push((component, score));
                        }
                    }
                    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    chosen_component = candidates.first().map(|s| s.0);

                    if chosen_component.is_some() {
                        // If only one component was available, and not at the current pressure, modify the current pressure.
                        if !chosen_component
                            .unwrap()
                            .can_exist_as_liquid(blackbody_temperature, atmospheric_pressure)
                        {
                            if let Some((_, triple_point_pressure)) =
                                chosen_component.unwrap().triple_point()
                            {
                                if triple_point_pressure <= 1.0 {
                                    let mut rng = SeededDiceRoller::new(
                                        &settings.seed,
                                        &format!(
                                            "sys_{}_{}_str_{}_bdy{}_ocean",
                                            coord, system_index, star_id, orbital_point_id
                                        ),
                                    );
                                    atmospheric_pressure = triple_point_pressure as f32
                                        + rng.roll(1, 200, -1) as f32 / 100.0;
                                } else {
                                    chosen_component = None;
                                }
                            }
                        }
                    }
                }
            }

            // Then add lakes or oceans to the body
            if chosen_component.is_some() {
                if hydrosphere > 0.001 && hydrosphere < 50.0 {
                    special_traits
                        .push(CelestialBodySpecialTrait::Lakes(chosen_component.unwrap()));
                } else if hydrosphere >= 50.0 {
                    special_traits
                        .push(CelestialBodySpecialTrait::Oceans(chosen_component.unwrap()));
                }
            }

            if chosen_component.is_none() {
                // Nothing can be liquid so remove the hydrosphere
                hydrosphere = 0.0;
            }
        }
        (hydrosphere, atmospheric_pressure)
    }

    fn compute_subsurface_oceans(
        coord: SpaceCoordinates,
        system_index: u16,
        star_id: u32,
        star_traits: &Vec<StarPeculiarity>,
        orbital_point_id: u32,
        settings: &GenerationSettings,
        special_traits: &mut Vec<CelestialBodySpecialTrait>,
        world_type: CelestialBodyWorldType,
        core_heat: CelestialBodyCoreHeat,
        tidal_heating: u32,
        density: f32,
    ) {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_sbocean",
                coord, system_index, star_id, orbital_point_id
            ),
        );

        let mut can_have_water = true;
        let mut random_chance = 1;
        let mut random_chance_for_other_types = 6;
        for peculiarity in star_traits {
            if let StarPeculiarity::UnusualElementPresence((peculiar_component, occurrence)) =
                peculiarity
            {
                if *peculiar_component == ChemicalComponent::Water {
                    match occurrence {
                        ElementPresenceOccurrence::Absence => {
                            can_have_water = false;
                        }
                        ElementPresenceOccurrence::VeryLow => {
                            random_chance = 3;
                            random_chance_for_other_types = 24;
                        }
                        ElementPresenceOccurrence::Low => {
                            random_chance = 2;
                            random_chance_for_other_types = 12
                        }
                        ElementPresenceOccurrence::Normal => {}
                        ElementPresenceOccurrence::High => {
                            random_chance_for_other_types = 3;
                        }
                        ElementPresenceOccurrence::VeryHigh => {
                            random_chance_for_other_types = 2;
                        }
                        ElementPresenceOccurrence::Omnipresence => {
                            random_chance_for_other_types = 1;
                        }
                    }
                }
            }
        }
        if (can_have_water
            && (core_heat == CelestialBodyCoreHeat::ActiveCore
                || core_heat == CelestialBodyCoreHeat::IntenseCore)
            || tidal_heating >= 5)
            && ((world_type == CelestialBodyWorldType::Ice && rng.roll(1, random_chance, 0) == 1)
                || ((world_type == CelestialBodyWorldType::Ammonia
                    || world_type == CelestialBodyWorldType::Hadean
                    || world_type == CelestialBodyWorldType::DirtySnowball)
                    && density < 5.0
                    && rng.roll(1, random_chance_for_other_types, 0) == 1))
        {
            special_traits.push(CelestialBodySpecialTrait::SubSurfaceOceans(
                ChemicalComponent::Water,
            ));
        }
    }

    fn adjust_blackbody_temperature(
        coord: &SpaceCoordinates,
        system_index: &u16,
        star_id: &u32,
        orbital_point_id: &u32,
        settings: &GenerationSettings,
        blackbody_temperature: u32,
        size: CelestialBodySize,
        world_type: CelestialBodyWorldType,
        hydrosphere: f32,
        atmospheric_pressure: f32,
    ) -> u32 {
        // TODO: Make sure that proto-worlds are always hotter than rock melting point
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_temp",
                coord, system_index, star_id, orbital_point_id
            ),
        );

        let absorption_factor = match world_type {
            CelestialBodyWorldType::Ice | CelestialBodyWorldType::DirtySnowball => {
                if size == CelestialBodySize::Standard || size == CelestialBodySize::Large {
                    0.86
                } else if size == CelestialBodySize::Small {
                    0.93
                } else {
                    0.86
                }
            }
            CelestialBodyWorldType::Ammonia => 0.84,
            CelestialBodyWorldType::GeoActive => 0.77,
            CelestialBodyWorldType::Hadean => 0.67,
            CelestialBodyWorldType::Rock => {
                if size == CelestialBodySize::Standard || size == CelestialBodySize::Large {
                    0.95
                } else if size == CelestialBodySize::Small {
                    0.96
                } else {
                    0.97
                }
            }
            CelestialBodyWorldType::Ocean | CelestialBodyWorldType::Terrestrial => {
                if hydrosphere <= 20.0 {
                    0.95
                } else if hydrosphere <= 50.0 {
                    0.92
                } else if hydrosphere <= 90.0 {
                    0.88
                } else {
                    0.84
                }
            }
            CelestialBodyWorldType::Greenhouse => 0.77,
            CelestialBodyWorldType::Chthonian => 0.97,
            CelestialBodyWorldType::ProtoWorld => 1.21,
            _ => 0.97,
        };
        let greenhouse_factor = if atmospheric_pressure <= 1.0 {
            0.16 * atmospheric_pressure
        } else if atmospheric_pressure < 92.0 {
            0.16 + atmospheric_pressure * 0.0202
        } else {
            0.16 + 92.0 * 0.0202 + (0.001 * (atmospheric_pressure - 92.0))
        };
        (blackbody_temperature as f32 * (absorption_factor * (1.0 + greenhouse_factor))).max(0.0)
            as u32
    }

    fn generate_volcanism(
        coord: &SpaceCoordinates,
        system_index: &u16,
        star_id: &u32,
        star_age: f32,
        orbital_point_id: &u32,
        moons: &Vec<OrbitalPoint>,
        tidal_heating: u32,
        settings: &GenerationSettings,
        gravity: f32,
        size: CelestialBodySize,
        world_type: CelestialBodyWorldType,
        special_traits: &Vec<CelestialBodySpecialTrait>,
        core_heat: CelestialBodyCoreHeat,
    ) -> f32 {
        if world_type == CelestialBodyWorldType::ProtoWorld {
            return 100.0;
        }

        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_vol",
                coord, system_index, star_id, orbital_point_id
            ),
        );

        let mut modifier = if size == CelestialBodySize::Puny {
            -500
        } else {
            (((gravity / star_age) * 20.0) as i32) - 16
        };
        modifier += if core_heat == CelestialBodyCoreHeat::IntenseCore {
            15
        } else if core_heat == CelestialBodyCoreHeat::ActiveCore {
            10
        } else if core_heat == CelestialBodyCoreHeat::WarmCore {
            5
        } else {
            0
        };
        modifier +=
            if special_traits.contains(&CelestialBodySpecialTrait::SpecificGeologicActivity(
                TelluricGeologicActivity::GeologicallyDead,
            )) || special_traits.contains(&CelestialBodySpecialTrait::SpecificGeologicActivity(
                TelluricGeologicActivity::GeologicallyExtinct,
            )) {
                -500
            } else if special_traits.contains(&CelestialBodySpecialTrait::SpecificGeologicActivity(
                TelluricGeologicActivity::GeologicallyActive,
            )) {
                20
            } else {
                0
            };
        modifier += (tidal_heating * 4) as i32;
        modifier += (get_major_moons(moons).count() * 5) as i32;

        let mut roll = (rng.roll(3, 6, modifier) as f32).max(0.0).min(100.0);
        if roll > 0.01 {
            roll = (roll + (rng.roll(1, 201, -101) as f32) / 100.0)
                .max(0.0)
                .min(100.0);
        }

        roll
    }

    fn generate_tectonic_activity(
        coord: &SpaceCoordinates,
        system_index: &u16,
        star_id: &u32,
        orbital_point_id: &u32,
        moons: &Vec<OrbitalPoint>,
        settings: &GenerationSettings,
        size: CelestialBodySize,
        world_type: CelestialBodyWorldType,
        special_traits: &Vec<CelestialBodySpecialTrait>,
        core_heat: CelestialBodyCoreHeat,
        hydrosphere: f32,
        volcanism: f32,
    ) -> f32 {
        if world_type == CelestialBodyWorldType::ProtoWorld {
            return 100.0;
        }

        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_tct",
                coord, system_index, star_id, orbital_point_id
            ),
        );

        let mut modifier = if size == CelestialBodySize::Puny {
            -500
        } else if size == CelestialBodySize::Tiny && core_heat != CelestialBodyCoreHeat::FrozenCore
        {
            -26
        } else if size == CelestialBodySize::Small && core_heat != CelestialBodyCoreHeat::FrozenCore
        {
            -18
        } else {
            -6
        };
        modifier +=
            if special_traits.contains(&CelestialBodySpecialTrait::SpecificGeologicActivity(
                TelluricGeologicActivity::GeologicallyDead,
            )) || special_traits.contains(&CelestialBodySpecialTrait::SpecificGeologicActivity(
                TelluricGeologicActivity::GeologicallyExtinct,
            )) {
                -500
            } else if special_traits.contains(&CelestialBodySpecialTrait::SpecificGeologicActivity(
                TelluricGeologicActivity::GeologicallyActive,
            )) {
                5
            } else {
                0
            };
        modifier += if volcanism < 0.01 {
            -8
        } else if volcanism <= 4.0 {
            -4
        } else if volcanism > 19.0 && volcanism <= 54.0 {
            4
        } else {
            8
        };
        modifier += if hydrosphere <= 0.0 {
            -4
        } else if hydrosphere < 50.0 {
            -2
        } else {
            0
        };
        modifier += if get_major_moons(moons).count() > 1 {
            4
        } else if get_major_moons(moons).count() > 0 {
            2
        } else {
            0
        };

        let mut roll = (rng.roll(3, 6, modifier) as f32 * 4.0).max(0.0).min(100.0);
        if roll > 0.01 {
            roll = (roll + (rng.roll(1, 801, -401) as f32) / 100.0)
                .max(0.0)
                .min(100.0);
        }

        roll
    }

    fn generate_hydrosphere(
        coord: &SpaceCoordinates,
        system_index: &u16,
        star_id: &u32,
        orbital_point_id: &u32,
        settings: &GenerationSettings,
        size: CelestialBodySize,
        world_type: CelestialBodyWorldType,
        special_traits: &Vec<CelestialBodySpecialTrait>,
        tidal_heating: u32,
        magnetic_field: MagneticFieldStrength,
        density: f32,
    ) -> f32 {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_hydr",
                coord, system_index, star_id, orbital_point_id
            ),
        );

        let mut hydrosphere = {
            let mut modifier = match magnetic_field {
                MagneticFieldStrength::None => -3000,
                MagneticFieldStrength::Weak => -1500,
                MagneticFieldStrength::Moderate => 0,
                MagneticFieldStrength::Strong => 1000,
                MagneticFieldStrength::VeryStrong => 2000,
                MagneticFieldStrength::Extreme => 3000,
            };
            modifier += if special_traits.contains(
                &CelestialBodySpecialTrait::UnusualVolatileDensity(
                    TelluricVolatileDensityDifference::Poor,
                ),
            ) {
                -2000
            } else if special_traits.contains(&CelestialBodySpecialTrait::UnusualVolatileDensity(
                TelluricVolatileDensityDifference::Rich,
            )) {
                2000
            } else {
                0
            };
            modifier += if tidal_heating < 1 {
                -1000
            } else if tidal_heating < 3 {
                0
            } else if tidal_heating < 5 {
                1000
            } else {
                2000
            };
            match world_type {
                CelestialBodyWorldType::Ice => {
                    if size == CelestialBodySize::Small {
                        (rng.roll(1, 6000, 2499 + modifier) as f32 / 100.0)
                            .min(85.0)
                            .max(0.0)
                    } else if size == CelestialBodySize::Standard
                        || size == CelestialBodySize::Large
                    {
                        (rng.roll(2, 6000, -10000 + modifier) as f32 / 100.0)
                            .min(20.0)
                            .max(0.0)
                    } else {
                        0.0
                    }
                }
                CelestialBodyWorldType::Greenhouse => {
                    if size == CelestialBodySize::Standard || size == CelestialBodySize::Large {
                        (rng.roll(2, 6000, -7000 + modifier) as f32 / 100.0)
                            .min(50.0)
                            .max(0.0)
                    } else {
                        0.0
                    }
                }
                CelestialBodyWorldType::Ammonia => {
                    if size == CelestialBodySize::Standard || size == CelestialBodySize::Large {
                        (rng.roll(2, 5001, 1998 + modifier) as f32 / 100.0)
                            .min(100.0)
                            .max(20.0)
                    } else {
                        0.0
                    }
                }
                CelestialBodyWorldType::Ocean => (rng.roll(1, 2250, 8249 + modifier) as f32
                    / 100.0)
                    .min(100.0)
                    .max(if density < 1.5 {
                        97.5
                    } else if density < 1.9 {
                        95.0
                    } else if density < 2.5 {
                        90.0
                    } else {
                        87.5
                    }),
                CelestialBodyWorldType::Terrestrial => (rng.roll(3, 2903, 997 + modifier) as f32
                    / 100.0)
                    .min(if density < 1.9 {
                        100.0
                    } else if density < 2.5 {
                        95.0
                    } else {
                        90.0
                    })
                    .max(if density < 1.5 {
                        95.0
                    } else if density < 1.9 {
                        70.0
                    } else if density < 2.5 {
                        50.5
                    } else {
                        9.8
                    }),
                _ => 0.0,
            }
        };

        if hydrosphere > 0.01 {
            hydrosphere += (rng.roll(1, 101, -51) as f32 / 100.0);
        }

        hydrosphere.min(100.0).max(0.0)
    }

    fn generate_magnetic_field(
        coord: SpaceCoordinates,
        system_index: u16,
        star_id: u32,
        orbital_point_id: u32,
        orbit: Option<Orbit>,
        density: f32,
        size: CelestialBodySize,
        core_heat: CelestialBodyCoreHeat,
        special_traits: &Vec<CelestialBodySpecialTrait>,
        settings: &GenerationSettings,
    ) -> MagneticFieldStrength {
        let mut rng = SeededDiceRoller::new(
            &settings.seed,
            &format!(
                "sys_{}_{}_str_{}_bdy{}_mag",
                coord, system_index, star_id, orbital_point_id
            ),
        );

        let mut modifier = if size == CelestialBodySize::Puny {
            -12
        } else if size == CelestialBodySize::Tiny {
            -6
        } else if size == CelestialBodySize::Small {
            -3
        } else if size == CelestialBodySize::Standard {
            0
        } else if size == CelestialBodySize::Large {
            3
        } else {
            18
        };
        modifier += if special_traits.contains(&CelestialBodySpecialTrait::UnusualMagneticField(
            TelluricMagneticFieldDifference::MuchStronger,
        )) {
            12
        } else if special_traits.contains(&CelestialBodySpecialTrait::UnusualMagneticField(
            TelluricMagneticFieldDifference::Stronger,
        )) {
            6
        } else if special_traits.contains(&CelestialBodySpecialTrait::UnusualMagneticField(
            TelluricMagneticFieldDifference::Weaker,
        )) {
            -6
        } else if special_traits.contains(&CelestialBodySpecialTrait::UnusualMagneticField(
            TelluricMagneticFieldDifference::MuchWeaker,
        )) {
            -12
        } else {
            0
        };
        modifier += {
            let rotation_speed = orbit.clone().unwrap_or_default().rotation;
            if rotation_speed <= 0.3 {
                5
            } else if rotation_speed <= 0.7 {
                3
            } else if rotation_speed <= 1.1 {
                1
            } else {
                0
            }
        };
        modifier += if density >= 10.0 {
            5
        } else if density >= 7.0 {
            3
        } else if density >= 5.0 {
            1
        } else {
            0
        };
        modifier += if core_heat == CelestialBodyCoreHeat::IntenseCore {
            9
        } else if core_heat == CelestialBodyCoreHeat::ActiveCore {
            3
        } else if core_heat == CelestialBodyCoreHeat::WarmCore {
            0
        } else {
            -6
        };
        let roll = rng.roll(3, 6, modifier);

        let mut magnetic_field;
        if roll <= 2 {
            magnetic_field = MagneticFieldStrength::None;
        } else if roll <= 8 {
            magnetic_field = MagneticFieldStrength::Weak;
        } else if roll <= 16 {
            magnetic_field = MagneticFieldStrength::Moderate;
        } else {
            magnetic_field = MagneticFieldStrength::Strong;
        }

        magnetic_field
    }

    fn generate_core_heat(
        coord: SpaceCoordinates,
        system_index: u16,
        star_id: u32,
        star_age: f32,
        orbital_point_id: u32,
        own_orbit: &Orbit,
        size: CelestialBodySize,
        density: f32,
        body_type: TelluricBodyComposition,
        world_type: CelestialBodyWorldType,
        special_traits: &Vec<CelestialBodySpecialTrait>,
        tidal_heating: u32,
        rotation_speed: f32,
        settings: &GenerationSettings,
        distance_from_star: f64,
    ) -> CelestialBodyCoreHeat {
        if size == CelestialBodySize::Tiny {
            CelestialBodyCoreHeat::FrozenCore
        } else if world_type == CelestialBodyWorldType::ProtoWorld {
            CelestialBodyCoreHeat::IntenseCore
        } else {
            let mut rng = SeededDiceRoller::new(
                &settings.seed,
                &format!(
                    "sys_{}_{}_str_{}_bdy{}_core",
                    coord, system_index, star_id, orbital_point_id
                ),
            );
            let mut core_heat_modifier = 0;
            core_heat_modifier += if size == CelestialBodySize::Puny {
                -100
            } else if size == CelestialBodySize::Tiny {
                -5
            } else if size == CelestialBodySize::Small {
                -2
            } else if size == CelestialBodySize::Standard {
                2
            } else if size == CelestialBodySize::Large {
                3
            } else {
                5
            };
            core_heat_modifier += if special_traits.iter().any(|x| {
                matches!(
                    x,
                    CelestialBodySpecialTrait::UnusualCore(TelluricCoreDifference::Coreless)
                )
            }) {
                -100
            } else if special_traits.iter().any(|x| {
                matches!(
                    x,
                    CelestialBodySpecialTrait::UnusualCore(TelluricCoreDifference::Smaller)
                )
            }) {
                -2
            } else if special_traits.iter().any(|x| {
                matches!(
                    x,
                    CelestialBodySpecialTrait::UnusualCore(TelluricCoreDifference::Smaller)
                )
            }) {
                2
            } else {
                0
            };
            core_heat_modifier += if special_traits.iter().any(|x| {
                matches!(
                    x,
                    CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyExtinct
                    ) | CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyDead
                    )
                )
            }) {
                -100
            } else if special_traits.iter().any(|x| {
                matches!(
                    x,
                    CelestialBodySpecialTrait::SpecificGeologicActivity(
                        TelluricGeologicActivity::GeologicallyActive
                    )
                )
            }) {
                5
            } else {
                0
            };
            core_heat_modifier += if star_age < 0.703 {
                5
            } else if star_age < 1.251 {
                3
            } else if star_age < 1.6 {
                1
            } else if star_age < 2.0 {
                0
            } else if star_age < 5.730 {
                -1
            } else if star_age < 7.0 {
                -2
            } else if star_age < 10.0 {
                -3
            } else if star_age < 14.05 {
                -4
            } else if star_age < 20.0 {
                -5
            } else if star_age < 25.0 {
                -6
            } else if star_age < 30.0 {
                -7
            } else if star_age < 35.0 {
                -8
            } else {
                -9
            };
            core_heat_modifier += if body_type == TelluricBodyComposition::Metallic {
                1
            } else if body_type == TelluricBodyComposition::Rocky {
                0
            } else {
                -1
            };
            core_heat_modifier += if density < 3.0 {
                -1
            } else if density > 5.0 {
                1
            } else {
                0
            };
            core_heat_modifier += if distance_from_star <= 0.1 {
                2
            } else if distance_from_star <= 0.5 {
                1
            } else if distance_from_star <= 1.5 {
                0
            } else if distance_from_star <= 5.0 {
                -1
            } else {
                -2
            };
            core_heat_modifier += if rotation_speed <= 0.3 {
                5
            } else if rotation_speed <= 0.7 {
                3
            } else if rotation_speed <= 1.1 {
                1
            } else {
                0
            };
            core_heat_modifier += if own_orbit.eccentricity > 0.3 {
                2
            } else if own_orbit.eccentricity >= 0.1 {
                1
            } else {
                0
            };
            core_heat_modifier += (tidal_heating / 5) as i32;
            // TODO: Lower the results, not enough active cores in systems as old as ours
            rng.get_result(&CopyableRollToProcess::new(
                vec![
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::FrozenCore, 1),
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::WarmCore, 4),
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::ActiveCore, 6),
                    CopyableWeightedResult::new(CelestialBodyCoreHeat::IntenseCore, 1),
                ],
                RollMethod::PreparedRoll(PreparedRoll::new(2, 6, core_heat_modifier)),
            ))
            .expect("Should have generated a core heat value.")
        }
    }
}

fn generate_atmosphere(
    coord: SpaceCoordinates,
    system_index: u16,
    star_id: u32,
    star_age: f32,
    star_type: &StarSpectralType,
    star_class: &StarLuminosityClass,
    star_traits: &Vec<StarPeculiarity>,
    orbital_point_id: u32,
    own_orbit: &Orbit,
    size: CelestialBodySize,
    mass: f64,
    body_type: TelluricBodyComposition,
    world_type: CelestialBodyWorldType,
    volcanism_and_tectonics: f32,
    hydrosphere: f32,
    is_moon: bool,
    settings: &GenerationSettings,
) -> f32 {
    let mut atmospheric_mass_modifier = 0;
    atmospheric_mass_modifier += if own_orbit.average_distance <= 0.2 {
        -2
    } else if own_orbit.average_distance <= 2.0 {
        -1
    } else {
        0
    };
    atmospheric_mass_modifier += if mass >= 20.0 {
        5
    } else if mass >= 10.0 {
        4
    } else if mass >= 6.0 {
        3
    } else if mass >= 3.0 {
        2
    } else if mass >= 1.0 {
        1
    } else if mass < 0.1 {
        -1
    } else {
        0
    };
    atmospheric_mass_modifier += if star_age <= 0.1 {
        1
    } else if star_age >= 10.0 {
        -2
    } else if star_age >= 5.0 {
        -1
    } else {
        0
    };
    atmospheric_mass_modifier += if body_type == TelluricBodyComposition::Icy {
        2
    } else {
        0
    };
    atmospheric_mass_modifier += if size == CelestialBodySize::Tiny {
        -4
    } else if size == CelestialBodySize::Small {
        -2
    } else {
        0
    };
    atmospheric_mass_modifier += if volcanism_and_tectonics < 10.0 {
        -1
    } else if volcanism_and_tectonics < 20.0 {
        0
    } else if volcanism_and_tectonics < 50.0 {
        1
    } else if volcanism_and_tectonics < 100.0 {
        2
    } else {
        5
    };
    atmospheric_mass_modifier += match star_type {
        StarSpectralType::WR(_) => -5,
        StarSpectralType::O(_) => -4,
        StarSpectralType::B(_) => -3,
        _ => 0,
    };
    atmospheric_mass_modifier += match star_class {
        StarLuminosityClass::O => -5,
        StarLuminosityClass::Ia => -5,
        StarLuminosityClass::Ib => -5,
        StarLuminosityClass::II => -4,
        StarLuminosityClass::III => -3,
        StarLuminosityClass::XNS => -5,
        _ => 0,
    };
    atmospheric_mass_modifier += if world_type == CelestialBodyWorldType::Chthonian {
        -5
    } else {
        0
    };
    atmospheric_mass_modifier += if is_moon { -2 } else { 0 };
    for peculiarity in star_traits {
        if let StarPeculiarity::UnusualMetallicity(metallicity_difference) = peculiarity {
            if discriminant(metallicity_difference)
                == discriminant(&StarMetallicityDifference::MuchHigher)
            {
                2
            } else if discriminant(metallicity_difference)
                == discriminant(&StarMetallicityDifference::Higher)
            {
                1
            } else if discriminant(metallicity_difference)
                == discriminant(&StarMetallicityDifference::Lower)
            {
                -1
            } else {
                -2
            }
        } else if discriminant(peculiarity) == discriminant(&StarPeculiarity::ExcessiveRadiation)
            || discriminant(peculiarity) == discriminant(&StarPeculiarity::PowerfulStellarWinds)
            || discriminant(peculiarity) == discriminant(&StarPeculiarity::StrongMagneticField)
        {
            -2
        } else {
            0
        };
    }
    let mut rng = SeededDiceRoller::new(
        &settings.seed,
        &format!(
            "sys_{}_{}_str_{}_bdy{}_atmo",
            coord, system_index, star_id, orbital_point_id
        ),
    );
    let mut atmospheric_pressure = if size == CelestialBodySize::Puny {
        0.0
    } else if hydrosphere > 0.1 && world_type != CelestialBodyWorldType::Greenhouse {
        -1.0
    } else {
        match world_type {
            CelestialBodyWorldType::Ice
            | CelestialBodyWorldType::DirtySnowball
            | CelestialBodyWorldType::GeoActive => {
                if size == CelestialBodySize::Tiny && volcanism_and_tectonics < 10.0 {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Rock => {
                if (size == CelestialBodySize::Tiny || size == CelestialBodySize::Small)
                    && volcanism_and_tectonics < 10.0
                {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Hadean => {
                if (size == CelestialBodySize::Tiny
                    || size == CelestialBodySize::Small
                    || size == CelestialBodySize::Standard)
                    && volcanism_and_tectonics < 10.0
                {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Chthonian => {
                if volcanism_and_tectonics < 10.0 {
                    0.0
                } else {
                    -1.0
                }
            }
            CelestialBodyWorldType::Greenhouse => rng.gen_range(1.5..300.0),
            _ => -1.0,
        }
    };
    if atmospheric_pressure < 0.0 {
        let random_pressure_table = [
            (0.0, 0.01),
            (0.0, 0.01),
            (0.01, 0.5),
            (0.01, 0.5),
            (0.01, 0.5),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.8, 1.2),
            (0.8, 1.2),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.5, 10.0),
            (1.5, 10.0),
            (10.0, 300.0),
        ];
        let generic_pressure_table = [
            (0.01, 0.5),
            (0.01, 0.5),
            (0.01, 0.5),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.5, 10.0),
            (1.5, 10.0),
        ];
        let terrestrial_pressure_table = [
            (0.5, 0.8),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.5, 0.8),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (0.8, 1.2),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
            (1.2, 1.5),
        ];
        let atmospheric_pressure_bracket: (f32, f32) = (match world_type {
            CelestialBodyWorldType::GeoActive | CelestialBodyWorldType::Ammonia => {
                generic_pressure_table
            }
            CelestialBodyWorldType::Ocean | CelestialBodyWorldType::Terrestrial => {
                terrestrial_pressure_table
            }
            _ => random_pressure_table,
        })[(rng.roll(1, 10, atmospheric_mass_modifier).min(14).max(0) as usize)];
        atmospheric_pressure =
            rng.gen_range(atmospheric_pressure_bracket.0..atmospheric_pressure_bracket.1);
    }
    atmospheric_pressure
}
