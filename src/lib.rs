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
    // unused,
    bad_style,
    clippy::unwrap_used
)]
#![warn(clippy::pedantic, clippy::nursery)]

//! Rena is a crate fo bulk renaming of files.

#[cfg(test)]
mod test;

use clap::{parser::MatchesError, ArgMatches};
use color_eyre::{eyre::eyre, Report, Result};
use paris::{info, warn};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::PathBuf,
    string::ToString,
};

/// All the arguments after being turned into their respective types.
#[derive(Debug, Clone, Default)]
pub struct Arguments {
    /// Folder in which to act
    pub folder: PathBuf,
    /// Whether to run on directories instead of files
    pub directory: bool,
    /// Whether to output more logging information
    pub verbose: bool,
    /// If renaming numerically, what number to start with
    pub origin: usize,
    /// The prefix for the item's name
    pub prefix: String,
    /// How much padding the number should have
    pub padding: usize,
    /// Which direction the number should be padded in
    pub padding_direction: PaddingDirection,
    /// A Regex to filter input items
    pub match_regex: Option<Regex>,
    /// When renaming, this is used to apply regex capture groups
    pub match_rename: Option<String>,
    /// Whether to not actually execute any rename operations
    pub dry_run: bool,
}

/// Direction in which to pad.
#[derive(Debug, Clone, Default)]
pub enum PaddingDirection {
    /// Pad left (00001)
    #[default]
    Left,
    /// Pad right (10000)
    Right,
    /// Pad middle (00100)
    Middle,
}

impl From<&String> for PaddingDirection {
    fn from(a: &String) -> Self {
        let a = a.to_lowercase();

        match a.as_ref() {
            "left" | "l" | "<" => Self::Left,
            "right" | "r" | ">" => Self::Right,
            "middle" | "m" | "|" => Self::Middle,
            _ => unreachable!(
                "If this is reached, something in validation has gone *horribly* wrong."
            ),
        }
    }
}

impl From<String> for PaddingDirection {
    fn from(a: String) -> Self {
        let a = a.to_lowercase();

        match a.as_ref() {
            "left" | "l" | "<" => Self::Left,
            "right" | "r" | ">" => Self::Right,
            "middle" | "m" | "|" => Self::Middle,
            _ => unreachable!(
                "If this is reached, something in validation has gone *horribly* wrong."
            ),
        }
    }
}

/// Just an intermediary struct to contain some data.
#[derive(Debug, Clone)]
struct RenameItem {
    pub original_path: PathBuf,
    pub new_path: PathBuf,
}

impl TryFrom<ArgMatches> for Arguments {
    type Error = Report;

    fn try_from(a: ArgMatches) -> Result<Self, Self::Error> {
        let folder = a
            .get_one::<PathBuf>("folder")
            .cloned()
            .ok_or_else(|| Report::msg("Unable to turn 'folder' argument into path"))?;
        let directory = a.get_flag("directory");
        let verbose = a.get_flag("verbose");
        let origin = a
            .get_one::<usize>("origin")
            .copied()
            .ok_or_else(|| Report::msg("Unable to turn 'origin' argument into usize"))?;
        let prefix = a
            .get_one::<String>("prefix")
            .cloned()
            .ok_or_else(|| Report::msg("Unable to find 'prefix' argument or use default"))?;
        let padding = a
            .get_one::<usize>("padding")
            .copied()
            .ok_or_else(|| Report::msg("Unable to turn 'padding' argument into usize"))?;
        let padding_direction = match a.try_get_one::<String>("padding_direction") {
            // For some reason the default wasn't working here so I removed it and made it manually default
            Ok(value) => value.map_or_else(PaddingDirection::default, PaddingDirection::from),
            Err(e) => match e {
                MatchesError::UnknownArgument { .. } => PaddingDirection::default(),
                _ => return Err(Report::msg("Invalid `--padding-direction argument.`")),
            },
        };
        let match_regex = match a.try_get_one::<String>("match") {
            Ok(Some(regex)) => Some(Regex::new(regex)?),
            Ok(None) => None,
            Err(e) => {
                return Err(Report::msg(format!("Invalid `--match` argument: {e}")));
            }
        };
        let match_rename = match a.try_get_one::<String>("match-rename") {
            Ok(Some(a)) => Some(a.clone()),
            Ok(None) => None,
            Err(e) => {
                return Err(Report::msg(format!(
                    "Invalid `--match-rename` argument: {e}"
                )));
            }
        };
        let dry_run = a.get_flag("dry-run");

        Ok(Self {
            folder,
            directory,
            verbose,
            origin,
            prefix,
            padding,
            padding_direction,
            match_regex,
            match_rename,
            dry_run,
        })
    }
}

