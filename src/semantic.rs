
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
    Function(Type, Type), // Return type, function type (params, return)
    Effect(Identifier),
    TypeAlias(Type), // For 'type MyType = OtherType;'

    // --- OOP Symbols ---
    // These now hold a reference to the fully resolved Type::Class/Interface in the symbol table
    ClassRef(Identifier), 
    InterfaceRef(Identifier),
}

/// Manages scopes and symbols during semantic analysis.
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    // Global storage for fully resolved Class and Interface types
    resolved_classes: HashMap<String, Type>, 
    resolved_interfaces: HashMap<String, Type>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { 
            scopes: vec![HashMap::new()],
            resolved_classes: HashMap::new(),
            resolved_interfaces: HashMap::new(),
        } // Start with a global scope
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

    // New methods for OOP types
    pub fn define_class_type(&mut self, name: String, class_type: Type) {
        self.resolved_classes.insert(name.clone(), class_type.clone());
        self.define(name, Symbol::ClassRef(class_type.get_name()));
    }
    pub fn define_interface_type(&mut self, name: String, interface_type: Type) {
        self.resolved_interfaces.insert(name.clone(), interface_type.clone());
        self.define(name, Symbol::InterfaceRef(interface_type.get_name()));
    }
    pub fn lookup_class_type(&self, name: &str) -> Option<&Type> { self.resolved_classes.get(name) }
    pub fn lookup_interface_type(&self, name: &str) -> Option<&Type> { self.resolved_interfaces.get(name) }
}

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    current_function_return_type: Option<Type>, // For return statement checks
    current_class_context: Option<Type>, // Track current class for 'this'/'super' and member access
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            current_function_return_type: None,
            current_class_context: None,
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        
        // Pass 1: Collect all global type declarations (classes, interfaces, type aliases, functions, effects)
        // This allows for forward references and building initial symbol table entries.
        for stmt in &program.statements {
            match stmt {
                Statement::Class(span, name, parents, members) => {
                    // Just register the class name. Full resolution in Pass 2.
                    if self.symbol_table.lookup(&name.0).is_some() {
                        self.errors.push(SemanticError { message: format!("Class '{}' already declared.", name.0), span: name.1.clone() });
                    } else {
                        self.symbol_table.define(name.0.clone(), Symbol::ClassRef(name.clone()));
                    }
                }
                Statement::Interface(span, name, parents, members) => {
                    // Just register the interface name. Full resolution in Pass 2.
                    if self.symbol_table.lookup(&name.0).is_some() {
                        self.errors.push(SemanticError { message: format!("Interface '{}' already declared.", name.0), span: name.1.clone() });
                    } else {
                        self.symbol_table.define(name.0.clone(), Symbol::InterfaceRef(name.clone()));
                    }
                }
                Statement::TypeDeclaration(span, name, type_expr) => {
                    // Type aliases need to be resolved. For simplicity, just store the name.
                    self.symbol_table.define(name.0.clone(), Symbol::TypeAlias(Type::Unknown)); // Placeholder
                }
                Statement::Function(span, name, params, ret_type_expr, _) => {
                    // Define function signature now
                    let func_type = self.resolve_function_signature(params, ret_type_expr)?;
                    self.symbol_table.define(name.clone(), Symbol::Function(Type::Unknown, func_type)); // Placeholder for actual return type for now
                }
                Statement::EffectDeclaration(_, name) => {
                    self.symbol_table.define(name.0.clone(), Symbol::Effect(name.clone()));
                }
                _ => {}
            }
        }

        // Pass 2: Resolve inheritance and fill in members for classes/interfaces. Build full Type::Class/Interface definitions.
        for stmt in &program.statements {
            match stmt {
                Statement::Class(span, name, parents, members) => {
                    self.analyze_class_declaration(span.clone(), name.clone(), parents, members);
                }
                Statement::Interface(span, name, parents, members) => {
                    self.analyze_interface_declaration(span.clone(), name.clone(), parents, members);
                }
                _ => {}
            }
        }
        
        // Pass 3: Full analysis of all statements (function bodies, method bodies, expressions, etc.)
        for stmt in &program.statements {
            if let Err(mut stmt_errors) = self.analyze_statement(stmt) {
                self.errors.append(&mut stmt_errors);
            }
        }

        if self.errors.is_empty() { Ok(()) } else { Err(self.errors.drain(..).collect()) }
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), Vec<SemanticError>> {
        match stmt {
            Statement::Let(span, name, typ_expr, expr) => {
                let expr_type = self.analyze_expression(expr)?;
                let mut declared_type = expr_type.clone();
                if let Some(annotated_typ_expr) = typ_expr {
                    declared_type = self.resolve_type_expression(annotated_typ_expr)?;
                    if !self.is_compatible(&declared_type, &expr_type) {
                        return Err(vec![SemanticError {
                            message: format!("Mismatched types in assignment to '{}'. Expected {:?}, found {:?}.", name, declared_type, expr_type),
                            span: expr.span(),
                        }]);
                    }
                }
                self.symbol_table.define(name.clone(), Symbol::Variable(declared_type));
                Ok(())
            }
            Statement::Function(span, name, params, ret_type_expr, body) => {
                let func_type = self.resolve_function_signature(params, ret_type_expr)?;
                self.symbol_table.define(name.clone(), Symbol::Function(Type::Unknown, func_type)); // Update with actual return type
                
                self.symbol_table.enter_scope();
                // Define parameters in new scope
                for param in params {
                    let param_type = self.resolve_type_expression(param.typ.as_ref().unwrap())?;
                    self.symbol_table.define(param.name.0.clone(), Symbol::Variable(param_type));
                }
                self.current_function_return_type = ret_type_expr.as_ref().map(|t| self.resolve_type_expression(t).unwrap_or(Type::Unknown));
                self.analyze_expression(body)?;
                self.current_function_return_type = None;
                self.symbol_table.exit_scope();
                Ok(())
            }
            Statement::Return(span, expr) => {
                let expr_type = self.analyze_expression(expr)?;
                if let Some(expected_return_type) = &self.current_function_return_type {
                    if !self.is_compatible(expected_return_type, &expr_type) {
                        return Err(vec![SemanticError { message: format!("Mismatched return type. Expected {:?}, found {:?}.", expected_return_type, expr_type), span: expr.span() }]);
                    }
                }
                Ok(())
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr)?;
                Ok(())
            }
            Statement::While(span, cond, body) => {
                let cond_type = self.analyze_expression(cond)?;
                if cond_type != Type::Bool {
                    return Err(vec![SemanticError { message: "While condition must be a boolean.".to_string(), span: cond.span() }]);
                }
                self.analyze_expression(body)?;
                Ok(())
            }
            // --- Other Statements ---
            _ => Ok(()), // Placeholder for other statements
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> Result<Type, SemanticError> {
        match expr {
            Expression::Identifier(ident) => {
                if let Some(symbol) = self.symbol_table.lookup(&ident.0) {
                    match symbol {
                        Symbol::Variable(typ) => Ok(typ.clone()),
                        _ => Err(SemanticError { message: format!("Expected a variable, found a {:?}.", symbol), span: ident.1.clone() }),
                    }
                } else {
                    Err(SemanticError { message: format!("Unresolved identifier: '{}'.", ident.0), span: ident.1.clone() })
                }
            }
            Expression::Literal(lit) => match lit {
                Literal::Integer(_, _) => Ok(Type::Int(IntWidth::I32)),
                Literal::Float(_, _) => Ok(Type::Float(FloatWidth::F64)),
                Literal::Boolean(_, _) => Ok(Type::Bool),
                Literal::String(_, _) => Ok(Type::String),
                Literal::Char(_, _) => Ok(Type::Char),
                Literal::Quantum(_, _) => Ok(Type::Qubit),
                Literal::Nano(_, _) => Ok(Type::NanoAgent(None)),
                Literal::MTS(_, _) => Ok(Type::MtsSlice(Box::new(Type::Unknown))), // Placeholder
            },
            Expression::Infix(span, left, op, right) => {
                let left_type = self.analyze_expression(left)?;
                let right_type = self.analyze_expression(right)?;
                // Conceptual type checking for operators
                if !self.is_compatible(&left_type, &right_type) {
                    return Err(SemanticError { message: format!("Incompatible types for operator {:?}: {:?} and {:?}.", op, left_type, right_type), span: span.clone() });
                }
                match op {
                    TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash | TokenType::Modulo => {
                        if matches!(left_type, Type::Int(_) | Type::Float(_)) { Ok(left_type) } else { Err(SemanticError { message: "Arithmetic operations only supported on numeric types.".to_string(), span: span.clone() }) }
                    }
                    TokenType::Equals | TokenType::NotEquals | TokenType::LessThan | TokenType::GreaterThan | TokenType::LessThanEqual | TokenType::GreaterThanEqual => Ok(Type::Bool),
                    TokenType::LogicalAnd | TokenType::LogicalOr => {
                        if left_type == Type::Bool { Ok(Type::Bool) } else { Err(SemanticError { message: "Logical operations only supported on boolean types.".to_string(), span: span.clone() }) }
                    }
                    _ => Ok(Type::Unknown),
                }
            }
            Expression::Block(span, stmts) => {
                self.symbol_table.enter_scope();
                let mut last_type = Type::Unit;
                for stmt in stmts {
                    last_type = match self.analyze_statement(stmt) {
                        Ok(_) => {
                            // If the last statement was an expression, its type is the block's type
                            if let Statement::Expression(expr) = stmt { self.analyze_expression(expr)? } else { Type::Unit }
                        },
                        Err(mut e) => { self.errors.append(&mut e); Type::Error },
                    };
                }
                self.symbol_table.exit_scope();
                Ok(last_type)
            }
            Expression::If(span, cond, cons, alt) => {
                let cond_type = self.analyze_expression(cond)?;
                if cond_type != Type::Bool {
                    return Err(SemanticError { message: "If condition must be a boolean.".to_string(), span: cond.span() });
                }
                let cons_type = self.analyze_expression(cons)?;
                if let Some(alt_expr) = alt {
                    let alt_type = self.analyze_expression(alt_expr)?;
                    if !self.is_compatible(&cons_type, &alt_type) {
                        return Err(SemanticError { message: "If-else branches must return compatible types.".to_string(), span: span.clone() });
                    }
                }
                Ok(cons_type)
            }
            Expression::Call(span, func_expr, args) => {
                let func_type = self.analyze_expression(func_expr)?; // This should resolve to Function type
                if let Type::Function(param_types, ret_type) = func_type {
                    if param_types.len() != args.len() {
                        return Err(SemanticError { message: format!("Expected {} arguments, found {}.", param_types.len(), args.len()), span: span.clone() });
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.analyze_expression(arg)?; 
                        if !self.is_compatible(&param_types[i], &arg_type) {
                            return Err(SemanticError { message: format!("Argument {} type mismatch. Expected {:?}, found {:?}.", i, param_types[i], arg_type), span: arg.span() });
                        }
                    }
                    Ok(*ret_type)
                } else {
                    Err(SemanticError { message: format!("Attempted to call non-function type {:?}.", func_type), span: func_expr.span() })
                }
            }
            // --- OOP Expressions ---
            Expression::NewInstance(span, class_name, args) => self.analyze_new_instance(span.clone(), class_name.clone(), args),
            Expression::MethodCall(span, object, method_name, args) => self.analyze_method_call(span.clone(), object, method_name.clone(), args),
            Expression::FieldAccess(span, object, field_name) => self.analyze_field_access(span.clone(), object, field_name.clone()),
            Expression::This(span) => self.analyze_this(span.clone()),
            Expression::Super(span) => self.analyze_super(span.clone()),

            _ => Err(SemanticError { message: format!("Unsupported expression: {:?}", expr), span: expr.span() }),
        }
    }

    // --- Helper for Function Signatures ---
    fn resolve_function_signature(&mut self, params: &[Parameter], ret_type_expr: &Option<TypeExpr>) -> Result<Type, SemanticError> {
        let param_types: Result<Vec<Type>, SemanticError> = params.iter()
            .map(|p| self.resolve_type_expression(p.typ.as_ref().unwrap()))
            .collect();
        let return_type = ret_type_expr.as_ref().map_or(Ok(Type::Unit), |t| self.resolve_type_expression(t))?;
        Ok(Type::Function(param_types?, Box::new(return_type)))
    }

    // --- Type Resolution ---
    fn resolve_type_expression(&mut self, type_expr: &TypeExpr) -> Result<Type, SemanticError> {
        match type_expr {
            TypeExpr::Identifier(ident) => {
                match ident.0.as_str() {
                    "unit" => Ok(Type::Unit),
                    "bool" => Ok(Type::Bool),
                    "char" => Ok(Type::Char),
                    "int" => Ok(Type::Int(IntWidth::I32)), // Default int
                    "float" => Ok(Type::Float(FloatWidth::F64)), // Default float
                    "string" => Ok(Type::String),
                    "Qubit" => Ok(Type::Qubit),
                    // Look up in symbol table for user-defined types (classes, interfaces, type aliases)
                    _ => {
                        if let Some(symbol) = self.symbol_table.lookup(&ident.0) {
                            match symbol {
                                Symbol::TypeAlias(t) => Ok(t.clone()),
                                Symbol::ClassRef(class_ident) => {
                                    self.symbol_table.lookup_class_type(&class_ident.0)
                                        .cloned()
                                        .ok_or_else(|| SemanticError { message: format!("Failed to resolve class type '{}'.", class_ident.0), span: ident.1.clone() })
                                },
                                Symbol::InterfaceRef(iface_ident) => {
                                    self.symbol_table.lookup_interface_type(&iface_ident.0)
                                        .cloned()
                                        .ok_or_else(|| SemanticError { message: format!("Failed to resolve interface type '{}'.", iface_ident.0), span: ident.1.clone() })
                                },
                                _ => Err(SemanticError { message: format!("Identifier '{}' is not a type.", ident.0), span: ident.1.clone() }),
                            }
                        } else {
                            Err(SemanticError { message: format!("Unknown type: '{}'.", ident.0), span: ident.1.clone() })
                        }
                    }
                }
            }
            TypeExpr::Array(inner_type_expr) => {
                let inner_type = self.resolve_type_expression(inner_type_expr)?;
                Ok(Type::Array(Box::new(inner_type), None))
            }
        }
    }

    // --- New OOP Semantic Analysis Methods ---

    fn analyze_class_declaration(&mut self, span: Span, name: Identifier, parents: &[Identifier], members: &[ClassMember]) -> Result<(), Vec<SemanticError>> {
        let mut errors = Vec::new();
        let class_name_str = name.0.clone();

        let mut fields = HashMap::new();
        let mut methods = HashMap::new();
        let mut parent_class_type: Option<Box<Type>> = None;
        let mut implemented_interface_types: Vec<Type> = Vec::new();
        let mut is_abstract = false;

        // Resolve parents (check for single inheritance for class, multiple for interfaces)
        for parent_ident in parents {
            match self.symbol_table.lookup(&parent_ident.0) {
                Some(Symbol::ClassRef(_)) => {
                    if parent_class_type.is_some() {
                        errors.push(SemanticError { message: "Class can only extend one parent class.".to_string(), span: parent_ident.1.clone() });
                        continue;
                    }
                    parent_class_type = Some(Box::new(self.resolve_type_expression(&TypeExpr::Identifier(parent_ident.clone()))?));
                }
                Some(Symbol::InterfaceRef(_)) => {
                    implemented_interface_types.push(self.resolve_type_expression(&TypeExpr::Identifier(parent_ident.clone()))?);
                }
                _ => errors.push(SemanticError { message: format!("Undefined parent class or interface '{}'.", parent_ident.0), span: parent_ident.1.clone() }),
            }
        }

        // Gather members' info
        for member in members {
            match member {
                ClassMember::Field(field_span, access, field_name, type_expr, initializer) => {
                    let field_type = self.resolve_type_expression(type_expr)?;
                    fields.insert(field_name.0.clone(), field_type);
                    if let Some(init_expr) = initializer {
                        // Analyze initializer in context of the class (conceptual)
                        let init_type = self.analyze_expression(init_expr)?; 
                        if !self.is_compatible(fields.get(&field_name.0).unwrap(), &init_type) {
                            errors.push(SemanticError { message: format!("Field '{}' initializer type mismatch. Expected {:?}, found {:?}.", field_name.0, fields.get(&field_name.0).unwrap(), init_type), span: init_expr.span() });
                        }
                    }
                }
                ClassMember::Method(method_span, access, method_mod, method_name, params, ret_type_expr, body, effects) => {
                    let param_types_res: Result<Vec<Type>, SemanticError> = params.iter().map(|p| self.resolve_type_expression(p.typ.as_ref().unwrap())).collect();
                    if let Err(e) = param_types_res { errors.push(e); continue; }
                    let param_types = param_types_res.unwrap();

                    let return_type = ret_type_expr.as_ref().map_or(Ok(Type::Unit), |t| self.resolve_type_expression(t))?;
                    
                    if method_mod == Some(MethodModifier::Abstract) { is_abstract = true; }

                    methods.insert(method_name.0.clone(), MethodType {
                        params: param_types,
                        return_type: Box::new(return_type),
                        access_modifier: access.clone(),
                        method_modifier: method_mod.clone(),
                        effects: effects.clone(),
                    });
                }
            }
        }

        let class_type = Type::Class {
            name: name.clone(),
            fields,
            methods,
            parent_class: parent_class_type,
            implemented_interfaces: implemented_interface_types,
            is_abstract,
        };
        self.symbol_table.define_class_type(class_name_str.clone(), class_type.clone());

        // Pass 3: Analyze method bodies in their own scope, with 'this'
        self.current_class_context = Some(class_type.clone());
        for member in members {
            if let ClassMember::Method(_, _, _, method_name, params, _, body, _) = member {
                self.symbol_table.enter_scope();
                self.symbol_table.define("this".to_string(), Symbol::Variable(self.current_class_context.clone().unwrap())); // Define 'this'
                // Define method parameters
                for param in params {
                    let param_type = self.resolve_type_expression(param.typ.as_ref().unwrap())?;
                    self.symbol_table.define(param.name.0.clone(), Symbol::Variable(param_type));
                }
                // Check body
                self.analyze_expression(body)?;
                self.symbol_table.exit_scope();
            }
        }
        self.current_class_context = None;

        // Perform inheritance checks (conceptual)
        // - Override checks (conceptual: ensure @override methods match parent signature)
        // - Interface implementation checks (conceptual: ensure all interface methods are implemented if not abstract)

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn analyze_interface_declaration(&mut self, span: Span, name: Identifier, parents: &[Identifier], members: &[InterfaceMember]) -> Result<(), Vec<SemanticError>> {
        let mut errors = Vec::new();
        let interface_name_str = name.0.clone();
        let mut methods = HashMap::new();
        let mut parent_interface_types: Vec<Type> = Vec::new();

        // Resolve parent interfaces
        for parent_ident in parents {
            match self.symbol_table.lookup(&parent_ident.0) {
                Some(Symbol::InterfaceRef(_)) => {
                    parent_interface_types.push(self.resolve_type_expression(&TypeExpr::Identifier(parent_ident.clone()))?);
                }
                _ => errors.push(SemanticError { message: format!("Undefined parent interface '{}'.", parent_ident.0), span: parent_ident.1.clone() }),
            }
        }

        // Gather method signatures
        for member in members {
            if let InterfaceMember::MethodSignature(method_span, method_name, params, ret_type_expr, effects) = member {
                let param_types_res: Result<Vec<Type>, SemanticError> = params.iter().map(|p| self.resolve_type_expression(p.typ.as_ref().unwrap())).collect();
                if let Err(e) = param_types_res { errors.push(e); continue; }
                let param_types = param_types_res.unwrap();

                let return_type = ret_type_expr.as_ref().map_or(Ok(Type::Unit), |t| self.resolve_type_expression(t))?;
                methods.insert(method_name.0.clone(), MethodType {
                    params: param_types,
                    return_type: Box::new(return_type),
                    access_modifier: AccessModifier::Public, // Interfaces are public implicitly
                    method_modifier: Some(MethodModifier::Abstract), // Interface methods are abstract implicitly
                    effects: effects.clone(),
                });
            }
        }

        let interface_type = Type::Interface {
            name: name.clone(),
            methods,
            parent_interfaces: parent_interface_types,
        };
        self.symbol_table.define_interface_type(interface_name_str.clone(), interface_type);
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn analyze_new_instance(&mut self, span: Span, class_name: Identifier, args: &[Expression]) -> Result<Type, SemanticError> {
        match self.symbol_table.lookup_class_type(&class_name.0) {
            Some(Type::Class { name, is_abstract, methods: class_methods, .. }) => { 
                if *is_abstract {
                    return Err(SemanticError { message: format!("Cannot instantiate abstract class '{}'.", class_name.0), span: class_name.1 });
                }
                // Conceptual: Resolve constructor (implicitly a method named 'new' or default) and type check arguments
                // For simplicity, assume a default constructor with no args for now.
                if !args.is_empty() {
                    return Err(SemanticError { message: "Conceptual: Default constructor takes no arguments.".to_string(), span: span });
                }
                Ok(Type::Class { name: name.clone(), fields: HashMap::new(), methods: class_methods.clone(), parent_class: None, implemented_interfaces: Vec::new(), is_abstract: false })
            }
            _ => Err(SemanticError { message: format!("Unknown or non-instantiable class '{}'.", class_name.0), span: class_name.1 }),
        }
    }

    fn analyze_method_call(&mut self, span: Span, object_expr: &Expression, method_name: Identifier, args: &[Expression]) -> Result<Type, SemanticError> {
        let obj_type = self.analyze_expression(object_expr)?; // Get type of 'object'
        match obj_type {
            Type::Class { methods: class_methods, parent_class, .. } => {
                // Conceptual: Lookup method in class hierarchy (current class first, then parent_class)
                if let Some(method_type) = class_methods.get(&method_name.0) {
                    // Check access modifier (conceptual: needs current context for private/protected)
                    // Check arguments compatibility
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.analyze_expression(arg)?;
                        if i >= method_type.params.len() || !self.is_compatible(&method_type.params[i], &arg_type) {
                            return Err(SemanticError { message: format!("Method '{}' argument {} type mismatch. Expected {:?}, found {:?}.", method_name.0, i, method_type.params.get(i), arg_type), span: arg.span() });
                        }
                    }
                    Ok(*method_type.return_type.clone())
                } else {
                    Err(SemanticError { message: format!("Method '{}' not found on class type '{}'.", method_name.0, obj_type.get_name().0), span: method_name.1 })
                }
            }
            Type::Interface { methods: iface_methods, .. } => {
                 // Conceptual: Lookup method in interface hierarchy
                if let Some(method_type) = iface_methods.get(&method_name.0) {
                    // Interface methods are implicitly public, no access check needed.
                    // Check arguments compatibility
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.analyze_expression(arg)?;
                        if i >= method_type.params.len() || !self.is_compatible(&method_type.params[i], &arg_type) {
                            return Err(SemanticError { message: format!("Interface method '{}' argument {} type mismatch. Expected {:?}, found {:?}.", method_name.0, i, method_type.params.get(i), arg_type), span: arg.span() });
                        }
                    }
                    Ok(*method_type.return_type.clone())
                } else {
                    Err(SemanticError { message: format!("Method '{}' not found on interface type '{}'.", method_name.0, obj_type.get_name().0), span: method_name.1 })
                }
            }
            _ => Err(SemanticError { message: format!("Cannot call method '{}' on non-object type {:?}.".to_string(), method_name.0, obj_type), span: object_expr.span() }),
        }
    }

    fn analyze_field_access(&mut self, span: Span, object_expr: &Expression, field_name: Identifier) -> Result<Type, SemanticError> {
        let obj_type = self.analyze_expression(object_expr)?; // Get type of 'object'
        match obj_type {
            Type::Class { fields: class_fields, parent_class, .. } => {
                // Conceptual: Lookup field in class hierarchy
                if let Some(field_type) = class_fields.get(&field_name.0) {
                    // Check access modifier (conceptual)
                    Ok(field_type.clone())
                } else {
                    Err(SemanticError { message: format!("Field '{}' not found on class type '{}'.", field_name.0, obj_type.get_name().0), span: field_name.1 })
                }
            }
            _ => Err(SemanticError { message: format!("Cannot access field '{}' on non-object type {:?}.".to_string(), field_name.0, obj_type), span: object_expr.span() }),
        }
    }

    fn analyze_this(&mut self, span: Span) -> Result<Type, SemanticError> {
        if let Some(class_type) = &self.current_class_context {
            Ok(class_type.clone())
        } else {
            Err(SemanticError { message: "'this' can only be used inside a class method.".to_string(), span: span.clone() })
        }
    }

    fn analyze_super(&mut self, span: Span) -> Result<Type, SemanticError> {
        if let Some(Type::Class { parent_class: Some(parent_type), .. }) = &self.current_class_context {
            Ok(*parent_type.clone())
        } else {
            Err(SemanticError { message: "'super' can only be used inside a method of a class with a parent class.".to_string(), span: span.clone() })
        }
    }

    fn is_compatible(&self, target: &Type, source: &Type) -> bool {
        // Conceptual: Implement subtyping rules
        // e.g., if source is Class A, target is Interface I, and A implements I, then compatible
        match (target, source) {
            (t, s) if t == s => true,
            // Conceptual: if source is a Class and target is an Interface it implements
            (Type::Interface { name: target_iface_name, .. }, Type::Class { implemented_interfaces, .. }) => {
                implemented_interfaces.iter().any(|iface_type| 
                    if let Type::Interface { name: iface_name, .. } = iface_type { target_iface_name == iface_name } else { false }
                )
            }
            // Conceptual: if source is a subclass of target
            (Type::Class { name: target_name, .. }, Type::Class { parent_class: Some(parent), .. }) => {
                self.is_compatible(target, parent) // Recursive check up the inheritance chain
            }
            _ => false,
        }
    }

    pub fn get_global_symbols(&self) -> &SymbolTable {
        &self.symbol_table
    }

    // Helper for adding errors
    fn add_error(&mut self, message: String, span: Span) {
        self.errors.push(SemanticError { message, span });
    }
}
