use crate::internal::*;
use crate::prelude::*;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    SmartDefault,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum ChemicalComponent {
    #[default]
    Hydrogen,
    Helium,
    Carbon,
    Nitrogen,
    Oxygen,
    Silicon,
    Magnesium,
    Iron,
    Sulfur,
    Sodium,
    Potassium,
    Calcium,
    Aluminum,
    Phosphorus,
    Chlorine,
    Argon,
    Titanium,
    Chromium,
    Manganese,
    Nickel,
    Water,
    CarbonMonoxide,
    CarbonDioxide,
    Methane,
    Ammonia,
    HydrogenSulfide,
    SulfurDioxide,
    Hydroxyl,
    NitricOxide,
    NitrogenDioxide,
    Formaldehyde,
    Methanol,
    Ethylene,
    Ethane,
    Acetylene,
    Benzene,
    Acetonitrile,
    Methylamine,
    HydrogenCyanide,
    Glycine,
    Silicates,
    PolycyclicAromaticHydrocarbons,
}

impl ChemicalComponent {
    pub fn molecular_weight_amu(&self) -> f64 {
        match self {
            ChemicalComponent::Hydrogen => 1.008,
            ChemicalComponent::Helium => 4.0026,
            ChemicalComponent::Carbon => 12.01,
            ChemicalComponent::Nitrogen => 14.007,
            ChemicalComponent::Oxygen => 16.00,
            ChemicalComponent::Silicon => 28.0855,
            ChemicalComponent::Magnesium => 24.305,
            ChemicalComponent::Iron => 55.845,
            ChemicalComponent::Sulfur => 32.06,
            ChemicalComponent::Sodium => 22.99,
            ChemicalComponent::Potassium => 39.10,
            ChemicalComponent::Calcium => 40.08,
            ChemicalComponent::Aluminum => 26.98,
            ChemicalComponent::Phosphorus => 30.97,
            ChemicalComponent::Chlorine => 35.45,
            ChemicalComponent::Argon => 39.95,
            ChemicalComponent::Titanium => 47.867,
            ChemicalComponent::Chromium => 51.996,
            ChemicalComponent::Manganese => 54.938,
            ChemicalComponent::Nickel => 58.693,
            ChemicalComponent::Water => 18.015,
            ChemicalComponent::CarbonMonoxide => 28.01,
            ChemicalComponent::CarbonDioxide => 44.01,
            ChemicalComponent::Methane => 16.04,
            ChemicalComponent::Ammonia => 17.031,
            ChemicalComponent::HydrogenSulfide => 34.08,
            ChemicalComponent::SulfurDioxide => 64.066,
            ChemicalComponent::Hydroxyl => 17.007,
            ChemicalComponent::NitricOxide => 30.006,
            ChemicalComponent::NitrogenDioxide => 46.0055,
            ChemicalComponent::Formaldehyde => 30.03,
            ChemicalComponent::Methanol => 32.04,
            ChemicalComponent::Ethylene => 28.05,
            ChemicalComponent::Ethane => 30.07,
            ChemicalComponent::Acetylene => 26.04,
            ChemicalComponent::Benzene => 78.11,
            ChemicalComponent::Acetonitrile => 41.05,
            ChemicalComponent::Methylamine => 31.06,
            ChemicalComponent::HydrogenCyanide => 27.025,
            ChemicalComponent::Glycine => 75.07,
            ChemicalComponent::Silicates => 60.08, // Approximate weight of the silicate group (SiO₄)
            ChemicalComponent::PolycyclicAromaticHydrocarbons => 128.16, // Approximate weight of naphthalene (C₁₀H₈)
        }
    }

