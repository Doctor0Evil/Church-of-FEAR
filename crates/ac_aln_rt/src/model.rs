use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptConfig {
    pub session_key_template: String,
    pub bot_id: String,
    pub virtual_fs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scope {
    All,
    System,
    Global,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GitDiffType {
    WorkingTree,
    Staged,
    Branch,
    Folder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoryAction {
    UndoCommit,
    Clean,
    CreatePatch,
    Squash,
    Rebase { target: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmoduleAction {
    Init,
    Sync,
    Add {
        repo_url: String,
        path: String,
        branch: Option<String>,
        depth: Option<i32>,
    },
    SetBranch { path: String, branch: String },
    Move { old_path: String, new_path: String },
    Remove { path: String },
    Deinit { path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum P4Action {
    Clone { depot_path: String },
    Submit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneOptions {
    pub autocrlf: bool,
    pub depth: Option<u32>,
    pub single_branch: bool,
}

impl Default for CloneOptions {
    fn default() -> Self {
        Self {
            autocrlf: false,
            depth: None,
            single_branch: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnCall {
    pub name: String,
    pub args: HashMap<String, serde_json::Value>,
}
