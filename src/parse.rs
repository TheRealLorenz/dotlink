use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{expand, link, print::print_result};

#[derive(Debug, Deserialize)]
pub struct SingleEntry {
    name: String,
    to: String,
    rename: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MultipleEntry {
    names: Vec<String>,
    to: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    Single(SingleEntry),
    Multiple(MultipleEntry),
}

pub type Presets = HashMap<String, Vec<Entry>>;

pub trait Symlinkable {
    fn apply(&self, src_dir: &Path, dry_run: bool) -> anyhow::Result<()>;
}

impl Symlinkable for SingleEntry {
    fn apply(&self, src_dir: &Path, dry_run: bool) -> anyhow::Result<()> {
        let src = src_dir.join(&self.name);

        let dst_dir = expand::path(PathBuf::from(&self.to))?;
        let dst = dst_dir.join(self.rename.as_ref().unwrap_or(&self.name));

        let result = link::symlink(&src, &dst, dry_run);
        print_result(&self.name, &dst, result);
        Ok(())
    }
}

impl Symlinkable for MultipleEntry {
    fn apply(&self, src_dir: &Path, dry_run: bool) -> anyhow::Result<()> {
        let dst_dir = expand::path(PathBuf::from(&self.to))?;

        for name in &self.names {
            let src = src_dir.join(name);
            let dst = dst_dir.join(name);

            let result = link::symlink(&src, &dst, dry_run);
            print_result(name, &dst, result);
        }

        Ok(())
    }
}

impl Symlinkable for Entry {
    fn apply(&self, src_dir: &Path, dry_run: bool) -> anyhow::Result<()> {
        match self {
            Entry::Single(single_entry) => single_entry.apply(src_dir, dry_run),
            Entry::Multiple(multiple_entry) => multiple_entry.apply(src_dir, dry_run),
        }
    }
}
