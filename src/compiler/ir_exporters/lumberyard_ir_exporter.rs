#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — Amazon Lumberyard Gem IR
//! Automatically generated dedicated intermediate representation backend.

pub struct LumberyardIrExporter;

impl LumberyardIrExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// Amazon Lumberyard Gem IR for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
