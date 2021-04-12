use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use shrub_rs::models::commands::EvgCommand;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalSourceDesc {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BonsaiPotSource {
    Local(LocalSourceDesc),
    Github,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotDesc {
    pub name: String,
    pub source: BonsaiPotSource,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPot {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, Vec<EvgCommand>>,
}

impl BonsaiPotDesc {
    pub fn get_module(&self) -> BonsaiPot {
        let contents = read_to_string(&self.path).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }
}
