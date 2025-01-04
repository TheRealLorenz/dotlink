use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Single {
    pub name: String,
    pub to: String,
    pub rename: Option<String>,
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

pub type RawPresets = HashMap<String, Vec<Raw>>;
