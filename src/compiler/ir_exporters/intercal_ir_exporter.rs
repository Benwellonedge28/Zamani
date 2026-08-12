#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — INTERCAL IR Export
//! Automatically generated dedicated intermediate representation backend.

pub struct IntercalIrExporter;

impl IntercalIrExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// INTERCAL IR Export for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
