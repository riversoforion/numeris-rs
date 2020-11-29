use std::collections::HashMap;
use std::iter::FromIterator;

use itertools;
use itertools::Itertools;
use lazy_static::lazy_static;

pub fn integer_to_roman(val: u32) -> String {
    assert!(val >= MIN_VALUE, "number must be greater than or equal to {}", MIN_VALUE);
    assert!(val <= MAX_VALUE, "number must be less than or equal to {}", MAX_VALUE);
    itertools::unfold(val, digit_extractor)
        .filter_map(|digit| VALUES_TO_SYMBOLS.get(&digit))
        .join("")
}

pub fn roman_to_integer(_numeral: &str) -> Result<u32, RomanNumeralError> {
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

pub const MIN_VALUE: u32 = 1;
pub const MAX_VALUE: u32 = 3999;

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

pub enum RomanNumeralError {}

#[cfg(test)]
mod tests {
    use super::{integer_to_roman, ATOMS, DIGITS, MAX_VALUE, MIN_VALUE, VALUES_TO_SYMBOLS};

    #[test]
    fn convert_x_to_10() {}

    #[test]
    fn convert_10_to_x() {
        let roman = integer_to_roman(10);
        assert_eq!(roman, String::from("X"));
    }

    #[test]
    fn convert_mcxlii_to_1142() {}

    #[test]
    fn convert_1142_to_mcxlii() {
        let roman = integer_to_roman(1142);
        assert_eq!(roman, String::from("MCXLII"));
    }

    #[test]
    #[should_panic]
    fn reject_values_less_than_min() {
        integer_to_roman(MIN_VALUE - 1);
    }

    #[test]
    #[should_panic]
    fn reject_values_greater_than_max() {
        integer_to_roman(MAX_VALUE + 1);
    }

    #[test]
    fn check_constants() {
        // Spot check
        assert_eq!(1000, ATOMS[0].value);
        assert_eq!(40, ATOMS[7].value);
        assert_eq!(1, ATOMS[12].value);
        assert_eq!(&"CM", VALUES_TO_SYMBOLS.get(&900).unwrap());
        assert_eq!(&"CD", VALUES_TO_SYMBOLS.get(&400).unwrap());
        assert_eq!(&"C", VALUES_TO_SYMBOLS.get(&100).unwrap());
        assert_eq!(&"IX", VALUES_TO_SYMBOLS.get(&9).unwrap());
        assert_eq!(500, DIGITS[2]);
        assert_eq!(40, DIGITS[7]);
        assert_eq!(5, DIGITS[10]);
    }
}
