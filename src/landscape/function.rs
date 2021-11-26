use crate::landscape::command::BonsaiCommand;
use serde::{Deserialize, Serialize};
use shrub_rs::models::commands::EvgCommand;
use shrub_rs::models::project::FunctionDefinition;
use std::collections::HashMap;

use super::command::BonsaiTranslator;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BonsaiFunctionDef {
    SingleCommand(BonsaiCommand),
    CommandList(Vec<BonsaiCommand>),
}

fn cmd_list_to_function_def(
    cmd_list: &[BonsaiCommand],
    bonsai_cmds: &HashMap<String, FunctionDefinition>,
) -> Vec<EvgCommand> {
    let mut evg_cmd_list = vec![];
    cmd_list.iter().for_each(|bc| match bc {
        BonsaiCommand::EvergreenNative(cmd) => evg_cmd_list.push(cmd.clone()),
        BonsaiCommand::Bonsai(cmd) => {
            let commands = bonsai_cmds.get(&cmd.get_fn_name()).unwrap();
            match commands {
                FunctionDefinition::SingleCommand(evg_cmd) => evg_cmd_list.push(evg_cmd.clone()),
                FunctionDefinition::CommandList(cmds) => evg_cmd_list.extend(cmds.to_vec()),
            }
        }
    });

    evg_cmd_list
}

pub fn translate_fn_def(
    fn_def: &BonsaiFunctionDef,
    bonsai_fns: &HashMap<String, FunctionDefinition>,
    bonsai_translator: &mut BonsaiTranslator,
) -> FunctionDefinition {
    match fn_def {
        BonsaiFunctionDef::SingleCommand(cmd) => {
            FunctionDefinition::SingleCommand(bonsai_translator.translate_command(cmd))
        }
        BonsaiFunctionDef::CommandList(cmd_list) => {
            FunctionDefinition::CommandList(cmd_list_to_function_def(cmd_list, bonsai_fns))
        }
    }
}

pub fn translate_functions(
    fn_map: &HashMap<String, BonsaiFunctionDef>,
    bonsai_fns: &HashMap<String, FunctionDefinition>,
    bonsai_translator: &mut BonsaiTranslator,
) -> HashMap<String, FunctionDefinition> {
    let mut new_map = HashMap::new();
    for (k, v) in fn_map {
        new_map.insert(
            k.clone(),
            translate_fn_def(v, bonsai_fns, bonsai_translator),
        );
    }
    new_map
}
