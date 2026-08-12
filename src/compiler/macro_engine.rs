#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Compiler — MacroEngine (Meta-Programming & Code Expansion)

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MacroDefinition {
    pub name: String,
    pub parameters: Vec<String>,
    pub template: String,
}

pub struct MacroEngine {
    macros: HashMap<String, MacroDefinition>,
}

impl MacroEngine {
    pub fn new() -> Self {
        let mut engine = MacroEngine {
            macros: HashMap::new(),
        };
        // Register default built-in macros
        engine.register(MacroDefinition {
            name: "assert_omni".into(),
            parameters: vec!["condition".into()],
            template: "if !({condition}) { panic(\"Omniversal Assertion Failed: {condition}\"); }".into(),
        });
        engine
    }

    pub fn register(&mut self, def: MacroDefinition) {
        println!("[MacroEngine] Registering macro rule: '{}'", def.name);
        self.macros.insert(def.name.clone(), def);
    }

    pub fn expand(&self, macro_name: &str, args: &[String]) -> Result<String, String> {
        println!("[MacroEngine] Expanding macro '{}' with args: {:?}", macro_name, args);
        if let Some(mac) = self.macros.get(macro_name) {
            let mut expanded = mac.template.clone();
            for (i, param) in mac.parameters.iter().enumerate() {
                if let Some(arg) = args.get(i) {
                    expanded = expanded.replace(&format!("{{{}}}", param), arg);
                }
            }
            println!("  -> Macro expanded successfully: '{}'", expanded);
            Ok(expanded)
        } else {
            Err(format!("Macro not found: '{}'", macro_name))
        }
    }
}
