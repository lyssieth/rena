use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use paris::warn;
use std::{fs::ReadDir, path::PathBuf};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Arguments {
    pub folder: PathBuf,
    pub yes: bool,
    pub origin: usize,
    pub prefix: String,
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

    let images = get_images(read.unwrap());

    Ok(())
}

fn get_images(read: ReadDir) -> Vec<PathBuf> {
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

            if item_type.is_dir() || item_type.is_symlink() {
                return false;
            }

            let mime = tree_magic::from_filepath(&item.path());

            mime.starts_with(mime::IMAGE.as_str())
        }
    });

    images
        .map(|x| {
            let x = x.unwrap();

            x.path()
        })
        .collect::<Vec<PathBuf>>()
}
