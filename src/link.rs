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
            self.from.as_path().display(),
            self.to.as_path().display()
        )
    }
}

pub fn symlink(link_entry: &LinkEntry) -> Result<(), io::Error> {
    let from = &link_entry.from;
    let to = &link_entry.to;

    if let Err(e) = std::os::unix::fs::symlink(from, to) {
        if e.kind() == std::io::ErrorKind::AlreadyExists
            && fs::symlink_metadata(to).unwrap().is_symlink()
            && &fs::read_link(to).unwrap() == from
        {
            return Ok(());
        }

        return Err(e);
    }

    Ok(())
}