    pub fn triple_point(&self) -> Option<(f64, f64)> {
        match self {
            ChemicalComponent::Hydrogen => Some((13.8, 0.070)),
            ChemicalComponent::Helium => Some((2.2, 0.0052)),
            ChemicalComponent::Carbon => None, // Carbon does not have a standard triple point
            ChemicalComponent::Nitrogen => Some((63.15, 0.1235)),
            ChemicalComponent::Oxygen => Some((54.36, 0.0015)),
            ChemicalComponent::Silicon => None, // Silicon does not have a standard triple point
            ChemicalComponent::Magnesium => None, // Magnesium does not have a standard triple point
            ChemicalComponent::Iron => None,    // Iron does not have a standard triple point
            ChemicalComponent::Sulfur => None,  // Sulfur does not have a standard triple point
            ChemicalComponent::Sodium => None,  // Sodium does not have a standard triple point
            ChemicalComponent::Potassium => None, // Potassium does not have a standard triple point
            ChemicalComponent::Calcium => None, // Calcium does not have a standard triple point
            ChemicalComponent::Aluminum => None, // Aluminum does not have a standard triple point
            ChemicalComponent::Phosphorus => None, // Phosphorus does not have a standard triple point
            ChemicalComponent::Chlorine => Some((172.2, 0.4)),
            ChemicalComponent::Argon => Some((83.81, 0.687)),
            ChemicalComponent::Titanium => None, // Titanium does not have a standard triple point
            ChemicalComponent::Chromium => None, // Chromium does not have a standard triple point
            ChemicalComponent::Manganese => None, // Manganese does not have a standard triple point
            ChemicalComponent::Nickel => None,   // Nickel does not have a standard triple point
            ChemicalComponent::Water => Some((273.16, 0.00604)),
            ChemicalComponent::CarbonMonoxide => Some((68.15, 0.00015)),
            ChemicalComponent::CarbonDioxide => Some((216.55, 5.11)),
            ChemicalComponent::Methane => Some((90.67, 0.117)),
            ChemicalComponent::Ammonia => Some((195.4, 0.060)),
            ChemicalComponent::HydrogenSulfide => Some((187.61, 0.0276)),
            ChemicalComponent::SulfurDioxide => Some((197.67, 0.0169)),
            ChemicalComponent::Hydroxyl => None, // Hydroxyl radical does not have a standard triple point
            ChemicalComponent::NitricOxide => Some((109.5, 0.00015)),
            ChemicalComponent::NitrogenDioxide => Some((261.93, 0.001)),
            ChemicalComponent::Formaldehyde => Some((155.2, 0.016)),
            ChemicalComponent::Methanol => Some((175.47, 0.08)),
            ChemicalComponent::Ethylene => Some((104.0, 0.00033)),
            ChemicalComponent::Ethane => Some((89.89, 0.00014)),
            ChemicalComponent::Acetylene => Some((192.34, 0.0127)),
            ChemicalComponent::Benzene => Some((278.68, 0.048)),
            ChemicalComponent::Acetonitrile => Some((229.4, 0.042)),
            ChemicalComponent::Methylamine => Some((175.8, 0.0014)),
            ChemicalComponent::HydrogenCyanide => Some((260.8, 0.02)),
            ChemicalComponent::Glycine => None, // Glycine does not have a standard triple point
            ChemicalComponent::Silicates => None, // Silicates as a group do not have a standard triple point
            ChemicalComponent::PolycyclicAromaticHydrocarbons => None, // PAHs do not have a standard triple point
        }
    }

