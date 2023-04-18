use colored::*;
use std::{
    fmt, fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

pub struct LinkEntry {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl fmt::Display for LinkEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}' -> '{}'",
            self.from.file_name().unwrap().to_str().unwrap(),
            self.to.as_path().display()
        )
    }
}

trait Symlink {
    fn resolves_to(&self, to: &Path) -> io::Result<bool>;
    fn is_symlink(&self) -> io::Result<bool>;
}

impl Symlink for PathBuf {
    fn resolves_to(&self, destination: &Path) -> io::Result<bool> {
        Ok(fs::read_link(self)? == destination)
    }

    fn is_symlink(&self) -> io::Result<bool> {
        Ok(fs::symlink_metadata(self)?.is_symlink())
    }
}

enum LinkSuccess {
    Linked,
    AlreadyLinked,
}

enum LinkError {
    SourceNotFound,
    DestinationExists,
    Io(io::Error),
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkError::SourceNotFound => write!(f, "source doesn't exist"),
            LinkError::DestinationExists => write!(f, "destination already exists"),
            LinkError::Io(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for LinkError {
    fn from(value: io::Error) -> Self {
        LinkError::Io(value)
    }
}

fn symlink(from: &PathBuf, to: &PathBuf) -> Result<LinkSuccess, LinkError> {
    if !from.exists() {
        return Err(LinkError::SourceNotFound);
    }

    if let Err(e) = std::os::unix::fs::symlink(from, to) {
        return match e.kind() {
            ErrorKind::AlreadyExists => {
                if !to.is_symlink()? || !to.resolves_to(from)? {
                    return Err(LinkError::DestinationExists);
                }

                return Ok(LinkSuccess::AlreadyLinked);
            }
            _ => Err(LinkError::Io(e)),
        };
    }

    Ok(LinkSuccess::Linked)
}

impl LinkEntry {
    pub fn symlink(&self, dry_run: bool) {
        print!("Linking {}: ", self);

        if dry_run {
            println!("{}", "dry".yellow().bold());
            return;
        }

        match symlink(&self.from, &self.to) {
            Ok(LinkSuccess::Linked) => println!("{}", "✓ - linked".green().bold()),
            Ok(LinkSuccess::AlreadyLinked) => println!("{}", "✓ - already linked".yellow().bold()),
            Err(e) => println!("{}", format!("X - {}", e).red().bold()),
        }
    }
}
