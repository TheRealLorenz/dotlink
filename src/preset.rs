use crate::{expand, link::LinkEntry};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};
use tabled::builder::Builder;

pub mod error;

#[derive(Deserialize, Debug, Clone)]
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

impl MultipleEntry {
    fn flatten(&self) -> Vec<SingleEntry> {
        self.names
            .iter()
            .map(|name| SingleEntry {
                name: name.clone(),
                to: self.to.clone(),
                rename: None,
            })
            .collect()
    }
}

#[derive(Deserialize)]
pub struct Preset(Vec<Entry>);

impl Preset {
    pub fn apply(&self, from_dir: &Path, dry_run: bool) -> Result<(), expand::ExpandError> {
        let mut builder = Builder::default();
        builder.set_header(["Name", "Destination", "Result"]);

        self.0
            .iter()
            .flat_map(|entry| match entry {
                Entry::Single(single_entry) => vec![single_entry.clone()],
                Entry::Multiple(multiple_entry) => multiple_entry.flatten(),
            })
            .map(|single_entry| {
                let from = from_dir.join(&single_entry.name);
                let to = expand::expand_tilde(
                    &Path::new(&single_entry.to)
                        .join(single_entry.rename.unwrap_or(single_entry.name)),
                )?;
                Ok(LinkEntry { from, to })
            })
            .try_for_each(
                |link_entry: Result<_, expand::ExpandError>| -> Result<(), expand::ExpandError> {
                    let link_entry = link_entry?;
                    builder.push_record([
                        link_entry
                            .from
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        link_entry.to.display().to_string(),
                        link_entry.symlink(dry_run).to_string(),
                    ]);

                    Ok(())
                },
            )?;

        println!("\n{}", builder.build());

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
            "yaml" | "yml" => Self::from_yaml(&path),
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
