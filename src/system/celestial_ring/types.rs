use crate::prelude::*;

/// A list of settings used to configure the [CelestialRing] generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct CelestialRingSettings {}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum CelestialRingDetails {
    Telluric(TelluricRingDetails),
    Gaseous(GaseousRingDetails),
    Icy(IcyRingDetails),
}
