#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — Concise Data Definition Language (CDDL)
//! Automatically generated dedicated intermediate representation backend.

pub struct CddlExporter;

impl CddlExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// Concise Data Definition Language (CDDL) for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
