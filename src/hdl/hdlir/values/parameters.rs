
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR parameter declarations, bindings, and resolution
//!
//! Path: `src/hdl/hdlir/values/parameters.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent hardware-domain parameters independently of a particular
//! physical implementation. Provide validated parameter declarations,
//! defaults, explicit bindings, parameter expressions, constraints, and
//! deterministic resolution into an immutable specialization plan.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hardware-generics.g4`.
//! - `grammar/hdl/hardware-modules.g4`.
//! - `grammar/Zamani.g4`.
//! - `src/hdl/api.rs`.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, particularly section 8.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/values/constants.rs`.
//! - `src/hdl/hdlir/values/expressions.rs`.
//!
//! Grammar defines syntax. The canonical frontend semantic model defines
//! source meaning. HDLIR owns the representation and local invariants here.
//!
//! ## Ownership
//!
//! This module owns parameter identity within a parameter set, declaration
//! metadata, binding storage, default selection, structural binding checks,
//! constraint retention, and explicit unresolved outcomes.
//!
//! It does not own parsing, name resolution, general type inference,
//! expression evaluation, constraint proof, physical resource allocation,
//! target selection, or mutation of a hardware design during specialization.
//!
//! ## Scalability
//!
//! No universal maximum parameter count, value width, module count, or
//! specialization count is imposed. Collections grow according to available
//! resources. `BitWidth` retains the established symbolic-width contract.
//!
//! Concrete evaluation, if required, belongs to the expression and width
//! evaluators and must use their checked arithmetic and operational budgets.
//!
//! ## Integration
//!
//! - `values/mod.rs` must register this module and export its intended API.
//! - `values/constants.rs` owns concrete HDLIR constants.
//! - `values/expressions.rs` owns expression graphs and expression nodes.
//! - `types/bit_width.rs` owns symbolic width expressions and width evaluation.
//! - `ir/module.rs` should own the parameterized module and its identity.
//! - `builder/module.rs` should use declarations and binding validation when
//!   constructing parameterized modules.
//! - `transform/specialize.rs` should consume a resolved plan, evaluate
//!   permitted expressions, and produce a new specialized design.
//! - `verify/types.rs` and the canonical semantic layer remain responsible
//!   for semantic type compatibility and constraint validity.
//! - `source_map.rs` remains the authority for source provenance.
//!
//! This module intentionally does not depend on future IR builders,
//! verification implementations, or unverified frontend adapter APIs.
//!
//! ## Important semantic boundary
//!
//! A resolved parameter plan is not proof that all parameter expressions
//! have been evaluated or that every constraint has been satisfied.
//! Constraints remain pending until an authoritative evaluator verifies them.
//! Target feasibility is a separate downstream decision.
//!
//! ## Freeze criteria
//!
//! This module is complete only when its parent module registers it, all
//! sibling APIs compile together, integration tests exercise the actual
//! frontend and module builder, and the repository's required checks pass.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::types::bit_width::BitWidth;
use crate::hdl::hdlir::values::constants::ConstantValue;
use crate::hdl::hdlir::values::expressions::{
    ExprError,
    ExprGraph,
    ExprId,
    ExprKind,
};

/// A validated parameter name.
///
/// The original spelling is retained. This type does not perform source
/// name resolution, case folding, or namespace qualification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParameterName(String);

impl ParameterName {
    /// Construct a parameter name from a non-empty string.
    ///
    /// # Errors
    ///
    /// Returns [`ParameterError::EmptyName`] if the name contains only
    /// whitespace.
    pub fn new(name: impl Into<String>) -> Result<Self, ParameterError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ParameterError::EmptyName);
        }

        Ok(Self(name))
    }

    /// Return the retained name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ParameterName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// The semantic category declared for a parameter.
