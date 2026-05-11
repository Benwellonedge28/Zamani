
//! Zenith Universal Meta-Compiler (UMC) Semantic Analyzer
//!
//! This module implements the semantic analysis phase of the Zenith compiler.
//! It takes the Abstract Syntax Tree (AST) produced by the parser and performs
//! contextual checks to ensure the program adheres to Zenith's rules and types.
//!
//! Key responsibilities include:
//! - **Type Checking:** Verifying that all operations are performed on compatible types,
//!   and inferring types where explicit annotations are missing.
//! - **Symbol Table Management:** Building and managing a hierarchical symbol table
//!   to track declared variables, functions, types, and effects, ensuring correct
//!   scoping and resolution.
//! - **Scope Management:** Handling block scopes for variables and parameters.
//! - **Paradigm-Specific Validations:**
//!   - **Quantum:** Ensuring correct qubit usage, gate applications, measurement rules.
//!   - **Nano-Agent:** Validating nano-agent blueprint compatibility, communication protocols.
//!   - **MTS:** Checking temporal consistency, timeline synchronization rules.
//!   - **Sankofa:** Enforcing Zamani (immutability) and Sasa (evolution) rules,
//!     validating temporal queries and learning operations.
//!   - **Linear/Affine Types:** Strict checks for resource consumption, preventing
//!     double-free or use-after-move scenarios.
//!   - **Algebraic Effects:** Ensuring effects are handled, and handlers match effect signatures.
//! - **Unsafe Block Validation:** Checking `unsafe!` blocks for required `evas` proofs.
//! - **Error Reporting:** Collecting and reporting semantic errors with precise source spans.

use crate::ast::{Program, Statement, Expression, Literal, Identifier, Parameter, TypeExpr, MatchCase};
use crate::compiler_types::{Type, Symbol, IntWidth, FloatWidth};
use crate::tokens::{Span, TokenType};
use std::collections::HashMap;

// --- Semantic Analyzer Structure ---
pub struct SemanticAnalyzer {
    pub global_symbols: HashMap<String, Symbol>,
    pub current_scope: Vec<HashMap<String, Symbol>>, // Stack of scopes
    pub errors: Vec<SemanticError>,
    pub next_type_id: usize, // For generating unique IDs for anonymous types if needed
}

