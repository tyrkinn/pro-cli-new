use std::fs;

use handlebars::{Handlebars, TemplateError};

use tiny_http::{Method, Server};
use url::Url;

use crate::{
    config::{Editor, RELATIVE_CONFIG_DIR},
    helpers::system_home,
    types::Project,
    web_server::handlers::{self, open_handler},
};

fn template_path(template_name: &str) -> String {
    let home = system_home().unwrap();

    format!("{home}{RELATIVE_CONFIG_DIR}/templates/{template_name}")
}

fn prepare_handlebars<'a>() -> Result<Handlebars<'a>, TemplateError> {
    let mut hbs = Handlebars::new();
    let project_partial = fs::read_to_string(template_path("project.hbs")).unwrap();

    hbs.register_template_file("index", template_path("index.hbs"))?;
    hbs.register_partial("project", project_partial)?;

    Ok(hbs)
}

pub fn start_server(projects: Vec<Project>, editor: Editor) -> Result<(), ()> {
    let url = "0.0.0.0:8000";
    let base_url =
        Url::parse(&format!("http://{url}")).map_err(|_| eprintln!("Can't parse url"))?;
    let server = Server::http(url).unwrap();
    let hbs =
        prepare_handlebars().map_err(|e| eprintln!("Can't prepare handlebars because of {e}"))?;
    println!("INFO: Starting server at {base_url}");
    for request in server.incoming_requests() {
        let endpoint = request.url();
        match endpoint {
            "/" => handlers::root_handler(request, &hbs, &projects)?,
            x if x.starts_with("/open") && *request.method() == Method::Get => {
                let full_url = format!("{base_url}{x}");

                open_handler(request, &projects, &full_url, editor.clone())?
            }

            _ => {}
        }
    }
    Ok(())
}
