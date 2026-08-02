use crate::system::celestial_body::world::types::WorldTemperatureCategory;
use crate::system::celestial_body::world::utils::get_category_from_temperature;
use crate::system::contents::utils::calculate_distance_for_temperature;
use crate::system::orbital_point::types::AstronomicalObject;
use crate::system::orbital_point::types::Orbit;
use crate::system::orbital_point::OrbitalPoint;
use crate::system::star::types::StarZone;
use crate::system::star::types::ZoneType;
use crate::system::star::Star;
use crate::utils::conversion::ConversionUtils;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

pub fn generate_star_zones(all_objects: &mut Vec<OrbitalPoint>) {
    // The companion lookup only runs for stars in a binary relationship; single-star
    // systems skip the full clone of the object list entirely.
    let has_binary = all_objects
        .iter()
        .any(|o| matches!(&o.object, AstronomicalObject::Star(star) if star.orbit.is_some()));
    if has_binary {
        let all_objects_clone = all_objects.clone();
        all_objects
            .iter_mut()
            .for_each(|o| calculate_star_zones(o, &all_objects_clone));
    } else {
        all_objects
            .iter_mut()
            .for_each(|o| calculate_star_zones(o, &[]));
    }
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
    let snow_line = calculate_distance_for_temperature(star.luminosity, 150);
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
    let inner_habitable_zone = calculate_distance_for_temperature(star.luminosity, 344);
    let outer_habitable_zone = calculate_distance_for_temperature(star.luminosity, 244);
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
    if let Some(_bio_zone) = star
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
    // None is unreachable for a star in a binary relationship; skip the forbidden
    // zone rather than panic if the invariant is ever broken.
    let Some(companion) = get_closest_companion(orbital_point, all_objects) else {
        return;
    };
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
/// Returns [None] if no other star with an orbit exists, which cannot happen when the
/// given star is in a binary relationship (the only caller checks that first).
///
/// Note: for hierarchical multiples (a pair orbiting another pair or star), pairing
/// each star with its closest companion is only a heuristic for the forbidden zone.
fn get_closest_companion(
    star: &OrbitalPoint,
    all_objects: &[OrbitalPoint],
) -> Option<OrbitalPoint> {
    let other_stars: Vec<_> = all_objects.iter().filter(|&o| o.id != star.id).collect();

    // The caller only invokes this for stars in a binary relationship, which always
    // have their own orbit filled; 0.0 is a deterministic, unreachable stand-in.
    let star_distance = star
        .own_orbit
        .as_ref()
        .map_or(0.0, |orbit| orbit.average_distance_from_system_center);

    other_stars
        .iter()
        .filter_map(|object| {
            object.own_orbit.as_ref().map(|orbit| {
                let distance = (orbit.average_distance_from_system_center - star_distance).abs();
                (object, distance)
            })
        })
        .min_by_key(|&(_, distance)| OrderedFloat(distance))
        .map(|(closest_star, _)| (*closest_star).clone())
}

/// Get the radius before which planets could have a stable orbit around the first star of a binary pair.
fn get_min_star_separation(object1: &OrbitalPoint, object2: &OrbitalPoint) -> f64 {
    // Both objects are binary-pair members and always have their own orbit filled;
    // a default orbit is a deterministic, unreachable stand-in.
    let orbit1 = object1.own_orbit.clone().unwrap_or_default();
    let orbit2 = object2.own_orbit.clone().unwrap_or_default();
    // The two stars orbit their shared barycentre on OPPOSITE sides, each at its own
    // barycentric distance, so the true separation is the SUM of both distances.
    // Both orbits carry the same eccentricity (rolled once per pair, aligned apsides),
    // hence the closest approach is (a1 + a2) * (1 - e).
    (1.0 - orbit1.eccentricity as f64) * (orbit1.average_distance + orbit2.average_distance)
}

/// Get the radius after which planets could have a stable orbit around the first star of a binary pair.
fn get_max_star_separation(object1: &OrbitalPoint, object2: &OrbitalPoint) -> f64 {
    // See `get_min_star_separation`: the default-orbit stand-in is unreachable, and
    // the widest separation is (a1 + a2) * (1 + e) with the shared pair eccentricity.
    let orbit1 = object1.own_orbit.clone().unwrap_or_default();
    let orbit2 = object2.own_orbit.clone().unwrap_or_default();
    (1.0 + orbit1.eccentricity as f64) * (orbit1.average_distance + orbit2.average_distance)
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

pub(crate) fn get_orbit_with_updated_zone(orbit: Orbit, blackbody_temperature: u32) -> Orbit {
    let temp_category = get_category_from_temperature(blackbody_temperature);

    Orbit {
        zone: if temp_category != WorldTemperatureCategory::Frozen
            && temp_category != WorldTemperatureCategory::Infernal
        {
            ZoneType::BioZone
        } else if orbit.zone == ZoneType::BioZone
            && temp_category == WorldTemperatureCategory::Infernal
        {
            ZoneType::InnerZone
        } else if orbit.zone == ZoneType::BioZone
            && temp_category == WorldTemperatureCategory::Frozen
            && blackbody_temperature > 150
        {
            ZoneType::InnerZone
        } else if orbit.zone == ZoneType::BioZone
            && temp_category == WorldTemperatureCategory::Frozen
            && blackbody_temperature <= 150
        {
            ZoneType::OuterZone
        } else {
            orbit.zone
        },
        ..orbit
    }
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
        let zone1 = all_zones[i].clone();
        let zone2 = all_zones[i + 1].clone();

        if zone1.end > zone2.start {
            // The list is sorted by start, so zone1 starts no later than zone2. The
            // surviving zone keeps zone1.start, extends to the furthest end (so a
            // zone fully contained in the other never shrinks the coverage), and is
            // typed after the higher-priority of the two; the other index is removed.
            if zone_priority(&zone1.zone_type) >= zone_priority(&zone2.zone_type) {
                all_zones[i].end = zone1.end.max(zone2.end);
                all_zones.remove(i + 1);
            } else {
                all_zones[i + 1].start = zone1.start;
                all_zones[i + 1].end = zone1.end.max(zone2.end);
                all_zones.remove(i);
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn binary_member(id: u32, average_distance: f64, eccentricity: f32) -> OrbitalPoint {
        let mut point = OrbitalPoint::new(id, None, AstronomicalObject::Void, vec![]);
        point.set_own_orbit(Orbit {
            average_distance,
            eccentricity,
            ..Default::default()
        });
        point
    }

    #[test]
    fn binary_separation_is_the_sum_of_barycentric_distances() {
        // Equal masses => equal barycentric distances (1.0 AU each), shared e = 0.3.
        // The stars orbit their barycentre on opposite sides, so the true
        // separation ranges over (a1 + a2) * (1 - e) .. (a1 + a2) * (1 + e).
        let star1 = binary_member(1, 1.0, 0.3);
        let star2 = binary_member(2, 1.0, 0.3);

        let min_separation = get_min_star_separation(&star1, &star2);
        let max_separation = get_max_star_separation(&star1, &star2);

        assert!(
            min_separation < max_separation,
            "an eccentric binary must have a non-empty separation range, got [{min_separation}, {max_separation}]"
        );
        // 0.3 is stored as f32, so compare with an f32-precision tolerance.
        assert!((min_separation - 2.0 * 0.7).abs() < 1e-6);
        assert!((max_separation - 2.0 * 1.3).abs() < 1e-6);
    }

    #[test]
    fn consolidate_zones_keeps_higher_priority_type_over_union() {
        // Overlapping zones, the higher-priority one comes second.
        let mut zones = vec![
            StarZone::new(0.5, 1.8, ZoneType::Corona),
            StarZone::new(1.0, 2.0, ZoneType::BioZone),
        ];
        consolidate_zones(&mut zones);
        assert_eq!(
            zones,
            vec![StarZone::new(0.5, 2.0, ZoneType::Corona)],
            "the surviving zone must keep the higher-priority type and span the union"
        );

        // Overlapping zones, the higher-priority one comes first.
        let mut zones = vec![
            StarZone::new(0.5, 1.5, ZoneType::BioZone),
            StarZone::new(1.0, 2.0, ZoneType::ForbiddenZone),
        ];
        consolidate_zones(&mut zones);
        assert_eq!(
            zones,
            vec![StarZone::new(0.5, 2.0, ZoneType::ForbiddenZone)],
        );
    }

    #[test]
    fn consolidate_zones_containment_never_shrinks_coverage() {
        // A lower-priority zone fully inside a higher-priority one.
        let mut zones = vec![
            StarZone::new(0.0, 10.0, ZoneType::ForbiddenZone),
            StarZone::new(2.0, 3.0, ZoneType::BioZone),
        ];
        consolidate_zones(&mut zones);
        assert_eq!(
            zones,
            vec![StarZone::new(0.0, 10.0, ZoneType::ForbiddenZone)],
        );

        // A higher-priority zone fully inside a lower-priority one: the union
        // still spans the container, typed after the higher-priority zone.
        let mut zones = vec![
            StarZone::new(0.0, 10.0, ZoneType::OuterZone),
            StarZone::new(2.0, 3.0, ZoneType::ForbiddenZone),
        ];
        consolidate_zones(&mut zones);
        assert_eq!(
            zones,
            vec![StarZone::new(0.0, 10.0, ZoneType::ForbiddenZone)],
        );
    }

    #[test]
    fn consolidate_zones_leaves_disjoint_zones_alone() {
        let mut zones = vec![
            StarZone::new(0.0, 1.0, ZoneType::Corona),
            StarZone::new(1.0, 2.0, ZoneType::BioZone),
        ];
        consolidate_zones(&mut zones);
        assert_eq!(
            zones,
            vec![
                StarZone::new(0.0, 1.0, ZoneType::Corona),
                StarZone::new(1.0, 2.0, ZoneType::BioZone),
            ],
        );
    }
}
