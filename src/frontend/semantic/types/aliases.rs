
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani semantic analysis — type aliases.
//!
//! PURPOSE
//!
//! Register, validate and expand source-level type aliases using the
//! canonical frontend AST. This module does not define a second AST or
//! semantic type system.
//!
//! AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/types/types.g4`
//! - `grammar/types/generic.g4`
//! - `grammar/types/bounds.g4`
//! - `src/frontend/ast/node/types/type_expr.rs`
//!
//! The grammar defines source syntax. The canonical AST defines its
//! structural representation. This module implements alias expansion.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Alias declarations registered with this resolver.
//! - Alias-name indexing.
//! - Generic alias arity validation.
//! - Capture-avoiding substitution of alias parameters.
//! - Direct and indirect alias-cycle detection.
//! - Bounded, deterministic alias expansion.
//!
//! This module does not own:
//! - AST parsing or declaration collection.
//! - Ordinary symbol-table ownership or name resolution.
//! - Type inference, unification or compatibility.
//! - Trait implementation selection.
//! - Associated-type projection normalization.
//! - Constant evaluation or resource allocation.
//! - Hardware discovery, quantum routing, scheduling or lowering.
//!
//! INTEGRATION
//!
//! Input/output type:
//! `crate::frontend::ast::node::types::TypeExpr`.
//!
//! The semantic declaration collector should register validated aliases
//! after declaration identities and module paths are known. Type inference
//! and type checking can consume the expanded result.
//!
//! Qualified names are indexed by their complete source path. A production
//! integration must supply canonical, module-resolved paths; this module
//! must not guess which import or namespace a spelling refers to.
//!
//! SCALABILITY
//!
//! There are no hardware-capacity limits or fixed generic-arity limits.
//! An optional expansion-work budget protects compiler operation against
//! hostile or excessively expensive inputs. Exceeding that budget is an
//! explicit error, never a successful partial expansion.
//!
//! FREEZE CRITERIA
//!
//! - Duplicate aliases and duplicate parameters are rejected.
//! - Generic arity is checked before substitution.
//! - Direct and indirect cycles produce actionable errors.
//! - Symbolic values and unrelated type constructors are preserved.
//! - Expansion is deterministic for identical registered declarations.
//! - No unsafe Rust is used.
//!
//! Rust baseline: edition 2021, Rust 1.97 or later.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::types::type_expr::{
    LifetimeName, TypeAttribute, TypeExpr, TypeExtension, TypeName, TypeParameterName,
    TypePath, TypeValueExpr, TypeValueExtension,
};

/// A validated source-level type-alias declaration.
///
/// `path` must be the canonical, module-qualified path chosen by declaration
/// collection. `parameters` retain declaration order. `body` is the canonical
/// source-level type expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeAliasDefinition {
    path: TypePath,
    parameters: Vec<TypeParameterName>,
    body: TypeExpr,
}

impl TypeAliasDefinition {
    /// Create an alias definition.
    ///
    /// Structural validation is performed by [`TypeAliasResolver::register`],
    /// where the configured expansion policy is available.
    #[must_use]
    pub fn new(
        path: TypePath,
        parameters: Vec<TypeParameterName>,
        body: TypeExpr,
    ) -> Self {
        Self {
            path,
            parameters,
            body,
        }
    }

    /// Return the canonical alias path.
    #[must_use]
    pub fn path(&self) -> &TypePath {
        &self.path
    }

    /// Return the declared generic parameters in source order.
    #[must_use]
    pub fn parameters(&self) -> &[TypeParameterName] {
        &self.parameters
    }

    /// Return the alias body.
    #[must_use]
    pub fn body(&self) -> &TypeExpr {
        &self.body
    }
}

