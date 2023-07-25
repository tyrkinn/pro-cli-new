use serde::{Deserialize, Serialize};

use crate::config::ProjectPath;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub project_path: ProjectPath,
    pub full_path: String,
    pub name: String,
}
