//! Golden-output determinism harness (Phase 0 of the cleanup campaign).
//!
//! What this proves: for a fixed set of seeds and generation settings, the full
//! generation pipeline (universe -> neighborhood -> galaxy -> hexes/systems ->
//! divisions) produces bit-identical serialized output across runs. The exact
//! seed/step strings and roll order at every `SeededDiceRoller::new` site are
//! the crate's determinism contract; they cannot be unit-tested directly, so
//! each case serializes the generated artifacts with `serde_json` and reduces
//! them to a single FNV-1a (u64) hash, compared against a blessed golden file
//! (`tests/golden/determinism.txt`).
//!
//! Blessing: run `BLESS_GOLDEN=1 cargo test --test determinism` to (re)write
//! the golden file with the current hashes. Blessing is ONLY legitimate right
//! after an intentional behavior change, and that re-bless must be called out
//! explicitly in the commit message. Any other hash change is a determinism
//! regression: do not bless it, fix the change instead.
//!
//! The harness also regenerates the `default_pipeline` case twice from scratch
//! within the same run and asserts identical hashes, guarding against
//! HashMap-iteration-order or float-formatting nondeterminism inside the
//! pipeline itself.
//!
//! Build profiles: the golden file pins BOTH debug and `--release` output —
//! `cargo test --test determinism` and `cargo test --test determinism --release`
//! must both pass against it. That was not always true: `powi`/`powf` calls with
//! small constant integer exponents are lowered differently by LLVM depending on
//! the profile (release inlines/const-folds multiplication chains, debug calls
//! libm), which shifted e.g. star zone boundaries by one ULP. Those calls were
//! replaced by explicit multiplications (see `calculate_distance_for_temperature`
//! in src/system/contents/utils.rs); do not reintroduce `powi`/`powf` with
//! integer literal exponents on pipeline values — non-integer exponents stay
//! libm calls in both profiles and are fine.

use planet_generator::prelude::*;
use std::collections::BTreeMap;
use std::rc::Rc;

const GOLDEN_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/determinism.txt");

/// Inline SVG region map copied from the e2e test in `src/lib.rs`: the left
/// half of the galaxy is painted Arm (#0000FF), the right half stays empty.
const HALF_ARM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="50" height="100" fill="#0000FF" /></svg>"##;
/// Full-map Exile gray.
const FULL_EXILE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="#808080" /></svg>"##;

/// FNV-1a (64-bit) over the given bytes, same flavor as the golden-hash
/// precedent in `src/galaxy/map/region_map/generator.rs`.
fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// Serializes one artifact to JSON and folds it into the case's hash input.
fn push_artifact<T: serde::Serialize>(input: &mut String, artifact: &T) {
    input.push_str(
        &serde_json::to_string(artifact).expect("Generated artifacts should serialize to JSON."),
    );
    input.push('\n');
}

/// A generated galaxy plus everything the case pulled out of it.
struct PipelineOutput {
    universe: Universe,
    neighborhood: GalacticNeighborhood,
    galaxy: Galaxy,
    hexes: Vec<GalacticHex>,
    divisions: Vec<GalacticMapDivision>,
}

/// Runs the standard pipeline: universe -> neighborhood -> galaxy, then for
/// each coordinate the hex (which generates its star systems) and the level-1
/// and level-2 divisions.
fn run_pipeline(
    settings: &GenerationSettings,
    galaxy_index: u16,
    coords: &[(i64, i64, i64)],
) -> PipelineOutput {
    // `Universe` and `GalacticNeighborhood` are `Copy`, so the originals stay
    // available for hashing after being passed into the next generation step.
    let universe = Universe::generate(settings, &mut Diagnostics::default());
    let neighborhood =
        GalacticNeighborhood::generate(universe, settings, &mut Diagnostics::default());
    let mut galaxy = Galaxy::generate(neighborhood, galaxy_index, settings);

    let mut hexes = Vec::new();
    let mut divisions = Vec::new();
    for &(x, y, z) in coords {
        let coord = SpaceCoordinates::new(x, y, z);
        let hex = galaxy
            .get_hex(coord)
            .unwrap_or_else(|e| panic!("get_hex({coord}) should succeed: {e}"))
            .clone();
        hexes.push(hex);
        for level in 1..=2 {
            divisions.push(
                galaxy
                    .get_division_at_level(coord, level)
                    .unwrap_or_else(|e| {
                        panic!("get_division_at_level({coord}, {level}) should succeed: {e}")
                    })
                    .clone(),
            );
        }
    }

    PipelineOutput {
        universe,
        neighborhood,
        galaxy,
        hexes,
        divisions,
    }
}

