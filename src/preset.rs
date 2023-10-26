use crate::{expand, link::LinkEntry};
use anyhow::anyhow;
use colored::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tabled::{builder::Builder, settings::Style};

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

const DEFAULT_CONFIG_FILES: &[&str; 3] = &["dotlink.toml", "dotlink.yaml", "dotlink.json"];

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
    pub fn apply(&self, from_dir: &Path, dry_run: bool) -> anyhow::Result<()> {
        let mut builder = Builder::default();
        builder.set_header([
            "Name".underline().to_string(),
            "Destination".underline().to_string(),
            "Result".underline().to_string(),
        ]);

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
            .try_for_each(|link_entry: anyhow::Result<_>| -> anyhow::Result<()> {
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
            })?;

        println!("\n{}", builder.build().with(Style::blank()));

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Presets(HashMap<String, Preset>);

impl Presets {
    fn get_config_file_name(path: &Path) -> Option<PathBuf> {
        path.read_dir()
            .ok()?
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry.path()),
                Err(_) => None,
            })
            .find(|entry| match entry.file_name() {
                Some(file_name) => DEFAULT_CONFIG_FILES.contains(&file_name.to_str().unwrap()),
                None => false,
            })
    }

    pub fn from_config(config_path: &Path) -> anyhow::Result<Self> {
        let path = if config_path.is_dir() {
            Self::get_config_file_name(config_path).ok_or(anyhow!("config file not found"))?
        } else {
            config_path.into()
        };

        let extension = path
            .extension()
            .ok_or(anyhow!("Invalid config file extension"))?
            .to_str()
            .unwrap();

        match extension {
            "toml" => Self::from_toml(&path),
            "yaml" | "yml" => Self::from_yaml(&path),
            "json" => Self::from_json(&path),
            _ => Err(anyhow!("Invalid config file extension")),
        }
    }

    fn from_toml(path: &dyn AsRef<Path>) -> anyhow::Result<Self> {
        let file_content = fs::read_to_string(path)?;
        let presets = toml::from_str::<Presets>(&file_content)?;

        Ok(presets)
    }

    fn from_json(path: &dyn AsRef<Path>) -> anyhow::Result<Self> {
        let file_content = fs::read_to_string(path)?;
        let presets = serde_json::from_str::<Presets>(&file_content)?;

        Ok(presets)
    }

    fn from_yaml(path: &dyn AsRef<Path>) -> anyhow::Result<Self> {
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
