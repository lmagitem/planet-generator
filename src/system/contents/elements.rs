use crate::internal::*;
use crate::prelude::*;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum ChemicalElement {
    #[default]
    Hydrogen,
    Helium,
    Methane,
    Ammonia,
    WaterVapor,
    Neon,
    Nitrogen,
    CarbonMonoxide,
    Oxygen,
    Fluorine,
    Argon,
    CarbonDioxide,
    Ozone,
    SulfurDioxide,
    Krypton,
    Xenon,
    Ethane,
    Propane,
    Butane,
    Phosphine,
    HydrogenSulfide,
    NitrousOxide,
    Chlorine,
    HydrochloricAcid,
    NitricOxide,
    HydrogenCyanide,
    Acetylene,
    Benzene,
    CarbonylSulfide,
    Chloromethane,
    Dinitrogen,
    Ethylene,
    Formaldehyde,
    Hydrazine,
    HydrogenBromide,
    HydrogenFluoride,
    HydrogenPeroxide,
    Methanol,
    Methylamine,
    NitrogenDioxide,
    NitrogenTrioxide,
    NitrogenPentoxide,
    Silane,
    SulfurHexafluoride,
    Tetrafluoromethane,
}

impl ChemicalElement {
    pub fn molecular_weight_amu(&self) -> f64 {
        match self {
            ChemicalElement::Hydrogen => 2.016, // H2
            ChemicalElement::Helium => 4.0026,
            ChemicalElement::Methane => 16.04,     // CH4
            ChemicalElement::Ammonia => 17.031,    // NH3
            ChemicalElement::WaterVapor => 18.015, // H2O
            ChemicalElement::Neon => 20.1797,
            ChemicalElement::Nitrogen => 28.014,      // N2
            ChemicalElement::CarbonMonoxide => 28.01, // CO
            ChemicalElement::Oxygen => 32.00,         // O2
            ChemicalElement::Fluorine => 38.00,       // F2
            ChemicalElement::Argon => 39.948,
            ChemicalElement::CarbonDioxide => 44.01,  // CO2
            ChemicalElement::Ozone => 48.00,          // O3
            ChemicalElement::SulfurDioxide => 64.066, // SO2
            ChemicalElement::Krypton => 83.798,
            ChemicalElement::Xenon => 131.293,
            ChemicalElement::Ethane => 30.07,               // C2H6
            ChemicalElement::Propane => 44.097,             // C3H8
            ChemicalElement::Butane => 58.124,              // C4H10
            ChemicalElement::Phosphine => 33.998,           // PH3
            ChemicalElement::HydrogenSulfide => 34.08,      // H2S
            ChemicalElement::NitrousOxide => 44.013,        // N2O
            ChemicalElement::Chlorine => 70.906,            // Cl2
            ChemicalElement::HydrochloricAcid => 36.46,     // HCl
            ChemicalElement::NitricOxide => 30.006,         // NO
            ChemicalElement::HydrogenCyanide => 27.0253,    // HCN
            ChemicalElement::Acetylene => 26.04,            // C2H2
            ChemicalElement::Benzene => 78.11,              // C6H6
            ChemicalElement::CarbonylSulfide => 60.07,      // OCS
            ChemicalElement::Chloromethane => 50.49,        // CH3Cl
            ChemicalElement::Dinitrogen => 28.014,          // N2
            ChemicalElement::Ethylene => 28.05,             // C2H4
            ChemicalElement::Formaldehyde => 30.03,         // CH2O
            ChemicalElement::Hydrazine => 32.045,           // N2H4
            ChemicalElement::HydrogenBromide => 80.91,      // HBr
            ChemicalElement::HydrogenFluoride => 20.01,     // HF
            ChemicalElement::HydrogenPeroxide => 34.0147,   // H2O2
            ChemicalElement::Methanol => 32.04,             // CH3OH
            ChemicalElement::Methylamine => 31.06,          // CH3NH2
            ChemicalElement::NitrogenDioxide => 46.0055,    // NO2
            ChemicalElement::NitrogenTrioxide => 76.01,     // N2O3
            ChemicalElement::NitrogenPentoxide => 108.01,   // N2O5
            ChemicalElement::Silane => 32.12,               // SiH4
            ChemicalElement::SulfurHexafluoride => 146.06,  // SF6
            ChemicalElement::Tetrafluoromethane => 88.0043, // CF4
        }
    }

    pub fn molecular_weight_kg(&self) -> f64 {
        const AMU_TO_KG: f64 = 1.66053906660e-27;
        self.molecular_weight_amu() * AMU_TO_KG
    }
}

