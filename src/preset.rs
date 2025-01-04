use anyhow::anyhow;
use colored::{Color, Colorize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tabled::{builder::Builder, settings::Style};

use crate::link;

mod entries;

pub type Entry = entries::Single;
pub struct Preset(Vec<Entry>);
pub struct Presets(HashMap<String, Preset>);

const DEFAULT_CONFIG_FILES: &[&str; 3] = &["dotlink.toml", "dotlink.yaml", "dotlink.json"];

impl Preset {
    pub fn apply(&self, from_dir: &Path, dry_run: bool) -> anyhow::Result<()> {
        let mut builder = Builder::default();
        builder.set_header([
            "Name".underline().to_string(),
            "Destination".underline().to_string(),
            "Result".underline().to_string(),
        ]);

        let link_entries = self
            .0
            .iter()
            .map(|entry| link::LinkEntry::new(entry, from_dir))
            .collect::<anyhow::Result<Vec<_>>>()?;

        for link_entry in link_entries {
            let from = link_entry
                .from
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let to = link_entry.to.display().to_string();

            let result = {
                let (icon, color, message) = match link_entry.symlink(dry_run) {
                    Ok(s) => ("", Color::Green, s.to_string()),
                    Err(s) => ("", Color::Red, s.to_string()),
                };

                format!("{} {}", icon.color(color), message.color(color))
            };

            builder.push_record([from, to, result]);
        }

        println!("\n{}", builder.build().with(Style::blank()));

        Ok(())
    }
}

impl From<HashMap<String, Vec<entries::Raw>>> for Presets {
    fn from(value: HashMap<String, Vec<entries::Raw>>) -> Self {
        let x = value
            .into_iter()
            .map(|(k, v)| {
                let v = v
                    .into_iter()
                    .flat_map(|raw_entry| match raw_entry {
                        entries::Raw::Single(s) => vec![s],
                        entries::Raw::Multiple(m) => m.flatten(),
                    })
                    .collect();

                (k, Preset(v))
            })
            .collect();

        Presets(x)
    }
}

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

        let file_content = fs::read_to_string(&path)?;

        let raw_presets = match extension {
            "toml" => toml::from_str::<entries::RawPresets>(&file_content)?,
            "yaml" | "yml" => serde_yaml::from_str::<entries::RawPresets>(&file_content)?,
            "json" => serde_json::from_str::<entries::RawPresets>(&file_content)?,
            _ => return Err(anyhow!("Invalid config file extension")),
        };

        Ok(raw_presets.into())
    }

    pub fn names(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect::<Vec<&str>>()
    }

    pub fn get(&self, preset_name: &str) -> Option<&Preset> {
        self.0.get(preset_name)
    }
}