    pub fn boiling_point(&self) -> Option<f64> {
        match self {
            ChemicalComponent::Hydrogen => Some(20.28),
            ChemicalComponent::Helium => Some(4.22),
            ChemicalComponent::Carbon => None, // Carbon does not have a standard boiling point
            ChemicalComponent::Nitrogen => Some(77.36),
            ChemicalComponent::Oxygen => Some(90.20),
            ChemicalComponent::Silicon => None, // Silicon does not have a standard boiling point
            ChemicalComponent::Magnesium => None, // Magnesium does not have a standard boiling point
            ChemicalComponent::Iron => None,      // Iron does not have a standard boiling point
            ChemicalComponent::Sulfur => None,    // Sulfur does not have a standard boiling point
            ChemicalComponent::Sodium => None,    // Sodium does not have a standard boiling point
            ChemicalComponent::Potassium => None, // Potassium does not have a standard boiling point
            ChemicalComponent::Calcium => None,   // Calcium does not have a standard boiling point
            ChemicalComponent::Aluminum => None,  // Aluminum does not have a standard boiling point
            ChemicalComponent::Phosphorus => None, // Phosphorus does not have a standard boiling point
            ChemicalComponent::Chlorine => Some(239.11),
            ChemicalComponent::Argon => Some(87.3),
            ChemicalComponent::Titanium => None, // Titanium does not have a standard boiling point
            ChemicalComponent::Chromium => None, // Chromium does not have a standard boiling point
            ChemicalComponent::Manganese => None, // Manganese does not have a standard boiling point
            ChemicalComponent::Nickel => None,    // Nickel does not have a standard boiling point
            ChemicalComponent::Water => Some(373.16),
            ChemicalComponent::CarbonMonoxide => Some(82.9),
            ChemicalComponent::CarbonDioxide => Some(304.25),
            ChemicalComponent::Methane => Some(111.66),
            ChemicalComponent::Ammonia => Some(239.81),
            ChemicalComponent::HydrogenSulfide => Some(212.9),
            ChemicalComponent::SulfurDioxide => Some(263.05),
            ChemicalComponent::Hydroxyl => None, // Hydroxyl radical does not have a standard boiling point
            ChemicalComponent::NitricOxide => Some(121.36),
            ChemicalComponent::NitrogenDioxide => None, // NO₂ decomposes before boiling
            ChemicalComponent::Formaldehyde => Some(252.2),
            ChemicalComponent::Methanol => Some(337.85),
            ChemicalComponent::Ethylene => Some(169.42),
            ChemicalComponent::Ethane => Some(184.55),
            ChemicalComponent::Acetylene => Some(189.34),
            ChemicalComponent::Benzene => Some(353.25),
            ChemicalComponent::Acetonitrile => Some(354.6),
            ChemicalComponent::Methylamine => Some(266.1),
            ChemicalComponent::HydrogenCyanide => Some(299.2),
            ChemicalComponent::Glycine => None, // Glycine does not have a standard boiling point
            ChemicalComponent::Silicates => None, // Silicates as a group do not have a standard boiling point
            ChemicalComponent::PolycyclicAromaticHydrocarbons => None, // PAHs do not have a standard boiling point
        }
    }

    /// Determines if a substance can exist in liquid state at given temperature and pressure.
    ///
    /// # Parameters
    /// - `substance`: The substance to check.
    /// - `temperature`: The temperature in Kelvin.
    /// - `pressure`: The pressure in atm.
    ///
    /// # Returns
    /// `true` if the substance can exist in liquid state, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use crate::planet_generator::prelude::*;
    ///
    /// let water = ChemicalComponent::Water;
    /// let can_exist = water.can_exist_as_liquid(280, 1.0);
    /// println!("Can water exist as liquid at 280 K and 1 atm? {}", can_exist);
    /// ```
    pub fn can_exist_as_liquid(&self, temperature: u32, pressure: f32) -> bool {
        let temperature = temperature as f64;
        let pressure = pressure as f64;
        if let Some((triple_point_temp, triple_point_pressure)) = self.triple_point() {
            if let Some(boiling_point_temp) = self.boiling_point() {
                return temperature > triple_point_temp
                    && temperature < boiling_point_temp
                    && pressure > triple_point_pressure;
            }
        }
        false
    }

    /// Determines if a substance can exist in gas state at given temperature and pressure.
    ///
    /// # Parameters
    /// - `temperature`: The temperature in Kelvin.
    /// - `pressure`: The pressure in atm.
    ///
    /// # Returns
    /// `true` if the substance can exist in gas state, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use crate::planet_generator::prelude::*;
    ///
    /// let water = ChemicalComponent::Water;
    /// let can_exist = water.can_exist_as_gas(400, 1.0);
    /// println!("Can water exist as gas at 400 K and 1 atm? {}", can_exist);
    /// ```
    pub fn can_exist_as_gas(&self, temperature: u32, pressure: f32) -> bool {
        let temperature = temperature as f64;
        let pressure = pressure as f64;
        if let Some((triple_point_temp, triple_point_pressure)) = self.triple_point() {
            if let Some(boiling_point_temp) = self.boiling_point() {
                // Check if the temperature is above the boiling point or above the triple point temperature
                if temperature > boiling_point_temp as f64 {
                    return true;
                } else if temperature > triple_point_temp && pressure < triple_point_pressure {
                    return true;
                }
            } else if temperature > triple_point_temp {
                // For substances that sublimate, check if the temperature is above the triple point temperature
                return true;
            }
        }
        false
    }

