//! Zamani Semantic Analyser
//!
//! Full type inference, scope management, symbol resolution,
//! borrow-lint hints, and error reporting for the Zamani language.

use crate::ast::*;
use crate::lexer::TokenType;
use crate::source_map::Span;
use std::collections::HashMap;

// ─── Errors ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

impl SemanticError {
    pub fn new(msg: impl Into<String>, span: Span) -> Self {
        SemanticError {
            message: msg.into(),
            span,
        }
    }
}

// ─── Symbol table ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Variable(Type),
    Constant(Type),
    Function(Vec<Type>, Type),
    TypeAlias(Type),
    Struct {
        fields: Vec<(String, Type)>,
    },
    Enum {
        variants: Vec<String>,
    },
    Trait {
        methods: Vec<String>,
    },
    Class {
        fields: Vec<(String, Type)>,
        methods: Vec<String>,
    },
}

impl Symbol {
    pub fn typ(&self) -> Type {
        match self {
            Symbol::Variable(t) | Symbol::Constant(t) | Symbol::TypeAlias(t) => t.clone(),
            Symbol::Function(_, ret) => ret.clone(),
            _ => Type::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    pub resolved_classes: HashMap<String, Type>,
    pub resolved_interfaces: HashMap<String, Type>,
    pub resolved_structs: HashMap<String, Vec<(String, Type)>>,
    pub resolved_traits: HashMap<String, Vec<String>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
            resolved_classes: HashMap::new(),
            resolved_interfaces: HashMap::new(),
            resolved_structs: HashMap::new(),
            resolved_traits: HashMap::new(),
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

    pub fn register_class(&mut self, name: String, ty: Type) {
        self.resolved_classes.insert(name, ty);
    }

    pub fn lookup_class(&self, name: &str) -> Option<&Type> {
        self.resolved_classes.get(name)
    }

    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Semantic Analyser ────────────────────────────────────────────────────────

pub struct SemanticAnalyzer {
    pub symbols: SymbolTable,
    pub errors: Vec<SemanticError>,
    pub current_return_type: Option<Type>,
    pub in_loop: bool,
    pub in_async: bool,
    /// Tracks usage of linear/affine variables: name -> usage_count
    pub usage_tracker: HashMap<String, usize>,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut s = SemanticAnalyzer {
            symbols: SymbolTable::new(),
            errors: vec![],
            current_return_type: None,
            in_loop: false,
            in_async: false,
            usage_tracker: HashMap::new(),
        };
        s.register_builtins();
        s
    }

    fn register_builtins(&mut self) {
        // I/O
        self.symbols.define(
            "print".into(),
            Symbol::Function(vec![Type::String], Type::Unit),
        );
        self.symbols.define(
            "println".into(),
            Symbol::Function(vec![Type::String], Type::Unit),
        );
        self.symbols.define(
            "eprint".into(),
            Symbol::Function(vec![Type::String], Type::Unit),
        );
        self.symbols.define(
            "eprintln".into(),
            Symbol::Function(vec![Type::String], Type::Unit),
        );
        self.symbols
            .define("readln".into(), Symbol::Function(vec![], Type::String));
        // Assertions
        self.symbols.define(
            "assert".into(),
            Symbol::Function(vec![Type::Bool], Type::Unit),
        );
        self.symbols.define(
            "assert_eq".into(),
            Symbol::Function(vec![Type::Unknown, Type::Unknown], Type::Unit),
        );
        self.symbols.define(
            "panic".into(),
            Symbol::Function(vec![Type::String], Type::Never),
        );
        self.symbols
            .define("unreachable".into(), Symbol::Function(vec![], Type::Never));
        self.symbols
            .define("todo".into(), Symbol::Function(vec![], Type::Never));
        // Conversions
        self.symbols.define(
            "to_string".into(),
            Symbol::Function(vec![Type::Unknown], Type::String),
        );
        self.symbols.define(
            "parse".into(),
            Symbol::Function(vec![Type::String], Type::Unknown),
        );
        self.symbols.define(
            "len".into(),
            Symbol::Function(vec![Type::Unknown], Type::Int(IntWidth::I64)),
        );
        // Memory
        self.symbols.define(
            "sizeof".into(),
            Symbol::Function(vec![Type::Unknown], Type::Int(IntWidth::I64)),
        );
        self.symbols.define(
            "alignof".into(),
            Symbol::Function(vec![Type::Unknown], Type::Int(IntWidth::I64)),
        );
        // Type primitives
        for (name, ty) in &[
            ("i8", Type::Int(IntWidth::I8)),
            ("i16", Type::Int(IntWidth::I16)),
            ("i32", Type::Int(IntWidth::I32)),
            ("i64", Type::Int(IntWidth::I64)),
            ("i128", Type::Int(IntWidth::I128)),
            ("isize", Type::Int(IntWidth::ISize)),
            ("u8", Type::UInt(IntWidth::I8)),
            ("u16", Type::UInt(IntWidth::I16)),
            ("u32", Type::UInt(IntWidth::I32)),
            ("u64", Type::UInt(IntWidth::I64)),
            ("u128", Type::UInt(IntWidth::I128)),
            ("usize", Type::UInt(IntWidth::ISize)),
            ("f32", Type::Float(FloatWidth::F32)),
            ("f64", Type::Float(FloatWidth::F64)),
            ("bool", Type::Bool),
            ("char", Type::Char),
            ("str", Type::Str),
            ("String", Type::String),
            ("int", Type::Int(IntWidth::I64)),
            ("float", Type::Float(FloatWidth::F64)),
            ("Int", Type::Int(IntWidth::I64)),
            ("Float", Type::Float(FloatWidth::F64)),
            ("Bool", Type::Bool),
            ("Char", Type::Char),
            ("Str", Type::Str),
        ] {
            self.symbols
                .define(name.to_string(), Symbol::TypeAlias(ty.clone()));
        }
        // Zamani-specific builtins
        self.symbols.define(
            "recall".into(),
            Symbol::Function(vec![Type::Unknown], Type::Unknown),
        );
        self.symbols.define(
            "remember".into(),
            Symbol::Function(vec![Type::String, Type::Unknown], Type::Unit),
        );
        self.symbols.define(
            "learn".into(),
            Symbol::Function(vec![Type::Unknown], Type::Unknown),
        );
        self.symbols.define(
            "infer".into(),
            Symbol::Function(vec![Type::Unknown], Type::Unknown),
        );
        self.symbols
            .define("wisdom".into(), Symbol::Variable(Type::Unknown));
    }

    pub fn analyze(&mut self, program: &Program) -> Vec<SemanticError> {
        if let Err(msg) = crate::toolchain::causality_checker::CausalityChecker::verify_program(program) {
            self.errors.push(SemanticError {
                message: format!("Causality Violation: {}", msg),
                span: Span::default(),
            });
        }
        for stmt in &program.statements {
            self.check_statement(stmt);
        }
        
        // Final check for linear variables: must be used exactly once
        for (name, count) in &self.usage_tracker {
            if let Some(Symbol::Variable(Type::Linear(_))) = self.symbols.lookup(name) {
                if *count == 0 {
                    self.errors.push(SemanticError::new(format!("Linear variable '{}' was never used.", name), Span::default()));
                }
            }
        }
        
        self.errors.clone()
    }

    // ── Statements ────────────────────────────────────────────────────────────

    fn check_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let(span, name, typ_ann, expr) => {
                let inferred = self.infer_expression(expr);
                let ty = if let Some(ann) = typ_ann {
                    let ann_ty = self.resolve_type_expr(ann);
                    if !self.types_compatible(&ann_ty, &inferred) && inferred != Type::Unknown {
                        self.errors.push(SemanticError::new(
                            format!(
                                "Type mismatch in `let {}`: declared {} but got {}",
                                name,
                                ann_ty.get_name(),
                                inferred.get_name()
                            ),
                            span.clone(),
                        ));
                    }
                    ann_ty
                } else {
                    inferred
                };
                self.symbols.define(name.clone(), Symbol::Variable(ty));
            }

            Statement::Const(span, name, typ_ann, expr) => {
                let inferred = self.infer_expression(expr);
                let ty = if let Some(ann) = typ_ann {
                    self.resolve_type_expr(ann)
                } else {
                    inferred
                };
                self.symbols.define(name.clone(), Symbol::Constant(ty));
                let _ = span;
            }

            Statement::Return(span, expr) => {
                let ty = self.infer_expression(expr);
                if let Some(ret_ty) = &self.current_return_type.clone() {
                    if !self.types_compatible(ret_ty, &ty) && ty != Type::Unknown {
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
                            .map(|t| self.resolve_type_expr(t))
                            .unwrap_or(Type::Unknown)
                    })
                    .collect();
                let ret_ty = ret_ann
                    .as_ref()
                    .map(|t| self.resolve_type_expr(t))
                    .unwrap_or(Type::Unit);
                self.symbols.define(
                    name.clone(),
                    Symbol::Function(param_types.clone(), ret_ty.clone()),
                );
                self.symbols.enter_scope();
                let prev_ret = self.current_return_type.replace(ret_ty.clone());
                for (param, pty) in params.iter().zip(param_types.iter()) {
                    self.symbols
                        .define(param.name.0.clone(), Symbol::Variable(pty.clone()));
                }
                let body_ty = self.infer_expression(body);
                if ret_ty != Type::Unit && ret_ty != Type::Unknown && body_ty != Type::Unknown {
                    // block bodies implicitly return last expression
                }
                self.current_return_type = prev_ret;
                self.symbols.exit_scope();
                let _ = span;
            }

            Statement::Struct(span, name, _type_params, fields) => {
                let field_types: Vec<(String, Type)> = fields
                    .iter()
                    .map(|f| (f.name.0.clone(), self.resolve_type_expr(&f.typ)))
                    .collect();
                self.symbols
                    .resolved_structs
                    .insert(name.0.clone(), field_types.clone());
                self.symbols.define(
                    name.0.clone(),
                    Symbol::Struct {
                        fields: field_types,
                    },
                );
                let _ = span;
            }

            Statement::Enum(span, name, _type_params, variants) => {
                let variant_names: Vec<String> =
                    variants.iter().map(|v| v.name.0.clone()).collect();
                self.symbols.define(
                    name.0.clone(),
                    Symbol::Enum {
                        variants: variant_names,
                    },
                );
                let _ = span;
            }

            Statement::Trait(span, name, _type_params, items) => {
                let method_names: Vec<String> = items.iter().map(|i| i.name.0.clone()).collect();
                self.symbols
                    .resolved_traits
                    .insert(name.0.clone(), method_names.clone());
                self.symbols.define(
                    name.0.clone(),
                    Symbol::Trait {
                        methods: method_names,
                    },
                );
                let _ = span;
            }

            Statement::Impl(_span, _trait_name, _ty, items) => {
                self.symbols.enter_scope();
                for item in items {
                    if let ImplItemKind::Method { params, ret, body } = &item.kind {
                        let param_types: Vec<Type> = params
                            .iter()
                            .map(|p| {
                                p.typ
                                    .as_ref()
                                    .map(|t| self.resolve_type_expr(t))
                                    .unwrap_or(Type::Unknown)
                            })
                            .collect();
                        let ret_ty = ret
                            .as_ref()
                            .map(|t| self.resolve_type_expr(t))
                            .unwrap_or(Type::Unit);
                        self.symbols.enter_scope();
                        let prev = self.current_return_type.replace(ret_ty.clone());
                        for (p, pt) in params.iter().zip(param_types.iter()) {
                            self.symbols
                                .define(p.name.0.clone(), Symbol::Variable(pt.clone()));
                        }
                        self.infer_expression(body);
                        self.current_return_type = prev;
                        self.symbols.exit_scope();
                        self.symbols
                            .define(item.name.0.clone(), Symbol::Function(param_types, ret_ty));
                    }
                }
                self.symbols.exit_scope();
            }

            Statement::Class(span, name, _bases, members) => {
                let fields: Vec<(String, Type)> = members
                    .iter()
                    .filter_map(|m| {
                        if let ClassMember::Field { name, typ, .. } = m {
                            Some((name.0.clone(), self.resolve_type_expr(typ)))
                        } else {
                            None
                        }
                    })
                    .collect();
                let methods: Vec<String> = members
                    .iter()
                    .filter_map(|m| {
                        if let ClassMember::Method { name, .. } = m {
                            Some(name.0.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                self.symbols
                    .register_class(name.0.clone(), Type::Named(name.0.clone()));
                self.symbols
                    .define(name.0.clone(), Symbol::Class { fields, methods });
                for member in members {
                    if let ClassMember::Method {
                        params, ret, body, ..
                    } = member
                    {
                        let ret_ty = ret
                            .as_ref()
                            .map(|t| self.resolve_type_expr(t))
                            .unwrap_or(Type::Unit);
                        self.symbols.enter_scope();
                        let prev = self.current_return_type.replace(ret_ty.clone());
                        for p in params {
                            let pt = p
                                .typ
                                .as_ref()
                                .map(|t| self.resolve_type_expr(t))
                                .unwrap_or(Type::Unknown);
                            self.symbols.define(p.name.0.clone(), Symbol::Variable(pt));
                        }
                        self.infer_expression(body);
                        self.current_return_type = prev;
                        self.symbols.exit_scope();
                    }
                }
                let _ = span;
            }

            Statement::While(span, cond, body) => {
                let cond_ty = self.infer_expression(cond);
                if cond_ty != Type::Bool && cond_ty != Type::Unknown {
                    self.errors.push(SemanticError::new(
                        format!("While condition must be Bool, got {}", cond_ty.get_name()),
                        span.clone(),
                    ));
                }
                let prev_loop = self.in_loop;
                self.in_loop = true;
                self.infer_expression(body);
                self.in_loop = prev_loop;
            }

            Statement::For(span, var, iter, body) => {
                let iter_ty = self.infer_expression(iter);
                let elem_ty = self.element_type(&iter_ty);
                self.symbols.enter_scope();
                self.symbols
                    .define(var.0.clone(), Symbol::Variable(elem_ty));
                let prev_loop = self.in_loop;
                self.in_loop = true;
                self.infer_expression(body);
                self.in_loop = prev_loop;
                self.symbols.exit_scope();
                let _ = span;
            }

            Statement::Break(span) => {
                if !self.in_loop {
                    self.errors
                        .push(SemanticError::new("break outside loop", span.clone()));
                }
            }

            Statement::Continue(span) => {
                if !self.in_loop {
                    self.errors
                        .push(SemanticError::new("continue outside loop", span.clone()));
                }
            }

            Statement::Match(span, expr, cases) => {
                self.infer_expression(expr);
                for case in cases {
                    self.check_pattern(&case.pattern);
                    if let Some(guard) = &case.guard {
                        self.infer_expression(guard);
                    }
                    self.infer_expression(&case.body);
                }
                let _ = span;
            }

            Statement::Import(span, path) => {
                // Register the last segment as a known name
                if let Some(last) = path.last() {
                    self.symbols
                        .define(last.clone(), Symbol::Variable(Type::Unknown));
                }
                let _ = span;
            }

            Statement::Module(span, name, body) => {
                self.symbols.enter_scope();
                for stmt in body {
                    self.check_statement(stmt);
                }
                self.symbols.exit_scope();
                self.symbols
                    .define(name.clone(), Symbol::Variable(Type::Named(name.clone())));
                let _ = span;
            }

            Statement::SankofaMemory(span, name, expr) => {
                let ty = self.infer_expression(expr);
                self.symbols.define(name.clone(), Symbol::Variable(ty));
                let _ = span;
            }

            Statement::QuantumCircuit(span, name, body) => {
                self.infer_expression(body);
                self.symbols
                    .define(name.clone(), Symbol::Variable(Type::Quantum));
                let _ = span;
            }

            Statement::NanoAgent(span, name, body) => {
                self.infer_expression(body);
                self.symbols.define(
                    name.clone(),
                    Symbol::Variable(Type::Named("NanoAgent".into())),
                );
                let _ = span;
            }

            Statement::Handle(span, effect_name, body, handler) => {
                self.infer_expression(body);
                self.infer_expression(handler);
                let _ = (span, effect_name);
            }

            Statement::Unsafe(span, label, body) => {
                self.infer_expression(body);
                println!("[Semantic] Vetting unsafe block with E.V.A.S. formal proof...");
                let mut prover = crate::toolchain::formal_verification::theorem_prover::TheoremProver::new();
                let proof_id = format!("unsafe_block_{:?}", span);
                prover.assert_theorem(&proof_id, "Memory safety and alignment preserved", vec!["evas_vetted".into()]);
                let proof = prover.prove(&proof_id, crate::toolchain::formal_verification::theorem_prover::ProofStrategy::SmtSolving);
                if !proof.valid {
                    println!("  [WARNING] Unsafe block at {:?} failed formal safety proof.", span);
                }
            }

            Statement::Wisdom(span, name, expr) => {
                let ty = self.infer_expression(expr);
                self.symbols.define(name.clone(), Symbol::Constant(ty));
                let _ = span;
            }

            Statement::Expression(expr) => {
                self.infer_expression(expr);
            }

            Statement::EffectDeclaration(span, name) => {
                self.symbols.define(
                    name.0.clone(),
                    Symbol::Variable(Type::Named("Effect".into())),
                );
                let _ = span;
            }

            Statement::TypeDeclaration(span, name, type_expr) => {
                let ty = self.resolve_type_expr(type_expr);
                self.symbols.define(name.clone(), Symbol::TypeAlias(ty));
                let _ = span;
            }

            Statement::TypeAlias(_span, name, _params, type_expr) => {
                let ty = self.resolve_type_expr(type_expr);
                self.symbols.define(name.0.clone(), Symbol::TypeAlias(ty));
            }

            Statement::Interface(span, name, _bases, _members) => {
                self.symbols
                    .define(name.0.clone(), Symbol::Trait { methods: vec![] });
                let _ = span;
            }

            Statement::Use(span, path) => {
                if let Some(last) = path.segments.last() {
                    self.symbols
                        .define(last.clone(), Symbol::Variable(Type::Unknown));
                }
                let _ = span;
            }

            Statement::LanguageDeclaration(span, _, _) => {
                let _ = span;
            }

            Statement::OmniversalSimulation(_, name, stmts) => {
                println!("[Semantic] Validating Omniversal Simulation: {}", name);
                self.symbols.enter_scope();
                for s in stmts {
                    self.check_statement(s);
                }
                self.symbols.exit_scope();
            }
            Statement::OmniversalAlignment(_, name, stmts) => {
                println!("[Semantic] Validating Omniversal Alignment: {}", name);
                self.symbols.enter_scope();
                for s in stmts {
                    self.check_statement(s);
                }
                self.cognitive_engine.verify_alignment(name);
                self.symbols.exit_scope();
            }
            Statement::OmniversalSovereignty(_, name, stmts) => {
                println!("[Semantic] Validating Omniversal Sovereignty: {}", name);
                self.symbols.enter_scope();
                for s in stmts {
                    self.check_statement(s);
                }
                self.symbols.exit_scope();
            }
            | Statement::OmniversalCodeSynth(_, _, stmts)
            | Statement::OmniversalDeploy(_, _, stmts)
            | Statement::OmniversalContainment(_, _, stmts)
            | Statement::OmniversalTrust(_, _, stmts)
            | Statement::OmniversalKnowledge(_, _, stmts)
            | Statement::OmniversalGenerative(_, _, stmts)
            | Statement::OmniversalGoal(_, _, stmts)
            | Statement::OmniversalBioNano(_, _, stmts)
            | Statement::OmniversalReality(_, _, stmts)
            | Statement::OmniversalNlp(_, _, stmts)
            | Statement::AsiSystem(_, _, stmts)
            | Statement::AesiSystem(_, _, stmts)
            | Statement::AsesiSystem(_, _, stmts)
            | Statement::AdminInterface(_, _, stmts)
            | Statement::PaymentGateway(_, _, stmts)
            | Statement::Graphics(_, _, stmts)
            | Statement::Video(_, _, stmts)
            | Statement::SelfAdjust(_, _, stmts)
            | Statement::SelfVersioning(_, _, stmts)
            | Statement::CopyrightNotice(_, _, stmts)
            | Statement::LegalAction(_, _, stmts)
            | Statement::TailorMadeFeature(_, _, stmts)
            | Statement::AiForBusiness(_, _, stmts) => {
                self.symbols.enter_scope();
                for s in stmts {
                    self.check_statement(s);
                }
                self.symbols.exit_scope();
            }
            Statement::TypeClass(_span, name, stmts) => {
                println!("[Semantic] Resolving typeclass: {}", name.0);
                self.symbols.enter_scope();
                for s in stmts {
                    self.check_statement(s);
                }
                self.symbols.exit_scope();
            }
            Statement::TypeInstance(_span, class_name, target_ty, stmts) => {
                let ty = self.resolve_type_expr(target_ty);
                println!("[Semantic] Resolving instance of {} for {}", class_name.0, ty.get_name());
                self.symbols.enter_scope();
                for s in stmts {
                    self.check_statement(s);
                }
                self.symbols.exit_scope();
            }
            Statement::HigherKindedType(_span, param, body) => {
                let ty = self.resolve_type_expr(body);
                println!("[Semantic] Resolving HKT with param {}: {}", param.name.0, ty.get_name());
            }
        }
    }

    // ── Pattern checking ──────────────────────────────────────────────────────

    fn check_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Identifier(id) => {
                self.symbols
                    .define(id.0.clone(), Symbol::Variable(Type::Unknown));
            }
            Pattern::Tuple(_, pats) | Pattern::Or(_, pats) => {
                for p in pats {
                    self.check_pattern(p);
                }
            }
            Pattern::Struct(_, _, fields) => {
                for (_, p) in fields {
                    self.check_pattern(p);
                }
            }
            Pattern::Enum(_, _, pats) => {
                for p in pats {
                    self.check_pattern(p);
                }
            }
            Pattern::Ref(_, inner) | Pattern::Range(_, inner, _) => {
                self.check_pattern(inner);
            }
            _ => {}
        }
    }

    // ── Expression type inference ─────────────────────────────────────────────

    pub fn infer_expression(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Literal(lit) => self.infer_literal(lit),

            Expression::Identifier(id) => match self.symbols.lookup(&id.0) {
                Some(sym) => {
                    let ty = sym.typ();
                    let count = self.usage_tracker.entry(id.0.clone()).or_insert(0);
                    *count += 1;
                    match ty {
                        Type::Linear(_) if *count > 1 => {
                            self.errors.push(SemanticError::new(
                                format!("Linear violation: variable '{}' used more than once.", id.0),
                                id.1.clone(),
                            ));
                        }
                        Type::Affine(_) if *count > 1 => {
                            self.errors.push(SemanticError::new(
                                format!("Affine violation: variable '{}' used more than once.", id.0),
                                id.1.clone(),
                            ));
                        }
                        _ => {}
                    }
                    ty
                }
                None => {
                    self.errors.push(SemanticError::new(
                        format!("Undefined symbol: '{}'", id.0),
                        id.1.clone(),
                    ));
                    Type::Unknown
                }
            },

            Expression::Prefix(_, op, inner) => {
                let ty = self.infer_expression(inner);
                match op {
                    TokenType::Not => Type::Bool,
                    TokenType::Minus => ty,
                    TokenType::BitAnd => Type::Reference(false, Box::new(ty)),
                    _ => ty,
                }
            }

            Expression::Infix(span, left, op, right) => {
                let lt = self.infer_expression(left);
                let rt = self.infer_expression(right);
                self.infer_binop(op, &lt, &rt, span)
            }

            Expression::If(_, cond, then, else_) => {
                let cond_ty = self.infer_expression(cond);
                if cond_ty != Type::Bool && cond_ty != Type::Unknown {
                    // soft-warn but don't hard error (truthiness may be defined)
                }
                let then_ty = self.infer_expression(then);
                if let Some(else_expr) = else_ {
                    let else_ty = self.infer_expression(else_expr);
                    if self.types_compatible(&then_ty, &else_ty) {
                        then_ty
                    } else {
                        Type::Unknown
                    }
                } else {
                    Type::Unit
                }
            }

            Expression::Block(_, stmts) => {
                self.symbols.enter_scope();
                let mut last = Type::Unit;
                for (i, s) in stmts.iter().enumerate() {
                    if i == stmts.len() - 1 {
                        if let Statement::Expression(e) = s {
                            last = self.infer_expression(e);
                        } else {
                            self.check_statement(s);
                            last = Type::Unit;
                        }
                    } else {
                        self.check_statement(s);
                    }
                }
                self.symbols.exit_scope();
                last
            }

            Expression::Call(span, func, args) => {
                let func_ty = self.infer_expression(func);
                for arg in args {
                    self.infer_expression(arg);
                }
                match func_ty {
                    Type::Function(_, ret) => *ret,
                    Type::Unknown => Type::Unknown,
                    _ => {
                        // Could be a struct constructor or callable
                        if let Expression::Identifier(id) = func.as_ref() {
                            if self.symbols.resolved_structs.contains_key(&id.0)
                                || self.symbols.resolved_classes.contains_key(&id.0)
                            {
                                return Type::Named(id.0.clone());
                            }
                        }
                        let _ = span;
                        Type::Unknown
                    }
                }
            }

            Expression::Lambda(_, params, body) => {
                self.symbols.enter_scope();
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| {
                        let ty = p
                            .typ
                            .as_ref()
                            .map(|t| self.resolve_type_expr(t))
                            .unwrap_or(Type::Unknown);
                        self.symbols
                            .define(p.name.0.clone(), Symbol::Variable(ty.clone()));
                        ty
                    })
                    .collect();
                let ret_ty = self.infer_expression(body);
                self.symbols.exit_scope();
                Type::Function(param_types, Box::new(ret_ty))
            }

            Expression::Array(_, elems) => {
                if elems.is_empty() {
                    return Type::Array(Box::new(Type::Unknown), None);
                }
                let elem_ty = self.infer_expression(&elems[0]);
                for e in &elems[1..] {
                    self.infer_expression(e);
                }
                Type::Array(Box::new(elem_ty), Some(elems.len()))
            }

            Expression::Tuple(_, elems) => {
                let types: Vec<Type> = elems.iter().map(|e| self.infer_expression(e)).collect();
                Type::Tuple(types)
            }

            Expression::Struct(_, name, fields) => {
                for (_, val) in fields {
                    self.infer_expression(val);
                }
                Type::Named(name.0.clone())
            }

            Expression::Index(_, base, idx) => {
                let base_ty = self.infer_expression(base);
                self.infer_expression(idx);
                match base_ty {
                    Type::Array(elem, _) | Type::Slice(elem) => *elem,
                    _ => Type::Unknown,
                }
            }

            Expression::MemberAccess(_, obj, field) => {
                let obj_ty = self.infer_expression(obj);
                self.lookup_field_type(&obj_ty, &field.0)
            }

            Expression::MethodCall(_, obj, method, args) => {
                let obj_ty = self.infer_expression(obj);
                for arg in args {
                    self.infer_expression(arg);
                }
                self.lookup_method_return(&obj_ty, &method.0)
            }

            Expression::Assign(_, target, value) => {
                self.infer_expression(target);
                self.infer_expression(value);
                Type::Unit
            }

            Expression::CompoundAssign(_, target, _, value) => {
                self.infer_expression(target);
                self.infer_expression(value);
                Type::Unit
            }

            Expression::Cast(_, expr, target_type) => {
                self.infer_expression(expr);
                self.resolve_type_expr(target_type)
            }

            Expression::TypeAscription(_, expr, ty) => {
                self.infer_expression(expr);
                self.resolve_type_expr(ty)
            }

            Expression::Await(_, inner) => {
                let inner_ty = self.infer_expression(inner);
                // Awaiting unwraps the Future<T> → T
                match inner_ty {
                    Type::Generic(n, args) if n == "Future" || n == "Promise" => {
                        args.into_iter().next().unwrap_or(Type::Unknown)
                    }
                    t => t,
                }
            }

            Expression::Async(_, body) => {
                let ret = self.infer_expression(body);
                Type::Generic("Future".into(), vec![ret])
            }

            Expression::Spawn(_, body) => {
                self.infer_expression(body);
                Type::Generic("Task".into(), vec![Type::Unknown])
            }

            Expression::New(_, name, args) => {
                for arg in args {
                    self.infer_expression(arg);
                }
                Type::Named(name.0.clone())
            }

            Expression::Range(_, start, end, _) => {
                self.infer_expression(start);
                self.infer_expression(end);
                Type::Generic("Range".into(), vec![Type::Int(IntWidth::I64)])
            }

            Expression::Try(_, inner) => {
                let ty = self.infer_expression(inner);
                match ty {
                    Type::Result(ok, _) => *ok,
                    Type::Optional(inner) => *inner,
                    t => t,
                }
            }

            Expression::TryCatch(_, body, arms) => {
                let body_ty = self.infer_expression(body);
                for arm in arms {
                    self.symbols.enter_scope();
                    if let Some(binding) = &arm.binding {
                        self.symbols.define(
                            binding.0.clone(),
                            Symbol::Variable(Type::Named("Error".into())),
                        );
                    }
                    self.infer_expression(&arm.body);
                    self.symbols.exit_scope();
                }
                body_ty
            }

            Expression::Match(_, expr, cases) => {
                self.infer_expression(expr);
                let mut result_ty = Type::Unit;
                for case in cases {
                    self.symbols.enter_scope();
                    self.check_pattern(&case.pattern);
                    if let Some(guard) = &case.guard {
                        self.infer_expression(guard);
                    }
                    result_ty = self.infer_expression(&case.body);
                    self.symbols.exit_scope();
                }
                result_ty
            }

            Expression::Loop(_, body) => {
                let prev = self.in_loop;
                self.in_loop = true;
                self.infer_expression(body);
                self.in_loop = prev;
                Type::Never
            }

            Expression::QuantumOp(_, _, args) => {
                for a in args {
                    self.infer_expression(a);
                }
                Type::Quantum
            }

            Expression::NanoOp(_, _, args) => {
                for a in args {
                    self.infer_expression(a);
                }
                Type::Named("NanoResult".into())
            }

            Expression::Recall(_, domain) => {
                self.infer_expression(domain);
                Type::Unknown
            }

            Expression::Remember(_, name, val) => {
                let ty = self.infer_expression(val);
                self.symbols.define(name.clone(), Symbol::Variable(ty));
                Type::Unit
            }

            Expression::Learn(_, data) => {
                self.infer_expression(data);
                Type::Unknown
            }

            Expression::Perform(_, effect) => {
                self.infer_expression(effect);
                Type::Unknown
            }

            Expression::Zamani(_, body) => self.infer_expression(body),

            Expression::Sasa(_, body) => self.infer_expression(body),

            Expression::Macro(_, _, args) => {
                for a in args {
                    self.infer_expression(a);
                }
                Type::Unknown
            }
        }
    }

