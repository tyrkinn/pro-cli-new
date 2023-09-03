use crate::{config::ProConfig, helpers::read_paths, types::Project};

pub struct ProContext {
    pub config: ProConfig,
    pub projects: Vec<Project>,
}

impl ProContext {
    pub fn new(config: ProConfig) -> Self {
        let mut context = Self {
            config,
            projects: Vec::new(),
        };
        context.fetch_projects();
        context
    }

    pub fn fetch_projects(&mut self) {
        self.projects = read_paths(&self.config.home_dir, &self.config.projects_paths);
    }
}
