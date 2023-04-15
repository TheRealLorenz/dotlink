use clap::Parser;
use std::{env, fmt, io, path::PathBuf};

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

#[derive(Debug)]
enum CliError {
    LoadError(preset::error::LoadError),
    IoError(io::Error),
}

impl From<preset::error::LoadError> for CliError {
    fn from(value: preset::error::LoadError) -> Self {
        CliError::LoadError(value)
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::LoadError(e) => write!(f, "{e}"),
            CliError::IoError(e) => write!(f, "{e}"),
        }
    }
}

fn try_main() -> Result<(), CliError> {
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

    if args.dry_run {
        println!("Running in dry-run mode");
    }

    preset.apply(pwd, args.dry_run)?;

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");

        #[cfg(debug_assertions)]
        println!("Error: {e:#?}");

        std::process::exit(1);
    }
}
