//! Cache-identity probes for lazily generated galactic hexes.
//!
//! The hex cache is keyed by a hex's index, so two coordinates that fall in the
//! same hex must resolve to the same cached value. These tests exercise that
//! identity with owned `GalacticHex` clones compared through the derived
//! `PartialEq`, and keep the open question of whether a cached hex depends on
//! which aliased coordinate was accessed first behind an explicitly ignored
//! hypothesis test. `PartialEq` is numerical equality (including +0 == -0), not
//! a bitwise guarantee.

use planet_generator::prelude::*;
use std::collections::BTreeMap;
use std::rc::Rc;

/// A single-layer region map painting the whole galaxy Core (white).
const CORE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="3" height="1" viewBox="0 0 3 1"><rect width="3" height="1" fill="#FFFFFF"/></svg>"##;

/// Under hex size (2, 1, 1), A(-1, 0, 0) and B(0, 0, 0) alias to hex index
/// (0, 0, 0); C(1, 0, 0) is the partial hex index (1, 0, 0) at the far edge.
const A: SpaceCoordinates = SpaceCoordinates { x: -1, y: 0, z: 0 };
const B: SpaceCoordinates = SpaceCoordinates { x: 0, y: 0, z: 0 };
const C: SpaceCoordinates = SpaceCoordinates { x: 1, y: 0, z: 0 };
const INDEX_0: SpaceCoordinates = SpaceCoordinates { x: 0, y: 0, z: 0 };
const INDEX_1: SpaceCoordinates = SpaceCoordinates { x: 1, y: 0, z: 0 };
const INDEX_2: SpaceCoordinates = SpaceCoordinates { x: 2, y: 0, z: 0 };
/// The two ends of the fixture galaxy (Irregular(3, 1, 1) spans x in [-1, 1]).
const START: SpaceCoordinates = A;
const END: SpaceCoordinates = C;

/// One step outside each of the six faces of the fixture galaxy's bounds.
const OUTSIDE_FACES: [SpaceCoordinates; 6] = [
    SpaceCoordinates { x: -2, y: 0, z: 0 },
    SpaceCoordinates { x: 2, y: 0, z: 0 },
    SpaceCoordinates { x: 0, y: -1, z: 0 },
    SpaceCoordinates { x: 0, y: 1, z: 0 },
    SpaceCoordinates { x: 0, y: 0, z: -1 },
    SpaceCoordinates { x: 0, y: 0, z: 1 },
];

fn fixture_settings() -> GenerationSettings {
    fixture_settings_with_hex_size((2, 1, 1))
}

