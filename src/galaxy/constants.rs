#![allow(dead_code)]
#[cfg(test)]
use crate::galaxy::map::hex::types::SpaceCoordinates;
#[cfg(test)]
use crate::galaxy::map::region_map::types::GalacticRegionMap;
#[cfg(test)]
use crate::galaxy::map::types::GalacticRegion;
use crate::galaxy::types::GalaxyCategory;
use crate::galaxy::types::GalaxySpecialTrait;
use crate::galaxy::types::GalaxySubCategory;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

/// The current age of the Milky Way.
pub const OUR_GALAXYS_AGE: f32 = 13.61;
/// The name of our galaxy.
pub const OUR_GALAXYS_NAME: &str = "Milky Way";
/// The category of our galaxy.
pub const OUR_GALAXYS_CATEGORY: GalaxyCategory = GalaxyCategory::Spiral(16203, 160);
/// The sub-category of our galaxy.
pub const OUR_GALAXYS_SUB_CATEGORY: GalaxySubCategory = GalaxySubCategory::BarredSpiral;
/// A galaxy with no special trait.
pub const NO_SPECIAL_TRAIT: GalaxySpecialTrait = GalaxySpecialTrait::NoPeculiarity;
/// The width and height, in cells, of a rasterized galactic region map layer.
pub const REGION_GRID_RESOLUTION: usize = 256;
/// The real galaxies of our Local Group, used when `galaxy.use_ours` is set. Indices 0-2 are the
/// three spirals (the 2 majors of the `Group(2, 36)` density plus Triangulum); indices 3-37 are
/// minor members in rough order of distance from the Milky Way: its own satellites first, then the
/// Andromeda subsystem, then the more isolated dwarfs. Sizes are plausible spans in parsecs
/// (dwarf spheroidals as [Elliptical](GalaxyCategory::Elliptical) radii, dwarf irregulars as
/// [Irregular](GalaxyCategory::Irregular) length/width/height); ages are old, in billions of years.
pub const LOCAL_GROUP_GALAXIES: [GalaxyWithoutTraits; 38] = [
    GalaxyWithoutTraits {
        index: 0,
        name: OUR_GALAXYS_NAME,
        age: OUR_GALAXYS_AGE,
        is_dominant: false,
        is_major: true,
        category: OUR_GALAXYS_CATEGORY,
        sub_category: OUR_GALAXYS_SUB_CATEGORY,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 1,
        name: "Andromeda",
        age: -1.0,
        is_dominant: false,
        is_major: true,
        category: GalaxyCategory::Spiral(23280, 230),
        sub_category: OUR_GALAXYS_SUB_CATEGORY,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 2,
        name: "Triangulum",
        age: -1.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Spiral(9370, 94),
        sub_category: GalaxySubCategory::FlatSpiral,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    // Satellites of the Milky Way.
    GalaxyWithoutTraits {
        index: 3,
        name: "Sagittarius Dwarf Spheroidal",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(1000),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::Interacting,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 4,
        name: "Large Magellanic Cloud",
        age: 13.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(9500, 8500, 1500),
        sub_category: GalaxySubCategory::BarredSpiral,
        first_trait: GalaxySpecialTrait::Interacting,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 5,
        name: "Small Magellanic Cloud",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(5500, 4500, 2000),
        sub_category: GalaxySubCategory::Amorphous,
        first_trait: GalaxySpecialTrait::Interacting,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 6,
        name: "Ursa Minor Dwarf",
        age: 13.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(450),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::MetalPoor,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 7,
        name: "Draco Dwarf",
        age: 13.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(400),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::MetalPoor,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 8,
        name: "Sculptor Dwarf",
        age: 13.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(550),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::MetalPoor,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 9,
        name: "Sextans Dwarf",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(800),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::MetalPoor,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 10,
        name: "Carina Dwarf",
        age: 12.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(500),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 11,
        name: "Fornax Dwarf",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(1000),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::ExtendedHalo,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 12,
        name: "Leo II",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(400),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 13,
        name: "Leo I",
        age: 11.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(500),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 14,
        name: "Phoenix Dwarf",
        age: 12.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(600, 550, 300),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 15,
        name: "Leo T",
        age: 12.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(400, 350, 250),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: GalaxySpecialTrait::GasRich,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 16,
        name: "NGC 6822",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(2200, 2000, 600),
        sub_category: GalaxySubCategory::BarredSpiral,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    // The Andromeda subsystem.
    GalaxyWithoutTraits {
        index: 17,
        name: "NGC 185",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(1500),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 18,
        name: "NGC 147",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(1500),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 19,
        name: "IC 1613",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(2000, 1800, 500),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 20,
        name: "Andromeda I",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(600),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 21,
        name: "Andromeda III",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(600),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 22,
        name: "M32",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(1000),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::Compact(150),
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 23,
        name: "M110",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(4500),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 24,
        name: "Andromeda V",
        age: 13.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(500),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: GalaxySpecialTrait::MetalPoor,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 25,
        name: "Andromeda II",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(1500),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    // The more isolated members of the group.
    GalaxyWithoutTraits {
        index: 26,
        name: "Cetus Dwarf",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Elliptical(900),
        sub_category: GalaxySubCategory::DwarfElliptical,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 27,
        name: "Leo A",
        age: 12.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(800, 700, 300),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 28,
        name: "LGS 3",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(600, 500, 250),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 29,
        name: "IC 10",
        age: 12.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(1800, 1600, 600),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: GalaxySpecialTrait::Starburst,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 30,
        name: "Andromeda VIII",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(2200, 700, 300),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: GalaxySpecialTrait::Interacting,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 31,
        name: "Pegasus Dwarf",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(900, 800, 350),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 32,
        name: "WLM",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(2800, 2500, 800),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 33,
        name: "Aquarius Dwarf",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(700, 600, 300),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 34,
        name: "Sagittarius Dwarf Irregular",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(900, 800, 300),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 35,
        name: "Antlia Dwarf",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(800, 700, 300),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 36,
        name: "Sextans B",
        age: 12.5,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(1600, 1400, 500),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
    GalaxyWithoutTraits {
        index: 37,
        name: "Sextans A",
        age: 12.0,
        is_dominant: false,
        is_major: false,
        category: GalaxyCategory::Irregular(1800, 1600, 500),
        sub_category: GalaxySubCategory::DwarfAmorphous,
        first_trait: NO_SPECIAL_TRAIT,
        second_trait: NO_SPECIAL_TRAIT,
        third_trait: NO_SPECIAL_TRAIT,
    },
];

/// Data used to calculate a universe's age.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalaxyWithoutTraits<'a> {
    /// The numeric identifier of this galaxy in its neighborhood.
    pub index: u16,
    /// The name of this galaxy.
    pub name: &'a str,
    /// The age of this galaxy in billions of years.
    pub age: f32,
    /// Is this galaxy a dominant one in its cluster?
    pub is_dominant: bool,
    /// Is this galaxy a major one in its neighborhood?
    pub is_major: bool,
    /// In what category this galaxy belongs to.
    pub category: GalaxyCategory,
    /// In what sub-category this galaxy belongs to.
    pub sub_category: GalaxySubCategory,
    /// The first peculiarity this galaxy has, if any.
    pub first_trait: GalaxySpecialTrait,
    /// The second peculiarity this galaxy has, if any.
    pub second_trait: GalaxySpecialTrait,
    /// The third peculiarity this galaxy has, if any.
    pub third_trait: GalaxySpecialTrait,
}
/// Pairs traits that are incompatible with one another.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub struct OppositeTraits(pub Vec<GalaxySpecialTrait>, pub Vec<GalaxySpecialTrait>);

/// Generates a list of pairs of incompatible traits.
pub fn get_opposite_traits() -> Vec<OppositeTraits> {
    vec![
        OppositeTraits(
            vec![GalaxySpecialTrait::Younger],
            vec![GalaxySpecialTrait::Older],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::Dusty],
            vec![GalaxySpecialTrait::MetalPoor],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::GasPoor],
            vec![GalaxySpecialTrait::GasRich],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::Compact(0)],
            vec![GalaxySpecialTrait::Expansive(0)],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::SubSize(0)],
            vec![GalaxySpecialTrait::SuperSize(0)],
        ),
        OppositeTraits(
            vec![GalaxySpecialTrait::Starburst],
            vec![GalaxySpecialTrait::Dead, GalaxySpecialTrait::Dormant],
        ),
    ]
}

