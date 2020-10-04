#![forbid(unsafe_code)]

use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgSettings, ValueHint};
use color_eyre::{install, Result};
use paris::{error, info};

mod logic;
mod util;

fn main() -> Result<()> {
    install()?; // Install color_eyre

    let app = build_app();

    let matches = app.get_matches();

    info!("Starting execution...");
    match logic::run(matches.into()) {
        Ok(_) => info!("Completed successfully!"),
        Err(e) => error!("Encountered an error: {}", e),
    }
    Ok(())
}

fn build_app() -> App<'static> {
    App::new("rena")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("folder")
                .about("Path to the folder containing items")
                .index(1)
                .takes_value(true)
                .required(true)
                .value_hint(ValueHint::DirPath)
                .value_name("FOLDER")
                .validator(util::validate_directory),
        )
        .arg(
            Arg::new("verbose")
                .about("Turns on some (potentially) annoying logging for more verbose output.")
                .takes_value(false)
                .long("verbose")
                .required(false),
        )
        .arg(
            Arg::new("origin")
                .about("Number to start counting at. Default: 0")
                .takes_value(true)
                .short('n')
                .long("origin")
                .required(false)
                .default_value("0")
                .default_missing_value("0")
                .validator(util::validate_usize)
                .unset_setting(ArgSettings::UseValueDelimiter),
        )
        .arg(
            Arg::new("prefix")
                .about("Prefix for every file")
                .long_about("Prefix for every file, without any delimiters. Defaults to `item`.")
                .takes_value(true)
                .short('p')
                .long("prefix")
                .required(false)
                .default_value("item")
                .default_missing_value("item")
                .unset_setting(ArgSettings::UseValueDelimiter),
        )
        .arg(
            Arg::new("padding")
                .about("Amount of padding to add to a file. Default: 10")
                .takes_value(true)
                .long("padding")
                .required(false)
                .default_value("10")
                .default_missing_value("10")
                .unset_setting(ArgSettings::UseValueDelimiter),
        )
        .arg(
            Arg::new("match")
                .about(
                    "Valid RegEx for matching input files (does not support capture groups yet).",
                )
                .takes_value(true)
                .short('m')
                .long("match")
                .required(false)
                .unset_setting(ArgSettings::UseValueDelimiter)
                .validator(util::validate_regex),
        )
}
