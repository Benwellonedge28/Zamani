//! Zenith Semantic Analyzer
//!
//! This module implements the semantic analysis phase of the Zenith compiler.
//! It takes the Abstract Syntax Tree (AST) from the parser and performs checks
//! that go beyond the grammatical structure. This includes type checking, scope
//! resolution, and enforcing language-specific semantic rules, including Zenith's
//! unique paradigms like quantum entanglement, linear types, nano-agent constraints,
//! and Sankofa temporal memory rules.

use crate::ast::{Program, Statement, Expression, Literal, Identifier, TypeExpr, Parameter, MatchCase};
use crate::compiler_types::{Type, Symbol, Environment, EvasPolicy, Constraint, SymbolKind};
use crate::source_map::Span; // Corrected Span import
use std::collections::{HashMap, VecDeque};

/// Represents a semantic error.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

/// The main semantic analyzer structure.
pub struct SemanticAnalyzer {
    /// The global environment, including built-in types and functions.
    pub global_env: Environment,
    /// A stack of local environments for scope management.
    pub scope_stack: VecDeque<Environment>,
    /// Collected semantic errors.
    errors: Vec<SemanticError>,
    /// The active EVAS policy for the current compilation unit.
    evas_policy: EvasPolicy,
}

impl SemanticAnalyzer {
    /// Creates a new SemanticAnalyzer instance.
    pub fn new() -> Self {
        let mut analyzer = SemanticAnalyzer {
            global_env: Environment::new_global(),
            scope_stack: VecDeque::new(),
            errors: Vec::new(),
            evas_policy: EvasPolicy::default(),
        };
        analyzer.enter_scope(); // Enter global scope
        analyzer.define_builtins();
        analyzer
    }

    /// Defines built-in types, functions, and effects.
    fn define_builtins(&mut self) {
        // Built-in types
        self.define_type("int".to_string(), Type::Int, Span::dummy());
        self.define_type("float".to_string(), Type::Float, Span::dummy());
        self.define_type("bool".to_string(), Type::Bool, Span::dummy());
        self.define_type("char".to_string(), Type::Char, Span::dummy());
        self.define_type("string".to_string(), Type::String, Span::dummy());
        self.define_type("unit".to_string(), Type::Unit, Span::dummy());
        self.define_type("Qubit".to_string(), Type::Qubit, Span::dummy());
        self.define_type("NanoParticle".to_string(), Type::NanoParticle, Span::dummy());

        // Built-in effects (conceptual)
        self.define_effect("Read".to_string(), Span::dummy());
        self.define_effect("Write".to_string(), Span::dummy());

        // Built-in functions (conceptual)
        self.define_function(
            "print".to_string(),
            vec![Type::String], // Takes a string
            Type::Unit, // Returns Unit
            Span::dummy(),
        );
        self.define_function(
            "Hadamard".to_string(),
            vec![Type::Qubit], // Takes a Qubit
            Type::Qubit, // Returns a Qubit
            Span::dummy(),
        );
    }

    /// Enters a new scope by pushing a new environment onto the stack.
    fn enter_scope(&mut self) {
        self.scope_stack.push_back(Environment::new_local(self.current_scope().map(|e| e.id)));
    }

    /// Exits the current scope by popping an environment from the stack.
    fn exit_scope(&mut self) {
        self.scope_stack.pop_back();
    }

    /// Gets a mutable reference to the current (innermost) scope's environment.
    fn current_scope(&mut self) -> Option<&mut Environment> {
        self.scope_stack.back_mut()
    }

