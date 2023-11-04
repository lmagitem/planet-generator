use crate::internal::*;
use crate::prelude::*;
use std::fmt;

/// A list of settings used to configure the [CelestialDisk] generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct CelestialDiskSettings {}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum CelestialDiskType {
    /// A broad, flat structure of gas and dust surrounding a young star.
    ProtoplanetaryDisk,
    /// A ring, like Saturn's ones.
    Ring(CelestialRingDetails),
    /// A belt, like the asteroid belt.
    Belt(CelestialBeltDetails),
    /// Like the Oort cloud, a shell that goes around what it orbits without being constrained to a plane.
    Shell,
}

impl Display for CelestialDiskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CelestialDiskType::ProtoplanetaryDisk => "Protoplanetary Disk".into(),
                CelestialDiskType::Ring(ring) =>
                    format!("{} {} Ring", ring.level, ring.composition),
                CelestialDiskType::Belt(belt) => format!("{} Belt", belt.composition),
                CelestialDiskType::Shell => "Shell".into(),
            }
        )
    }
}
