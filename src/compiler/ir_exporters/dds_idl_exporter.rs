#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — Data Distribution Service (DDS) IDL
//! Automatically generated dedicated intermediate representation backend.

pub struct DdsIdlExporter;

impl DdsIdlExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// Data Distribution Service (DDS) IDL for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