    pub fn components_liquid_at(temperature: u32, pressure: f32) -> Option<Vec<ChemicalComponent>> {
        let mut possible_liquids = Vec::new();
        let mut possible_lowest_pressure = None;

        if (ChemicalComponent::Water.can_exist_as_liquid(temperature, pressure)) {
            possible_liquids.push(ChemicalComponent::Water);
        } else {
            for component in ChemicalComponent::iter() {
                if component.can_exist_as_liquid(temperature, pressure) {
                    possible_liquids.push(component);
                } else if let Some((triple_point_temp, triple_point_pressure)) =
                    component.triple_point()
                {
                    if component.can_exist_as_liquid(temperature, triple_point_pressure as f32) {
                        if let Some((_, lowest_pressure)) = possible_lowest_pressure {
                            if triple_point_pressure < lowest_pressure {
                                possible_lowest_pressure = Some((component, triple_point_pressure));
                            }
                        } else {
                            possible_lowest_pressure = Some((component, triple_point_pressure));
                        }
                    }
                }
            }
        }

        if !possible_liquids.is_empty() {
            return Some(possible_liquids);
        } else if let Some((component, _)) = possible_lowest_pressure {
            return Some(vec![component]);
        }

        None
    }

    pub fn molecular_weight_kg(&self) -> f64 {
        const AMU_TO_KG: f64 = 1.66053906660e-27;
        self.molecular_weight_amu() * AMU_TO_KG
    }
}

impl Display for ChemicalComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChemicalComponent::Hydrogen => write!(f, "Hydrogen"),
            ChemicalComponent::Helium => write!(f, "Helium"),
            ChemicalComponent::Carbon => write!(f, "Carbon"),
            ChemicalComponent::Nitrogen => write!(f, "Nitrogen"),
            ChemicalComponent::Oxygen => write!(f, "Oxygen"),
            ChemicalComponent::Silicon => write!(f, "Silicon"),
            ChemicalComponent::Magnesium => write!(f, "Magnesium"),
            ChemicalComponent::Iron => write!(f, "Iron"),
            ChemicalComponent::Sulfur => write!(f, "Sulfur"),
            ChemicalComponent::Sodium => write!(f, "Sodium"),
            ChemicalComponent::Potassium => write!(f, "Potassium"),
            ChemicalComponent::Calcium => write!(f, "Calcium"),
            ChemicalComponent::Aluminum => write!(f, "Aluminum"),
            ChemicalComponent::Phosphorus => write!(f, "Phosphorus"),
            ChemicalComponent::Chlorine => write!(f, "Chlorine"),
            ChemicalComponent::Argon => write!(f, "Argon"),
            ChemicalComponent::Titanium => write!(f, "Titanium"),
            ChemicalComponent::Chromium => write!(f, "Chromium"),
            ChemicalComponent::Manganese => write!(f, "Manganese"),
            ChemicalComponent::Nickel => write!(f, "Nickel"),
            ChemicalComponent::Water => write!(f, "Water"),
            ChemicalComponent::CarbonMonoxide => write!(f, "Carbon Monoxide"),
            ChemicalComponent::CarbonDioxide => write!(f, "Carbon Dioxide"),
            ChemicalComponent::Methane => write!(f, "Methane"),
            ChemicalComponent::Ammonia => write!(f, "Ammonia"),
            ChemicalComponent::HydrogenSulfide => write!(f, "Hydrogen Sulfide"),
            ChemicalComponent::SulfurDioxide => write!(f, "Sulfur Dioxide"),
            ChemicalComponent::Hydroxyl => write!(f, "Hydroxyl"),
            ChemicalComponent::NitricOxide => write!(f, "Nitric Oxide"),
            ChemicalComponent::NitrogenDioxide => write!(f, "Nitrogen Dioxide"),
            ChemicalComponent::Formaldehyde => write!(f, "Formaldehyde"),
            ChemicalComponent::Methanol => write!(f, "Methanol"),
            ChemicalComponent::Ethylene => write!(f, "Ethylene"),
            ChemicalComponent::Ethane => write!(f, "Ethane"),
            ChemicalComponent::Acetylene => write!(f, "Acetylene"),
            ChemicalComponent::Benzene => write!(f, "Benzene"),
            ChemicalComponent::Acetonitrile => write!(f, "Acetonitrile"),
            ChemicalComponent::Methylamine => write!(f, "Methylamine"),
            ChemicalComponent::HydrogenCyanide => write!(f, "Hydrogen Cyanide"),
            ChemicalComponent::Glycine => write!(f, "Glycine"),
            ChemicalComponent::Silicates => write!(f, "Silicates"),
            ChemicalComponent::PolycyclicAromaticHydrocarbons => {
                write!(f, "Polycyclic Aromatic Hydrocarbons")
            }
        }
    }
}

