pub fn integer_to_roman(_val: u32) -> String {
    String::from("")
}

pub fn roman_to_integer(_numeral: &str) -> Result<u32, ParsingError> {
    Ok(0)
}

pub enum ParsingError {}

#[cfg(test)]
mod tests {
    #[test]
    fn convert_x_to_10() {}

    #[test]
    fn convert_10_to_x() {}

    #[test]
    fn convert_mcxlii_to_1142() {}

    #[test]
    fn convert_1142_to_mcxlii() {}
}
