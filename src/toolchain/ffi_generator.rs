#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Toolchain — C-FFI Header Generator

use crate::ir_gen::IrModule;

pub struct CFFIGenerator {
    pub module_name: String,
}

impl CFFIGenerator {
    pub fn new(module_name: impl Into<String>) -> Self {
        CFFIGenerator {
            module_name: module_name.into(),
        }
    }

    pub fn generate_header(&self, module: &IrModule) -> String {
        println!("[C-FFI] Generating C header bindings for module '{}'...", self.module_name);
        let mut header = format!("/* Auto-generated C header for Zamani module: {} */\n", self.module_name);
        header.push_str("#ifndef ZAMANI_FFI_H\n#define ZAMANI_FFI_H\n\n#include <stdint.h>\n#include <stdbool.h>\n\n");

        header.push_str("#ifdef __cplusplus\nextern \"C\" {\n#endif\n\n");

        for f in &module.functions {
            header.push_str(&format!("int64_t zamani_export_{}(void);\n", f.name));
        }

        header.push_str("\n#ifdef __cplusplus\n}\n#endif\n\n#endif /* ZAMANI_FFI_H */\n");
        println!("  -> C header generated successfully ({} bytes).", header.len());
        header
    }
}
