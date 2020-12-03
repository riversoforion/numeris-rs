use std::collections::HashMap;
use std::iter::FromIterator;

use itertools;
use itertools::Itertools;
use lazy_static::lazy_static;

use super::{Result, RomanNumeralError, ATOMS, MAX_VALUE, MIN_VALUE};

/// Converts an integer into a string representing a Roman Numeral.
///
/// The input must be greater than or equal to [`MIN_VALUE`] and less than or equal to
/// [`MAX_VALUE`]. The Roman numeral will be returned in upper-case characters.
///
/// # Examples
///
/// ### Normal usage
/// ```
/// use romanus::integer_to_roman;
///
/// let rn = integer_to_roman(1142).unwrap();
/// assert_eq!(rn, "MCXLII");
/// ```
///
/// ### Value too small
/// ```
/// use romanus::{integer_to_roman, RomanNumeralError};
///
/// match integer_to_roman(0) {
///     Err(RomanNumeralError::ValueTooSmall(_)) => (),
///     Err(_) => panic!("not enough Roman"),
///     Ok(_) => panic!("0's not good"),
/// }
/// ```
///
/// ### Value too large
/// ```
/// use romanus::{integer_to_roman, RomanNumeralError};
///
/// match integer_to_roman(6000) {
///     Err(RomanNumeralError::ValueTooLarge(_)) => (),
///     Err(_) => panic!("too much Roman"),
///     Ok(_) => panic!("0's not good"),
/// }
/// ```
///
/// # Errors
///
/// | `RomanNumeralErrorKind` | Reason |
/// | ----------------------- | ------ |
/// | [`ValueTooSmall`][a] | `val` is too small to be converted to a Roman numeral |
/// | [`ValueTooLarge`][b] |  `val` is too large to be converted to a Roman numeral |
///
/// [a]: crate::RomanNumeralError::ValueTooSmall
/// [b]: crate::RomanNumeralError::ValueTooLarge
pub fn integer_to_roman(val: u32) -> Result<String> {
    if val < MIN_VALUE {
        Err(RomanNumeralError::ValueTooSmall(val))
    } else if val > MAX_VALUE {
        Err(RomanNumeralError::ValueTooLarge(val))
    } else {
        let result = itertools::unfold(val, digit_extractor)
            .filter_map(|digit| VALUES_TO_SYMBOLS.get(&digit))
            .join("");
        Ok(result)
    }
}

fn digit_extractor(seed: &mut u32) -> Option<u32> {
    if *seed == 0 {
        return None;
    }
    let next_digit = DIGITS.iter().find(|digit| *seed >= **digit).unwrap_or(&1);
    *seed = *seed - *next_digit;
    Some(*next_digit)
}

lazy_static! {
    static ref VALUES_TO_SYMBOLS: HashMap<u32, &'static str> =
        HashMap::from_iter(ATOMS.iter().map(|rn| (rn.value, rn.symbol)));
    static ref DIGITS: Vec<u32> = ATOMS.iter().map(|rn| rn.value).collect_vec();
}

#[cfg(test)]
mod tests {
    use crate::{integer_to_roman, RomanNumeralError, MAX_VALUE, MIN_VALUE};

    use super::{DIGITS, VALUES_TO_SYMBOLS};

    #[test]
    fn check_digits() {
        assert_eq!(500, DIGITS[2]);
        assert_eq!(40, DIGITS[7]);
        assert_eq!(5, DIGITS[10]);
    }

    #[test]
    fn check_values_to_symbols() {
        assert_eq!(&"CM", VALUES_TO_SYMBOLS.get(&900).unwrap());
        assert_eq!(&"CD", VALUES_TO_SYMBOLS.get(&400).unwrap());
        assert_eq!(&"C", VALUES_TO_SYMBOLS.get(&100).unwrap());
        assert_eq!(&"IX", VALUES_TO_SYMBOLS.get(&9).unwrap());
    }

    #[test]
    fn reject_values_less_than_min() {
        match integer_to_roman(MIN_VALUE - 1) {
            Err(RomanNumeralError::ValueTooSmall(_)) => (),
            Err(_) => panic!("wrong kind of error"),
            Ok(_) => panic!("unexpected ok result"),
        };
    }

    #[test]
    fn reject_values_greater_than_max() {
        match integer_to_roman(MAX_VALUE + 1) {
            Err(RomanNumeralError::ValueTooLarge(_)) => (),
            Err(_) => panic!("wrong kind of error"),
            Ok(_) => panic!("unexpected ok result"),
        };
    }

    mod simple {
        use super::integer_to_roman;

        #[test]
        fn convert_1_to_i() {
            assert_eq!(integer_to_roman(1).unwrap(), String::from("I"));
        }

        #[test]
        fn convert_5_to_v() {
            assert_eq!(integer_to_roman(5).unwrap(), String::from("V"));
        }

        #[test]
        fn convert_10_to_x() {
            assert_eq!(integer_to_roman(10).unwrap(), String::from("X"));
        }

        #[test]
        fn convert_50_to_l() {
            assert_eq!(integer_to_roman(50).unwrap(), String::from("L"));
        }

        #[test]
        fn convert_100_to_c() {
            assert_eq!(integer_to_roman(100).unwrap(), String::from("C"));
        }

        #[test]
        fn convert_500_to_d() {
            assert_eq!(integer_to_roman(500).unwrap(), String::from("D"));
        }

        #[test]
        fn convert_1000_to_m() {
            assert_eq!(integer_to_roman(1000).unwrap(), String::from("M"));
        }
    }

    mod compound {
        use super::integer_to_roman;

        #[test]
        fn convert_4_to_iv() {
            assert_eq!(integer_to_roman(4).unwrap(), String::from("IV"));
        }

        #[test]
        fn convert_9_to_ix() {
            assert_eq!(integer_to_roman(9).unwrap(), String::from("IX"));
        }

        #[test]
        fn convert_48_to_xlviii() {
            assert_eq!(integer_to_roman(48).unwrap(), String::from("XLVIII"));
        }

        #[test]
        fn convert_701_to_dcci() {
            assert_eq!(integer_to_roman(701).unwrap(), String::from("DCCI"));
        }

        #[test]
        fn convert_1142_to_mcxlii() {
            assert_eq!(integer_to_roman(1142).unwrap(), String::from("MCXLII"));
        }

        #[test]
        fn convert_2468_to_mmcdlxviii() {
            assert_eq!(integer_to_roman(2468).unwrap(), String::from("MMCDLXVIII"));
        }
    }
}
