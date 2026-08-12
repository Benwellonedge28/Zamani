#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Compiler — Linker & Link-Time Optimization (ZLink)

use crate::ir_gen::{IrModule, IrFunction, IrGlobal};
use std::collections::HashMap;

pub struct ZamaniLinker {
    pub modules: Vec<IrModule>,
}

impl ZamaniLinker {
    pub fn new(modules: Vec<IrModule>) -> Self {
        ZamaniLinker { modules }
    }

    pub fn link(&self) -> IrModule {
        println!("[ZLink] Linking {} Zamani IR modules...", self.modules.len());
        let mut linked_module = IrModule::new("Zamani_Linked_Omniverse");

        for m in &self.modules {
            for f in &m.functions {
                linked_module.add_function(f.clone());
            }
            for g in &m.globals {
                linked_module.add_global(g.clone());
            }
            for s in &m.string_literals {
                linked_module.string_literals.push(s.clone());
            }
        }

        println!("  -> Linked total functions: {}, globals: {}", linked_module.functions.len(), linked_module.globals.len());
        linked_module
    }

    pub fn optimize_lto(&self, module: &mut IrModule) {
        println!("[ZLink-LTO] Running Link-Time Optimization across modules...");
        let initial_count = module.instruction_count();
        
        // Remove duplicate functions or dead globals
        module.functions.dedup_by(|a, b| a.name == b.name);

        let final_count = module.instruction_count();
        println!("  -> LTO Complete: Instructions optimized from {} to {}.", initial_count, final_count);
    }
}