/// Hashes a pipeline output: every artifact, caches included, in a fixed order.
fn hash_pipeline_output(output: &PipelineOutput) -> u64 {
    let mut input = String::new();
    push_artifact(&mut input, &output.universe);
    push_artifact(&mut input, &output.neighborhood);
    push_artifact(&mut input, &output.galaxy);
    push_artifact(&mut input, &output.hexes);
    push_artifact(&mut input, &output.divisions);
    fnv1a(input.as_bytes())
}

/// Default settings, seed "golden-default", five hexes across the galaxy.
/// The last two coordinates were added because the first three land on empty
/// hexes (no star systems) for this seed; (100, 100, 0) and (-500, -500, 0)
/// each generate a system with two stars, so the default pipeline also covers
/// system generation and binary relations.
fn case_default_pipeline() -> u64 {
    let settings = GenerationSettings {
        seed: Rc::from("golden-default"),
        ..Default::default()
    };
    hash_pipeline_output(&run_pipeline(
        &settings,
        0,
        &[
            (0, 0, 0),
            (37, -12, 0),
            (-120, 45, 0),
            (100, 100, 0),
            (-500, -500, 0),
        ],
    ))
}

/// Our own universe and galaxy (`use_ours`), seed "golden-ours". Covers two
/// entries of the filled local-group table: the Milky Way (index 0) and the
/// Large Magellanic Cloud (index 4, a minor Irregular member), so both the
/// major-galaxy and the filled minor-galaxy paths of `use_ours` have golden
/// coverage. For the LMC, (0, 0, 0) is legitimately empty and (1000, -1000, 0)
/// generates one system. The universe and neighborhood are identical for both
/// galaxies (same settings), so they are hashed once, before each galaxy's
/// (galaxy, hexes, divisions) triple.
fn case_ours_pipeline() -> u64 {
    let settings = GenerationSettings {
        seed: Rc::from("golden-ours"),
        universe: UniverseSettings {
            use_ours: true,
            ..Default::default()
        },
        galaxy: GalaxySettings {
            use_ours: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut input = String::new();
    let milky_way = run_pipeline(&settings, 0, &[(0, 0, 0), (500, 200, 0)]);
    push_artifact(&mut input, &milky_way.universe);
    push_artifact(&mut input, &milky_way.neighborhood);
    push_artifact(&mut input, &milky_way.galaxy);
    push_artifact(&mut input, &milky_way.hexes);
    push_artifact(&mut input, &milky_way.divisions);

    let lmc = run_pipeline(&settings, 4, &[(0, 0, 0), (1000, -1000, 0)]);
    push_artifact(&mut input, &lmc.galaxy);
    push_artifact(&mut input, &lmc.hexes);
    push_artifact(&mut input, &lmc.divisions);

    fnv1a(input.as_bytes())
}

/// Only "interesting" systems are kept, seed "golden-interesting".
fn case_interesting_systems() -> u64 {
    let settings = GenerationSettings {
        seed: Rc::from("golden-interesting"),
        system: SystemSettings {
            only_interesting: true,
            ..Default::default()
        },
        ..Default::default()
    };
    hash_pipeline_output(&run_pipeline(&settings, 0, &[(0, 0, 0)]))
}

/// A fixed-size irregular galaxy with a user-provided single-layer SVG region
/// map (left half Arm, right half empty), seed "golden-svg".
fn case_svg_region_map() -> u64 {
    let settings = GenerationSettings {
        seed: Rc::from("golden-svg"),
        galaxy: GalaxySettings {
            fixed_category: Some(GalaxyCategory::Irregular(500, 500, 100)),
            galactic_map_layers: vec![HALF_ARM_SVG.to_string()],
            ..Default::default()
        },
        ..Default::default()
    };
    hash_pipeline_output(&run_pipeline(&settings, 0, &[(-200, 0, 0), (200, 0, 0)]))
}

/// A fixed-size irregular galaxy painted entirely with the Exile region color
/// (#808080), seed "golden-exile". This is the only case exercising the Exile
/// arm of the stellar-neighborhood age modifiers.
fn case_exile_region_map() -> u64 {
    let settings = GenerationSettings {
        seed: Rc::from("golden-exile"),
        galaxy: GalaxySettings {
            fixed_category: Some(GalaxyCategory::Irregular(500, 500, 100)),
            galactic_map_layers: vec![FULL_EXILE_SVG.to_string()],
            ..Default::default()
        },
        ..Default::default()
    };
    hash_pipeline_output(&run_pipeline(&settings, 0, &[(0, 0, 0), (100, 100, 0)]))
}

/// A fixed Cluster neighborhood (1 dominant, 2 major, 5 minor galaxies), seed
/// "golden-cluster". Generates ALL 8 galaxies the way `Generator::generate`'s
/// galaxies loop does, then pulls one hex at (0, 0, 0) — plus its level-1/2
/// divisions — from the dominant galaxy (index 0). This is the only case
/// covering multi-galaxy generation and the dominant/is_major branches of the
/// galaxy category and size tables (dominant galaxies roll on the
/// DominantElliptical-flavored weights).
fn case_cluster_dominant() -> u64 {
    let settings = GenerationSettings {
        seed: Rc::from("golden-cluster"),
        galaxy: GalaxySettings {
            fixed_neighborhood: Some(GalacticNeighborhoodDensity::Cluster(1, 2, 5)),
            ..Default::default()
        },
        ..Default::default()
    };
    let universe = Universe::generate(&settings, &mut Diagnostics::default());
    let neighborhood =
        GalacticNeighborhood::generate(universe, &settings, &mut Diagnostics::default());

    // Mirrors `generate_galaxies` in src/generator/mod.rs: the galaxy count is
    // derived from the neighborhood density and each galaxy is generated by index.
    let galaxy_count = match neighborhood.density {
        GalacticNeighborhoodDensity::Void(g, m) | GalacticNeighborhoodDensity::Group(g, m) => {
            u16::from(g) + m
        }
        GalacticNeighborhoodDensity::Cluster(d, g, m) => u16::from(d) + u16::from(g) + m,
    };
    let mut galaxies: Vec<Galaxy> = (0..galaxy_count)
        .map(|index| Galaxy::generate(neighborhood, index, &settings))
        .collect();
    assert_eq!(
        galaxies.len(),
        8,
        "Cluster(1, 2, 5) should yield 1 dominant + 2 major + 5 minor galaxies."
    );

    let coord = SpaceCoordinates::new(0, 0, 0);
    let hex = galaxies[0]
        .get_hex(coord)
        .unwrap_or_else(|e| panic!("get_hex({coord}) on the dominant galaxy should succeed: {e}"))
        .clone();
    let mut divisions = Vec::new();
    for level in 1..=2 {
        divisions.push(
            galaxies[0]
                .get_division_at_level(coord, level)
                .unwrap_or_else(|e| {
                    panic!("get_division_at_level({coord}, {level}) should succeed: {e}")
                })
                .clone(),
        );
    }

    let mut input = String::new();
    push_artifact(&mut input, &universe);
    push_artifact(&mut input, &neighborhood);
    push_artifact(&mut input, &galaxies);
    push_artifact(&mut input, &hex);
    push_artifact(&mut input, &divisions);
    fnv1a(input.as_bytes())
}

/// One galaxy per `GalaxyCategory` variant (all 7), each pinned through
/// `fixed_category` at that variant's default sizes, seed "golden-category",
/// one hex at (0, 0, 0) plus its level-1/2 divisions each. All seven fold into
/// ONE case hash: the universe and neighborhood are identical across the sweep
/// (`fixed_category` does not affect them), so they are hashed once, followed
/// by each (galaxy, hexes, divisions) triple in variant order. This covers the
/// per-category default region maps and sub-category tables end to end. Some
/// center hexes may legitimately be empty; that is part of the blessed output.
fn case_category_sweep() -> u64 {
    const CATEGORIES: [GalaxyCategory; 7] = [
        GalaxyCategory::Intergalactic(1000, 3000, 1000),
        GalaxyCategory::Irregular(3000, 3000, 2000),
        GalaxyCategory::Spiral(10000, 100),
        GalaxyCategory::Lenticular(10000, 600),
        GalaxyCategory::Elliptical(10000),
        GalaxyCategory::Intracluster(1, 3, 1),
        GalaxyCategory::DominantElliptical(300000),
    ];

    let mut input = String::new();
    for (position, category) in CATEGORIES.iter().enumerate() {
        let settings = GenerationSettings {
            seed: Rc::from("golden-category"),
            galaxy: GalaxySettings {
                fixed_category: Some(*category),
                ..Default::default()
            },
            ..Default::default()
        };
        let output = run_pipeline(&settings, 0, &[(0, 0, 0)]);
        if position == 0 {
            push_artifact(&mut input, &output.universe);
            push_artifact(&mut input, &output.neighborhood);
        }
        push_artifact(&mut input, &output.galaxy);
        push_artifact(&mut input, &output.hexes);
        push_artifact(&mut input, &output.divisions);
    }
    fnv1a(input.as_bytes())
}

/// One galaxy + one hex at (0, 0, 0) under a universe pinned to the given era.
fn era_case(era: StelliferousEra, seed: &'static str) -> u64 {
    let settings = GenerationSettings {
        seed: Rc::from(seed),
        universe: UniverseSettings {
            fixed_era: Some(era),
            ..Default::default()
        },
        ..Default::default()
    };
    hash_pipeline_output(&run_pipeline(&settings, 0, &[(0, 0, 0)]))
}

/// Universe pinned to the Late Stelliferous era (age in [50, 2000) billion
/// years), seed "golden-late-era" — chosen so the Older-trait roll in
/// `add_age_related_traits` fires (the galaxy gets [Older, ActiveNucleus]).
/// Also covers the late-era neighborhood weights.
fn case_late_stelliferous_era() -> u64 {
    era_case(StelliferousEra::LateStelliferous, "golden-late-era")
}

/// Universe pinned to the End Stelliferous era (age in [2000, 100000) billion
/// years), seed "golden-dead-galaxy" — chosen so the generated universe is old
/// enough (age 91635 > 50000) that BOTH age-related trait paths fire: the
/// galaxy gets [Older, Dead, Expansive(75)]. Also covers the end-era
/// neighborhood weights (no minor galaxies in clusters).
fn case_end_stelliferous_era() -> u64 {
    era_case(StelliferousEra::EndStelliferous, "golden-dead-galaxy")
}

/// Non-default sector settings, seed "golden-nondefault-sector": `flat_map`
/// off (3D map), `max_one_system_per_hex` off (several systems per hex
/// allowed) and `density_by_hex_instead_of_parsec` off (one density roll per
/// cubic parsec). `hex_size` is set to (2, 2, 2) so the per-parsec path
/// actually rolls 8 times per hex — with the default (1, 1, 1) it would
/// degenerate to a single roll, indistinguishable from the per-hex path.
///
/// Probes a FIXED list of coordinates (in order) and keeps the first hex with
/// at least one system, so the case always covers actual system generation.
/// The probe list must stay stable: changing it re-rolls the case. As blessed,
/// the FIRST probe (100, 100, 0) is accepted: it lands in a dense central
/// region and generates 2462 systems in a single hex — exactly the behavior
/// `max_one_system_per_hex: false` + per-parsec density unlock.
fn case_non_default_sector() -> u64 {
    const PROBES: &[(i64, i64, i64)] = &[
        (100, 100, 0),
        (-500, -500, 0),
        (250, -250, 4),
        (0, 0, 0),
        (37, -12, 0),
        (-120, 45, 0),
        (-50, 300, -3),
        (800, 0, 1),
        (0, -800, -1),
        (1000, 1000, 2),
    ];

    for &(x, y, z) in PROBES {
        let settings = GenerationSettings {
            seed: Rc::from("golden-nondefault-sector"),
            sector: SectorSettings {
                flat_map: false,
                max_one_system_per_hex: false,
                density_by_hex_instead_of_parsec: false,
                hex_size: (2, 2, 2),
                ..Default::default()
            },
            ..Default::default()
        };
        let output = run_pipeline(&settings, 0, &[(x, y, z)]);
        if !output.hexes[0].contents.is_empty() {
            return hash_pipeline_output(&output);
        }
    }

    panic!(
        "None of the fixed probe coords produced a populated hex; \
         extend the probe list (a deliberate, commit-message-worthy change)."
    );
}

/// Counts the stars among a system's orbital points.
fn star_count(system: &StarSystem) -> usize {
    system
        .all_objects
        .iter()
        .filter(|o| matches!(o.object, AstronomicalObject::Star(_)))
        .count()
}

/// Probes a FIXED list of (seed, coord) candidates until one yields a system
/// with at least two stars, so the binary-relations code paths are covered.
/// The probe list must stay stable: changing it re-rolls the case.
fn case_multi_star() -> u64 {
    const PROBES: &[(&str, (i64, i64, i64))] = &[
        ("golden-multi-0", (0, 0, 0)),
        ("golden-multi-1", (0, 0, 0)),
        ("golden-multi-2", (0, 0, 0)),
        ("golden-multi-3", (0, 0, 0)),
        ("golden-multi-4", (0, 0, 0)),
        ("golden-multi-5", (0, 0, 0)),
        ("golden-multi-6", (0, 0, 0)),
        ("golden-multi-7", (0, 0, 0)),
        ("golden-multi-8", (0, 0, 0)),
        ("golden-multi-9", (0, 0, 0)),
    ];

    for &(seed, (x, y, z)) in PROBES {
        let settings = GenerationSettings {
            seed: Rc::from(seed),
            ..Default::default()
        };
        let output = run_pipeline(&settings, 0, &[(x, y, z)]);
        let multi_star_system = output.hexes[0]
            .contents
            .iter()
            .find(|system| star_count(system) >= 2);
        if let Some(system) = multi_star_system {
            assert!(
                star_count(system) >= 2,
                "The multi-star case silently degraded to a single-star system."
            );
            let mut input = String::new();
            push_artifact(&mut input, &output.universe);
            push_artifact(&mut input, &output.neighborhood);
            push_artifact(&mut input, &output.galaxy);
            push_artifact(&mut input, &output.hexes);
            push_artifact(&mut input, &output.divisions);
            push_artifact(&mut input, system);
            return fnv1a(input.as_bytes());
        }
    }

    panic!(
        "None of the fixed probe seeds/coords produced a multi-star system; \
         extend the probe list (a deliberate, commit-message-worthy change)."
    );
}

/// Computes the hash of every case, in a fixed order.
fn compute_all_cases() -> BTreeMap<&'static str, u64> {
    let mut hashes = BTreeMap::new();
    hashes.insert("category_sweep", case_category_sweep());
    hashes.insert("cluster_dominant", case_cluster_dominant());
    hashes.insert("default_pipeline", case_default_pipeline());
    hashes.insert("end_stelliferous_era", case_end_stelliferous_era());
    hashes.insert("exile_region_map", case_exile_region_map());
    hashes.insert("interesting_systems", case_interesting_systems());
    hashes.insert("late_stelliferous_era", case_late_stelliferous_era());
    hashes.insert("multi_star", case_multi_star());
    hashes.insert("non_default_sector", case_non_default_sector());
    hashes.insert("ours_pipeline", case_ours_pipeline());
    hashes.insert("svg_region_map", case_svg_region_map());
    hashes
}

fn parse_golden_file(contents: &str) -> BTreeMap<String, u64> {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (name, hex) = line
                .split_once(" = ")
                .unwrap_or_else(|| panic!("Malformed golden line: {line:?}"));
            let value = u64::from_str_radix(
                hex.trim()
                    .strip_prefix("0x")
                    .unwrap_or_else(|| panic!("Golden hash should start with 0x: {line:?}")),
                16,
            )
            .unwrap_or_else(|e| panic!("Malformed golden hash in {line:?}: {e}"));
            (name.to_string(), value)
        })
        .collect()
}

