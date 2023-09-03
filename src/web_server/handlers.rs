use std::collections::HashMap;

use handlebars::{to_json, Handlebars};
use serde_json::{Map, Value};
use tiny_http::{Request, Response, StatusCode};

use crate::{commands, context::ProContext};

use super::utils::{parse_query, respond_html};

pub fn root_handler(
    context: &mut ProContext,
    request: Request,
    hbs: &Handlebars,
) -> Result<(), ()> {
    let mut data: Map<String, Value> = Map::new();

    data.insert("projects".to_string(), to_json(&context.projects));
    data.insert("count".to_string(), to_json(context.projects.len()));

    let template: String = hbs
        .render("index", &data)
        .map_err(|e| eprintln!("Can't render template because of {e}"))?;

    let _ = request.respond(respond_html(&template));
    context.fetch_projects();

    Ok(())
}

pub fn open_handler(context: &ProContext, request: Request, full_url: &str) -> Result<(), ()> {
    let params: HashMap<String, String> =
        parse_query(full_url).map_err(|_| eprintln!("Can't parse query params"))?;
    if let Some(project_name) = params.get("project") {
        let open = commands::open(context, project_name);
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

pub fn delete_handler(
    context: &mut ProContext,
    request: Request,
    full_url: &str,
) -> Result<(), ()> {
    let params: HashMap<String, String> =
        parse_query(full_url).map_err(|_| eprintln!("Can't parse query params"))?;

    if let Some(project_name) = params.get("project") {
        match commands::remove_project(context, project_name) {
            Ok(_) => {
                let _ = request.respond(Response::new_empty(StatusCode(200)));
                context.fetch_projects();
            }
            Err(_) => {
                let _ = request.respond(Response::new_empty(StatusCode(500)));
            }
        }
    } else {
        let _ = request.respond(Response::new_empty(StatusCode(500)));
    }
    Ok(())
}
