use crate::internal::generator::get_major_moons;
use crate::internal::*;
use crate::prelude::*;
use crate::system::celestial_body::world::utils::get_climate_from_temperature;
use crate::system::contents::elements::ChemicalComponent;
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
                    WorldClimateType::Frozen,
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
            world_type,
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
        );

        let cryosphere = 0.0;

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
            &special_traits,
            core_heat,
            hydrosphere,
            volcanism,
        );

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

        let present_volatiles: Vec<ChemicalComponent> = Vec::new();
        if hydrosphere > 0.001 {
            if let Some(components) = ChemicalComponent::components_liquid_at(
                blackbody_temperature as f64,
                atmospheric_pressure as f64,
            ) {
                if !components.is_empty() {
                    // Pixk the most likely component to be the majority of liquid
                    let mut candidates: Vec<(ChemicalComponent, f64)> = vec![];
                    for &component in &components {
                        let score = liquid_majority_composition_likelihood(component, star_traits);
                        if score > 0.0 {
                            candidates.push((component, score));
                        }
                    }
                    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    let chosen_component = candidates.first().map(|s| s.0);

                    if chosen_component.is_some() {
                        // If only one component was available, and not at the current pressure, modify the current pressure.
                        if !chosen_component.unwrap().can_exist_as_liquid(
                            blackbody_temperature as f64,
                            atmospheric_pressure as f64,
                        ) {
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
                                    // Nothing can be liquid so remove the hydrosphere
                                    hydrosphere = 0.0;
                                }
                            }
                        }

                        // Then add lakes or oceans to the body
                        if hydrosphere > 0.001 && hydrosphere < 50.0 {
                            special_traits
                                .push(CelestialBodySpecialTrait::Lakes(chosen_component.unwrap()));
                        } else if hydrosphere >= 50.0 {
                            special_traits
                                .push(CelestialBodySpecialTrait::Oceans(chosen_component.unwrap()));
                        } else {
                            // Nothing can be liquid so remove the hydrosphere
                            hydrosphere = 0.0;
                        }
                    } else {
                        // Nothing can be liquid so remove the hydrosphere
                        hydrosphere = 0.0;
                    }
                } else {
                    // Nothing can be liquid so remove the hydrosphere
                    hydrosphere = 0.0;
                }
            } else {
                // Nothing can be liquid so remove the hydrosphere
                hydrosphere = 0.0;
            }
        }

        let climate = get_climate_from_temperature(blackbody_temperature);

        // TODO: Atmospheric composition
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
                    cryosphere,
                    volcanism,
                    tectonics,
                    climate,
                )),
            )),
            orbits.clone(),
        )
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
            CelestialBodyWorldType::Sulfur => 0.77,
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
        special_traits: &Vec<CelestialBodySpecialTrait>,
        core_heat: CelestialBodyCoreHeat,
    ) -> f32 {
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
        special_traits: &Vec<CelestialBodySpecialTrait>,
        core_heat: CelestialBodyCoreHeat,
        hydrosphere: f32,
        volcanism: f32,
    ) -> f32 {
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
                CelestialBodyWorldType::Ocean => (rng.roll(1, 3000, 6999 + modifier) as f32
                    / 100.0)
                    .min(100.0)
                    .max(70.0),
                CelestialBodyWorldType::Terrestrial => (rng.roll(3, 2903, 997 + modifier) as f32
                    / 100.0)
                    .min(90.0)
                    .max(9.8),
                _ => 0.0,
            }
        };

        if hydrosphere > 0.01 {
            hydrosphere += (rng.roll(1, 1001, -501) as f32 / 100.0);
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
        special_traits: &Vec<CelestialBodySpecialTrait>,
        tidal_heating: u32,
        rotation_speed: f32,
        settings: &GenerationSettings,
        distance_from_star: f64,
    ) -> CelestialBodyCoreHeat {
        if size == CelestialBodySize::Tiny {
            CelestialBodyCoreHeat::FrozenCore
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
            | CelestialBodyWorldType::Sulfur => {
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
            CelestialBodyWorldType::Sulfur | CelestialBodyWorldType::Ammonia => {
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
