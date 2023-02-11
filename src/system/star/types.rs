use crate::prelude::*;

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
    DA(u8),
    DB(u8),
    DC(u8),
    DO(u8),
    DZ(u8),
    DQ(u8),
    DX(u8),
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
            StarSpectralType::DA(d) => write!(f, "DA{}", d),
            StarSpectralType::DB(d) => write!(f, "DB{}", d),
            StarSpectralType::DC(d) => write!(f, "DC{}", d),
            StarSpectralType::DO(d) => write!(f, "DO{}", d),
            StarSpectralType::DZ(d) => write!(f, "DZ{}", d),
            StarSpectralType::DQ(d) => write!(f, "DQ{}", d),
            StarSpectralType::DX(d) => write!(f, "DX{}", d),
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
        }
    }
}
