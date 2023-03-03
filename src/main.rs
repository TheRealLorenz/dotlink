use home::home_dir;
use lazy_static::lazy_static;
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
    process::Command,
};

lazy_static! {
    static ref BLACKLISTED: &'static [&'static str; 3] = &[".git", "README.md", ".gitignore"];
}

fn symlink(a: &Path, b: &Path) {
    Command::new("ln")
        .arg("-sf")
        .arg(a)
        .arg(b)
        .output()
        .expect(format!("Failed to symlink '{}' to '{}'", a.display(), b.display()).as_str());
}

fn expand_path(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    if !path.starts_with("~") {
        return Ok(path.canonicalize()?);
    }

    Ok(home_dir()
        .ok_or("Couldn't retreive home dir path")?
        .join(path.strip_prefix("~")?)
        .canonicalize()?)
}

fn is_blacklisted(entry: &Path) -> bool {
    BLACKLISTED.contains(
        &entry
            .file_name()
            .and_then(|name| name.to_str())
            .expect(format!("Invalid file name '{}'", entry.to_string_lossy()).as_str()),
    )
}

fn link_dir(path: &Path) -> Result<(), io::Error> {
    path.read_dir()?
        .filter(|entry| !is_blacklisted(&entry.as_ref().unwrap().path()))
        .for_each(|entry| {
            let entry = entry.unwrap();
            link_file(&entry.path());
        });

    Ok(())
}

fn link_file(path: &Path) {
    #[cfg(debug_assertions)]
    println!("Linking '{}'", path.display());

    if is_blacklisted(path) {
        return;
    }

    match expand_path(path) {
        Ok(path) => symlink(path.as_path(), path.as_path()),
        Err(err) => println!("Couldn't expand path '{}': {}", path.display(), err),
    }
}

fn main() {
    link_dir(Path::new(".")).expect(format!("Failed to link '{}'", ".").as_str());
}
