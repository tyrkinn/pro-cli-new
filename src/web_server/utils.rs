use std::fs;

use tiny_http::{Header, Response};

use crate::{config::RELATIVE_CONFIG_DIR, helpers::system_home};

pub const REQUIRED_TEMPLATES: &'static [&'static str] = &["project.hbs", "index.hbs"];

pub fn ensure_templates(templates_path: &str) -> bool {
    let home_path = system_home().unwrap();
    REQUIRED_TEMPLATES.into_iter().all(|template| {
        path_exists(&format!(
            "{home_path}{RELATIVE_CONFIG_DIR}{templates_path}{template}"
        ))
    })
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn respond_html(html_str: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let html_header = Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
    Response::from_string(html_str).with_header(html_header)
}