///
/// These categories describe parameter intent, not physical hardware
/// resources. More detailed source-level typing remains the responsibility
/// of the canonical semantic model.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParameterKind {
    /// A compile-time or elaboration-time value.
    Value,

    /// A parameter whose value must be a constant or compile-time expression.
    Constant,

    /// A type-level parameter represented by a qualified type reference.
    Type,

    /// A parameter used to describe structural elaboration intent.
    Structural,
}

/// A symbolic expression owned by an immutable expression graph.
///
/// The graph and root are retained together so an expression ID cannot
/// accidentally be interpreted against an unrelated graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterExpression {
    graph: ExprGraph,
    root: ExprId,
}

impl ParameterExpression {
    /// Construct a parameter expression from an existing graph and root.
    ///
    /// # Errors
    ///
    /// Returns an error if the graph is invalid or the root does not exist
    /// in that graph.
    pub fn new(graph: ExprGraph, root: ExprId) -> Result<Self, ParameterError> {
        graph.validate().map_err(ParameterError::Expression)?;

        if graph.get(root).is_none() {
            return Err(ParameterError::InvalidExpressionRoot {
                root: root.index(),
            });
        }

        Ok(Self { graph, root })
    }

    /// Return the expression graph.
    #[must_use]
    pub const fn graph(&self) -> &ExprGraph {
        &self.graph
    }

    /// Return the graph-local root identity.
    #[must_use]
    pub const fn root(&self) -> ExprId {
        self.root
    }

    /// Return names of symbolic references reachable from this expression.
    ///
    /// Names are reported in deterministic sorted order. This method
    /// discovers syntactic dependencies only; it does not resolve names,
    /// prove that they are parameters, or evaluate the expression.
    ///
    /// Traversal is iterative and does not consume call-stack space
    /// proportional to expression depth.
    #[must_use]
    pub fn referenced_symbols(&self) -> BTreeSet<String> {
        let mut symbols = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut pending = vec![self.root];

        while let Some(id) = pending.pop() {
            if !visited.insert(id) {
                continue;
            }

            let Some(node) = self.graph.get(id) else {
                // Construction validates the graph. This defensive branch
                // keeps this query total if an invariant is ever violated.
                continue;
            };

            match node.kind() {
                ExprKind::Constant(_) => {}

                ExprKind::Symbol(name) => {
                    symbols.insert(name.clone());
                }

                ExprKind::Unary { operand, .. } => {
                    pending.push(*operand);
                }

                ExprKind::Binary { lhs, rhs, .. } => {
                    pending.push(*lhs);
                    pending.push(*rhs);
                }

                ExprKind::Conditional {
                    condition,
                    then_value,
                    else_value,
                } => {
                    pending.push(*condition);
                    pending.push(*then_value);
                    pending.push(*else_value);
                }

                ExprKind::Concat { operands } => {
                    pending.extend(operands.iter().copied());
                }

                ExprKind::Extract { value, offset, .. } => {
                    pending.push(*value);
                    pending.push(*offset);
                }

                ExprKind::Cast { value, .. } => {
                    pending.push(*value);
                }

                ExprKind::Extension { operands, .. } => {
                    pending.extend(operands.iter().copied());
                }
            }
        }

        symbols
    }
}

/// A value that can be supplied to a parameter.
///
/// Symbolic widths remain distinct from ordinary constants because
/// `BitWidth` has its own established evaluation semantics. Expressions
/// retain their graph and root. Unresolved values remain explicit rather
/// than being replaced by guessed defaults.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParameterValue {
    /// A concrete HDLIR constant.
    Constant(ConstantValue),

    /// A symbolic or concrete hardware bit-width expression.
    Width(BitWidth),

    /// A type-level reference.
    ///
    /// The canonical semantic layer must resolve and validate the name.
    TypeReference(String),

    /// A symbolic expression whose meaning is defined by its expression
    /// graph and the applicable operation semantics.
    Expression(ParameterExpression),

    /// An intentionally unresolved value.
    ///
    /// `symbol` identifies the unresolved reference when known. `reason`
    /// records why the producer could not supply a more precise value.
    Unresolved {
        /// Unresolved symbol or value name, when known.
        symbol: Option<String>,

        /// Explanation of the unresolved state.
        reason: String,
    },
}

