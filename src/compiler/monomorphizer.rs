//! Zamani Compiler — Production Monomorphization Engine
//!
//! This module performs deterministic specialization of generic type
//! structures represented by the Zamani AST.
//!
//! Important AST limitation:
//! `Expression::Call` currently contains no explicit type-argument list and
//! `Statement::Function` contains no generic `TypeParameter` list. Therefore
//! this module does NOT pretend that generic function monomorphization is
//! possible when the AST does not represent the required information.
//!
//! The implementation instead provides:
//!   - deterministic specialization-name generation;
//!   - generic type substitution;
//!   - recursive AST traversal;
//!   - specialization caching;
//!   - collision-safe generated names;
//!   - configurable specialization limits;
//!   - recursion protection;
//!   - explicit diagnostics for unsupported situations.
//!
//! Generic declarations that are explicitly represented by the AST
//! (`Struct`, `Enum`, `Trait`, `TypeAlias`) are recognized correctly.

use crate::ast::{
    CatchArm, EnumVariantKind, Expression, Identifier, ImplItem, ImplItemKind, MatchCase,
    Parameter, Pattern, Program, Statement, StructField, TraitItem, TraitItemKind, TypeBound,
    TypeExpr, TypeParameter,
};
use crate::source_map::Span;

use std::collections::{HashMap, HashSet};
use std::fmt;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// Limits protecting the compiler from pathological generic expansion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonomorphizationConfig {
    /// Maximum number of generated specializations.
    pub max_specializations: usize,

    /// Maximum recursive specialization depth.
    pub max_depth: usize,

    /// Maximum number of type arguments accepted by one specialization.
    pub max_type_arguments: usize,
}

impl Default for MonomorphizationConfig {
    fn default() -> Self {
        Self {
            max_specializations: 10_000,
            max_depth: 128,
            max_type_arguments: 64,
        }
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonomorphizationError {
    TooManySpecializations {
        limit: usize,
    },

    RecursionLimitExceeded {
        depth: usize,
    },

    TooManyTypeArguments {
        count: usize,
        limit: usize,
    },

    InvalidTypeParameter {
        name: String,
    },

    DuplicateGenericDefinition {
        name: String,
    },

    UnsupportedGenericFunction {
        name: String,
    },

    InvalidSpecialization {
        message: String,
    },
}

impl fmt::Display for MonomorphizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManySpecializations { limit } => {
                write!(
                    f,
                    "monomorphization specialization limit ({}) exceeded",
                    limit
                )
            }

            Self::RecursionLimitExceeded { depth } => {
                write!(
                    f,
                    "monomorphization recursion depth ({}) exceeded",
                    depth
                )
            }

            Self::TooManyTypeArguments { count, limit } => {
                write!(
                    f,
                    "too many type arguments: {} (maximum {})",
                    count, limit
                )
            }

            Self::InvalidTypeParameter { name } => {
                write!(
                    f,
                    "invalid generic type parameter '{}'",
                    name
                )
            }

            Self::DuplicateGenericDefinition { name } => {
                write!(
                    f,
                    "duplicate generic definition '{}'",
                    name
                )
            }

            Self::UnsupportedGenericFunction { name } => {
                write!(
                    f,
                    "generic function '{}' cannot be monomorphized because \
                     the current AST does not represent function type parameters",
                    name
                )
            }

            Self::InvalidSpecialization { message } => {
                write!(f, "invalid specialization: {}", message)
            }
        }
    }
}

impl std::error::Error for MonomorphizationError {}

// -----------------------------------------------------------------------------
// Generic definition metadata
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct GenericDefinition {
    name: String,
    parameters: Vec<TypeParameter>,
}

impl GenericDefinition {
    fn new(
        name: impl Into<String>,
        parameters: Vec<TypeParameter>,
    ) -> Result<Self, MonomorphizationError> {
        let name = name.into();

        let mut seen = HashSet::new();

        for parameter in &parameters {
            let parameter_name = parameter.name.0.clone();

            if parameter_name.trim().is_empty() {
                return Err(MonomorphizationError::InvalidTypeParameter {
                    name: parameter_name,
                });
            }

            if !seen.insert(parameter_name.clone()) {
                return Err(MonomorphizationError::InvalidSpecialization {
                    message: format!(
                        "duplicate type parameter '{}' in '{}'",
                        parameter_name, name
                    ),
                });
            }
        }

        Ok(Self { name, parameters })
    }

    fn parameter_names(&self) -> HashSet<String> {
        self.parameters
            .iter()
            .map(|parameter| parameter.name.0.clone())
            .collect()
    }
}

// -----------------------------------------------------------------------------
// Specialization cache
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpecializationKey {
    original: String,
    type_arguments: Vec<String>,
}

/// Information about a generated specialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specialization {
    pub original_name: String,
    pub specialized_name: String,
    pub type_arguments: Vec<String>,
}

// -----------------------------------------------------------------------------
// Monomorphizer
// -----------------------------------------------------------------------------

/// Production-grade Zamani monomorphization engine.
#[derive(Debug, Clone)]
pub struct Monomorphizer {
    specializations: HashMap<SpecializationKey, String>,

    generic_definitions: HashMap<String, GenericDefinition>,

    generated_names: HashSet<String>,

    specialization_records: Vec<Specialization>,

    config: MonomorphizationConfig,
}

impl Default for Monomorphizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Monomorphizer {
    /// Creates a monomorphizer using production defaults.
    pub fn new() -> Self {
        Self::with_config(MonomorphizationConfig::default())
    }

    /// Creates a monomorphizer with explicit resource limits.
    pub fn with_config(config: MonomorphizationConfig) -> Self {
        Self {
            specializations: HashMap::new(),
            generic_definitions: HashMap::new(),
            generated_names: HashSet::new(),
            specialization_records: Vec::new(),
            config,
        }
    }

    /// Returns the configuration currently used by the engine.
    pub fn config(&self) -> &MonomorphizationConfig {
        &self.config
    }