pub const NON_METALS_ELEMENTS: [ChemicalComponent; 2] =
    [ChemicalComponent::Hydrogen, ChemicalComponent::Helium];

pub const MOST_COMMON_ELEMENTS: [ChemicalComponent; 23] = [
    ChemicalComponent::Hydrogen,
    ChemicalComponent::Helium,
    ChemicalComponent::Carbon,
    ChemicalComponent::Nitrogen,
    ChemicalComponent::Oxygen,
    ChemicalComponent::Silicon,
    ChemicalComponent::Magnesium,
    ChemicalComponent::Iron,
    ChemicalComponent::Sulfur,
    ChemicalComponent::Sodium,
    ChemicalComponent::Potassium,
    ChemicalComponent::Calcium,
    ChemicalComponent::Aluminum,
    ChemicalComponent::Phosphorus,
    ChemicalComponent::Chlorine,
    ChemicalComponent::Argon,
    ChemicalComponent::Water,
    ChemicalComponent::CarbonMonoxide,
    ChemicalComponent::CarbonDioxide,
    ChemicalComponent::Methane,
    ChemicalComponent::Ammonia,
    ChemicalComponent::HydrogenSulfide,
    ChemicalComponent::SulfurDioxide,
];

pub const ALL_ELEMENTS: [ChemicalComponent; 42] = [
    ChemicalComponent::Hydrogen,
    ChemicalComponent::Helium,
    ChemicalComponent::Carbon,
    ChemicalComponent::Nitrogen,
    ChemicalComponent::Oxygen,
    ChemicalComponent::Silicon,
    ChemicalComponent::Magnesium,
    ChemicalComponent::Iron,
    ChemicalComponent::Sulfur,
    ChemicalComponent::Sodium,
    ChemicalComponent::Potassium,
    ChemicalComponent::Calcium,
    ChemicalComponent::Aluminum,
    ChemicalComponent::Phosphorus,
    ChemicalComponent::Chlorine,
    ChemicalComponent::Argon,
    ChemicalComponent::Titanium,
    ChemicalComponent::Chromium,
    ChemicalComponent::Manganese,
    ChemicalComponent::Nickel,
    ChemicalComponent::Water,
    ChemicalComponent::CarbonMonoxide,
    ChemicalComponent::CarbonDioxide,
    ChemicalComponent::Methane,
    ChemicalComponent::Ammonia,
    ChemicalComponent::HydrogenSulfide,
    ChemicalComponent::SulfurDioxide,
    ChemicalComponent::Hydroxyl,
    ChemicalComponent::NitricOxide,
    ChemicalComponent::NitrogenDioxide,
    ChemicalComponent::Formaldehyde,
    ChemicalComponent::Methanol,
    ChemicalComponent::Ethylene,
    ChemicalComponent::Ethane,
    ChemicalComponent::Acetylene,
    ChemicalComponent::Benzene,
    ChemicalComponent::Acetonitrile,
    ChemicalComponent::Methylamine,
    ChemicalComponent::HydrogenCyanide,
    ChemicalComponent::Glycine,
    ChemicalComponent::Silicates,
    ChemicalComponent::PolycyclicAromaticHydrocarbons,
];

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum ElementPresenceOccurrence {
    Absence,
    VeryLow,
    Low,
    #[default]
    Normal,
    High,
    VeryHigh,
    Omnipresence,
}

