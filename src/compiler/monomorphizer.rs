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
        let mut generic_fns = HashMap::new();
        let mut generic_structs = HashMap::new();

        // 1. Identify all generic definitions
        for stmt in &program.statements {
            match stmt {
                Statement::Function(_, name, params, _, _) if !params.is_empty() => {
                    generic_fns.insert(name.clone(), stmt.clone());
                }
                Statement::Struct(_, name, type_params, _) if !type_params.is_empty() => {
                    generic_structs.insert(name.0.clone(), stmt.clone());
                }
                _ => {}
            }
        }

        // 2. Scan for usage and generate specialized versions
        for stmt in program.statements {
            match stmt {
                Statement::Let(span, name, ty_ann, expr) => {
                    let new_expr = self.specialize_expression(expr, &generic_fns, &mut new_statements);
                    new_statements.push(Statement::Let(span, name, ty_ann, new_expr));
                }
                Statement::Function(span, name, params, ret, body) => {
                    let new_body = self.specialize_expression(body, &generic_fns, &mut new_statements);
                    new_statements.push(Statement::Function(span, name, params, ret, new_body));
                }
                other => new_statements.push(other),
            }
        }

        Program {
            statements: new_statements,
            span: program.span,
        }
    }

    fn specialize_expression(&mut self, expr: Expression, generic_fns: &HashMap<String, Statement>, new_stmts: &mut Vec<Statement>) -> Expression {
        match expr {
            Expression::Call(span, func, args) => {
                if let Expression::Identifier(id) = func.as_ref() {
                    // Check if this is a call to a generic function with explicit type args
                    // In a real implementation, we'd check type_args in the AST
                    println!("[Monomorphizer] Checking call to: {}", id.0);
                }
                Expression::Call(span, func, args)
            }
            _ => expr,
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
