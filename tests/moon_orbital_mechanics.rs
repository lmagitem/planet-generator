//! Seeded end-to-end probe for cleanup-campaign Batch C, fix C1: a moon's
//! orbital period must be computed against the parent PLANET's mass, not the
//! star's (`moon/generator.rs` passed `solar_mass_to_earth_mass(ctx.star_mass)`
//! to the orbit-completion helpers at both call sites).
//!
//! Per campaign rules this is a seeded probe asserting a physical property
//! (Kepler's third law around the parent planet), never exact values. It fails
//! on the pre-fix code: with the star's mass as the orbited mass, stored moon
//! periods come out sqrt(M_star / M_planet) too short (e.g. ~0.05 days instead
//! of ~27.3 days for an Earth-Moon analog around a Sun-like star).

use planet_generator::prelude::*;
use std::rc::Rc;

/// Kepler period in days, replicating
/// `calculate_orbital_period_from_earth_masses` (masses in Earth masses,
/// distance in AU) so the probe stays on public types only.
fn kepler_period_days(orbital_radius_au: f64, mass1_earth: f64, mass2_earth: f64) -> f64 {
    let combined_solar_masses = (mass1_earth + mass2_earth) / 333000.0;
    (orbital_radius_au.powi(3) / combined_solar_masses).sqrt() * 365.256
}

/// The mass (in Earth masses) of a body-holding orbital point, if it is one.
fn body_mass(point: &OrbitalPoint) -> Option<f64> {
    match &point.object {
        AstronomicalObject::TelluricBody(body)
        | AstronomicalObject::IcyBody(body)
        | AstronomicalObject::GaseousBody(body) => Some(body.mass),
        _ => None,
    }
}

struct MoonSample {
    system_name: String,
    moon_id: u32,
    orbit_distance_au: f64,
    moon_mass: f64,
    planet_mass: f64,
    stored_period_days: f32,
}

/// Collects every moon in a system: bodies are laid out flat in
/// `all_objects`, so a moon is any body whose `own_orbit.primary_body_id`
/// points at ANOTHER body (a planet) rather than at a star or barycentre.
fn collect_moons(system: &StarSystem, out: &mut Vec<MoonSample>) {
    for point in &system.all_objects {
        let Some(moon_mass) = body_mass(point) else {
            continue;
        };
        let Some(orbit) = &point.own_orbit else {
            continue;
        };
        let Some(primary) = system.get_point(orbit.primary_body_id) else {
            continue;
        };
        let Some(planet_mass) = body_mass(primary) else {
            continue;
        };
        if orbit.orbital_period > 0.0 && planet_mass > 0.0 && moon_mass > 0.0 {
            out.push(MoonSample {
                system_name: system.name.to_string(),
                moon_id: point.id,
                orbit_distance_au: orbit.average_distance,
                moon_mass,
                planet_mass,
                stored_period_days: orbit.orbital_period,
            });
        }
    }
}

/// Probes a FIXED list of (seed, coord) candidates and aggregates every moon
/// found in the generated systems. The probe list must stay stable: changing
/// it re-rolls the cases. As written it collects 17 moons (moonlets of a small
/// rocky world plus a gas giant's full moon system); the count assertion below
/// guards against the probe silently degrading to vacuity.
#[test]
fn moon_orbital_periods_are_computed_against_the_planet_mass() {
    const PROBES: &[(&str, (i64, i64, i64))] = &[
        ("golden-moon-0", (0, 0, 0)),
        ("golden-moon-1", (0, 0, 0)),
        ("golden-moon-2", (0, 0, 0)),
    ];

    let mut moons = Vec::new();
    for &(seed, (x, y, z)) in PROBES {
        // `only_interesting` retries generation until the system holds a
        // Terrestrial/Ocean world, guaranteeing planetary content per probe.
        let settings = GenerationSettings {
            seed: Rc::from(seed),
            system: SystemSettings {
                only_interesting: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let universe = Universe::generate(&settings, &mut Diagnostics::default());
        let neighborhood =
            GalacticNeighborhood::generate(universe, &settings, &mut Diagnostics::default());
        let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
        let coord = SpaceCoordinates::new(x, y, z);
        let hex = galaxy
            .get_hex(coord)
            .unwrap_or_else(|e| panic!("get_hex({coord}) should succeed: {e}"));
        for system in &hex.contents {
            collect_moons(system, &mut moons);
        }
    }

    assert!(
        moons.len() >= 3,
        "The probe silently degraded: only {} moon(s) found across the fixed probe list. \
         Extend the probe list (a deliberate, commit-message-worthy change).",
        moons.len()
    );

    let mut violations = Vec::new();
    for moon in &moons {
        let expected = kepler_period_days(moon.orbit_distance_au, moon.planet_mass, moon.moon_mass);
        let stored = f64::from(moon.stored_period_days);
        let relative_error = (stored - expected).abs() / expected;
        eprintln!(
            "moon {:>3} in {:<24} a={:.5} AU, planet={:.2} M earth, moon={:.4} M earth: \
             stored {:.4} d vs Kepler-around-planet {:.4} d (rel err {:.1}%)",
            moon.moon_id,
            moon.system_name,
            moon.orbit_distance_au,
            moon.planet_mass,
            moon.moon_mass,
            stored,
            expected,
            relative_error * 100.0,
        );
        // The stored period is an f32 cast of the same f64 computation, so a
        // 1% band is generous; the star-mass bug is off by a factor of
        // sqrt(M_star/M_planet), i.e. at least ~2x and typically 20-600x.
        if relative_error > 0.01 {
            violations.push(format!(
                "moon {} in {}: stored {:.4} d, expected {:.4} d around its {:.2} M earth planet",
                moon.moon_id, moon.system_name, stored, expected, moon.planet_mass
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "Moon orbital period(s) not computed against the parent planet's mass:\n{}",
        violations.join("\n")
    );
}
