#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani stdlib — Web Development (routing, templating, WebAssembly)
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
}
#[derive(Debug, Clone)]
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: String,
}
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct WebRouter {
    routes: Vec<Route>,
    middlewares: Vec<String>,
}
impl WebRouter {
    pub fn new() -> Self {
        WebRouter { 
            routes: Vec::new(),
            middlewares: Vec::new(),
        }
    }
    pub fn route(&mut self, m: HttpMethod, path: &str, handler: &str) {
        self.routes.push(Route {
            method: m,
            path: path.into(),
            handler: handler.into(),
        });
    }
    pub fn use_middleware(&mut self, middleware: &str) {
        self.middlewares.push(middleware.into());
    }
    pub fn serve_static(&self, path: &str, dir: &str) {
        println!("[WebDev] Serving static files from '{}' at '{}'.", dir, path);
    }
    pub fn dispatch(&self, req: &HttpRequest) -> Option<&Route> {
        for mw in &self.middlewares {
            println!("[WebDev] Running middleware: {}", mw);
        }
        self.routes
            .iter()
            .find(|r| r.method == req.method && r.path == req.path)
    }
    pub fn render(&self, tmpl: &str, vars: &HashMap<String, String>) -> String {
        vars.iter().fold(tmpl.into(), |a, (k, v)| {
            a.replace(&format!("{{{{{}}}}}", k), v)
        })
    }
}
impl Default for WebRouter {
    fn default() -> Self {
        Self::new()
    }
}
pub fn init_web_development() {
    println!("  - Initializing Web Development...");
}
pub fn shutdown_web_development() {
    println!("  - Shutting down Web Development...");
}
