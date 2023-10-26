use anyhow::anyhow;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

pub fn expand_tilde(path: &Path) -> anyhow::Result<PathBuf> {
    let path = match path.strip_prefix("~") {
        Ok(stripped) => BaseDirs::new()
            .ok_or(anyhow!("Couldn't retreive home directory"))?
            .home_dir()
            .join(stripped),
        Err(_) => PathBuf::from(path),
    };

    Ok(path)
}

pub fn expand_path(path: &Path) -> anyhow::Result<PathBuf> {
    Ok(expand_tilde(path)?.canonicalize()?)
}
