use std::path::PathBuf;
use colored::*;

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
            println!("\t{e}")
            println!("{}", "X".red().bold());
        }
    };
}
