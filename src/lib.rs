use std::{collections::HashMap, fs::read_to_string};
use serde::{Deserialize, Serialize};
use shrub_rs::models::{commands::{Command, CommandType, FunctionCall, ParamValue}, project::{EvgModule, EvgParameter, EvgProject}, task::{EvgTask, TaskDependency}, variant::BuildVariant};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BonsaiModuleLocation {
    Local,
    Github,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiModuleDesc {
    pub name: String,
    pub location: BonsaiModuleLocation,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiModule {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, Vec<Command>>,
}

impl BonsaiModuleDesc {
    fn get_module(&self) -> BonsaiModule {
        let contents = read_to_string(&self.path).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }
}

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
    pub fn to_evg_task(&self) -> EvgTask {
        let command_list = translate_command_list(&self.commands);

        EvgTask {
            name: self.name.clone(),
            commands: command_list,
            depends_on: None, // XXX
            exec_timeout_secs: self.exec_timeout_secs,
            tags: self.tags.clone(),
            patchable: self.patchable,
            stepback: self.stepback,
        }
    }
}

/// Description of an Bonsai Consumer Project.
#[derive(Serialize, Deserialize, Debug)]
pub struct BonsaiProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonsai: Option<Vec<BonsaiModuleDesc>>,
    /// List of build variants belonging to this project.
    pub buildvariants: Vec<BuildVariant>,
    /// List of task definitions.
    pub tasks: Vec<BonsaiTask>,
    /// Definitions of functions belonging to this project.
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

    /// Description of modules to include in this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<EvgModule>>,

    /// Describe if skipped tasks should be run on failures to determine source of failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
    /// Describe if failures in `pre` commands should cause a task to be failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_error_fails_task: Option<bool>,
    /// Describe if evergreen should track out of memory failure in this project.
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

impl Default for BonsaiProject {
    fn default() -> Self {
        BonsaiProject {
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

impl BonsaiProject {
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

            functions: self.translate_functions().into_iter().chain(function_map).collect(),

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

fn translate_command_list(bonsai_command_list: &Vec<BonsaiCommand>) -> Vec<Command> {
    let mut command_list = vec![];
    for command in bonsai_command_list {
        let evg_command = match command {
            BonsaiCommand::Bonsai(b_cmd) => {
                let parameters = match &b_cmd.params {
                    Some(vars) => Some(vars.clone()),
                    None => None
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
