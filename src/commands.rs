use std::process::Command;

use crate::types::Project;

use itertools::Itertools;

pub fn help() {
    println!(
        r#"Pro CLI v0.1.0

Usage:
    pro list                  -> List projects
    pro path <PROJECT_NAME>   -> Get full project path
    pro remove <PROJECT_NAME> -> Remove project
    pro open <PROJECT_NAME>   -> Open project in vscode
    pro help                  -> Display this message
    pro server start          -> Start web ui"#
    );
}

pub fn list(projects: Vec<Project>) {
    let groups = projects.into_iter().group_by(|p| p.project_path.to_owned());
    for group in groups.into_iter() {
        let (k, values) = group;

        println!("{}", k.label);
        values.for_each(|p| {
            println!("{:>7}{}", "", p.name);
        });
        println!();
    }
}

fn find_project(projects: Vec<Project>, project_name: &str) -> Option<Project> {
    projects.into_iter().find(|p| p.name == project_name)
}

pub fn open(
    projects: Vec<Project>,
    project_name: &str,
    editor: &str,
    editor_flags: Vec<String>,
) -> Result<(), String> {
    let selected_project = find_project(projects, project_name)
        .ok_or("Can't find project with this name".to_string())?;

    let project_path = &selected_project.full_path;

    let editor_cmd = &mut Command::new(editor);
    editor_cmd.current_dir(project_path).arg(project_path);

    for flag in editor_flags {
        editor_cmd.arg(flag);
    }

    editor_cmd
        .output()
        .map_err(|e| format!("Can't start editor beacuse of {e}"))?;

    Ok(())
}

pub fn path(projects: Vec<Project>, project_name: &str) -> Result<String, String> {
    let selected_project =
        find_project(projects, project_name).ok_or("Can't find project".to_string())?;

    Ok(selected_project.full_path)
}

pub fn remove_project(projects: Vec<Project>, project_name: &str) -> Result<String, String> {
    let path = path(projects, project_name)?;
    std::fs::remove_dir_all(path).map_err(|e| format!("Can't remove directory because of {e}"))?;

    Ok("Successfully removed {project_name} project".to_string())
}