#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — HP-UX PA-RISC/Itanium IR
//! Automatically generated dedicated intermediate representation backend.

pub struct HpuxIrExporter;

impl HpuxIrExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// HP-UX PA-RISC/Itanium IR for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
