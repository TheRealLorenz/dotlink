use anyhow::anyhow;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

pub fn tilde<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf> {
    let path = match path.as_ref().strip_prefix("~") {
        Ok(stripped) => BaseDirs::new()
            .ok_or(anyhow!("Couldn't retreive home directory"))?
            .home_dir()
            .join(stripped),
        Err(_) => PathBuf::from(path.as_ref()),
    };

    Ok(path)
}

pub fn path<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf> {
    Ok(tilde(path)?.canonicalize()?)
}
