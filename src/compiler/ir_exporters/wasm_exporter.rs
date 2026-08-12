#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — WebAssembly Exporter
//! Translates Zamani IR functions into Wasm text format (.wat).

pub struct WasmExporter;

impl WasmExporter {
    pub fn export_module(module_name: &str, func_body: &str) -> String {
        format!(
            "(module\n  (memory (export \"memory\") 1)\n  (func (export \"{}\") (param i32) (result i32)\n    {}\n  )\n)\n",
            module_name, func_body
        )
    }
}
