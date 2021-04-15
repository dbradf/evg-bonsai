use serde::{Deserialize, Serialize};
use shrub_rs::models::commands::FunctionCall;
use shrub_rs::models::{commands::EvgCommand, params::ParamValue};
use std::collections::HashMap;

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

pub fn translate_command(bonsai_command: &BonsaiCommand) -> EvgCommand {
    match bonsai_command {
        BonsaiCommand::Bonsai(b_cmd) => b_cmd.to_evg_command(),
        BonsaiCommand::EvergreenNative(c) => c.clone(),
    }
}

pub fn translate_command_list(bonsai_command_list: &[BonsaiCommand]) -> Vec<EvgCommand> {
    bonsai_command_list
        .iter()
        .map(|c| translate_command(c))
        .collect()
}