impl ParameterValue {
    /// Construct an explicitly unresolved value.
    ///
    /// Neither the symbol nor the reason is interpreted as executable code.
    pub fn unresolved(
        symbol: Option<String>,
        reason: impl Into<String>,
    ) -> Result<Self, ParameterError> {
        let reason = reason.into();

        if reason.trim().is_empty() {
            return Err(ParameterError::EmptyUnresolvedReason);
        }

        if symbol
            .as_deref()
            .is_some_and(|candidate| candidate.trim().is_empty())
        {
            return Err(ParameterError::EmptyUnresolvedSymbol);
        }

        Ok(Self::Unresolved { symbol, reason })
    }

    /// Return whether this value is explicitly unresolved.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        matches!(self, Self::Unresolved { .. })
    }

    /// Return whether this value is a concrete constant.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Check the local relationship between a parameter category and value.
    ///
    /// This is a structural check, not source-language type checking.
    fn validate_for(&self, kind: ParameterKind) -> Result<(), ParameterError> {
        let valid = match kind {
            ParameterKind::Type => matches!(self, Self::TypeReference(_)),

            ParameterKind::Constant => {
                !matches!(self, Self::TypeReference(_))
            }

            ParameterKind::Value | ParameterKind::Structural => {
                !matches!(self, Self::TypeReference(_))
            }
        };

        if !valid {
            return Err(ParameterError::ValueKindMismatch { kind });
        }

        if let Self::TypeReference(name) = self {
            if name.trim().is_empty() {
                return Err(ParameterError::EmptyTypeReference);
            }
        }

        if let Self::Expression(expression) = self {
            expression
                .graph
                .validate()
                .map_err(ParameterError::Expression)?;

            if expression.graph.get(expression.root).is_none() {
                return Err(ParameterError::InvalidExpressionRoot {
                    root: expression.root.index(),
                });
            }
        }

        if let Self::Width(width) = self {
            validate_width_structure(width)?;
        }

        Ok(())
    }
}

/// Validate symbolic-width structure without prematurely evaluating it.
///
/// Concrete arithmetic evaluation remains owned by `BitWidth`.
fn validate_width_structure(width: &BitWidth) -> Result<(), ParameterError> {
    // BitWidth constructors validate individual symbols and concrete widths.
    // The enum is public, so callers can also build nested expressions.
    // Walk iteratively to avoid recursion on deeply nested widths.
    let mut pending = vec![width];

    while let Some(current) = pending.pop() {
        match current {
            BitWidth::Known(_) => {}

            BitWidth::Symbol(symbol) | BitWidth::Unresolved(symbol) => {
                if symbol.as_str().trim().is_empty() {
                    return Err(ParameterError::InvalidWidthSymbol);
                }
            }

            BitWidth::Add(lhs, rhs) | BitWidth::Multiply(lhs, rhs) => {
                pending.push(lhs);
                pending.push(rhs);
            }
        }
    }

    Ok(())
}

/// A boolean predicate retained as a parameter constraint.
///
/// The expression is not evaluated here. Constraint interpretation and
/// proof belong to the semantic evaluator or an explicitly registered
/// HDLIR evaluation service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterConstraint {
    predicate: ParameterExpression,
    description: Option<String>,
}

impl ParameterConstraint {
    /// Construct a constraint from a predicate expression.
    ///
    /// The predicate is expected to evaluate to an acceptable truth value
    /// under the applicable HDL semantics. This constructor cannot prove it.
    pub fn new(
        predicate: ParameterExpression,
        description: Option<String>,
    ) -> Result<Self, ParameterError> {
        if description
            .as_deref()
            .is_some_and(|text| text.trim().is_empty())
        {
            return Err(ParameterError::EmptyConstraintDescription);
        }

        Ok(Self {
            predicate,
            description,
        })
    }