    /// Looks up a symbol in the current and enclosing scopes.
    fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
        }
        self.global_env.symbols.get(name)
    }

    /// Defines a variable in the current scope.
    fn define_variable(&mut self, name: String, symbol_type: Type, is_mutable: bool, span: Span) {
        if let Some(env) = self.scope_stack.back_mut() {
            if env.symbols.contains_key(&name) {
                self.add_error(format!("Redefinition of variable '{}'.", name), span);
            } else {
                env.symbols.insert(
                    name.clone(),
                    Symbol { name, symbol_type, is_mutable, span, kind: SymbolKind::Variable },
                );
            }
        } else {
            self.add_error("Cannot define variable outside any scope.".to_string(), span);
        }
    }

    /// Defines a function in the current scope.
    fn define_function(&mut self, name: String, param_types: Vec<Type>, return_type: Type, span: Span) {
        let func_type = Type::Function(param_types, Box::new(return_type));
        if let Some(env) = self.scope_stack.back_mut() {
            if env.symbols.contains_key(&name) {
                self.add_error(format!("Redefinition of function '{}'.", name), span);
            } else {
                env.symbols.insert(
                    name.clone(),
                    Symbol { name, symbol_type: func_type, is_mutable: false, span, kind: SymbolKind::Function },
                );
            }
        } else {
            self.add_error("Cannot define function outside any scope.".to_string(), span);
        }
    }

    /// Defines a type alias in the current scope.
    fn define_type(&mut self, name: String, aliased_type: Type, span: Span) {
        if let Some(env) = self.scope_stack.back_mut() {
            if env.symbols.contains_key(&name) {
                self.add_error(format!("Redefinition of type '{}'.", name), span);
            } else {
                env.symbols.insert(
                    name.clone(),
                    Symbol { name, symbol_type: aliased_type, is_mutable: false, span, kind: SymbolKind::TypeAlias },
                );
            }
        } else {
            self.add_error("Cannot define type outside any scope.".to_string(), span);
        }
    }

    /// Defines an effect in the current scope.
    fn define_effect(&mut self, name: String, span: Span) {
        if let Some(env) = self.scope_stack.back_mut() {
            if env.symbols.contains_key(&name) {
                self.add_error(format!("Redefinition of effect '{}'.", name), span);
            } else {
                env.symbols.insert(
                    name.clone(),
                    Symbol { name, symbol_type: Type::Unit, is_mutable: false, span, kind: SymbolKind::Effect }, // Effects don't have a 'type' in the traditional sense, use Unit as placeholder
                );
            }
        } else {
            self.add_error("Cannot define effect outside any scope.".to_string(), span);
        }
    }

    /// Adds a semantic error.
    fn add_error(&mut self, message: String, span: Span) {
        self.errors.push(SemanticError { message, span });
    }

    /// Main entry point for semantic analysis.
    pub fn analyze(&mut self, program: &ast::Program) -> Result<HashMap<String, Symbol>, Vec<SemanticError>> {
        println!("Starting semantic analysis...");

        for stmt in &program.statements {
            self.analyze_statement(stmt);
        }

        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            // Return the global symbol table at the end of successful analysis
            Ok(self.global_env.symbols.clone())
        }
    }

    /// Analyzes a single statement.
    fn analyze_statement(&mut self, stmt: &ast::Statement) {
        match stmt {
            ast::Statement::Let(span, name, type_expr_opt, expr) => {
                let expr_type = self.analyze_expression(expr);
                let declared_type = type_expr_opt.as_ref().map_or(Type::Unknown, |te| self.resolve_type_expr(te));

                if declared_type != Type::Unknown && declared_type != expr_type {
                    self.add_error(format!("Type mismatch: expected {:?}, got {:?}.", declared_type, expr_type), *span);
                }
                self.define_variable(name.clone(), expr_type, false, *span); // For simplicity, let vars are not mutable by default
            }
            ast::Statement::Return(span, expr) => {
                let _expr_type = self.analyze_expression(expr);
                // TODO: Check against function's declared return type
            }
            ast::Statement::Expression(expr) => {
                self.analyze_expression(expr);
            }
            ast::Statement::Function(span, name, params, return_type_expr_opt, body) => {
                let return_type = return_type_expr_opt.as_ref().map_or(Type::Unit, |te| self.resolve_type_expr(te));
                let param_types: Vec<Type> = params.iter().map(|p| self.resolve_type_expr(&p.param_type)).collect();
                self.define_function(name.clone(), param_types.clone(), return_type.clone(), *span);

                // Analyze function body in a new scope
                self.enter_scope();
                for p in params {
                    self.define_variable(p.name.clone(), self.resolve_type_expr(&p.param_type), false, p.span);
                }
                self.analyze_expression(body);
                self.exit_scope();
            }
            ast::Statement::QuantumCircuit(span, name, body) => {
                // Conceptual: Special semantic checks for quantum circuits
                // e.g., ensure qubit allocation/deallocation, no classical operations on qubits
                self.enter_scope();
                self.analyze_expression(body);
                self.exit_scope();
            }
            ast::Statement::NanoAgent(span, name, body) => {
                // Conceptual: Special semantic checks for nano-agents
                // e.g., resource usage, valid nano-actions
                self.enter_scope();
                self.analyze_expression(body);
                self.exit_scope();
            }
            ast::Statement::SankofaMemory(span, name, expr) => {
                // Conceptual: Semantic checks for Sankofa memory
                // e.g., ensure memory key is valid, check temporal consistency
                let _expr_type = self.analyze_expression(expr);
                // `remember` defines a new temporal memory key
                self.define_variable(name.clone(), Type::History(Box::new(_expr_type)), false, *span);
            }
            ast::Statement::TypeDeclaration(span, name, type_expr) => {
                let resolved_type = self.resolve_type_expr(type_expr);
                self.define_type(name.clone(), resolved_type, *span);
            }
            ast::Statement::EffectDeclaration(span, name) => {
                self.define_effect(name.clone(), *span);
            }
            ast::Statement::LanguageDeclaration(span, name, grammar_expr) => {
                // Conceptual: Semantic checks for language extensions
                // e.g., grammar validity, ensure no conflicts
                self.analyze_expression(grammar_expr);
            }
            ast::Statement::While(span, cond, body) => {
                let cond_type = self.analyze_expression(cond);
                if cond_type != Type::Bool {
                    self.add_error("While condition must be of type bool.".to_string(), *span);
                }
                self.enter_scope();
                self.analyze_expression(body);
                self.exit_scope();
            }
            ast::Statement::For(span, iterator_var, iterable, body) => {
                let iterable_type = self.analyze_expression(iterable);
                // Conceptual: Check if iterable_type implements an iterator trait
                // For simplicity, assume `iterable` produces `int` for `iterator_var`
                self.enter_scope();
                self.define_variable(iterator_var.0.clone(), Type::Int, false, iterator_var.1);
                self.analyze_expression(body);
                self.exit_scope();
            }
            ast::Statement::Break(_) | ast::Statement::Continue(_) => {
                // TODO: Ensure these are within a loop context
            }
            ast::Statement::Match(span, matched_expr, cases) => {
                let matched_type = self.analyze_expression(matched_expr);
                // TODO: Ensure all cases are exhaustive and patterns are of compatible type
                for case in cases {
                    // Conceptual: Analyze pattern (if it's a literal or simple identifier)
                    // let pattern_type = self.analyze_expression(&case.pattern);
                    // if pattern_type != matched_type {
                    //     self.add_error(format!("Match pattern type mismatch: expected {:?}, got {:?}.", matched_type, pattern_type), case.span);
                    // }
                    self.enter_scope(); // Each case body has its own scope
                    self.analyze_expression(&case.body);
                    self.exit_scope();
                }
            }
            ast::Statement::Unsafe(span, proof_opt, body) => {
                if self.evas_policy.strict_resource_management {
                    if proof_opt.is_none() || !self.evas_policy.approved_proofs.get(proof_opt.as_ref().unwrap_or(&"unknown".to_string())).map_or(false, |&b| b) {
                         self.add_error("Unsafe block requires a valid EVAS proof under current policy.".to_string(), *span);
                    }
                }
                // Analyze body, possibly with relaxed semantic rules for `unsafe`
                self.enter_scope();
                self.analyze_expression(body);
                self.exit_scope();
            }
        }
    }

    /// Analyzes an expression and returns its resolved type.
    fn analyze_expression(&mut self, expr: &ast::Expression) -> Type {
        match expr {
            ast::Expression::Literal(literal) => self.analyze_literal(literal),
            ast::Expression::Identifier(ident) => self.analyze_identifier(ident),
            ast::Expression::Prefix(span, op, right) => {
                let right_type = self.analyze_expression(right);
                self.check_prefix_operation(*op, right_type, *span)
            }
            ast::Expression::Infix(span, left, op, right) => {
                let left_type = self.analyze_expression(left);
                let right_type = self.analyze_expression(right);
                self.check_infix_operation(left_type, *op, right_type, *span)
            }
            ast::Expression::If(span, cond, then_block, else_block_opt) => {
                let cond_type = self.analyze_expression(cond);
                if cond_type != Type::Bool {
                    self.add_error("If condition must be of type bool.".to_string(), *span);
                }
                let then_type = self.analyze_expression(then_block);
                if let Some(else_block) = else_block_opt {
                    let else_type = self.analyze_expression(else_block);
                    if then_type != else_type {
                        self.add_error(format!("If-else branches must return compatible types: {:?} vs {:?}.", then_type, else_type), *span);
                        Type::Error
                    } else {
                        then_type
                    }
                } else {
                    then_type
                }
            }
            ast::Expression::Block(span, statements) => {
                self.enter_scope();
                let mut last_type = Type::Unit;
                for stmt in statements {
                    self.analyze_statement(stmt);
                    // The type of a block is the type of its last expression
                    if let ast::Statement::Expression(last_expr) = stmt {
                        last_type = self.analyze_expression(last_expr);
                    }
                }
                self.exit_scope();
                last_type
            }
            ast::Expression::Call(span, func_expr, args) => {
                let func_type = self.analyze_expression(func_expr);
                let arg_types: Vec<Type> = args.iter().map(|arg| self.analyze_expression(arg)).collect();
                self.check_function_call(func_type, arg_types, *span)
            }
            ast::Expression::Index(span, array_expr, index_expr) => {
                let array_type = self.analyze_expression(array_expr);
                let index_type = self.analyze_expression(index_expr);
                self.check_index_operation(array_type, index_type, *span)
            }
            ast::Expression::MemberAccess(span, object_expr, member_id) => {
                let object_type = self.analyze_expression(object_expr);
                self.check_member_access(object_type, member_id, *span)
            }
            ast::Expression::QuantumGateApplication(span, gate_name, args) => {
                let arg_types: Vec<Type> = args.iter().map(|arg| self.analyze_expression(arg)).collect();
                self.check_quantum_gate_application(gate_name, arg_types, *span)
            }
            ast::Expression::NanoAction(span, action_name, args) => {
                let arg_types: Vec<Type> = args.iter().map(|arg| self.analyze_expression(arg)).collect();
                self.check_nano_action(action_name, arg_types, *span)
            }
            ast::Expression::MtsOperation(span, op_name, args) => {
                let arg_types: Vec<Type> = args.iter().map(|arg| self.analyze_expression(arg)).collect();
                self.check_mts_operation(op_name, arg_types, *span)
            }
            ast::Expression::PerformEffect(span, effect_name, args) => {
                // Conceptual: check if effect is declared, and if the current context allows performing it
                if self.lookup_symbol(effect_name).map_or(false, |s| s.kind == SymbolKind::Effect) {
                    // Proceed with type checking args for the effect if needed
                    for arg in args {
                        self.analyze_expression(arg);
                    }
                    Type::Unit // Effects generally don't return a value to the caller's type system
                } else {
                    self.add_error(format!("Undeclared effect '{}'.", effect_name), *span);
                    Type::Error
                }
            }
        }
    }

    /// Resolves an AST TypeExpr into a compiler internal Type.
    fn resolve_type_expr(&mut self, type_expr: &TypeExpr) -> Type {
        match type_expr {
            TypeExpr::Base(Identifier(name, span)) => match name.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "bool" => Type::Bool,
                "char" => Type::Char,
                "string" => Type::String,
                "unit" => Type::Unit,
                "Qubit" => Type::Qubit,
                "NanoParticle" => Type::NanoParticle,
                _ => {
                    // Look up user-defined types
                    if let Some(symbol) = self.lookup_symbol(name) {
                        if symbol.kind == SymbolKind::TypeAlias {
                            symbol.symbol_type.clone()
                        } else {
                            self.add_error(format!("Identifier '{}' is not a type.", name), *span);
                            Type::Error
                        }
                    } else {
                        self.add_error(format!("Undefined type '{}'.", name), *span);
                        Type::Error
                    }
                }
            },
            TypeExpr::Array(element_type_expr, _size_opt) => {
                let element_type = self.resolve_type_expr(element_type_expr);
                Type::QubitArray(0) // Placeholder: actual size check would happen here
            }
            TypeExpr::FunctionType(param_type_exprs, return_type_expr) => {
                let param_types: Vec<Type> = param_type_exprs.iter().map(|te| self.resolve_type_expr(te)).collect();
                let return_type = self.resolve_type_expr(return_type_expr);
                Type::Function(param_types, Box::new(return_type))
            }
            TypeExpr::Tuple(member_type_exprs) => {
                let member_types: Vec<Type> = member_type_exprs.iter().map(|te| self.resolve_type_expr(te)).collect();
                Type::Tuple(member_types)
            }
            TypeExpr::Generic(base_ident, generic_arg_type_exprs) => {
                let base_type = self.resolve_type_expr(&TypeExpr::Base(base_ident.clone()));
                let generic_arg_types: Vec<Type> = generic_arg_type_exprs.iter().map(|te| self.resolve_type_expr(te)).collect();

                match base_ident.0.as_str() {
                    "Superposition" if generic_arg_types.len() == 1 && generic_arg_types[0] == Type::Qubit => {
                        Type::Superposition(Box::new(Type::Qubit))
                    }
                    "Entangled" if generic_arg_types.len() >= 2 && generic_arg_types.iter().all(|t| *t == Type::Qubit) => {
                        Type::Entangled(generic_arg_types)
                    }
                    // ... more specific generic type resolution for Zenith types
                    _ => {
                        // For user-defined generics, check if base_type is a valid generic type constructor
                        self.add_error(format!("Unresolved generic type '{}'.", base_ident.0), base_ident.1);
                        Type::Error
                    }
                }
            }
            TypeExpr::Linear(inner_type_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                Type::Linear(Box::new(inner_type))
            }
            TypeExpr::Affine(inner_type_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                Type::Affine(Box::new(inner_type))
            }
            TypeExpr::Effectful(inner_type_expr, effects) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                // Conceptual: Verify if all effects are declared
                for effect_id in effects {
                    if self.lookup_symbol(&effect_id.0).map_or(false, |s| s.kind == SymbolKind::Effect) {
                        // Effect is valid
                    } else {
                        self.add_error(format!("Undeclared effect '{}'.", effect_id.0), effect_id.1);
                    }
                }
                Type::Effectful(Box::new(inner_type), effects.clone())
            }
            TypeExpr::Dependent(base_type_expr, _proof_expr) => {
                let base_type = self.resolve_type_expr(base_type_expr);
                // Conceptual: analyze _proof_expr for validity in dependent type context
                base_type // Return base type for now
            }
            TypeExpr::PiType(_name, binder_type_expr, return_type_expr) => {
                let binder_type = self.resolve_type_expr(binder_type_expr);
                let return_type = self.resolve_type_expr(return_type_expr);
                Type::Pi(_name.clone(), Box::new(binder_type), Box::new(return_type))
            }
            TypeExpr::SigmaType(_name, first_type_expr, second_type_expr) => {
                let first_type = self.resolve_type_expr(first_type_expr);
                let second_type = self.resolve_type_expr(second_type_expr);
                Type::Sigma(_name.clone(), Box::new(first_type), Box::new(second_type))
            }
            TypeExpr::Proof(inner_type_expr, _proof_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                Type::Proof(Box::new(inner_type))
            }
            TypeExpr::TypeFamily(name, args) => {
                let resolved_args: Vec<Type> = args.iter().map(|arg| self.resolve_type_expr(arg)).collect();
                Type::TypeFamily(name.0.clone(), resolved_args)
            }
            TypeExpr::QuantumReg(element_type_expr, size_str) => {
                let element_type = self.resolve_type_expr(element_type_expr);
                if element_type == Type::Qubit {
                    if let Ok(size) = size_str.parse::<usize>() {
                        Type::QubitArray(size)
                    } else {
                        self.add_error(format!("Invalid quantum register size '{}'.", size_str), element_type_expr.get_span());
                        Type::Error
                    }
                } else {
                    self.add_error("Quantum register must be composed of Qubits.".to_string(), element_type_expr.get_span());
                    Type::Error
                }
            }
            TypeExpr::Superposition(inner_type_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                if inner_type == Type::Qubit {
                    Type::Superposition(Box::new(inner_type))
                } else {
                    self.add_error("Superposition can only apply to Qubit types.".to_string(), inner_type_expr.get_span());
                    Type::Error
                }
            }
            TypeExpr::Entangled(type1_expr, type2_expr) => {
                let type1 = self.resolve_type_expr(type1_expr);
                let type2 = self.resolve_type_expr(type2_expr);
                if type1 == Type::Qubit && type2 == Type::Qubit {
                    Type::Entangled(vec![type1, type2])
                } else {
                    self.add_error("Entanglement can only occur between Qubit types.".to_string(), type1_expr.get_span());
                    Type::Error
                }
            }
            TypeExpr::QMeasured(inner_type_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                // QMeasured typically resolves to a classical type (bool or int)
                Type::QMeasured(Box::new(inner_type))
            }
            TypeExpr::NanoAgentType(inner_type_expr) => {
                let _inner_type = self.resolve_type_expr(inner_type_expr); // Can be a blueprint type
                Type::NanoParticle
            }
            TypeExpr::ArchaeveType(inner_type_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                Type::Sasa // Conceptual: Archaeve data might resolve to Sasa knowledge
            }
            TypeExpr::MtsSlice(inner_type_expr, _size_opt) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                Type::MtsSlice(Box::new(inner_type))
            }
            TypeExpr::HistoryType(inner_type_expr, _duration_opt) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                Type::History(Box::new(inner_type))
            }
            TypeExpr::ConsensusTrueType(inner_type_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                Type::Bool // Conceptual: ConsensusTrue typically evaluates to a boolean assertion
            }
            TypeExpr::InterMemoryType(_lang_id, inner_type_expr) => {
                let inner_type = self.resolve_type_expr(inner_type_expr);
                inner_type // InterMemory just provides access to the inner type across languages
            }
            TypeExpr::Error(span) => {
                self.add_error("Error type encountered during resolution.".to_string(), *span);
                Type::Error
            }
        }
    }

    /// Analyzes a literal and returns its type.
    fn analyze_literal(&mut self, literal: &ast::Literal) -> Type {
        match literal {
            ast::Literal::Integer(_, _) => Type::Int,
            ast::Literal::Float(_, _) => Type::Float,
            ast::Literal::String(_, _) => Type::String,
            ast::Literal::Char(_, _) => Type::Char,
            ast::Literal::Boolean(_, _) => Type::Bool,
            ast::Literal::Quantum(_, span) => {
                // Conceptual: Validate quantum literal format
                Type::Qubit
            }
            ast::Literal::MTS(_, span) => {
                // Conceptual: Validate MTS literal format/value
                Type::MtsSlice(Box::new(Type::Unknown)) // MTS literal might denote a specific timeline or slice config
            }
        }
    }

    /// Analyzes an identifier and returns its type.
    fn analyze_identifier(&mut self, ident: &ast::Identifier) -> Type {
        if let Some(symbol) = self.lookup_symbol(&ident.0) {
            symbol.symbol_type.clone()
        } else {
            self.add_error(format!("Undeclared identifier '{}'.", ident.0), ident.1);
            Type::Error
        }
    }

    /// Checks a prefix operation and returns the result type.
    fn check_prefix_operation(&mut self, op: TokenType, right_type: Type, span: Span) -> Type {
        match op {
            TokenType::Bang => {
                if right_type == Type::Bool {
                    Type::Bool
                } else {
                    self.add_error("Operator '!' can only be applied to boolean types.".to_string(), span);
                    Type::Error
                }
            }
            TokenType::Minus => {
                if right_type == Type::Int || right_type == Type::Float {
                    right_type
                } else {
                    self.add_error("Operator '-' can only be applied to numeric types.".to_string(), span);
                    Type::Error
                }
            }
            _ => {
                self.add_error(format!("Unsupported prefix operator {:?}.", op), span);
                Type::Error
            }
        }
    }

    /// Checks an infix operation and returns the result type.
    fn check_infix_operation(&mut self, left_type: Type, op: TokenType, right_type: Type, span: Span) -> Type {
        match op {
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                if left_type == Type::Int && right_type == Type::Int {
                    Type::Int
                } else if left_type == Type::Float && right_type == Type::Float {
                    Type::Float
                } else {
                    self.add_error(format!("Type mismatch for arithmetic operation {:?}: {:?} and {:?}.", op, left_type, right_type), span);
                    Type::Error
                }
            }
            TokenType::Equals | TokenType::NotEquals => {
                if left_type == right_type {
                    Type::Bool
                } else {
                    self.add_error(format!("Type mismatch for comparison operation {:?}: {:?} and {:?}.", op, left_type, right_type), span);
                    Type::Error
                }
            }
            // TODO: Add support for quantum infix operations, e.g., entanglement operator
            _ => {
                self.add_error(format!("Unsupported infix operator {:?}.", op), span);
                Type::Error
            }
        }
    }

    /// Checks a function call and returns its result type.
    fn check_function_call(&mut self, func_type: Type, arg_types: Vec<Type>, span: Span) -> Type {
        if let Type::Function(param_types, return_type) = func_type {
            if param_types.len() != arg_types.len() {
                self.add_error("Incorrect number of arguments in function call.".to_string(), span);
                return Type::Error;
            }
            for (i, (param_t, arg_t)) in param_types.iter().zip(arg_types.iter()).enumerate() {
                if param_t != arg_t {
                    self.add_error(format!("Argument {} type mismatch: expected {:?}, got {:?}.", i, param_t, arg_t), span);
                    return Type::Error;
                }
            }
            *return_type
        } else {
            self.add_error("Called expression is not a function.".to_string(), span);
            Type::Error
        }
    }

    /// Checks an index operation and returns the element type.
    fn check_index_operation(&mut self, array_type: Type, index_type: Type, span: Span) -> Type {
        if index_type != Type::Int {
            self.add_error("Array index must be of type int.".to_string(), span);
            return Type::Error;
        }
        match array_type {
            Type::QubitArray(_) => Type::Qubit,
            _ => {
                self.add_error("Indexing is only supported for QubitArray types conceptually.".to_string(), span);
                Type::Error
            }
        }
    }

    /// Checks a member access operation and returns the member's type.
    fn check_member_access(&mut self, object_type: Type, member_id: &Identifier, span: Span) -> Type {
        match object_type {
            Type::Struct(_, fields) => {
                if let Some(member_type) = fields.get(&member_id.0) {
                    member_type.clone()
                } else {
                    self.add_error(format!("Struct has no member named '{}'.", member_id.0), span);
                    Type::Error
                }
            }
            _ => {
                self.add_error("Member access is only supported for struct types conceptually.".to_string(), span);
                Type::Error
            }
        }
    }

    /// Checks a quantum gate application and returns its result type.
    fn check_quantum_gate_application(&mut self, gate_name: &str, arg_types: Vec<Type>, span: Span) -> Type {
        // Conceptual: Validate gate_name, number of args, and arg types (must be Qubit/QubitArray)
        if arg_types.iter().all(|t| *t == Type::Qubit || matches!(t, Type::QubitArray(_))) {
            // Example: Hadamard operates on a single qubit
            if gate_name == "Hadamard" && arg_types.len() == 1 && arg_types[0] == Type::Qubit {
                Type::Qubit
            } else if gate_name == "CNOT" && arg_types.len() == 2 && arg_types[0] == Type::Qubit && arg_types[1] == Type::Qubit {
                Type::Tuple(vec![Type::Qubit, Type::Qubit]) // CNOT returns two qubits
            }
            // ... more gates
            else {
                self.add_error(format!("Invalid arguments for quantum gate '{}'.", gate_name), span);
                Type::Error
            }
        } else {
            self.add_error("Quantum gate arguments must be Qubit or QubitArray types.".to_string(), span);
            Type::Error
        }
    }

    /// Checks a nano-action and returns its result type.
    fn check_nano_action(&mut self, action_name: &str, arg_types: Vec<Type>, span: Span) -> Type {
        // Conceptual: Validate nano-action against nano-agent capabilities/context
        // Example: 'move_to' action
        if action_name == "move_to" && arg_types.len() == 2 && arg_types[0] == Type::NanoParticle {
            // Assume second arg is target_coords, could be a custom struct/tuple
            Type::Unit
        } else {
            self.add_error(format!("Invalid arguments for nano-action '{}'.", action_name), span);
            Type::Error
        }
    }

    /// Checks an MTS operation and returns its result type.
    fn check_mts_operation(&mut self, op_name: &str, arg_types: Vec<Type>, span: Span) -> Type {
        // Conceptual: Validate MTS operation against multi-timeline rules
        // Example: 'load' operation on an MtsSlice
        if op_name == "load" && arg_types.len() == 2 && matches!(arg_types[0], Type::MtsSlice(_)) && arg_types[1] == Type::Int {
            // Returns the inner type of the MtsSlice
            if let Type::MtsSlice(inner_type) = &arg_types[0] {
                *(inner_type.clone())
            } else { Type::Error }
        } else {
            self.add_error(format!("Invalid arguments for MTS operation '{}'.", op_name), span);
            Type::Error
        }
    }

    pub fn get_errors(&self) -> &[SemanticError] {
        &self.errors
    }
}


/// Represents a scope in the program (e.g., function body, block).
#[derive(Debug, Clone, Default)]
pub struct Environment {
    pub id: usize, // Unique ID for this environment
    pub parent_id: Option<usize>, // ID of the parent environment
    pub symbols: HashMap<String, Symbol>,
    next_id: usize,
}

impl Environment {
    pub fn new_global() -> Self {
        Environment {
            id: 0, // Global scope ID
            parent_id: None,
            symbols: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn new_local(parent_id: Option<usize>) -> Self {
        Environment {
            id: 0, // Will be set by a manager, or could auto-increment from a global counter
            parent_id,
            symbols: HashMap::new(),
            next_id: 0,
        }
    }

    // Conceptual: In a real compiler, Environment would be managed by a ScopeManager
    // that assigns unique IDs and handles nesting.
}
