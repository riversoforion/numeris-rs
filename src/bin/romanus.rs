use std::io;
use std::io::Write;

use clap::{
    clap_app, crate_authors, crate_description, crate_name, crate_version, value_t, ArgMatches,
};

use romanus::{integer_to_roman, roman_to_integer, RomanNumeralError};

fn main() {
    let args = app_args();
    let debug = args.is_present("debug");
    let bare = args.is_present("bare");
    if args.is_present("integer") {
        let i = value_t!(args.value_of("integer"), u32).unwrap_or_else(|e| e.exit());
        if debug {
            println!("integer = {}", i);
        }
        print_roman_numeral(i, bare, &mut io::stdout(), &mut io::stderr());
    } else {
        let rn = args.value_of("roman").unwrap();
        if debug {
            println!("roman = {}", rn);
        }
        print_integer(rn, bare, &mut io::stdout(), &mut io::stderr());
    }
}

fn print_roman_numeral(val: u32, bare: bool, mut out: impl Write, mut err: impl Write) {
    match integer_to_roman(val) {
        Ok(rn) => writeln!(out, "{}{}", result_prefix(bare), rn),
        Err(e) => {
            let msg = match e {
                RomanNumeralError::ValueTooLarge(n) => format!("{} is too large", n),
                RomanNumeralError::ValueTooSmall(n) => format!("{} is too small", n),
                _ => String::from("Well, this is awkward"),
            };
            writeln!(err, "{}{}", error_prefix(bare), msg)
        }
    }
    .unwrap();
}

fn print_integer(val: &str, bare: bool, mut out: impl Write, mut err: impl Write) {
    match roman_to_integer(val) {
        Ok(i) => writeln!(out, "{}{}", result_prefix(bare), i),
        Err(e) => {
            let msg = match e {
                RomanNumeralError::Unparsable(v) => format!("{} is not a valid Roman numeral", v),
                RomanNumeralError::EmptyString => format!("No Roman numeral provided"),
                _ => String::from("Well, this is awkward"),
            };
            writeln!(err, "{}{}", error_prefix(bare), msg)
        }
    }
    .unwrap();
}

fn app_args() -> ArgMatches<'static> {
    clap_app!(romanus =>
        (@group conversion +required =>
            (@arg integer: -i --integer [NUMBER] "Convert the given integer value to a roman numeral")
            (@arg roman: -r --roman [NUMERAL] "Convert the given roman numeral to an integer value")
        )
        (@arg debug: -d --debug "Debugging output")
        (@arg bare: -b --bare "Only output the result")
    )
        .name(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches()
}

fn result_prefix(bare: bool) -> String {
    if bare {
        String::from("")
    } else {
        String::from("RESULT: ")
    }
}

fn error_prefix(bare: bool) -> String {
    if bare {
        String::from("")
    } else {
        String::from("ERROR: ")
    }
}

#[cfg(test)]
mod tests {
    use crate::{error_prefix, print_integer, print_roman_numeral, result_prefix};

    #[test]
    fn result_prefix_is_correct() {
        assert_eq!(result_prefix(false), String::from("RESULT: "));
        assert_eq!(result_prefix(true), String::from(""));
    }

    #[test]
    fn error_prefix_is_correct() {
        assert_eq!(error_prefix(false), String::from("ERROR: "));
        assert_eq!(error_prefix(true), String::from(""));
    }

    #[test]
    fn print_roman_numeral_with_full_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(1, false, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        assert_eq!(out, b"RESULT: I\n");
    }

    #[test]
    fn print_roman_numeral_with_bare_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(1, true, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        assert_eq!(out, b"I\n");
    }

    #[test]
    fn print_roman_numeral_with_full_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(0, false, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        assert_eq!(err, b"ERROR: 0 is too small\n");
    }

    #[test]
    fn print_roman_numeral_with_bare_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(0, true, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        assert_eq!(err, b"0 is too small\n");
    }

    #[test]
    fn print_integer_with_full_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("XI", false, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        assert_eq!(out, b"RESULT: 11\n");
    }

    #[test]
    fn print_integer_with_bare_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("XI", true, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        assert_eq!(out, b"11\n");
    }

    #[test]
    fn print_integer_with_full_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("Blah", false, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        assert_eq!(err, b"ERROR: BLAH is not a valid Roman numeral\n");
    }

    #[test]
    fn print_integer_with_bare_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("Blah", true, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        assert_eq!(err, b"BLAH is not a valid Roman numeral\n");
    }
}
