#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — RAML RESTful API Modeling
//! Automatically generated dedicated intermediate representation backend.

pub struct RamlExporter;

impl RamlExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// RAML RESTful API Modeling for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
