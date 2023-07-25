use handlebars::{to_json, Handlebars};
use serde_json::{Map, Value};
use tiny_http::Request;

use crate::types::Project;

use super::utils::respond_html;

pub fn root_handler(request: Request, hbs: &Handlebars, projects: &Vec<Project>) -> Result<(), ()> {
    let mut data: Map<String, Value> = Map::new();
    data.insert("projects".to_string(), to_json(projects));

    let template: String = hbs
        .render("index", &data)
        .map_err(|e| eprintln!("Can't render template because of {e}"))?;

    let _ = request.respond(respond_html(&template));
    Ok(())
}