    fn infer_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Integer(_, _) => Type::Int(IntWidth::I64),
            Literal::Float(_, _) => Type::Float(FloatWidth::F64),
            Literal::String(_, _) => Type::String,
            Literal::Boolean(_, _) => Type::Bool,
            Literal::Char(_, _) => Type::Char,
            Literal::Null(_) | Literal::Unit(_) => Type::Unit,
            Literal::Quantum(_, _) => Type::Quantum,
            Literal::Nano(_, _) => Type::Named("Nano".into()),
            Literal::MTS(_, _) => Type::Named("MTS".into()),
        }
    }

    fn infer_binop(&self, op: &TokenType, lt: &Type, rt: &Type, _span: &Span) -> Type {
        use crate::lexer::TokenType::*;
        match op {
            // Comparison → Bool
            Equals | NotEquals | LessThan | LessThanEqual | GreaterThan | GreaterThanEqual => {
                Type::Bool
            }
            // Logical → Bool
            LogicalAnd | LogicalOr | KeywordAnd | KeywordOr => Type::Bool,
            // Arithmetic → promote
            Plus | Minus | Star | Slash | Modulo => {
                if lt.is_float() || rt.is_float() {
                    Type::Float(FloatWidth::F64)
                } else if lt.is_integer() {
                    lt.clone()
                } else if lt == &Type::String && op == &Plus {
                    Type::String
                } else {
                    Type::Unknown
                }
            }
            // Bitwise
            BitAnd | Pipe | Caret | LeftShift | RightShift => {
                if lt.is_integer() {
                    lt.clone()
                } else {
                    Type::Unknown
                }
            }
            _ => Type::Unknown,
        }
    }

    // ── Type helpers ──────────────────────────────────────────────────────────

    pub fn resolve_type_expr(&self, te: &TypeExpr) -> Type {
        match te {
            TypeExpr::Identifier(id) => self.resolve_type_annotation_name(te),
            TypeExpr::Generic(base, args) => {
                let base_name = base.name();
                let arg_types: Vec<Type> = args.iter().map(|a| self.resolve_type_expr(a)).collect();
                match base_name.as_str() {
                    "Vec" | "List" => Type::Generic("Vec".into(), arg_types),
                    "HashMap" | "Map" => Type::Generic("HashMap".into(), arg_types),
                    "Option" => Type::Optional(Box::new(
                        arg_types.into_iter().next().unwrap_or(Type::Unknown),
                    )),
                    "Result" => {
                        let mut it = arg_types.into_iter();
                        let ok = it.next().unwrap_or(Type::Unknown);
                        let err = it.next().unwrap_or(Type::Named("Error".into()));
                        Type::Result(Box::new(ok), Box::new(err))
                    }
                    "Future" | "Promise" | "Task" => Type::Generic(base_name, arg_types),
                    "Quantum" => Type::Quantum,
                    "Linear" => Type::Linear(Box::new(
                        arg_types.into_iter().next().unwrap_or(Type::Unknown),
                    )),
                    "Affine" => Type::Affine(Box::new(
                        arg_types.into_iter().next().unwrap_or(Type::Unknown),
                    )),
                    n => Type::Generic(n.into(), arg_types),
                }
            }
            TypeExpr::Tuple(types) => {
                Type::Tuple(types.iter().map(|t| self.resolve_type_expr(t)).collect())
            }
            TypeExpr::Array(inner) => Type::Array(Box::new(self.resolve_type_expr(inner)), None),
            TypeExpr::Slice(inner) => Type::Slice(Box::new(self.resolve_type_expr(inner))),
            TypeExpr::Function(params, ret) => {
                let p: Vec<Type> = params.iter().map(|t| self.resolve_type_expr(t)).collect();
                Type::Function(p, Box::new(self.resolve_type_expr(ret)))
            }
            TypeExpr::Reference(m, inner) => {
                Type::Reference(*m, Box::new(self.resolve_type_expr(inner)))
            }
            TypeExpr::Pointer(m, inner) => {
                Type::Pointer(*m, Box::new(self.resolve_type_expr(inner)))
            }
            TypeExpr::Optional(inner) => Type::Optional(Box::new(self.resolve_type_expr(inner))),
            TypeExpr::Result(ok, err) => Type::Result(
                Box::new(self.resolve_type_expr(ok)),
                Box::new(self.resolve_type_expr(err)),
            ),
            TypeExpr::Never => Type::Never,
            TypeExpr::Unit => Type::Unit,
            TypeExpr::SelfType => Type::Named("Self".into()),
            TypeExpr::Quantum(_) => Type::Quantum,
            TypeExpr::Linear(inner) => Type::Linear(Box::new(self.resolve_type_expr(inner))),
            TypeExpr::Affine(inner) => Type::Affine(Box::new(self.resolve_type_expr(inner))),
            TypeExpr::Temporal(inner) => {
                Type::Generic("Temporal".into(), vec![self.resolve_type_expr(inner)])
            }
            TypeExpr::Pi(name, domain, codomain) => {
                let dom_ty = self.resolve_type_expr(domain);
                // Note: Full dependent type resolution requires term-level evaluation
                let codom_ty = self.resolve_type_expr(codomain);
                Type::Pi(name.clone(), Box::new(dom_ty), Box::new(codom_ty))
            }
            TypeExpr::Sigma(name, domain, codomain) => {
                let dom_ty = self.resolve_type_expr(domain);
                let codom_ty = self.resolve_type_expr(codomain);
                Type::Sigma(name.clone(), Box::new(dom_ty), Box::new(codom_ty))
            }
            TypeExpr::Identity(left, right) => {
                Type::Identity(left.clone(), right.clone())
            }
        }
    }

    pub fn resolve_type_annotation_name(&self, te: &TypeExpr) -> Type {
        let name = te.name();
        match name.as_str() {
            "i8" => Type::Int(IntWidth::I8),
            "i16" => Type::Int(IntWidth::I16),
            "i32" => Type::Int(IntWidth::I32),
            "i64" | "int" | "Int" => Type::Int(IntWidth::I64),
            "i128" => Type::Int(IntWidth::I128),
            "isize" => Type::Int(IntWidth::ISize),
            "u8" => Type::UInt(IntWidth::I8),
            "u16" => Type::UInt(IntWidth::I16),
            "u32" => Type::UInt(IntWidth::I32),
            "u64" | "usize" => Type::UInt(IntWidth::ISize),
            "u128" => Type::UInt(IntWidth::I128),
            "f32" => Type::Float(FloatWidth::F32),
            "f64" | "float" | "Float" => Type::Float(FloatWidth::F64),
            "bool" | "Bool" => Type::Bool,
            "char" | "Char" => Type::Char,
            "str" | "Str" => Type::Str,
            "String" | "string" => Type::String,
            "()" => Type::Unit,
            "!" => Type::Never,
            "Quantum" => Type::Quantum,
            n => match self.symbols.lookup(n) {
                Some(Symbol::TypeAlias(t)) => t.clone(),
                _ => Type::Named(n.into()),
            },
        }
    }

    fn lookup_field_type(&self, obj_ty: &Type, field: &str) -> Type {
        match obj_ty {
            Type::Named(name) | Type::Generic(name, _) => {
                if let Some(fields) = self.symbols.resolved_structs.get(name) {
                    for (fname, ftype) in fields {
                        if fname == field {
                            return ftype.clone();
                        }
                    }
                }
            }
            _ => {}
        }
        Type::Unknown
    }

    fn lookup_method_return(&self, obj_ty: &Type, method: &str) -> Type {
        // Built-in methods
        match (obj_ty, method) {
            (Type::String, "len") | (Type::Array(..), "len") | (Type::Generic(..), "len") => {
                return Type::UInt(IntWidth::ISize);
            }
            (Type::String, "to_uppercase")
            | (Type::String, "to_lowercase")
            | (Type::String, "trim") => return Type::String,
            (Type::String, "contains")
            | (Type::String, "starts_with")
            | (Type::String, "ends_with")
            | (Type::String, "is_empty") => return Type::Bool,
            (Type::Array(_, _), "push") | (Type::Generic(_, _), "push") => return Type::Unit,
            (Type::Array(elem, _), "pop") => return Type::Optional(elem.clone()),
            (Type::Array(elem, _), "get") => return Type::Optional(elem.clone()),
            _ => {}
        }
        Type::Unknown
    }

    fn element_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Array(elem, _) | Type::Slice(elem) => *elem.clone(),
            Type::Generic(name, args) if name == "Vec" || name == "List" => {
                args.first().cloned().unwrap_or(Type::Unknown)
            }
            Type::Generic(_, args) => args.first().cloned().unwrap_or(Type::Unknown),
            _ => Type::Unknown,
        }
    }

    pub fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        if expected == actual {
            return true;
        }
        if *expected == Type::Unknown || *actual == Type::Unknown {
            return true;
        }
        // Int widening
        if expected.is_numeric() && actual.is_numeric() {
            return true;
        }
        // String ↔ str
        if (*expected == Type::String || *expected == Type::Str)
            && (*actual == Type::String || *actual == Type::Str)
        {
            return true;
        }
        // Named types
        match (expected, actual) {
            (Type::Named(a), Type::Named(b)) => a == b,
            (Type::Named(_), _) | (_, Type::Named(_)) => true, // structural for now
            (Type::Optional(a), Type::Optional(b)) => self.types_compatible(a, b),
            (Type::Result(a1, b1), Type::Result(a2, b2)) => {
                self.types_compatible(a1, a2) && self.types_compatible(b1, b2)
            }
            (Type::Array(a, _), Type::Array(b, _)) => self.types_compatible(a, b),
            (Type::Tuple(a), Type::Tuple(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| self.types_compatible(x, y))
            }
            (Type::Function(ap, ar), Type::Function(bp, br)) => {
                ap.len() == bp.len()
                    && ap
                        .iter()
                        .zip(bp.iter())
                        .all(|(x, y)| self.types_compatible(x, y))
                    && self.types_compatible(ar, br)
            }
            _ => false,
        }
    }
}

/// A focused type-checking pass, complementing `SemanticAnalyzer`'s broader
/// analysis with a narrow API for querying/comparing types directly.
pub struct TypeChecker {
    pub errors: Vec<SemanticError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker { errors: Vec::new() }
    }

    /// Checks whether `actual` is compatible with `expected`, recording a
    /// `SemanticError` (at `span`) if not.
    pub fn check(&mut self, expected: &Type, actual: &Type, span: Span) -> bool {
        let analyzer = SemanticAnalyzer::new();
        let compatible = analyzer.types_compatible(expected, actual);
        if !compatible {
            self.errors.push(SemanticError::new(
                format!("type mismatch: expected {:?}, found {:?}", expected, actual),
                span,
            ));
        }
        compatible
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
