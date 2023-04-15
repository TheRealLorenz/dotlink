use colored::*;
use std::{fs, path::PathBuf};

pub fn symlink(from: &PathBuf, to: &PathBuf, dry_run: bool) {
    print!(
        "Linking '{}' to '{}': ",
        from.as_path().display(),
        to.as_path().display()
    );

    if dry_run {
        println!("dry");
        return;
    }

    match std::os::unix::fs::symlink(from, to) {
        Ok(_) => println!("{}", "✓".green().bold()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists
                && fs::symlink_metadata(to).unwrap().is_symlink()
                && &fs::read_link(to).unwrap() == from
            {
                println!("{}", "✓".green().bold());
                return;
            }

            println!("{}", "X".red().bold());
            eprintln!("  - {e}")
        }
    };
}