/// The default galactic region map layer for [spiral](GalaxyCategory::Spiral) galaxies: a small Nucleus
/// (yellow) and Bulge (orange) at the center, two Arm (blue) spiral arcs, a Disk (teal) filling the space
/// between the arms out to the rim, and a thin Halo (purple) band at the very edge. The unfilled corners
/// count as Void.
///
/// The Nucleus and Bulge zones are kept deliberately tiny (about 1% of the disc area): their density tiers
/// spawn roughly one system per hex at 99%+ probability, so large central zones would unbalance generation.
pub const DEFAULT_REGION_MAP_SPIRAL: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024" shape-rendering="crispEdges">
  <circle cx="512" cy="512" r="512" fill="#800080"/>
  <circle cx="512" cy="512" r="478" fill="#008080"/>
  <polygon points="571.2,512.0 575.7,508.2 580.3,503.8 584.9,498.7 589.5,493.0 594.1,486.6 598.5,479.4 601.0,472.2 603.2,464.5 605.0,456.2 606.4,447.4 607.3,438.0 607.7,428.1 607.5,417.6 606.5,406.5 604.9,395.0 602.4,382.9 599.0,370.3 594.6,357.3 589.2,343.9 582.7,330.2 574.9,316.1 565.9,301.7 555.4,287.2 543.5,272.5 530.0,257.9 514.9,243.3 498.1,228.9 479.4,214.8 458.9,201.1 436.4,188.0 411.9,175.6 385.3,164.2 356.6,153.8 325.8,144.7 292.7,137.0 257.4,131.0 219.8,127.0 189.6,136.4 167.7,156.4 147.0,177.6 237.7,260.7 266.7,258.6 294.3,258.3 320.4,259.5 345.0,262.2 368.2,266.1 389.9,271.1 410.1,277.1 428.9,283.9 446.4,291.4 462.4,299.5 477.2,308.1 490.6,317.1 502.9,326.3 513.9,335.8 523.8,345.4 532.7,355.0 540.5,364.6 547.3,374.1 553.3,383.5 558.4,392.8 562.6,401.8 566.2,410.6 569.1,419.1 571.3,427.3 572.9,435.3 574.0,442.8 574.6,450.1 574.8,457.0 574.5,463.5 573.9,469.6 573.0,475.4 571.8,480.9 570.4,485.9 568.7,490.7 568.4,494.6 567.8,498.4 566.9,502.0 565.9,505.5 564.6,508.8 563.1,512.0" fill="#0000FF"/>
  <polygon points="452.8,512.0 448.3,515.8 443.7,520.2 439.1,525.3 434.5,531.0 429.9,537.4 425.5,544.6 423.0,551.8 420.8,559.5 419.0,567.8 417.6,576.6 416.7,586.0 416.3,595.9 416.5,606.4 417.5,617.5 419.1,629.0 421.6,641.1 425.0,653.7 429.4,666.7 434.8,680.1 441.3,693.8 449.1,707.9 458.1,722.3 468.6,736.8 480.5,751.5 494.0,766.1 509.1,780.7 525.9,795.1 544.6,809.2 565.1,822.9 587.6,836.0 612.1,848.4 638.7,859.8 667.4,870.2 698.2,879.3 731.3,887.0 766.6,893.0 804.2,897.0 834.4,887.6 856.3,867.6 877.0,846.4 786.3,763.3 757.3,765.4 729.7,765.7 703.6,764.5 679.0,761.8 655.8,757.9 634.1,752.9 613.9,746.9 595.1,740.1 577.6,732.6 561.6,724.5 546.8,715.9 533.4,706.9 521.1,697.7 510.1,688.2 500.2,678.6 491.3,669.0 483.5,659.4 476.7,649.9 470.7,640.5 465.6,631.2 461.4,622.2 457.8,613.4 454.9,604.9 452.7,596.7 451.1,588.7 450.0,581.2 449.4,573.9 449.2,567.0 449.5,560.5 450.1,554.4 451.0,548.6 452.2,543.1 453.6,538.1 455.3,533.3 455.6,529.4 456.2,525.6 457.1,522.0 458.1,518.5 459.4,515.2 460.9,512.0" fill="#0000FF"/>
  <circle cx="512" cy="512" r="48" fill="#FFA500"/>
  <circle cx="512" cy="512" r="17" fill="#FFFF00"/>
