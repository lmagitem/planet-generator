//! Criterion benchmarks for the generation hot paths.
//!
//! Modelled on the `generate_example_systems` test in `src/lib.rs`: fixed seed, default
//! settings with `use_ours` universe/galaxy, no `only_interesting` filtering.

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use planet_generator::prelude::*;
use std::rc::Rc;
use std::time::Duration;

const BENCH_SEED: &str = "bench-fixed-seed";

/// Settings matching the example-systems test: fixed seed, ours-based universe and galaxy.
fn bench_settings() -> GenerationSettings {
    GenerationSettings {
        seed: Rc::from(BENCH_SEED),
        universe: UniverseSettings {
            use_ours: true,
            ..Default::default()
        },
        galaxy: GalaxySettings {
            use_ours: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Runs the full chain: universe, galactic neighborhood, galaxy.
fn build_galaxy(settings: &GenerationSettings) -> Galaxy {
    let universe = Universe::generate(settings, &mut Diagnostics::default());
    let neighborhood =
        GalacticNeighborhood::generate(universe, settings, &mut Diagnostics::default());
    Galaxy::generate(neighborhood, 0, settings)
}

fn bench_galaxy_generation(c: &mut Criterion) {
    c.bench_function("galaxy_generation", |b| {
        b.iter_batched(
            bench_settings,
            |settings| black_box(build_galaxy(&settings)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_hex_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("hex_generation");
    let settings = bench_settings();
    for (x, y, z) in [(0, 0, 0), (50, 10, 0), (-120, -30, 0)] {
        group.bench_function(format!("({x},{y},{z})"), |b| {
            b.iter_batched(
                || build_galaxy(&settings),
                |mut galaxy| {
                    black_box(
                        galaxy
                            .get_hex(black_box(SpaceCoordinates::new(x, y, z)))
                            .expect("Should have returned an hex.")
                            .clone(),
                    )
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn bench_system_generation(c: &mut Criterion) {
    let settings = bench_settings();
    let coord = SpaceCoordinates::new(0, 0, 0);
    let base_galaxy = build_galaxy(&settings);
    c.bench_function("system_generation", |b| {
        b.iter_batched(
            || {
                let mut galaxy = base_galaxy.clone();
                let sub_sector = galaxy
                    .get_division_at_level(coord, 1)
                    .expect("Should have returned a sub-sector.")
                    .clone();
                let hex = galaxy
                    .get_hex(coord)
                    .expect("Should have returned an hex.")
                    .clone();
                (galaxy, hex, sub_sector)
            },
            |(mut galaxy, hex, sub_sector)| {
                black_box(StarSystem::generate(
                    0,
                    black_box(coord),
                    &hex,
                    &sub_sector,
                    &mut galaxy,
                ))
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_region_map(c: &mut Criterion) {
    let start = SpaceCoordinates::new(-10000, -10000, -50);
    let end = SpaceCoordinates::new(10000, 10000, 50);
    let mut group = c.benchmark_group("region_lookup");

    group.bench_function("build", |b| {
        b.iter(|| {
            black_box(
                GalacticRegionMap::build(
                    &[DEFAULT_REGION_MAP_SPIRAL.to_string()],
                    black_box(start),
                    black_box(end),
                )
                .expect("Should have built a region map."),
            )
        })
    });

    let map = GalacticRegionMap::build(&[DEFAULT_REGION_MAP_SPIRAL.to_string()], start, end)
        .expect("Should have built a region map.");
    group.bench_function("region_at", |b| {
        b.iter(|| black_box(map.region_at(black_box(SpaceCoordinates::new(3500, -1200, 0)))))
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(2))
        .warm_up_time(Duration::from_millis(500));
    targets = bench_galaxy_generation, bench_hex_generation, bench_system_generation, bench_region_map
}
criterion_main!(benches);
