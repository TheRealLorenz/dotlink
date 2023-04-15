use std::path::{Path, PathBuf};

use directories::BaseDirs;

pub fn expand_tilde(path: &Path) -> PathBuf {
    match path.strip_prefix("~") {
        Ok(stripped) => BaseDirs::new()
            .expect("Couldn't retreive home directory")
            .home_dir()
            .join(stripped),
        Err(_) => PathBuf::from(path),
    }
}
