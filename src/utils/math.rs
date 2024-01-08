pub struct MathUtils {}
impl MathUtils {
    /// Rounds a `f32` number to a specified number of decimal places.
    ///
    /// # Arguments
    ///
    /// * `num` - The `f32` number to round.
    /// * `zeroes` - The number of decimal places to round to.
    pub fn round_f32_to_precision(num: f32, zeroes: u32) -> f32 {
        let scale = 10f32.powi(zeroes as i32);
        (num * scale).round() / scale
    }

    /// Checks if an `f32` number is almost equal to zero.
    ///
    /// # Arguments
    ///
    /// * `num` - The `f32` number to check.
    pub fn does_f32_equal_zero(num: f32) -> bool {
        let epsilon = 1e-6;
        num.abs() < epsilon
    }

    /// Rounds a `f64` number to a specified number of decimal places.
    ///
    /// # Arguments
    ///
    /// * `num` - The `f64` number to round.
    /// * `zeroes` - The number of decimal places to round to.
    pub fn round_f64_to_precision(num: f64, zeroes: u32) -> f64 {
        let scale = 10f64.powi(zeroes as i32);
        (num * scale).round() / scale
    }

    /// Checks if an `f64` number is almost equal to zero.
    ///
    /// # Arguments
    ///
    /// * `num` - The `f64` number to check.
    pub fn does_f64_equal_zero(num: f64) -> bool {
        let epsilon = 1e-15;
        num.abs() < epsilon
    }

    pub fn get_difference_percentage(number: f64, compare_to: f64) -> f64 {
        let result = if compare_to <= 0.0 && number >= 0.0 {
            (number - compare_to) / compare_to.abs()
        } else if compare_to <= 0.0 && compare_to <= number {
            (number.abs() - compare_to.abs()) / compare_to
        } else if compare_to >= 0.0 && number <= 0.0 {
            (number - compare_to) / compare_to
        } else if compare_to >= 0.0 && number >= 0.0 {
            (number - compare_to) / compare_to
        } else if compare_to <= 0.0 && compare_to >= number {
            (number.abs() - compare_to.abs()) / compare_to
        } else if compare_to <= number {
            (number - compare_to) / number
        } else {
            -((compare_to - number) / compare_to)
        };
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_f32_equal_zero() {
        // Test with numbers that are almost equal to zero
        assert!(MathUtils::does_f32_equal_zero(0.0));
        assert!(MathUtils::does_f32_equal_zero(1e-7));
        assert!(MathUtils::does_f32_equal_zero(-1e-7));

        // Test with non-zero numbers
        assert!(!MathUtils::does_f32_equal_zero(1.0));
        assert!(!MathUtils::does_f32_equal_zero(-1.0));
        assert!(!MathUtils::does_f32_equal_zero(0.1));
        assert!(!MathUtils::does_f32_equal_zero(-0.1));
    }

    #[test]
    fn test_does_f64_equal_zero() {
        // Test with numbers that are almost equal to zero
        assert!(MathUtils::does_f64_equal_zero(0.0));
        assert!(MathUtils::does_f64_equal_zero(1e-16));
        assert!(MathUtils::does_f64_equal_zero(-1e-16));

        // Test with non-zero numbers
        assert!(!MathUtils::does_f64_equal_zero(1.0));
        assert!(!MathUtils::does_f64_equal_zero(-1.0));
        assert!(!MathUtils::does_f64_equal_zero(0.1));
        assert!(!MathUtils::does_f64_equal_zero(-0.1));
    }
}