impl Display for ElementPresenceOccurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementPresenceOccurrence::Absence => write!(f, "Absence"),
            ElementPresenceOccurrence::VeryLow => write!(f, "Very Low Occurrence"),
            ElementPresenceOccurrence::Low => write!(f, "Low Occurrence"),
            ElementPresenceOccurrence::Normal => write!(f, "Normal Occurrence"),
            ElementPresenceOccurrence::High => write!(f, "High Occurrence"),
            ElementPresenceOccurrence::VeryHigh => write!(f, "Very High Occurrence"),
            ElementPresenceOccurrence::Omnipresence => write!(f, "Omnipresence"),
        }
    }
}

pub(crate) fn generate_random_non_metal_element(rng: &mut SeededDiceRoller) -> ChemicalComponent {
    NON_METALS_ELEMENTS[rng.gen_range(0..NON_METALS_ELEMENTS.len())]
}

pub(crate) fn generate_random_common_element(rng: &mut SeededDiceRoller) -> ChemicalComponent {
    MOST_COMMON_ELEMENTS[rng.gen_range(0..MOST_COMMON_ELEMENTS.len())]
}

pub(crate) fn generate_random_element(rng: &mut SeededDiceRoller) -> ChemicalComponent {
    ALL_ELEMENTS[rng.gen_range(0..ALL_ELEMENTS.len())]
}

