use super::Result;

pub fn roman_to_integer(_numeral: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    mod simple {
        #[test]
        fn convert_x_to_10() {}
    }

    mod compound {
        #[test]
        fn convert_mcxlii_to_1142() {}
    }
}
