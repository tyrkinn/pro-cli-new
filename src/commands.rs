use std::process::Command;

use crate::{
    config::{Editor, ProjectPath, RELATIVE_CONFIG_DIR},
    helpers::system_home,
    types::Project,
};

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
    pro comps                 -> Generate zsh comps
    pro config                -> Open config file in editor
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

pub fn open_config(editor: Editor) -> Result<(), String> {
    let editor_cmd = &mut Command::new(editor.command);
    let home = system_home().ok_or("Can't get home dir")?;
    editor_cmd
        .current_dir(home + RELATIVE_CONFIG_DIR)
        .arg("config.ron");

    for flag in editor.flags {
        editor_cmd.arg(flag);
    }

    editor_cmd
        .output()
        .map_err(|e| format!("Can't start editor beacuse of {e}"))?;

    Ok(())
}

pub fn open(projects: Vec<Project>, project_name: &str, editor: Editor) -> Result<(), String> {
    let selected_project = find_project(projects, project_name)
        .ok_or("Can't find project with this name".to_string())?;

    let project_path = &selected_project.full_path;

    let editor_cmd = &mut Command::new(editor.command);
    editor_cmd.current_dir(project_path).arg(project_path);

    for flag in editor.flags {
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

    Ok(format!("Successfully removed {project_name} project"))
}

pub fn gen_comps(project_dirs: Vec<ProjectPath>) {
    let dirs_str = project_dirs
        .iter()
        .map(|p| "~".to_string() + &p.path.clone())
        .join(" ");
    let comp = format!(
        r#"
_pro() {{
    local line state

    _arguments -C \
               "1: :->cmds" \
               "*::arg:->args"
    case "$state" in
        cmds)
            _values "pro command" \
                    "list[list all projects in project directories]" \
                    "open[open project in editor]" \
                    "remove[remove project dir]"  \
                    "path[get full project path]" \
                    "help[display help message]" \
            ;;
        args)
            case $line[1] in
                path | remove | open)
                    _select_project_cmd
                    ;;
            esac
            ;;
    esac
}}
_select_project_cmd() {{
     local oomph_dirs oomph_dirs=(-/ {dirs_str})
     _files -W oomph_dirs -g '*'
}}
compdef _pro pro
"#
    );

    println!("{comp}")
}
