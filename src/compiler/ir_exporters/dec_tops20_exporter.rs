#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — DEC TOPS-20 Export
//! Automatically generated dedicated intermediate representation backend.

pub struct DecTops20Exporter;

impl DecTops20Exporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// DEC TOPS-20 Export for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
