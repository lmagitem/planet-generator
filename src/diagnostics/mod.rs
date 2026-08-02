pub mod types;
pub use types::*;

use std::fmt::Display;

impl Diagnostics {
    pub fn push_warning(&mut self, warning: GenerationWarning) {
        self.warnings.push(warning);
    }
    pub fn push_error(&mut self, error: GenerationError) {
        self.errors.push(error);
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn is_empty(&self) -> bool {
        self.warnings.is_empty() && self.errors.is_empty()
    }
}

impl Display for Diagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} warning(s), {} error(s)",
            self.warnings.len(),
            self.errors.len()
        )
    }
}

impl Display for GenerationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationWarning::SvgLayerParseFailed { layer_index } => {
                write!(f, "SVG map layer {layer_index} could not be parsed and was skipped.")
            }
            GenerationWarning::SvgPixelsUnmatched { layer_index, count } => write!(
                f,
                "{count} pixel(s) of SVG map layer {layer_index} matched no region color."
            ),
            GenerationWarning::UserLayersReplacedWithDefault => write!(
                f,
                "The user-provided map layers could not be used and were replaced with the default ones."
            ),
            GenerationWarning::RegionMapRebuiltFromDefault => {
                write!(f, "The region map was rebuilt from the default layers.")
            }
            GenerationWarning::AgeOutOfRange { requested, used } => write!(
                f,
                "Requested universe age {requested} is out of range; clamped to {used}."
            ),
            GenerationWarning::RngTableFallback(what) => write!(
                f,
                "A weighted roll for {what} returned no result; the table's first entry was used."
            ),
        }
    }
}

impl Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationError::InvalidCoordinates(coord) => write!(
                f,
                "Invalid coordinates: {coord} is outside the bounds of the galaxy."
            ),
            GenerationError::InvalidDivisionLevel(level) => write!(
                f,
                "Invalid division level {level}: must be between 1 and 9 inclusive."
            ),
            GenerationError::InterestingSystemNotFound { tries } => {
                write!(f, "No interesting system found after {tries} tries.")
            }
            GenerationError::InvalidSettings(errors) => write!(
                f,
                "Invalid generation settings: {}",
                errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            GenerationError::InternalInvariant(what) => {
                write!(f, "Internal invariant violated: {what}.")
            }
        }
    }
}

impl std::error::Error for GenerationError {}

impl Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::InvalidHexSize(x, y, z) => write!(
                f,
                "Invalid hex size ({x}, {y}, {z}): every component must be between 1 and 255 inclusive."
            ),
            SettingsError::InvalidLevelSize { level, size } => write!(
                f,
                "Invalid level {level} size ({}, {}, {}): components must be between 4 and 62 inclusive.",
                size.0, size.1, size.2
            ),
            SettingsError::InvalidUniverseAge(age) => write!(
                f,
                "Invalid universe age {age}: must be higher or equal to 0.4 and lower than 100000."
            ),
            SettingsError::InvalidStarAge(age) => {
                write!(f, "Invalid star age {age}: must not be negative.")
            }
            SettingsError::InvalidStarMass(mass) => {
                write!(f, "Invalid star mass {mass}: must not be negative.")
            }
            SettingsError::AllBodyTypesDisabled => {
                write!(
                    f,
                    "All celestial body types are disabled: at least one do_not_generate_* flag must be false."
                )
            }
            SettingsError::AllStarChancesZero => {
                write!(
                    f,
                    "All star class generation chances are zero: at least one *_gen_chance must be positive."
                )
            }
        }
    }
}

impl std::error::Error for SettingsError {}
