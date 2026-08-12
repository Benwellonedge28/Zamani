#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — PostCSS IR Export
//! Automatically generated dedicated intermediate representation backend.

pub struct PostCssExporter;

impl PostCssExporter {
    pub fn export_ir(target: &str, body: &str) -> String {
        format!(
            "// PostCSS IR Export for target {0}\n---\n{1}\n",
            target, body
        )
    }
}
