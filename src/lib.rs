pub use itor::integer_to_roman;
pub use rtoi::roman_to_integer;

mod itor;
mod rtoi;

pub const MIN_VALUE: u32 = 1;
pub const MAX_VALUE: u32 = 3999;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RomanNumeralErrorKind {
    ValueTooLarge(u32),
    ValueTooSmall(u32),
    Unparsable(String),
    EmptyString,
}

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
    fn unparsable(val: &str) -> Self {
        RomanNumeralError { kind: RomanNumeralErrorKind::Unparsable(String::from(val)) }
    }

    #[allow(dead_code)]
    fn empty_string() -> Self {
        RomanNumeralError { kind: RomanNumeralErrorKind::EmptyString }
    }
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
