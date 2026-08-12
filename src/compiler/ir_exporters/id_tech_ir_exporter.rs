#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — id Tech Material & Shader IR
//! Automatically generated dedicated intermediate representation backend.

pub struct IdTechIrExporter;

impl IdTechIrExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// id Tech Material & Shader IR for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
