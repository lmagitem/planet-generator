use crate::internal::*;
use crate::prelude::*;

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
    Belt,
    /// Like the Oort cloud, a shell that goes around what it orbits without being constrained to a plane.
    Shell,
}
