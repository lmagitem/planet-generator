//! Seeded end-to-end probes for cleanup-campaign Batch D: fixes K4 (dice gate
//! on the methane+oxygen atmosphere branches around M/L/T/Y stars) and K7
//! (proto-gas-giant conventional position spread around the snow line).
//!
//! Per campaign rules these are seeded probes asserting statistical properties
//! over a FIXED seed list, never exact values. Both fail on the pre-fix code:
//! - K4: pre-fix, the `&& rng.roll(...) != k` gate only applied to Y stars
//!   (operator precedence), so EVERY branch hit around an M/L/T star added
//!   methane+oxygen; post-fix roughly half to two thirds of those hits take the
//!   carbon-dioxide arm instead, dropping the methane+oxygen fraction.
//! - K7: pre-fix, conventional proto-giants sat at `snow_line + [0, 0.5]` AU
//!   (addition instead of multiplication); post-fix they spread over
//!   `[1.0, 1.5] * snow_line`, so far more giants land in the
//!   `(snow_line + 0.5, 1.5 * snow_line]` band.

use planet_generator::prelude::*;
use std::rc::Rc;

/// Fixed probe seeds, iterated with one hex each at (100, 100, 0). The list
/// must stay stable: changing it re-rolls the cases. With the sector settings
/// below (several systems per hex, per-parsec density rolls), that coordinate
/// lands in a dense central region and generates up to thousands of systems
/// per seed, like in the `non_default_sector` golden case.
const PROBE_SEEDS: &[&str] = &[
    "batch-d-0",
    "batch-d-1",
    "batch-d-2",
    "batch-d-3",
    "batch-d-4",
    "batch-d-5",
    "batch-d-6",
    "batch-d-7",
];

/// Generates one galaxy per probe seed and collects every star system in the
/// probed hex.
fn collect_systems() -> Vec<StarSystem> {
    let mut systems = Vec::new();
    for seed in PROBE_SEEDS {
        let settings = GenerationSettings {
            seed: Rc::from(*seed),
            sector: SectorSettings {
                flat_map: false,
                max_one_system_per_hex: false,
                density_by_hex_instead_of_parsec: false,
                hex_size: (2, 2, 2),
                ..Default::default()
            },
            ..Default::default()
        };
        let universe = Universe::generate(&settings, &mut Diagnostics::default());
        let neighborhood =
            GalacticNeighborhood::generate(universe, &settings, &mut Diagnostics::default());
        let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
        let hex = galaxy
            .get_hex(SpaceCoordinates::new(100, 100, 0))
            .unwrap_or_else(|e| panic!("get_hex((100, 100, 0)) should succeed: {e}"));
        systems.extend(hex.contents.iter().cloned());
    }
    systems
}

/// The star a body-holding orbital point directly orbits, if its primary is a
/// star (bodies orbiting a barycentre or a planet are skipped).
fn primary_star<'a>(system: &'a StarSystem, point: &OrbitalPoint) -> Option<&'a Star> {
    let orbit = point.own_orbit.as_ref()?;
    match &system.get_point(orbit.primary_body_id)?.object {
        AstronomicalObject::Star(star) => Some(star),
        _ => None,
    }
}

fn is_mlt_star(star: &Star) -> bool {
    matches!(
        star.spectral_type,
        StarSpectralType::M(_) | StarSpectralType::L(_) | StarSpectralType::T(_)
    )
}

