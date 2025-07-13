use std::path::Path;

use colored::Colorize;

pub fn print_result(src: &str, dst: &Path, result: anyhow::Result<()>) {
    let (icon, message) = match result {
        Ok(_) => ("✓".green(), None),
        Err(e) => ("✘".red(), Some(e.to_string())),
    };
    print!("\t{icon} {} → {}", src.bold(), dst.display());
    if let Some(message) = message {
        print!(": {message}");
    }
    println!()
}
