use clap::Parser;
use lazy_static::lazy_static;
use std::{
    env::{self, VarError},
    error::Error,
    io,
    path::{Path, PathBuf},
    process::Command,
};

lazy_static! {
    static ref BLACKLISTED: &'static [&'static str; 3] = &[".git", "README.md", ".gitignore"];
}

#[derive(Parser, Debug)]
struct Args {
    paths: Vec<String>,

    #[arg(short, long, default_value_t = (&"$XDG_CONFIG_HOME").to_string())]
    base_dir: String,
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

    let prefix = env::var("XDG_CONFIG_HOME")
        .or_else(|_| Ok::<_, VarError>(env::var("HOME")? + "/.config"))?;

    Ok(PathBuf::from(prefix)
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

fn link_dir(path: &Path, to: &Path) -> Result<(), io::Error> {
    path.read_dir()?
        .filter(|entry| !is_blacklisted(&entry.as_ref().unwrap().path()))
        .for_each(|entry| {
            let entry = entry.unwrap();
            match link_entry(&entry.path(), &to) {
                Ok(()) => (),
                Err(err) => println!(
                    "Couldn't link '{}': {}",
                    entry.file_name().to_string_lossy(),
                    err
                ),
            }
        });

    Ok(())
}

fn link_entry(path: &Path, to: &Path) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    println!("Linking '{}'", path.display());

    if is_blacklisted(path) {
        return Ok(());
    }

    symlink(&expand_path(path)?, &expand_path(to)?);

    Ok(())
}

fn main() {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Args: {:?}", args);

    link_dir(Path::new("."), &Path::new(&args.base_dir))
        .expect(format!("Failed to link '{}'", ".").as_str());
}
