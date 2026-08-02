# Documentation

How to use `planet_generator` to build universes, galaxies, sectors,
star systems and planets. Everything starts from a seed string: the same
seed always gives the same content, in debug and release builds alike.

## Install

```toml
[dependencies]
planet_generator = "0.0.7-pre-alpha"
```

The public API is exposed through the prelude:

```rust
use planet_generator::prelude::*;
```

## The pipeline

Generation works in layers, each one feeding the next. The typical
entry point when you want control over each step:

```rust
use std::rc::Rc;

// 1. Describe what you want.
let settings = GenerationSettings {
    seed: Rc::from("my universe"),
    ..Default::default()
};

// 2. Check the settings are usable.
settings.validate().expect("Invalid settings");

// 3. Generate, layer by layer.
let mut diagnostics = Diagnostics::default();
let universe = Universe::generate(&settings, &mut diagnostics);
let neighborhood = GalacticNeighborhood::generate(universe, &settings, &mut diagnostics);
let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);

// 4. Pick a spot, in parsecs from the galaxy's center.
let coord = SpaceCoordinates::new(0, 0, 0);
let hex = galaxy.get_hex(coord)?.clone(); // generating the hex also generates its star systems
```

Hexes and divisions are generated lazily and cached on the galaxy, so
`get_hex` and `get_division_at_level` take `&mut self` — the first call
at a coordinate does the work, the next ones are lookups.

To generate a single system yourself:

```rust
let sub_sector = galaxy.get_division_at_level(coord, 1)?.clone();
let system = StarSystem::generate(0, coord, &hex, &sub_sector, &mut galaxy)?;
println!("{}", system.name);
```

## One-shot generation

If you just want everything at once:

```rust
let universe = Generator::generate(settings)?;
for galaxy in &universe.galaxies {
    println!("{}", galaxy);
}
```

`Generator::generate` validates the settings for you and returns
`Err(GenerationError::InvalidSettings(..))` listing every problem found.

## Settings

`GenerationSettings` is one struct with a section per layer. The knobs
you'll reach for most:

```rust
let settings = GenerationSettings {
    seed: Rc::from("42"),
    universe: UniverseSettings {
        use_ours: true,        // our universe's age and era
        fixed_age: None,       // or Some(13.8) to force an age in billions of years
        ..Default::default()
    },
    galaxy: GalaxySettings {
        use_ours: true,        // local group galaxies (Milky Way, Andromeda...)
        fixed_category: None,  // or e.g. Some(GalaxyCategory::Spiral(30000, 1000))
        ..Default::default()
    },
    sector: SectorSettings {
        hex_size: (4, 4, 4),   // parsecs per map hex
        flat_map: true,        // single z level; set hex_size z > 1 for some height
        ..Default::default()
    },
    system: SystemSettings {
        only_interesting: false, // retry systems until one has a Terrestrial or Ocean world
        ..Default::default()
    },
    ..Default::default()
};
```

Ranges are enforced by `validate()`; the docs on each field give the
details. A galaxy is only a bounding box with a shape and a density —
with a small `Irregular` category and your own SVG map (below), it works
just as well to model a single sector.

## Custom maps from SVG

`galactic_map_layers` takes SVG strings describing which region is
where. Paint zones with these exact colors:

| Color | Region | Color | Region |
|---|---|---|---|
| `#FFFFFF` | Core | `#008000` | Ellipse |
| `#FFFF00` | Nucleus | `#800080` | Halo |
| `#FFA500` | Bulge | `#FF00FF` | Aura |
| `#FF0000` | Bar | `#000000` | Void |
| `#0000FF` | Arm | `#808000` | GlobularCluster |
| `#008080` | Disk | `#00FFFF` | OpenCluster |
| `#808080` | Exile | `#000080` | Association |
| `#00FF00` | Stream | | |

Everything unpainted counts as Void (sparse, occasional stray stars).
Regions drive both the density of star systems and their age and
population, so a painted arm behaves like an arm.

Rules of thumb:

- Solid fills only, with the exact colors above. No gradients, no text
  labels. Give the SVG an explicit size or viewBox; it is stretched to
  the galaxy's bounds, with the top edge being the maximum galactic y.
- Pass several layers to give the map height: first layer is the top
  (max z), last is the bottom. With one layer, z is ignored.
- With no layers at all, a built-in default map for the galaxy's
  category is used instead.

A minimal working example ships in the crate as `EXAMPLE_REGION_MAP_SVG`.

## Errors

Fallible calls return `Result<_, GenerationError>`:

```rust
match galaxy.get_hex(coord) {
    Ok(hex) => println!("{} system(s)", hex.contents.len()),
    Err(GenerationError::InvalidCoordinates(c)) => println!("out of bounds: {}", c),
    Err(e) => return Err(e),
}
```

Library code never panics; internal invariants surface as
`GenerationError::InternalInvariant(..)` and configuration problems are
caught by `validate()` with a `SettingsError` per issue.

## Warnings

Non-fatal problems are collected instead of stopping generation. They
land on the object that produced them:

```rust
for warning in &galaxy.diagnostics.warnings {
    println!("{}", warning);
}
```

Typical cases: an SVG layer that failed to parse (the category's
default map is used instead), pixels that match no palette color, an
out-of-range age that was clamped.

## Serialization

Everything generated derives `Serialize`/`Deserialize`:

```rust
let json = serde_json::to_string(&galaxy)?;
let galaxy_back: Galaxy = serde_json::from_str(&json)?;
```

Parsed caches (the region map, diagnostics) are not serialized; they
rebuild themselves automatically on the next query. Two consequences:
a round-tripped object can compare unequal to the original until its
caches are rebuilt, and the first map query on a deserialized galaxy
regenerates its region map, clearing the deserialized hex and division
caches (they regenerate on demand, deterministically).

## Threading

The library is deliberately single-threaded: generated objects use
`Rc<str>` and `RefCell`, so they are `!Send` and `!Sync`. Generate on
one thread; to use the results elsewhere (another thread, a game
engine, an FFI boundary), serialize them and deserialize on the other
side.
