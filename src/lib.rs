use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RomanNumeral {
    value: u32
}

impl RomanNumeral {
    pub fn value(&self) -> u32 {
        self.value
    }
}

impl From<u32> for RomanNumeral {
    fn from(value: u32) -> Self {
        RomanNumeral { value }
    }
}

impl TryFrom<&str> for RomanNumeral {
    type Error = ParsingError;

    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        Ok(RomanNumeral { value: 0 })
    }
}

impl Into<u32> for RomanNumeral {
    fn into(self) -> u32 {
        self.value
    }
}

impl Into<u64> for RomanNumeral {
    fn into(self) -> u64 {
        self.value as u64
    }
}

impl Into<i64> for RomanNumeral {
    fn into(self) -> i64 {
        self.value as i64
    }
}

impl Into<String> for RomanNumeral {
    fn into(self) -> String {
        String::from("")
    }
}

// Other implementations
// Add/AddAssign<RomanNumeral|u32|u16|u8>
// Sub/SubAssign<RomanNumeral|u32|u16|u8>
// RangeBounds
// Display

pub enum ParsingError {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
