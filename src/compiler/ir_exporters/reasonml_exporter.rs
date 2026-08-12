#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — ReasonML Lambda Export
//! Automatically generated dedicated intermediate representation backend.

pub struct ReasonMlExporter;

impl ReasonMlExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// ReasonML Lambda Export for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
