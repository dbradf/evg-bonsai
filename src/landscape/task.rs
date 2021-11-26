use crate::landscape::command::BonsaiCommand;
use serde::{Deserialize, Serialize};
use shrub_rs::models::task::{EvgTask, TaskDependency};

use super::command::BonsaiTranslator;

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiTask {
    /// Name of task being defined.
    pub name: String,
    /// List of command that make up the task.
    pub commands: Vec<BonsaiCommand>,
    /// List of other tasks that need to be completed before this is done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<TaskDependency>>,
    /// How long this task can run before timing out (in seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_timeout_secs: Option<u64>,
    /// List of tags describing this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Describe if this patch should be runnable in patch builds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patchable: Option<bool>,
    /// Describe if previously skipped versions of this task should be run on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
}

impl BonsaiTask {
    pub fn to_evg_task(&self, bonsai_translator: &mut BonsaiTranslator) -> EvgTask {
        let command_list = bonsai_translator.translate_command_list(&self.commands);

        EvgTask {
            name: self.name.clone(),
            commands: command_list,
            depends_on: self.depends_on.as_ref().map(|d| d.to_vec()),
            exec_timeout_secs: self.exec_timeout_secs,
            tags: self.tags.clone(),
            patchable: self.patchable,
            stepback: self.stepback,
        }
    }
}
