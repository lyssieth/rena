use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use paris::warn;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    path::PathBuf,
};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Arguments {
    pub folder: PathBuf,
    pub yes: bool,
    pub origin: usize,
    pub prefix: String,
}

struct RenameFile {
    pub original_path: PathBuf,
    pub new_path: PathBuf,
}

impl Into<Arguments> for ArgMatches {
    fn into(self) -> Arguments {
        let folder = self
            .value_of_t::<PathBuf>("folder")
            .expect("Unable to turn 'folder' argument into path");
        let yes = self.is_present("yes");
        let origin = self
            .value_of_t::<usize>("origin")
            .expect("Unable to turn 'origin' argument into usize");
        let prefix = self
            .value_of_t::<String>("prefix")
            .expect("Unable to find 'prefix' argument or use default");

        Arguments {
            folder,
            yes,
            origin,
            prefix,
        }
    }
}

pub fn run(args: Arguments) -> Result<()> {
    dbg!(&args);
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

    let files = filter_files(read.unwrap());

    let fmt = "{folder}/{prefix}_{number:0>10}{ext}";

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
                new_path: strfmt::strfmt(fmt, &map).unwrap().into(),
            }
        })
        .collect::<Vec<RenameFile>>();

    files
        .par_iter()
        .for_each(|x| match fs::rename(&x.original_path, &x.new_path) {
            Ok(_) => {}
            Err(e) => warn!(
                "Failed to rename `{}` to `{}`: {}",
                x.original_path.to_string_lossy(),
                x.new_path.to_string_lossy(),
                e
            ),
        });

    Ok(())
}

fn filter_files(read: ReadDir) -> Vec<PathBuf> {
    let images = read.filter(|x| match x {
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

            let item_type = item_type.unwrap();

            item_type.is_file()
        }
    });

    images
        .map(|x| {
            let x = x.unwrap();

            x.path()
        })
        .collect::<Vec<PathBuf>>()
}
