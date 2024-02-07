use std::path::Path;

use serde::Deserialize;

use crate::{expand, link::LinkEntry};

#[derive(Deserialize, Debug, Clone)]
pub struct Single {
    name: String,
    to: String,
    rename: Option<String>,
}

impl Single {
    pub fn to_link_entry(&self, from_dir: &Path) -> anyhow::Result<LinkEntry> {
        let from = from_dir.join(&self.name);
        let to = expand::tilde(&self.to)?.join(self.rename.as_ref().unwrap_or(&self.name));
        Ok(LinkEntry { from, to })
    }
}

#[derive(Deserialize, Debug)]
pub struct Multiple {
    names: Vec<String>,
    to: String,
}

impl Multiple {
    pub fn flatten(self) -> Vec<Single> {
        self.names
            .into_iter()
            .map(|name| Single {
                name,
                to: self.to.clone(),
                rename: None,
            })
            .collect()
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Raw {
    Single(Single),
    Multiple(Multiple),
}
