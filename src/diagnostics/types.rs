use crate::galaxy::map::hex::types::SpaceCoordinates;

/// Collects non-fatal issues raised during generation. Transient: deliberately
/// NOT serialized. Carried by [crate::system::types::StarSystem],
/// [crate::galaxy::Galaxy] and [crate::generator::types::GeneratedUniverse].
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Diagnostics {
    pub warnings: Vec<GenerationWarning>,
    pub errors: Vec<GenerationError>,
}

/// A non-fatal generation issue, collected in [Diagnostics] while generation
/// carries on with a fallback.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
#[non_exhaustive]
pub enum GenerationWarning {
    /// The SVG galactic map layer at the given index could not be parsed and was skipped.
    SvgLayerParseFailed {
        /// Index of the offending layer in `galactic_map_layers`.
        layer_index: usize,
    },
    /// The given number of pixels of the SVG layer at the given index matched no region color.
    SvgPixelsUnmatched {
        /// Index of the offending layer in `galactic_map_layers`.
        layer_index: usize,
        /// How many pixels matched no region color.
        count: usize,
    },
    /// The user-provided galactic map layers could not be used and were replaced with the galaxy category's default layers.
    UserLayersReplacedWithDefault,
    /// The region map was rebuilt from the galaxy category's default layers.
    RegionMapRebuiltFromDefault,
    /// The requested universe age was outside the valid range and was clamped to the given value.
    AgeOutOfRange {
        /// The age that was asked for, in billions of years.
        requested: f32,
        /// The age actually used, in billions of years.
        used: f32,
    },
    /// A hardcoded weighted-roll table returned no result; the table's first entry was
    /// used as a deterministic fallback. Built-in tables are non-empty with positive
    /// weights, so this is only reachable with degenerate custom settings or maps;
    /// the payload names what was being rolled.
    RngTableFallback(&'static str),
}

/// A fatal-in-context generation issue: the operation that raised it cannot
/// complete, so it backs the public `Result` error type.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
#[non_exhaustive]
pub enum GenerationError {
    /// The given coordinates are outside the bounds of the galaxy.
    InvalidCoordinates(SpaceCoordinates),
    /// The given division level is outside the valid 1..=9 range.
    InvalidDivisionLevel(u8),
    /// `only_interesting` was set but no interesting system was found within the given number of tries.
    InterestingSystemNotFound {
        /// How many generation tries were attempted before giving up.
        tries: u32,
    },
    /// The given [`GenerationSettings`](crate::generator::types::GenerationSettings) failed validation; contains every violation found.
    InvalidSettings(Vec<SettingsError>),
    /// An internal invariant was violated during generation. Never raised for
    /// reachable inputs under validated settings; the payload names the invariant.
    InternalInvariant(&'static str),
}

/// A single settings violation found by
/// [`GenerationSettings::validate`](crate::generator::types::GenerationSettings::validate).
#[derive(Clone, PartialEq, PartialOrd, Debug)]
#[non_exhaustive]
pub enum SettingsError {
    /// A `hex_size` component is 0; every component must be between 1 and 255 inclusive.
    InvalidHexSize(u8, u8, u8),
    /// A `level_N_size` component is outside the valid 4..=62 range.
    InvalidLevelSize {
        /// Which division level the offending size belongs to (1..=9).
        level: u8,
        /// The offending (x, y, z) size.
        size: (u8, u8, u8),
    },
    /// `universe.fixed_age` is outside the valid [0.4, 100000) range, in billions of years.
    InvalidUniverseAge(f32),
    /// `star.fixed_age` is negative.
    InvalidStarAge(f32),
    /// `star.fixed_mass` is negative.
    InvalidStarMass(f64),
    /// Every `celestial_body.do_not_generate_*` flag is set: no body type can ever
    /// spawn, which would divide by zero inside the roll tables.
    AllBodyTypesDisabled,
    /// Every `star.*_gen_chance` weight is 0: no star class can ever be rolled,
    /// which would divide by zero inside the roll tables.
    AllStarChancesZero,
}
