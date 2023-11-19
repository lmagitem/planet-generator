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
}
