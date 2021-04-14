use crate::pot::remote_pot::get_remote_pots;
use serde::{Deserialize, Serialize};
use shrub_rs::models::commands::EvgCommand;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use crate::landscape::command::BonsaiCommand;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalSourceDesc {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GithubVersionSpec {
    Revision(String),
    Branch(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubSourceDesc {
    pub owner: String,
    pub repo: String,
    #[serde(flatten)]
    pub version: Option<GithubVersionSpec>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase", tag = "source")]
pub enum BonsaiPotSource {
    Local(LocalSourceDesc),
    Github(GithubSourceDesc),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BonsaiPotDesc {
    #[serde(flatten)]
    pub source: BonsaiPotSource,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BonsaiPotParam {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BonsaiPotFunction {
    pub description: String,
    pub actions: Vec<BonsaiCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<BonsaiPotParam>>,
}

impl BonsaiPotFunction {
    pub fn try_translate(&self, fn_map: &HashMap<String, Vec<EvgCommand>>) -> Option<Vec<EvgCommand>> {
        let mut command_list = vec![];
        for cmd in &self.actions {
            match cmd {
                BonsaiCommand::EvergreenNative(evg_cmd) => {
                    command_list.push(evg_cmd.clone());
                }
                BonsaiCommand::Bonsai(bonsai_call) => {
                    if let Some(fn_cmds) = fn_map.get(&bonsai_call.get_fn_name()) {
                        command_list.extend(fn_cmds.to_vec());
                    } else {
                        return None
                    }
                }
            }
        }

        Some(command_list)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BonsaiPot {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonsai: Option<Vec<BonsaiPotDesc>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, BonsaiPotFunction>,
}

impl BonsaiPot {
    pub fn from_path(path: &Path) -> Result<BonsaiPot, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        Ok(serde_yaml::from_str(&contents)?)
    }

    fn get_fn_name(&self, base_name: &str) -> String {
        format!("{}_{}", self.name, base_name)
    }

    pub fn update_fn_map(&self, fn_map: &mut HashMap<String, Vec<EvgCommand>>) -> bool {
        let mut n_updates = 0;
        for (fn_name, fn_def) in &self.functions {
            if !fn_map.contains_key(&self.get_fn_name(fn_name)) {
                let maybe_fn_cmds = fn_def.try_translate(fn_map);
                if let Some(fn_cmds) = maybe_fn_cmds {
                    n_updates += 1;
                    fn_map.insert(self.get_fn_name(fn_name), fn_cmds);
                }
            }
        }

        n_updates != 0
    }
}

impl BonsaiPotDesc {
    pub fn get_pots(&self) -> Result<Vec<BonsaiPot>, Box<dyn Error>> {
        match &self.source {
            BonsaiPotSource::Local(local_source) => {
                Ok(vec![BonsaiPot::from_path(Path::new(&local_source.path))?])
            }
            BonsaiPotSource::Github(github) => get_remote_pots(github),
        }
    }

    pub fn update_pot_map(&self, pot_map: &mut HashMap<String, BonsaiPot>) -> Result<(), Box<dyn Error>> {
        let pots = self.get_pots()?;
        for pot in pots {
            if !pot_map.contains_key(&pot.name) {
                pot_map.insert(pot.name.clone(), pot.clone());
                if let Some(child_pots) = &pot.bonsai {
                    child_pots.iter().try_for_each(|p| p.update_pot_map(pot_map))?;
                }
            }
        }

        Ok(())
    }
}
