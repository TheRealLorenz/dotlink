use colored::*;
use std::{fmt, fs, io, path::PathBuf};

pub struct LinkEntry {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl fmt::Display for LinkEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}) -> ({})",
            self.from.file_name().unwrap().to_str().unwrap(),
            self.to.as_path().display()
        )
    }
}

enum LinkSuccess {
    Linked,
    AlreadyLinked,
}

fn symlink(from: &PathBuf, to: &PathBuf) -> Result<LinkSuccess, io::Error> {
    if let Err(e) = std::os::unix::fs::symlink(from, to) {
        if e.kind() == std::io::ErrorKind::AlreadyExists
            && fs::symlink_metadata(to).unwrap().is_symlink()
            && &fs::read_link(to).unwrap() == from
        {
            return Ok(LinkSuccess::AlreadyLinked);
        }

        return Err(e);
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
            Ok(LinkSuccess::Linked) => println!("{}", "✓".green().bold()),
            Ok(LinkSuccess::AlreadyLinked) => println!("{}", "✓ - already linked".yellow().bold()),
            Err(e) => println!("{}", format!("X - {}", e).red().bold()),
        }
    }
}
