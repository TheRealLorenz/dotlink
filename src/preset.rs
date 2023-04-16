use crate::{expand, link};
use colored::*;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

pub mod error;

#[derive(Deserialize)]
pub struct Presets {
    presets: HashMap<String, Preset>,
}

#[derive(Deserialize, Debug)]
pub struct Preset {
    links: Vec<Entry>,
    to: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Entry {
    SimpleEntry(SimpleEntry),
    CustomEntry(CustomEntry),
}

type SimpleEntry = String;

#[derive(Deserialize, Debug)]
struct CustomEntry {
    name: String,
    to: String,
}

impl Entry {
    fn to_link_entry(
        &self,
        from_dir: &Path,
        default_to: &String,
    ) -> Result<link::LinkEntry, expand::ExpandError> {
        match self {
            Entry::SimpleEntry(name) => {
                let from = from_dir.join(name);
                let to = expand::expand_tilde(Path::new(default_to))?.join(name);

                Ok(link::LinkEntry { from, to })
            }
            Entry::CustomEntry(a) => {
                let from = from_dir.join(&a.name);
                let to = expand::expand_tilde(Path::new(&a.to))?;

                Ok(link::LinkEntry { from, to })
            }
        }
    }
}

impl Presets {
    pub fn from_file(path: &dyn AsRef<Path>) -> Result<Self, error::LoadError> {
        let file_content = fs::read_to_string(path)?;
        let presets = toml::from_str::<Presets>(&file_content)?;

        Ok(presets)
    }

    pub fn names(&self) -> Vec<String> {
        self.presets.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get(&self, preset_name: &String) -> Option<&Preset> {
        self.presets.get(preset_name)
    }
}

impl Preset {
    pub fn apply(&self, from_dir: &Path, dry_run: bool) -> Result<(), expand::ExpandError> {
        for entry in &self.links {
            let link_entry = entry.to_link_entry(from_dir, &self.to)?;

            print!("Linking {}: ", link_entry);

            if dry_run {
                println!("{}", "dry".yellow().bold());
                continue;
            }

            match link::symlink(&link_entry) {
                Ok(_) => println!("{}", "✓".green().bold()),
                Err(e) => {
                    println!("{}", "X".red().bold());
                    eprintln!("  - {e}")
                }
            }
        }

        Ok(())
    }
}