</svg>"##;

/// The default galactic region map layer for [lenticular](GalaxyCategory::Lenticular) galaxies: a small
/// Bulge (orange) at the center, a Disk (teal) body without arms, and a Halo (purple) rim. The unfilled
/// corners count as Void.
///
/// The Bulge zone is kept deliberately small (under 2% of the disc area): its density tier spawns roughly
/// one system per hex at ~99% probability, so a large bulge would unbalance generation.
pub const DEFAULT_REGION_MAP_LENTICULAR: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024" shape-rendering="crispEdges">
  <circle cx="512" cy="512" r="512" fill="#800080"/>
  <circle cx="512" cy="512" r="470" fill="#008080"/>
  <circle cx="512" cy="512" r="56" fill="#FFA500"/>
</svg>"##;

/// The default galactic region map layer for [elliptical](GalaxyCategory::Elliptical) and
/// [dominant elliptical](GalaxyCategory::DominantElliptical) galaxies: a small Core (white) at the center,
/// an Ellipse (green) body, and a Halo (purple) rim. The unfilled corners count as Void.
///
/// The Core zone is kept deliberately small (about 1% of the ellipse area): its density tier spawns roughly
/// one system per hex at ~99% probability, so a large core would unbalance generation.
pub const DEFAULT_REGION_MAP_ELLIPTICAL: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024" shape-rendering="crispEdges">
  <circle cx="512" cy="512" r="512" fill="#800080"/>
  <ellipse cx="512" cy="512" rx="470" ry="360" fill="#008000"/>
  <circle cx="512" cy="512" r="40" fill="#FFFFFF"/>
