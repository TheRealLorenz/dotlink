use clap::Parser;
use std::{env, io, path::PathBuf};

mod preset;
mod utils;

#[derive(Parser, Debug)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long, default_value = "default")]
    /// Which preset to use
    preset: String,

    #[arg(short, long)]
    list_presets: bool,

    #[arg(short = 'F', long)]
    /// Custom config file location
    file: Option<PathBuf>,

    #[arg(long)]
    dry_run: bool
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Args: {args:?}");

    let pwd = args
        .path
        .map(|path| utils::expand_tilde(&path).canonicalize())
        .unwrap_or(env::current_dir())?;

    let config_file_path = args
        .file
        .map(|path| utils::expand_tilde(&path).canonicalize())
        .unwrap_or_else(|| Ok(pwd.join("dotlink.toml")))?;

    let presets = preset::Presets::from_file(&config_file_path)?;

    if args.list_presets {
        println!("Available presets: {}", presets.names().join(", "));
        std::process::exit(0);
    }

    let preset = presets.get(&args.preset).unwrap_or_else(|| {
        println!("Invalid preset '{}'", args.preset);
        std::process::exit(0);
    });

    println!("Loaded preset: {}", args.preset);
    preset.apply(pwd, args.dry_run)?;

    Ok(())
}
