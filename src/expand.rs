use std::{
    fmt, io,
    path::{Path, PathBuf},
};

use directories::BaseDirs;

#[derive(Debug)]
pub enum ExpandError {
    HomeDir,
    Io(io::Error),
}

impl From<io::Error> for ExpandError {
    fn from(value: io::Error) -> Self {
        ExpandError::Io(value)
    }
}

impl fmt::Display for ExpandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpandError::HomeDir => {
                write!(f, "Couldn't expand path: couldn't retreive home directory")
            }
            ExpandError::Io(e) => write!(f, "couldn't expand path: {e}"),
        }
    }
}

pub fn expand_tilde(path: &Path) -> Result<PathBuf, ExpandError> {
    let path = match path.strip_prefix("~") {
        Ok(stripped) => BaseDirs::new()
            .ok_or(ExpandError::HomeDir)?
            .home_dir()
            .join(stripped),
        Err(_) => PathBuf::from(path),
    };

    Ok(path)
}

pub fn expand_path(path: &Path) -> Result<PathBuf, ExpandError> {
    Ok(expand_tilde(path)?.canonicalize()?)
}