</svg>"##;

/// The default galactic region map layer for [irregular](GalaxyCategory::Irregular),
/// [intergalactic](GalaxyCategory::Intergalactic) and [intracluster](GalaxyCategory::Intracluster)
/// galaxies: an off-center Disk (teal) blob made of two overlapping ellipses with two `OpenCluster` (cyan)
/// knots, surrounded by a generous Aura (magenta) halo reaching the map edges. The unfilled corners count
/// as Void.
///
/// `shape-rendering="crispEdges"` turns off anti-aliasing: every rasterized pixel is exactly one palette
/// color, so no pixel blends into an off-palette color that would trigger an
/// [`SvgPixelsUnmatched`](crate::diagnostics::types::GenerationWarning::SvgPixelsUnmatched) warning.
pub const DEFAULT_REGION_MAP_IRREGULAR: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024" shape-rendering="crispEdges">
  <ellipse cx="512" cy="512" rx="510" ry="500" fill="#FF00FF"/>
  <ellipse cx="460" cy="530" rx="220" ry="160" fill="#008080"/>
  <ellipse cx="570" cy="480" rx="150" ry="115" fill="#008080"/>
  <circle cx="430" cy="480" r="28" fill="#00FFFF"/>
  <circle cx="585" cy="565" r="22" fill="#00FFFF"/>
