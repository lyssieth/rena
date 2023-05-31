/*
MIT License
Copyright (c) 2020-2023 Lyssieth

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
#![deny(
    missing_docs,
    missing_debug_implementations,
    rustdoc::missing_crate_level_docs,
    unused,
    bad_style,
    clippy::unwrap_used
)]
#![warn(clippy::pedantic)]

//! Main executable of rena.

use clap::{
    builder::{NonEmptyStringValueParser, PossibleValuesParser, ValueParser},
    crate_authors, crate_description, crate_version, Arg, ArgAction, Command, ValueHint,
};
use color_eyre::{config::HookBuilder, Result};
use paris::{error, info};

fn main() -> Result<()> {
    HookBuilder::default()
        .issue_url("https://github.com/lyssieth/rena/issues/new")
        .add_issue_metadata("version", env!("CARGO_PKG_VERSION"))
        .install()?; // Install color_eyre

    let app = build_app();

    let matches = app.get_matches();

    info!("Starting execution...");
    match rena::run(matches.into()) {
        Ok(_) => info!("Completed successfully!"),
        Err(e) => error!("Encountered an error: {}", e),
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn build_app() -> Command {
    Command::new("rena")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .after_help("For more information, visit <https://github.com/lyssieth/rena>")
        .arg(
            Arg::new("folder")
                .help("Path to the folder containing items")
                .index(1)
                .required(true)
                .value_hint(ValueHint::DirPath)
                .action(ArgAction::Set)
                .value_name("FOLDER")
                .value_parser(ValueParser::path_buf())
        )
        .arg(
            Arg::new("directory")
                .help("Causes the app to act on directories instead of files.")
                .action(ArgAction::SetTrue)
                .value_hint(ValueHint::Other)
                .value_parser(ValueParser::bool())
                .long("dir")
                .required(false)
        )
        .arg(
            Arg::new("verbose")
                .help("Turns on some (potentially) annoying logging for more verbose output.")
                .action(ArgAction::SetTrue)
                .value_hint(ValueHint::Other)
                .value_parser(ValueParser::bool())
                .long("verbose")
                .required(false),
        )
        .arg(
            Arg::new("origin")
                .help("Number to start counting at. Default: 0")
                .short('n')
                .long("origin")
                .required(false)
                .default_value("0")
                .action(ArgAction::Set)
                .default_missing_value("0")
                .value_parser(ValueParser::new(NonEmptyStringValueParser::new()))
                .value_hint(ValueHint::Other)
                .value_name("INDEX")
                .use_value_delimiter(false)
        )
        .arg(
            Arg::new("prefix")
                .help("Prefix for every file")
                .short('p')
                .long("prefix")
                .required(false)
                .action(ArgAction::Set)
                .value_parser(ValueParser::new(NonEmptyStringValueParser::new()))
                .default_value("item")
                .default_missing_value("item")
                .value_name("PREFIX")
                .value_hint(ValueHint::Other)
                .use_value_delimiter(false)
        )
        .arg(
            Arg::new("padding")
                .help("Amount of padding to add to a file.")
                .long("padding")
                .action(ArgAction::Set)
                .required(false)
                .default_value("10")
                .value_parser(ValueParser::new(NonEmptyStringValueParser::new()))
                .default_missing_value("10")
                .value_name("PADDING")
                .value_hint(ValueHint::Other)
                .use_value_delimiter(false)
        )
        .arg(
            Arg::new("padding-direction")
            .help("Changes the direction of the padding. Defaults ro `right`")
            .long("padding-direction")
            .required(false)
            .value_parser(PossibleValuesParser::new(["left", "l", "<", "middle", "m", "|", "right", "r", ">"]))
                .value_hint(ValueHint::Other)
            .action(ArgAction::Set)
            .use_value_delimiter(false)
        )
        .arg(
            Arg::new("match")
                .help("Valid RegEx for matching input files (see 'match-rename' argument).")
                .action(ArgAction::Set)
                .value_parser(ValueParser::new(NonEmptyStringValueParser::new()))
                .value_hint(ValueHint::Other)
                .short('m')
                .long("match")
                .required(false)
                .use_value_delimiter(false)
        )
        .arg(
            Arg::new("match-rename")
                .help("Use capture groups from 'match' argument to rename files. See `--help` for more info.")
                .long_help(
                    "Use capture groups from 'match' argument to rename files.
                    Capture group numbers need a `$` prefix, so `$1` for the first, 
                    `$2` for the second, and so on, with `$0` matching the entire name.
                    Recommend using `--dry-run` flag.
                    If it fails to see groups try using `${1}`, as in surround the
                    group index with `{}`.",
                )
                .action(ArgAction::Set)
                .value_parser(ValueParser::new(NonEmptyStringValueParser::new()))
                .value_hint(ValueHint::Other)
                .long("match-rename")
                .requires("match")
                .required(false)
                .use_value_delimiter(false)
        )
        .arg(
            Arg::new("dry-run")
                .help("Disables performing actual renaming.")
                .action(ArgAction::SetTrue)
                .value_hint(ValueHint::Other)
                .value_parser(ValueParser::bool())
                .long("dry-run")
                .required(false),
        )
}
