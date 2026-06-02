//! Zenith UMC Semantic Analyser
//!
//! Performs type checking, symbol resolution, scope management,
//! effect tracking, and OOP validation over the Zenith AST.

use crate::ast::{
    ClassMember, Expression, Identifier, InterfaceMember, Literal, MatchCase, Parameter, Program,
    Statement,
};
use crate::compiler_types::{AccessModifier, FloatWidth, IntWidth, MethodType, Type};
use crate::source_map::Span;
use std::collections::HashMap;

// ─── Errors ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

impl SemanticError {
    fn new(message: impl Into<String>, span: Span) -> Self {
        SemanticError {
            message: message.into(),
            span,
        }
    }
}

// ─── Symbol ───────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum Symbol {
    Variable(Type),
    Function(Vec<Type>, Type),
    Effect(String),
    TypeAlias(Type),
    ClassRef(String),
    InterfaceRef(String),
    Module(String),
}

// ─── Symbol table ─────────────────────────────────────────────────────────────

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    resolved_classes: HashMap<String, Type>,
    resolved_interfaces: HashMap<String, Type>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
            resolved_classes: HashMap::new(),
            resolved_interfaces: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, sym: Symbol) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, sym);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(s) = scope.get(name) {
                return Some(s);
            }
        }
        None
    }

    pub fn define_class(&mut self, name: String, ty: Type) {
        self.resolved_classes.insert(name.clone(), ty.clone());
        self.define(name, Symbol::ClassRef(ty.get_name()));
    }

    pub fn lookup_class(&self, name: &str) -> Option<&Type> {
        self.resolved_classes.get(name)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Analyser ─────────────────────────────────────────────────────────────────

pub struct SemanticAnalyzer {
    pub symbols: SymbolTable,
    pub errors: Vec<SemanticError>,
    pub current_return_type: Option<Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut s = SemanticAnalyzer {
            symbols: SymbolTable::new(),
            errors: vec![],
            current_return_type: None,
        };
        s.register_builtins();
        s
    }

    fn register_builtins(&mut self) {
        // Standard builtins
        self.symbols.define(
            "print".into(),
            Symbol::Function(vec![Type::String], Type::Unit),
        );
        self.symbols.define(
            "println".into(),
            Symbol::Function(vec![Type::String], Type::Unit),
        );
        self.symbols.define(
            "assert".into(),
            Symbol::Function(vec![Type::Bool], Type::Unit),
        );
        self.symbols.define(
            "panic".into(),
            Symbol::Function(vec![Type::String], Type::Unit),
        );
        // Type names
        self.symbols
            .define("int".into(), Symbol::TypeAlias(Type::Int(IntWidth::I64)));
        self.symbols
            .define("i32".into(), Symbol::TypeAlias(Type::Int(IntWidth::I32)));
        self.symbols
            .define("i64".into(), Symbol::TypeAlias(Type::Int(IntWidth::I64)));
        self.symbols.define(
            "f64".into(),
            Symbol::TypeAlias(Type::Float(FloatWidth::F64)),
        );
        self.symbols
            .define("bool".into(), Symbol::TypeAlias(Type::Bool));
        self.symbols
            .define("String".into(), Symbol::TypeAlias(Type::String));
        self.symbols
            .define("str".into(), Symbol::TypeAlias(Type::Str));
    }

    pub fn analyze(&mut self, program: &Program) -> Vec<SemanticError> {
        for stmt in &program.statements {
            self.check_statement(stmt);
        }
        self.errors.clone()
    }

    // ── Statements ────────────────────────────────────────────────────────────

    fn check_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let(span, name, typ_ann, expr) => {
                let inferred = self.infer_expression(expr);
                let ty = if let Some(ann) = typ_ann {
                    self.resolve_type_annotation_name(ann)
                } else {
                    inferred
                };
                self.symbols.define(name.clone(), Symbol::Variable(ty));
            }
            Statement::Return(span, expr) => {
                let ty = self.infer_expression(expr);
                if let Some(ret_ty) = &self.current_return_type.clone() {
                    if !self.types_compatible(ret_ty, &ty) {
                        self.errors.push(SemanticError::new(
                            format!(
                                "Return type mismatch: expected {}, got {}",
                                ret_ty.get_name(),
                                ty.get_name()
                            ),
                            span.clone(),
                        ));
                    }
                }
            }
            Statement::Function(span, name, params, ret_ann, body) => {
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| {
                        p.typ
                            .as_ref()
                            .map(|t| self.resolve_type_annotation_name(t))
                            .unwrap_or(Type::Unknown)
                    })
                    .collect();
                let ret_ty = ret_ann
                    .as_ref()
                    .map(|t| self.resolve_type_annotation_name(t))
                    .unwrap_or(Type::Unit);
                self.symbols.define(
                    name.clone(),
                    Symbol::Function(param_types.clone(), ret_ty.clone()),
                );
                self.symbols.enter_scope();
                let prev_ret = self.current_return_type.replace(ret_ty);
                for (param, ty) in params.iter().zip(param_types.iter()) {
                    self.symbols
                        .define(param.name.0.clone(), Symbol::Variable(ty.clone()));
                }
                self.infer_expression(body);
                self.current_return_type = prev_ret;
                self.symbols.exit_scope();
            }
            Statement::Expression(expr) => {
                self.infer_expression(expr);
            }
            Statement::While(span, cond, body) => {
                let ct = self.infer_expression(cond);
                if ct != Type::Bool && ct != Type::Unknown {
                    self.errors.push(SemanticError::new(
                        "While condition must be bool",
                        span.clone(),
                    ));
                }
                self.infer_expression(body);
            }
            Statement::For(span, var, iter, body) => {
                let iter_ty = self.infer_expression(iter);
                let elem_ty = match &iter_ty {
                    Type::Array(t, _) | Type::Slice(t) => *t.clone(),
                    _ => Type::Unknown,
                };
                self.symbols.enter_scope();
                self.symbols
                    .define(var.0.clone(), Symbol::Variable(elem_ty));
                self.infer_expression(body);
                self.symbols.exit_scope();
            }
            Statement::Match(span, subject, cases) => {
                self.infer_expression(subject);
                for case in cases {
                    self.infer_expression(&case.body);
                }
            }
            Statement::QuantumCircuit(span, name, body) => {
                self.symbols.define(
                    name.clone(),
                    Symbol::Variable(Type::Quantum(Box::new(Type::Unit))),
                );
                self.infer_expression(body);
            }
            Statement::NanoAgent(span, name, body) => {
                self.symbols.define(
                    name.clone(),
                    Symbol::Variable(Type::Nano(Box::new(Type::Unit))),
                );
                self.infer_expression(body);
            }
            Statement::SankofaMemory(span, name, expr) => {
                let ty = self.infer_expression(expr);
                self.symbols
                    .define(name.clone(), Symbol::Variable(Type::Sankofa(Box::new(ty))));
            }
            _ => {} // Other statements: Break, Continue, Import, etc.
        }
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    pub fn infer_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Literal(lit) => self.infer_literal(lit),
            Expression::Identifier(id) => match self.symbols.lookup(&id.0) {
                Some(Symbol::Variable(ty)) => ty.clone(),
                Some(Symbol::Function(params, ret)) => {
                    Type::Function(params.clone(), Box::new(ret.clone()))
                }
                Some(Symbol::TypeAlias(ty)) => ty.clone(),
                Some(_) => Type::Unknown,
                None => {
                    self.errors.push(SemanticError::new(
                        format!("Undefined symbol '{}'", id.0),
                        id.1.clone(),
                    ));
                    Type::Error
                }
            },
            Expression::Prefix(span, op, operand) => {
                let ty = self.infer_expression(operand);
                match op {
                    TokenType::Minus if ty.is_numeric() => ty,
                    TokenType::Not if ty == Type::Bool => Type::Bool,
                    _ => Type::Unknown,
                }
            }
            Expression::Infix(span, left, op, right) => {
                let lt = self.infer_expression(left);
                let rt = self.infer_expression(right);
                self.infer_infix_type(span, op, &lt, &rt)
            }
            Expression::Call(span, func, args) => {
                let fn_ty = self.infer_expression(func);
                match fn_ty {
                    Type::Function(_, ret) => *ret,
                    _ => Type::Unknown,
                }
            }
            Expression::If(_, cond, then, else_branch) => {
                let ct = self.infer_expression(cond);
                let tt = self.infer_expression(then);
                if let Some(eb) = else_branch {
                    self.infer_expression(eb);
                }
                tt
            }
            Expression::Block(_, stmts) => {
                self.symbols.enter_scope();
                let mut last = Type::Unit;
                for stmt in stmts {
                    if let Statement::Expression(e) = stmt {
                        last = self.infer_expression(e);
                    } else {
                        self.check_statement(stmt);
                        last = Type::Unit;
                    }
                }
                self.symbols.exit_scope();
                last
            }
            Expression::MemberAccess(span, obj, member) => {
                let obj_ty = self.infer_expression(obj);
                Type::Unknown // resolved during IR gen for known types
            }
            Expression::Index(span, arr, idx) => {
                let arr_ty = self.infer_expression(arr);
                match arr_ty {
                    Type::Array(elem, _) | Type::Slice(elem) => *elem,
                    _ => Type::Unknown,
                }
            }
            Expression::Recall(span, key) => {
                self.infer_expression(key);
                Type::Sankofa(Box::new(Type::Unknown))
            }
            _ => Type::Unknown,
        }
    }

    fn infer_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Integer(_, _) => Type::Int(IntWidth::I64),
            Literal::Float(_, _) => Type::Float(FloatWidth::F64),
            Literal::String(_, _) => Type::String,
            Literal::Boolean(_, _) => Type::Bool,
            Literal::Char(_, _) => Type::Char,
            Literal::Null(_) => Type::Option(Box::new(Type::Unknown)),
            Literal::Quantum(_, _) => Type::Quantum(Box::new(Type::Unknown)),
            Literal::Nano(_, _) => Type::Nano(Box::new(Type::Unknown)),
            Literal::MTS(_, _) => Type::MTS(Box::new(Type::Unknown)),
        }
    }

    fn infer_infix_type(&mut self, span: &Span, op: &TokenType, lt: &Type, rt: &Type) -> Type {
        use TokenType::*;
        match op {
            Plus | Minus | Star | Slash | Modulo => {
                if lt.is_numeric() && rt.is_numeric() {
                    lt.clone()
                } else {
                    Type::Unknown
                }
            }
            Equals | NotEquals | LessThan | GreaterThan | LessThanEqual | GreaterThanEqual => {
                Type::Bool
            }
            LogicalAnd | LogicalOr => {
                if *lt == Type::Bool && *rt == Type::Bool {
                    Type::Bool
                } else {
                    Type::Unknown
                }
            }
            _ => Type::Unknown,
        }
    }

    fn resolve_type_annotation_name(&self, ann: &crate::ast::TypeExpr) -> Type {
        match ann {
            crate::ast::TypeExpr::Identifier(id) => match id.0.as_str() {
                "int" | "i64" => Type::Int(IntWidth::I64),
                "i32" => Type::Int(IntWidth::I32),
                "i8" => Type::Int(IntWidth::I8),
                "u32" => Type::Int(IntWidth::U32),
                "u64" => Type::Int(IntWidth::U64),
                "f32" => Type::Float(FloatWidth::F32),
                "f64" => Type::Float(FloatWidth::F64),
                "bool" => Type::Bool,
                "char" => Type::Char,
                "str" => Type::Str,
                "String" => Type::String,
                "unit" | "()" => Type::Unit,
                other => Type::Generic(other.to_string(), vec![]),
            },
            crate::ast::TypeExpr::Quantum(inner) => {
                Type::Quantum(Box::new(self.resolve_type_annotation_name(inner)))
            }
            crate::ast::TypeExpr::Nano(inner) => {
                Type::Nano(Box::new(self.resolve_type_annotation_name(inner)))
            }
            crate::ast::TypeExpr::MTS(inner) => {
                Type::MTS(Box::new(self.resolve_type_annotation_name(inner)))
            }
            crate::ast::TypeExpr::Sankofa(inner) => {
                Type::Sankofa(Box::new(self.resolve_type_annotation_name(inner)))
            }
            _ => Type::Unknown,
        }
    }

    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        expected == actual
            || *actual == Type::Unknown
            || *expected == Type::Unknown
            || actual.is_error()
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// Import TokenType for infix checking
use crate::lexer::TokenType;