/// Runs rena with the given arguments.
///
///
/// # Errors
///
/// Returns an error in the following circumstances:
///
/// - The target doesn't exist
/// - The target is not a directory
/// - We can't read the directory's contents
///
/// # Panics
///
/// We currently verify that the result of [`read_dir()`] is not `Err` before
/// unwrapping it, so this shouldn't ever panic.
pub fn run(args: Arguments) -> Result<()> {
    if !args.folder.exists() {
        return Err(eyre!(format!(
            "Folder `{}` does not exist.",
            args.folder.to_string_lossy()
        )));
    }

    if !args.folder.is_dir() {
        return Err(eyre!(format!(
            "`{}` is not a folder.",
            args.folder.to_string_lossy()
        )));
    }

    let read = args.folder.read_dir();

    if let Err(e) = read {
        return Err(eyre!(format!(
            "Unable to read directory {}: {}",
            args.folder.to_string_lossy(),
            e
        )));
    }
    let read = read.expect("Failed to read directory");

    let items = match &args.match_regex {
        Some(r) => filter_items_regex(read, args.directory, r),
        None => filter_items(read, args.directory),
    };

    if args.match_rename.is_some() {
        rename_regex(&items, args);
    } else {
        rename_normal(&items, args);
    }

    Ok(())
}

// Janky, but it works. I think. We'll see, hopefully.
fn rename_normal(items: &[PathBuf], args: Arguments) {
    let verbose = args.verbose;
    let fmt = match args.padding_direction {
        PaddingDirection::Left => {
            "{folder}/{prefix}_{number:0>NUM}{ext}".replace("NUM", &format!("{}", args.padding))
        }
        PaddingDirection::Right => {
            "{folder}/{prefix}_{number:0<NUM}{ext}".replace("NUM", &format!("{}", args.padding))
        }
        PaddingDirection::Middle => {
            "{folder}/{prefix}_{number:0|NUM}{ext}".replace("NUM", &format!("{}", args.padding))
        }
    };

    let fmt = if args.directory { fmt + "/" } else { fmt };

    let mut count = args.origin;

    let mut map = HashMap::new();

    map.insert(
        "folder".to_string(),
        args.folder.to_string_lossy().to_string(),
    );
    map.insert("prefix".to_string(), args.prefix);

    let items = items
        .iter()
        .map(|x| {
            let ext = x
                .extension()
                .map_or_else(String::new, |x| format!(".{}", x.to_string_lossy()));
            map.insert("number".to_string(), format!("{count}"));
            map.insert("ext".to_string(), ext);
            count += 1;

            RenameItem {
                original_path: x.clone(),
                new_path: strfmt::strfmt(&fmt, &map)
                    .expect("String formatting failed")
                    .into(),
            }
        })
        .filter(|x| {
            if x.new_path.exists() {
                warn!(
                    "File `{}` already exists, unable to rename.",
                    x.new_path.to_string_lossy()
                );
                false
            } else {
                true
            }
        })
        .collect::<Vec<RenameItem>>();

    let dry_run = args.dry_run;
    for x in items {
        if x.new_path.exists() {
            warn!(
                "Item `{}` already exists, unable to rename.",
                x.new_path.to_string_lossy()
            );
            return;
        }
        if dry_run {
            info!(
                "[DRY RUN]: `{}` -> `{}`",
                x.original_path.to_string_lossy(),
                x.new_path.to_string_lossy()
            );
        } else {
            match fs::rename(&x.original_path, &x.new_path) {
                Ok(()) => {
                    if verbose {
                        info!(
                            "[DONE] `{}` -> `{}`",
                            x.original_path.to_string_lossy(),
                            x.new_path.to_string_lossy()
                        );
                    }
                }
                Err(e) => warn!(
                    "[FAIL] `{}` -> `{}`: {}",
                    x.original_path.to_string_lossy(),
                    x.new_path.to_string_lossy(),
                    e
                ),
            }
        }
    }
}

