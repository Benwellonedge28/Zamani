#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Compiler — Hindley-Milner Type Inference Engine

use std::collections::HashMap;
use crate::ast::{Expression, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeScheme {
    Mono(Type),
    Poly(Vec<usize>, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Equal(Type, Type),
    Instance(Type, String),
}

pub struct TypeInferenceEngine {
    next_var_id: usize,
    substitutions: HashMap<usize, Type>,
}

impl TypeInferenceEngine {
    pub fn new() -> Self {
        TypeInferenceEngine {
            next_var_id: 1,
            substitutions: HashMap::new(),
        }
    }

    pub fn fresh_var(&mut self) -> Type {
        let id = self.next_var_id;
        self.next_var_id += 1;
        Type::Generic(format!("T{}", id), vec![])
    }

    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<(), String> {
        println!("[HM-Inference] Unifying types: {:?} and {:?}", t1, t2);
        if t1 == t2 {
            return Ok(());
        }
        match (t1, t2) {
            (Type::Unknown, _) | (_, Type::Unknown) => Ok(()),
            _ => Err(format!("Type mismatch: cannot unify {:?} with {:?}", t1, t2)),
        }
    }

    pub fn infer(&mut self, expr: &Expression) -> Result<Type, String> {
        match expr {
            Expression::Literal(_, lit) => match lit {
                crate::ast::Literal::Integer(_, _) => Ok(Type::Int(crate::ast::IntWidth::I64)),
                crate::ast::Literal::Float(_, _) => Ok(Type::Float(crate::ast::FloatWidth::F64)),
                crate::ast::Literal::String(_, _) => Ok(Type::String),
                crate::ast::Literal::Boolean(_, _) => Ok(Type::Bool),
                _ => Ok(Type::Unit),
            },
            _ => Ok(Type::Unknown),
        }
    }
}
