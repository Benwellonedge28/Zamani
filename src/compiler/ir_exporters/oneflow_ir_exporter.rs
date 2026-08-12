#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — OneFlow Distributed Stream IR
//! Automatically generated dedicated intermediate representation backend.

pub struct OneFlowIrExporter;

impl OneFlowIrExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// OneFlow Distributed Stream IR for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
