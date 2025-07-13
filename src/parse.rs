use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::{
    context::Context,
    expand,
    link::{self, Symlinkable},
    print::print_result,
};

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

impl Symlinkable for SingleEntry {
    fn apply(&self, ctx: &Context) -> anyhow::Result<()> {
        let src = ctx.pwd.join(&self.name);

        let dst_dir = expand::path(PathBuf::from(&self.to))?;
        let dst = dst_dir.join(self.rename.as_ref().unwrap_or(&self.name));

        let result = link::symlink(&src, &dst, ctx.dry_run);
        print_result(&self.name, &dst, result);
        Ok(())
    }
}

impl Symlinkable for MultipleEntry {
    fn apply(&self, ctx: &Context) -> anyhow::Result<()> {
        let dst_dir = expand::path(PathBuf::from(&self.to))?;

        for name in &self.names {
            let src = ctx.pwd.join(name);
            let dst = dst_dir.join(name);

            let result = link::symlink(&src, &dst, ctx.dry_run);
            print_result(name, &dst, result);
        }

        Ok(())
    }
}

impl Symlinkable for Entry {
    fn apply(&self, ctx: &Context) -> anyhow::Result<()> {
        match self {
            Entry::Single(single_entry) => single_entry.apply(ctx),
            Entry::Multiple(multiple_entry) => multiple_entry.apply(ctx),
        }
    }
}