/// K4 probe: around M/L/T primaries, the methane+oxygen atmosphere branch must
/// now be gated by its dice roll, so the fraction of atmospheric worlds with
/// BOTH methane and oxygen at notable-or-higher levels must drop well below
/// the unconditional pre-fix rate.
#[test]
fn methane_and_oxygen_is_no_longer_unconditional_around_mlt_stars() {
    let systems = collect_systems();

    let mut atmospheric_worlds = 0usize;
    let mut methane_and_oxygen = 0usize;
    for system in &systems {
        for point in &system.all_objects {
            let AstronomicalObject::TelluricBody(body) = &point.object else {
                continue;
            };
            let CelestialBodyDetails::Telluric(details) = &body.details else {
                continue;
            };
            if details.atmospheric_pressure <= 0.05 || details.atmospheric_composition.is_empty() {
                continue;
            }
            let Some(star) = primary_star(system, point) else {
                continue;
            };
            if !is_mlt_star(star) {
                continue;
            }
            atmospheric_worlds += 1;
            // Notable-or-higher presences roll at/above 10% after scaling;
            // trace-level flavor additions stay far below that.
            let pct_of = |component: ChemicalComponent| {
                details
                    .atmospheric_composition
                    .iter()
                    .find(|(_, c)| *c == component)
                    .map(|(p, _)| *p)
                    .unwrap_or(0.0)
            };
            if pct_of(ChemicalComponent::Methane) >= 10.0
                && pct_of(ChemicalComponent::Oxygen) >= 10.0
            {
                methane_and_oxygen += 1;
            }
        }
    }

    eprintln!(
        "K4 probe: {methane_and_oxygen} methane+oxygen worlds out of \
         {atmospheric_worlds} atmospheric worlds around M/L/T stars"
    );
    assert!(
        atmospheric_worlds >= 200,
        "The probe silently degraded: only {atmospheric_worlds} atmospheric worlds \
         around M/L/T stars across the fixed probe seeds. Extend the probe list \
         (a deliberate, commit-message-worthy change)."
    );
    // Pre-fix every branch hit around an M/L/T star added methane+oxygen (the
    // dice gate only applied to Y stars): 85/1584 atmospheric worlds (5.4%) on
    // this probe set. Post-fix the gate diverts roughly half of those hits to
    // the carbon-dioxide arms: 48/1570 (3.1%).
    assert!(
        methane_and_oxygen * 1000 <= 40 * atmospheric_worlds,
        "methane+oxygen is still (quasi-)unconditional around M/L/T stars: \
         {methane_and_oxygen}/{atmospheric_worlds} worlds"
    );
}

/// K7 probe: conventional proto-gas-giants must spread over
/// `[1.0, 1.5] * snow_line`, so the `(snow_line + 0.5, 1.5 * snow_line]` band
/// — pre-fix only reachable by the rarer eccentric arrangement — must hold a
/// substantial share of all gas giants.
#[test]
fn conventional_gas_giants_spread_beyond_half_an_au_past_the_snow_line() {
    let systems = collect_systems();

    let mut gas_giants = 0usize;
    let mut in_band = 0usize;
    for system in &systems {
        for point in &system.all_objects {
            let AstronomicalObject::GaseousBody(_) = &point.object else {
                continue;
            };
            let Some(star) = primary_star(system, point) else {
                continue;
            };
            let Some(snow_line) = star
                .zones
                .iter()
                .find(|z| z.zone_type == ZoneType::OuterZone)
                .map(|z| z.start)
            else {
                continue;
            };
            let distance = point.own_orbit.as_ref().map_or(0.0, |o| o.average_distance);
            if distance <= 0.0 || snow_line <= 0.0 {
                continue;
            }
            gas_giants += 1;
            if distance > snow_line + 0.5 && distance <= 1.5 * snow_line {
                in_band += 1;
            }
        }
    }

    eprintln!("K7 probe: {in_band} gas giants in the band out of {gas_giants} gas giants");
    assert!(
        gas_giants >= 100,
        "The probe silently degraded: only {gas_giants} gas giants across the \
         fixed probe seeds. Extend the probe list (a deliberate, \
         commit-message-worthy change)."
    );
    // Pre-fix only eccentric-arrangement giants could land in the band:
    // 24/3399 (0.7%) on this probe set. Post-fix conventional giants spread
    // over [1.0, 1.5] * snow_line and the band count jumps to 131/3466 (3.8%)
    // — most giants live outside the band on both versions (epistellar
    // close-in, eccentric anywhere, post-placement adjustments), so the
    // threshold pins the jump, not an absolute majority.
    assert!(
        in_band * 100 >= 2 * gas_giants,
        "conventional gas giants still pile up within 0.5 AU of the snow line: \
         {in_band}/{gas_giants} in the band"
    );
}

