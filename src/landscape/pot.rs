use crate::pot::remote_pot::get_remote_pot;
use serde::{Deserialize, Serialize};
use shrub_rs::models::commands::EvgCommand;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalSourceDesc {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubSourceDesc {
    pub owner: String,
    pub repo: String,
    pub revision: Option<String>,
    pub pot_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase", tag = "source")]
pub enum BonsaiPotSource {
    Local(LocalSourceDesc),
    Github(GithubSourceDesc),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotDesc {
    pub name: String,
    #[serde(flatten)]
    pub source: BonsaiPotSource,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotParam {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotFunction {
    pub description: String,
    pub actions: Vec<EvgCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<BonsaiPotParam>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPot {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, BonsaiPotFunction>,
}

impl BonsaiPot {
    pub fn from_path(path: &Path) -> Result<BonsaiPot, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}

impl BonsaiPotDesc {
    pub fn get_module(&self) -> BonsaiPot {
        match &self.source {
            BonsaiPotSource::Local(local_source) => {
                BonsaiPot::from_path(Path::new(&local_source.path)).unwrap()
            }
            BonsaiPotSource::Github(github) => get_remote_pot(github).unwrap(),
        }
    }
}