    /// Return the retained predicate.
    #[must_use]
    pub const fn predicate(&self) -> &ParameterExpression {
        &self.predicate
    }

    /// Return the optional human-readable description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// A validated parameter declaration.
///
/// A declaration describes a parameter without binding it to a particular
/// hardware target. Defaults and constraints are preserved as declared.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterDeclaration {
    name: ParameterName,
    kind: ParameterKind,
    type_name: Option<String>,
    default: Option<ParameterValue>,
    constraints: Vec<ParameterConstraint>,
}

impl ParameterDeclaration {
    /// Create a declaration without a default value or constraints.
    pub fn new(
        name: impl Into<String>,
        kind: ParameterKind,
    ) -> Result<Self, ParameterError> {
        Ok(Self {
            name: ParameterName::new(name)?,
            kind,
            type_name: None,
            default: None,
            constraints: Vec::new(),
        })
    }

    /// Attach a source-semantic type name.
    ///
    /// The name is retained without attempting resolution. An empty name
    /// is rejected; actual type compatibility is checked by semantic
    /// analysis or HDLIR verification.
    pub fn with_type_name(
        mut self,
        type_name: impl Into<String>,
    ) -> Result<Self, ParameterError> {
        let type_name = type_name.into();

        if type_name.trim().is_empty() {
            return Err(ParameterError::EmptyTypeReference);
        }

        self.type_name = Some(type_name);
        Ok(self)
    }

    /// Attach a default value.
    ///
    /// This checks only the local parameter-category relationship. It does
    /// not evaluate the value or prove its source-language type.
    pub fn with_default(
        mut self,
        value: ParameterValue,
    ) -> Result<Self, ParameterError> {
        value.validate_for(self.kind)?;
        self.default = Some(value);
        Ok(self)
    }

    /// Attach a constraint predicate.
    pub fn with_constraint(
        mut self,
        constraint: ParameterConstraint,
    ) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Return the parameter name.
    #[must_use]
    pub const fn name(&self) -> &ParameterName {
        &self.name
    }

    /// Return the declared parameter category.
    #[must_use]
    pub const fn kind(&self) -> ParameterKind {
        self.kind
    }

    /// Return the optional source-semantic type name.
    #[must_use]
    pub fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }

    /// Return the optional default value.
    #[must_use]
    pub fn default_value(&self) -> Option<&ParameterValue> {
        self.default.as_ref()
    }

    /// Return constraints in declaration order.
    #[must_use]
    pub fn constraints(&self) -> &[ParameterConstraint] {
        &self.constraints
    }
}

/// Origin of a parameter value selected for a specialization plan.
///
/// This is lightweight resolution provenance, not a replacement for the
/// canonical source map or source-span representation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParameterValueOrigin {
    /// A value explicitly supplied by the caller.
    ExplicitBinding,

    /// A value selected from the declaration's default.
    DeclarationDefault,
}

/// A value selected for one parameter.
///
/// Selection does not imply that symbolic expressions are evaluated or
/// that the parameter's constraints have been proven.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedParameter {
    name: ParameterName,
    value: ParameterValue,
    origin: ParameterValueOrigin,
}

impl ResolvedParameter {
    /// Return the parameter name.
    #[must_use]
    pub const fn name(&self) -> &ParameterName {
        &self.name
    }

    /// Return the selected value.
    #[must_use]
    pub const fn value(&self) -> &ParameterValue {
        &self.value
    }

    /// Return how the value was selected.
    #[must_use]
    pub const fn origin(&self) -> ParameterValueOrigin {
        self.origin
    }
}