pub(crate) fn liquid_majority_composition_likelihood(
    component: ChemicalComponent,
    peculiarities: &[StarPeculiarity],
) -> f64 {
    let base_likelihood = match component {
        ChemicalComponent::Water => 1.0,
        ChemicalComponent::Methane => 0.8,
        ChemicalComponent::Ammonia => 0.6,
        ChemicalComponent::CarbonDioxide => 0.5,
        ChemicalComponent::SulfurDioxide => 0.4,
        ChemicalComponent::Formaldehyde => 0.2,
        ChemicalComponent::Hydrogen => 0.3,
        ChemicalComponent::Helium => 0.1,
        ChemicalComponent::Carbon => 0.2,
        ChemicalComponent::Nitrogen => 0.4,
        ChemicalComponent::Oxygen => 0.3,
        ChemicalComponent::Silicon => 0.1,
        ChemicalComponent::Magnesium => 0.1,
        ChemicalComponent::Iron => 0.1,
        ChemicalComponent::Sulfur => 0.3,
        ChemicalComponent::Sodium => 0.1,
        ChemicalComponent::Potassium => 0.1,
        ChemicalComponent::Calcium => 0.1,
        ChemicalComponent::Aluminum => 0.1,
        ChemicalComponent::Phosphorus => 0.2,
        ChemicalComponent::Chlorine => 0.3,
        ChemicalComponent::Argon => 0.1,
        ChemicalComponent::Titanium => 0.1,
        ChemicalComponent::Chromium => 0.1,
        ChemicalComponent::Manganese => 0.1,
        ChemicalComponent::Nickel => 0.1,
        ChemicalComponent::HydrogenSulfide => 0.3,
        ChemicalComponent::Hydroxyl => 0.1,
        ChemicalComponent::NitricOxide => 0.2,
        ChemicalComponent::NitrogenDioxide => 0.2,
        ChemicalComponent::Methanol => 0.4,
        ChemicalComponent::Ethylene => 0.3,
        ChemicalComponent::Ethane => 0.4,
        ChemicalComponent::Acetylene => 0.2,
        ChemicalComponent::Benzene => 0.3,
        ChemicalComponent::Acetonitrile => 0.2,
        ChemicalComponent::Methylamine => 0.2,
        ChemicalComponent::HydrogenCyanide => 0.2,
        ChemicalComponent::Glycine => 0.1,
        ChemicalComponent::Silicates => 0.1,
        ChemicalComponent::PolycyclicAromaticHydrocarbons => 0.2,
        ChemicalComponent::CarbonMonoxide => 0.4,
    };

    let stability = match component {
        ChemicalComponent::Hydrogen => 0.7,
        ChemicalComponent::Helium => 1.0,
        ChemicalComponent::Carbon => 0.9,
        ChemicalComponent::Nitrogen => 0.9,
        ChemicalComponent::Oxygen => 0.8,
        ChemicalComponent::Silicon => 0.8,
        ChemicalComponent::Magnesium => 0.7,
        ChemicalComponent::Iron => 0.7,
        ChemicalComponent::Sulfur => 0.6,
        ChemicalComponent::Sodium => 0.5,
        ChemicalComponent::Potassium => 0.5,
        ChemicalComponent::Calcium => 0.6,
        ChemicalComponent::Aluminum => 0.7,
        ChemicalComponent::Phosphorus => 0.6,
        ChemicalComponent::Chlorine => 0.6,
        ChemicalComponent::Argon => 1.0,
        ChemicalComponent::Titanium => 0.8,
        ChemicalComponent::Chromium => 0.7,
        ChemicalComponent::Manganese => 0.7,
        ChemicalComponent::Nickel => 0.7,
        ChemicalComponent::Water => 1.0,
        ChemicalComponent::CarbonMonoxide => 0.5,
        ChemicalComponent::CarbonDioxide => 0.9,
        ChemicalComponent::Methane => 0.8,
        ChemicalComponent::Ammonia => 0.6,
        ChemicalComponent::HydrogenSulfide => 0.4,
        ChemicalComponent::SulfurDioxide => 0.5,
        ChemicalComponent::Hydroxyl => 0.3,
        ChemicalComponent::NitricOxide => 0.4,
        ChemicalComponent::NitrogenDioxide => 0.4,
        ChemicalComponent::Formaldehyde => 0.2,
        ChemicalComponent::Methanol => 0.6,
        ChemicalComponent::Ethylene => 0.6,
        ChemicalComponent::Ethane => 0.7,
        ChemicalComponent::Acetylene => 0.5,
        ChemicalComponent::Benzene => 0.7,
        ChemicalComponent::Acetonitrile => 0.5,
        ChemicalComponent::Methylamine => 0.4,
        ChemicalComponent::HydrogenCyanide => 0.3,
        ChemicalComponent::Glycine => 0.8,
        ChemicalComponent::Silicates => 0.9,
        ChemicalComponent::PolycyclicAromaticHydrocarbons => 0.6,
    };

    let mut adjusted_likelihood = base_likelihood * stability;

    for peculiarity in peculiarities {
        if let StarPeculiarity::UnusualElementPresence((peculiar_component, occurrence)) =
            peculiarity
        {
            if *peculiar_component == component {
                adjusted_likelihood *= get_occurence_adjustment_factor(*occurrence);
            }
        }
    }

    adjusted_likelihood
}

