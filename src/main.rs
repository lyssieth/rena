/*
MIT License
Copyright (c) 2020 Lyssieth

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
#![forbid(unsafe_code)]

use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgSettings, ValueHint};
use color_eyre::{install, Result};
use paris::{error, info};

mod util;

fn main() -> Result<()> {
    install()?; // Install color_eyre

    let app = build_app();

    let matches = app.get_matches();

    info!("Starting execution...");
    match rena::run(matches.into()) {
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
            Arg::new("directory")
                .about("Causes the app to act on directories instead of files.")
                .takes_value(false)
                .long("dir")
                .required(false)
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
                .long_about("Prefix for every file, without any delimiters.")
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
                .about("Amount of padding to add to a file.")
                .takes_value(true)
                .long("padding")
                .required(false)
                .default_value("10")
                .default_missing_value("10")
                .unset_setting(ArgSettings::UseValueDelimiter),
        )
        .arg(Arg::new("padding-direction")
            .about("Changes the direction of the padding. Defaults ro `right`")
            .takes_value(true)
            .long("padding-direction")
            .required(false)
            .possible_values(&["left", "l", "<", "middle", "m", "|", "right", "r", ">"])
            .unset_setting(ArgSettings::UseValueDelimiter))
        .arg(
            Arg::new("match")
                .about("Valid RegEx for matching input files (see 'match-rename' argument).")
                .takes_value(true)
                .short('m')
                .long("match")
                .required(false)
                .unset_setting(ArgSettings::UseValueDelimiter)
                .validator(util::validate_regex),
        )
        .arg(
            Arg::new("match-rename")
                .about("Use capture groups from 'match' argument to rename files. See `--help` for more info.")
                .long_about(
                    "Use capture groups from 'match' argument to rename files.
                    Capture group numbers need a `$` prefix, so `$1` for the first, 
                    `$2` for the second, and so on, with `$0` matching the entire name.
                    Recommend using `--dry-run` flag.
                    If it fails to see groups try using `${1}`, as in surround the
                    group index with `{}`.",
                )
                .takes_value(true)
                .long("match-rename")
                .requires("match")
                .required(false)
                .unset_setting(ArgSettings::UseValueDelimiter),
        )
        .arg(
            Arg::new("dry-run")
                .about("Disables performing actual renaming.")
                .takes_value(false)
                .long("dry-run")
                .required(false),
        )
}
