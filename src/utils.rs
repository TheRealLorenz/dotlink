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

pub fn symlink(from_dir: &PathBuf, name: &String, to: &String, dry_run: bool) -> io::Result<()> {
    println!("Symlinking '{name}' to '{to}'");

    if dry_run {
        return Ok(())
    }

    let from = PathBuf::from(from_dir).join(name);
    let to = expand_tilde(Path::new(to));

    std::os::unix::fs::symlink(from, to)?;

    Ok(())
}
