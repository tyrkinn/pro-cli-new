use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

use crate::helpers::system_home;

pub const RELATIVE_CONFIG_DIR: &str = "/.config/pro/";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectPath {
    pub label: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Editor {
    pub command: String,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProConfig {
    pub home_dir: String,
    pub projects_paths: Vec<ProjectPath>,
    pub editor: Editor,
}

impl ProConfig {
    pub fn init() -> Result<Self, ConfigError> {
        let home_dir = system_home().ok_or(ConfigError::Message(
            "Can't read system home dir".to_string(),
        ))?;

        let config_file_path = home_dir.clone() + RELATIVE_CONFIG_DIR + "config.ron";

        let config = Config::builder()
            .add_source(config::File::with_name(&config_file_path).required(true))
            .set_default("editor", "nvim")?
            .set_default("editor_flags", Vec::<String>::new())?
            .set_default("home_dir", home_dir)?
            .build()?;

        config.try_deserialize::<Self>()
    }
}
