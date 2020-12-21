use itertools::fold;
use lazy_static::lazy_static;
use regex::Regex;

use super::{Result, RomanNumeral, RomanNumeralError, ATOMS};

/// Converts a string representing a Roman numeral into an integer.
///
/// The input must be a valid Roman numeral. Both upper- & lower-case values are supported, and
/// any leading and trailing whitespace will be trimmed before parsing.
///
/// # Examples
///
/// ### Normal usage
/// ```
/// use numeris::roman_to_integer;
///
/// let i = roman_to_integer("MCXLII").unwrap();
/// assert_eq!(i, 1142);
/// let i = roman_to_integer(" cv\n").unwrap();
/// assert_eq!(i, 105);
/// ```
///
/// ### Invalid characters
/// ```
/// use numeris::{roman_to_integer, RomanNumeralError};
///
/// match roman_to_integer("BAD") {
///     Err(RomanNumeralError::Unparsable(_)) => println!("BAD input"),
///     Err(_) => panic!("wrong kind of BAD"),
///     Ok(_) => panic!("BAD is not good"),
/// };
/// ```
///
/// ### Empty input
/// ```
/// use numeris::{roman_to_integer, RomanNumeralError};
///
/// match roman_to_integer("    ") {
///     Err(RomanNumeralError::EmptyString) => println!("no input"),
///     Err(_) => panic!("hmm"),
///     Ok(_) => panic!("unacceptable"),
/// };
/// ```
///
/// # Errors
///
/// | `RomanNumeralError` | Reason |
/// | ----------------------- | ------ |
/// | [`Unparsable`][a] | `numeral` cannot be parsed as a Roman numeral |
/// | [`EmptyString`][b] |  `numeral` is an empty string or contains only whitespace |
///
/// [a]: crate::RomanNumeralError::Unparsable
/// [b]: crate::RomanNumeralError::EmptyString
pub fn roman_to_integer(numeral: &str) -> Result<u32> {
    let numeral = normalize_numeral(&numeral);
    let numeral = check_numeral_format(&numeral)?;
    let digits: Vec<u32> = decompose_numeral(numeral.as_str())?;
    let result = fold(digits.as_slice(), 0, |seed, &val| seed + val);
    Ok(result)
}

fn normalize_numeral(numeral: &str) -> String {
    numeral.trim().to_ascii_uppercase()
}

fn check_numeral_format(numeral: &String) -> Result<&String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[IVXLCDM]+$").unwrap();
    }
    if numeral.len() == 0 {
        Err(RomanNumeralError::EmptyString)
    } else if !RE.is_match(numeral) {
        Err(RomanNumeralError::Unparsable(numeral.clone()))
    } else {
        Ok(numeral)
    }
}

fn decompose_numeral(numeral: &str) -> Result<Vec<u32>> {
    let mut parse_state = ParseState::new(numeral);
    let mut result: Vec<u32> = Vec::new();
    while !parse_state.is_complete() {
        let current = parse_state.current_numeral();
        if parse_state.remaining_to_parse.starts_with(current.symbol) {
            result.push(current.value);
            parse_state.remove_current();
            if !current.allow_multiples {
                parse_state.advance_numeral();
            }
        } else {
            parse_state.advance_numeral();
        }
    }
    if parse_state.remaining_to_parse.len() > 0 {
        Err(RomanNumeralError::Unparsable(String::from(numeral)))
    } else {
        Ok(result)
    }
}

#[derive(Debug)]
struct ParseState<'a> {
    remaining_numerals: &'static [RomanNumeral],
    numeral_pos: usize,
    remaining_to_parse: &'a str,
}

impl<'a> ParseState<'a> {
    fn new(to_parse: &'a str) -> Self {
        ParseState { remaining_numerals: &ATOMS[..], numeral_pos: 0, remaining_to_parse: to_parse }
    }

    fn current_numeral(&self) -> RomanNumeral {
        self.remaining_numerals[0].clone()
    }

    fn advance_numeral(&mut self) {
        self.numeral_pos += 1;
        self.remaining_numerals = &ATOMS[self.numeral_pos..];
    }

    fn is_complete(&self) -> bool {
        self.remaining_numerals.len() == 0
    }

    fn remove_current(&mut self) {
        let skip = self.current_numeral().symbol.len();
        self.remaining_to_parse = &self.remaining_to_parse[skip..];
    }
}

#[cfg(test)]
mod tests {
    use crate::{roman_to_integer, RomanNumeralError};

    #[test]
    fn reject_invalid_format() {
        for val in ["ABCDEF", "MMDL1", "934;-)", "CMM", "ID", "MMCCD", "XLXL"].iter() {
            match roman_to_integer(*val) {
                Err(RomanNumeralError::Unparsable(_)) => (),
                Err(_) => panic!("wrong kind of error"),
                Ok(_) => panic!("unexpected ok result"),
            }
        }
    }

    #[test]
    fn reject_empty_string() {
        for val in ["", "   ", "\t", "\n"].iter() {
            match roman_to_integer(*val) {
                Err(RomanNumeralError::EmptyString) => (),
                Err(_) => panic!("wrong kind of error"),
                Ok(_) => panic!("unexpected ok result"),
            }
        }
    }

    #[test]
    fn allow_lowercase_and_whitespace() {
        for val in ["  MCXLII", "CII  ", "  X  ", "V\n", "mcmxl", " cclxi ", "mmCCxXiI"].iter() {
            match roman_to_integer(*val) {
                Err(_) => panic!("error parsing value"),
                Ok(_) => (),
            }
        }
    }

    mod simple {
        use crate::roman_to_integer;

        #[test]
        fn convert_i_to_1() {
            assert_eq!(roman_to_integer("I").unwrap(), 1);
        }

        #[test]
        fn convert_v_to_5() {
            assert_eq!(roman_to_integer("V").unwrap(), 5);
        }

        #[test]
        fn convert_x_to_10() {
            assert_eq!(roman_to_integer("X").unwrap(), 10);
        }

        #[test]
        fn convert_l_to_50() {
            assert_eq!(roman_to_integer("L").unwrap(), 50);
        }

        #[test]
        fn convert_c_to_100() {
            assert_eq!(roman_to_integer("C").unwrap(), 100);
        }

        #[test]
        fn convert_d_to_500() {
            assert_eq!(roman_to_integer("D").unwrap(), 500);
        }

        #[test]
        fn convert_m_to_1000() {
            assert_eq!(roman_to_integer("M").unwrap(), 1000);
        }
    }

    mod compound {
        use crate::roman_to_integer;

        #[test]
        fn convert_iv_to_4() {
            assert_eq!(roman_to_integer("IV").unwrap(), 4);
        }

        #[test]
        fn convert_ix_to_9() {
            assert_eq!(roman_to_integer("IX").unwrap(), 9);
        }

        #[test]
        fn convert_xlviii_to_48() {
            assert_eq!(roman_to_integer("XLVIII").unwrap(), 48);
        }

        #[test]
        fn convert_dcci_to_701() {
            assert_eq!(roman_to_integer("DCCI").unwrap(), 701);
        }

        #[test]
        fn convert_mcxlii_to_1142() {
            assert_eq!(roman_to_integer("MCXLII").unwrap(), 1142);
        }

        #[test]
        fn convert_mmcdlxviii_to_2468() {
            assert_eq!(roman_to_integer("MMCDLXVIII").unwrap(), 2468);
        }
    }
}
