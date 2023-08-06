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

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct StarZone {
    /// The beginning of the zone in AU
    pub start: f64,
    /// The end of the zone in AU
    pub end: f64,
    /// The type of the zone
    pub zone_type: ZoneType,
}

impl StarZone {
    pub fn new(start: f64, end: f64, zone_type: ZoneType) -> Self {
        Self {
            start,
            end,
            zone_type,
        }
    }

    /// Returns true if the current zone is entirely inside a given zone.
    pub fn is_inside(&self, other: &StarZone) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    /// Returns true if the current zone overlaps with a given zone at all.
    pub fn is_overlapping(&self, other: &StarZone) -> bool {
        self.start < other.end && self.end > other.start
    }

    /// Adjusts the current zone to avoid overlap with a given zone.
    pub fn adjust_for_overlap(&mut self, other: &Self) -> Option<StarZone> {
        if other.start > self.start && other.end < self.end {
            let new_zone = StarZone::new(other.end, self.end, self.zone_type);
            self.end = other.start;
            Some(new_zone)
        } else if other.start > self.start && other.start < self.end {
            self.end = other.start;
            None
        } else if other.end > self.start && other.end < self.end {
            self.start = other.end;
            None
        } else {
            None
        }
    }

    /// Returns true if the current zone fully contains the other zone.
    pub fn contains(&self, other: &StarZone) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Splits the current zone into two zones at the boundaries of the other zone.
    /// Returns the two new zones. If the other zone is not fully contained within the current zone, returns None.
    pub fn split(&self, other: &StarZone) -> Option<(StarZone, StarZone)> {
        if self.contains(other) {
            let first_zone = StarZone::new(self.start, other.start, self.zone_type);
            let second_zone = StarZone::new(other.end, self.end, self.zone_type);
            Some((first_zone, second_zone))
        } else {
            None
        }
    }
}

impl Display for StarZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} from {:.5}AU to {:.5}AU", self.zone_type, self.start, self.end)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub enum ZoneType {
    /// Everything in this zone lies within the star's corona.
    Corona,
    /// The gravitational forces from the star would tear apart any forming planet in this zone.
    /// No stable planet can form or exist here.
    InnerLimit,
    /// In this region, we can find every orbit that lies before the "Snow Line" (also known as the
    /// "Frost Line" or "Ice Line"), which means that substances such as water, ammonia, methane,
    /// and carbon dioxide not situated on planets can only be found in gaseous form.
    InnerZone,
    /// This is the region where liquid water would be the most likely to exist on a planet's
    /// surface in the system, given suitable atmospheric conditions.
    BioZone,
    /// In this region, we can find every orbit strong enough to hold planets that lies after the
    /// "Snow Line" (also known as the "Frost Line" or "Ice Line"), which means that substances such
    /// as water, ammonia, methane, and carbon dioxide can condense into solid ice.
    OuterZone,
    /// No planet can maintain a stable orbit here, either because of the attraction of another body
    /// or because it's too far from a star.
    #[default]
    ForbiddenZone,
}

impl Display for ZoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZoneType::Corona => write!(f, "Star Corona"),
            ZoneType::InnerLimit => write!(f, "Inner Limit"),
            ZoneType::InnerZone => write!(f, "Inner Zone"),
            ZoneType::BioZone => write!(f, "Bio Zone"),
            ZoneType::OuterZone => write!(f, "Outer Zone"),
            ZoneType::ForbiddenZone => write!(f, "Forbidden Zone"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub enum StarPeculiarity {
    /// All planets, stars and other objects around this star are variably aligned to the ecliptic plane.
    ChaoticOrbits,
    ExcessiveRadiation,
    AgeDifference(StarAgeDifference),
    RotationAnomaly(RotationAnomalySpeed),
    UnusualMetallicity(StarMetallicityDifference),
    PowerfulStellarWinds,
    StrongMagneticField,
    VariableStar,
    #[default]
    NoPeculiarity,
}

impl Display for StarPeculiarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarPeculiarity::ChaoticOrbits => write!(f, "Chaotic Orbits"),
            StarPeculiarity::ExcessiveRadiation => write!(f, "Excessive Radiation"),
            StarPeculiarity::AgeDifference(difference) => write!(f, "{} Star", difference),
            StarPeculiarity::RotationAnomaly(speed) => write!(f, "{} Rotation Speed", speed),
            StarPeculiarity::UnusualMetallicity(difference) => write!(f, "{} Metallicity", difference),
            StarPeculiarity::PowerfulStellarWinds => write!(f, "Powerful Stellar Winds"),
            StarPeculiarity::StrongMagneticField => write!(f, "Strong Magnetic Field"),
            StarPeculiarity::VariableStar => write!(f, "Variable Star"),
            StarPeculiarity::NoPeculiarity => write!(f, "No Peculiarity"),
        }
    }
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
            StarLuminosityClass::Y => write!(f, "Y"),
            _ => write!(f, ""),
        }
    }
}

#[derive(
Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum StarAgeDifference {
    MuchOlder,
    #[default]
    Older,
    Younger,
    MuchYounger,
}

impl Display for StarAgeDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarAgeDifference::MuchOlder => write!(f, "Much Older"),
            StarAgeDifference::Older => write!(f, "Older"),
            StarAgeDifference::Younger => write!(f, "Younger"),
            StarAgeDifference::MuchYounger => write!(f, "Much Younger"),
        }
    }
}

#[derive(
Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum RotationAnomalySpeed {
    MuchSlower,
    #[default]
    Slower,
    Faster,
    MuchFaster,
}

impl Display for RotationAnomalySpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RotationAnomalySpeed::MuchSlower => write!(f, "Much Slower"),
            RotationAnomalySpeed::Slower => write!(f, "Slower"),
            RotationAnomalySpeed::Faster => write!(f, "Faster"),
            RotationAnomalySpeed::MuchFaster => write!(f, "Much Faster"),
        }
    }
}

#[derive(
Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum StarMetallicityDifference {
    MuchLower,
    #[default]
    Lower,
    Higher,
    MuchHigher,
}

impl Display for StarMetallicityDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarMetallicityDifference::MuchLower => write!(f, "Much Lower"),
            StarMetallicityDifference::Lower => write!(f, "Lower"),
            StarMetallicityDifference::Higher => write!(f, "Higher"),
            StarMetallicityDifference::MuchHigher => write!(f, "Much Higher"),
        }
    }
}
