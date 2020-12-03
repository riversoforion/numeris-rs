//! # Romanus
//!
//! A simple crate for working with Roman numerals.
//!
//! The entry points are two functions, [`integer_to_roman`] and [`roman_to_integer`], which
//! convert between integral values and string-representations of Roman numerals. See the
//! documentation on each function for details.

pub use itor::integer_to_roman;
pub use rtoi::roman_to_integer;

mod itor;
mod rtoi;

/// The minimum value supported for Roman numerals
pub const MIN_VALUE: u32 = 1;
/// The maximum value supported for Roman numerals
pub const MAX_VALUE: u32 = 3999;

/// The different kinds of errors that can be encountered when working with Roman numerals.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RomanNumeralError {
    /// Indicates that the numeric value is too large to be turned into a Roman numeral.
    ValueTooLarge(u32),
    /// Indicates that the numeric value is too small to be turned into a Roman numeral.
    ValueTooSmall(u32),
    /// Indicates a Roman numeral that could not be parsed into an integer.
    Unparsable(String),
    /// Indicates an empty Roman numeral value.
    EmptyString,
}

pub type Result<T> = std::result::Result<T, RomanNumeralError>;

#[derive(Debug, Clone)]
struct RomanNumeral {
    value: u32,
    symbol: &'static str,
    allow_multiples: bool,
}

const ATOMS: [RomanNumeral; 13] = [
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

#[cfg(test)]
mod tests {
    use super::ATOMS;

    #[test]
    fn check_atoms() {
        assert_eq!(1000, ATOMS[0].value);
        assert_eq!(40, ATOMS[7].value);
        assert_eq!(1, ATOMS[12].value);
    }
}
