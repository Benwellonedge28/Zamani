#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — NeXTSTEP Objective-C IR
//! Automatically generated dedicated intermediate representation backend.

pub struct NextStepIrExporter;

impl NextStepIrExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// NeXTSTEP Objective-C IR for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
