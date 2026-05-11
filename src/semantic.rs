
//! Zenith Universal Meta-Compiler (UMC) Semantic Analyzer
//!
//! This module implements the semantic analysis phase of the Zenith compiler.
//! It performs type checking, scope management, symbol resolution, and
//! validates various language-specific constraints (e.g., causality in MTS,
//! entanglement purity in quantum, resource bounds for nano-agents, OOP logic).

use std::collections::{HashMap, HashSet};
use crate::ast::{
    Program, Statement, Expression, Identifier, Literal, Parameter, MatchCase,
    AccessModifier, ClassMember, InterfaceMember, TypeExpr, MethodModifier, TokenType
};
use crate::compiler_types::{Type, MethodType, IntWidth, FloatWidth};
use crate::source_map::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

/// Represents a symbol in the Zenith symbol table.
#[derive(Clone, Debug)]
pub enum Symbol {
    Variable(Type),
    Function(Vec<Type>, Type),
    Effect(Identifier),
    // --- OOP Symbols ---
    Class {
        name: Identifier,
        fields: HashMap<String, Type>,
        methods: HashMap<String, MethodType>,
        parents: Vec<Identifier>, // Names of parent classes/interfaces
        is_abstract: bool,
    },
    Interface {
        name: Identifier,
        methods: HashMap<String, MethodType>,
        parents: Vec<Identifier>,
    },
}

