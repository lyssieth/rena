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
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use paris::{info, warn};
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Arguments {
    pub folder: PathBuf,
    pub verbose: bool,
    pub origin: usize,
    pub prefix: String,
    pub padding: usize,
    pub match_regex: Option<Regex>,
    pub match_rename: String,
}

#[derive(Debug, Clone)]
struct RenameFile {
    pub original_path: PathBuf,
    pub new_path: PathBuf,
}

impl From<ArgMatches> for Arguments {
    fn from(a: ArgMatches) -> Self {
        let folder = a
            .value_of_t::<PathBuf>("folder")
            .expect("Unable to turn 'folder' argument into path");
        let verbose = a.is_present("verbose");
        let origin = a
            .value_of_t::<usize>("origin")
            .expect("Unable to turn 'origin' argument into usize");
        let prefix = a
            .value_of_t::<String>("prefix")
            .expect("Unable to find 'prefix' argument or use default");
        let padding = a
            .value_of_t::<usize>("padding")
            .expect("Unable to turn 'padding' argument into usize");
        let match_regex = match a.value_of("match") {
            Some(regex) => {
                let a = Regex::new(regex);
                match a {
                    Ok(a) => Some(a),
                    Err(e) => {
                        panic!("Unable to parse regex `{}`: {}", regex, e);
                    }
                }
            }
            None => None,
        };
        let match_rename = a
            .value_of_t::<String>("match-rename")
            .expect("Unable to turn 'match-rename' argument into String");

        Self {
            folder,
            verbose,
            origin,
            prefix,
            padding,
            match_regex,
            match_rename,
        }
    }
}

pub fn run(args: Arguments) -> Result<()> {
    let verbose = args.verbose;
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
    let read = read.unwrap();

    let files = match args.match_regex {
        Some(r) => filter_files_regex(read, r),
        None => filter_files(read),
    };

    let fmt = "{folder}/{prefix}_{number:0>NUM}{ext}".replace("NUM", &format!("{}", args.padding));

    let mut count = args.origin;

    let mut map = HashMap::new();

    map.insert(
        "folder".to_string(),
        args.folder.to_string_lossy().to_string(),
    );
    map.insert("prefix".to_string(), args.prefix);

    let files = files
        .iter()
        .map(|x| {
            let ext = match x.extension() {
                Some(x) => format!(".{}", x.to_string_lossy().to_string()),
                None => "".to_string(),
            };
            map.insert("number".to_string(), format!("{}", count));
            map.insert("ext".to_string(), ext);
            count += 1;

            RenameFile {
                original_path: x.clone(),
                new_path: strfmt::strfmt(&fmt, &map).unwrap().into(),
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
        .collect::<Vec<RenameFile>>();

    files.par_iter().for_each(|x| {
        if x.new_path.exists() {
            warn!(
                "File `{}` already exists, unable to rename.",
                x.new_path.to_string_lossy()
            );
            return;
        }
        match fs::rename(&x.original_path, &x.new_path) {
            Ok(_) => {
                if verbose {
                    info!(
                        "`{}` -> `{}`",
                        x.original_path.to_string_lossy(),
                        x.new_path.to_string_lossy()
                    );
                }
            }
            Err(e) => warn!(
                "Failed to rename `{}` to `{}`: {}",
                x.original_path.to_string_lossy(),
                x.new_path.to_string_lossy(),
                e
            ),
        }
    });

    Ok(())
}

// TODO: Move all current logic here.
fn rename_normal(args: Arguments) -> Result<()> {
    panic!("This is unimplemented!");
}

// TODO: Write new logic for regex renaming.
fn rename_regex(args: Arguments) -> Result<()> {
    panic!("This is unimplemented!");
}

fn filter_files(read: ReadDir) -> Vec<PathBuf> {
    let files = read.filter(|x| match x {
        Err(e) => {
            warn!("Unable to read file: {}", e);
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

            let item_type = item_type.unwrap();

            item_type.is_file()
        }
    });

    files
        .map(|x| {
            let x = x.unwrap();

            x.path()
        })
        .collect::<Vec<PathBuf>>()
}

fn filter_files_regex(read: ReadDir, regex: Regex) -> Vec<PathBuf> {
    let files = read.filter(|x| match x {
        Err(e) => {
            warn!("Unable to read file: {}", e);
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

            let item_type = item_type.unwrap();

            item_type.is_file() && regex.is_match(&item_name)
        }
    });

    files
        .map(|x| {
            let x = x.unwrap();

            x.path()
        })
        .collect::<Vec<PathBuf>>()
}
