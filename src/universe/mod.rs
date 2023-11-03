use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

/// Data allowing us to model the universe.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct Universe {
    /// In which part of the Stelliferous Era the universe is currently.
    pub era: StelliferousEra,
    /// The time passed since the big bang, in billions of years.
    #[default = 13.8]
    pub age: f32,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A {} billion years old Universe in the {} era",
            self.age, self.era
        )
    }
}

impl Universe {
    /// Returns a new [Universe] using the given arguments.
    pub fn new(era: StelliferousEra, age: f32) -> Self {
        Self { era, age }
    }
}