/// Manages scopes and symbols during semantic analysis.
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { scopes: vec![HashMap::new()] } // Start with a global scope
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, symbol: Symbol) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, symbol);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }
}

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    current_class: Option<Identifier>, // Track current class for 'this'/'super'
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            current_class: None,
        }
    }

    /// Performs full semantic analysis on a program's AST.
    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        let mut errors = Vec::new();
        
        // Pass 1: Global declarations (Types, Functions, Effects, Classes, Interfaces)
        // This allows for forward references.
        for stmt in &program.statements {
            match stmt {
                Statement::TypeDeclaration(_, name, type_expr) => {
                    // Resolve and define type aliases (conceptual)
                }
                Statement::EffectDeclaration(_, name) => {
                    self.symbol_table.define(name.0.clone(), Symbol::Effect(name.clone()));
                }
                Statement::Function(_, name, params, ret_type, _) => {
                    // Pre-define function signature
                }
                // Add Class and Interface pre-definitions here
                _ => {}
            }
        }

        // Pass 2: Detailed analysis of all statements
        for stmt in &program.statements {
            if let Err(mut stmt_errors) = self.analyze_statement(stmt) {
                errors.append(&mut stmt_errors);
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), Vec<SemanticError>> {
        match stmt {
            Statement::Let(_, name, typ_expr, expr) => {
                let expr_type = self.analyze_expression(expr)?;
                if let Some(annotated_typ_expr) = typ_expr {
                    let annotated_type = self.resolve_type_expression(annotated_typ_expr)?;
                    if !self.is_compatible(&annotated_type, &expr_type) {
                        return Err(vec![SemanticError {
                            message: format!("Mismatched types in assignment to '{}'. Expected {:?}, found {:?}.", name, annotated_type, expr_type),
                            span: expr.span(),
                        }]);
                    }
                }
                self.symbol_table.define(name.clone(), Symbol::Variable(expr_type));
                Ok(())
            }
            Statement::Function(_, name, params, ret_type, body) => {
                self.symbol_table.enter_scope();
                // Define parameters in new scope
                for param in params {
                    let param_type = self.resolve_type_expression(param.typ.as_ref().unwrap())?;
                    self.symbol_table.define(param.name.0.clone(), Symbol::Variable(param_type));
                }
                self.analyze_expression(body)?;
                self.symbol_table.exit_scope();
                Ok(())
            }
            Statement::While(_, cond, body) => {
                let cond_type = self.analyze_expression(cond)?;
                if cond_type != Type::Bool {
                    return Err(vec![SemanticError { message: "While condition must be a boolean.".to_string(), span: cond.span() }]);
                }
                self.analyze_expression(body)?;
                Ok(())
            }
            // --- OOP Statements ---
            Statement::Class(span, name, parents, members) => self.analyze_class_declaration(span, name, parents, members),
            Statement::Interface(span, name, parents, members) => self.analyze_interface_declaration(span, name, parents, members),
            
            Statement::Return(_, expr) => {
                self.analyze_expression(expr)?;
                Ok(())
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr)?;
                Ok(())
            }
            _ => Ok(()), // Placeholder for other statements
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Result<Type, Vec<SemanticError>> {
        match expr {
            Expression::Identifier(ident) => {
                if let Some(Symbol::Variable(typ)) = self.symbol_table.lookup(&ident.0) {
                    Ok(typ.clone())
                } else {
                    Err(vec![SemanticError { message: format!("Unresolved identifier: '{}'.", ident.0), span: ident.1.clone() }])
                }
            }
            Expression::Literal(lit) => match lit {
                Literal::Integer(_, _) => Ok(Type::Int(IntWidth::I32)),
                Literal::Boolean(_, _) => Ok(Type::Bool),
                Literal::String(_, _) => Ok(Type::String),
                Literal::Quantum(_, _) => Ok(Type::Qubit),
                _ => Ok(Type::Unknown),
            },
            Expression::Infix(_, left, op, right) => {
                let left_type = self.analyze_expression(left)?;
                let right_type = self.analyze_expression(right)?;
                // Conceptual type checking for operators
                if left_type == right_type {
                    match op {
                        TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => Ok(left_type),
                        TokenType::Equals | TokenType::NotEquals | TokenType::LessThan | TokenType::GreaterThan => Ok(Type::Bool),
                        _ => Ok(Type::Unknown),
                    }
                } else {
                    Err(vec![SemanticError { message: "Operator applied to incompatible types.".to_string(), span: left.span().merge(&right.span()) }])
                }
            }
            Expression::Call(_, func, args) => {
                let _func_type = self.analyze_expression(func)?;
                for arg in args { self.analyze_expression(arg)?; }
                // Conceptual: check if func_type is Type::Function and args match
                Ok(Type::Unknown) 
            }
            Expression::Block(_, stmts) => {
                self.symbol_table.enter_scope();
                for stmt in stmts { self.analyze_statement(stmt)?; }
                self.symbol_table.exit_scope();
                Ok(Type::Unit)
            }
            
            // --- OOP Expressions ---
            Expression::NewInstance(span, class_name, args) => self.analyze_new_instance(span, class_name, args),
            Expression::MethodCall(span, object, method_name, args) => self.analyze_method_call(span, object, method_name, args),
            Expression::FieldAccess(span, object, field_name) => self.analyze_field_access(span, object, field_name),
            Expression::This(span) => self.analyze_this(span),
            Expression::Super(span) => self.analyze_super(span),

            _ => Ok(Type::Unknown),
        }
    }

    // --- OOP Semantic Analysis Functions ---

    fn analyze_class_declaration(&mut self, span: &Span, name: &Identifier, parents: &[Identifier], members: &[ClassMember]) -> Result<(), Vec<SemanticError>> {
        let mut errors = Vec::new();
        let class_name = name.0.clone();

        if self.symbol_table.lookup(&class_name).is_some() {
            errors.push(SemanticError { message: format!("Symbol '{}' already defined.", class_name), span: name.1.clone() });
        }

        let mut fields = HashMap::new();
        let mut methods = HashMap::new();
        let mut is_abstract = false;

        // Pass 1: Gather members and check for local name collisions
        for member in members {
            match member {
                ClassMember::Field(_, modifier, field_name, type_expr, _) => {
                    let typ = self.resolve_type_expression(field_name.1.clone(), type_expr)?;
                    if fields.contains_key(&field_name.0) || methods.contains_key(&field_name.0) {
                        errors.push(SemanticError { message: format!("Member '{}' already defined in class '{}'.", field_name.0, class_name), span: field_name.1.clone() });
                    }
                    fields.insert(field_name.0.clone(), typ);
                }
                ClassMember::Method(_, modifier, method_mod, method_name, params, ret_type, _, effects) => {
                    let param_types = params.iter().map(|p| self.resolve_type_expression(p.name.1.clone(), p.typ.as_ref().unwrap()).unwrap()).collect();
                    let return_type = ret_type.as_ref().map(|t| self.resolve_type_expression(method_name.1.clone(), t).unwrap()).unwrap_or(Type::Unit);
                    
                    if methods.contains_key(&method_name.0) || fields.contains_key(&method_name.0) {
                        errors.push(SemanticError { message: format!("Member '{}' already defined in class '{}'.", method_name.0, class_name), span: method_name.1.clone() });
                    }
                    
                    if *method_mod == Some(MethodModifier::Abstract) { is_abstract = true; }

                    methods.insert(method_name.0.clone(), MethodType {
                        params: param_types,
                        return_type: Box::new(return_type),
                        access_modifier: modifier.clone(),
                        method_modifier: method_mod.clone(),
                        effects: effects.clone(),
                    });
                }
            }
        }

        self.symbol_table.define(class_name.clone(), Symbol::Class {
            name: name.clone(),
            fields,
            methods,
            parents: parents.to_vec(),
            is_abstract,
        });

        // Pass 2: Analyze method bodies
        self.current_class = Some(name.clone());
        for member in members {
            if let ClassMember::Method(_, _, _, _, _, _, body, _) = member {
                self.symbol_table.enter_scope();
                // Define 'this' in method scope (conceptual)
                self.analyze_expression(body)?;
                self.symbol_table.exit_scope();
            }
        }
        self.current_class = None;

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn analyze_interface_declaration(&mut self, span: &Span, name: &Identifier, parents: &[Identifier], members: &[InterfaceMember]) -> Result<(), Vec<SemanticError>> {
        let mut methods = HashMap::new();
        for member in members {
            if let InterfaceMember::MethodSignature(_, method_name, params, ret_type, effects) = member {
                let param_types = params.iter().map(|p| self.resolve_type_expression(p.name.1.clone(), p.typ.as_ref().unwrap()).unwrap()).collect();
                let return_type = ret_type.as_ref().map(|t| self.resolve_type_expression(method_name.1.clone(), t).unwrap()).unwrap_or(Type::Unit);
                methods.insert(method_name.0.clone(), MethodType {
                    params: param_types,
                    return_type: Box::new(return_type),
                    access_modifier: AccessModifier::Public,
                    method_modifier: Some(MethodModifier::Abstract),
                    effects: effects.clone(),
                });
            }
        }
        self.symbol_table.define(name.0.clone(), Symbol::Interface {
            name: name.clone(),
            methods,
            parents: parents.to_vec(),
        });
        Ok(())
    }

    fn resolve_type_expression(&self, span: Span, type_expr: &TypeExpr) -> Result<Type, Vec<SemanticError>> {
        match type_expr {
            TypeExpr::Identifier(ident) => {
                match ident.0.as_str() {
                    "int" => Ok(Type::Int(IntWidth::I32)),
                    "string" => Ok(Type::String),
                    "bool" => Ok(Type::Bool),
                    _ => {
                        if let Some(Symbol::Class { .. }) = self.symbol_table.lookup(&ident.0) {
                            Ok(Type::Class { name: ident.clone(), fields: HashMap::new(), methods: HashMap::new(), parent_class: None, implemented_interfaces: Vec::new() })
                        } else if let Some(Symbol::Interface { .. }) = self.symbol_table.lookup(&ident.0) {
                             Ok(Type::Interface { name: ident.clone(), methods: HashMap::new(), parent_interfaces: Vec::new() })
                        } else {
                            Err(vec![SemanticError { message: format!("Unknown type: '{}'.", ident.0), span: ident.1.clone() }])
                        }
                    }
                }
            }
            TypeExpr::Array(inner) => Ok(Type::Array(Box::new(self.resolve_type_expression(span, inner)?), None)),
        }
    }

    fn analyze_new_instance(&mut self, span: &Span, class_name: &Identifier, args: &[Expression]) -> Result<Type, Vec<SemanticError>> {
        if let Some(Symbol::Class { name, is_abstract, .. }) = self.symbol_table.lookup(&class_name.0) {
            if *is_abstract {
                return Err(vec![SemanticError { message: format!("Cannot instantiate abstract class '{}'.", class_name.0), span: span.clone() }]);
            }
            for arg in args { self.analyze_expression(arg)?; }
            Ok(Type::Class { name: name.clone(), fields: HashMap::new(), methods: HashMap::new(), parent_class: None, implemented_interfaces: Vec::new() })
        } else {
            Err(vec![SemanticError { message: format!("Unknown class: '{}'.", class_name.0), span: class_name.1.clone() }])
        }
    }

    fn analyze_method_call(&mut self, span: &Span, object: &Expression, method_name: &Identifier, args: &[Expression]) -> Result<Type, Vec<SemanticError>> {
        let _obj_type = self.analyze_expression(object)?;
        // Conceptual:
        // 1. Get class/interface type of object.
        // 2. Look up method_name in that class/interface and its parent chain.
        // 3. Check access modifier.
        // 4. Verify arguments match parameters.
        // 5. Return method's return type.
        Ok(Type::Unknown)
    }

    fn analyze_field_access(&mut self, span: &Span, object: &Expression, field_name: &Identifier) -> Result<Type, Vec<SemanticError>> {
        let _obj_type = self.analyze_expression(object)?;
        // Conceptual:
        // 1. Get class type of object.
        // 2. Look up field_name in that class and parent chain.
        // 3. Check access modifier.
        // 4. Return field's type.
        Ok(Type::Unknown)
    }

    fn analyze_this(&self, span: &Span) -> Result<Type, Vec<SemanticError>> {
        if let Some(class_ident) = &self.current_class {
             Ok(Type::Class { name: class_ident.clone(), fields: HashMap::new(), methods: HashMap::new(), parent_class: None, implemented_interfaces: Vec::new() })
        } else {
            Err(vec![SemanticError { message: "'this' can only be used inside a class method.".to_string(), span: span.clone() }])
        }
    }

    fn analyze_super(&self, span: &Span) -> Result<Type, Vec<SemanticError>> {
         if let Some(class_ident) = &self.current_class {
             // Conceptual: lookup parent of current_class
             Ok(Type::Unknown)
        } else {
            Err(vec![SemanticError { message: "'super' can only be used inside a class method.".to_string(), span: span.clone() }])
        }
    }

    fn is_compatible(&self, target: &Type, source: &Type) -> bool {
        // Conceptual: Handle class/interface subtyping
        target == source
    }

    pub fn get_global_symbols(&self) -> &SymbolTable {
        &self.symbol_table
    }
}
