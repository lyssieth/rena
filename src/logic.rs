use std::path::PathBuf;

use color_eyre::Result;

pub struct Arguments {
    pub folder: PathBuf,
    pub save_log: bool,
    pub yes: bool,
    pub origin: usize,
    pub prefix: String,
}

pub fn run(_args: Arguments) -> Result<()> {
    Ok(())
}
