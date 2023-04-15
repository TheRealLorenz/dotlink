use std::{error, fmt, io};

#[derive(Debug)]
pub enum LoadError {
    Read(io::Error),
    Parse(toml::de::Error),
}

impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        LoadError::Read(value)
    }
}

impl From<toml::de::Error> for LoadError {
    fn from(value: toml::de::Error) -> Self {
        LoadError::Parse(value)
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Read(e) => write!(f, "Couldn't read config file: {e}"),
            LoadError::Parse(e) => write!(f, "Couldn't parse config file: {e}"),
        }
    }
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            LoadError::Read(e) => Some(e),
            LoadError::Parse(e) => Some(e),
        }
    }
}
