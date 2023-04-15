use std::{io, fmt, path::{PathBuf, Path}};

use crate::utils::expand_tilde;


#[derive(Debug)]
pub struct LinkError(io::Error);

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Couldn't link file: {}", self.0)
    }
}

impl From<io::Error> for LinkError {
    fn from(value: io::Error) -> Self {
        LinkError(value)
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
    let to = expand_tilde(Path::new(to));

    std::os::unix::fs::symlink(from, to)?;

    Ok(())
}