/// An immutable selection plan for a parameterized design.
///
/// Every declared parameter has exactly one entry in `parameters`.
/// A parameter without an explicit binding or default is represented as
/// unresolved. Constraints are reported as pending until an authoritative
/// evaluator checks them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterResolution {
    parameters: BTreeMap<ParameterName, ResolvedParameter>,
    unresolved: BTreeSet<ParameterName>,
    pending_constraints: BTreeMap<ParameterName, usize>,
}

impl ParameterResolution {
    /// Return the selected parameters in deterministic name order.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<ParameterName, ResolvedParameter> {
        &self.parameters
    }

    /// Return the names of parameters that have no selected concrete or
    /// symbolic value beyond an unresolved marker.
    #[must_use]
    pub fn unresolved(&self) -> &BTreeSet<ParameterName> {
        &self.unresolved
    }

    /// Return the number of unevaluated constraints for each parameter.
    ///
    /// The values are counts, not constraint results.
    #[must_use]
    pub fn pending_constraints(&self) -> &BTreeMap<ParameterName, usize> {
        &self.pending_constraints
    }

    /// Return whether every parameter has a selected, non-unresolved value.
    ///
    /// This does not mean expressions have been evaluated or constraints
    /// satisfied.
    #[must_use]
    pub fn has_no_unresolved_parameters(&self) -> bool {
        self.unresolved.is_empty()
    }

    /// Return whether the plan has constraints that still need evaluation.
    #[must_use]
    pub fn has_pending_constraints(&self) -> bool {
        self.pending_constraints.values().any(|count| *count > 0)
    }
}

/// A declared parameter set with explicit bindings.
///
/// Declaration and binding maps are ordered for deterministic traversal,
/// diagnostics, and plan generation. Parameter names are unique within this
/// set; they are not globally unique across all designs.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ParameterSet {
    declarations: BTreeMap<ParameterName, ParameterDeclaration>,
    bindings: BTreeMap<ParameterName, ParameterValue>,
}

impl ParameterSet {
    /// Create an empty parameter set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return declarations in deterministic name order.
    #[must_use]
    pub fn declarations(
        &self,
    ) -> &BTreeMap<ParameterName, ParameterDeclaration> {
        &self.declarations
    }

    /// Return explicitly supplied bindings in deterministic name order.
    #[must_use]
    pub fn bindings(&self) -> &BTreeMap<ParameterName, ParameterValue> {
        &self.bindings
    }

