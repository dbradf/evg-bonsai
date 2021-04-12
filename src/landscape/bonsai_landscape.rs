use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use shrub_rs::models::variant::BuildVariant;
use shrub_rs::models::project::{EvgModule, EvgParameter, EvgProject};
use shrub_rs::models::builtin::CommandType;
use shrub_rs::models::commands::Command;
use shrub_rs::models::task::EvgTask;
use crate::landscape::bonsai_task::BonsaiTask;
use crate::landscape::bonsai_command::{BonsaiCommand, translate_command_list};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BonsaiPotLocation {
    Local,
    Github,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPotDesc {
    pub name: String,
    pub location: BonsaiPotLocation,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiPot {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, Vec<Command>>,
}

impl BonsaiPotDesc {
    fn get_module(&self) -> BonsaiPot {
        let contents = read_to_string(&self.path).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }
}

/// Description of an Bonsai Consumer Project.
#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiLandscape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonsai: Option<Vec<BonsaiPotDesc>>,
    /// List of build variants belonging to this landscape.
    pub buildvariants: Vec<BuildVariant>,
    /// List of task definitions.
    pub tasks: Vec<BonsaiTask>,
    /// Definitions of functions belonging to this landscape.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, Vec<BonsaiCommand>>,
    /// List of commands to run at the start of each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre: Option<Vec<BonsaiCommand>>,
    /// List of commands to run at the end of each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Vec<BonsaiCommand>>,
    /// List of commands to run whenever a task hits a timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Vec<BonsaiCommand>>,

    /// Description of modules to include in this landscape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<EvgModule>>,

    /// Describe if skipped tasks should be run on failures to determine source of failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
    /// Describe if failures in `pre` commands should cause a task to be failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_error_fails_task: Option<bool>,
    /// Describe if evergreen should track out of memory failure in this landscape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_tracker: Option<bool>,
    /// Describe the type of failure a task failure should trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<CommandType>,
    /// List of globs that describe file changes that won't trigger a new build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    /// Parameters that can be specified to customize patch build functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<EvgParameter>>,
}

impl Default for BonsaiLandscape {
    fn default() -> Self {
        BonsaiLandscape {
            bonsai: None,
            buildvariants: vec![],
            tasks: vec![],
            functions: Default::default(),
            pre: None,
            post: None,
            timeout: None,
            modules: None,
            stepback: None,
            pre_error_fails_task: None,
            oom_tracker: None,
            command_type: None,
            ignore: None,
            parameters: None,
        }
    }
}

impl BonsaiLandscape {
    pub fn create_evg_project(self) -> EvgProject {
        let mut function_map = HashMap::new();
        if let Some(bonsai_modules) = &self.bonsai {
            for module in bonsai_modules {
                let module_details = module.get_module();
                for (fn_name, fn_def) in module_details.functions {
                    function_map.insert(format!("{}_{}", module.name, fn_name), fn_def);
                }
            }
        }

        EvgProject {
            buildvariants: self.buildvariants.clone(),

            functions: self
                .translate_functions()
                .into_iter()
                .chain(function_map)
                .collect(),

            tasks: self.translate_tasks(),
            pre: self.translate_pre(),
            post: self.translate_post(),
            timeout: self.translate_timeout(),

            modules: self.modules,
            stepback: self.stepback,
            pre_error_fails_task: self.pre_error_fails_task,
            oom_tracker: self.oom_tracker,
            command_type: self.command_type,
            ignore: self.ignore,
            parameters: self.parameters,
        }
    }

    fn translate_pre(&self) -> Option<Vec<Command>> {
        if let Some(pre_commands) = &self.pre {
            Some(translate_command_list(pre_commands))
        } else {
            None
        }
    }

    fn translate_post(&self) -> Option<Vec<Command>> {
        if let Some(post_commands) = &self.post {
            Some(translate_command_list(post_commands))
        } else {
            None
        }
    }

    fn translate_timeout(&self) -> Option<Vec<Command>> {
        if let Some(timeout_commands) = &self.timeout {
            Some(translate_command_list(timeout_commands))
        } else {
            None
        }
    }

    fn translate_tasks(&self) -> Vec<EvgTask> {
        self.tasks.iter().map(|t| t.to_evg_task()).collect()
    }

    fn translate_functions(&self) -> HashMap<String, Vec<Command>> {
        let mut new_map = HashMap::new();
        for (k, v) in &self.functions {
            new_map.insert(k.clone(), translate_command_list(&v));
        }
        new_map
    }
}

