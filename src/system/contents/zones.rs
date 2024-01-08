use crate::internal::*;
use crate::prelude::*;
use std::cmp::Ordering;

pub fn generate_star_zones(all_objects: &mut Vec<OrbitalPoint>) {
    let all_objects_clone = all_objects.clone();
    all_objects
        .iter_mut()
        .for_each(|o| calculate_star_zones(o, &all_objects_clone));
}

fn calculate_star_zones(orbital_point: &mut OrbitalPoint, all_objects: &[OrbitalPoint]) {
    let orbital_point_clone = orbital_point.clone();
    if let AstronomicalObject::Star(ref mut star) = orbital_point.object {
        calculate_corona_zone(star);
        calculate_inner_limit_zone(star);
        calculate_inner_zone(star);
        calculate_bio_zone(star);
        calculate_outer_zone(star);
        adjust_zones_for_bio(star);

        // If the star is orbiting a barycentre, it means that it's in a binary relationship
        if star.orbit.is_some() {
            calculate_forbidden_zone(star, &orbital_point_clone, all_objects);
            adjust_zones_for_forbidden(star);
        }

        split_zones(star);
        sort_zones(&mut star.zones);
    }
}

fn calculate_corona_zone(star: &mut Star) {
    let corona_radius = ConversionUtils::solar_radii_to_astronomical_units(star.radius as f64);
    star.zones
        .push(StarZone::new(0.0, corona_radius, ZoneType::Corona));
}

fn calculate_inner_limit_zone(star: &mut Star) {
    let using_mass = 0.1 * star.mass;
    let using_luminosity = 0.01 * star.luminosity.sqrt() as f64;
    let inner_limit_radius = if using_mass > using_luminosity {
        using_mass
    } else {
        using_luminosity
    } as f64;
    star.zones.push(StarZone::new(
        star.zones
            .iter()
            .find(|z| z.zone_type == ZoneType::Corona)
            .map(|z| z.end)
            .unwrap_or(0.0),
        inner_limit_radius,
        ZoneType::InnerLimit,
    ));
}

fn calculate_inner_zone(star: &mut Star) {
    let snow_line = 4.85 * star.luminosity.sqrt() as f64;
    let inner_limit = star
        .zones
        .iter()
        .find(|z| z.zone_type == ZoneType::InnerLimit)
        .map(|z| z.end)
        .unwrap_or(0.0);

    if snow_line > inner_limit {
        star.zones
            .push(StarZone::new(inner_limit, snow_line, ZoneType::InnerZone));
    }
}

fn calculate_bio_zone(star: &mut Star) {
    let inner_habitable_zone = (star.luminosity as f64).sqrt();
    let outer_habitable_zone = 1.77 * (star.luminosity as f64).sqrt();
    let inner_limit = star
        .zones
        .iter()
        .find(|z| z.zone_type == ZoneType::InnerLimit)
        .map(|z| z.end)
        .unwrap_or(0.0);

    if outer_habitable_zone > inner_limit {
        star.zones.push(StarZone::new(
            if inner_habitable_zone > inner_limit {
                inner_habitable_zone
            } else {
                inner_limit
            },
            outer_habitable_zone,
            ZoneType::BioZone,
        ));
    }
}

fn calculate_outer_zone(star: &mut Star) {
    let outer_limit_radius = 40.0 * star.mass as f64;
    let inner_limit = star
        .zones
        .iter()
        .find(|z| z.zone_type == ZoneType::InnerLimit)
        .map(|z| z.end)
        .unwrap_or(0.0);
    let snow_line = star
        .zones
        .iter()
        .find(|z| z.zone_type == ZoneType::InnerZone)
        .map(|z| z.end)
        .unwrap_or(0.0);

    if outer_limit_radius > inner_limit && outer_limit_radius > snow_line {
        star.zones.push(StarZone::new(
            if snow_line > inner_limit {
                snow_line
            } else {
                inner_limit
            },
            outer_limit_radius,
            ZoneType::OuterZone,
        ));
    }
}

