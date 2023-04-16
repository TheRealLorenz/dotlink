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

impl LinkEntry {
    pub fn symlink(&self) -> Result<(), io::Error> {
        let from = &self.from;
        let to = &self.to;

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
}
