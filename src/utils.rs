use std::{
    io,
    path::{Path, PathBuf},
};

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

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    std::os::unix::fs::symlink(from, to)?;

    Ok(())
}
