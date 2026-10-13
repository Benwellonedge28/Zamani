
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani frontend semantic analysis — expression type inference.
//!
//! Path: src/frontend/semantic/types/inference.rs
//! Rust: 2021 edition, Rust 1.97+
//!
//! PURPOSE
//!
//! Infer expression types from the canonical frontend AST, optional expected
//! types, symbol resolution, and the shared type-unification service.
//!
//! ARCHITECTURAL BOUNDARY
//!
//! The canonical AST is graph-shaped and identifies nodes with NodeId.
//! This module deliberately operates through an adapter instead of defining
//! a competing AST or assuming that expressions are recursively embedded.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - expression inference orchestration;
//! - inference memoization within one inference instance;
//! - detection of recursive expression dependencies;
//! - propagation of expected types;
//! - conversion of adapter failures into inference failures.
//!
//! This module does not own:
//! - AST construction or traversal outside expression inference;
//! - the canonical semantic type representation;
//! - symbol tables or name resolution;
//! - type-variable allocation or unification algorithms;
//! - overload and trait resolution;
//! - effect, capability, contract, or resource validation;
//! - target selection, hardware feasibility, or IR lowering.
//!
//! INTEGRATION
//!
//! - `frontend::ast::node::expressions::expression` supplies source AST
//!   constructs through an adapter implementing `InferenceHost`.
//! - `types::ty` supplies the canonical semantic type representation.
//! - `types::unification` supplies type-variable creation and unification.
//! - `symbols::resolution` supplies name and declaration resolution.
//! - `model::facts` records completed inference results.
//!
//! The adapter is responsible for translating canonical AST nodes and
//! TypeExpr syntax into these generic inference views. It must preserve node
//! identities, source provenance, and domain extension identities.
//!
//! SCALABILITY
//!
//! No hardware capacities, domain counts, operand counts, or generic-arity
//! ceilings are embedded here. A caller may configure an operational
//! expression-depth budget. Such a budget is a compiler safety policy,
//! not a restriction on programs expressible by Zamani.
//!
//! DETERMINISM
//!
//! Results are memoized by expression identity. This module does not rely on
//! hash-map iteration order. Callers must provide deterministic AST views,
//! name resolution, extension dispatch, and unification.
//!
//! ERROR POLICY
//!
//! Unresolved names, invalid expressions, unification failures, cyclic
//! dependencies, and budget exhaustion must remain explicit errors. This
//! module never treats an unknown type as automatically compatible.
//!
//! STATUS
//!
//! This file defines the inference engine and adapter contract. Integration
//! is complete only when the repository's semantic type, unification,
//! symbol-resolution, diagnostic, and AST adapter modules implement the
//! contracts below and the complete workspace passes its test suite.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// Stable identity of a source AST expression.
///
/// The concrete implementation should use the canonical AST's `NodeId`.
pub trait ExpressionIdentity: Copy + Ord + fmt::Debug {}

impl<T> ExpressionIdentity for T where T: Copy + Ord + fmt::Debug {}

/// Source-level operator identity.
///
/// Namespace and name remain open-ended: this type does not enumerate
/// classical operators, quantum operations, or domain-specific primitives.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperatorIdentity {
    pub namespace: String,
    pub name: String,
}

impl OperatorIdentity {
    #[must_use]
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    pub fn validate(&self) -> Result<(), InferenceErrorKind> {
        if self.name.is_empty() {
            return Err(InferenceErrorKind::InvalidOperator);
        }
        Ok(())
    }
}

/// Literal category, not a concrete semantic type.
///
/// The host applies Zamani's language rules to the literal spelling. For
/// example, integer suffixes, exact numeric ranges, and dialect rules must
/// be interpreted by the authoritative type/literal service.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralCategory {
    Integer,
    Decimal,
    Boolean,
    Character,
    String,
    Unit,
    Other,
}

