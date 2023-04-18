use crate::{expand, link::LinkEntry};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

pub mod error;

#[derive(Deserialize, Debug)]
struct SingleEntry {
    name: String,
    to: String,
}

#[derive(Deserialize, Debug)]
struct MultipleEntry {
    names: Vec<String>,
    to: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Entry {
    Single(SingleEntry),
    Multiple(MultipleEntry),
}

#[derive(Deserialize)]
pub struct Preset(Vec<Entry>);

impl Preset {
    pub fn apply(&self, from_dir: &Path, dry_run: bool) -> Result<(), expand::ExpandError> {
        for entry in &self.0 {
            match entry {
                Entry::Single(single_entry) => {
                    let from = from_dir.join(&single_entry.name);
                    let to = expand::expand_tilde(Path::new(&single_entry.to))?;

                    LinkEntry { from, to }.symlink(dry_run);
                }
                Entry::Multiple(multiple_entry) => {
                    for name in &multiple_entry.names {
                        let from = from_dir.join(name);
                        let to = expand::expand_tilde(Path::new(&multiple_entry.to))?.join(name);

                        LinkEntry { from, to }.symlink(dry_run);
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Presets(HashMap<String, Preset>);

impl Presets {
    pub fn from_file(path: &dyn AsRef<Path>) -> Result<Self, error::LoadError> {
        let extension = path
            .as_ref()
            .extension()
            .ok_or(error::LoadError::InvalidExtension)?
            .to_str()
            .unwrap();

        match extension {
            "toml" => Self::from_toml(path),
            "yaml" => Self::from_yaml(path),
            _ => Err(error::LoadError::InvalidExtension),
        }
    }

    fn from_toml(path: &dyn AsRef<Path>) -> Result<Self, error::LoadError> {
        let file_content = fs::read_to_string(path)?;
        let presets = toml::from_str::<Presets>(&file_content)?;

        Ok(presets)
    }

    fn from_yaml(path: &dyn AsRef<Path>) -> Result<Self, error::LoadError> {
        let file_content = fs::read_to_string(path)?;
        let presets = serde_yaml::from_str::<Presets>(&file_content)?;

        Ok(presets)
    }

    pub fn names(&self) -> Vec<String> {
        self.0.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get(&self, preset_name: &String) -> Option<&Preset> {
        self.0.get(preset_name)
    }
}