    /// Return the number of declared parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.declarations.len()
    }

    /// Return whether the set has no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty()
    }

    /// Declare a parameter.
    ///
    /// # Errors
    ///
    /// Returns [`ParameterError::DuplicateDeclaration`] if the name is
    /// already declared.
    pub fn declare(
        &mut self,
        declaration: ParameterDeclaration,
    ) -> Result<(), ParameterError> {
        let name = declaration.name.clone();

        if self.declarations.contains_key(&name) {
            return Err(ParameterError::DuplicateDeclaration(name));
        }

        if let Some(default) = declaration.default_value() {
            default.validate_for(declaration.kind())?;
        }

        self.declarations.insert(name, declaration);
        Ok(())
    }

    /// Find a declaration by name.
    #[must_use]
    pub fn declaration(
        &self,
        name: &str,
    ) -> Option<&ParameterDeclaration> {
        self.declarations
            .iter()
            .find_map(|(candidate, declaration)| {
                (candidate.as_str() == name).then_some(declaration)
            })
    }

    /// Bind one declared parameter.
    ///
    /// This validates the declaration category and expression structure.
    /// It does not perform general semantic type checking or evaluate
    /// constraints.
    pub fn bind(
        &mut self,
        name: impl Into<String>,
        value: ParameterValue,
    ) -> Result<(), ParameterError> {
        self.bind_many([(name.into(), value)])
    }

    /// Apply multiple bindings atomically.
    ///
    /// All names, duplicate arguments, declaration categories, and existing
    /// bindings are checked before any binding is inserted. If validation
    /// fails, the parameter set remains unchanged.
    ///
    /// # Errors
    ///
    /// Returns a structured error for an unknown parameter, duplicate
    /// argument, duplicate existing binding, or category mismatch.
    pub fn bind_many<I, N>(
        &mut self,
        bindings: I,
    ) -> Result<(), ParameterError>
    where
        I: IntoIterator<Item = (N, ParameterValue)>,
        N: Into<String>,
    {
        let mut staged = BTreeMap::<ParameterName, ParameterValue>::new();

        for (raw_name, value) in bindings {
            let name = ParameterName::new(raw_name)?;

            let declaration = self
                .declarations
                .get(&name)
                .ok_or_else(|| ParameterError::UnknownParameter(name.clone()))?;

            if self.bindings.contains_key(&name) {
                return Err(ParameterError::AlreadyBound(name));
            }

            if staged.contains_key(&name) {
                return Err(ParameterError::DuplicateArgument(name));
            }

            value.validate_for(declaration.kind())?;
            staged.insert(name, value);
        }

        // No validation that can return a normal error remains after this
        // point. The update is logically atomic with respect to validation.
        self.bindings.extend(staged);
        Ok(())
    }

    /// Remove an explicit binding.
    ///
    /// The declaration remains present. Returns the removed value, if any.
    pub fn unbind(&mut self, name: &str) -> Option<ParameterValue> {
        let key = self
            .bindings
            .keys()
            .find(|candidate| candidate.as_str() == name)
            .cloned()?;

        self.bindings.remove(&key)
    }

    /// Create a deterministic parameter-selection plan.
    ///
    /// Explicit bindings take precedence over declaration defaults.
    /// Missing values remain explicitly unresolved. Constraints are
    /// retained as pending and are never assumed to be satisfied.
    ///
    /// The original parameter set is not mutated.
    #[must_use]
    pub fn resolve(&self) -> ParameterResolution {
        let mut parameters = BTreeMap::new();
        let mut unresolved = BTreeSet::new();
        let mut pending_constraints = BTreeMap::new();

        for (name, declaration) in &self.declarations {
            let selected = if let Some(value) = self.bindings.get(name) {
                Some((value.clone(), ParameterValueOrigin::ExplicitBinding))
            } else {
                declaration
                    .default_value()
                    .cloned()
                    .map(|value| {
                        (value, ParameterValueOrigin::DeclarationDefault)
                    })
            };

            let (value, origin) = match selected {
                Some(selected) => selected,
                None => (
                    ParameterValue::Unresolved {
                        symbol: Some(name.as_str().to_owned()),
                        reason: "no explicit binding or declaration default"
                            .to_owned(),
                    },
                    ParameterValueOrigin::ExplicitBinding,
                ),
            };

            if value.is_unresolved() {
                unresolved.insert(name.clone());
            }

            parameters.insert(
                name.clone(),
                ResolvedParameter {
                    name: name.clone(),
                    value,
                    origin,
                },
            );

            if !declaration.constraints().is_empty() {
                pending_constraints
                    .insert(name.clone(), declaration.constraints().len());
            }
        }

        ParameterResolution {
            parameters,
            unresolved,
            pending_constraints,
        }
    }
}

/// Failures from parameter declaration, binding, and plan construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParameterError {
    /// A required name was empty or whitespace-only.
    EmptyName,

    /// A type reference was empty or whitespace-only.
    EmptyTypeReference,

    /// An unresolved value was created without an explanation.
    EmptyUnresolvedReason,

    /// An unresolved value contains an empty symbol.
    EmptyUnresolvedSymbol,

    /// A symbolic width contains an invalid symbol.
    InvalidWidthSymbol,

    /// A constraint description was empty or whitespace-only.
    EmptyConstraintDescription,

    /// An expression root does not exist in its graph.
    InvalidExpressionRoot {
        /// The graph-local root index supplied by the caller.
        root: usize,
    },

    /// A parameter value does not match the declared structural category.
    ValueKindMismatch {
        /// The declared category.
        kind: ParameterKind,
    },

    /// The same parameter was declared more than once.
    DuplicateDeclaration(ParameterName),

    /// A binding refers to a name that has no declaration.
    UnknownParameter(ParameterName),

    /// A parameter already has an explicit binding.
    AlreadyBound(ParameterName),

    /// One batch contains the same parameter name more than once.
    DuplicateArgument(ParameterName),

    /// The expression graph is invalid.
    Expression(ExprError),
}