fn adjust_zones_for_bio(star: &mut Star) {
    if let Some(bio_zone) = star
        .zones
        .iter_mut()
        .find(|zone| zone.zone_type == ZoneType::BioZone)
        .cloned()
    {
        let other_zones: Vec<_> = star
            .zones
            .iter()
            .filter(|zone| {
                zone.zone_type != ZoneType::BioZone && zone.zone_type != ZoneType::ForbiddenZone
            })
            .cloned()
            .collect();

        for zone in star.zones.iter_mut() {
            if zone.zone_type == ZoneType::BioZone {
                for other_zone in &other_zones {
                    if zone.is_overlapping(other_zone) {
                        zone.adjust_for_overlap(other_zone);
                    }
                }
            }
        }
    }
}

fn calculate_forbidden_zone(
    star: &mut Star,
    orbital_point: &OrbitalPoint,
    all_objects: &[OrbitalPoint],
) {
    let companion = get_closest_companion(orbital_point, all_objects);
    let min_separation = get_min_star_separation(orbital_point, &companion);
    let max_separation = get_max_star_separation(orbital_point, &companion);

    let forbidden_zone_inner_edge = min_separation / 3.0;
    let forbidden_zone_outer_edge = max_separation * 3.0;
    star.zones.push(StarZone::new(
        forbidden_zone_inner_edge,
        forbidden_zone_outer_edge,
        ZoneType::ForbiddenZone,
    ));
}

/// In the case of a multiple star system, returns the closest star from the given one.
fn get_closest_companion(star: &OrbitalPoint, all_objects: &[OrbitalPoint]) -> OrbitalPoint {
    let other_stars: Vec<_> = all_objects.iter().filter(|&o| o.id != star.id).collect();

    let star_distance = star
        .own_orbit
        .as_ref()
        .expect("Expected star to have an orbit")
        .average_distance_from_system_center;

    let (closest_star, _) = other_stars
        .iter()
        .filter(|object| object.own_orbit.is_some())
        .map(|object| {
            let distance = (object
                .own_orbit
                .as_ref()
                .expect("Expected object to have an orbit")
                .average_distance_from_system_center
                - star_distance)
                .abs();
            (object, distance)
        })
        .min_by_key(|&(_, distance)| OrderedFloat(distance))
        .expect("Expected at least one other star");

    (*closest_star).clone()
}

/// Get the radius before which planets could have a stable orbit around the first star of a binary pair.
fn get_min_star_separation(object1: &OrbitalPoint, object2: &OrbitalPoint) -> f64 {
    let orbit1 = object1
        .own_orbit
        .as_ref()
        .expect("An OrbitalPoint's own orbit should always be filled.");
    let perihelion_distance_object1 = (1.0 - orbit1.eccentricity as f64) * orbit1.average_distance;
    let orbit2 = object2
        .own_orbit
        .as_ref()
        .expect("An OrbitalPoint's own orbit should always be filled.");
    let aphelion_distance_object2 = (1.0 + orbit2.eccentricity as f64) * orbit2.average_distance;
    (aphelion_distance_object2 - perihelion_distance_object1).abs()
}

/// Get the radius after which planets could have a stable orbit around the first star of a binary pair.
fn get_max_star_separation(object1: &OrbitalPoint, object2: &OrbitalPoint) -> f64 {
    let orbit1 = object1
        .own_orbit
        .as_ref()
        .expect("An OrbitalPoint's own orbit should always be filled.");
    let aphelion_distance_object1 = (1.0 + orbit1.eccentricity as f64) * orbit1.average_distance;
    let orbit2 = object2
        .own_orbit
        .as_ref()
        .expect("An OrbitalPoint's own orbit should always be filled.");
    let perihelion_distance_object2 = (1.0 - orbit2.eccentricity as f64) * orbit2.average_distance;
    (aphelion_distance_object1 - perihelion_distance_object2).abs()
}

