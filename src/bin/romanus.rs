use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version, ArgMatches};

fn main() {
    let _args = app_args();
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
