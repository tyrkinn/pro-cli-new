use std::{fs::DirEntry, io};

use crate::{config::ProjectPath, types::Project};

pub fn system_home() -> Option<String> {
    std::env::var_os("HOME").map(|os_string| os_string.to_str().unwrap().to_string())
}

fn read_dir(full_path: &str) -> io::Result<Vec<DirEntry>> {
    Ok(std::fs::read_dir(full_path)?.flatten().collect())
}

pub fn read_paths(home_dir: &str, dir_paths: Vec<ProjectPath>) -> Vec<Project> {
    dir_paths
        .into_iter()
        .map(|p| {
            let entries = read_dir(&format!("{home_dir}{}", p.path))?;
            Ok::<Vec<Project>, io::Error>(
                entries
                    .into_iter()
                    .filter(|e| e.metadata().unwrap().is_dir())
                    .map(|e| Project {
                        name: e.file_name().to_str().unwrap().to_string(),
                        full_path: e.path().to_str().unwrap().to_string(),
                        project_path: p.clone(),
                    })
                    .collect::<Vec<Project>>(),
            )
        })
        .flatten()
        .flatten()
        .collect()
}