</svg>"##;

/// A small annotated two-zone example of a galactic region map layer, suitable for documentation: the left
/// half is painted with the exact Arm palette color, the right half is left unfilled and therefore counts
/// as Void.
pub const EXAMPLE_REGION_MAP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024">
  <!-- A galactic region map layer: the SVG is stretched over the galaxy's bounds, with the SVG top edge -->
  <!-- corresponding to the maximum galactic y coordinate. -->
  <!-- Only solid fills in the exact palette colors documented on GalacticRegion are recognized: no -->
  <!-- gradients, patterns, strokes, text or opacity tricks. Any unfilled background counts as Void. -->
  <!-- The left half of the galaxy is part of a spiral Arm. -->
  <rect x="0" y="0" width="512" height="1024" fill="#0000FF"/>
  <!-- The right half is left unpainted: it is Void. -->
</svg>"##;

/// Returns the default galactic region map layers for the given galaxy category, ready to be rasterized by
/// [`GalacticRegionMap::build`](crate::galaxy::map::region_map::types::GalacticRegionMap::build). Sub-categories
/// (e.g. barred spirals) do not alter the defaults.
///
/// Note that the outskirts of the irregular-style categories map to Aura, a much sparser region than the
/// catch-all used before region maps existed.
#[must_use]
pub(crate) fn default_layers_for(category: GalaxyCategory) -> Vec<String> {
    match category {
        GalaxyCategory::Spiral(..) => vec![DEFAULT_REGION_MAP_SPIRAL.to_string()],
        GalaxyCategory::Lenticular(..) => vec![DEFAULT_REGION_MAP_LENTICULAR.to_string()],
        GalaxyCategory::Elliptical(..) | GalaxyCategory::DominantElliptical(..) => {
            vec![DEFAULT_REGION_MAP_ELLIPTICAL.to_string()]
        }
        GalaxyCategory::Irregular(..)
        | GalaxyCategory::Intergalactic(..)
        | GalaxyCategory::Intracluster(..) => {
            vec![DEFAULT_REGION_MAP_IRREGULAR.to_string()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_bounds() -> (SpaceCoordinates, SpaceCoordinates) {
        (
            SpaceCoordinates::new(-500, -500, 0),
            SpaceCoordinates::new(500, 500, 0),
        )
    }

    fn build_default(svg: &str) -> GalacticRegionMap {
        let (start, end) = test_bounds();
        GalacticRegionMap::build(&[svg.to_string()], start, end)
            .expect("The default SVG should parse.")
    }

    /// Returns the region of the map cell at the given grid indices by probing the coordinates mapping to
    /// the middle of that cell.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn region_at_cell(map: &GalacticRegionMap, col: usize, row: usize) -> GalacticRegion {
        let x = -500.0 + (col as f64 + 0.5) / REGION_GRID_RESOLUTION as f64 * 1000.0;
        let y = 500.0 - (row as f64 + 0.5) / REGION_GRID_RESOLUTION as f64 * 1000.0;
        map.region_at(SpaceCoordinates::new(x.round() as i64, y.round() as i64, 0))
    }

    fn count_cells(map: &GalacticRegionMap, regions: &[GalacticRegion]) -> usize {
        let mut count = 0;
        for row in 0..REGION_GRID_RESOLUTION {
            for col in 0..REGION_GRID_RESOLUTION {
                if regions.contains(&region_at_cell(map, col, row)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn region_char(region: GalacticRegion) -> char {
        match region {
            GalacticRegion::Core => 'C',
            GalacticRegion::Nucleus => 'N',
            GalacticRegion::Bulge => 'B',
            GalacticRegion::Bar => '|',
            GalacticRegion::Arm => 'A',
            GalacticRegion::Disk => 'd',
            GalacticRegion::Ellipse => 'e',
            GalacticRegion::Halo => 'H',
            GalacticRegion::Aura => 'a',
            GalacticRegion::GlobularCluster => 'g',
            GalacticRegion::OpenCluster => 'o',
            GalacticRegion::Association => 's',
            GalacticRegion::Stream => '~',
            GalacticRegion::Exile => 'x',
            GalacticRegion::Multiple => '?',
            GalacticRegion::Void => '.',
        }
    }

    /// Prints a 64x64 ASCII rendering of each default map; run with `cargo test -- --nocapture --ignored`.
    #[test]
    #[ignore = "ASCII rendering helper, run on demand"]
    fn print_default_maps_ascii() {
        for (name, svg) in [
            ("SPIRAL", DEFAULT_REGION_MAP_SPIRAL),
            ("LENTICULAR", DEFAULT_REGION_MAP_LENTICULAR),
            ("ELLIPTICAL", DEFAULT_REGION_MAP_ELLIPTICAL),
            ("IRREGULAR", DEFAULT_REGION_MAP_IRREGULAR),
        ] {
            let map = build_default(svg);
            println!("\n=== {name} ===");
            for row in 0..64 {
                let line: String = (0..64)
                    .map(|col| region_char(region_at_cell(&map, col * 4 + 2, row * 4 + 2)))
                    .collect();
                println!("{line}");
            }
        }
    }

    #[test]
    fn default_maps_parse_and_build() {
        let spiral = build_default(DEFAULT_REGION_MAP_SPIRAL);
        for region in [
            GalacticRegion::Nucleus,
            GalacticRegion::Bulge,
            GalacticRegion::Arm,
            GalacticRegion::Disk,
            GalacticRegion::Halo,
        ] {
            assert!(
                count_cells(&spiral, &[region]) > 0,
                "The spiral map should contain {region:?} cells."
            );
        }
        // The corners and anti-aliased seams are the only acceptable Void: well under a third of the map.
        let void_cells = count_cells(&spiral, &[GalacticRegion::Void]);
        assert!(
            void_cells * 100 < 30 * REGION_GRID_RESOLUTION * REGION_GRID_RESOLUTION,
            "The spiral map has suspiciously many Void cells ({void_cells})."
        );

        let lenticular = build_default(DEFAULT_REGION_MAP_LENTICULAR);
        for region in [
            GalacticRegion::Bulge,
            GalacticRegion::Disk,
            GalacticRegion::Halo,
        ] {
            assert!(
                count_cells(&lenticular, &[region]) > 0,
                "The lenticular map should contain {region:?} cells."
            );
        }

        let elliptical = build_default(DEFAULT_REGION_MAP_ELLIPTICAL);
        for region in [
            GalacticRegion::Core,
            GalacticRegion::Ellipse,
            GalacticRegion::Halo,
        ] {
            assert!(
                count_cells(&elliptical, &[region]) > 0,
                "The elliptical map should contain {region:?} cells."
            );
        }

        let irregular = build_default(DEFAULT_REGION_MAP_IRREGULAR);
        for region in [
            GalacticRegion::Disk,
            GalacticRegion::OpenCluster,
            GalacticRegion::Aura,
        ] {
            assert!(
                count_cells(&irregular, &[region]) > 0,
                "The irregular map should contain {region:?} cells."
            );
        }
    }

    #[test]
    fn spiral_center_is_nucleus_or_bulge_and_rim_is_sparse() {
        let map = build_default(DEFAULT_REGION_MAP_SPIRAL);
        let center = map.region_at(SpaceCoordinates::new(0, 0, 0));
        assert!(
            center == GalacticRegion::Nucleus || center == GalacticRegion::Bulge,
            "The spiral center should be Nucleus or Bulge, not {center:?}."
        );
        // ~85% of the radius along the first arm's centerline.
        let on_arm = map.region_at(SpaceCoordinates::new(-291, 310, 0));
        assert!(
            on_arm == GalacticRegion::Arm || on_arm == GalacticRegion::Disk,
            "A point at 85% radius on an arm angle should be Arm or Disk, not {on_arm:?}."
        );
        // A corner-ish edge point is outside the galactic disc.
        let corner = map.region_at(SpaceCoordinates::new(480, 480, 0));
        assert!(
            corner == GalacticRegion::Halo || corner == GalacticRegion::Void,
            "A corner-ish edge point should be Halo or Void, not {corner:?}."
        );
        // The middle of a map edge lies in the thin Halo band.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 480, 0)),
            GalacticRegion::Halo
        );
    }

    #[test]
    fn dense_zones_stay_small() {
        let dense_regions = [
            GalacticRegion::Nucleus,
            GalacticRegion::Bulge,
            GalacticRegion::Core,
        ];
        let max_dense_cells = 3 * REGION_GRID_RESOLUTION * REGION_GRID_RESOLUTION / 100;
        let spiral = build_default(DEFAULT_REGION_MAP_SPIRAL);
        let spiral_dense = count_cells(&spiral, &dense_regions);
        assert!(
            spiral_dense <= max_dense_cells,
            "The spiral map has {spiral_dense} dense (Nucleus/Bulge/Core) cells, over 3% of the grid."
        );
        let elliptical = build_default(DEFAULT_REGION_MAP_ELLIPTICAL);
        let elliptical_dense = count_cells(&elliptical, &dense_regions);
        assert!(
            elliptical_dense <= max_dense_cells,
            "The elliptical map has {elliptical_dense} dense (Nucleus/Bulge/Core) cells, over 3% of the grid."
        );
    }

    #[test]
    fn default_layers_for_covers_every_category() {
        let (start, end) = test_bounds();
        for category in [
            GalaxyCategory::Spiral(10000, 100),
            GalaxyCategory::Lenticular(10000, 600),
            GalaxyCategory::Elliptical(10000),
            GalaxyCategory::DominantElliptical(300_000),
            GalaxyCategory::Irregular(3000, 3000, 2000),
            GalaxyCategory::Intergalactic(1000, 3000, 1000),
            GalaxyCategory::Intracluster(1, 3, 1),
        ] {
            let layers = default_layers_for(category);
            assert!(
                !layers.is_empty(),
                "{category} should have at least one default layer."
            );
            assert!(
                GalacticRegionMap::build(&layers, start, end).is_some(),
                "The default layers of {category} should build."
            );
        }
    }

    #[test]
    fn default_maps_rasterize_without_unmatched_pixels() {
        // All four default maps use `shape-rendering="crispEdges"` so their edges
        // never anti-alias into off-palette blends: no SvgPixelsUnmatched warning
        // allowed on any of them.
        let (start, end) = test_bounds();
        for svg in [
            DEFAULT_REGION_MAP_SPIRAL,
            DEFAULT_REGION_MAP_LENTICULAR,
            DEFAULT_REGION_MAP_ELLIPTICAL,
            DEFAULT_REGION_MAP_IRREGULAR,
        ] {
            let mut diagnostics = crate::diagnostics::types::Diagnostics::default();
            let map = GalacticRegionMap::build_with_diagnostics(
                &[svg.to_string()],
                start,
                end,
                &mut diagnostics,
            )
            .expect("The default SVG should parse.");
            let _ = map;
            assert!(
                diagnostics.warnings.is_empty(),
                "A default map should rasterize clean, got {:?}.",
                diagnostics.warnings
            );
        }
    }

    #[test]
    fn example_svg_builds_and_matches_docs() {
        let map = build_default(EXAMPLE_REGION_MAP_SVG);
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-250, 0, 0)),
            GalacticRegion::Arm
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(250, 0, 0)),
            GalacticRegion::Void
        );
    }
}
