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
}

impl<T: AsRef<Path>> Symlink for T {
    fn resolves_to(&self, destination: &Path) -> io::Result<bool> {
        Ok(fs::read_link(self)? == destination)
    }
}

#[derive(Debug)]
enum LinkSuccess {
    Linked,
    AlreadyLinked,
}

#[derive(Debug)]
enum LinkError {
    SourceNotFound,
    DestinationExists,
    DestinationDirectoryNotFound,
    Io(io::Error),
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkError::SourceNotFound => write!(f, "source doesn't exist"),
            LinkError::DestinationExists => write!(f, "destination already exists"),
            LinkError::DestinationDirectoryNotFound => write!(f, "destination directory not found"),
            LinkError::Io(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for LinkError {
    fn from(value: io::Error) -> Self {
        LinkError::Io(value)
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

fn symlink(from: &Path, to: &Path) -> Result<LinkSuccess, LinkError> {
    if !from.exists() {
        return Err(LinkError::SourceNotFound);
    }

    if !to.parent().map(|parent| parent.exists()).unwrap_or(false) {
        return Err(LinkError::DestinationDirectoryNotFound);
    }

    if let Err(e) = os_symlink(&from, &to) {
        return match e.kind() {
            ErrorKind::AlreadyExists => {
                if !to.is_symlink() || !to.resolves_to(from)? {
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
            Ok(LinkSuccess::Linked)
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
            Ok(LinkSuccess::AlreadyLinked)
        ));

        Ok(())
    }

    #[test]
    fn symlink_source_not_found() -> io::Result<()> {
        let dir = tempdir()?;

        assert!(matches!(
            symlink(&dir.path().join("file"), &dir.path().join("file2")),
            Err(LinkError::SourceNotFound)
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
            Err(LinkError::DestinationExists)
        ));

        Ok(())
    }

    #[test]
    fn test_destination_directory_not_found() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;

        assert!(matches!(
            symlink(&dir.path().join("file"), &dir.path().join("dir").join("file2")),
            Err(LinkError::DestinationDirectoryNotFound)
        ));

        Ok(())
    }
}
