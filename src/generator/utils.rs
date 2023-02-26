pub struct GeneratorUtils {}
impl GeneratorUtils {
    pub fn get_difference_percentage_str(number: f32, compare_to: f32) -> String {
        let result = GeneratorUtils::get_difference_percentage(number, compare_to);
        format!(
            "{}{}%",
            if result >= 0.0 { "+" } else { "" },
            (result * 100.0 * 100.0).round() / 100.0
        )
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
