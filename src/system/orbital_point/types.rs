use crate::prelude::*;

#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
pub enum AstronomicalObject {
    #[default]
    Void,
    Star(Star),
    TelluricPlanet,
    GasGiant,
    Ring,
    Station,
    Ship
}