fn adjust_zones_for_forbidden(star: &mut Star) {
    let forbidden_zones: Vec<_> = star
        .zones
        .iter()
        .filter(|zone| zone.zone_type == ZoneType::ForbiddenZone)
        .cloned()
        .collect();

    star.zones.retain(|zone| {
        zone.zone_type == ZoneType::ForbiddenZone
            || !forbidden_zones
                .iter()
                .any(|forbidden| zone.is_inside(forbidden))
    });

    let mut new_zones = Vec::new();
    for zone in &mut star.zones {
        for forbidden in &forbidden_zones {
            if zone.is_overlapping(forbidden) {
                if let Some(new_zone) = zone.adjust_for_overlap(forbidden) {
                    new_zones.push(new_zone);
                }
            }
        }
    }

    star.zones.append(&mut new_zones);
}

/// Splits any zones that are fully contained within another ones
fn split_zones(star: &mut Star) {
    let all_zones = star.zones.clone();

    let mut new_zones = star.zones.clone();
    let mut zones_to_remove = Vec::new();

    for zone in all_zones.iter() {
        let containing_zones: Vec<_> = all_zones
            .iter()
            .filter(|other_zone| other_zone.contains(zone) && *other_zone != zone)
            .collect();

        for containing_zone in containing_zones {
            let split_zones = containing_zone.split(zone);
            if let Some((zone1, zone2)) = split_zones {
                zones_to_remove.push(containing_zone.clone());
                new_zones.push(zone1);
                new_zones.push(zone2);
            }
        }
    }

    new_zones.retain(|zone| !zones_to_remove.contains(zone));

    star.zones = new_zones;
}

pub fn collect_all_zones(all_objects: &mut Vec<OrbitalPoint>) -> Vec<StarZone> {
    let mut all_zones: Vec<StarZone> = Vec::new();

    for o in all_objects {
        if let AstronomicalObject::Star(ref star) = o.object {
            for zone in &star.zones {
                let mut system_zone = zone.clone();
                if let Some(own_orbit) = &o.own_orbit {
                    system_zone.start += own_orbit.average_distance_from_system_center;
                    system_zone.end += own_orbit.average_distance_from_system_center;

                    if own_orbit.average_distance_from_system_center > zone.end {
                        let mirrored_start =
                            own_orbit.average_distance_from_system_center - zone.end;
                        let mirrored_end =
                            own_orbit.average_distance_from_system_center - zone.start;
                        let mirrored_zone = StarZone {
                            start: mirrored_start,
                            end: mirrored_end,
                            zone_type: zone.zone_type,
                        };
                        all_zones.push(mirrored_zone);
                    }
                }
                all_zones.push(system_zone);
            }
        }
    }

    sort_zones(&mut all_zones);
    consolidate_zones(&mut all_zones);
    merge_same_zones(&mut all_zones);

    all_zones
}

fn sort_zones(zones: &mut Vec<StarZone>) {
    zones.sort_by(|a, b| {
        a.start
            .partial_cmp(&b.start)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.end.partial_cmp(&b.end).unwrap_or(Ordering::Equal))
    });
}

fn consolidate_zones(all_zones: &mut Vec<StarZone>) {
    let mut i = 0;

    while i < all_zones.len() - 1 {
        let zone1 = &all_zones[i];
        let zone2 = &all_zones[i + 1];

        if zone1.end > zone2.start {
            if zone_priority(&zone1.zone_type) >= zone_priority(&zone2.zone_type) {
                all_zones[i].end = zone2.end;
            } else {
                all_zones[i + 1].start = zone1.start;
            }
            all_zones.remove(i);
        } else {
            i += 1;
        }
    }
}

fn zone_priority(zone: &ZoneType) -> u8 {
    match zone {
        ZoneType::ForbiddenZone => 6,
        ZoneType::Corona => 5,
        ZoneType::InnerLimit => 4,
        ZoneType::BioZone => 3,
        ZoneType::InnerZone => 2,
        ZoneType::OuterZone => 1,
    }
}

fn merge_same_zones(all_zones: &mut Vec<StarZone>) {
    let mut i = 0;
    while i < all_zones.len() - 1 {
        if all_zones[i].zone_type == all_zones[i + 1].zone_type {
            all_zones[i].end = all_zones[i + 1].end;
            all_zones.remove(i + 1);
        } else {
            i += 1;
        }
    }
}
