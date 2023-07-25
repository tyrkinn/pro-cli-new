pub mod commands;
pub mod config;
pub mod helpers;
pub mod types;
pub mod web_server;

use std::fs;

use helpers::{read_paths, system_home};
use web_server::{
    server::start_server,
    utils::{path_exists, REQUIRED_TEMPLATES},
};

use crate::config::{ProConfig, RELATIVE_CONFIG_DIR};

fn main() -> Result<(), ()> {
    let home = system_home().ok_or_else(|| eprintln!("Can't get home dir"))?;

    let pro_config =
        ProConfig::init().map_err(|err| eprintln!("Can't initialize config because of {err}"))?;

    let projects = read_paths(&home, pro_config.projects_paths);

    let args: Vec<String> = std::env::args().skip(1).collect();

    let str_args: Vec<&str> = args.iter().map(String::as_str).collect();

    match str_args[..] {
        ["help"] => commands::help(),
        ["list"] => commands::list(projects),
        ["config"] => commands::open_config(pro_config.editor).map_err(|e| eprintln!("{e}"))?,
        ["open", pr_name] => {
            commands::open(projects, pr_name, pro_config.editor).map_err(|e| eprintln!("{e}"))?
        }
        ["path", pr_name] => {
            let path = commands::path(projects, pr_name)
                .map_err(|e| eprintln!("Can't get project path beacuse of {e}"))?;
            println!("{path}");
        }
        ["remove", pr_name] => {
            let message =
                commands::remove_project(projects, pr_name).map_err(|e| eprintln!("{e}"))?;
            println!("{message}");
        }
        ["server", "install-templates"] => {
            let templates_path = format!("{home}{RELATIVE_CONFIG_DIR}templates/");
            if !path_exists(&templates_path) {
                fs::create_dir_all(&templates_path)
                    .map_err(|e| eprintln!("Can't create templates dir because of {e}"))?;
            }
            REQUIRED_TEMPLATES.into_iter().for_each(|&template_file| {
                fs::File::create(templates_path.clone() + template_file)
                    .map_err(|e| eprintln!("Can't create file because of {e}"))
                    .unwrap();
            })
        }
        ["server", "start"] => {
            start_server(projects, pro_config.editor)?;
        }
        _ => println!("Run `pro help` to get usage info"),
    }

    Ok(())
}
