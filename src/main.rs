use clap::Parser;
use lazy_static::lazy_static;
use std::{
    env::{self, VarError},
    error::Error,
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

fn is_blacklisted(entry: &dyn AsRef<Path>) -> bool {
    BLACKLISTED.contains(
        &entry
            .as_ref()
            .file_name()
            .and_then(|name| name.to_str())
            .expect(format!("Invalid file name '{}'", entry.as_ref().to_string_lossy()).as_str()),
    )
}

fn link_entry(path: &Path, to: &Path) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    println!("Linking '{}' to '{}'", path.display(), to.display());

    if is_blacklisted(&path) {
        return Ok(());
    }

    symlink(&expand_path(path)?, &expand_path(to)?);

    Ok(())
}

fn main() {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Args: {:?}", args);

    if args.paths.is_empty() {
        println!("No files to link");
        return;
    }

    let base_path = Path::new(&args.base_dir);

    for path in &args.paths {
        if is_blacklisted(path) {
            continue;
        }

        link_entry(Path::new(path), &base_path)
            .expect(format!("Failed to link '{}'", path).as_str());
    }
}
