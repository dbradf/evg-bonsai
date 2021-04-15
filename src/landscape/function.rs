use crate::landscape::command::{translate_command, translate_command_list, BonsaiCommand};
use serde::{Deserialize, Serialize};
use shrub_rs::models::project::FunctionDefinition;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BonsaiFunctionDef {
    SingleCommand(BonsaiCommand),
    CommandList(Vec<BonsaiCommand>),
}

pub fn translate_fn_def(fn_def: &BonsaiFunctionDef) -> FunctionDefinition {
    match fn_def {
        BonsaiFunctionDef::SingleCommand(cmd) => {
            FunctionDefinition::SingleCommand(translate_command(cmd))
        }
        BonsaiFunctionDef::CommandList(cmd_list) => {
            FunctionDefinition::CommandList(translate_command_list(cmd_list))
        }
    }
}

pub fn translate_functions(
    fn_map: &HashMap<String, BonsaiFunctionDef>,
) -> HashMap<String, FunctionDefinition> {
    let mut new_map = HashMap::new();
    for (k, v) in fn_map {
        new_map.insert(k.clone(), translate_fn_def(v));
    }
    new_map
}