impl fmt::Display for ParameterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("parameter name must not be empty")
            }
            Self::EmptyTypeReference => {
                formatter.write_str("parameter type reference must not be empty")
            }
            Self::EmptyUnresolvedReason => {
                formatter.write_str("unresolved value reason must not be empty")
            }
            Self::EmptyUnresolvedSymbol => {
                formatter.write_str("unresolved symbol must not be empty")
            }
            Self::InvalidWidthSymbol => {
                formatter.write_str("symbolic width contains an invalid symbol")
            }
            Self::EmptyConstraintDescription => {
                formatter.write_str("constraint description must not be empty")
            }
            Self::InvalidExpressionRoot { root } => {
                write!(formatter, "expression root {root} does not exist")
            }
            Self::ValueKindMismatch { kind } => {
                write!(
                    formatter,
                    "parameter value does not match declared category {kind:?}"
                )
            }
            Self::DuplicateDeclaration(name) => {
                write!(formatter, "parameter `{name}` is already declared")
            }
            Self::UnknownParameter(name) => {
                write!(formatter, "parameter `{name}` is not declared")
            }
            Self::AlreadyBound(name) => {
                write!(formatter, "parameter `{name}` is already bound")
            }
            Self::DuplicateArgument(name) => {
                write!(
                    formatter,
                    "parameter `{name}` occurs more than once in a binding batch"
                )
            }
            Self::Expression(error) => {
                write!(formatter, "invalid parameter expression: {error}")
            }
        }
    }
}