#[test]
fn generated_output_is_stable_within_a_run() {
    let first = case_default_pipeline();
    let second = case_default_pipeline();
    assert_eq!(
        first, second,
        "NONDETERMINISM: default_pipeline produced different hashes when generated twice \
         from scratch in the same run (first: {first:#018x}, second: {second:#018x}). \
         This points at HashMap-iteration-order or float-formatting nondeterminism in the \
         pipeline itself and must be investigated, not blessed."
    );
}

#[test]
fn generated_output_matches_golden_hashes() {
    let actual = compute_all_cases();

    if std::env::var_os("BLESS_GOLDEN").is_some() {
        let mut contents = String::from(
            "# Golden hashes for tests/determinism.rs. Regenerate with:\n\
             #   BLESS_GOLDEN=1 cargo test --test determinism\n\
             # Only bless after an INTENTIONAL behavior change, and call the\n\
             # re-bless out in the commit message. Anything else is a regression.\n",
        );
        for (name, hash) in &actual {
            contents.push_str(&format!("{name} = {hash:#018x}\n"));
        }
        std::fs::write(GOLDEN_PATH, contents).expect("Should write the golden file.");
        eprintln!("BLESS_GOLDEN set: rewrote {GOLDEN_PATH} with current hashes.");
        return;
    }

    let golden_contents = std::fs::read_to_string(GOLDEN_PATH).unwrap_or_else(|e| {
        panic!(
            "Could not read golden file {GOLDEN_PATH}: {e}. \
             Run `BLESS_GOLDEN=1 cargo test --test determinism` first."
        )
    });
    let expected = parse_golden_file(&golden_contents);

    let mut mismatches = Vec::new();
    for (name, actual_hash) in &actual {
        match expected.get(*name) {
            Some(expected_hash) if expected_hash == actual_hash => {}
            Some(expected_hash) => mismatches.push(format!(
                "- {name} = {expected_hash:#018x} (expected)\n+ {name} = {actual_hash:#018x} (actual)"
            )),
            None => mismatches
                .push(format!("+ {name} = {actual_hash:#018x} (missing from golden file)")),
        }
    }
    for name in expected.keys() {
        if !actual.contains_key(name.as_str()) {
            mismatches.push(format!("- {name} (in golden file but no longer computed)"));
        }
    }

    assert!(
        mismatches.is_empty(),
        "Golden hash mismatch(es) — generated output changed:\n{}\n\nIf this change is \
         intentional, re-bless with `BLESS_GOLDEN=1 cargo test --test determinism` and call \
         the re-bless out in the commit message. Otherwise this is a determinism regression.",
        mismatches.join("\n")
    );
}