fn get_occurence_adjustment_factor(occurrence: ElementPresenceOccurrence) -> f64 {
    match occurrence {
        ElementPresenceOccurrence::Absence => 0.0,
        ElementPresenceOccurrence::VeryLow => 0.2,
        ElementPresenceOccurrence::Low => 0.5,
        ElementPresenceOccurrence::Normal => 1.0,
        ElementPresenceOccurrence::High => 1.5,
        ElementPresenceOccurrence::VeryHigh => 2.0,
        ElementPresenceOccurrence::Omnipresence => 3.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molecular_weight_amu() {
        assert_eq!(ChemicalComponent::Hydrogen.molecular_weight_amu(), 1.008);
        assert_eq!(ChemicalComponent::Helium.molecular_weight_amu(), 4.0026);
        assert_eq!(ChemicalComponent::Oxygen.molecular_weight_amu(), 16.00);
        assert_eq!(ChemicalComponent::Water.molecular_weight_amu(), 18.015);
        assert_eq!(
            ChemicalComponent::CarbonDioxide.molecular_weight_amu(),
            44.01
        );
    }

    #[test]
    fn test_triple_point() {
        assert_eq!(
            ChemicalComponent::Hydrogen.triple_point(),
            Some((13.8, 0.070))
        );
        assert_eq!(
            ChemicalComponent::Helium.triple_point(),
            Some((2.2, 0.0052))
        );
        assert_eq!(
            ChemicalComponent::Oxygen.triple_point(),
            Some((54.36, 0.0015))
        );
        assert_eq!(
            ChemicalComponent::Water.triple_point(),
            Some((273.16, 0.00604))
        );
        assert_eq!(
            ChemicalComponent::CarbonDioxide.triple_point(),
            Some((216.55, 5.11))
        );
        assert_eq!(ChemicalComponent::Silicon.triple_point(), None);
    }

    #[test]
    fn test_boiling_point() {
        assert_eq!(ChemicalComponent::Hydrogen.boiling_point(), Some(20.28));
        assert_eq!(ChemicalComponent::Helium.boiling_point(), Some(4.22));
        assert_eq!(ChemicalComponent::Oxygen.boiling_point(), Some(90.20));
        assert_eq!(ChemicalComponent::Water.boiling_point(), Some(373.16));
        assert_eq!(ChemicalComponent::Silicon.boiling_point(), None);
    }

    #[test]
    fn test_can_exist_as_liquid() {
        assert!(ChemicalComponent::Water.can_exist_as_liquid(280, 1.0));
        assert!(!ChemicalComponent::Water.can_exist_as_liquid(280, 0.001));
        assert!(ChemicalComponent::Oxygen.can_exist_as_liquid(60, 0.002));
        assert!(!ChemicalComponent::Oxygen.can_exist_as_liquid(60, 0.0005));
        assert!(ChemicalComponent::CarbonDioxide.can_exist_as_liquid(220, 6.0));
        assert!(!ChemicalComponent::CarbonDioxide.can_exist_as_liquid(220, 4.0));
    }

    #[test]
    fn test_can_exist_as_gas() {
        assert!(ChemicalComponent::Water.can_exist_as_gas(400, 1.0));
        assert!(ChemicalComponent::Water.can_exist_as_gas(300, 0.005));
        assert!(ChemicalComponent::Oxygen.can_exist_as_gas(100, 0.001));
        assert!(!ChemicalComponent::Oxygen.can_exist_as_gas(50, 1.0));
        assert!(ChemicalComponent::CarbonDioxide.can_exist_as_gas(220, 5.0));
        assert!(!ChemicalComponent::CarbonDioxide.can_exist_as_gas(220, 6.0));
    }

    #[test]
    fn test_components_liquid_at() {
        let components = ChemicalComponent::components_liquid_at(280, 1.0);
        assert!(components.is_some());
        let components = components.unwrap();
        assert!(components.contains(&ChemicalComponent::Water));
        assert!(!components.contains(&ChemicalComponent::Oxygen));

        let components = ChemicalComponent::components_liquid_at(60, 0.002);
        assert!(components.is_some());
        let components = components.unwrap();
        assert!(components.contains(&ChemicalComponent::Oxygen));
        assert!(!components.contains(&ChemicalComponent::Water));

        let components = ChemicalComponent::components_liquid_at(220, 6.0);
        assert!(components.is_some());
        let components = components.unwrap();
        assert!(components.contains(&ChemicalComponent::CarbonDioxide));
    }

    #[test]
    fn test_molecular_weight_kg() {
        let amu_to_kg = |amu: f64| amu * 1.66053906660e-27;
        assert_eq!(
            ChemicalComponent::Hydrogen.molecular_weight_kg(),
            amu_to_kg(1.008)
        );
        assert_eq!(
            ChemicalComponent::Helium.molecular_weight_kg(),
            amu_to_kg(4.0026)
        );
        assert_eq!(
            ChemicalComponent::Oxygen.molecular_weight_kg(),
            amu_to_kg(16.00)
        );
        assert_eq!(
            ChemicalComponent::Water.molecular_weight_kg(),
            amu_to_kg(18.015)
        );
        assert_eq!(
            ChemicalComponent::CarbonDioxide.molecular_weight_kg(),
            amu_to_kg(44.01)
        );
    }
}
