use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct Star {
    /// That star's name.
    #[default("default")]
    pub name: Rc<str>,
    /// In solar masses.
    pub mass: f64,
    /// In solar luminosities.
    pub luminosity: f32,
    /// In solar radii.
    pub radius: f64,
    /// In billions of years.
    pub age: f32,
    /// In kelvins.
    pub temperature: u32,
    /// The population this star belongs to.
    pub population: StellarEvolution,
    /// Spectral type.
    pub spectral_type: StarSpectralType,
    /// Luminosity class.
    pub luminosity_class: StarLuminosityClass,
    /// The id of the orbital point this star inhabits.
    pub orbital_point_id: u32,
    /// The star's own orbit, along which it revolves.
    pub orbit: Option<Orbit>,
    /// The various zones around this star. The zones give various informations about star orbits.
    pub zones: Vec<StarZone>,
    /// What are the pecularities of this star.
    pub special_traits: Vec<StarPeculiarity>,
}

impl Star {
    pub fn new(
        name: Rc<str>,
        mass: f64,
        luminosity: f32,
        radius: f64,
        age: f32,
        temperature: u32,
        population: StellarEvolution,
        spectral_type: StarSpectralType,
        luminosity_class: StarLuminosityClass,
        special_traits: Vec<StarPeculiarity>,
        orbital_point_id: u32,
        orbit: Option<Orbit>,
        zones: Vec<StarZone>,
    ) -> Self {
        Self {
            name,
            mass,
            luminosity,
            radius,
            age,
            temperature,
            population,
            spectral_type,
            luminosity_class,
            special_traits,
            orbital_point_id,
            orbit,
            zones,
        }
    }

    /// Returns true if the star is currently in the main sequence phase of its life.
    pub fn is_main_sequence_dwarf(&self) -> bool {
        (self.luminosity_class == StarLuminosityClass::V
            || self.luminosity_class == StarLuminosityClass::IV)
            && self.is_more_luminous_than_brown_dwarf()
    }

    /// Returns true if the star is currently in the main sequence, subgiant or giant phase of its life.
    pub fn is_main_sequence_or_giant(&self) -> bool {
        (self.luminosity_class == StarLuminosityClass::O
            || self.luminosity_class == StarLuminosityClass::Ia
            || self.luminosity_class == StarLuminosityClass::Ib
            || self.luminosity_class == StarLuminosityClass::II
            || self.luminosity_class == StarLuminosityClass::III
            || self.luminosity_class == StarLuminosityClass::IV
            || self.luminosity_class == StarLuminosityClass::V
            || self.luminosity_class == StarLuminosityClass::IV)
            && self.is_more_luminous_than_brown_dwarf()
    }

    /// Returns true if the star is of a higher spectral type than a brown dwarf.
    pub fn is_more_luminous_than_brown_dwarf(&self) -> bool {
        discriminant(&self.spectral_type) == discriminant(&StarSpectralType::WR(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::O(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::B(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::A(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::F(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::G(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::K(0))
            || discriminant(&self.spectral_type) == discriminant(&StarSpectralType::M(0))
    }

    /// Returns the beggining of the minimum orbital separation between this object and the one it orbits in AU.
    pub fn get_minimum_orbital_separation(&self) -> f64 {
        ((1.0
            - self
                .orbit
                .clone()
                .unwrap_or(Orbit {
                    ..Default::default()
                })
                .eccentricity) as f64
            * self.radius) as f64
    }

    /// Returns the end of the minimum orbital separation between this object and the one it orbits in AU.
    pub fn get_maximum_orbital_separation(&self) -> f64 {
        ((1.0
            + self
                .orbit
                .clone()
                .unwrap_or(Orbit {
                    ..Default::default()
                })
                .eccentricity) as f64
            * self.radius) as f64
    }
}

pub(crate) fn get_star_color_code(star: &Star) -> &'static str {
    match star.spectral_type {
        StarSpectralType::WR(_) | StarSpectralType::O(_) => "\x1b[34m", // Blue
        StarSpectralType::B(_) => "\x1b[1;34m",                         // Bright Blue
        StarSpectralType::A(_) => "\x1b[1;37m",                         // Bright White
        StarSpectralType::F(_) => "\x1b[1;33m",                         // Bright Yellow
        StarSpectralType::G(_) => "\x1b[33m",                           // Yellow
        StarSpectralType::K(_) => "\x1b[1;31m", // Bright Red (as a stand-in for orange)
        StarSpectralType::M(_) => "\x1b[31m",   // Red
        StarSpectralType::L(_) | StarSpectralType::T(_) | StarSpectralType::Y(_) => "\x1b[31m", // Red
        StarSpectralType::DA
        | StarSpectralType::DB
        | StarSpectralType::DC
        | StarSpectralType::DO
        | StarSpectralType::DZ
        | StarSpectralType::DQ
        | StarSpectralType::DX => "\x1b[1;37m", // Bright White
        StarSpectralType::XNS => "\x1b[1;34m", // Bright Blue
        _ => "",
    }
}
