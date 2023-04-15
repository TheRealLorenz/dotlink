use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::utils;

#[derive(Deserialize)]
pub struct Presets {
    presets: HashMap<String, Preset>,
}

#[derive(Deserialize, Debug)]
pub struct Preset {
    link: Vec<Entry>,
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
    pub fn from_file(path: &dyn AsRef<Path>) -> io::Result<Self> {
        let file_content = fs::read_to_string(path)?;
        let presets = toml::from_str::<Presets>(&file_content).unwrap_or_else(|err| {
            panic!("Couldn't parse toml: {err}");
        });

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
    pub fn apply(&self, from_dir: PathBuf) -> io::Result<()> {
        for entry in &self.link {
            match entry {
                Entry::SimpleEntry(name) => utils::symlink(&from_dir, name, &self.to),
                Entry::CustomEntry(a) => utils::symlink(&from_dir, &a.name, &a.to),
            }?;
        }

        Ok(())
    }
}