/// Operational policy for alias expansion.
///
/// These are compiler safety/work policies, not source-language or hardware
/// limits. `None` disables the corresponding policy check; use a configured
/// budget when processing untrusted source.
///
/// The depth budget also prevents unbounded recursive traversal from being
/// mistaken for valid completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AliasExpansionPolicy {
    /// Maximum recursive expansion/traversal depth.
    pub max_depth: Option<usize>,

    /// Maximum number of expression visits during one expansion.
    pub max_work: Option<usize>,

    /// Optional AST-level structural validation policy.
    ///
    /// When present, it is passed to the canonical AST's validator.
    pub validate_ast: bool,
}

impl Default for AliasExpansionPolicy {
    fn default() -> Self {
        Self {
            // Operational default, not a language-level type-depth ceiling.
            // Callers can explicitly select another budget.
            max_depth: Some(256),
            max_work: Some(1_000_000),
            validate_ast: true,
        }
    }
}

/// Errors produced by registration or expansion.
///
/// These errors are structured so the semantic diagnostics layer can map
/// them to stable diagnostic codes without parsing display strings.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AliasError {
    /// An alias path contains no segments or an empty segment.
    InvalidPath {
        path: String,
    },

    /// An alias name is already registered.
    DuplicateAlias {
        path: String,
    },

    /// A generic parameter name is empty.
    EmptyParameter {
        alias: String,
    },

    /// A generic parameter occurs more than once in one alias declaration.
    DuplicateParameter {
        alias: String,
        parameter: String,
    },

    /// The canonical AST rejected the alias body.
    InvalidTypeExpression {
        alias: String,
        reason: String,
    },

    /// A referenced alias was not registered in this resolver.
    UnknownAlias {
        path: String,
    },

    /// The number of arguments does not match the declaration.
    ArityMismatch {
        alias: String,
        expected: usize,
        actual: usize,
    },

    /// Expansion discovered a direct or indirect alias cycle.
    Cycle {
        /// Ordered cycle path, including the repeated terminal alias.
        path: Vec<String>,
    },

    /// An operational depth budget was exceeded.
    DepthLimitExceeded {
        limit: usize,
    },

    /// An operational work budget was exceeded.
    WorkLimitExceeded {
        limit: usize,
    },

    /// The alias registry or expansion state is internally inconsistent.
    InternalInvariant {
        message: String,
    },
}

impl fmt::Display for AliasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath { path } => {
                write!(f, "invalid type-alias path `{path}`")
            }
            Self::DuplicateAlias { path } => {
                write!(f, "duplicate type alias `{path}`")
            }
            Self::EmptyParameter { alias } => {
                write!(f, "type alias `{alias}` has an empty generic parameter")
            }
            Self::DuplicateParameter { alias, parameter } => {
                write!(
                    f,
                    "type alias `{alias}` declares generic parameter `{parameter}` more than once"
                )
            }
            Self::InvalidTypeExpression { alias, reason } => {
                write!(f, "invalid type expression in alias `{alias}`: {reason}")
            }
            Self::UnknownAlias { path } => {
                write!(f, "unknown type alias `{path}`")
            }
            Self::ArityMismatch {
                alias,
                expected,
                actual,
            } => write!(
                f,
                "type alias `{alias}` expects {expected} generic argument(s), but received {actual}"
            ),
            Self::Cycle { path } => {
                write!(f, "cyclic type alias: {}", path.join(" -> "))
            }
            Self::DepthLimitExceeded { limit } => {
                write!(f, "type-alias expansion exceeded the configured depth budget of {limit}")
            }
            Self::WorkLimitExceeded { limit } => {
                write!(f, "type-alias expansion exceeded the configured work budget of {limit}")
            }
            Self::InternalInvariant { message } => {
                write!(f, "type-alias resolver invariant failed: {message}")
            }
        }
    }
}

impl Error for AliasError {}

/// Deterministic registry and expander for source-level type aliases.
///
/// The resolver is scoped to one semantic environment. A compiler should
/// create one per analysis session or per immutable module environment rather
/// than sharing mutable global state.
///
/// Alias keys use complete source paths joined with `::`. The declaration
/// collector must ensure this representation is unambiguous for the language's
/// identifier rules and provide already-resolved module paths.
#[derive(Clone, Debug)]
pub struct TypeAliasResolver {
    aliases: BTreeMap<String, TypeAliasDefinition>,
    policy: AliasExpansionPolicy,
}

