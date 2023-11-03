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
}
