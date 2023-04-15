use std::{fmt, io, path::PathBuf};

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

pub fn symlink(from: &PathBuf, to: &PathBuf, dry_run: bool) -> Result<(), LinkError> {
    println!(
        "Linking '{}' to '{}'",
        from.as_path().display(),
        to.as_path().display()
    );

    if dry_run {
        return Ok(());
    }

    std::os::unix::fs::symlink(from, to)?;

    Ok(())
}
