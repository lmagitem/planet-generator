//! JSON round-trip regression test.
//!
//! Generated star systems store non-finite floats: `Orbit.day_length` uses
//! `f32::INFINITY` as a sentinel for empty orbit slots, binary-star barycentre
//! orbits and tide-locked bodies. `serde_json` writes those as `null`, which
//! plain `f32` deserialization used to reject — every generated system failed
//! the JSON round-trip. The `non_finite_f32` serde adapter on that field maps
//! `null` back to infinity; this test proves a full generated galaxy (systems
//! included) now round-trips.

use planet_generator::prelude::*;
use std::rc::Rc;

#[test]
fn a_generated_galaxy_with_systems_survives_a_json_round_trip() {
    let settings = GenerationSettings {
        seed: Rc::from("serde-round-trip"),
        ..Default::default()
    };
    let mut diagnostics = Diagnostics::default();
    let universe = Universe::generate(&settings, &mut diagnostics);
    let neighborhood = GalacticNeighborhood::generate(universe, &settings, &mut diagnostics);
    let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
    // Generate a few hexes so the galaxy actually holds star systems (the same
    // coordinates the determinism harness uses to reach system generation).
    for (x, y, z) in [(0, 0, 0), (100, 100, 0), (-500, -500, 0)] {
        galaxy
            .get_hex(SpaceCoordinates::new(x, y, z))
            .expect("The coordinates should be inside the galaxy.");
    }
    assert!(
        galaxy.hexes.values().any(|hex| !hex.contents.is_empty()),
        "The chosen coordinates should have generated at least one star system."
    );
    // `region_map` is serde-skipped (rebuilt lazily on the first map query) and
    // `diagnostics` is serde-skipped (transient by design), so compare both
    // sides without them.
    galaxy.region_map = None;
    galaxy.diagnostics = Diagnostics::default();

    let json = serde_json::to_string(&galaxy).expect("The galaxy should serialize.");
    let round_tripped: Galaxy =
        serde_json::from_str(&json).expect("The galaxy should deserialize.");

    assert_eq!(round_tripped, galaxy);
    assert_eq!(
        serde_json::to_string(&round_tripped).expect("The galaxy should re-serialize."),
        json,
        "Serializing the round-tripped galaxy should give the same JSON."
    );
}