impl Default for TypeAliasResolver {
    fn default() -> Self {
        Self::new(AliasExpansionPolicy::default())
    }
}

impl TypeAliasResolver {
    /// Create an empty resolver with an explicit operational policy.
    #[must_use]
    pub fn new(policy: AliasExpansionPolicy) -> Self {
        Self {
            aliases: BTreeMap::new(),
            policy,
        }
    }

    /// Return the expansion policy.
    #[must_use]
    pub const fn policy(&self) -> AliasExpansionPolicy {
        self.policy
    }

    /// Return the number of registered aliases.
    #[must_use]
    pub fn len(&self) -> usize {
        self.aliases.len()
    }

    /// Return whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.aliases.is_empty()
    }

    /// Register a declaration.
    ///
    /// Registration is atomic: invalid declarations do not modify the
    /// registry. Duplicate names are errors rather than implicit replacement.
    pub fn register(
        &mut self,
        definition: TypeAliasDefinition,
    ) -> Result<(), AliasError> {
        let path = definition.path.to_source_string();

        validate_path(&definition.path)?;

        let mut parameters = BTreeSet::new();

        for parameter in &definition.parameters {
            let name = parameter.as_str();

            if name.trim().is_empty() {
                return Err(AliasError::EmptyParameter {
                    alias: path.clone(),
                });
            }

            if !parameters.insert(name.to_owned()) {
                return Err(AliasError::DuplicateParameter {
                    alias: path,
                    parameter: name.to_owned(),
                });
            }
        }

        if self.policy.validate_ast {
            definition.body.validate().map_err(|error| {
                AliasError::InvalidTypeExpression {
                    alias: definition.path.to_source_string(),
                    reason: error.to_string(),
                }
            })?;
        }

        if self.aliases.contains_key(&path) {
            return Err(AliasError::DuplicateAlias { path });
        }

        self.aliases.insert(path, definition);
        Ok(())
    }

    /// Look up a registered alias by its complete canonical path.
    #[must_use]
    pub fn get(&self, path: &TypePath) -> Option<&TypeAliasDefinition> {
        self.aliases.get(&path.to_source_string())
    }

    /// Expand aliases occurring anywhere in a type expression.
    ///
    /// Non-alias type constructors are preserved, including associated-type
    /// projections, type extensions, quantum abstractions, HDL abstractions,
    /// symbolic dimensions, and unresolved generic parameters.
    ///
    /// An identifier matching a registered alias is expanded. An identifier
    /// that does not match a registered alias is preserved as an ordinary
    /// type name. This allows primitive names, generic parameters, imported
    /// names and externally resolved names to be handled by their proper
    /// semantic owners.
    ///
    /// A generic application whose base is a registered alias is checked
    /// against that alias's declared parameter count before substitution.
    pub fn expand(&self, expression: &TypeExpr) -> Result<TypeExpr, AliasError> {
        let mut state = ExpansionState {
            work: 0,
            stack: Vec::new(),
        };

        self.expand_inner(expression, &BTreeMap::new(), &mut state, 0)
    }

    /// Expand an alias explicitly with its ordered generic arguments.
    ///
    /// This is useful for declaration checking and for tests that need to
    /// verify arity and cycle behavior without constructing a larger type.
    pub fn expand_alias(
        &self,
        path: &TypePath,
        arguments: &[TypeExpr],
    ) -> Result<TypeExpr, AliasError> {
        let key = canonical_key(path)?;
        let definition = self
            .aliases
            .get(&key)
            .ok_or_else(|| AliasError::UnknownAlias { path: key.clone() })?;

        let mut state = ExpansionState {
            work: 0,
            stack: Vec::new(),
        };

        self.expand_definition(definition, arguments, &mut state, 0)
    }

    fn expand_inner(
        &self,
        expression: &TypeExpr,
        substitutions: &BTreeMap<String, TypeExpr>,
        state: &mut ExpansionState,
        depth: usize,
    ) -> Result<TypeExpr, AliasError> {
        self.visit(state, depth)?;

        match expression {
            TypeExpr::Identifier(path) => {
                let key = canonical_key(path)?;

                if let Some(replacement) = substitutions.get(&key) {
                    return self.expand_inner(replacement, substitutions, state, depth + 1);
                }

                match self.aliases.get(&key) {
                    Some(definition) => {
                        self.expand_definition(definition, &[], state, depth + 1)
                    }
                    None => Ok(expression.clone()),
                }
            }

            TypeExpr::Generic { base, arguments } => {
                if let TypeExpr::Identifier(path) = base.as_ref() {
                    let key = canonical_key(path)?;

                    if let Some(definition) = self.aliases.get(&key) {
                        return self.expand_definition(
                            definition,
                            arguments,
                            state,
                            depth + 1,
                        );
                    }
                }

                let expanded_base =
                    self.expand_inner(base, substitutions, state, depth + 1)?;

                let expanded_arguments = arguments
                    .iter()
                    .map(|argument| {
                        self.expand_inner(argument, substitutions, state, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(TypeExpr::Generic {
                    base: Box::new(expanded_base),
                    arguments: expanded_arguments,
                })
            }

            TypeExpr::GenericParameter(parameter) => {
                match substitutions.get(parameter.as_str()) {
                    Some(replacement) => {
                        self.expand_inner(replacement, substitutions, state, depth + 1)
                    }
                    None => Ok(expression.clone()),
                }
            }

            TypeExpr::Tuple(items) => Ok(TypeExpr::Tuple(
                items
                    .iter()
                    .map(|item| {
                        self.expand_inner(item, substitutions, state, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),

            TypeExpr::Array { element, length } => Ok(TypeExpr::Array {
                element: Box::new(self.expand_inner(
                    element,
                    substitutions,
                    state,
                    depth + 1,
                )?),
                // Cardinalities remain symbolic. Type-level value evaluation
                // is owned by the constant/dependent-type subsystem.
                length: length.clone(),
            }),

            TypeExpr::Slice(inner) => Ok(TypeExpr::Slice(Box::new(
                self.expand_inner(inner, substitutions, state, depth + 1)?,
            ))),

            TypeExpr::Function {
                parameters,
                return_type,
            } => Ok(TypeExpr::Function {
                parameters: parameters
                    .iter()
                    .map(|parameter| {
                        self.expand_inner(parameter, substitutions, state, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                return_type: Box::new(self.expand_inner(
                    return_type,
                    substitutions,
                    state,
                    depth + 1,
                )?),
            }),

            TypeExpr::Reference {
                mutable,
                lifetime,
                inner,
            } => Ok(TypeExpr::Reference {
                mutable: *mutable,
                lifetime: lifetime.clone(),
                inner: Box::new(self.expand_inner(
                    inner,
                    substitutions,
                    state,
                    depth + 1,
                )?),
            }),

            TypeExpr::Pointer { mutable, inner } => Ok(TypeExpr::Pointer {
                mutable: *mutable,
                inner: Box::new(self.expand_inner(
                    inner,
                    substitutions,
                    state,
                    depth + 1,
                )?),
            }),

            TypeExpr::Optional(inner) => Ok(TypeExpr::Optional(Box::new(
                self.expand_inner(inner, substitutions, state, depth + 1)?,
            ))),

            TypeExpr::Result { ok, error } => Ok(TypeExpr::Result {
                ok: Box::new(self.expand_inner(ok, substitutions, state, depth + 1)?),
                error: Box::new(self.expand_inner(
                    error,
                    substitutions,
                    state,
                    depth + 1,
                )?),
            }),

            TypeExpr::Union(items) => Ok(TypeExpr::Union(
                items
                    .iter()
                    .map(|item| {
                        self.expand_inner(item, substitutions, state, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),

            TypeExpr::Intersection(items) => Ok(TypeExpr::Intersection(
                items
                    .iter()
                    .map(|item| {
                        self.expand_inner(item, substitutions, state, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),

            TypeExpr::Associated { base, member } => Ok(TypeExpr::Associated {
                base: Box::new(self.expand_inner(
                    base,
                    substitutions,
                    state,
                    depth + 1,
                )?),
                member: member.clone(),
            }),

            TypeExpr::TypeApplication {
                constructor,
                arguments,
            } => Ok(TypeExpr::TypeApplication {
                constructor: Box::new(self.expand_inner(
                    constructor,
                    substitutions,
                    state,
                    depth + 1,
                )?),
                arguments: arguments
                    .iter()
                    .map(|argument| {
                        self.expand_inner(argument, substitutions, state, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),

            TypeExpr::Quantum(inner) => Ok(TypeExpr::Quantum(Box::new(
                self.expand_inner(inner, substitutions, state, depth + 1)?,
            ))),

            TypeExpr::Linear(inner) => Ok(TypeExpr::Linear(Box::new(
                self.expand_inner(inner, substitutions, state, depth + 1)?,
            ))),

            TypeExpr::Affine(inner) => Ok(TypeExpr::Affine(Box::new(
                self.expand_inner(inner, substitutions, state, depth + 1)?,
            ))),

            TypeExpr::Temporal(inner) => Ok(TypeExpr::Temporal(Box::new(
                self.expand_inner(inner, substitutions, state, depth + 1)?,
            ))),

            TypeExpr::Pi {
                parameter,
                parameter_type,
                body,
            } => {
                let mut nested = substitutions.clone();
                nested.remove(parameter.as_str());

                Ok(TypeExpr::Pi {
                    parameter: parameter.clone(),
                    parameter_type: Box::new(self.expand_inner(
                        parameter_type,
                        substitutions,
                        state,
                        depth + 1,
                    )?),
                    body: Box::new(self.expand_inner(
                        body,
                        &nested,
                        state,
                        depth + 1,
                    )?),
                })
            }

            TypeExpr::Sigma {
                parameter,
                parameter_type,
                body,
            } => {
                let mut nested = substitutions.clone();
                nested.remove(parameter.as_str());

                Ok(TypeExpr::Sigma {
                    parameter: parameter.clone(),
                    parameter_type: Box::new(self.expand_inner(
                        parameter_type,
                        substitutions,
                        state,
                        depth + 1,
                    )?),
                    body: Box::new(self.expand_inner(
                        body,
                        &nested,
                        state,
                        depth + 1,
                    )?),
                })
            }

            TypeExpr::Identity { left, right } => Ok(TypeExpr::Identity {
                left: Box::new(self.expand_inner(
                    left,
                    substitutions,
                    state,
                    depth + 1,
                )?),
                right: Box::new(self.expand_inner(
                    right,
                    substitutions,
                    state,
                    depth + 1,
                )?),
            }),

            TypeExpr::Hkt {
                constructor,
                argument,
            } => Ok(TypeExpr::Hkt {
                constructor: constructor.clone(),
                argument: Box::new(self.expand_inner(
                    argument,
                    substitutions,
                    state,
                    depth + 1,
                )?),
            }),

            TypeExpr::Extension(extension) => {
                let arguments = extension
                    .arguments()
                    .iter()
                    .map(|argument| {
                        self.expand_inner(argument, substitutions, state, depth + 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let attributes = extension
                    .attributes()
                    .iter()
                    .map(|attribute| {
                        TypeAttribute::new(
                            attribute.namespace().clone(),
                            attribute.name().clone(),
                            attribute.value().map(str::to_owned),
                        )
                    })
                    .collect();

                Ok(TypeExpr::Extension(TypeExtension::new(
                    extension.namespace().clone(),
                    extension.name().clone(),
                    arguments,
                    attributes,
                )))
            }

            // These forms have no child type expressions requiring alias
            // substitution. Their semantics remain with later passes.
            TypeExpr::Never
            | TypeExpr::Unit
            | TypeExpr::SelfType
            | TypeExpr::Infer => Ok(expression.clone()),
        }
    }

    fn expand_definition(
        &self,
        definition: &TypeAliasDefinition,
        arguments: &[TypeExpr],
        state: &mut ExpansionState,
        depth: usize,
    ) -> Result<TypeExpr, AliasError> {
        self.visit(state, depth)?;

        let key = definition.path.to_source_string();

        if definition.parameters.len() != arguments.len() {
            return Err(AliasError::ArityMismatch {
                alias: key,
                expected: definition.parameters.len(),
                actual: arguments.len(),
            });
        }

        if let Some(position) = state.stack.iter().position(|entry| entry == &key) {
            let mut cycle = state.stack[position..].to_vec();
            cycle.push(key);
            return Err(AliasError::Cycle { path: cycle });
        }

        // Expand arguments in the caller's environment before introducing
        // the callee's parameter bindings. This prevents argument names from
        // accidentally being interpreted in the alias body's environment.
        let mut expanded_arguments = Vec::with_capacity(arguments.len());

        for argument in arguments {
            expanded_arguments.push(self.expand_inner(
                argument,
                &BTreeMap::new(),
                state,
                depth + 1,
            )?);
        }

        let mut substitutions = BTreeMap::new();

        for (parameter, argument) in definition
            .parameters
            .iter()
            .zip(expanded_arguments.into_iter())
        {
            substitutions.insert(parameter.as_str().to_owned(), argument);
        }

        state.stack.push(key);

        let result = self.expand_inner(
            &definition.body,
            &substitutions,
            state,
            depth + 1,
        );

        state.stack.pop();
        result
    }

    fn visit(
        &self,
        state: &mut ExpansionState,
        depth: usize,
    ) -> Result<(), AliasError> {
        if let Some(limit) = self.policy.max_depth {
            if depth > limit {
                return Err(AliasError::DepthLimitExceeded { limit });
            }
        }

        state.work = state.work.checked_add(1).ok_or_else(|| {
            AliasError::InternalInvariant {
                message: "expansion work counter overflowed".to_owned(),
            }
        })?;

        if let Some(limit) = self.policy.max_work {
            if state.work > limit {
                return Err(AliasError::WorkLimitExceeded { limit });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct ExpansionState {
    work: usize,
    stack: Vec<String>,
}

fn validate_path(path: &TypePath) -> Result<(), AliasError> {
    canonical_key(path).map(|_| ())
}

fn canonical_key(path: &TypePath) -> Result<String, AliasError> {
    if path.is_empty() || path.segments().iter().any(|part| part.as_str().trim().is_empty()) {
        return Err(AliasError::InvalidPath {
            path: path.to_source_string(),
        });
    }

    Ok(path.to_source_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(name: &str) -> TypePath {
        TypePath::from_names(name.split("::"))
    }

    fn named(name: &str) -> TypeExpr {
        TypeExpr::named(path(name))
    }

    fn resolver() -> TypeAliasResolver {
        TypeAliasResolver::new(AliasExpansionPolicy {
            max_depth: Some(128),
            max_work: Some(10_000),
            validate_ast: true,
        })
    }

    #[test]
    fn expands_a_simple_alias() {
        let mut aliases = resolver();

        aliases
            .register(TypeAliasDefinition::new(
                path("example::Count"),
                vec![],
                named("Int"),
            ))
            .expect("registration succeeds");

        assert_eq!(
            aliases
                .expand(&named("example::Count"))
                .expect("expansion succeeds"),
            named("Int")
        );
    }

    #[test]
    fn substitutes_generic_alias_arguments() {
        let mut aliases = resolver();

        aliases
            .register(TypeAliasDefinition::new(
                path("example::Pair"),
                vec![
                    TypeParameterName::from("A"),
                    TypeParameterName::from("B"),
                ],
                TypeExpr::Tuple(vec![
                    TypeExpr::GenericParameter(TypeParameterName::from("A")),
                    TypeExpr::GenericParameter(TypeParameterName::from("B")),
                ]),
            ))
            .expect("registration succeeds");

        let input = TypeExpr::Generic {
            base: Box::new(named("example::Pair")),
            arguments: vec![named("Int"), named("String")],
        };

        assert_eq!(
            aliases.expand(&input).expect("expansion succeeds"),
            TypeExpr::Tuple(vec![named("Int"), named("String")])
        );
    }

    #[test]
    fn rejects_wrong_generic_arity() {
        let mut aliases = resolver();

        aliases
            .register(TypeAliasDefinition::new(
                path("example::Boxed"),
                vec![TypeParameterName::from("T")],
                TypeExpr::Generic {
                    base: Box::new(named("Container")),
                    arguments: vec![TypeExpr::GenericParameter(
                        TypeParameterName::from("T"),
                    )],
                },
            ))
            .expect("registration succeeds");

        let result = aliases.expand(&TypeExpr::Generic {
            base: Box::new(named("example::Boxed")),
            arguments: vec![],
        });

        assert!(matches!(result, Err(AliasError::ArityMismatch { .. })));
    }

    #[test]
    fn detects_a_direct_cycle() {
        let mut aliases = resolver();

        aliases
            .register(TypeAliasDefinition::new(
                path("example::A"),
                vec![],
                named("example::A"),
            ))
            .expect("registration succeeds");

        assert!(matches!(
            aliases.expand(&named("example::A")),
            Err(AliasError::Cycle { .. })
        ));
    }

    #[test]
    fn detects_an_indirect_cycle() {
        let mut aliases = resolver();

        aliases
            .register(TypeAliasDefinition::new(
                path("example::A"),
                vec![],
                named("example::B"),
            ))
            .expect("registration succeeds");

        aliases
            .register(TypeAliasDefinition::new(
                path("example::B"),
                vec![],
                named("example::A"),
            ))
            .expect("registration succeeds");

        assert!(matches!(
            aliases.expand(&named("example::A")),
            Err(AliasError::Cycle { .. })
        ));
    }

    #[test]
    fn preserves_symbolic_array_cardinality() {
        let mut aliases = resolver();

        aliases
            .register(TypeAliasDefinition::new(
                path("example::Register"),
                vec![TypeParameterName::from("T")],
                TypeExpr::Array {
                    element: Box::new(TypeExpr::GenericParameter(
                        TypeParameterName::from("T"),
                    )),
                    length: Some(TypeValueExpr::Generic(TypeParameterName::from("N"))),
                },
            ))
            .expect("registration succeeds");

        let input = TypeExpr::Generic {
            base: Box::new(named("example::Register")),
            arguments: vec![named("Qubit")],
        };

        let result = aliases.expand(&input).expect("expansion succeeds");

        assert!(matches!(
            result,
            TypeExpr::Array {
                element,
                length: Some(_)
            } if *element == named("Qubit")
        ));
    }

    #[test]
    fn does_not_rewrite_unregistered_names() {
        let aliases = resolver();
        let input = named("external::FutureType");

        assert_eq!(aliases.expand(&input).expect("expansion succeeds"), input);
    }

    #[test]
    fn rejects_duplicate_alias_registration() {
        let mut aliases = resolver();
        let definition = TypeAliasDefinition::new(
            path("example::Value"),
            vec![],
            named("Int"),
        );

        aliases
            .register(definition.clone())
            .expect("first registration succeeds");

        assert!(matches!(
            aliases.register(definition),
            Err(AliasError::DuplicateAlias { .. })
        ));
    }

    #[test]
    fn expansion_work_budget_is_enforced() {
        let aliases = TypeAliasResolver::new(AliasExpansionPolicy {
            max_depth: None,
            max_work: Some(0),
            validate_ast: true,
        });

        assert!(matches!(
            aliases.expand(&named("Int")),
            Err(AliasError::WorkLimitExceeded { .. })
        ));
    }
}
