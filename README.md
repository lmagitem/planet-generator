# Planet Generator

A Rust library that aims to generate galaxies, sectors, solar systems, planets with maps, and their inhabitants, along with tons of narrative elements and ideas.

The library is intended for use in game development to generate believable worlds for Roguelikes, 4X games, or any other project that sparks your imagination.

I've tried my best to use realistic formulas and up-to-date data when possible for generation. However, as I am not an astrophysicist myself, and my limited knowledge on the subject isn't sufficient to build something entirely accurate, I've compensated for my shortfalls by borrowing ideas from various other generators I have previously used and loved, mostly from RPGs. These include, but are not limited to: [the RTT Complicated Star System Generator](https://wiki.rpg.net/index.php/RTT_Worldgen), [Instant Universe](https://www.drivethrurpg.com/product/153512/Instant-Universe), and generators from various editions of [GURPS Traveller](https://en.wikipedia.org/wiki/GURPS_Traveller), [Stars Without Number](https://www.drivethrurpg.com/product/226996/Stars-Without-Number-Revised-Edition), [Rogue Trader](<https://en.wikipedia.org/wiki/Rogue_Trader_(role-playing_game)>) and [Alternity](https://en.wikipedia.org/wiki/Alternity).

## Example

An example of how to use this library can be found in [this project, a simple Actix server that serves generated results](https://github.com/lmagitem/galactic-scanner). A web app that displays the generation results using the previous project [is available here](https://galactic-explorer.n42c.dev/) - please note that it is also a work in progress, and not all library features are available yet.

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
  - [ ] Our local group galaxies
- [x] Sector and subsector generation
  - [x] Configurable divisions
  - [x] Hex and division calculations
  - [ ] Temporary region mapping
  - [ ] Proper region mapping
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
    - [ ] Multiple star system orbit eccentricity
  - [x] Orbital zones
  - [ ] Filling orbits
  - [ ] Orbit eccentricity
- [ ] Planet generation
  - [ ] Resources
    - [ ] Accessibility
    - [ ] Rarity
    - [ ] Quantity
  - [ ] Life presence
  - [ ] Points of interest
  - [ ] Map generation
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

Licensed under [MIT license](https://github.com/lmagitem/seeded-dice-roller/blob/master/LICENSE.md).
