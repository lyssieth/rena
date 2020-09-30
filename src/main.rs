use std::path::PathBuf;

use clap::{crate_authors, crate_description, crate_version, App, Arg, ValueHint};
use color_eyre::{install, Result};

mod logic;
mod util;

fn main() -> Result<()> {
    install()?; // Install color_eyre

    let app = build_app();

    let matches = app.get_matches();

    let folder = matches
        .value_of_t::<PathBuf>("folder")
        .expect("Unable to turn 'folder' argument into path");
    let save_log = matches
        .value_of_t::<bool>("save-log")
        .expect("Unable to turn 'save-log' argument into bool");
    let yes = matches
        .value_of_t::<bool>("yes")
        .expect("Unable to turn 'yes' argument into bool");
    let origin = matches
        .value_of_t::<usize>("origin")
        .expect("Unable to turn 'origin' argument into usize");
    let prefix = matches
        .value_of_t::<String>("prefix")
        .expect("Unable to find 'prefix' argument or use default");

    logic::run(logic::Arguments {
        folder,
        save_log,
        yes,
        origin,
        prefix,
    })?;
    Ok(())
}

fn build_app() -> App<'static> {
    App::new("rena")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("folder")
                .about("Path to the image folder")
                .index(1)
                .takes_value(true)
                .required(true)
                .value_hint(ValueHint::DirPath)
                .value_name("FOLDER")
                .validator(util::validate_directory),
        )
        .arg(
            Arg::new("save-log")
                .about("Save a log file to [rename.log]")
                .takes_value(false)
                .short('l')
                .long("save-log"),
        )
        .arg(
            Arg::new("yes")
                .about("Automatically answers 'yes' to prompts.")
                .takes_value(false)
                .short('y')
                .long("yes"),
        )
        .arg(
            Arg::new("origin")
                .about("Number to start counting at")
                .takes_value(true)
                .short('n')
                .long("origin")
                .required(false)
                .default_value("0")
                .default_missing_value("0")
                .validator(util::validate_usize),
        )
        .arg(
            Arg::new("prefix")
                .about("Prefix for every file")
                .takes_value(true)
                .short('p')
                .long("prefix")
                .required(false)
                .default_value("")
                .default_missing_value(""),
        )
}