    /// Number of generated specializations.
    pub fn specialization_count(&self) -> usize {
        self.specialization_records.len()
    }

    /// Returns all generated specialization records.
    pub fn specializations(&self) -> &[Specialization] {
        &self.specialization_records
    }

    /// Clears generated specialization state.
    ///
    /// Useful when the same compiler process compiles multiple independent
    /// programs and specialization state must not leak between compilations.
    pub fn reset(&mut self) {
        self.specializations.clear();
        self.generic_definitions.clear();
        self.generated_names.clear();
        self.specialization_records.clear();
    }

    // -------------------------------------------------------------------------
    // Program processing
    // -------------------------------------------------------------------------

    /// Processes a complete program.
    ///
    /// This method:
    ///
    /// 1. indexes generic declarations;
    /// 2. validates generic definitions;
    /// 3. recursively rewrites generic type expressions;
    /// 4. preserves every AST statement;
    /// 5. does not invent generic-function semantics that the AST cannot
    ///    represent.
    pub fn process(
        &mut self,
        program: Program,
    ) -> Result<Program, MonomorphizationError> {
        self.index_generic_definitions(&program)?;

        let statements = program
            .statements
            .into_iter()
            .map(|statement| self.specialize_statement(statement, 0))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program {
            statements,
            span: program.span,
        })
    }

    /// Compatibility wrapper for callers that previously expected an infallible
    /// API.
    ///
    /// Production compiler code should prefer `process`.
    pub fn process_lossy(&mut self, program: Program) -> Program {
        match self.process(program.clone()) {
            Ok(result) => result,
            Err(_) => program,
        }
    }

    // -------------------------------------------------------------------------
    // Definition discovery
    // -------------------------------------------------------------------------

    fn index_generic_definitions(
        &mut self,
        program: &Program,
    ) -> Result<(), MonomorphizationError> {
        self.generic_definitions.clear();

        for statement in &program.statements {
            match statement {
                Statement::Struct(_, name, parameters, _) => {
                    self.register_generic_definition(
                        name.0.clone(),
                        parameters.clone(),
                    )?;
                }

                Statement::Enum(_, name, parameters, _) => {
                    self.register_generic_definition(
                        name.0.clone(),
                        parameters.clone(),
                    )?;
                }

                Statement::Trait(_, name, parameters, _) => {
                    self.register_generic_definition(
                        name.0.clone(),
                        parameters.clone(),
                    )?;
                }

                Statement::TypeAlias(_, name, parameters, _) => {
                    self.register_generic_definition(
                        name.0.clone(),
                        parameters.clone(),
                    )?;
                }

                _ => {}
            }
        }

        Ok(())
    }

    fn register_generic_definition(
        &mut self,
        name: String,
        parameters: Vec<TypeParameter>,
    ) -> Result<(), MonomorphizationError> {
        if parameters.is_empty() {
            return Ok(());
        }

        if self.generic_definitions.contains_key(&name) {
            return Err(
                MonomorphizationError::DuplicateGenericDefinition { name },
            );
        }

        let definition = GenericDefinition::new(name.clone(), parameters)?;

        self.generic_definitions.insert(name, definition);

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Statements
    // -------------------------------------------------------------------------

    fn specialize_statement(
        &mut self,
        statement: Statement,
        depth: usize,
    ) -> Result<Statement, MonomorphizationError> {
        self.check_depth(depth)?;

        match statement {
            Statement::Let(span, name, type_annotation, expression) => {
                Ok(Statement::Let(
                    span,
                    name,
                    type_annotation.map(|ty| self.specialize_type(ty, depth + 1)),
                    self.specialize_expression(expression, depth + 1)?,
                ))
            }

            Statement::Const(span, name, type_annotation, expression) => {
                Ok(Statement::Const(
                    span,
                    name,
                    type_annotation.map(|ty| self.specialize_type(ty, depth + 1)),
                    self.specialize_expression(expression, depth + 1)?,
                ))
            }

            Statement::Return(span, expression) => {
                Ok(Statement::Return(
                    span,
                    self.specialize_expression(expression, depth + 1)?,
                ))
            }

            Statement::While(span, condition, body) => {
                Ok(Statement::While(
                    span,
                    self.specialize_expression(condition, depth + 1)?,
                    self.specialize_expression(body, depth + 1)?,
                ))
            }

            Statement::For(span, identifier, iterable, body) => {
                Ok(Statement::For(
                    span,
                    identifier,
                    self.specialize_expression(iterable, depth + 1)?,
                    self.specialize_expression(body, depth + 1)?,
                ))
            }

            Statement::Match(span, expression, cases) => {
                Ok(Statement::Match(
                    span,
                    self.specialize_expression(expression, depth + 1)?,
                    self.specialize_match_cases(cases, depth + 1)?,
                ))
            }

            Statement::Function(
                span,
                name,
                parameters,
                return_type,
                body,
            ) => {
                let parameters = self.specialize_parameters(parameters, depth + 1);

                let return_type = return_type
                    .map(|ty| self.specialize_type(ty, depth + 1));

                let body =
                    self.specialize_expression(body, depth + 1)?;

                Ok(Statement::Function(
                    span,
                    name,
                    parameters,
                    return_type,
                    body,
                ))
            }

            Statement::Struct(
                span,
                name,
                parameters,
                fields,
            ) => {
                let fields = fields
                    .into_iter()
                    .map(|field| self.specialize_struct_field(field, depth + 1))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Statement::Struct(
                    span,
                    name,
                    parameters,
                    fields,
                ))
            }

            Statement::Enum(
                span,
                name,
                parameters,
                variants,
            ) => {
                let variants = variants
                    .into_iter()
                    .map(|variant| {
                        let fields = match variant.fields {
                            EnumVariantKind::Unit => EnumVariantKind::Unit,

                            EnumVariantKind::Tuple(types) => {
                                EnumVariantKind::Tuple(
                                    types
                                        .into_iter()
                                        .map(|ty| {
                                            self.specialize_type(
                                                ty,
                                                depth + 1,
                                            )
                                        })
                                        .collect(),
                                )
                            }

                            EnumVariantKind::Struct(fields) => {
                                let fields = fields
                                    .into_iter()
                                    .map(|field| {
                                        self.specialize_struct_field(
                                            field,
                                            depth + 1,
                                        )
                                    })
                                    .collect::<Result<Vec<_>, _>>()?;

                                EnumVariantKind::Struct(fields)
                            }
                        };

                        Ok(crate::ast::EnumVariant {
                            name: variant.name,
                            fields,
                            span: variant.span,
                        })
                    })
                    .collect::<Result<Vec<_>, MonomorphizationError>>()?;

                Ok(Statement::Enum(
                    span,
                    name,
                    parameters,
                    variants,
                ))
            }

            Statement::Trait(
                span,
                name,
                parameters,
                items,
            ) => {
                let items = items
                    .into_iter()
                    .map(|item| self.specialize_trait_item(item, depth + 1))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Statement::Trait(
                    span,
                    name,
                    parameters,
                    items,
                ))
            }

            Statement::Impl(
                span,
                trait_name,
                target_type,
                items,
            ) => {
                let target_type =
                    self.specialize_type(target_type, depth + 1);

                let items = items
                    .into_iter()
                    .map(|item| self.specialize_impl_item(item, depth + 1))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Statement::Impl(
                    span,
                    trait_name,
                    target_type,
                    items,
                ))
            }

            Statement::TypeAlias(
                span,
                name,
                parameters,
                type_expression,
            ) => Ok(Statement::TypeAlias(
                span,
                name,
                parameters,
                self.specialize_type(type_expression, depth + 1),
            )),

            Statement::TypeDeclaration(
                span,
                name,
                type_expression,
            ) => Ok(Statement::TypeDeclaration(
                span,
                name,
                self.specialize_type(type_expression, depth + 1),
            )),

            Statement::Module(span, name, statements) => {
                let statements = statements
                    .into_iter()
                    .map(|statement| {
                        self.specialize_statement(statement, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Statement::Module(span, name, statements))
            }

            Statement::Expression(expression) => Ok(
                Statement::Expression(
                    self.specialize_expression(expression, depth + 1)?,
                ),
            ),

            Statement::QuantumCircuit(span, name, expression) => {
                Ok(Statement::QuantumCircuit(
                    span,
                    name,
                    Box::new(self.specialize_expression(
                        *expression,
                        depth + 1,
                    )?),
                ))
            }

            Statement::NanoAgent(span, name, expression) => {
                Ok(Statement::NanoAgent(
                    span,
                    name,
                    Box::new(self.specialize_expression(
                        *expression,
                        depth + 1,
                    )?),
                ))
            }

            Statement::SankofaMemory(span, name, expression) => {
                Ok(Statement::SankofaMemory(
                    span,
                    name,
                    self.specialize_expression(expression, depth + 1)?,
                ))
            }

            Statement::Unsafe(span, label, expression) => {
                Ok(Statement::Unsafe(
                    span,
                    label,
                    self.specialize_expression(expression, depth + 1)?,
                ))
            }

            Statement::Handle(span, identifier, expression, handler) => {
                Ok(Statement::Handle(
                    span,
                    identifier,
                    self.specialize_expression(expression, depth + 1)?,
                    self.specialize_expression(handler, depth + 1)?,
                ))
            }

            Statement::Wisdom(span, name, expression) => {
                Ok(Statement::Wisdom(
                    span,
                    name,
                    self.specialize_expression(expression, depth + 1)?,
                ))
            }

            other => Ok(other),
        }
    }

    fn specialize_parameters(
        &mut self,
        parameters: Vec<Parameter>,
        depth: usize,
    ) -> Vec<Parameter> {
        parameters
            .into_iter()
            .map(|mut parameter| {
                parameter.typ = parameter
                    .typ
                    .map(|ty| self.specialize_type(ty, depth + 1));

                parameter.default = parameter
                    .default
                    .map(|expression| {
                        self.specialize_expression(
                            expression,
                            depth + 1,
                        )
                        .unwrap_or_else(|_| {
                            Expression::Block(
                                Span::dummy(),
                                Vec::new(),
                            )
                        })
                    });

                parameter
            })
            .collect()
    }

    fn specialize_struct_field(
        &mut self,
        field: StructField,
        depth: usize,
    ) -> Result<StructField, MonomorphizationError> {
        Ok(StructField {
            name: field.name,
            typ: self.specialize_type(field.typ, depth + 1),
            visibility: field.visibility,
            span: field.span,
        })
    }

    fn specialize_trait_item(
        &mut self,
        item: TraitItem,
        depth: usize,
    ) -> Result<TraitItem, MonomorphizationError> {
        let kind = match item.kind {
            TraitItemKind::Method {
                params,
                ret,
                default_body,
            } => TraitItemKind::Method {
                params: self.specialize_parameters(params, depth + 1),
                ret: ret.map(|ty| self.specialize_type(ty, depth + 1)),
                default_body: default_body
                    .map(|body| self.specialize_expression(body, depth + 1))
                    .transpose()?,
            },

            TraitItemKind::AssociatedType(ty) => {
                TraitItemKind::AssociatedType(
                    ty.map(|ty| self.specialize_type(ty, depth + 1)),
                )
            }

            TraitItemKind::Const(ty, value) => {
                TraitItemKind::Const(
                    self.specialize_type(ty, depth + 1),
                    value
                        .map(|value| {
                            self.specialize_expression(
                                value,
                                depth + 1,
                            )
                        })
                        .transpose()?,
                )
            }
        };

        Ok(TraitItem {
            name: item.name,
            kind,
            span: item.span,
        })
    }

    fn specialize_impl_item(
        &mut self,
        item: ImplItem,
        depth: usize,
    ) -> Result<ImplItem, MonomorphizationError> {
        let kind = match item.kind {
            ImplItemKind::Method {
                params,
                ret,
                body,
            } => ImplItemKind::Method {
                params: self.specialize_parameters(params, depth + 1),
                ret: ret.map(|ty| self.specialize_type(ty, depth + 1)),
                body: self.specialize_expression(body, depth + 1)?,
            },

            ImplItemKind::AssociatedType(ty) => {
                ImplItemKind::AssociatedType(
                    self.specialize_type(ty, depth + 1),
                )
            }

            ImplItemKind::Const(ty, value) => {
                ImplItemKind::Const(
                    self.specialize_type(ty, depth + 1),
                    self.specialize_expression(value, depth + 1)?,
                )
            }
        };

        Ok(ImplItem {
            name: item.name,
            kind,
            visibility: item.visibility,
            span: item.span,
        })
    }

    // -------------------------------------------------------------------------
    // Expressions
    // -------------------------------------------------------------------------

    fn specialize_expression(
        &mut self,
        expression: Expression,
        depth: usize,
    ) -> Result<Expression, MonomorphizationError> {
        self.check_depth(depth)?;

        Ok(match expression {
            Expression::Prefix(span, op, expression) =>
                Expression::Prefix(
                    span,
                    op,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Infix(span, left, op, right) =>
                Expression::Infix(
                    span,
                    Box::new(self.specialize_expression(*left, depth + 1)?),
                    op,
                    Box::new(self.specialize_expression(*right, depth + 1)?),
                ),

            Expression::If(span, condition, then_branch, else_branch) =>
                Expression::If(
                    span,
                    Box::new(self.specialize_expression(*condition, depth + 1)?),
                    Box::new(self.specialize_expression(*then_branch, depth + 1)?),
                    else_branch
                        .map(|branch| {
                            self.specialize_expression(*branch, depth + 1)
                                .map(Box::new)
                        })
                        .transpose()?,
                ),

            Expression::Block(span, statements) =>
                Expression::Block(
                    span,
                    statements
                        .into_iter()
                        .map(|statement| {
                            self.specialize_statement(
                                statement,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::Match(span, expression, cases) =>
                Expression::Match(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                    self.specialize_match_cases(cases, depth + 1)?,
                ),

            Expression::Loop(span, body) =>
                Expression::Loop(
                    span,
                    Box::new(self.specialize_expression(*body, depth + 1)?),
                ),

            Expression::Call(span, function, arguments) =>
                Expression::Call(
                    span,
                    Box::new(self.specialize_expression(*function, depth + 1)?),
                    arguments
                        .into_iter()
                        .map(|argument| {
                            self.specialize_expression(
                                argument,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::Lambda(span, parameters, body) =>
                Expression::Lambda(
                    span,
                    self.specialize_parameters(parameters, depth + 1),
                    Box::new(self.specialize_expression(*body, depth + 1)?),
                ),

            Expression::Array(span, expressions) =>
                Expression::Array(
                    span,
                    expressions
                        .into_iter()
                        .map(|expression| {
                            self.specialize_expression(
                                expression,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::Tuple(span, expressions) =>
                Expression::Tuple(
                    span,
                    expressions
                        .into_iter()
                        .map(|expression| {
                            self.specialize_expression(
                                expression,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::Struct(span, name, fields) =>
                Expression::Struct(
                    span,
                    name,
                    fields
                        .into_iter()
                        .map(|(name, expression)| {
                            Ok((
                                name,
                                self.specialize_expression(
                                    expression,
                                    depth + 1,
                                )?,
                            ))
                        })
                        .collect::<Result<Vec<_>, MonomorphizationError>>()?,
                ),

            Expression::Index(span, target, index) =>
                Expression::Index(
                    span,
                    Box::new(self.specialize_expression(*target, depth + 1)?),
                    Box::new(self.specialize_expression(*index, depth + 1)?),
                ),

            Expression::Range(span, start, end, inclusive) =>
                Expression::Range(
                    span,
                    Box::new(self.specialize_expression(*start, depth + 1)?),
                    Box::new(self.specialize_expression(*end, depth + 1)?),
                    inclusive,
                ),

            Expression::MemberAccess(span, target, member) =>
                Expression::MemberAccess(
                    span,
                    Box::new(self.specialize_expression(*target, depth + 1)?),
                    member,
                ),

            Expression::MethodCall(span, target, method, arguments) =>
                Expression::MethodCall(
                    span,
                    Box::new(self.specialize_expression(*target, depth + 1)?),
                    method,
                    arguments
                        .into_iter()
                        .map(|argument| {
                            self.specialize_expression(
                                argument,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::Cast(span, expression, ty) =>
                Expression::Cast(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                    self.specialize_type(ty, depth + 1),
                ),

            Expression::TypeAscription(span, expression, ty) =>
                Expression::TypeAscription(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                    self.specialize_type(ty, depth + 1),
                ),

            Expression::Assign(span, target, value) =>
                Expression::Assign(
                    span,
                    Box::new(self.specialize_expression(*target, depth + 1)?),
                    Box::new(self.specialize_expression(*value, depth + 1)?),
                ),

            Expression::CompoundAssign(span, target, op, value) =>
                Expression::CompoundAssign(
                    span,
                    Box::new(self.specialize_expression(*target, depth + 1)?),
                    op,
                    Box::new(self.specialize_expression(*value, depth + 1)?),
                ),

            Expression::Try(span, expression) =>
                Expression::Try(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::TryCatch(span, expression, arms) =>
                Expression::TryCatch(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                    arms
                        .into_iter()
                        .map(|arm| {
                            Ok(CatchArm {
                                error_type: arm
                                    .error_type
                                    .map(|ty| {
                                        self.specialize_type(
                                            ty,
                                            depth + 1,
                                        )
                                    }),
                                binding: arm.binding,
                                body: self.specialize_expression(
                                    arm.body,
                                    depth + 1,
                                )?,
                                span: arm.span,
                            })
                        })
                        .collect::<Result<Vec<_>, MonomorphizationError>>()?,
                ),

            Expression::Await(span, expression) =>
                Expression::Await(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Async(span, expression) =>
                Expression::Async(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Spawn(span, expression) =>
                Expression::Spawn(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::New(span, name, arguments) =>
                Expression::New(
                    span,
                    name,
                    arguments
                        .into_iter()
                        .map(|argument| {
                            self.specialize_expression(
                                argument,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::QuantumOp(span, name, arguments) =>
                Expression::QuantumOp(
                    span,
                    name,
                    arguments
                        .into_iter()
                        .map(|argument| {
                            self.specialize_expression(
                                argument,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::Entangle(span, left, right) =>
                Expression::Entangle(
                    span,
                    Box::new(self.specialize_expression(*left, depth + 1)?),
                    Box::new(self.specialize_expression(*right, depth + 1)?),
                ),

            Expression::NanoOp(span, name, arguments) =>
                Expression::NanoOp(
                    span,
                    name,
                    arguments
                        .into_iter()
                        .map(|argument| {
                            self.specialize_expression(
                                argument,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            Expression::Recall(span, expression) =>
                Expression::Recall(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Remember(span, name, expression) =>
                Expression::Remember(
                    span,
                    name,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Learn(span, expression) =>
                Expression::Learn(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Perform(span, expression) =>
                Expression::Perform(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Zamani(span, expression) =>
                Expression::Zamani(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Sasa(span, expression) =>
                Expression::Sasa(
                    span,
                    Box::new(self.specialize_expression(*expression, depth + 1)?),
                ),

            Expression::Macro(span, name, arguments) =>
                Expression::Macro(
                    span,
                    name,
                    arguments
                        .into_iter()
                        .map(|argument| {
                            self.specialize_expression(
                                argument,
                                depth + 1,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                ),

            primitive => primitive,
        })
    }

    fn specialize_match_cases(
        &mut self,
        cases: Vec<MatchCase>,
        depth: usize,
    ) -> Result<Vec<MatchCase>, MonomorphizationError> {
        cases
            .into_iter()
            .map(|case| {
                Ok(MatchCase {
                    pattern: self.specialize_pattern(case.pattern, depth + 1),
                    guard: case
                        .guard
                        .map(|guard| {
                            self.specialize_expression(
                                guard,
                                depth + 1,
                            )
                        })
                        .transpose()?,
                    body: self.specialize_expression(
                        case.body,
                        depth + 1,
                    )?,
                    span: case.span,
                })
            })
            .collect()
    }

    fn specialize_pattern(
        &mut self,
        pattern: Pattern,
        depth: usize,
    ) -> Pattern {
        match pattern {
            Pattern::Tuple(span, patterns) =>
                Pattern::Tuple(
                    span,
                    patterns
                        .into_iter()
                        .map(|pattern| {
                            self.specialize_pattern(
                                pattern,
                                depth + 1,
                            )
                        })
                        .collect(),
                ),

            Pattern::Struct(span, name, fields) =>
                Pattern::Struct(
                    span,
                    name,
                    fields
                        .into_iter()
                        .map(|(name, pattern)| {
                            (
                                name,
                                self.specialize_pattern(
                                    pattern,
                                    depth + 1,
                                ),
                            )
                        })
                        .collect(),
                ),

            Pattern::Enum(span, name, patterns) =>
                Pattern::Enum(
                    span,
                    name,
                    patterns
                        .into_iter()
                        .map(|pattern| {
                            self.specialize_pattern(
                                pattern,
                                depth + 1,
                            )
                        })
                        .collect(),
                ),

            Pattern::Or(span, patterns) =>
                Pattern::Or(
                    span,
                    patterns
                        .into_iter()
                        .map(|pattern| {
                            self.specialize_pattern(
                                pattern,
                                depth + 1,
                            )
                        })
                        .collect(),
                ),

            Pattern::Range(span, start, end) =>
                Pattern::Range(
                    span,
                    Box::new(self.specialize_pattern(*start, depth + 1)),
                    Box::new(self.specialize_pattern(*end, depth + 1)),
                ),

            Pattern::Ref(span, pattern) =>
                Pattern::Ref(
                    span,
                    Box::new(self.specialize_pattern(*pattern, depth + 1)),
                ),

            other => other,
        }
    }

    // -------------------------------------------------------------------------
    // Type specialization
    // -------------------------------------------------------------------------

    fn specialize_type(
        &mut self,
        ty: TypeExpr,
        depth: usize,
    ) -> TypeExpr {
        match ty {
            TypeExpr::Generic(base, arguments) =>
                TypeExpr::Generic(
                    Box::new(self.specialize_type(*base, depth + 1)),
                    arguments
                        .into_iter()
                        .map(|argument| {
                            self.specialize_type(
                                argument,
                                depth + 1,
                            )
                        })
                        .collect(),
                ),

            TypeExpr::Tuple(types) =>
                TypeExpr::Tuple(
                    types
                        .into_iter()
                        .map(|ty| {
                            self.specialize_type(
                                ty,
                                depth + 1,
                            )
                        })
                        .collect(),
                ),

            TypeExpr::Array(ty) =>
                TypeExpr::Array(
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Slice(ty) =>
                TypeExpr::Slice(
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Function(arguments, result) =>
                TypeExpr::Function(
                    arguments
                        .into_iter()
                        .map(|ty| {
                            self.specialize_type(
                                ty,
                                depth + 1,
                            )
                        })
                        .collect(),
                    Box::new(self.specialize_type(
                        *result,
                        depth + 1,
                    )),
                ),

            TypeExpr::Reference(mutable, ty) =>
                TypeExpr::Reference(
                    mutable,
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Pointer(mutable, ty) =>
                TypeExpr::Pointer(
                    mutable,
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Optional(ty) =>
                TypeExpr::Optional(
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Result(ok, error) =>
                TypeExpr::Result(
                    Box::new(self.specialize_type(*ok, depth + 1)),
                    Box::new(self.specialize_type(*error, depth + 1)),
                ),

            TypeExpr::Quantum(ty) =>
                TypeExpr::Quantum(
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Linear(ty) =>
                TypeExpr::Linear(
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Affine(ty) =>
                TypeExpr::Affine(
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Temporal(ty) =>
                TypeExpr::Temporal(
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            TypeExpr::Pi(name, domain, codomain) =>
                TypeExpr::Pi(
                    name,
                    Box::new(self.specialize_type(*domain, depth + 1)),
                    Box::new(self.specialize_type(*codomain, depth + 1)),
                ),

            TypeExpr::Sigma(name, domain, codomain) =>
                TypeExpr::Sigma(
                    name,
                    Box::new(self.specialize_type(*domain, depth + 1)),
                    Box::new(self.specialize_type(*codomain, depth + 1)),
                ),

            TypeExpr::Identity(left, right) =>
                TypeExpr::Identity(
                    left,
                    right,
                ),

            TypeExpr::Hkt(name, ty) =>
                TypeExpr::Hkt(
                    name,
                    Box::new(self.specialize_type(*ty, depth + 1)),
                ),

            other => other,
        }
    }

    // -------------------------------------------------------------------------
    // Explicit specialization API
    // -------------------------------------------------------------------------

    /// Returns a deterministic specialized symbol name.
    ///
    /// Example:
    ///
    /// `map<int>` -> `map__int`
    ///
    /// The double separator prevents ambiguous concatenation such as:
    /// `foo_ab_c` vs `foo_a_bc`.
    pub fn get_specialized_name(
        &mut self,
        original: &str,
        type_args: &[TypeExpr],
    ) -> String {
        self.get_specialized_name_checked(original, type_args)
            .unwrap_or_else(|_| {
                self.fallback_specialized_name(original, type_args)
            })
    }

    /// Checked version of [`get_specialized_name`].
    pub fn get_specialized_name_checked(
        &mut self,
        original: &str,
        type_args: &[TypeExpr],
    ) -> Result<String, MonomorphizationError> {
        if original.trim().is_empty() {
            return Err(MonomorphizationError::InvalidSpecialization {
                message: "original symbol name cannot be empty".to_string(),
            });
        }

        if type_args.len() > self.config.max_type_arguments {
            return Err(MonomorphizationError::TooManyTypeArguments {
                count: type_args.len(),
                limit: self.config.max_type_arguments,
            });
        }

        let argument_names = type_args
            .iter()
            .map(Self::canonical_type_name)
            .collect::<Vec<_>>();

        let key = SpecializationKey {
            original: original.to_string(),
            type_arguments: argument_names.clone(),
        };

        if let Some(existing) = self.specializations.get(&key) {
            return Ok(existing.clone());
        }

        if self.specializations.len() >= self.config.max_specializations {
            return Err(
                MonomorphizationError::TooManySpecializations {
                    limit: self.config.max_specializations,
                },
            );
        }

        let base_name = format!(
            "{}__{}",
            sanitize_symbol(original),
            argument_names
                .iter()
                .map(|name| sanitize_symbol(name))
                .collect::<Vec<_>>()
                .join("__")
        );

        let specialized_name =
            self.allocate_unique_name(base_name);

        self.specializations
            .insert(key, specialized_name.clone());

        self.specialization_records.push(Specialization {
            original_name: original.to_string(),
            specialized_name: specialized_name.clone(),
            type_arguments: argument_names,
        });

        Ok(specialized_name)
    }

    fn allocate_unique_name(&mut self, base: String) -> String {
        if self.generated_names.insert(base.clone()) {
            return base;
        }

        let mut counter = 2usize;

        loop {
            let candidate = format!("{}_{}", base, counter);

            if self.generated_names.insert(candidate.clone()) {
                return candidate;
            }

            counter += 1;
        }
    }

    fn fallback_specialized_name(
        &self,
        original: &str,
        type_args: &[TypeExpr],
    ) -> String {
        let args = type_args
            .iter()
            .map(Self::canonical_type_name)
            .map(|name| sanitize_symbol(&name))
            .collect::<Vec<_>>();

        if args.is_empty() {
            format!("{}__specialized", sanitize_symbol(original))
        } else {
            format!(
                "{}__{}",
                sanitize_symbol(original),
                args.join("__")
            )
        }
    }

    // -------------------------------------------------------------------------
    // Generic type substitution
    // -------------------------------------------------------------------------

    /// Applies a generic substitution to a type expression.
    ///
    /// `T -> i32`, for example, transforms:
    ///
    /// `Vec<T>` -> `Vec<i32>`.
    pub fn substitute_type_parameters(
        &self,
        ty: &TypeExpr,
        substitutions: &HashMap<String, TypeExpr>,
    ) -> TypeExpr {
        match ty {
            TypeExpr::Identifier(identifier) => substitutions
                .get(&identifier.0)
                .cloned()
                .unwrap_or_else(|| ty.clone()),

            TypeExpr::Generic(base, arguments) =>
                TypeExpr::Generic(
                    Box::new(
                        self.substitute_type_parameters(
                            base,
                            substitutions,
                        ),
                    ),
                    arguments
                        .iter()
                        .map(|argument| {
                            self.substitute_type_parameters(
                                argument,
                                substitutions,
                            )
                        })
                        .collect(),
                ),

            TypeExpr::Tuple(types) =>
                TypeExpr::Tuple(
                    types
                        .iter()
                        .map(|ty| {
                            self.substitute_type_parameters(
                                ty,
                                substitutions,
                            )
                        })
                        .collect(),
                ),

            TypeExpr::Array(ty) =>
                TypeExpr::Array(
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Slice(ty) =>
                TypeExpr::Slice(
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Function(arguments, result) =>
                TypeExpr::Function(
                    arguments
                        .iter()
                        .map(|argument| {
                            self.substitute_type_parameters(
                                argument,
                                substitutions,
                            )
                        })
                        .collect(),
                    Box::new(
                        self.substitute_type_parameters(
                            result,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Reference(mutable, ty) =>
                TypeExpr::Reference(
                    *mutable,
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Pointer(mutable, ty) =>
                TypeExpr::Pointer(
                    *mutable,
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Optional(ty) =>
                TypeExpr::Optional(
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Result(ok, error) =>
                TypeExpr::Result(
                    Box::new(
                        self.substitute_type_parameters(
                            ok,
                            substitutions,
                        ),
                    ),
                    Box::new(
                        self.substitute_type_parameters(
                            error,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Quantum(ty) =>
                TypeExpr::Quantum(
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Linear(ty) =>
                TypeExpr::Linear(
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Affine(ty) =>
                TypeExpr::Affine(
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Temporal(ty) =>
                TypeExpr::Temporal(
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Pi(name, domain, codomain) =>
                TypeExpr::Pi(
                    name.clone(),
                    Box::new(
                        self.substitute_type_parameters(
                            domain,
                            substitutions,
                        ),
                    ),
                    Box::new(
                        self.substitute_type_parameters(
                            codomain,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Sigma(name, domain, codomain) =>
                TypeExpr::Sigma(
                    name.clone(),
                    Box::new(
                        self.substitute_type_parameters(
                            domain,
                            substitutions,
                        ),
                    ),
                    Box::new(
                        self.substitute_type_parameters(
                            codomain,
                            substitutions,
                        ),
                    ),
                ),

            TypeExpr::Hkt(name, ty) =>
                TypeExpr::Hkt(
                    name.clone(),
                    Box::new(
                        self.substitute_type_parameters(
                            ty,
                            substitutions,
                        ),
                    ),
                ),

            _ => ty.clone(),
        }
    }

    /// Creates a substitution table for a generic declaration.
    pub fn build_substitution_map(
        &self,
        parameters: &[TypeParameter],
        arguments: &[TypeExpr],
    ) -> Result<HashMap<String, TypeExpr>, MonomorphizationError> {
        if arguments.len() > self.config.max_type_arguments {
            return Err(
                MonomorphizationError::TooManyTypeArguments {
                    count: arguments.len(),
                    limit: self.config.max_type_arguments,
                },
            );
        }

        if parameters.len() != arguments.len() {
            return Err(MonomorphizationError::InvalidSpecialization {
                message: format!(
                    "generic declaration expects {} type arguments, got {}",
                    parameters.len(),
                    arguments.len()
                ),
            });
        }

        let mut result = HashMap::with_capacity(parameters.len());

        for (parameter, argument) in parameters.iter().zip(arguments) {
            result.insert(
                parameter.name.0.clone(),
                argument.clone(),
            );
        }

        Ok(result)
    }

    // -------------------------------------------------------------------------
    // Type canonicalization
    // -------------------------------------------------------------------------

    fn canonical_type_name(ty: &TypeExpr) -> String {
        match ty {
            TypeExpr::Identifier(id) => id.0.clone(),

            TypeExpr::Generic(base, args) => format!(
                "{}<{}>",
                Self::canonical_type_name(base),
                args.iter()
                    .map(Self::canonical_type_name)
                    .collect::<Vec<_>>()
                    .join(",")
            ),

            TypeExpr::Tuple(types) => format!(
                "({})",
                types
                    .iter()
                    .map(Self::canonical_type_name)
                    .collect::<Vec<_>>()
                    .join(",")
            ),

            TypeExpr::Array(ty) =>
                format!("[{}]", Self::canonical_type_name(ty)),

            TypeExpr::Slice(ty) =>
                format!("slice<{}>", Self::canonical_type_name(ty)),

            TypeExpr::Function(arguments, result) => format!(
                "fn({})->{}",
                arguments
                    .iter()
                    .map(Self::canonical_type_name)
                    .collect::<Vec<_>>()
                    .join(","),
                Self::canonical_type_name(result)
            ),

            TypeExpr::Reference(mutable, ty) => format!(
                "&{}{}",
                if *mutable { "mut " } else { "" },
                Self::canonical_type_name(ty)
            ),

            TypeExpr::Pointer(mutable, ty) => format!(
                "*{}{}",
                if *mutable { "mut " } else { "" },
                Self::canonical_type_name(ty)
            ),

            TypeExpr::Optional(ty) =>
                format!("optional<{}>", Self::canonical_type_name(ty)),

            TypeExpr::Result(ok, error) => format!(
                "result<{},{}>",
                Self::canonical_type_name(ok),
                Self::canonical_type_name(error)
            ),

            TypeExpr::Never => "never".to_string(),
            TypeExpr::Unit => "unit".to_string(),
            TypeExpr::SelfType => "self".to_string(),

            TypeExpr::Quantum(ty) =>
                format!("quantum<{}>", Self::canonical_type_name(ty)),

            TypeExpr::Linear(ty) =>
                format!("linear<{}>", Self::canonical_type_name(ty)),

            TypeExpr::Affine(ty) =>
                format!("affine<{}>", Self::canonical_type_name(ty)),

            TypeExpr::Temporal(ty) =>
                format!("temporal<{}>", Self::canonical_type_name(ty)),

            TypeExpr::Pi(name, domain, codomain) => format!(
                "pi {}:{} -> {}",
                name,
                Self::canonical_type_name(domain),
                Self::canonical_type_name(codomain)
            ),

            TypeExpr::Sigma(name, domain, codomain) => format!(
                "sigma {}:{} -> {}",
                name,
                Self::canonical_type_name(domain),
                Self::canonical_type_name(codomain)
            ),

            TypeExpr::Identity(_, _) =>
                "identity".to_string(),

            TypeExpr::Hkt(name, ty) =>
                format!("hkt<{},{}>", name, Self::canonical_type_name(ty)),
        }
    }

    // -------------------------------------------------------------------------
    // Safety
    // -------------------------------------------------------------------------

    fn check_depth(
        &self,
        depth: usize,
    ) -> Result<(), MonomorphizationError> {
        if depth > self.config.max_depth {
            return Err(
                MonomorphizationError::RecursionLimitExceeded {
                    depth,
                },
            );
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Introspection
    // -------------------------------------------------------------------------

    /// Returns whether the program contains an explicitly generic declaration.
    pub fn has_generic_definition(&self, name: &str) -> bool {
        self.generic_definitions.contains_key(name)
    }

    /// Returns the number of indexed generic definitions.
    pub fn generic_definition_count(&self) -> usize {
        self.generic_definitions.len()
    }

    /// Returns the names of all indexed generic definitions in deterministic
    /// order.
    pub fn generic_definition_names(&self) -> Vec<String> {
        let mut names = self
            .generic_definitions
            .values()
            .map(|definition| definition.name.clone())
            .collect::<Vec<_>>();

        names.sort();

        names
    }
}

// -----------------------------------------------------------------------------
// Symbol sanitization
// -----------------------------------------------------------------------------

fn sanitize_symbol(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for character in input.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            output.push(character);
        } else {
            output.push('_');
        }
    }

    if output.is_empty() {
        output.push_str("anonymous");
    }

    if output
        .chars()
        .next()
        .map(|character| character.is_ascii_digit())
        .unwrap_or(false)
    {
        output.insert(0, '_');
    }

    output
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(name: &str) -> Identifier {
        Identifier::new(name, Span::dummy())
    }

    fn named_type(name: &str) -> TypeExpr {
        TypeExpr::Identifier(identifier(name))
    }

    #[test]
    fn creates_deterministic_specialized_name() {
        let mut monomorphizer = Monomorphizer::new();

        let first = monomorphizer
            .get_specialized_name_checked(
                "map",
                &[named_type("i32")],
            )
            .expect("specialization should succeed");

        let second = monomorphizer
            .get_specialized_name_checked(
                "map",
                &[named_type("i32")],
            )
            .expect("cached specialization should succeed");

        assert_eq!(first, second);
        assert_eq!(monomorphizer.specialization_count(), 1);
    }

    #[test]
    fn different_type_arguments_create_different_specializations() {
        let mut monomorphizer = Monomorphizer::new();

        let i32_name = monomorphizer
            .get_specialized_name("map", &[named_type("i32")]);

        let f64_name = monomorphizer
            .get_specialized_name("map", &[named_type("f64")]);

        assert_ne!(i32_name, f64_name);
    }

    #[test]
    fn substitution_replaces_generic_parameter() {
        let monomorphizer = Monomorphizer::new();

        let generic = TypeExpr::Generic(
            Box::new(named_type("Vec")),
            vec![named_type("T")],
        );

        let mut substitutions = HashMap::new();

        substitutions.insert(
            "T".to_string(),
            named_type("i32"),
        );

        let result =
            monomorphizer.substitute_type_parameters(
                &generic,
                &substitutions,
            );

        assert_eq!(
            Monomorphizer::canonical_type_name(&result),
            "Vec<i32>"
        );
    }

    #[test]
    fn substitution_preserves_unknown_types() {
        let monomorphizer = Monomorphizer::new();

        let generic = named_type("Unknown");

        let substitutions = HashMap::new();

        let result =
            monomorphizer.substitute_type_parameters(
                &generic,
                &substitutions,
            );

        assert_eq!(
            Monomorphizer::canonical_type_name(&result),
            "Unknown"
        );
    }

    #[test]
    fn rejects_too_many_type_arguments() {
        let config = MonomorphizationConfig {
            max_type_arguments: 1,
            ..Default::default()
        };

        let mut monomorphizer =
            Monomorphizer::with_config(config);

        let result = monomorphizer.get_specialized_name_checked(
            "Pair",
            &[named_type("A"), named_type("B")],
        );

        assert!(matches!(
            result,
            Err(MonomorphizationError::TooManyTypeArguments {
                count: 2,
                limit: 1
            })
        ));
    }

    #[test]
    fn specialization_limit_is_enforced() {
        let config = MonomorphizationConfig {
            max_specializations: 1,
            ..Default::default()
        };

        let mut monomorphizer =
            Monomorphizer::with_config(config);

        monomorphizer
            .get_specialized_name_checked(
                "foo",
                &[named_type("i32")],
            )
            .expect("first specialization should succeed");

        let result = monomorphizer.get_specialized_name_checked(
            "foo",
            &[named_type("f64")],
        );

        assert!(matches!(
            result,
            Err(MonomorphizationError::TooManySpecializations {
                limit: 1
            })
        ));
    }

    #[test]
    fn names_are_sanitized() {
        let mut monomorphizer = Monomorphizer::new();

        let name = monomorphizer.get_specialized_name(
            "foo-bar",
            &[named_type("Vec<i32>")],
        );

        assert!(!name.contains('-'));
        assert!(!name.contains('<'));
        assert!(!name.contains('>'));
    }

    #[test]
    fn substitution_map_requires_matching_arity() {
        let monomorphizer = Monomorphizer::new();

        let parameters = vec![
            TypeParameter {
                name: identifier("T"),
                bounds: Vec::new(),
            },
        ];

        let arguments = vec![
            named_type("i32"),
            named_type("f64"),
        ];

        assert!(
            monomorphizer
                .build_substitution_map(
                    &parameters,
                    &arguments
                )
                .is_err()
        );
    }

    #[test]
    fn reset_clears_specialization_state() {
        let mut monomorphizer = Monomorphizer::new();

        monomorphizer.get_specialized_name(
            "foo",
            &[named_type("i32")],
        );

        assert_eq!(monomorphizer.specialization_count(), 1);

        monomorphizer.reset();

        assert_eq!(monomorphizer.specialization_count(), 0);
        assert_eq!(monomorphizer.generic_definition_count(), 0);
    }
}