/// A semantic view of an expression.
///
/// This is an adapter DTO, not another AST. Its IDs must refer to nodes in
/// the canonical AST. Children are referenced by identity, never by recursive
/// Rust ownership.
///
/// `Extension` allows new language domains to participate without requiring
/// edits to this enum for every new operation or computational domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionView<Id> {
    Literal {
        category: LiteralCategory,
        spelling: String,
    },
    Name {
        path: Vec<String>,
    },
    Unary {
        operator: OperatorIdentity,
        operand: Id,
    },
    Binary {
        operator: OperatorIdentity,
        left: Id,
        right: Id,
    },
    Call {
        callee: Id,
        arguments: Vec<Id>,
    },
    Tuple {
        elements: Vec<Id>,
    },
    Array {
        elements: Vec<Id>,
    },
    Index {
        base: Id,
        index: Id,
    },
    Member {
        base: Id,
        member: String,
    },
    Conditional {
        condition: Id,
        then_branch: Id,
        else_branch: Id,
    },
    Ascription {
        expression: Id,
        type_node: Id,
    },
    Extension {
        namespace: String,
        name: String,
        operands: Vec<Id>,
    },
}

/// Inference policy.
///
/// `max_depth = None` imposes no explicit depth budget. In production,
/// untrusted inputs should normally use a configured budget or an iterative
/// work-list implementation. This is an operational safeguard only.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InferenceConfig {
    pub max_depth: Option<usize>,
}

/// Errors owned by the inference engine itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferenceErrorKind {
    InvalidOperator,
    InvalidExtensionIdentity,
    EmptyNamePath,
    EmptyMemberName,
    EmptyExtensionName,
    RecursiveExpressionDependency,
    DepthBudgetExceeded {
        maximum: usize,
    },
    ConflictingExpectedTypes,
    MissingExpectedType,
    AdapterContractViolation,
}

/// An inference error with the expression identity that triggered it.
///
/// `source` is optional because structural errors can be detected by this
/// module before the host creates a semantic diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceError<Id, E> {
    pub expression: Option<Id>,
    pub kind: InferenceErrorKind,
    pub source: Option<E>,
}

impl<Id, E> InferenceError<Id, E> {
    fn new(kind: InferenceErrorKind, expression: Option<Id>) -> Self {
        Self {
            expression,
            kind,
            source: None,
        }
    }

    fn from_host(expression: Option<Id>, source: E) -> Self {
        Self {
            expression,
            kind: InferenceErrorKind::AdapterContractViolation,
            source: Some(source),
        }
    }
}

impl<Id: fmt::Debug, E: fmt::Debug> fmt::Display for InferenceError<Id, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type inference failed at {:?}: {:?}",
            self.expression, self.kind
        )?;
        if let Some(source) = &self.source {
            write!(f, " ({source:?})")?;
        }
        Ok(())
    }
}

impl<Id: fmt::Debug, E: fmt::Debug> std::error::Error for InferenceError<Id, E> {}

/// Domain-neutral bridge to the rest of the semantic subsystem.
///
/// Implement this trait in the integration layer. The associated `Type`
/// should be the canonical type from `types::ty`, not an AST TypeExpr.
///
/// The host owns all language-specific rules and must return errors for
/// unresolved or invalid constructs rather than fabricating a successful
/// type.
pub trait InferenceHost {
    type ExprId: ExpressionIdentity;
    type Type: Clone + PartialEq + fmt::Debug;
    type Error;

    /// Fetch an expression view from the canonical AST graph.
    fn expression(
        &self,
        id: Self::ExprId,
    ) -> Result<ExpressionView<Self::ExprId>, Self::Error>;

    /// Resolve a source-level name/path and return its semantic type.
    fn resolve_name(
        &mut self,
        expression: Self::ExprId,
        path: &[String],
    ) -> Result<Self::Type, Self::Error>;

