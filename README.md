# Planet Generator

A Rust library that aims to generate galaxies, sectors, solar systems, planets with maps, and their inhabitants, along with tons of narrative elements and ideas.

The library is intended for use in game development to generate believable worlds for Roguelikes, 4X games, or any other project that sparks your imagination.

**Thread safety:** this library is deliberately single-threaded. Generated objects use `Rc<str>` and `RefCell` internally, so `GeneratedUniverse`, `Galaxy`, `StarSystem` and everything below them are `!Send` and `!Sync`. Generate on one thread; to move results across threads or an FFI boundary, serialize them and deserialize on the other side.

I've tried my best to use realistic formulas and up-to-date data when possible for generation. However, as I am not an astrophysicist myself, and my limited knowledge on the subject isn't sufficient to build something entirely accurate, I've compensated for my shortfalls by borrowing ideas from various other generators I have previously used and loved, mostly from RPGs. These include, but are not limited to: [the RTT Complicated Star System Generator](https://wiki.rpg.net/index.php/RTT_Worldgen), [Instant Universe](https://www.drivethrurpg.com/product/153512/Instant-Universe), and generators from various editions of [GURPS Traveller](https://en.wikipedia.org/wiki/GURPS_Traveller), [Stars Without Number](https://www.drivethrurpg.com/product/226996/Stars-Without-Number-Revised-Edition), [Rogue Trader](<https://en.wikipedia.org/wiki/Rogue_Trader_(role-playing_game)>) and [Alternity](https://en.wikipedia.org/wiki/Alternity).

## Example

An example of how to use this library can be found in [this project, a simple Actix server that serves generated results](https://github.com/lmagitem/galactic-scanner). A web app that displays the generation results using the previous project [is available here](https://galactic-explorer.n42c.dev/) - please note that it is also a work in progress, and not all library features are available in the web app yet.

See [DOCUMENTATION.md](DOCUMENTATION.md) for a guided tour of the API with code examples.

## Custom galaxy maps

You can hand the generator an SVG to decide what a galaxy (or just a sector) looks like and how dense each part is. Paint zones with the documented palette colors, pass the file as a string, and the regions, densities and stellar populations follow:

```rust
let settings = GenerationSettings {
    galaxy: GalaxySettings {
        fixed_category: Some(GalaxyCategory::Irregular(500, 500, 100)),
        galactic_map_layers: vec![include_str!("my_map.svg").to_string()],
        ..Default::default()
    },
    ..Default::default()
};
```

Several layers can be stacked to give the map some height, and built-in defaults exist for the usual galaxy shapes when you don't provide one. The palette and authoring rules live in [DOCUMENTATION.md](DOCUMENTATION.md).

## Errors and warnings

The library does not panic. Check your settings upfront, then handle errors like anywhere else in Rust:

```rust
settings.validate()?;
let universe = Generator::generate(settings)?;
```

Smaller problems (an SVG that couldn't be parsed, an out-of-range age that was clamped...) don't stop generation; they're collected as typed warnings on the generated objects (`universe.diagnostics`, `galaxy.diagnostics`, `system.diagnostics`) so you can show or ignore them as you see fit.

## Determinism

The same seed always produces the same content, in debug and release builds alike. This is checked continuously by a golden test harness.

## Development

```bash
cargo test          # unit, integration and golden determinism tests
cargo bench         # criterion benchmarks on the generation hot paths
BLESS_GOLDEN=1 cargo test --test determinism   # re-bless goldens after an intentional behavior change
```

## Roadmap

This is the current roadmap of the library:

- [x] Universe generation
  - [x] Age
  - [x] Era
- [x] Galaxy generation
  - [x] Neighborhood
  - [x] Age
  - [x] Shape
  - [x] Peculiarities
  - [ ] Names
  - [x] Our local group galaxies
- [x] Sector and subsector generation
  - [x] Configurable divisions
  - [x] Hex and division calculations
  - [x] Temporary region mapping
  - [x] Proper region mapping
  - [ ] Names
- [ ] Star system generation
  - [x] Spawn chance according to density
  - [x] Stars generation
    - [x] Age
    - [x] Spectral type
    - [x] Luminosity
    - [ ] Subdwarfs
    - [x] Star differences according to population
    - [x] Name generation
    - [ ] Configurable stars
    - [ ] Peculiarities
    - [x] Multiple star system orbit eccentricity
    - [ ] Multiple star system orbit inclination
  - [ ] Peculiarities
  - [x] Orbital zones
  - [x] Filling orbits
  - [x] Orbit eccentricity
  - [ ] Orbit inclination
- [x] Planet generation
  - [x] Orbit parameters
  - [x] Moons
  - [x] World parameters and climate
  - [ ] Resources
    - [ ] Accessibility
    - [ ] Rarity
    - [ ] Quantity
  - [ ] Life presence
  - [ ] Points of interest
  - [ ] Map generation
  - [x] Proto planets
  - [ ] Exotic planets
- [ ] Species generation
  - [ ] Add species using the given settings
  - [ ] Spawn species using conditions found in specific systems
  - [ ] Writing the species' history
  - [ ] Filling the various systems with appropriate life
- [ ] Populated sectors/systems/planets
  - [ ] Add methods to generate populated objects "directly"

## Contribute

I'd be happy to receive issues requesting new features or reporting bug fixes. Feel free to point out areas where the code could be improved, whether in terms of performance, readability, documentation, or adherence to best practices, and/or submit pull requests yourselves.

##### License:

Licensed under [MIT license](LICENSE.md).
