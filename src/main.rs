use clap::Parser;
use std::{env, path::PathBuf};

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
    dry_run: bool,
}

fn main() {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Args: {args:?}");

    let pwd = args
        .path
        .map(|path| utils::expand_path(&path))
        .unwrap_or(env::current_dir().expect("Invalid current directory"));

    let config_file_path = args
        .file
        .map(|path| utils::expand_path(&path))
        .unwrap_or_else(|| pwd.join("dotlink.toml"));

    let presets = preset::Presets::from_file(&config_file_path);

    if args.list_presets {
        println!("Available presets: {}", presets.names().join(", "));
        std::process::exit(0);
    }

    let preset = presets.get(&args.preset).unwrap_or_else(|| {
        println!("Invalid preset '{}'", args.preset);
        std::process::exit(0);
    });

    println!("Loaded preset: {}", args.preset);

    if args.dry_run {
        println!("Running in dry-run mode");
    }

    preset.apply(pwd, args.dry_run);
}
