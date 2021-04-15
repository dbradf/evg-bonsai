use crate::landscape::command::{translate_command_list, BonsaiCommand};
use crate::landscape::function::{translate_functions, BonsaiFunctionDef};
use crate::landscape::pot::{BonsaiPotDesc, BonsaiPotSource};
use crate::landscape::task::BonsaiTask;
use crate::pot::remote_pot::copy_support_files;
use serde::{Deserialize, Serialize};
use shrub_rs::models::builtin::EvgCommandType;
use shrub_rs::models::commands::EvgCommand;
use shrub_rs::models::project::{EvgModule, EvgParameter, EvgProject, FunctionDefinition};
use shrub_rs::models::task::EvgTask;
use shrub_rs::models::variant::BuildVariant;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use shrub_rs::models::task_group::EvgTaskGroup;

/// Description of an Bonsai Consumer Project.
#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiLandscape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonsai: Option<Vec<BonsaiPotDesc>>,
    /// List of build variants belonging to this landscape.
    pub buildvariants: Vec<BuildVariant>,
    /// List of task definitions.
    pub tasks: Vec<BonsaiTask>,

    pub task_groups: Vec<EvgTaskGroup>,

    /// Definitions of functions belonging to this landscape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<HashMap<String, BonsaiFunctionDef>>,
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
    pub command_type: Option<EvgCommandType>,
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
            task_groups: vec![],
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
    pub fn copy_remote_support_files(&self, destination_dir: &Path) -> Result<(), Box<dyn Error>> {
        if let Some(bonsai_pot_list) = &self.bonsai {
            for pot_descriptor in bonsai_pot_list {
                if let BonsaiPotSource::Github(github_source) = &pot_descriptor.source {
                    copy_support_files(github_source, destination_dir)?;
                }
            }
        }

        Ok(())
    }

    pub fn create_evg_project(&self) -> Result<EvgProject, Box<dyn Error>> {
        let mut pot_map = HashMap::new();
        if let Some(bonsai_pot_list) = &self.bonsai {
            bonsai_pot_list
                .iter()
                .try_for_each(|p| p.update_pot_map(&mut pot_map))?;
        }

        let mut function_map = HashMap::new();
        let mut still_updating = true;
        while still_updating {
            still_updating = pot_map.values().any(|p| p.update_fn_map(&mut function_map));
        }

        Ok(EvgProject {
            buildvariants: self.buildvariants.clone(),

            functions: self
                .translate_functions()
                .into_iter()
                .chain(function_map)
                .collect(),

            tasks: self.translate_tasks(),
            task_groups: self.task_groups.to_vec(),
            pre: self.translate_pre(),
            post: self.translate_post(),
            timeout: self.translate_timeout(),

            modules: self.modules.as_ref().map(|m| m.to_vec()),
            stepback: self.stepback,
            pre_error_fails_task: self.pre_error_fails_task,
            oom_tracker: self.oom_tracker,
            command_type: self.command_type.as_ref().cloned(),
            ignore: self.ignore.as_ref().cloned(),
            parameters: self.parameters.as_ref().cloned(),
        })
    }

    fn translate_pre(&self) -> Option<Vec<EvgCommand>> {
        if let Some(pre_commands) = &self.pre {
            Some(translate_command_list(pre_commands))
        } else {
            None
        }
    }

    fn translate_post(&self) -> Option<Vec<EvgCommand>> {
        if let Some(post_commands) = &self.post {
            Some(translate_command_list(post_commands))
        } else {
            None
        }
    }

    fn translate_timeout(&self) -> Option<Vec<EvgCommand>> {
        if let Some(timeout_commands) = &self.timeout {
            Some(translate_command_list(timeout_commands))
        } else {
            None
        }
    }

    fn translate_tasks(&self) -> Vec<EvgTask> {
        self.tasks.iter().map(|t| t.to_evg_task()).collect()
    }

    fn translate_functions(&self) -> HashMap<String, FunctionDefinition> {
        if let Some(functions) = &self.functions {
            translate_functions(functions)
        } else {
            HashMap::new()
        }
    }
}
