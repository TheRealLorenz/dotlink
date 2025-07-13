use std::{env, fs, path::PathBuf};

use anyhow::anyhow;
use clap::Parser;

use colored::Colorize;
use context::Context;
use link::Symlinkable;
use parse::Presets;

mod context;
mod expand;
mod link;
mod parse;
mod print;

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

    let pwd = match args.path {
        Some(path) => expand::path(path)?,
        None => env::current_dir()?,
    };

    let ctx = Context {
        pwd: &pwd,
        dry_run: args.dry_run,
    };

    let config_file = args.file.unwrap_or(pwd.join("dotlink.toml"));

    let presets = toml::from_str::<Presets>(&fs::read_to_string(config_file)?)?;

    if args.list {
        if presets.is_empty() {
            return Err(anyhow!("No available presets"));
        }
        println!("Availabile presets:");
        for value in presets.keys() {
            println!("  - {value}");
        }

        return Ok(());
    }

    let preset = presets
        .get(&args.preset)
        .ok_or(anyhow!("Preset not found '{}'", args.preset))?;

    if args.dry_run {
        println!("{}", "Performing dry-run".bold());
    }

    println!("Loading preset '{}':", args.preset);
    for item in preset {
        item.apply(&ctx)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");

        #[cfg(debug_assertions)]
        eprintln!("Debug Error: {e:#?}");

        std::process::exit(1);
    }
}
