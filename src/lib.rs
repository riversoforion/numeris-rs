use std::collections::HashMap;
use std::iter::FromIterator;

use itertools;
use itertools::Itertools;
use lazy_static::lazy_static;

pub fn integer_to_roman(val: u32) -> Result<String> {
    if val < MIN_VALUE {
        Err(RomanNumeralError::too_small(val))
    } else if val > MAX_VALUE {
        Err(RomanNumeralError::too_large(val))
    } else {
        let result = itertools::unfold(val, digit_extractor)
            .filter_map(|digit| VALUES_TO_SYMBOLS.get(&digit))
            .join("");
        Ok(result)
    }
}

pub fn roman_to_integer(_numeral: &str) -> Result<u32> {
    Ok(0)
}

fn digit_extractor(seed: &mut u32) -> Option<u32> {
    if *seed == 0 {
        return None;
    }
    let next_digit = DIGITS.iter().find(|digit| *seed >= **digit).unwrap_or(&1);
    *seed = *seed - *next_digit;
    Some(*next_digit)
}

#[derive(Debug)]
struct RomanNumeral {
    value: u32,
    symbol: &'static str,
    allow_multiples: bool,
}

lazy_static! {
    static ref ATOMS: [RomanNumeral; 13] = [
        RomanNumeral { value: 1000, symbol: "M", allow_multiples: true },
        RomanNumeral { value: 900, symbol: "CM", allow_multiples: false },
        RomanNumeral { value: 500, symbol: "D", allow_multiples: true },
        RomanNumeral { value: 400, symbol: "CD", allow_multiples: false },
        RomanNumeral { value: 100, symbol: "C", allow_multiples: true },
        RomanNumeral { value: 90, symbol: "XC", allow_multiples: false },
        RomanNumeral { value: 50, symbol: "L", allow_multiples: true },
        RomanNumeral { value: 40, symbol: "XL", allow_multiples: false },
        RomanNumeral { value: 10, symbol: "X", allow_multiples: true },
        RomanNumeral { value: 9, symbol: "IX", allow_multiples: false },
        RomanNumeral { value: 5, symbol: "V", allow_multiples: true },
        RomanNumeral { value: 4, symbol: "IV", allow_multiples: false },
        RomanNumeral { value: 1, symbol: "I", allow_multiples: true },
    ];
    static ref VALUES_TO_SYMBOLS: HashMap<u32, &'static str> =
        HashMap::from_iter(ATOMS.iter().map(|rn| (rn.value, rn.symbol)));
    static ref DIGITS: Vec<u32> = ATOMS.iter().map(|rn| rn.value).collect_vec();
}

pub const MIN_VALUE: u32 = 1;
pub const MAX_VALUE: u32 = 3999;

#[derive(Debug, Clone)]
pub struct RomanNumeralError {
    kind: RomanNumeralErrorKind,
}

impl RomanNumeralError {
    fn too_large(val: u32) -> Self {
        RomanNumeralError { kind: RomanNumeralErrorKind::ValueTooLarge(val) }
    }

    fn too_small(val: u32) -> Self {
        RomanNumeralError { kind: RomanNumeralErrorKind::ValueTooSmall(val) }
    }

    #[allow(dead_code)]
    fn unparsable(val: &String) -> Self {
        RomanNumeralError { kind: RomanNumeralErrorKind::Unparsable(val.clone()) }
    }

    #[allow(dead_code)]
    fn empty_string() -> Self {
        RomanNumeralError { kind: RomanNumeralErrorKind::EmptyString }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RomanNumeralErrorKind {
    ValueTooLarge(u32),
    ValueTooSmall(u32),
    Unparsable(String),
    EmptyString,
}

type Result<T> = std::result::Result<T, RomanNumeralError>;

#[cfg(test)]
mod tests {
    use super::{ATOMS, DIGITS, VALUES_TO_SYMBOLS};

    mod itor {
        use crate::{integer_to_roman, MAX_VALUE, MIN_VALUE};

        #[test]
        #[should_panic]
        fn reject_values_less_than_min() {
            integer_to_roman(MIN_VALUE - 1).unwrap();
        }

        #[test]
        #[should_panic]
        fn reject_values_greater_than_max() {
            integer_to_roman(MAX_VALUE + 1).unwrap();
        }

        mod simple {
            use crate::integer_to_roman;

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
            use crate::integer_to_roman;

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

    mod rtoi {
        mod simple {
            #[test]
            fn convert_x_to_10() {}
        }

        mod compound {
            #[test]
            fn convert_mcxlii_to_1142() {}
        }
    }

    #[test]
    fn check_atoms() {
        assert_eq!(1000, ATOMS[0].value);
        assert_eq!(40, ATOMS[7].value);
        assert_eq!(1, ATOMS[12].value);
    }

    #[test]
    fn check_values_to_symbols() {
        assert_eq!(&"CM", VALUES_TO_SYMBOLS.get(&900).unwrap());
        assert_eq!(&"CD", VALUES_TO_SYMBOLS.get(&400).unwrap());
        assert_eq!(&"C", VALUES_TO_SYMBOLS.get(&100).unwrap());
        assert_eq!(&"IX", VALUES_TO_SYMBOLS.get(&9).unwrap());
    }

    #[test]
    fn check_digits() {
        assert_eq!(500, DIGITS[2]);
        assert_eq!(40, DIGITS[7]);
        assert_eq!(5, DIGITS[10]);
    }
}
