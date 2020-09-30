use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgSettings, ValueHint};
use color_eyre::{install, Result};
use paris::{error, info};

mod logic;
mod util;

fn main() -> Result<()> {
    install()?; // Install color_eyre

    let app = build_app();

    let matches = app.get_matches();

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
                .about("Path to the image folder")
                .index(1)
                .takes_value(true)
                .required(true)
                .value_hint(ValueHint::DirPath)
                .value_name("FOLDER")
                .validator(util::validate_directory),
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
                .validator(util::validate_usize)
                .unset_setting(ArgSettings::UseValueDelimiter),
        )
        .arg(
            Arg::new("prefix")
                .about("Prefix for every file")
                .takes_value(true)
                .short('p')
                .long("prefix")
                .required(false)
                .default_value("image_")
                .default_missing_value("image_")
                .unset_setting(ArgSettings::UseValueDelimiter),
        )
}
