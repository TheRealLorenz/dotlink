use crate::{expand, link::LinkEntry};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

pub mod error;

#[derive(Deserialize, Debug)]
struct SingleEntry {
    name: String,
    to: String,
    rename: Option<String>,
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
                    let to = expand::expand_tilde(
                        &Path::new(&single_entry.to).join(
                            single_entry
                                .rename
                                .clone()
                                .unwrap_or(single_entry.name.clone()),
                            // Maybe .clone() could be removed
                        ),
                    )?;

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
    fn get_config_file_name(path: &dyn AsRef<Path>) -> Option<PathBuf> {
        path.as_ref()
            .read_dir()
            .ok()?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    return Some(entry.path());
                }
                None
            })
            .find(|entry| {
                if let Some(file_name) = entry.file_name() {
                    return ["dotlink.toml", "dotlink.yaml", "dotlink.json"]
                        .contains(&file_name.to_str().unwrap());
                }

                false
            })
    }

    pub fn from_path(path: &dyn AsRef<Path>) -> Result<Self, error::LoadError> {
        let path = if path.as_ref().is_dir() {
            Self::get_config_file_name(path).ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "config file not found",
            ))?
        } else {
            PathBuf::from(path.as_ref())
        };

        let extension = path
            .extension()
            .ok_or(error::LoadError::InvalidExtension)?
            .to_str()
            .unwrap();

        match extension {
            "toml" => Self::from_toml(&path),
            "yaml" => Self::from_yaml(&path),
            "json" => Self::from_json(&path),
            _ => Err(error::LoadError::InvalidExtension),
        }
    }

    fn from_toml(path: &dyn AsRef<Path>) -> Result<Self, error::LoadError> {
        let file_content = fs::read_to_string(path)?;
        let presets = toml::from_str::<Presets>(&file_content)?;

        Ok(presets)
    }

    fn from_json(path: &dyn AsRef<Path>) -> Result<Self, error::LoadError> {
        let file_content = fs::read_to_string(path)?;
        let presets = serde_json::from_str::<Presets>(&file_content)?;

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