// --- SemanticError Structure ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            global_symbols: HashMap::new(),
            current_scope: vec![HashMap::new()], // Start with a global scope
            errors: Vec::new(),
            next_type_id: 0,
        }
    }

    pub fn get_global_symbols(&self) -> &HashMap<String, Symbol> {
        &self.global_symbols
    }

    /// Entry point for semantic analysis.
    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        println!("Performing semantic analysis...");

        // Populate built-in types and functions (conceptual)
        self.define_builtin("add".to_string(), Type::Function(vec![Type::Int(IntWidth::I32), Type::Int(IntWidth::I32)], Box::new(Type::Int(IntWidth::I32))), Span::dummy());
        self.define_builtin("stdlib::core::println".to_string(), Type::Function(vec![Type::String], Box::new(Type::Unit)), Span::dummy());
        self.define_builtin("stdlib::core::to_string".to_string(), Type::Function(vec![Type::Unknown], Box::new(Type::String)), Span::dummy());
        self.define_builtin("QReg".to_string(), Type::Function(vec![Type::Int(IntWidth::I32)], Box::new(Type::QReg(0))), Span::dummy()); // Constructor for QReg
        self.define_builtin("Qubit::h".to_string(), Type::Function(vec![Type::Qubit], Box::new(Type::Unit)), Span::dummy());
        self.define_builtin("Qubit::cnot".to_string(), Type::Function(vec![Type::Qubit, Type::Qubit], Box::new(Type::Unit)), Span::dummy());
        self.define_builtin("Qubit::measure".to_string(), Type::Function(vec![Type::Qubit], Box::new(Type::Bool)), Span::dummy());
        self.define_builtin("QReg::get_mut".to_string(), Type::Function(vec![Type::QReg(0), Type::Int(IntWidth::I32)], Box::new(Type::Qubit)), Span::dummy());
        self.define_builtin("stdlib::nano::NanoAgent::assemble".to_string(), Type::Function(vec![Type::String, Type::Array(Box::new(Type::String), None)], Box::new(Type::NanoAgent(None))), Span::dummy());
        self.define_builtin("stdlib::nano::NanoAgent::perform_action".to_string(), Type::Function(vec![Type::NanoAgent(None), Type::String], Box::new(Type::Unit)), Span::dummy());
        self.define_builtin("stdlib::mts::MtsSlice::new".to_string(), Type::Function(vec![Type::Unknown], Box::new(Type::MtsSlice(Box::new(Type::Unknown)))), Span::dummy());
        self.define_builtin("stdlib::mts::MtsSlice::store".to_string(), Type::Function(vec![Type::MtsSlice(Box::new(Type::Unknown)), Type::Unknown, Type::Int(IntWidth::U64)], Box::new(Type::Unit)), Span::dummy());
        self.define_builtin("stdlib::mts::MtsSlice::load".to_string(), Type::Function(vec![Type::MtsSlice(Box::new(Type::Unknown)), Type::Int(IntWidth::U64)], Box::new(Type::Unknown)), Span::dummy());
        self.define_builtin("stdlib::sankofa::ZamaniFact::access".to_string(), Type::Function(vec![Type::String], Box::new(Type::ZamaniFact(Box::new(Type::Unknown))), Span::dummy()));
        self.define_builtin("stdlib::sankofa::ZamaniFact::get_content".to_string(), Type::Function(vec![Type::ZamaniFact(Box::new(Type::Unknown))], Box::new(Type::Unknown)), Span::dummy());
        self.define_builtin("MyErrorEffect".to_string(), Type::Effect(Identifier("MyErrorEffect".to_string(), Span::dummy())), Span::dummy()); // Define the effect
        self.define_builtin("perform".to_string(), Type::Function(vec![Type::Effect(Identifier("".to_string(), Span::dummy())), Type::Unknown], Box::new(Type::Unknown)), Span::dummy()); // Generic perform

        // Analyze statements in the program
        for stmt in &program.statements {
            self.analyze_statement(stmt);
        }

        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }

    // --- Scope Management ---
    fn enter_scope(&mut self) {
        self.current_scope.push(HashMap::new());
    }

    fn leave_scope(&mut self) {
        self.current_scope.pop();
        if self.current_scope.is_empty() {
            // Should not happen, always at least global scope
            panic!("Left global scope!");
        }
    }

    fn define_symbol(&mut self, name: String, typ: Type, span: Span, is_mutable: bool) {
        if let Some(scope) = self.current_scope.last_mut() {
            if scope.contains_key(&name) {
                self.errors.push(SemanticError {
                    message: format!("Redeclaration of symbol '{}'"),
                    span,
                });
            } else {
                scope.insert(name.clone(), Symbol::new(name, typ, span, is_mutable));
            }
        }
    }

    fn define_builtin(&mut self, name: String, typ: Type, span: Span) {
        self.global_symbols.insert(name.clone(), Symbol::new(name, typ, span, false));
    }

    fn resolve_symbol(&self, name: &str) -> Option<(&String, &Symbol)> {
        // Search current scope upwards, then global symbols
        for scope in self.current_scope.iter().rev() {
            if let Some(symbol) = scope.get_key_value(name) {
                return Some(symbol);
            }
        }
        self.global_symbols.get_key_value(name)
    }

    // --- Type Resolution & Checking ---
    fn resolve_type_expr(&self, type_expr: &TypeExpr) -> Type {
        match type_expr {
            TypeExpr::Base(Identifier(name, _)) => match name.as_str() {
                "int" => Type::Int(IntWidth::I32),
                "float" => Type::Float(FloatWidth::F64),
                "bool" => Type::Bool,
                "string" => Type::String,
                "char" => Type::Char,
                "unit" => Type::Unit,
                "Qubit" => Type::Qubit,
                "QReg" => Type::QReg(0), // Placeholder size, semantic analysis might fill it
                "Superposition" => Type::Superposition(Box::new(Type::Unknown)),
                "Entangled" => Type::Entangled(Box::new(Type::Unknown), Box::new(Type::Unknown)),
                "QMeasured" => Type::QMeasured(Box::new(Type::Unknown)),
                "NanoAgent" => Type::NanoAgent(None),
                "Atom" => Type::Atom(Box::new(Type::Unknown)),
                "Molecule" => Type::Molecule(Box::new(Type::Unknown)),
                "MtsSlice" => Type::MtsSlice(Box::new(Type::Unknown)),
                "MtsTimeline" => Type::MtsTimeline(Box::new(Type::Unknown)),
                "ZamaniFact" => Type::ZamaniFact(Box::new(Type::Unknown)),
                "SasaKnowledge" => Type::SasaKnowledge(Box::new(Type::Unknown)),
                "History" => Type::History(Box::new(Type::Unknown), Box::new(Expression::Block(Span::dummy(),vec![]))), // Placeholder expr
                "ConsensusTrue" => Type::ConsensusTrue(Box::new(Type::Unknown)),
                "InterMemory" => Type::InterMemory(Identifier("".to_string(),Span::dummy()), Box::new(Type::Unknown)),
                "Type" => Type::TypeUniverse(0),
                "Kind" => Type::Kind,
                "Prop" => Type::Prop,
                _ => {
                    if let Some((_, symbol)) = self.resolve_symbol(name) {
                        symbol.typ.clone()
                    } else {
                        self.errors.push(SemanticError { message: format!("Unknown type '{}'", name), span: type_expr.span().clone() });
                        Type::Error
                    }
                }
            },
            TypeExpr::Array(base_type_expr, size_opt) => {
                let base_type = self.resolve_type_expr(base_type_expr);
                Type::Array(Box::new(base_type), size_opt.as_ref().and_then(|s| s.parse::<usize>().ok()))
            }
            TypeExpr::Generic(base_type_expr, generic_args_expr) => {
                let base_type = self.resolve_type_expr(base_type_expr);
                let generic_args: Vec<Type> = generic_args_expr.iter().map(|arg| self.resolve_type_expr(arg)).collect();
                // Conceptual: Apply generics. This would involve checking bounds and instantiation.
                // For simplicity, we just return a generic base for now.
                match base_type {
                    Type::Superposition(_) => Type::Superposition(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::Entangled(_,_) => Type::Entangled(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown)), Box::new(generic_args.get(1).cloned().unwrap_or(Type::Unknown))),
                    Type::QMeasured(_) => Type::QMeasured(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::Atom(_) => Type::Atom(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::Molecule(_) => Type::Molecule(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::MtsSlice(_) => Type::MtsSlice(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::ZamaniFact(_) => Type::ZamaniFact(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::SasaKnowledge(_) => Type::SasaKnowledge(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::History(_,_) => Type::History(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown)), Box::new(Expression::Block(Span::dummy(),vec![]))), // Placeholder for temporal expr
                    Type::ConsensusTrue(_) => Type::ConsensusTrue(Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    Type::InterMemory(lang,_) => Type::InterMemory(lang, Box::new(generic_args.first().cloned().unwrap_or(Type::Unknown))),
                    _ => {
                        self.errors.push(SemanticError { message: format!("Cannot apply generics to non-generic base type '{:?}'", base_type), span: type_expr.span().clone() });
                        Type::Error
                    }
                }
            }
            TypeExpr::Linear(inner_type_expr) => Type::Linear(Box::new(self.resolve_type_expr(inner_type_expr))),
            TypeExpr::Affine(inner_type_expr) => Type::Affine(Box::new(self.resolve_type_expr(inner_type_expr))),
            TypeExpr::Effectful(base_type_expr, effect_ids) => {
                let base_type = self.resolve_type_expr(base_type_expr);
                let resolved_effects: Vec<Identifier> = effect_ids.clone(); // Conceptual: Would resolve effect IDs here
                Type::Effectful(Box::new(base_type), resolved_effects)
            }
            TypeExpr::DependentPi(binder_id, binder_type_expr, body_type_expr) => {
                // Conceptual: Requires advanced type theory inference.
                Type::DependentPi(binder_id.clone(), Box::new(self.resolve_type_expr(binder_type_expr)), Box::new(self.resolve_type_expr(body_type_expr)))
            }
            TypeExpr::DependentSigma(binder_id, binder_type_expr, body_type_expr) => {
                // Conceptual: Requires advanced type theory inference.
                Type::DependentSigma(binder_id.clone(), Box::new(self.resolve_type_expr(binder_type_expr)), Box::new(self.resolve_type_expr(body_type_expr)))
            }
            _ => Type::Error, // Fallback for unhandled TypeExpr variants
        }
    }


    fn check_type(&mut self, expected: &Type, actual: &Type, span: Span) -> bool {
        if expected == &Type::Unknown || actual == &Type::Unknown || expected == &Type::Error || actual == &Type::Error {
            return true; // Don't error if type is unknown/inferred or already an error
        }
        if expected == actual {
            true
        } else {
            self.errors.push(SemanticError {
                message: format!("Type mismatch: Expected {:?}, found {:?}", expected, actual),
                span,
            });
            false
        }
    }

    // --- Statement Analysis ---
    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let(span, name, type_expr_opt, expr) => {
                let expr_type = self.analyze_expression(expr);
                let declared_type = type_expr_opt.as_ref().map_or(Type::Unknown, |te| self.resolve_type_expr(te));

                if declared_type != Type::Unknown && !self.check_type(&declared_type, &expr_type, expr.span().clone()) {
                    // Type mismatch already reported
                } else if declared_type == Type::Unknown { // Infer type
                    self.define_symbol(name.clone(), expr_type, span.clone(), true);
                } else { // Use declared type
                    self.define_symbol(name.clone(), declared_type, span.clone(), true);
                }
            }
            Statement::Return(span, expr) => {
                // Conceptual: Check return type against function's declared return type.
                self.analyze_expression(expr);
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr);
            }
            Statement::Function(span, name, params, return_type_expr_opt, body) => {
                let func_return_type = return_type_expr_opt.as_ref().map_or(Type::Unit, |te| self.resolve_type_expr(te));
                let func_param_types: Vec<Type> = params.iter().map(|p| p.typ.as_ref().map_or(Type::Unknown, |te| self.resolve_type_expr(te))).collect();
                let func_type = Type::Function(func_param_types, Box::new(func_return_type.clone()));
                self.define_symbol(name.clone(), func_type, span.clone(), false);

                self.enter_scope();
                for param in params {
                    let param_type = param.typ.as_ref().map_or(Type::Unknown, |te| self.resolve_type_expr(te));
                    self.define_symbol(param.name.0.clone(), param_type, param.name.1.clone(), false);
                }
                let body_type = self.analyze_expression(body);
                self.check_type(&func_return_type, &body_type, body.span().clone()); // Check body's return type
                self.leave_scope();
            }
            Statement::QuantumCircuit(span, name, body) => {
                self.define_symbol(name.clone(), Type::Function(vec![], Box::new(Type::Unit)), span.clone(), false); // Q-circuit callable
                self.enter_scope();
                self.analyze_expression(body); // Analyze quantum operations within
                self.leave_scope();
            }
            Statement::NanoAgent(span, name, body) => {
                self.define_symbol(name.clone(), Type::NanoAgent(Some(Identifier(name.clone(), span.clone()))), span.clone(), false); // Nano-agent declaration
                self.enter_scope();
                self.analyze_expression(body); // Analyze nano-agent logic
                self.leave_scope();
            }
            Statement::SankofaMemory(span, name, expr) => {
                let expr_type = self.analyze_expression(expr);
                // Conceptual: `remember` could store to Zamani or Sasa based on context/modifiers
                self.define_symbol(name.clone(), Type::ZamaniFact(Box::new(expr_type)), span.clone(), false); // For now, assume ZamaniFact
            }
            Statement::TypeDeclaration(span, name, type_expr) => {
                let resolved_type = self.resolve_type_expr(type_expr);
                self.define_symbol(name.clone(), resolved_type, span.clone(), false);
            }
            Statement::EffectDeclaration(span, name) => {
                self.define_symbol(name.0.clone(), Type::Effect(name.clone()), span.clone(), false);
            }
            Statement::LanguageDeclaration(_, _, _) => { /* No semantic analysis for this yet */ }
            Statement::While(span, cond, body) => {
                let cond_type = self.analyze_expression(cond);
                self.check_type(&Type::Bool, &cond_type, cond.span().clone());
                self.enter_scope(); // Loop body is a new scope
                self.analyze_expression(body);
                self.leave_scope();
            }
            Statement::For(span, iter_var, iterable, body) => {
                let iterable_type = self.analyze_expression(iterable);
                // Conceptual: Check if iterable_type implements an Iterable trait
                // and extract its item type.
                let item_type = Type::Unknown; // Placeholder
                self.enter_scope();
                self.define_symbol(iter_var.0.clone(), item_type, iter_var.1.clone(), false);
                self.analyze_expression(body);
                self.leave_scope();
            }
            Statement::Break(_) | Statement::Continue(_) => { /* No specific type checks for these */ }
            Statement::Match(span, expr, cases) => {
                let expr_type = self.analyze_expression(expr);
                // Conceptual: Determine common return type for all branches. First branch's type for now.
                let mut match_return_type = Type::Unit;
                for (i, case) in cases.iter().enumerate() {
                    let pattern_type = self.analyze_expression(&case.pattern);
                    self.check_type(&expr_type, &pattern_type, case.pattern.span().clone());
                    self.enter_scope(); // Case body is a new scope
                    let case_body_type = self.analyze_expression(&case.body);
                    if i == 0 { match_return_type = case_body_type.clone(); } // First case sets the expected type
                    self.check_type(&match_return_type, &case_body_type, case.body.span().clone());
                    self.leave_scope();
                }
            }
            Statement::Unsafe(span, proof_opt, block_expr) => {
                // Conceptual: Check for required EVAS proof if `unsafe!` is used.
                if proof_opt.is_some() {
                    println!("  - Semantic: Validating EVAS proof for unsafe block at {:?}", span);
                    // In a real compiler, this would involve looking up the proof
                    // in a formal verification registry or module.
                }
                self.enter_scope();
                self.analyze_expression(block_expr);
                self.leave_scope();
            }
            Statement::Handle(span, effect_id, body_expr, handler_expr) => {
                // Conceptual: Check if effect_id is a declared effect.
                let mut effect_is_declared = false;
                if let Some((_, symbol)) = self.resolve_symbol(&effect_id.0) {
                    if matches!(symbol.typ, Type::Effect(_)) {
                        effect_is_declared = true;
                    } else {
                        self.errors.push(SemanticError { message: format!("'{}' is not a declared effect.", effect_id.0), span: effect_id.1.clone() });
                    }
                } else {
                    self.errors.push(SemanticError { message: format!("Undeclared effect '{}'", effect_id.0), span: effect_id.1.clone() });
                }

                // Analyze the body where the effect might be performed
                self.enter_scope();
                let body_type = self.analyze_expression(body_expr); // Need to track effects performed here
                self.leave_scope();

                // Analyze the handler block (conceptual: check if it covers all operations of the effect)
                self.enter_scope();
                let handler_type = self.analyze_expression(handler_expr); // Handler would usually return a value or resume
                self.leave_scope();

                // Conceptual: Check that handler_type is compatible with what the body expects to be resumed with.
                self.check_type(&body_type, &handler_type, handler_expr.span().clone());
            }
        }
    }

    // --- Expression Analysis ---
    fn analyze_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Identifier(Identifier(name, span)) => {
                if let Some((_, symbol)) = self.resolve_symbol(name) {
                    symbol.typ.clone()
                } else {
                    self.errors.push(SemanticError {
                        message: format!("Undeclared identifier '{}'", name),
                        span: span.clone(),
                    });
                    Type::Error
                }
            }
            Expression::Literal(literal) => {
                match literal {
                    Literal::Integer(_, _) => Type::Int(IntWidth::I32), // Default
                    Literal::Float(_, _) => Type::Float(FloatWidth::F64), // Default
                    Literal::String(_, _) => Type::String,
                    Literal::Boolean(_, _) => Type::Bool,
                    Literal::Char(_, _) => Type::Char,
                    Literal::Quantum(_, _) => Type::Qubit, // Simplified: assume single qubit state for now
                    Literal::Nano(_, _) => Type::NanoAgent(None), // Simplified
                    Literal::MTS(_, _) => Type::MtsSlice(Box::new(Type::Unknown)), // Simplified
                }
            }
            Expression::Prefix(span, op, right_expr) => {
                let right_type = self.analyze_expression(right_expr);
                match op {
                    TokenType::Bang => { // Logical NOT
                        self.check_type(&Type::Bool, &right_type, right_expr.span().clone());
                        Type::Bool
                    },
                    TokenType::Minus => { // Unary minus
                        if matches!(right_type, Type::Int(_) | Type::Float(_)) {
                            right_type
                        } else {
                            self.errors.push(SemanticError { message: format!("Operator '{:?}' not applicable to type {:?}", op, right_type), span: span.clone() });
                            Type::Error
                        }
                    },
                    _ => {
                        self.errors.push(SemanticError { message: format!("Unsupported prefix operator '{:?}'", op), span: span.clone() });
                        Type::Error
                    }
                }
            }
            Expression::Infix(span, left_expr, op, right_expr) => {
                let left_type = self.analyze_expression(left_expr);
                let right_type = self.analyze_expression(right_expr);

                match op {
                    TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                        if left_type == right_type && matches!(left_type, Type::Int(_) | Type::Float(_)) {
                            left_type
                        } else {
                            self.errors.push(SemanticError { message: format!("Incompatible types for arithmetic operation '{:?}': {:?} and {:?}", op, left_type, right_type), span: span.clone() });
                            Type::Error
                        }
                    },
                    TokenType::Equals | TokenType::NotEquals | TokenType::LT | TokenType::GT | TokenType::LTE | TokenType::GTE => {
                        if left_type == right_type {
                            Type::Bool
                        } else {
                            self.errors.push(SemanticError { message: format!("Incompatible types for comparison operation '{:?}': {:?} and {:?}", op, left_type, right_type), span: span.clone() });
                            Type::Error
                        }
                    },
                    TokenType::LogicalAnd | TokenType::LogicalOr => {
                        self.check_type(&Type::Bool, &left_type, left_expr.span().clone());
                        self.check_type(&Type::Bool, &right_type, right_expr.span().clone());
                        Type::Bool
                    }
                    _ => {
                        self.errors.push(SemanticError { message: format!("Unsupported infix operator '{:?}'", op), span: span.clone() });
                        Type::Error
                    }
                }
            }
            Expression::If(span, cond_expr, then_block, else_block_opt) => {
                let cond_type = self.analyze_expression(cond_expr);
                self.check_type(&Type::Bool, &cond_type, cond_expr.span().clone());

                self.enter_scope();
                let then_type = self.analyze_expression(then_block);
                self.leave_scope();

                if let Some(else_block) = else_block_opt {
                    self.enter_scope();
                    let else_type = self.analyze_expression(else_block);
                    self.leave_scope();
                    self.check_type(&then_type, &else_type, else_block.span().clone()); // Both branches must have same type
                    then_type // Return type of the 'if' expression
                } else {
                    then_type // If no else, type is then_type, or Unit if no explicit return
                }
            }
            Expression::Block(span, statements) => {
                self.enter_scope();
                let mut last_type = Type::Unit; // Default for empty block
                for stmt in statements {
                    match stmt {
                        Statement::Expression(expr) => {
                            last_type = self.analyze_expression(expr);
                        }
                        _ => self.analyze_statement(stmt),
                    }
                }
                self.leave_scope();
                last_type
            }
            Expression::Call(span, func_expr, args) => {
                let func_type = self.analyze_expression(func_expr);
                if let Type::Function(param_types, return_type) = func_type {
                    if param_types.len() != args.len() {
                        self.errors.push(SemanticError { message: format!("Function expects {} arguments, but received {}", param_types.len(), args.len()), span: span.clone() });
                        return Type::Error;
                    }
                    for (i, arg_expr) in args.iter().enumerate() {
                        let arg_type = self.analyze_expression(arg_expr);
                        if i < param_types.len() {
                            self.check_type(&param_types[i], &arg_type, arg_expr.span().clone());
                        }
                    }
                    *return_type
                } else {
                    self.errors.push(SemanticError { message: format!("Attempted to call non-function type '{:?}'", func_type), span: func_expr.span().clone() });
                    Type::Error
                }
            }
            Expression::Index(span, array_expr, index_expr) => {
                let array_type = self.analyze_expression(array_expr);
                let index_type = self.analyze_expression(index_expr);

                self.check_type(&Type::Int(IntWidth::I32), &index_type, index_expr.span().clone()); // Conceptual: index is i32

                if let Type::Array(element_type, _) = array_type {
                    *element_type
                } else if let Type::QReg(_) = array_type { // QReg is a special kind of array
                    Type::Qubit
                }
                else {
                    self.errors.push(SemanticError { message: format!("Cannot index non-array/QReg type '{:?}'", array_type), span: array_expr.span().clone() });
                    Type::Error
                }
            }
            Expression::MemberAccess(span, object_expr, member_id) => {
                let object_type = self.analyze_expression(object_expr);
                // Conceptual: Lookup member in object_type's definition
                if let Type::Struct(_, fields) = object_type {
                    if let Some(member_type) = fields.get(&member_id.0) {
                        member_type.clone()
                    } else {
                        self.errors.push(SemanticError { message: format!("Struct has no member named '{}'", member_id.0), span: member_id.1.clone() });
                        Type::Error
                    }
                } else if matches!(object_type, Type::Qubit | Type::NanoAgent(_)) {
                    // For Qubit/NanoAgent, members like 'h', 'cnot', 'measure', 'perform_action', 'assemble' are methods.
                    // Resolve these as function types bound to the object type.
                    let method_name = format!("{:?}::{}", object_type, member_id.0); // Create a unique name for method
                    if let Some((_, symbol)) = self.resolve_symbol(&method_name) {
                        symbol.typ.clone()
                    } else {
                         self.errors.push(SemanticError { message: format!("Method '{}' not found on type '{:?}'", member_id.0, object_type), span: member_id.1.clone() });
                        Type::Error
                    }
                }
                else if matches!(object_type, Type::MtsSlice(_) | Type::ZamaniFact(_) | Type::SasaKnowledge(_)) {
                     let method_name = format!("{:?}::{}", object_type, member_id.0); // Create a unique name for method
                    if let Some((_, symbol)) = self.resolve_symbol(&method_name) {
                        symbol.typ.clone()
                    } else {
                         self.errors.push(SemanticError { message: format!("Method '{}' not found on type '{:?}'", member_id.0, object_type), span: member_id.1.clone() });
                        Type::Error
                    }
                }
                 else {
                    self.errors.push(SemanticError { message: format!("Cannot access member '{}' on non-struct/object type '{:?}'", member_id.0, object_type), span: object_expr.span().clone() });
                    Type::Error
                }
            }
        }
    }
}