fn fixture_settings_with_hex_size(hex_size: (u8, u8, u8)) -> GenerationSettings {
    GenerationSettings {
        seed: Rc::from("hex-cache-identity"),
        universe: UniverseSettings {
            fixed_age: Some(13.8),
            ..Default::default()
        },
        galaxy: GalaxySettings {
            use_ours: false,
            fixed_category: Some(GalaxyCategory::Irregular(3, 1, 1)),
            fixed_neighborhood: Some(GalacticNeighborhoodDensity::Group(1, 0)),
            galactic_map_layers: vec![CORE_SVG.to_string()],
            ..Default::default()
        },
        sector: SectorSettings {
            hex_size,
            flat_map: true,
            density_by_hex_instead_of_parsec: true,
            max_one_system_per_hex: true,
            ..Default::default()
        },
        system: SystemSettings {
            only_interesting: false,
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Asserts that a generation stage raised no fatal error. Warnings are
/// non-fatal by design and are deliberately not rejected here.
fn assert_no_errors(diagnostics: &Diagnostics, stage: &str) {
    assert!(
        diagnostics.errors.is_empty(),
        "{stage} fatal error(s): {:?}",
        diagnostics.errors
    );
}

/// Generates a fixture galaxy through the real pipeline (validate -> universe
/// -> neighborhood -> galaxy), rejecting fatal errors at each stage.
fn generate_fixture_galaxy(settings: &GenerationSettings) -> Galaxy {
    settings
        .validate()
        .unwrap_or_else(|errors| panic!("fixture settings must validate: {errors:?}"));
    let mut universe_diagnostics = Diagnostics::default();
    let universe = Universe::generate(settings, &mut universe_diagnostics);
    assert_no_errors(&universe_diagnostics, "universe generation");
    let mut neighborhood_diagnostics = Diagnostics::default();
    let neighborhood =
        GalacticNeighborhood::generate(universe, settings, &mut neighborhood_diagnostics);
    assert_no_errors(&neighborhood_diagnostics, "neighborhood generation");
    let galaxy = Galaxy::generate(neighborhood, 0, settings);
    assert_no_errors(&galaxy.diagnostics, "galaxy generation");
    galaxy
}

/// Returns the owned hex at `coord`, asserting every non-vacuity control: the
/// expected index, exactly one populated system with no fatal diagnostics, a
/// non-empty object list containing a star, and whole-value reflexivity.
fn get_hex(
    galaxy: &mut Galaxy,
    coord: SpaceCoordinates,
    expected_index: SpaceCoordinates,
) -> GalacticHex {
    let hex = galaxy
        .get_hex(coord)
        .unwrap_or_else(|e| panic!("get_hex({coord}): {e}"))
        .clone();
    assert_eq!(hex.index, expected_index, "get_hex({coord}) index");
    assert_eq!(
        hex.contents.len(),
        1,
        "get_hex({coord}) must hold one system"
    );
    for system in &hex.contents {
        assert!(
            system.diagnostics.errors.is_empty(),
            "system {:?} at {coord} errors",
            system.name
        );
        assert!(
            !system.all_objects.is_empty(),
            "system {:?} at {coord}: no objects",
            system.name
        );
        let has_star = system
            .all_objects
            .iter()
            .any(|point| matches!(&point.object, AstronomicalObject::Star(_)));
        assert!(has_star, "system {:?} at {coord}: no star", system.name);
    }
    assert!(hex == hex.clone(), "get_hex({coord}) is not reflexive");
    hex
}

/// Compact identity summary: index, system count, and each system's name,
/// center/main IDs and first-star mass/temperature.
fn describe_hex(hex: &GalacticHex) -> String {
    let systems = hex
        .contents
        .iter()
        .map(|system| {
            let first_star = system
                .all_objects
                .iter()
                .find_map(|point| match &point.object {
                    AstronomicalObject::Star(star) => Some(format!(
                        "name={:?}, mass={}, temperature={}",
                        star.name, star.mass, star.temperature
                    )),
                    _ => None,
                });
            format!(
                "name={:?}, center_id={}, main_star_id={}, first_star={}",
                system.name,
                system.center_id,
                system.main_star_id,
                first_star.as_deref().unwrap_or("none"),
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    format!(
        "index={:?}, system_count={}, systems=[{}]",
        hex.index,
        hex.contents.len(),
        systems
    )
}

/// Compares two owned hex clones through their derived `PartialEq`, reporting a
/// compact identity summary rather than a full `Debug` dump.
fn assert_hexes_equal(actual: &GalacticHex, expected: &GalacticHex, context: &str) {
    assert!(
        actual == expected,
        "{context}: owned hexes differ\n  actual:   {}\n  expected: {}",
        describe_hex(actual),
        describe_hex(expected),
    );
}

/// Replays a schedule of (query, expected index) pairs against a freshly
/// generated galaxy, returning the owned hex for each query keyed by the query.
fn replay_schedule(
    settings: &GenerationSettings,
    queries: &[(SpaceCoordinates, SpaceCoordinates)],
) -> BTreeMap<SpaceCoordinates, GalacticHex> {
    let mut galaxy = generate_fixture_galaxy(settings);
    let mut replayed = BTreeMap::new();
    for &(query, expected_index) in queries {
        let hex = get_hex(&mut galaxy, query, expected_index);
        if let Some(previous) = replayed.insert(query, hex) {
            assert_hexes_equal(
                &previous,
                &replayed[&query],
                &format!("repeated get_hex({query})"),
            );
        }
    }
    replayed
}

/// Compares two replays aligned by query/index rather than acquisition order.
fn assert_same_schedule_equal(
    actual: &BTreeMap<SpaceCoordinates, GalacticHex>,
    expected: &BTreeMap<SpaceCoordinates, GalacticHex>,
    context: &str,
) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{context}: different query counts"
    );
    for (query, hex) in actual {
        let other = expected
            .get(query)
            .unwrap_or_else(|| panic!("{context}: missing"));
        assert_hexes_equal(hex, other, &format!("{context}: query {query}"));
    }
}

#[test]
fn fixture_settings_validate_and_generation_reports_no_errors() {
    let settings = fixture_settings();
    eprintln!("fixture settings: {settings:?}");
    assert_eq!(
        settings.validate(),
        Ok(()),
        "the fixture settings must validate"
    );
    let mut universe_diagnostics = Diagnostics::default();
    let universe = Universe::generate(&settings, &mut universe_diagnostics);
    assert_no_errors(&universe_diagnostics, "universe generation");
    let mut neighborhood_diagnostics = Diagnostics::default();
    let neighborhood =
        GalacticNeighborhood::generate(universe, &settings, &mut neighborhood_diagnostics);
    assert_no_errors(&neighborhood_diagnostics, "neighborhood generation");
    let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
    assert_no_errors(&galaxy.diagnostics, "galaxy generation");
    assert_eq!(galaxy.index, 0);
    assert_eq!(galaxy.category, GalaxyCategory::Irregular(3, 1, 1));
    assert_eq!(
        galaxy.neighborhood.density,
        GalacticNeighborhoodDensity::Group(1, 0)
    );
    for &(query, index) in &[(A, INDEX_0), (B, INDEX_0), (C, INDEX_1)] {
        let _ = get_hex(&mut galaxy, query, index);
    }
    // Zero hex-size components are invalid; 255 is the accepted upper bound.
    for hex_size in [(0, 1, 1), (1, 0, 1), (1, 1, 0)] {
        assert_eq!(
            fixture_settings_with_hex_size(hex_size).validate(),
            Err(vec![SettingsError::InvalidHexSize(
                hex_size.0, hex_size.1, hex_size.2
            )]),
            "hex_size {hex_size:?} must be rejected by validation"
        );
    }
    assert_eq!(
        fixture_settings_with_hex_size((255, 255, 255)).validate(),
        Ok(()),
        "hex_size (255, 255, 255) must validate"
    );
}

#[test]
fn repeated_cache_reuse_and_index_cardinality() {
    let mut galaxy = generate_fixture_galaxy(&fixture_settings());
    let first = get_hex(&mut galaxy, A, INDEX_0);
    let repeated = get_hex(&mut galaxy, A, INDEX_0);
    assert!(first == repeated, "repeated get_hex(A) should be identical");
    let aliased = get_hex(&mut galaxy, B, INDEX_0);
    assert!(
        first == aliased,
        "get_hex(B) after get_hex(A) should hit the same cache entry"
    );
    let _edge = get_hex(&mut galaxy, C, INDEX_1);
    assert_eq!(
        galaxy.hexes.len(),
        2,
        "A and B share index (0, 0, 0) and C adds index (1, 0, 0)"
    );
    assert!(galaxy.hexes.contains_key(&INDEX_0));
    assert!(galaxy.hexes.contains_key(&INDEX_1));
    assert!(!galaxy.hexes.contains_key(&INDEX_2));
}

#[test]
fn schedule_replay_and_order_controls() {
    let settings = fixture_settings();
    let first = replay_schedule(&settings, &[(A, INDEX_0), (B, INDEX_0), (C, INDEX_1)]);
    let second = replay_schedule(&settings, &[(A, INDEX_0), (B, INDEX_0), (C, INDEX_1)]);
    assert_same_schedule_equal(&first, &second, "same A-B-C replay");
    let ac = replay_schedule(&settings, &[(A, INDEX_0), (C, INDEX_1)]);
    let ca = replay_schedule(&settings, &[(C, INDEX_1), (A, INDEX_0)]);
    assert_same_schedule_equal(&ac, &ca, "distinct A-C vs C-A");
    let unit = fixture_settings_with_hex_size((1, 1, 1));
    let abc = replay_schedule(&unit, &[(A, INDEX_0), (B, INDEX_1), (C, INDEX_2)]);
    let cba = replay_schedule(&unit, &[(C, INDEX_2), (B, INDEX_1), (A, INDEX_0)]);
    assert_same_schedule_equal(&abc, &cba, "unit-size A-B-C vs C-B-A");
    assert_ne!(ac[&A].index, ac[&C].index);
    assert_eq!(abc[&A].index, INDEX_0);
    assert_eq!(abc[&B].index, INDEX_1);
}

#[test]
fn boundary_faces_are_rejected_without_cache_mutation() {
    let mut galaxy = generate_fixture_galaxy(&fixture_settings());
    let empty_cache = galaxy.hexes.clone();
    for &outside in &OUTSIDE_FACES {
        let before = galaxy.hexes.clone();
        assert!(
            matches!(
                galaxy.get_hex(outside),
                Err(GenerationError::InvalidCoordinates(coord)) if coord == outside
            ),
            "outside face {outside} must be rejected"
        );
        assert!(
            before == galaxy.hexes,
            "rejected get_hex({outside}) mutated the cache"
        );
    }
    assert!(
        empty_cache == galaxy.hexes,
        "the cache should still be empty"
    );
    let _start = get_hex(&mut galaxy, START, INDEX_0);
    let _end = get_hex(&mut galaxy, END, INDEX_1);
    let warmed_cache = galaxy.hexes.clone();
    assert_eq!(warmed_cache.len(), 2);
    for &outside in &OUTSIDE_FACES {
        let before = galaxy.hexes.clone();
        assert!(
            matches!(
                galaxy.get_hex(outside),
                Err(GenerationError::InvalidCoordinates(coord)) if coord == outside
            ),
            "outside face {outside} must stay rejected"
        );
        assert!(
            before == galaxy.hexes,
            "rejected get_hex({outside}) mutated the cache"
        );
    }
    assert!(
        warmed_cache == galaxy.hexes,
        "the warmed cache must be unchanged"
    );
}

#[test]
#[ignore = "Unqualified non-unit alias-independence hypothesis; run explicitly before selecting a contract"]
fn alias_first_access_data_equality_is_a_hypothesis() {
    let settings = fixture_settings();
    eprintln!("fixture settings: {settings:?}");
    // Fresh A-first context: A, a repeated A hit, then B (which hits the cache).
    let mut a_first = generate_fixture_galaxy(&settings);
    let a_hex = get_hex(&mut a_first, A, INDEX_0);
    let a_again = get_hex(&mut a_first, A, INDEX_0);
    assert!(a_hex == a_again, "repeated get_hex(A) should be identical");
    let b_after_a = get_hex(&mut a_first, B, INDEX_0);
    // Fresh B-first context: B, a repeated B hit, then A (which hits the cache).
    let mut b_first = generate_fixture_galaxy(&settings);
    let b_hex = get_hex(&mut b_first, B, INDEX_0);
    let b_again = get_hex(&mut b_first, B, INDEX_0);
    assert!(b_hex == b_again, "repeated get_hex(B) should be identical");
    let a_after_b = get_hex(&mut b_first, A, INDEX_0);
    // Exact fresh replay of EACH schedule before the cross-order comparison.
    let replay_ab = replay_schedule(&settings, &[(A, INDEX_0), (A, INDEX_0), (B, INDEX_0)]);
    let replay_ba = replay_schedule(&settings, &[(B, INDEX_0), (B, INDEX_0), (A, INDEX_0)]);
    assert_hexes_equal(&replay_ab[&A], &a_hex, "A-first replay vs A-first context");
    assert_hexes_equal(
        &replay_ab[&B],
        &b_after_a,
        "A-first replay vs A-first context",
    );
    assert_hexes_equal(&replay_ba[&B], &b_hex, "B-first replay vs B-first context");
    assert_hexes_equal(
        &replay_ba[&A],
        &a_after_b,
        "B-first replay vs B-first context",
    );
    // Cache identity: the second aliased access hits the cache in both contexts.
    assert_hexes_equal(&a_hex, &b_after_a, "B after A");
    assert_hexes_equal(&b_hex, &a_after_b, "A after B");
    // Record compact concrete identities and physical fields for both owned hexes.
    eprintln!("A-first owned hex: {}", describe_hex(&a_hex));
    eprintln!("B-first owned hex: {}", describe_hex(&b_hex));
    eprintln!("owned data equal: {}", a_hex == b_hex);
    // Hypothesis: the cached hex does not depend on which alias was accessed first.
    assert!(
        a_hex == b_hex,
        "alias first-access equality hypothesis failed\n  A-first: {}\n  B-first: {}",
        describe_hex(&a_hex),
        describe_hex(&b_hex),
    );
}
