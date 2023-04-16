use std::{fs, io, path::PathBuf};

pub fn symlink(from: &PathBuf, to: &PathBuf) -> Result<(), io::Error> {
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
