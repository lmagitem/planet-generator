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
        write!(
            f,
            "{} from {:.5}AU to {:.5}AU",
            self.zone_type, self.start, self.end
        )
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
    /// All planets, stars and other objects around this star are variably aligned to the ecliptic
    /// plane.
    ChaoticOrbits,
    /// The star emits excessive radiations, affecting the habitability and atmospheres of the
    /// planets orbiting it.
    ExcessiveRadiation,
    /// The star is of a different age than the other ones in this region.
    AgeDifference(StarAgeDifference),
    /// The star rotates at an unusual speed, which has an impact on its system and lifespan.
    RotationAnomaly(RotationAnomalySpeed),
    /// The star has an unusual metallicity (the abundance of elements heavier than hydrogen and
    /// helium in its composition) for its region.
    UnusualMetallicity(StarMetallicityDifference),
    /// The star emits very powerful stellar winds which shortens its lifespan. Also has a
    /// detrimental effect on the atmospheres of planets that haven't strong enough, and on planets
    /// habitability (as stellar winds cause radiation).
    PowerfulStellarWinds,
    /// The star has a strong magnetic field, which is often correlated with stronger stellar winds,
    /// bigger and more frequent solar flares and stellar anomalies. Those points have a detrimental
    /// effect on habitability and planets atmospheres. The magnetic field might also interact with
    /// planets magnetic fields and lead to intense aurora and radiation belts.
    StrongMagneticField,
    /// The star is one whose brightness or magnitude changes over time. The reasons could cause a
    /// lot of radiations, and depending on the interval, changes in stellar brightness could be
    /// very bad for its planets' climate and habitability. If the behavior is predictable and
    /// regular it can make the star a very useful astronomical "beacon" for systems in this
    /// region of the galaxy.
    VariableStar(VariableStarInterval),
    /// The planetary formation around this star wasn't very successful. As a result, the star is
    /// still orbited by a dust-rich circumstellar disk.
    CircumstellarDisk,
    /// The star seems perfectly standard for its size and type.
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
            StarPeculiarity::UnusualMetallicity(difference) => {
                write!(f, "{} Metallicity", difference)
            }
            StarPeculiarity::PowerfulStellarWinds => write!(f, "Powerful Stellar Winds"),
            StarPeculiarity::StrongMagneticField => write!(f, "Strong Magnetic Field"),
            StarPeculiarity::VariableStar(interval) => {
                write!(f, "Variable Star ({} Interval)", interval)
            }
            StarPeculiarity::CircumstellarDisk => write!(f, "Circumstellar Disk"),
            StarPeculiarity::NoPeculiarity => write!(f, "No Peculiarity"),
        }
    }
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum StarSpectralType {
    /// Wolf-Rayet stars, known for strong stellar winds and emission lines.
    WR(u8),
    /// O-type stars, hot and blue with temperatures over 30,000 K.
    O(u8),
    /// B-type stars, blue-white and very luminous.
    B(u8),
    /// A-type stars, white or bluish-white with strong hydrogen lines.
    A(u8),
    /// F-type stars, yellow-white with moderate temperatures.
    F(u8),
    /// G-type stars, like our Sun, yellow and well-balanced.
    #[default]
    G(#[default = 2] u8),
    /// K-type stars, orange to red, cooler than our Sun.
    K(u8),
    /// M-type stars, red dwarfs, coolest and most common.
    M(u8),
    /// L-type brown dwarfs, cooler than M dwarfs with metal hydride bands.
    L(u8),
    /// T-type brown dwarfs, characterized by methane absorption.
    T(u8),
    /// Y-type brown dwarfs, coolest known objects with temperatures below 500 K.
    Y(u8),
    /// DA white dwarfs, with hydrogen-rich atmospheres.
    DA,
    /// DB white dwarfs, with helium-rich atmospheres, no hydrogen lines.
    DB,
    /// DC white dwarfs, with no strong spectral lines.
    DC,
    /// DO white dwarfs, with helium-rich atmospheres, showing ionized helium.
    DO,
    /// DZ white dwarfs, with metal-rich atmospheres.
    DZ,
    /// DQ white dwarfs, with carbon-rich atmospheres.
    DQ,
    /// DX white dwarfs, with unidentified spectral lines.
    DX,
    /// Fictional category for neutron stars, ultra-dense remnants of supernovae.
    XNS,
    /// Fictional category for black holes, regions of spacetime where gravity pulls so much that nothing can escape.
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
    /// Hypergiants, extremely luminous and massive stars.
    O,
    /// Luminous supergiants, very bright with high luminosity.
    Ia,
    /// Less luminous supergiants, still very bright but with slightly lower luminosity than Ia.
    Ib,
    /// Bright giants, stars that have left the main sequence and are brighter than normal giants.
    II,
    /// Normal giants, evolved stars that have left the main sequence.
    III,
    /// Subgiants, stars transitioning from the main sequence to the giant phase.
    IV,
    /// Main sequence stars, like our Sun, that fuse hydrogen in their cores.
    #[default]
    V,
    /// Subdwarfs, stars with lower luminosity than main sequence stars.
    VI,
    /// White dwarfs, remnants of stars that have exhausted their nuclear fuel.
    VII,
    /// Brown dwarfs, objects with insufficient mass to sustain hydrogen fusion.
    Y,
    /// Fictional category for neutron stars, ultra-dense remnants of supernovae.
    XNS,
    /// Fictional category for black holes, regions of spacetime where gravity pulls so much that nothing can escape.
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
    /// If the star has a high mass, it'll have a longer lifespan. It also has a much weaker
    /// magnetic field because of its abnormal speed, which means less protection for the star's
    /// planets from cosmic rays and interstellar radiation. It also has reduced stellar winds,
    /// which might mean more stable and thick planet atmospheres.
    MuchSlower,
    /// If the star has a high mass, it'll have a longer lifespan. It also has a weaker
    /// magnetic field because of its abnormal speed, which means less protection for the star's
    /// planets from cosmic rays and interstellar radiation. It also has reduced stellar winds,
    /// which might mean more stable and thick planet atmospheres.
    #[default]
    Slower,
    /// If the star has a high mass, it'll have a shorter lifespan. It also have a stronger magnetic
    /// field, which protects the star's planets from cosmic rays and interstellar radiations. But
    /// it also leads to more frequent and intense solar flares and stellar winds. Said stellar
    /// winds could have a detrimental effect on the atmospheres in the system. Finally, the
    /// radiation emitted by the star would also be higher, which poses a threat to any potential
    /// life.
    Faster,
    /// If the star has a high mass, it'll have a shorter lifespan. It also have a much stronger
    /// magnetic field, which protects the star's planets from cosmic rays and interstellar
    /// radiations. But it also leads to more frequent and intense solar flares and stellar winds.
    /// Said stellar winds could have a detrimental effect on the atmospheres in the system.
    /// Finally, the radiation emitted by the star would also be higher, which poses a threat to any
    /// potential life.
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
    /// The star is probably made almost entirely of hydrogen and helium. It is from a Population
    /// two levels lower than its neighbors.
    MuchLower,
    /// The star has a higher hydrogen and helium composition than expected. It is from a Population
    /// one level lower than its neighbors.
    #[default]
    Lower,
    /// The star has a lower hydrogen and helium composition than expected. It is from a Population
    /// one level higher than its neighbors and probably comes from an interstellar gas enriched by
    /// many previous generations of stars. It will be more likely to have planets.
    Higher,
    /// The star is exceptionally metal-rich for its neighborhood. It is from a Population two
    /// levels higher than its neighbors and probably formed in a place with a history of intense
    /// star formation and supernova activity. It will be far more likely to have planets, with a
    /// diverse and rich composition.
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

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum VariableStarInterval {
    Minutes,
    Hours,
    #[default]
    Days,
    Months,
    Years,
    Decades,
    Centuries,
}

impl Display for VariableStarInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableStarInterval::Minutes => write!(f, "Minutes"),
            VariableStarInterval::Hours => write!(f, "Hours"),
            VariableStarInterval::Days => write!(f, "Days"),
            VariableStarInterval::Months => write!(f, "Months"),
            VariableStarInterval::Years => write!(f, "Years"),
            VariableStarInterval::Decades => write!(f, "Decades"),
            VariableStarInterval::Centuries => write!(f, "Centuries"),
        }
    }
}
