use crate::internal::*;
use crate::prelude::*;

pub struct StringUtils {}
impl StringUtils {
    pub fn get_difference_percentage_str(number: f32, compare_to: f32) -> Rc<str> {
        let result = MathUtils::get_difference_percentage(number, compare_to);
        format!(
            "{}{}%",
            if result >= 0.0 { "+" } else { "" },
            (result * 100.0 * 100.0).round() / 100.0
        )
        .into()
    }

    pub fn number_to_lowercase_letter(number: u8) -> char {
        if number > 25 {
            panic!("Number must be between 0 and 25");
        }
        (97u8 + number) as char // 'a' as u8 is 97
    }
}
