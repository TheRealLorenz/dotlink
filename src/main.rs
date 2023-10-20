use clap::Parser;
use colored::*;
use std::{env, fmt, io, path::PathBuf};

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

#[derive(Debug)]
enum CliError {
    Load(preset::error::LoadError),
    Io(io::Error),
    Expand(expand::ExpandError),
}

impl From<preset::error::LoadError> for CliError {
    fn from(value: preset::error::LoadError) -> Self {
        CliError::Load(value)
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::Io(value)
    }
}

impl From<expand::ExpandError> for CliError {
    fn from(value: expand::ExpandError) -> Self {
        CliError::Expand(value)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Load(e) => write!(f, "{e}"),
            CliError::Io(e) => write!(f, "{e}"),
            CliError::Expand(e) => write!(f, "{e}"),
        }
    }
}

fn try_main() -> Result<(), CliError> {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Args: {args:?}");

    let pwd = args
        .path
        .map(|path| expand::expand_path(&path))
        .unwrap_or(env::current_dir().map_err(expand::ExpandError::Io))?;

    let presets = if let Some(file_path) = args.file {
        preset::Presets::from_path(&expand::expand_path(&file_path)?)
    } else {
        preset::Presets::from_path(&pwd)
    }?;

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
