use anyhow::anyhow;
use clap::Parser;
use colored::*;
use std::{env, path::PathBuf};

mod expand;
mod link;
mod preset;

#[derive(Parser, Debug)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long, default_value = "default")]
    /// Which preset to use
    preset: String,

    #[arg(short, long)]
    /// List available presets
    list: bool,

    #[arg(short = 'F', long)]
    /// Custom config file location
    file: Option<PathBuf>,

    #[arg(long)]
    /// Run dotlink in dry-run mode
    dry_run: bool,
}

fn try_main() -> anyhow::Result<()> {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Args: {args:?}");

    let pwd = args
        .path
        .map(|path| expand::expand_path(&path))
        .unwrap_or(env::current_dir().map_err(|e| anyhow!(e)))?;

    let config_file_path = args.file.as_ref().unwrap_or(&pwd);
    let presets = preset::Presets::from_config(config_file_path)?;

    if args.list {
        println!("Available presets: {}", presets.names().join(", "));
        std::process::exit(0);
    }

    let preset = presets.get(&args.preset).unwrap_or_else(|| {
        println!("Invalid preset '{}'", args.preset);
        std::process::exit(1);
    });

    println!("Loaded preset: {}", args.preset.bold());

    if args.dry_run {
        println!("Running in {} mode", "dry-run".yellow().bold());
    }

    preset.apply(&pwd, args.dry_run)?;

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");

        #[cfg(debug_assertions)]
        eprintln!("Error: {e:#?}");

        std::process::exit(1);
    }
}
