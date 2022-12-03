# Planet Generator
A Rust library that generates galaxies, sectors, solar systems, planets with maps and their inhabitants alongside tons of narrative elements and ideas.

[An explorer](https://github.com/lmagitem/galactic-explorer) will allow to play with the generator. The library is also intended to be used in game development to generate believable worlds usable in Roguelikes, 4X or whatever suits your imagination.

As I am not an astrophysician myself and my limited knowledge on the matter isn't enough to build something of that level, the generator is put together using ideas shamelessly borrowed from various other generators I previously used and loved, mostly from RPGs. Those include but are not limited to : [the RTT Complicated Star System Generator](https://wiki.rpg.net/index.php/RTT_Worldgen), [Instant Universe](https://www.drivethrurpg.com/product/153512/Instant-Universe), and generators from various editions of [GURPS Traveller](https://en.wikipedia.org/wiki/GURPS_Traveller), [Stars Without Number](https://www.drivethrurpg.com/product/226996/Stars-Without-Number-Revised-Edition), [Rogue Trader](https://en.wikipedia.org/wiki/Rogue_Trader_(role-playing_game)) and [Alternity](https://en.wikipedia.org/wiki/Alternity).

## Roadmap
This is the current roadmap of the library:
- [x] Universe generation
    - [x] Age
    - [x] Era
- [ ] Galaxy generation
    - [x] Neighborhood
    - [x] Age
    - [x] Shape
    - [x] Peculiarities
    - [ ] Names
    - [ ] Our local group galaxies
- [ ] Sector and subsector generation
    - [x] Configurable divisions
    - [x] Hex and division calculations
    - [ ] Temporary region mapping
    - [ ] Proper region mapping
    - [ ] Names
- [ ] Star system generation
    - [ ] Spawn chance according to density
    - [ ] Stars generation
        - [ ] Age
        - [ ] Spectral type
        - [ ] Luminosity
    - [ ] Filling orbits
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
I'd be happy to receive issues asking for new features or bug fixes. Also feel free to point out where code could be improved (either in performance, readability, documentation, following best practices...) and/or make pull requests yourselves.

##### License:
Licensed under [MIT license](https://github.com/lmagitem/seeded-dice-roller/blob/master/LICENSE.md).
