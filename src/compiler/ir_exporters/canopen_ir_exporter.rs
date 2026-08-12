#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — CANopen Object Dictionary IR
//! Automatically generated dedicated intermediate representation backend.

pub struct CanOpenIrExporter;

impl CanOpenIrExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// CANopen Object Dictionary IR for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
