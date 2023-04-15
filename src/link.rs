use std::{
    fmt, io,
    path::{Path, PathBuf},
};

use crate::expand;

#[derive(Debug)]
pub enum LinkError {
    Io(io::Error),
    Expand(expand::ExpandError),
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkError::Io(e) => write!(f, "Couldn't link file: {}", e),
            LinkError::Expand(e) => write!(f, "Couldn't link file: {}", e),
        }
    }
}

impl From<io::Error> for LinkError {
    fn from(value: io::Error) -> Self {
        LinkError::Io(value)
    }
}

impl From<expand::ExpandError> for LinkError {
    fn from(value: expand::ExpandError) -> Self {
        LinkError::Expand(value)
    }
}

pub fn symlink(
    from_dir: &PathBuf,
    name: &String,
    to: &String,
    dry_run: bool,
) -> Result<(), LinkError> {
    println!("Linking '{name}' to '{to}'");

    if dry_run {
        return Ok(());
    }

    let from = PathBuf::from(from_dir).join(name);
    let to = expand::expand_tilde(Path::new(to))?;

    std::os::unix::fs::symlink(from, to)?;

    Ok(())
}
