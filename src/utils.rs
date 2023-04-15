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

pub fn expand_path(path: &Path) -> PathBuf {
    expand_tilde(path)
        .canonicalize()
        .unwrap_or_else(|error| panic!("Couldn't expand '{:?}': {}", path, error))
}

pub fn symlink(from_dir: &PathBuf, name: &String, to: &String, dry_run: bool) {
    println!("Linking '{name}' to '{to}'");

    if dry_run {
        return;
    }

    let from = PathBuf::from(from_dir).join(name);
    let to = expand_tilde(Path::new(to));

    std::os::unix::fs::symlink(&from, &to).unwrap_or_else(|error| {
        panic!(
            "An error occurred while linking '{:?}' to '{:?}': {}",
            from, to, error
        );
    });
}
