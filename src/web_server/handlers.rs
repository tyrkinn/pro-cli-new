use std::collections::HashMap;

use handlebars::{to_json, Handlebars};
use serde_json::{Map, Value};
use tiny_http::{Request, Response, StatusCode};

use crate::{commands, config::Editor, types::Project};

use super::utils::{parse_query, respond_html};

pub fn root_handler(request: Request, hbs: &Handlebars, projects: &Vec<Project>) -> Result<(), ()> {
    let mut data: Map<String, Value> = Map::new();

    data.insert("projects".to_string(), to_json(projects));
    data.insert("count".to_string(), to_json(projects.len()));

    let template: String = hbs
        .render("index", &data)
        .map_err(|e| eprintln!("Can't render template because of {e}"))?;

    let _ = request.respond(respond_html(&template));
    Ok(())
}

pub fn open_handler(
    request: Request,
    projects: &Vec<Project>,
    full_url: &str,
    editor: Editor,
) -> Result<(), ()> {
    let params: HashMap<String, String> =
        parse_query(full_url).map_err(|_| eprintln!("Can't parse query params"))?;
    if let Some(project_name) = params.get("project") {
        let open = commands::open(projects.clone(), project_name, editor);
        if open.is_err() {
            let _ = request.respond(Response::new_empty(StatusCode(400)));
        } else {
            let _ = request.respond(Response::new_empty(StatusCode(200)));
        }
    } else {
        let _ = request.respond(Response::new_empty(StatusCode(400)));
    }
    Ok(())
}