/// Batch H smoke test: every completed telluric world across the fixed probe
/// seeds must satisfy basic coherence invariants — percentage fields within
/// [0, 100] (humidity may be the -1.0 "not applicable" sentinel), atmosphere
/// shares normalized to ~100%, and a temperature category consistent with the
/// stored blackbody temperature. Also asserts the probe seeds actually produce
/// habitable-zone worlds, so the smoke test cannot silently pass on an empty
/// sample.
#[test]
fn telluric_worlds_are_internally_coherent() {
    let systems = collect_systems();

    let mut worlds = 0usize;
    let mut habitable_zone_worlds = 0usize;
    for system in &systems {
        for point in &system.all_objects {
            let AstronomicalObject::TelluricBody(body) = &point.object else {
                continue;
            };
            if body.is_stub() {
                continue;
            }
            let CelestialBodyDetails::Telluric(details) = &body.details else {
                continue;
            };
            worlds += 1;
            if point
                .own_orbit
                .as_ref()
                .is_some_and(|orbit| orbit.zone == ZoneType::BioZone)
            {
                habitable_zone_worlds += 1;
            }

            for (name, value) in [
                ("hydrosphere", details.hydrosphere),
                ("ice_over_water", details.ice_over_water),
                ("ice_over_land", details.ice_over_land),
                ("volcanism", details.volcanism),
                ("tectonic_activity", details.tectonic_activity),
            ] {
                assert!(
                    (0.0..=100.0).contains(&value),
                    "{name} of {} should be within [0, 100], got {value}",
                    body.name
                );
            }
            // `land_area_percentage` is clamped to >= 0 in the generator: on worlds
            // whose surface liquid isn't water, liquid and ice are accounted
            // separately and could otherwise exceed 100% together.
            assert!(
                (0.0..=100.0).contains(&details.land_area_percentage),
                "land_area_percentage of {} is out of range: {}",
                body.name,
                details.land_area_percentage
            );
            assert!(
                (-1.0..=100.0).contains(&details.humidity),
                "humidity of {} should be within [-1, 100] (-1 = not applicable), got {}",
                body.name,
                details.humidity
            );
            assert!(
                details.atmospheric_pressure >= 0.0,
                "atmospheric pressure of {} should be non-negative, got {}",
                body.name,
                details.atmospheric_pressure
            );

            // Atmosphere shares are normalized to sum to 99.99 when there is
            // any atmosphere at all.
            if !details.atmospheric_composition.is_empty() {
                let sum: f32 = details
                    .atmospheric_composition
                    .iter()
                    .map(|(share, _)| *share)
                    .sum();
                assert!(
                    (99.0..=100.01).contains(&sum),
                    "atmosphere shares of {} should sum to ~100, got {sum}",
                    body.name
                );
                for (share, component) in &details.atmospheric_composition {
                    assert!(
                        (0.0..=100.0).contains(share),
                        "atmosphere share of {component:?} on {} should be within [0, 100], got {share}",
                        body.name
                    );
                }
            }

            // The stored temperature category must match the stored (final,
            // greenhouse/tidal-adjusted) blackbody temperature. NOTE: no
            // world_type <-> temperature coherence is asserted — world_type is
            // assigned from the FORMATION temperature and the blackbody
            // temperature is adjusted afterwards (absorption/greenhouse
            // factors, `adjust_blackbody_temperature`), so e.g. a Terrestrial
            // world can legitimately end up VeryHot (observed: "Tiet 1d"
            // with these probe seeds), like a runaway-greenhouse Venus.
            assert_eq!(
                details.temperature_category,
                get_category_from_temperature(body.blackbody_temperature),
                "temperature category of {} is inconsistent with its blackbody temperature {} K",
                body.name,
                body.blackbody_temperature
            );
        }
    }

    eprintln!(
        "telluric smoke: {worlds} completed telluric worlds, \
         {habitable_zone_worlds} in the habitable zone"
    );
    assert!(
        worlds >= 50,
        "The smoke test silently degraded: only {worlds} telluric worlds across \
         the fixed probe seeds. Extend the probe list (a deliberate, \
         commit-message-worthy change)."
    );
    assert!(
        habitable_zone_worlds >= 1,
        "The probe seeds should produce at least one habitable-zone telluric world."
    );
}