impl std::error::Error for ParameterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Expression(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value_parameter(name: &str) -> ParameterDeclaration {
        ParameterDeclaration::new(name, ParameterKind::Value)
            .expect("valid parameter declaration")
    }

    fn width_value(name: &str) -> ParameterValue {
        ParameterValue::Width(
            BitWidth::symbol(name).expect("valid symbolic width"),
        )
    }

    #[test]
    fn rejects_empty_parameter_names() {
        assert!(matches!(
            ParameterName::new(" \t "),
            Err(ParameterError::EmptyName)
        ));
    }

    #[test]
    fn rejects_duplicate_declarations() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(value_parameter("WIDTH"))
            .expect("first declaration");

        assert!(matches!(
            parameters.declare(value_parameter("WIDTH")),
            Err(ParameterError::DuplicateDeclaration(_))
        ));
    }

    #[test]
    fn binding_batch_is_atomic_on_unknown_parameter() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(value_parameter("WIDTH"))
            .expect("declaration");

        let result = parameters.bind_many([
            ("WIDTH", width_value("W")),
            ("MISSING", width_value("M")),
        ]);

        assert!(matches!(
            result,
            Err(ParameterError::UnknownParameter(_))
        ));
        assert!(parameters.bindings().is_empty());
    }

    #[test]
    fn rejects_duplicate_arguments_in_one_batch() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(value_parameter("WIDTH"))
            .expect("declaration");

        assert!(matches!(
            parameters.bind_many([
                ("WIDTH", width_value("W1")),
                ("WIDTH", width_value("W2")),
            ]),
            Err(ParameterError::DuplicateArgument(_))
        ));

        assert!(parameters.bindings().is_empty());
    }

    #[test]
    fn explicit_binding_takes_precedence_over_default() {
        let mut parameters = ParameterSet::new();

        let declaration = value_parameter("WIDTH")
            .with_default(width_value("DEFAULT_WIDTH"))
            .expect("default");

        parameters
            .declare(declaration)
            .expect("declaration");

        parameters
            .bind("WIDTH", width_value("SUPPLIED_WIDTH"))
            .expect("binding");

        let plan = parameters.resolve();
        let selected = plan
            .parameters()
            .get(&ParameterName::new("WIDTH").expect("name"))
            .expect("resolved parameter");

        assert_eq!(
            selected.origin(),
            ParameterValueOrigin::ExplicitBinding
        );
        assert_eq!(
            selected.value(),
            &width_value("SUPPLIED_WIDTH")
        );
        assert!(plan.unresolved().is_empty());
    }

    #[test]
    fn missing_values_remain_unresolved() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(value_parameter("WIDTH"))
            .expect("declaration");

        let plan = parameters.resolve();

        assert_eq!(plan.unresolved().len(), 1);
        assert!(!plan.has_no_unresolved_parameters());
    }

    #[test]
    fn defaults_are_selected_without_mutating_the_set() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(
                value_parameter("WIDTH")
                    .with_default(width_value("DEFAULT_WIDTH"))
                    .expect("default"),
            )
            .expect("declaration");

        let plan = parameters.resolve();

        assert_eq!(parameters.bindings().len(), 0);
        assert_eq!(
            plan.parameters()
                .values()
                .next()
                .expect("parameter")
                .origin(),
            ParameterValueOrigin::DeclarationDefault
        );
    }

    #[test]
    fn constraints_remain_pending_until_evaluated_elsewhere() {
        let mut graph = ExprGraph::new();
        let root = graph
            .symbol("WIDTH")
            .expect("symbol expression");

        let expression =
            ParameterExpression::new(graph, root).expect("expression");

        let constraint = ParameterConstraint::new(
            expression,
            Some("WIDTH must satisfy the module contract".to_owned()),
        )
        .expect("constraint");

        let mut parameters = ParameterSet::new();

        parameters
            .declare(
                value_parameter("WIDTH")
                    .with_default(width_value("DEFAULT_WIDTH"))
                    .expect("default")
                    .with_constraint(constraint),
            )
            .expect("declaration");

        let plan = parameters.resolve();

        assert!(plan.has_pending_constraints());
        assert_eq!(
            plan.pending_constraints().values().copied().sum::<usize>(),
            1
        );
    }

    #[test]
    fn expression_dependencies_are_deterministic() {
        let mut graph = ExprGraph::new();
        let left = graph.symbol("WIDTH").expect("left symbol");
        let right = graph.symbol("LANES").expect("right symbol");

        let root = graph
            .insert(ExprKind::Binary {
                operator:
                    crate::hdl::hdlir::values::expressions::BinaryOperator::Add,
                lhs: left,
                rhs: right,
            })
            .expect("binary expression");

        let expression =
            ParameterExpression::new(graph, root).expect("expression");

        let symbols = expression.referenced_symbols();

        assert!(symbols.contains("WIDTH"));
        assert!(symbols.contains("LANES"));
        assert_eq!(symbols.len(), 2);
    }

    #[test]
    fn type_parameters_require_type_references() {
        let declaration =
            ParameterDeclaration::new("ELEMENT_TYPE", ParameterKind::Type)
                .expect("type parameter");

        let mut parameters = ParameterSet::new();
        parameters.declare(declaration).expect("declaration");

        assert!(matches!(
            parameters.bind("ELEMENT_TYPE", width_value("WIDTH")),
            Err(ParameterError::ValueKindMismatch {
                kind: ParameterKind::Type
            })
        ));

        parameters
            .bind(
                "ELEMENT_TYPE",
                ParameterValue::TypeReference("logic.word".to_owned()),
            )
            .expect("type binding");
    }
}
