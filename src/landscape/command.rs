use serde::{Deserialize, Serialize};
use shrub_rs::models::commands::FunctionCall;
use shrub_rs::models::{commands::EvgCommand, params::ParamValue};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BonsaiCall {
    pub bonsai: String,
    pub params: Option<HashMap<String, ParamValue>>,
}

impl BonsaiCall {
    pub fn get_fn_name(&self) -> String {
        let parts: Vec<&str> = self.bonsai.split(':').collect();
        format!("{}_{}", parts[0], parts[1])
    }

    pub fn get_pot_name(&self) -> String {
        let parts: Vec<&str> = self.bonsai.split(':').collect();
        parts[0].to_string()
    }

    pub fn to_evg_command(&self) -> EvgCommand {
        let parameters = match &self.params {
            Some(vars) => Some(vars.clone()),
            None => None,
        };
        EvgCommand::Function(FunctionCall {
            func: self.get_fn_name(),
            vars: parameters,
            timeout_secs: None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BonsaiCommand {
    EvergreenNative(EvgCommand),
    Bonsai(BonsaiCall),
}

pub struct BonsaiTranslator {
    pub seen_pots: HashSet<String>,
}

impl BonsaiTranslator {
    pub fn new() -> Self {
        Self {
            seen_pots: HashSet::new(),
        }
    }

    pub fn translate_command(&mut self, bonsai_command: &BonsaiCommand) -> EvgCommand {
        match bonsai_command {
            BonsaiCommand::Bonsai(b_cmd) => {
                self.seen_pots.insert(b_cmd.get_pot_name());
                b_cmd.to_evg_command()
            }
            BonsaiCommand::EvergreenNative(c) => c.clone(),
        }
    }

    pub fn translate_command_list(
        &mut self,
        bonsai_command_list: &[BonsaiCommand],
    ) -> Vec<EvgCommand> {
        bonsai_command_list
            .iter()
            .map(|c| self.translate_command(c))
            .collect()
    }

    pub fn is_used(&self, fn_name: &str) -> bool {
        self.seen_pots
            .iter()
            .any(|pot_name| fn_name.starts_with(pot_name))
    }
}
