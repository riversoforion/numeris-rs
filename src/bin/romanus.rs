use std::io;
use std::io::Write;

use ansi_term::{Colour::Cyan, Colour::Green, Colour::Red, Style};
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
            print_debug(String::from("integer"), i.to_string());
        }
        print_roman_numeral(i, bare, &mut io::stdout(), &mut io::stderr());
    } else {
        let rn = args.value_of("roman").unwrap();
        if debug {
            print_debug(String::from("roman"), String::from(rn))
        }
        print_integer(rn, bare, &mut io::stdout(), &mut io::stderr());
    }
}

fn print_roman_numeral(val: u32, bare: bool, mut out: impl Write, mut err: impl Write) {
    match integer_to_roman(val) {
        Ok(rn) => writeln!(out, "{}{}", result_prefix(bare), Green.paint(rn)),
        Err(e) => {
            let msg = match e {
                RomanNumeralError::ValueTooLarge(n) => format!("{} is too large", n),
                RomanNumeralError::ValueTooSmall(n) => format!("{} is too small", n),
                _ => String::from("Well, this is awkward"),
            };
            writeln!(err, "{}{}", error_prefix(bare), Red.paint(msg))
        }
    }
    .unwrap();
}

fn print_integer(val: &str, bare: bool, mut out: impl Write, mut err: impl Write) {
    match roman_to_integer(val) {
        Ok(i) => writeln!(out, "{}{}", result_prefix(bare), Green.paint(i.to_string())),
        Err(e) => {
            let msg = match e {
                RomanNumeralError::Unparsable(v) => format!("{} is not a valid Roman numeral", v),
                RomanNumeralError::EmptyString => format!("No Roman numeral provided"),
                _ => String::from("Well, this is awkward"),
            };
            writeln!(err, "{}{}", error_prefix(bare), Red.paint(msg))
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
        format!("{} ", Green.bold().reverse().paint("RESULT:"))
    }
}

fn error_prefix(bare: bool) -> String {
    if bare {
        String::from("")
    } else {
        format!("{} ", Red.bold().reverse().paint("ERROR:"))
    }
}

fn print_debug(prefix: String, val: String) {
    let dim = Style::new().dimmed();
    let cyan = Cyan.dimmed();
    let prefix = format!("{} = ", prefix);
    println!("{}{}", dim.paint(prefix), cyan.paint(val.to_string()));
}

#[cfg(test)]
mod tests {
    use ansi_term::{Colour::Green, Colour::Red};

    use crate::{error_prefix, print_integer, print_roman_numeral, result_prefix};

    #[test]
    fn result_prefix_is_correct() {
        let expected = format!("{} ", Green.bold().reverse().paint("RESULT:"));
        assert_eq!(result_prefix(false), expected);
        let expected = String::from("");
        assert_eq!(result_prefix(true), expected);
    }

    #[test]
    fn error_prefix_is_correct() {
        let expected = format!("{} ", Red.bold().reverse().paint("ERROR:"));
        assert_eq!(error_prefix(false), expected);
        let expected = String::from("");
        assert_eq!(error_prefix(true), expected);
    }

    #[test]
    fn print_roman_numeral_with_full_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(1, false, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        let expected =
            format!("{} {}\n", Green.bold().reverse().paint("RESULT:"), Green.paint("I"));
        assert_eq!(out, expected.as_bytes());
    }

    #[test]
    fn print_roman_numeral_with_bare_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(1, true, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        let expected = format!("{}\n", Green.paint("I"));
        assert_eq!(out, expected.as_bytes());
    }

    #[test]
    fn print_roman_numeral_with_full_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(0, false, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        let expected =
            format!("{} {}\n", Red.bold().reverse().paint("ERROR:"), Red.paint("0 is too small"));
        assert_eq!(err, expected.as_bytes());
    }

    #[test]
    fn print_roman_numeral_with_bare_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_roman_numeral(0, true, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        let expected = format!("{}\n", Red.paint("0 is too small"));
        assert_eq!(err, expected.as_bytes());
    }

    #[test]
    fn print_integer_with_full_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("XI", false, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        let expected =
            format!("{} {}\n", Green.bold().reverse().paint("RESULT:"), Green.paint("11"));
        assert_eq!(out, expected.as_bytes());
    }

    #[test]
    fn print_integer_with_bare_output() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("XI", true, &mut out, &mut err);
        assert_eq!(err.len(), 0);
        let expected = format!("{}\n", Green.paint("11"));
        assert_eq!(out, expected.as_bytes());
    }

    #[test]
    fn print_integer_with_full_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("Blah", false, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        let expected = format!(
            "{} {}\n",
            Red.bold().reverse().paint("ERROR:"),
            Red.paint("BLAH is not a valid Roman numeral")
        );
        assert_eq!(err, expected.as_bytes());
    }

    #[test]
    fn print_integer_with_bare_error() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        print_integer("Blah", true, &mut out, &mut err);
        assert_eq!(out.len(), 0);
        let expected = format!("{}\n", Red.paint("BLAH is not a valid Roman numeral"));
        assert_eq!(err, expected.as_bytes());
    }
}
