use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use shrub_rs::models::params::ParamValue;
use shrub_rs::models::commands::{Command, FunctionCall};

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiCall {
    pub bonsai: String,
    pub params: Option<HashMap<String, ParamValue>>,
}

impl BonsaiCall {
    fn get_fn_name(&self) -> String {
        let parts: Vec<&str> = self.bonsai.split(":").collect();
        format!("{}_{}", parts[0], parts[1])
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BonsaiCommand {
    EvergreenNative(Command),
    Bonsai(BonsaiCall),
}

pub fn translate_command_list(bonsai_command_list: &Vec<BonsaiCommand>) -> Vec<Command> {
    let mut command_list = vec![];
    for command in bonsai_command_list {
        let evg_command = match command {
            BonsaiCommand::Bonsai(b_cmd) => {
                let parameters = match &b_cmd.params {
                    Some(vars) => Some(vars.clone()),
                    None => None,
                };
                Command::Function(FunctionCall {
                    func: b_cmd.get_fn_name(),
                    vars: parameters,
                    timeout_secs: None,
                })
            }
            BonsaiCommand::EvergreenNative(c) => c.clone(),
        };
        command_list.push(evg_command);
    }

    command_list
}