    /// Determine the semantic type of a source literal.
    fn literal_type(
        &mut self,
        expression: Self::ExprId,
        category: LiteralCategory,
        spelling: &str,
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Resolve a type expression represented by a canonical AST node.
    fn resolve_type_expression(
        &mut self,
        type_node: Self::ExprId,
    ) -> Result<Self::Type, Self::Error>;

    /// Infer the result type of an operator.
    ///
    /// The host handles overload resolution, operand constraints, domain
    /// metadata, generic substitutions, and the canonical unifier.
    fn operator_type(
        &mut self,
        expression: Self::ExprId,
        operator: &OperatorIdentity,
        operands: &[Self::Type],
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Infer a function or callable invocation.
    ///
    /// The host owns callability, argument arity, generic instantiation,
    /// parameter compatibility, overload resolution, and return type rules.
    fn call_type(
        &mut self,
        expression: Self::ExprId,
        callee: &Self::Type,
        arguments: &[Self::Type],
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Resolve indexing semantics for a base and index type.
    fn index_type(
        &mut self,
        expression: Self::ExprId,
        base: &Self::Type,
        index: &Self::Type,
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Resolve a member access using the canonical declaration/type services.
    fn member_type(
        &mut self,
        expression: Self::ExprId,
        base: &Self::Type,
        member: &str,
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Construct the semantic type of a tuple from its element types.
    fn tuple_type(
        &mut self,
        expression: Self::ExprId,
        elements: &[Self::Type],
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Construct an array type while checking all element types.
    ///
    /// Array length and symbolic dimensions must be preserved by the host.
    fn array_type(
        &mut self,
        expression: Self::ExprId,
        elements: &[Self::Type],
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Check that a condition is semantically valid and return the common
    /// result type of both branches.
    fn conditional_type(
        &mut self,
        expression: Self::ExprId,
        condition: &Self::Type,
        then_type: &Self::Type,
        else_type: &Self::Type,
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Infer a registered extension expression.
    ///
    /// Unknown extensions must be rejected explicitly. A host may delegate
    /// to namespaced extension registries, which must be deterministic.
    fn extension_type(
        &mut self,
        expression: Self::ExprId,
        namespace: &str,
        name: &str,
        operands: &[Self::Type],
        expected: Option<&Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Unify an inferred type with an expected type.
    ///
    /// Implement using `types::unification`. This must perform occurs checks,
    /// reject incompatible types, and preserve unresolved constraints.
    fn unify(
        &mut self,
        expression: Self::ExprId,
        inferred: Self::Type,
        expected: &Self::Type,
    ) -> Result<Self::Type, Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VisitState {
    Visiting,
    Complete,
}

/// One inference instance. Do not share it between independent analyses.
///
/// The host may contain a per-session symbol resolver and unifier. All
/// mutable inference state is local to this instance, preventing hidden
/// global state and cross-compilation leakage.
pub struct TypeInference<'host, H: InferenceHost> {
    host: &'host mut H,
    config: InferenceConfig,
    cache: BTreeMap<H::ExprId, H::Type>,
    states: BTreeMap<H::ExprId, VisitState>,
}

impl<'host, H: InferenceHost> TypeInference<'host, H> {
    #[must_use]
    pub fn new(host: &'host mut H, config: InferenceConfig) -> Self {
        Self {
            host,
            config,
            cache: BTreeMap::new(),
            states: BTreeMap::new(),
        }
    }

    /// Infer an expression without an expected type.
    pub fn infer(
        &mut self,
        expression: H::ExprId,
    ) -> Result<H::Type, InferenceError<H::ExprId, H::Error>> {
        self.infer_inner(expression, None, 0)
    }

    /// Infer an expression under an expected semantic type.
    ///
    /// Expected types are constraints, not permission to silently coerce an
    /// incompatible expression. The host applies the language's explicit
    /// compatibility and unification rules.
    pub fn infer_expected(
        &mut self,
        expression: H::ExprId,
        expected: &H::Type,
    ) -> Result<H::Type, InferenceError<H::ExprId, H::Error>> {
        self.infer_inner(expression, Some(expected), 0)
    }

    /// Return a completed cached result, if this expression was inferred.
    #[must_use]
    pub fn cached_type(&self, expression: H::ExprId) -> Option<&H::Type> {
        self.cache.get(&expression)
    }

    /// Consume this instance and return completed inference facts.
    ///
    /// The returned map is ordered by expression identity for deterministic
    /// downstream serialization and test assertions.
    #[must_use]
    pub fn into_facts(self) -> BTreeMap<H::ExprId, H::Type> {
        self.cache
    }

    fn host_error(
        expression: H::ExprId,
        source: H::Error,
    ) -> InferenceError<H::ExprId, H::Error> {
        InferenceError::from_host(Some(expression), source)
    }

    fn infer_child(
        &mut self,
        expression: H::ExprId,
        expected: Option<&H::Type>,
        depth: usize,
    ) -> Result<H::Type, InferenceError<H::ExprId, H::Error>> {
        self.infer_inner(expression, expected, depth)
    }

    fn infer_inner(
        &mut self,
        expression: H::ExprId,
        expected: Option<&H::Type>,
        depth: usize,
    ) -> Result<H::Type, InferenceError<H::ExprId, H::Error>> {
        if let Some(maximum) = self.config.max_depth {
            if depth > maximum {
                return Err(InferenceError::new(
                    InferenceErrorKind::DepthBudgetExceeded { maximum },
                    Some(expression),
                ));
            }
        }

        if let Some(cached) = self.cache.get(&expression).cloned() {
            return match expected {
                Some(expected) => self
                    .host
                    .unify(expression, cached, expected)
                    .map_err(|error| Self::host_error(expression, error)),
                None => Ok(cached),
            };
        }

        if self.states.get(&expression) == Some(&VisitState::Visiting) {
            return Err(InferenceError::new(
                InferenceErrorKind::RecursiveExpressionDependency,
                Some(expression),
            ));
        }

        self.states.insert(expression, VisitState::Visiting);

        let view = match self.host.expression(expression) {
            Ok(view) => view,
            Err(error) => {
                self.states.remove(&expression);
                return Err(Self::host_error(expression, error));
            }
        };

        let inferred = self.infer_view(expression, view, expected, depth);

        match inferred {
            Ok(ty) => {
                self.states.insert(expression, VisitState::Complete);
                self.cache.insert(expression, ty.clone());
                Ok(ty)
            }
            Err(error) => {
                self.states.remove(&expression);
                Err(error)
            }
        }
    }

    fn infer_view(
        &mut self,
        expression: H::ExprId,
        view: ExpressionView<H::ExprId>,
        expected: Option<&H::Type>,
        depth: usize,
    ) -> Result<H::Type, InferenceError<H::ExprId, H::Error>> {
        let child_depth = depth.saturating_add(1);

        let inferred = match view {
            ExpressionView::Literal { category, spelling } => self
                .host
                .literal_type(expression, category, &spelling, expected)
                .map_err(|error| Self::host_error(expression, error))?,

            ExpressionView::Name { path } => {
                if path.is_empty() || path.iter().any(String::is_empty) {
                    return Err(InferenceError::new(
                        InferenceErrorKind::EmptyNamePath,
                        Some(expression),
                    ));
                }
                self.host
                    .resolve_name(expression, &path)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Unary { operator, operand } => {
                operator
                    .validate()
                    .map_err(|kind| InferenceError::new(kind, Some(expression)))?;
                let operand_type = self.infer_child(operand, None, child_depth)?;
                self.host
                    .operator_type(expression, &operator, &[operand_type], expected)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Binary {
                operator,
                left,
                right,
            } => {
                operator
                    .validate()
                    .map_err(|kind| InferenceError::new(kind, Some(expression)))?;
                let left_type = self.infer_child(left, None, child_depth)?;
                let right_type = self.infer_child(right, None, child_depth)?;
                self.host
                    .operator_type(
                        expression,
                        &operator,
                        &[left_type, right_type],
                        expected,
                    )
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Call { callee, arguments } => {
                let callee_type = self.infer_child(callee, None, child_depth)?;
                let mut argument_types = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    argument_types.push(self.infer_child(argument, None, child_depth)?);
                }
                self.host
                    .call_type(expression, &callee_type, &argument_types, expected)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Tuple { elements } => {
                let mut types = Vec::with_capacity(elements.len());
                for element in elements {
                    types.push(self.infer_child(element, None, child_depth)?);
                }
                self.host
                    .tuple_type(expression, &types, expected)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Array { elements } => {
                let mut types = Vec::with_capacity(elements.len());
                for element in elements {
                    types.push(self.infer_child(element, None, child_depth)?);
                }
                self.host
                    .array_type(expression, &types, expected)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Index { base, index } => {
                let base_type = self.infer_child(base, None, child_depth)?;
                let index_type = self.infer_child(index, None, child_depth)?;
                self.host
                    .index_type(expression, &base_type, &index_type, expected)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Member { base, member } => {
                if member.is_empty() {
                    return Err(InferenceError::new(
                        InferenceErrorKind::EmptyMemberName,
                        Some(expression),
                    ));
                }
                let base_type = self.infer_child(base, None, child_depth)?;
                self.host
                    .member_type(expression, &base_type, &member, expected)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_type = self.infer_child(condition, None, child_depth)?;
                let then_type = self.infer_child(then_branch, expected, child_depth)?;
                let else_type = self.infer_child(else_branch, expected, child_depth)?;
                self.host
                    .conditional_type(
                        expression,
                        &condition_type,
                        &then_type,
                        &else_type,
                        expected,
                    )
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Ascription {
                expression: inner,
                type_node,
            } => {
                let declared_type = self
                    .host
                    .resolve_type_expression(type_node)
                    .map_err(|error| Self::host_error(expression, error))?;
                let inner_type =
                    self.infer_child(inner, Some(&declared_type), child_depth)?;
                self.host
                    .unify(expression, inner_type, &declared_type)
                    .map_err(|error| Self::host_error(expression, error))?
            }

            ExpressionView::Extension {
                namespace,
                name,
                operands,
            } => {
                if namespace.is_empty() || name.is_empty() {
                    return Err(InferenceError::new(
                        InferenceErrorKind::InvalidExtensionIdentity,
                        Some(expression),
                    ));
                }

                let mut operand_types = Vec::with_capacity(operands.len());
                for operand in operands {
                    operand_types.push(self.infer_child(operand, None, child_depth)?);
                }
                self.host
                    .extension_type(
                        expression,
                        &namespace,
                        &name,
                        &operand_types,
                        expected,
                    )
                    .map_err(|error| Self::host_error(expression, error))?
            }
        };

        match expected {
            Some(expected) => self
                .host
                .unify(expression, inferred, expected)
                .map_err(|error| Self::host_error(expression, error)),
            None => Ok(inferred),
        }
    }
}

/// Validate that an ordered list of dependency identities contains no
/// duplicates. Useful for AST adapters before exposing child dependencies.
pub fn validate_unique_dependencies<Id: Ord + Copy>(
    dependencies: &[Id],
) -> Result<(), Id> {
    let mut seen = BTreeSet::new();
    for dependency in dependencies {
        if !seen.insert(*dependency) {
            return Err(*dependency);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_identity_rejects_empty_name() {
        let operator = OperatorIdentity::new("zamani.core", "");
        assert_eq!(
            operator.validate(),
            Err(InferenceErrorKind::InvalidOperator)
        );
    }

    #[test]
    fn operator_identity_allows_extension_namespaces() {
        let operator = OperatorIdentity::new("vendor.domain", "operation_v2");
        assert_eq!(operator.validate(), Ok(()));
    }

    #[test]
    fn dependency_validation_detects_duplicates() {
        assert_eq!(validate_unique_dependencies(&[1_u32, 2, 1]), Err(1));
        assert_eq!(validate_unique_dependencies(&[1_u32, 2, 3]), Ok(()));
    }

    #[test]
    fn default_policy_does_not_claim_a_language_depth_limit() {
        assert_eq!(InferenceConfig::default().max_depth, None);
    }
}