fn rename_regex(items: &[PathBuf], args: Arguments) {
    let verbose = args.verbose;

    let regex = args.match_regex.expect("Regex is None");
    let match_rename = args.match_rename.expect("Match rename is None");
    let items = items
        .iter()
        .map(|x| {
            let text = x
                .file_name()
                .expect("there to be a filename")
                .to_string_lossy();
            let after = regex.replace(&text, match_rename.as_str()).to_string();
            let mut new_x = x.clone();

            new_x.set_file_name(after);

            RenameItem {
                original_path: x.clone(),
                new_path: new_x,
            }
        })
        .filter(|x| {
            if x.new_path.exists() {
                warn!(
                    "Item `{}` already exists, unable to rename.",
                    x.new_path.to_string_lossy()
                );
                false
            } else {
                true
            }
        })
        .collect::<Vec<RenameItem>>();

    let dry_run = args.dry_run;
    for x in items {
        if x.new_path.exists() {
            warn!(
                "Item `{}` already exists, unable to rename.",
                x.new_path.to_string_lossy()
            );
            return;
        }
        if dry_run {
            info!(
                "[DRY RUN]: `{}` -> `{}`",
                x.original_path.to_string_lossy(),
                x.new_path.to_string_lossy()
            );
        } else {
            match fs::rename(&x.original_path, &x.new_path) {
                Ok(()) => {
                    if verbose {
                        info!(
                            "[DONE] `{}` -> `{}`",
                            x.original_path.to_string_lossy(),
                            x.new_path.to_string_lossy()
                        );
                    }
                }
                Err(e) => warn!(
                    "[FAIL] `{}` -> `{}`: {}",
                    x.original_path.to_string_lossy(),
                    x.new_path.to_string_lossy(),
                    e
                ),
            }
        }
    }
}

fn filter_items<I>(read: I, dir: bool) -> Vec<PathBuf>
where
    I: Iterator<Item = std::io::Result<DirEntry>>,
{
    let items = read.filter(|x| match x {
        Err(e) => {
            warn!("Unable to read item: {}", e);
            false
        }
        Ok(item) => {
            let item_type = item.file_type();

            if let Err(e) = item_type {
                warn!(
                    "Unable to get filetype of {}: {}",
                    item.file_name().to_string_lossy(),
                    e
                );
                return false;
            }

            let item_type = item_type.expect("item_type is None");

            if dir {
                item_type.is_dir()
            } else {
                item_type.is_file()
            }
        }
    });

    items
        .map(|x| {
            let x = x.expect("item is None");

            x.path()
        })
        .collect::<Vec<PathBuf>>()
}

fn filter_items_regex<I>(read: I, dir: bool, regex: &Regex) -> Vec<PathBuf>
where
    I: Iterator<Item = std::io::Result<DirEntry>>,
{
    let items = read.filter(|x| match x {
        Err(e) => {
            warn!("Unable to read item: {}", e);
            false
        }
        Ok(item) => {
            let item_type = item.file_type();
            let item_name = item.file_name();
            let item_name = item_name.to_string_lossy();

            if let Err(e) = item_type {
                warn!("Unable to get filetype of {}: {}", item_name, e);
                return false;
            }

            let item_type = item_type.expect("item_type is None");

            regex.is_match(&item_name)
                && if dir {
                    item_type.is_dir()
                } else {
                    item_type.is_file()
                }
        }
    });

    items
        .map(|x| {
            let x = x.expect("item is None");

            x.path()
        })
        .collect::<Vec<PathBuf>>()
}
