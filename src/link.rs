use anyhow::anyhow;
use std::{fs, io, path::Path};

use crate::context::Context;

fn resolves_to<P: AsRef<Path>>(source: P, destination: P) -> io::Result<bool> {
    Ok(fs::read_link(source)? == destination.as_ref())
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

pub fn symlink<T>(relative_from: T, to: &Path, ctx: &Context) -> anyhow::Result<()>
where
    T: AsRef<Path>,
{
    let from = ctx.pwd.join(relative_from);

    if !from.exists() {
        return Err(anyhow!("Source not found"));
    }

    if !to.parent().is_some_and(Path::exists) {
        return Err(anyhow!("Destination directory not found"));
    }

    if to.exists() {
        if to.is_symlink() && resolves_to(to, &from)? {
            return Ok(());
        }
        return Err(anyhow!("Destination exists"));
    }

    if !ctx.dry_run {
        os_symlink(&from, &to)?;
    }

    Ok(())
}

pub trait Symlinkable {
    fn apply(&self, ctx: &Context) -> anyhow::Result<()>;
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

        let result = symlink("file", &dir.path().join("file2"), &Context::new(dir.path()));
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn symlink_already_linked() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;
        os_symlink(&dir.path().join("file"), &dir.path().join("file2"))?;

        let result = symlink("file", &dir.path().join("file2"), &Context::new(dir.path()));
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn symlink_other_link() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;
        File::create(dir.path().join("file2"))?;
        os_symlink(&dir.path().join("file"), &dir.path().join("file1"))?;

        let result = symlink(
            "file2",
            &dir.path().join("file1"),
            &Context::new(dir.path()),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Destination exists");

        Ok(())
    }

    #[test]
    fn symlink_source_not_found() -> io::Result<()> {
        let dir = tempdir()?;

        let result = symlink("file", &dir.path().join("file2"), &Context::new(dir.path()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Source not found");

        Ok(())
    }

    #[test]
    fn symlink_destination_exists() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;
        File::create(dir.path().join("file2"))?;

        let result = symlink("file", &dir.path().join("file2"), &Context::new(dir.path()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Destination exists");

        Ok(())
    }

    #[test]
    fn test_destination_directory_not_found() -> io::Result<()> {
        let dir = tempdir()?;
        File::create(dir.path().join("file"))?;

        let result = symlink(
            "file",
            &dir.path().join("dir").join("file2"),
            &Context::new(dir.path()),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Destination directory not found"
        );

        Ok(())
    }
}
