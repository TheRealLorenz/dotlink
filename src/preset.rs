use crate::{expand, link};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

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
    pub fn apply(&self, from_dir: &PathBuf, dry_run: bool) -> Result<(), link::LinkError> {
        for entry in &self.links {
            match entry {
                Entry::SimpleEntry(name) => {
                    let from = PathBuf::from(&from_dir).join(name);
                    let to = expand::expand_tilde(Path::new(&self.to))?.join(name);

                    link::symlink(&from, &to, dry_run)?;
                }
                Entry::CustomEntry(a) => {
                    let from = PathBuf::from(&from_dir).join(&a.name);
                    let to = expand::expand_tilde(Path::new(&a.to))?;

                    link::symlink(&from, &to, dry_run)?;
                }
            };
        }

        Ok(())
    }
}