pub const NON_METALS_ELEMENTS: [ChemicalElement; 2] =
    [ChemicalElement::Hydrogen, ChemicalElement::Helium];

pub const MOST_COMMON_ELEMENTS: [ChemicalElement; 23] = [
    ChemicalElement::Hydrogen,
    ChemicalElement::Helium,
    ChemicalElement::Methane,
    ChemicalElement::Ammonia,
    ChemicalElement::WaterVapor,
    ChemicalElement::Neon,
    ChemicalElement::Nitrogen,
    ChemicalElement::CarbonMonoxide,
    ChemicalElement::Oxygen,
    ChemicalElement::Fluorine,
    ChemicalElement::Argon,
    ChemicalElement::CarbonDioxide,
    ChemicalElement::Ozone,
    ChemicalElement::SulfurDioxide,
    ChemicalElement::Krypton,
    ChemicalElement::Xenon,
    ChemicalElement::Ethane,
    ChemicalElement::Propane,
    ChemicalElement::Butane,
    ChemicalElement::Phosphine,
    ChemicalElement::HydrogenSulfide,
    ChemicalElement::NitrousOxide,
    ChemicalElement::Chlorine,
];

pub const ALL_ELEMENTS: [ChemicalElement; 45] = [
    ChemicalElement::Hydrogen,
    ChemicalElement::Helium,
    ChemicalElement::Methane,
    ChemicalElement::Ammonia,
    ChemicalElement::WaterVapor,
    ChemicalElement::Neon,
    ChemicalElement::Nitrogen,
    ChemicalElement::CarbonMonoxide,
    ChemicalElement::Oxygen,
    ChemicalElement::Fluorine,
    ChemicalElement::Argon,
    ChemicalElement::CarbonDioxide,
    ChemicalElement::Ozone,
    ChemicalElement::SulfurDioxide,
    ChemicalElement::Krypton,
    ChemicalElement::Xenon,
    ChemicalElement::Ethane,
    ChemicalElement::Propane,
    ChemicalElement::Butane,
    ChemicalElement::Phosphine,
    ChemicalElement::HydrogenSulfide,
    ChemicalElement::NitrousOxide,
    ChemicalElement::Chlorine,
    ChemicalElement::HydrochloricAcid,
    ChemicalElement::NitricOxide,
    ChemicalElement::HydrogenCyanide,
    ChemicalElement::Acetylene,
    ChemicalElement::Benzene,
    ChemicalElement::CarbonylSulfide,
    ChemicalElement::Chloromethane,
    ChemicalElement::Dinitrogen,
    ChemicalElement::Ethylene,
    ChemicalElement::Formaldehyde,
    ChemicalElement::Hydrazine,
    ChemicalElement::HydrogenBromide,
    ChemicalElement::HydrogenFluoride,
    ChemicalElement::HydrogenPeroxide,
    ChemicalElement::Methanol,
    ChemicalElement::Methylamine,
    ChemicalElement::NitrogenDioxide,
    ChemicalElement::NitrogenTrioxide,
    ChemicalElement::NitrogenPentoxide,
    ChemicalElement::Silane,
    ChemicalElement::SulfurHexafluoride,
    ChemicalElement::Tetrafluoromethane,
];

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum ElementPresenceOccurence {
    Absence,
    MuchLower,
    Lower,
    #[default]
    Normal,
    Higher,
    MuchHigher,
    Omnipresence,
}

impl Display for ElementPresenceOccurence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementPresenceOccurence::Absence => write!(f, "Absence"),
            ElementPresenceOccurence::MuchLower => write!(f, "Much Lower"),
            ElementPresenceOccurence::Lower => write!(f, "Lower"),
            ElementPresenceOccurence::Normal => write!(f, "Normal"),
            ElementPresenceOccurence::Higher => write!(f, "Higher"),
            ElementPresenceOccurence::MuchHigher => write!(f, "Much Higher"),
            ElementPresenceOccurence::Omnipresence => write!(f, "Omnipresence"),
        }
    }
}

pub(crate) fn generate_random_non_metal_element(rng: &mut SeededDiceRoller) -> ChemicalElement {
    NON_METALS_ELEMENTS[rng.gen_range(0..NON_METALS_ELEMENTS.len())]
}

pub(crate) fn generate_random_common_element(rng: &mut SeededDiceRoller) -> ChemicalElement {
    MOST_COMMON_ELEMENTS[rng.gen_range(0..MOST_COMMON_ELEMENTS.len())]
}

pub(crate) fn generate_random_element(rng: &mut SeededDiceRoller) -> ChemicalElement {
    ALL_ELEMENTS[rng.gen_range(0..ALL_ELEMENTS.len())]
}
