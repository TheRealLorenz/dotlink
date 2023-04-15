use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

use crate::utils;

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
    pub fn from_file(path: &PathBuf) -> Self {
        let file_content = fs::read_to_string(path).unwrap_or_else(|error| {
            panic!("Couldn't read file '{:?}': {error}", path);
        });

        toml::from_str::<Presets>(&file_content).unwrap_or_else(|err| {
            panic!("Couldn't parse toml: {err}");
        })
    }

    pub fn names(&self) -> Vec<String> {
        self.presets.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get(&self, preset_name: &String) -> Option<&Preset> {
        self.presets.get(preset_name)
    }
}

impl Preset {
    pub fn apply(&self, from_dir: PathBuf, dry_run: bool) {
        for entry in &self.links {
            match entry {
                Entry::SimpleEntry(name) => utils::symlink(&from_dir, name, &self.to, dry_run),
                Entry::CustomEntry(a) => utils::symlink(&from_dir, &a.name, &a.to, dry_run),
            };
        }
    }
}
