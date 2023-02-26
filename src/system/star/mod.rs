use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Star {
    // That star's name.
    pub name: String,
    /// In solar masses.
    pub mass: f32,
    /// In solar luminosities.
    pub luminosity: f32,
    /// In solar radii.
    pub radius: f32,
    /// In billions of years.
    pub age: f32,
    /// In kelvins.
    pub temperature: u32,
    /// Spectral type.
    pub spectral_type: StarSpectralType,
    /// Luminosity class.
    pub luminosity_class: StarLuminosityClass,
}

impl Star {
    pub fn new(
        name: String,
        mass: f32,
        luminosity: f32,
        radius: f32,
        age: f32,
        temperature: u32,
        spectral_type: StarSpectralType,
        luminosity_class: StarLuminosityClass,
    ) -> Self {
        Self {
            name,
            mass,
            luminosity,
            radius,
            age,
            temperature,
            spectral_type,
            luminosity_class,
        }
    }

    /// Returns true if the star is currently in the main sequence phase of its life.
    pub fn is_main_sequence_dwarf(self) -> bool {
        (self.luminosity_class == StarLuminosityClass::V
            || self.luminosity_class == StarLuminosityClass::IV)
            && (discriminant(&self.spectral_type) == discriminant(&StarSpectralType::WR(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::O(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::B(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::A(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::F(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::G(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::K(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::M(0)))
    }

    /// Returns true if the star is currently in the main sequence, subgiant or giant phase of its life.
    pub fn is_main_sequence_or_giant(self) -> bool {
        (self.luminosity_class == StarLuminosityClass::O
            || self.luminosity_class == StarLuminosityClass::Ia
            || self.luminosity_class == StarLuminosityClass::Ib
            || self.luminosity_class == StarLuminosityClass::II
            || self.luminosity_class == StarLuminosityClass::III
            || self.luminosity_class == StarLuminosityClass::IV
            || self.luminosity_class == StarLuminosityClass::V
            || self.luminosity_class == StarLuminosityClass::IV)
            && (discriminant(&self.spectral_type) == discriminant(&StarSpectralType::WR(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::O(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::B(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::A(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::F(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::G(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::K(0))
                || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::M(0)))
    }
}
