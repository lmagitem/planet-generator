use crate::internal::*;
use crate::prelude::*;

pub struct StringUtils {}
impl StringUtils {
    pub fn get_difference_percentage_str(number: f64, compare_to: f64) -> Rc<str> {
        let result = MathUtils::get_difference_percentage(number, compare_to);
        format!(
            "{}{}%",
            if result >= 0.0 { "+" } else { "" },
            (result * 100.0 * 100.0).round() / 100.0
        )
        .into()
    }

    pub fn number_to_lowercase_letter(number: u8) -> String {
        if number > 25 {
            return format!("{}", number);
        }
        String::from((97u8 + number) as char)
    }

    pub fn to_significant_decimals(num: f64) -> String {
        let epsilon: f64 = 1e-12;
        if num.abs() < epsilon {
            "0.0".to_string()
        } else if num >= 1.0 {
            format!("{:.2}", num)
        } else {
            // For numbers less than 1, find the first two non-zero digits after the decimal
            let s = format!("{:.12}", num); // Convert to string with sufficient precision
            let mut non_zero_count = 0;
            let mut last_non_zero_index = 0;
            let mut zero_count = 0;
            let mut dot_index = None;

            for (i, c) in s.chars().enumerate() {
                if c == '.' {
                    dot_index = Some(i);
                } else if dot_index.is_some() {
                    if c != '0' {
                        non_zero_count += 1;
                        last_non_zero_index = i;
                        zero_count = 0;
                    } else {
                        zero_count += 1;
                    }

                    if non_zero_count == 2 || (non_zero_count >= 1 && zero_count >= 3) {
                        // Cut to the last non-zero number or keep two non-zero digits
                        return format!(
                            "{:.1$}",
                            num,
                            last_non_zero_index - dot_index.unwrap() + 1
                        );
                    }
                }
            }

            s // Return original string if conditions not met
        }
    }
}
