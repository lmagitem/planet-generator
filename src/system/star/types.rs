use crate::prelude::*;

/// A list of settings used to configure the [Star] generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct StarSettings {
    /// The specific age to use for star generation, if any. In billions of years.
    pub fixed_age: Option<f32>,
    /// The specific mass to use for star generation, if any. Only applies during the lifespan of the star, in other words, if the star
    /// is older than its estimated lifespan, it will be generated as a remnant, with a new mass calculated using the given mass.
    pub fixed_mass: Option<f32>,
    /// Skip the star generation and just uses a copy of ours.
    pub use_ours: bool,
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum StarSpectralType {
    WR(u8),
    O(u8),
    B(u8),
    A(u8),
    F(u8),
    #[default]
    G(#[default = 2] u8),
    K(u8),
    M(u8),
    L(u8),
    T(u8),
    Y(u8),
    DA,
    DB,
    DC,
    DO,
    DZ,
    DQ,
    DX,
    // Made up category to indicate a neutron star
    XNS,
    // Made up category to indicate a black hole
    XBH,
}

impl Display for StarSpectralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarSpectralType::WR(d) => write!(f, "WR{}", d),
            StarSpectralType::O(d) => write!(f, "O{}", d),
            StarSpectralType::B(d) => write!(f, "B{}", d),
            StarSpectralType::A(d) => write!(f, "A{}", d),
            StarSpectralType::F(d) => write!(f, "F{}", d),
            StarSpectralType::G(d) => write!(f, "G{}", d),
            StarSpectralType::K(d) => write!(f, "K{}", d),
            StarSpectralType::M(d) => write!(f, "M{}", d),
            StarSpectralType::L(d) => write!(f, "L{}", d),
            StarSpectralType::T(d) => write!(f, "T{}", d),
            StarSpectralType::Y(d) => write!(f, "Y{}", d),
            StarSpectralType::DA => write!(f, "DA"),
            StarSpectralType::DB => write!(f, "DB"),
            StarSpectralType::DC => write!(f, "DC"),
            StarSpectralType::DO => write!(f, "DO"),
            StarSpectralType::DZ => write!(f, "DZ"),
            StarSpectralType::DQ => write!(f, "DQ"),
            StarSpectralType::DX => write!(f, "DX"),
            StarSpectralType::XNS => write!(f, "Neutron Star"),
            StarSpectralType::XBH => write!(f, "Black Hole"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum StarLuminosityClass {
    /// Hypergiant
    O,
    /// Luminous supergiant
    Ia,
    /// Less luminous supergiant
    Ib,
    /// Bright giants
    II,
    /// Normal giants
    III,
    /// Subgiants
    IV,
    /// Main sequence
    #[default]
    V,
    /// Subdwarfs
    VI,
    /// White dwarfs
    VII,
    /// Brown dwarfs
    Y,
    /// Made up category to indicate a neutron star
    XNS,
    /// Made up category to indicate a black hole
    XBH,
}

impl Display for StarLuminosityClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarLuminosityClass::O => write!(f, "O"),
            StarLuminosityClass::Ia => write!(f, "Ia"),
            StarLuminosityClass::Ib => write!(f, "Ib"),
            StarLuminosityClass::II => write!(f, "II"),
            StarLuminosityClass::III => write!(f, "III"),
            StarLuminosityClass::IV => write!(f, "IV"),
            StarLuminosityClass::V => write!(f, "V"),
            StarLuminosityClass::VI => write!(f, "VI"),
            StarLuminosityClass::VII => write!(f, "VII"),
            _ => write!(f, ""),
        }
    }
}
