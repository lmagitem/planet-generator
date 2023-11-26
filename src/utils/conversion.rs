pub struct ConversionUtils {}
impl ConversionUtils {
    /// Converts a value from Solar radii to Astronomical units.
    pub fn solar_radii_to_astronomical_units(radius: f64) -> f64 {
        radius * 4.6524726374 / 1000.0
    }

    /// Converts a temperature from Kelvin to Celsius.
    pub fn kelvin_to_celsius(temperature: u32) -> i32 {
        (temperature as f32 - 273.15) as i32
    }

    /// Converts a value expressed in Earth Masses into Solar Masses.
    pub fn earth_mass_to_solar_mass(mass: f64) -> f64 {
        mass / 333000.0
    }

    /// Converts a value expressed in Solar Masses into Earth Masses..
    pub fn solar_mass_to_earth_mass(mass: f64) -> f64 {
        mass * 333000.0
    }
}
