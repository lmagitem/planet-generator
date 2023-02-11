use crate::prelude::*;
pub mod generator;
pub mod types;

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
}
