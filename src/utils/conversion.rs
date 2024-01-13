pub struct ConversionUtils {}
impl ConversionUtils {
    /// Converts a value from Solar radii to Astronomical units.
    pub fn solar_radii_to_astronomical_units(radius: f64) -> f64 {
        radius * 4.6524726374 / 1000.0
    }

    /// Converts a value from Astronomical units to Solar radii.
    pub fn astronomical_units_to_solar_radii(au: f64) -> f64 {
        au * 214.9394693836
    }

    /// Converts a value from Earth radii to Astronomical units.
    pub fn earth_radii_to_astronomical_units(radius: f64) -> f64 {
        radius * 6371.0 / 149597870.7
    }

    /// Converts a value from Astronomical units to Earth radii.
    pub fn astronomical_units_to_earth_radii(au: f64) -> f64 {
        au * 23454.706481336
    }

    /// Converts a value from Astronomical units to Earth diameters.
    pub fn astronomical_units_to_earth_diameters(au: f64) -> f64 {
        au * 11727.3532407
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

#[cfg(test)]
mod tests {
    use super::ConversionUtils;

    /// Precision for floating-point comparisons
    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_solar_radii_to_astronomical_units() {
        let solar_radii = 1.0;
        let au = ConversionUtils::solar_radii_to_astronomical_units(solar_radii);
        assert!((au - 0.0046524726374).abs() < EPSILON);
    }

    #[test]
    fn test_earth_radii_to_astronomical_units() {
        let earth_radii = 1.0;
        let au = ConversionUtils::earth_radii_to_astronomical_units(earth_radii);
        assert!((au - (6371.0 / 149597870.7)).abs() < EPSILON);
    }

    #[test]
    fn test_astronomical_units_to_solar_radii() {
        let au = 1.0; // 1 AU
        let solar_radii = ConversionUtils::astronomical_units_to_solar_radii(au);
        let expected_solar_radii = 214.939;
        assert!(
            (solar_radii - expected_solar_radii).abs() < 0.001,
            "solar_radii = {}, expected_solar_radii = {}",
            solar_radii,
            expected_solar_radii
        );
    }

    #[test]
    fn test_astronomical_units_to_earth_radii() {
        let au = 1.0; // 1 AU
        let earth_radii = ConversionUtils::astronomical_units_to_earth_radii(au);
        let expected_earth_radii = 23454.706;
        assert!(
            (earth_radii - expected_earth_radii).abs() < 0.001,
            "earth_radii = {}, expected_earth_radii = {}",
            earth_radii,
            expected_earth_radii
        );
    }

    #[test]
    fn test_kelvin_to_celsius() {
        let kelvin = 273;
        let celsius = ConversionUtils::kelvin_to_celsius(kelvin);
        assert_eq!(celsius, 0);
    }

    #[test]
    fn test_earth_mass_to_solar_mass() {
        let earth_mass = 333000.0;
        let solar_mass = ConversionUtils::earth_mass_to_solar_mass(earth_mass);
        assert!((solar_mass - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_solar_mass_to_earth_mass() {
        let solar_mass = 1.0;
        let earth_mass = ConversionUtils::solar_mass_to_earth_mass(solar_mass);
        assert!((earth_mass - 333000.0).abs() < EPSILON);
    }
}
