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

    pub fn get_difference_percentage(number: f32, compare_to: f32) -> f32 {
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
