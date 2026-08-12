#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — HPGL Plotter Export
//! Automatically generated dedicated intermediate representation backend.

pub struct HpglExporter;

impl HpglExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// HPGL Plotter Export for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
