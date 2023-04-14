use clap::Parser;
use std::{env, io, path::PathBuf};

mod load;
mod utils;

#[derive(Parser, Debug)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long, default_value = "default")]
    /// Which preset to use
    preset: String,

    #[arg(short, long)]
    list_presets: bool,

    #[arg(short, long)]
    /// Custom config file location
    config_file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Args: {args:?}");

    let pwd = args
        .path
        .map(|path| path.canonicalize())
        .unwrap_or(env::current_dir())?;

    let config_file_path = args.config_file.unwrap_or_else(|| pwd.join("dotlink.toml"));

    let presets = load::Presets::from_file(&config_file_path)?;

    if args.list_presets {
        println!("Available presets: {}", presets.names().join(", "));
        std::process::exit(0);
    }

    let preset = presets.get(&args.preset).unwrap_or_else(|| {
        println!("Invalid preset '{}'", args.preset);
        std::process::exit(0);
    });

    #[cfg(debug_assertions)]
    println!("Loaded preset: {preset:?}");

    preset.apply(pwd)?;

    Ok(())
}
