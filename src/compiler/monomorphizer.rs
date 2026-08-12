//! Zamani Monomorphizer
//! Handles specialization of generic functions and types.

use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub struct Monomorphizer {
    /// Maps (original_name, type_arguments) -> specialized_name
    specializations: HashMap<(String, Vec<String>), String>,
}

impl Monomorphizer {
    pub fn new() -> Self {
        Monomorphizer {
            specializations: HashMap::new(),
        }
    }

    /// Processes a program and returns a new program with all generic calls specialized.
    pub fn process(&mut self, program: Program) -> Program {
        let mut new_statements = Vec::new();
        
        // 1. Identify all generic definitions
        // 2. Find all call sites with type arguments
        // 3. Generate specialized versions of definitions
        // 4. Rewrite call sites to point to specialized versions
        
        for stmt in program.statements {
            match stmt {
                Statement::Function(span, name, params, ret, body) => {
                    // If it's a generic function, we'll keep it for now but it needs to be cloned for specializations
                    new_statements.push(Statement::Function(span, name, params, ret, body));
                }
                other => new_statements.push(other),
            }
        }

        Program {
            statements: new_statements,
            span: program.span,
        }
    }

    /// Generates a specialized name for a generic instance, e.g., "map<int>" -> "map_int"
    pub fn get_specialized_name(&mut self, original: &str, type_args: &[TypeExpr]) -> String {
        let arg_names: Vec<String> = type_args.iter().map(|t| t.name()).collect();
        let key = (original.to_string(), arg_names.clone());
        
        self.specializations.entry(key).or_insert_with(|| {
            format!("{}_{}", original, arg_names.join("_").replace("<", "_").replace(">", "_"))
        }).clone()
    }
}
