use std::{
    fmt, fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

pub struct LinkEntry {
    pub from: PathBuf,
    pub to: PathBuf,
}

trait Symlink {
    fn resolves_to<P: AsRef<Path>>(&self, to: P) -> io::Result<bool>;
}

impl<T: AsRef<Path>> Symlink for T {
    fn resolves_to<P: AsRef<Path>>(&self, destination: P) -> io::Result<bool> {
        Ok(fs::read_link(self)? == destination.as_ref())
    }
}

#[derive(Debug)]
pub enum Success {
    Linked,
    AlreadyLinked,
}

#[derive(Debug)]
pub enum Error {
    SourceNotFound,
    DestinationExists,
    DestinationDirectoryNotFound,
    Io(io::Error),
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Success::Linked => write!(f, "linked/ok to link"),
            Success::AlreadyLinked => write!(f, "already linked"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SourceNotFound => write!(f, "source doesn't exist"),
            Error::DestinationExists => write!(f, "destination already exists"),
            Error::DestinationDirectoryNotFound => write!(f, "destination directory not found"),
            Error::Io(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

#[cfg(target_family = "unix")]
fn os_symlink(from: &dyn AsRef<Path>, to: &dyn AsRef<Path>) -> io::Result<()> {
    std::os::unix::fs::symlink(from, to)
}

#[cfg(target_family = "windows")]
fn os_symlink(from: &dyn AsRef<Path>, to: &dyn AsRef<Path>) -> io::Result<()> {
    if from.as_ref().is_dir() {
        return std::os::windows::fs::symlink_dir(from, to);
    }

    std::os::windows::fs::symlink_file(from, to)
}

fn symlink_dry(from: &Path, to: &Path) -> Result<Success, Error> {
    if !from.exists() {
        return Err(Error::SourceNotFound);
    }

    if !to.parent().is_some_and(Path::exists) {
        return Err(Error::DestinationDirectoryNotFound);
    }

    if to.exists() {
        if !to.is_symlink() || !to.resolves_to(from)? {
            return Err(Error::DestinationExists);
        }

        return Ok(Success::AlreadyLinked);
    }

    Ok(Success::Linked)
}

fn symlink(from: &Path, to: &Path) -> Result<Success, Error> {
    if !from.exists() {
        return Err(Error::SourceNotFound);
    }

    if !to.parent().is_some_and(Path::exists) {
        return Err(Error::DestinationDirectoryNotFound);
    }

    if let Err(e) = os_symlink(&from, &to) {
        return match e.kind() {
            ErrorKind::AlreadyExists => {
                if !to.is_symlink() || !to.resolves_to(from)? {
                    return Err(Error::DestinationExists);
                }

                return Ok(Success::AlreadyLinked);
            }
            _ => Err(Error::Io(e)),
        };
    }

    Ok(Success::Linked)
}

impl LinkEntry {
    pub fn symlink(&self, dry_run: bool) -> Result<Success, Error> {
        if dry_run {
            symlink_dry(&self.from, &self.to)
        } else {
            symlink(&self.from, &self.to)
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use std::fs::File;

    #[test]
    fn symlink_linked() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;

        assert!(matches!(
            symlink(&dir.path().join("file"), &dir.path().join("file2")),
            Ok(Success::Linked)
        ));

        Ok(())
    }

    #[test]
    fn symlink_already_linked() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;
        os_symlink(&dir.path().join("file"), &dir.path().join("file2"))?;

        assert!(matches!(
            symlink(&dir.path().join("file"), &dir.path().join("file2")),
            Ok(Success::AlreadyLinked)
        ));

        Ok(())
    }

    #[test]
    fn symlink_source_not_found() -> io::Result<()> {
        let dir = tempdir()?;

        assert!(matches!(
            symlink(&dir.path().join("file"), &dir.path().join("file2")),
            Err(Error::SourceNotFound)
        ));

        Ok(())
    }

    #[test]
    fn symlink_destination_exists() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;
        File::create(dir.path().join("file2"))?;

        assert!(matches!(
            symlink(&dir.path().join("file"), &dir.path().join("file2")),
            Err(Error::DestinationExists)
        ));

        Ok(())
    }

    #[test]
    fn test_destination_directory_not_found() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;

        assert!(matches!(
            symlink(
                &dir.path().join("file"),
                &dir.path().join("dir").join("file2")
            ),
            Err(Error::DestinationDirectoryNotFound)
        ));

        Ok(())
    }
}
