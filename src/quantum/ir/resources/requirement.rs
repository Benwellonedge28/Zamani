//! Zamani Quantum IR — Resource Requirement Algebra
//!
//! This module defines the semantic requirement layer for quantum resources.
//!
//! # Architectural role
//!
//! `requirement.rs` answers:
//!
//! > What resources must be available for this semantic IR object to be
//! > validly lowered or executed?
//!
//! It does NOT:
//!
//! - discover hardware;
//! - allocate hardware;
//! - route qubits;
//! - schedule operations;
//! - select a backend;
//! - inspect device calibration;
//! - execute quantum programs;
//! - simulate quantum states;
//! - perform optimization;
//! - perform QEC decoding.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Separation of responsibilities
//!
//! ```text
//! resource.rs
//!     │
//!     │ primitive resource vocabulary
//!     ▼
//! requirement.rs
//!     │
//!     │ semantic resource demand
//!     ▼
//! capability.rs / topology.rs / mapping.rs
//!     │
//!     │ target compatibility
//!     ▼
//! quantum::hardware
//!     │
//!     │ physical realization
//!     ▼
//! routing / scheduling / backend
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level.
//!
//! Resource requirements describe what that program needs without encoding a
//! fixed quantum-machine size.
//!
//! The following are data:
//!
//! ```text
//! 1 qubit
//! 64 qubits
//! 4_096 qubits
//! 1_000_000 qubits
//! N qubits
//! ```
//!
//! None is an architectural limit.
//!
//! An explicitly unbounded requirement means that the semantic object has no
//! finite upper requirement at this IR boundary. It does NOT claim that a
//! physical machine has infinite resources.
//!
//! # Dependency boundary
//!
//! This module depends only on lower-level IR contracts:
//!
//! - `quantum::ir::resources::resource`;
//! - `quantum::ir::qubit`.
//!
//! It must not depend on:
//!
//! - frontend;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - simulator;
//! - QEC;
//! - backend execution.
//!
//! # Qubit identity
//!
//! Logical qubits are identified exclusively by:
//!
//! `quantum::ir::qubit::QubitId`
//!
//! This file never defines another qubit identity type.
//!
//! # Scaling
//!
//! No fixed qubit count, register size, operation count, topology size,
//! architecture count, vendor count, or resource-kind count is encoded here.
//!
//! Quantities use the resource primitives supplied by `resource.rs`.
//!
//! # Determinism
//!
//! Requirement collections preserve insertion order.
//!
//! This is intentional:
//!
//! - semantic equality remains deterministic;
//! - diagnostics can preserve source/compiler order;
//! - serialization layers can sort when canonical encoding requires it;
//! - no `HashMap` iteration order is exposed.
//!
//! # Arithmetic
//!
//! Aggregate calculations use checked arithmetic through
//! `ResourceQuantity::checked_add`.
//!
//! Arithmetic overflow is therefore an explicit error rather than wraparound.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! `resource.rs` owns primitive resource concepts.
//!
//! This module owns:
//!
//! - requirement identity;
//! - requirement mode;
//! - requirement origin;
//! - requirement composition;
//! - requirement collections;
//! - aggregate minimums;
//! - compatibility checks against capacities;
//! - deterministic diagnostics.
//!
//! `resource.rs` remains responsible for:
//!
//! - `ResourceKind`;
//! - `ResourceQuantity`;
//! - `ResourceRange`;
//! - `ResourceScope`;
//! - `ResourceCapacity`;
//! - low-level resource errors.
//!
//! The canonical `ResourceRequirement` type is moved here when the resources
//! submodule is integrated. `resource.rs` should then re-export it for legacy
//! callers if compatibility is required.
//!
//! # Important invariant
//!
//! A requirement is declarative.
//!
//! Calling any method in this module must never allocate hardware, mutate a
//! target, or perform routing/scheduling.
//!
//! # Canonical integration
//!
//! Recommended imports elsewhere:
//!
//! ```text
//! use crate::quantum::ir::resources::requirement::ResourceRequirement;
//! use crate::quantum::ir::resources::requirement::ResourceRequirements;
//! ```
//!
//! Compatibility exports may additionally expose the types through
//! `quantum::ir::resources` and, temporarily, through the legacy
//! `quantum::ir::resource` path.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::resource::{
    ResourceCapacity,
    ResourceError,
    ResourceKind,
    ResourceQuantity,
    ResourceRange,
    ResourceScope,
};
use super::super::qubit::QubitId;

// =============================================================================
// Requirement identity
// =============================================================================

/// Stable semantic identity for a resource requirement.
///
/// The identity is local to the IR requirement collection and is not a
/// hardware identifier.
///
/// `u64` is used instead of `usize` so semantic identity does not depend on
/// host pointer width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRequirementId(u64);

impl ResourceRequirementId {
    /// Creates an identifier from its stable numeric value.
    ///
    /// The value is opaque to consumers. Requirement identity must not be
    /// interpreted as a memory address or hardware identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ResourceRequirementId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "requirement:{}", self.0)
    }
}

// =============================================================================
// Requirement mode
// =============================================================================

/// Semantic relationship between a requirement and the available capacity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequirementMode {
    /// The resource must exist and satisfy the specified range.
    Required,

    /// The resource is useful but not mandatory for the semantic program.
    ///
    /// A downstream compiler may choose an alternative implementation when
    /// this resource is unavailable.
    Optional,

    /// The resource is required only when the corresponding semantic feature
    /// is selected during lowering.
    Conditional,
}

impl Default for RequirementMode {
    fn default() -> Self {
        Self::Required
    }
}

impl RequirementMode {
    /// Returns whether the requirement is mandatory at the current IR level.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Returns whether the requirement is optional.
    #[must_use]
    pub const fn is_optional(self) -> bool {
        matches!(self, Self::Optional)
    }

    /// Returns whether the requirement is conditional.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(self, Self::Conditional)
    }

    /// Returns a stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Conditional => "conditional",
        }
    }
}

impl fmt::Display for RequirementMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Requirement origin
// =============================================================================

/// Semantic source of a resource requirement.
///
/// This identifies why the requirement exists without coupling the IR to a
/// particular compiler pass implementation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequirementOrigin {
    /// Declared explicitly by the Zamani program.
    Program,

    /// Implied by a semantic operation.
    Operation,

    /// Implied by a quantum model.
    Model,

    /// Implied by a pulse program.
    Pulse,

    /// Implied by control flow.
    ControlFlow,

    /// Implied by logical/fault-tolerant representation.
    Logical,

    /// Implied by a distributed quantum program.
    Distributed,

    /// Added by a compilation transformation.
    Compilation,

    /// Added by a target-independent lowering step.
    Lowering,

    /// Extension-defined origin.
    Custom(String),
}

impl Default for RequirementOrigin {
    fn default() -> Self {
        Self::Program
    }
}

impl RequirementOrigin {
    /// Creates a custom origin.
    pub fn custom(value: impl Into<String>) -> Result<Self, RequirementError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RequirementError::EmptyOrigin);
        }

        Ok(Self::Custom(value))
    }
}

// =============================================================================
// Requirement
// =============================================================================

/// One semantic quantum resource requirement.
///
/// This is deliberately a declarative object.
///
/// It does not allocate or select resources.
///
/// # Examples
///
/// ```text
/// at least 100 logical qubits
/// at least 1 measurement capability resource
/// between 2 and 8 channels
/// exactly 3 logical qubits in a semantic scope
/// ```
///
/// The underlying resource kind/range/scope remain owned by `resource.rs`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceRequirement {
    id: ResourceRequirementId,
    kind: ResourceKind,
    range: ResourceRange,
    scope: ResourceScope,
    mode: RequirementMode,
    origin: RequirementOrigin,
    label: Option<String>,
}

impl ResourceRequirement {
    /// Creates a required exact global resource requirement.
    #[must_use]
    pub fn exact(
        id: ResourceRequirementId,
        kind: ResourceKind,
        amount: u64,
    ) -> Self {
        Self {
            id,
            kind,
            range: ResourceRange::exact(amount),
            scope: ResourceScope::Global,
            mode: RequirementMode::Required,
            origin: RequirementOrigin::Program,
            label: None,
        }
    }

    /// Creates a required minimum global resource requirement.
    #[must_use]
    pub fn at_least(
        id: ResourceRequirementId,
        kind: ResourceKind,
        amount: u64,
    ) -> Self {
        Self {
            id,
            kind,
            range: ResourceRange::at_least(amount),
            scope: ResourceScope::Global,
            mode: RequirementMode::Required,
            origin: RequirementOrigin::Program,
            label: None,
        }
    }

    /// Creates a finite bounded global requirement.
    pub fn between(
        id: ResourceRequirementId,
        kind: ResourceKind,
        minimum: u64,
        maximum: u64,
    ) -> Result<Self, RequirementError> {
        Ok(Self {
            id,
            kind,
            range: ResourceRange::between(minimum, maximum)?,
            scope: ResourceScope::Global,
            mode: RequirementMode::Required,
            origin: RequirementOrigin::Program,
            label: None,
        })
    }

    /// Creates a requirement from an existing resource range.
    #[must_use]
    pub fn from_range(
        id: ResourceRequirementId,
        kind: ResourceKind,
        range: ResourceRange,
    ) -> Self {
        Self {
            id,
            kind,
            range,
            scope: ResourceScope::Global,
            mode: RequirementMode::Required,
            origin: RequirementOrigin::Program,
            label: None,
        }
    }

    /// Returns the stable requirement identifier.
    #[must_use]
    pub const fn id(&self) -> ResourceRequirementId {
        self.id
    }

    /// Returns the resource kind.
    #[must_use]
    pub fn kind(&self) -> &ResourceKind {
        &self.kind
    }

    /// Returns the required range.
    #[must_use]
    pub const fn range(&self) -> ResourceRange {
        self.range
    }

    /// Returns the semantic scope.
    #[must_use]
    pub fn scope(&self) -> &ResourceScope {
        &self.scope
    }

    /// Returns the requirement mode.
    #[must_use]
    pub const fn mode(&self) -> RequirementMode {
        self.mode
    }

    /// Returns the requirement origin.
    #[must_use]
    pub fn origin(&self) -> &RequirementOrigin {
        &self.origin
    }

    /// Returns the optional semantic label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Associates this requirement with a semantic scope.
    #[must_use]
    pub fn with_scope(mut self, scope: ResourceScope) -> Self {
        self.scope = scope;
        self
    }

    /// Associates this requirement with one logical qubit.
    #[must_use]
    pub const fn for_logical_qubit(mut self, qubit: QubitId) -> Self {
        self.scope = ResourceScope::LogicalQubit(qubit);
        self
    }

    /// Sets the requirement mode.
    #[must_use]
    pub const fn with_mode(mut self, mode: RequirementMode) -> Self {
        self.mode = mode;
        self
    }

    /// Marks this requirement as optional.
    #[must_use]
    pub const fn optional(self) -> Self {
        self.with_mode(RequirementMode::Optional)
    }

    /// Marks this requirement as conditional.
    #[must_use]
    pub const fn conditional(self) -> Self {
        self.with_mode(RequirementMode::Conditional)
    }

    /// Sets the semantic origin.
    #[must_use]
    pub fn with_origin(mut self, origin: RequirementOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// Adds a descriptive semantic label.
    ///
    /// The label is never interpreted as a hardware identifier.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the minimum quantity demanded by this requirement.
    #[must_use]
    pub const fn minimum(&self) -> u64 {
        self.range.minimum()
    }

    /// Returns the maximum semantic quantity demanded by this requirement.
    #[must_use]
    pub const fn maximum(&self) -> ResourceQuantity {
        self.range.maximum()
    }

    /// Returns whether the upper bound is unbounded.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.range.is_unbounded()
    }

    /// Checks a finite capacity against this requirement.
    #[must_use]
    pub fn accepts(&self, capacity: u64) -> bool {
        self.range.accepts(capacity)
    }

    /// Checks a resource capacity against this requirement.
    ///
    /// Resource kind and scope must match.
    #[must_use]
    pub fn satisfies(&self, capacity: &ResourceCapacity) -> bool {
        if self.kind != *capacity.kind() {
            return false;
        }

        if self.scope != *capacity.scope() {
            return false;
        }

        match capacity.capacity() {
            ResourceQuantity::Unbounded => true,
            ResourceQuantity::Finite(value) => self.accepts(value),
        }
    }

    /// Validates the local semantic invariants.
    pub fn validate(&self) -> Result<(), RequirementError> {
        if let Some(label) = self.label() {
            if label.trim().is_empty() {
                return Err(RequirementError::EmptyLabel);
            }
        }

        if let RequirementOrigin::Custom(origin) = &self.origin {
            if origin.trim().is_empty() {
                return Err(RequirementError::EmptyOrigin);
            }
        }

        if let ResourceScope::Named(scope) = self.scope() {
            if scope.trim().is_empty() {
                return Err(RequirementError::EmptyScope);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Requirement satisfaction
// =============================================================================

/// Detailed result of evaluating a requirement against a capacity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequirementSatisfaction {
    /// Requirement is satisfied.
    Satisfied,

    /// Requirement is optional and is unavailable.
    OptionalUnavailable,

    /// Requirement is conditional and is currently inactive.
    ConditionalInactive,

    /// Resource kinds differ.
    KindMismatch,

    /// Resource scopes differ.
    ScopeMismatch,

    /// Available finite capacity is below the minimum.
    Insufficient {
        /// Required minimum.
        required: u64,

        /// Available capacity.
        available: u64,
    },

    /// Available capacity violates a finite upper requirement.
    ExceedsMaximum {
        /// Maximum accepted capacity.
        maximum: u64,

        /// Available capacity.
        available: u64,
    },
}

impl RequirementSatisfaction {
    /// Returns whether the requirement is satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        matches!(
            self,
            Self::Satisfied
                | Self::OptionalUnavailable
                | Self::ConditionalInactive
        )
    }

    /// Returns whether the resource is genuinely available and satisfies the
    /// requirement rather than merely being optional/inactive.
    #[must_use]
    pub const fn is_fulfilled(&self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Returns whether this result represents a hard failure.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        matches!(
            self,
            Self::KindMismatch
                | Self::ScopeMismatch
                | Self::Insufficient { .. }
                | Self::ExceedsMaximum { .. }
        )
    }
}

// =============================================================================
// Requirement collection
// =============================================================================

/// Deterministic collection of resource requirements.
///
/// Requirements remain independent.
///
/// They are not automatically merged because two requirements may represent
/// different semantic consumers even when they use the same resource kind.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ResourceRequirements {
    requirements: Vec<ResourceRequirement>,
}

impl ResourceRequirements {
    /// Creates an empty requirement collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    /// Creates a collection from existing requirements.
    ///
    /// The supplied order is preserved.
    #[must_use]
    pub fn from_vec(requirements: Vec<ResourceRequirement>) -> Self {
        Self { requirements }
    }

    /// Adds one requirement.
    ///
    /// Duplicate requirement IDs are rejected because IDs are stable semantic
    /// identities within this collection.
    pub fn add(
        &mut self,
        requirement: ResourceRequirement,
    ) -> Result<(), RequirementError> {
        requirement.validate()?;

        if self.contains_id(requirement.id()) {
            return Err(RequirementError::DuplicateId(requirement.id()));
        }

        self.requirements.push(requirement);
        Ok(())
    }

    /// Adds an exact required global resource.
    pub fn require_exact(
        &mut self,
        id: ResourceRequirementId,
        kind: ResourceKind,
        amount: u64,
    ) -> Result<(), RequirementError> {
        self.add(ResourceRequirement::exact(id, kind, amount))
    }

    /// Adds a minimum required global resource.
    pub fn require_at_least(
        &mut self,
        id: ResourceRequirementId,
        kind: ResourceKind,
        amount: u64,
    ) -> Result<(), RequirementError> {
        self.add(ResourceRequirement::at_least(id, kind, amount))
    }

    /// Adds a bounded required global resource.
    pub fn require_between(
        &mut self,
        id: ResourceRequirementId,
        kind: ResourceKind,
        minimum: u64,
        maximum: u64,
    ) -> Result<(), RequirementError> {
        self.add(ResourceRequirement::between(
            id,
            kind,
            minimum,
            maximum,
        )?)
    }

    /// Adds an arbitrary requirement.
    pub fn push(
        &mut self,
        requirement: ResourceRequirement,
    ) -> Result<(), RequirementError> {
        self.add(requirement)
    }

    /// Returns all requirements in insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[ResourceRequirement] {
        &self.requirements
    }

    /// Returns the number of requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns an iterator over requirements.
    pub fn iter(&self) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements.iter()
    }

    /// Finds a requirement by stable identifier.
    #[must_use]
    pub fn get(&self, id: ResourceRequirementId) -> Option<&ResourceRequirement> {
        self.requirements.iter().find(|requirement| requirement.id() == id)
    }

    /// Returns whether an ID already exists.
    #[must_use]
    pub fn contains_id(&self, id: ResourceRequirementId) -> bool {
        self.get(id).is_some()
    }

    /// Returns requirements for one resource kind.
    pub fn for_kind(
        &self,
        kind: &ResourceKind,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements
            .iter()
            .filter(move |requirement| requirement.kind() == kind)
    }

    /// Returns requirements for one kind and scope.
    pub fn for_kind_and_scope(
        &self,
        kind: &ResourceKind,
        scope: &ResourceScope,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements.iter().filter(move |requirement| {
            requirement.kind() == kind && requirement.scope() == scope
        })
    }

    /// Returns all hard requirements.
    pub fn required(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements
            .iter()
            .filter(|requirement| requirement.mode().is_required())
    }

    /// Returns all optional requirements.
    pub fn optional(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements
            .iter()
            .filter(|requirement| requirement.mode().is_optional())
    }

    /// Returns all conditional requirements.
    pub fn conditional(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements
            .iter()
            .filter(|requirement| requirement.mode().is_conditional())
    }

    /// Merges another collection.
    ///
    /// Requirement identities remain independent and are checked for
    /// collisions.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), RequirementError> {
        for requirement in &other.requirements {
            if self.contains_id(requirement.id()) {
                return Err(RequirementError::DuplicateId(requirement.id()));
            }
        }

        self.requirements
            .try_reserve(other.requirements.len())
            .map_err(|_| RequirementError::AllocationFailure)?;

        self.requirements
            .extend(other.requirements.iter().cloned());

        Ok(())
    }

    /// Computes the aggregate minimum required amount for a kind and scope.
    ///
    /// Only `Required` requirements participate.
    ///
    /// Optional and inactive conditional requirements do not increase the hard
    /// minimum.
    pub fn minimum_required(
        &self,
        kind: &ResourceKind,
        scope: &ResourceScope,
    ) -> Result<ResourceQuantity, RequirementError> {
        let mut total = ResourceQuantity::Finite(0);

        for requirement in self.for_kind_and_scope(kind, scope) {
            if !requirement.mode().is_required() {
                continue;
            }

            total = total
                .checked_add(ResourceQuantity::Finite(requirement.minimum()))
                .map_err(RequirementError::Resource)?;
        }

        Ok(total)
    }

    /// Checks all required requirements against a set of capacities.
    ///
    /// The caller supplies capacities explicitly; this function performs no
    /// hardware discovery.
    pub fn evaluate(
        &self,
        capacities: &[ResourceCapacity],
    ) -> Vec<RequirementEvaluation> {
        self.requirements
            .iter()
            .map(|requirement| {
                RequirementEvaluation::evaluate(requirement, capacities)
            })
            .collect()
    }

    /// Validates every requirement.
    pub fn validate(&self) -> Result<(), RequirementError> {
        for requirement in &self.requirements {
            requirement.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Evaluation
// =============================================================================

/// Evaluation of one requirement against available capacities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequirementEvaluation {
    requirement_id: ResourceRequirementId,
    result: RequirementSatisfaction,
}

impl RequirementEvaluation {
    /// Evaluates a requirement against explicitly supplied capacities.
    #[must_use]
    pub fn evaluate(
        requirement: &ResourceRequirement,
        capacities: &[ResourceCapacity],
    ) -> Self {
        if requirement.mode().is_conditional() {
            return Self {
                requirement_id: requirement.id(),
                result: RequirementSatisfaction::ConditionalInactive,
            };
        }

        let matching = capacities.iter().find(|capacity| {
            capacity.kind() == requirement.kind()
                && capacity.scope() == requirement.scope()
        });

        let result = match matching {
            Some(capacity) => match capacity.capacity() {
                ResourceQuantity::Unbounded => RequirementSatisfaction::Satisfied,

                ResourceQuantity::Finite(value) => {
                    if value < requirement.minimum() {
                        RequirementSatisfaction::Insufficient {
                            required: requirement.minimum(),
                            available: value,
                        }
                    } else {
                        match requirement.maximum() {
                            ResourceQuantity::Finite(maximum)
                                if value > maximum =>
                            {
                                RequirementSatisfaction::ExceedsMaximum {
                                    maximum,
                                    available: value,
                                }
                            }
                            _ => RequirementSatisfaction::Satisfied,
                        }
                    }
                }
            },

            None if requirement.mode().is_optional() => {
                RequirementSatisfaction::OptionalUnavailable
            }

            None => RequirementSatisfaction::Insufficient {
                required: requirement.minimum(),
                available: 0,
            },
        };

        Self {
            requirement_id: requirement.id(),
            result,
        }
    }

    /// Returns the requirement identifier.
    #[must_use]
    pub const fn requirement_id(&self) -> ResourceRequirementId {
        self.requirement_id
    }

    /// Returns the evaluation result.
    #[must_use]
    pub fn result(&self) -> &RequirementSatisfaction {
        &self.result
    }

    /// Returns whether the requirement is satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        self.result.is_satisfied()
    }

    /// Returns whether this evaluation is a hard failure.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        self.result.is_failure()
    }
}

// =============================================================================
// Requirement errors
// =============================================================================

/// Errors specific to requirement construction and composition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequirementError {
    /// Underlying resource-model error.
    Resource(ResourceError),

    /// Two requirements use the same stable identity.
    DuplicateId(ResourceRequirementId),

    /// Requirement label is empty.
    EmptyLabel,

    /// Requirement origin is empty.
    EmptyOrigin,

    /// Requirement scope is empty.
    EmptyScope,

    /// Requirement collection allocation failed.
    AllocationFailure,
}

impl fmt::Display for RequirementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resource(error) => write!(formatter, "{error}"),

            Self::DuplicateId(id) => {
                write!(formatter, "duplicate resource requirement id: {id}")
            }

            Self::EmptyLabel => {
                formatter.write_str("resource requirement label cannot be empty")
            }

            Self::EmptyOrigin => {
                formatter.write_str("resource requirement origin cannot be empty")
            }

            Self::EmptyScope => {
                formatter.write_str("resource requirement scope cannot be empty")
            }

            Self::AllocationFailure => {
                formatter.write_str(
                    "resource requirement collection allocation failed",
                )
            }
        }
    }
}

impl std::error::Error for RequirementError {}

impl From<ResourceError> for RequirementError {
    fn from(error: ResourceError) -> Self {
        Self::Resource(error)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> ResourceRequirementId {
        ResourceRequirementId::new(value)
    }

    #[test]
    fn exact_requirement_accepts_exact_capacity() {
        let requirement = ResourceRequirement::exact(
            id(1),
            ResourceKind::logical_qubits(),
            8,
        );

        assert!(requirement.accepts(8));
        assert!(!requirement.accepts(7));
        assert!(!requirement.accepts(9));
    }

    #[test]
    fn minimum_requirement_scales_without_architectural_limit() {
        let requirement = ResourceRequirement::at_least(
            id(1),
            ResourceKind::logical_qubits(),
            1_000_000,
        );

        assert!(requirement.accepts(1_000_000));
        assert!(requirement.accepts(u64::MAX));
    }

    #[test]
    fn unbounded_range_is_not_numeric_maximum() {
        let requirement = ResourceRequirement::at_least(
            id(1),
            ResourceKind::logical_qubits(),
            u64::MAX,
        );

        assert!(requirement.is_unbounded());
        assert!(requirement.accepts(u64::MAX));
    }

    #[test]
    fn logical_qubit_scope_uses_canonical_qubit_id() {
        let qubit = QubitId::new(17);

        let requirement = ResourceRequirement::exact(
            id(1),
            ResourceKind::logical_qubits(),
            1,
        )
        .for_logical_qubit(qubit);

        assert_eq!(
            requirement.scope().logical_qubit_id(),
            Some(qubit)
        );
    }

    #[test]
    fn optional_requirement_can_be_missing() {
        let requirement = ResourceRequirement::exact(
            id(1),
            ResourceKind::channels(),
            1,
        )
        .optional();

        let collection = ResourceRequirements::from_vec(vec![requirement]);

        let result = collection.evaluate(&[]);

        assert_eq!(
            result[0].result(),
            &RequirementSatisfaction::OptionalUnavailable
        );

        assert!(result[0].is_satisfied());
        assert!(!result[0].is_failure());
    }

    #[test]
    fn required_missing_resource_is_failure() {
        let requirement = ResourceRequirement::exact(
            id(1),
            ResourceKind::logical_qubits(),
            8,
        );

        let collection = ResourceRequirements::from_vec(vec![requirement]);

        let result = collection.evaluate(&[]);

        assert_eq!(
            result[0].result(),
            &RequirementSatisfaction::Insufficient {
                required: 8,
                available: 0,
            }
        );

        assert!(result[0].is_failure());
    }

    #[test]
    fn finite_capacity_satisfies_requirement() {
        let requirement = ResourceRequirement::at_least(
            id(1),
            ResourceKind::logical_qubits(),
            8,
        );

        let capacity = ResourceCapacity::finite(
            ResourceKind::logical_qubits(),
            16,
        );

        assert!(requirement.satisfies(&capacity));
    }

    #[test]
    fn unbounded_capacity_satisfies_requirement() {
        let requirement = ResourceRequirement::at_least(
            id(1),
            ResourceKind::logical_qubits(),
            u64::MAX,
        );

        let capacity =
            ResourceCapacity::unbounded(ResourceKind::logical_qubits());

        assert!(requirement.satisfies(&capacity));
    }

    #[test]
    fn mismatched_kind_is_rejected() {
        let requirement = ResourceRequirement::exact(
            id(1),
            ResourceKind::logical_qubits(),
            8,
        );

        let capacity = ResourceCapacity::finite(
            ResourceKind::physical_qubits(),
            8,
        );

        assert!(!requirement.satisfies(&capacity));
    }

    #[test]
    fn duplicate_requirement_ids_are_rejected() {
        let mut requirements = ResourceRequirements::new();

        requirements
            .add(ResourceRequirement::exact(
                id(1),
                ResourceKind::logical_qubits(),
                1,
            ))
            .expect("first requirement should succeed");

        let result = requirements.add(
            ResourceRequirement::exact(
                id(1),
                ResourceKind::classical_bits(),
                1,
            ),
        );

        assert_eq!(
            result,
            Err(RequirementError::DuplicateId(id(1)))
        );
    }

    #[test]
    fn minimum_requirements_are_checked_with_checked_arithmetic() {
        let mut requirements = ResourceRequirements::new();

        requirements
            .add(ResourceRequirement::at_least(
                id(1),
                ResourceKind::logical_qubits(),
                u64::MAX,
            ))
            .expect("first requirement should succeed");

        requirements
            .add(ResourceRequirement::at_least(
                id(2),
                ResourceKind::logical_qubits(),
                1,
            ))
            .expect("second requirement should succeed");

        let result = requirements.minimum_required(
            &ResourceKind::logical_qubits(),
            &ResourceScope::Global,
        );

        assert_eq!(
            result,
            Err(RequirementError::Resource(
                ResourceError::ArithmeticOverflow
            ))
        );
    }

    #[test]
    fn custom_origin_is_extensible() {
        let origin = RequirementOrigin::custom("future_quantum_model")
            .expect("custom origin should be accepted");

        assert_eq!(
            origin,
            RequirementOrigin::Custom(
                String::from("future_quantum_model")
            )
        );
    }

    #[test]
    fn deterministic_iteration_order_is_preserved() {
        let mut requirements = ResourceRequirements::new();

        requirements
            .add(ResourceRequirement::exact(
                id(2),
                ResourceKind::logical_qubits(),
                2,
            ))
            .expect("requirement should succeed");

        requirements
            .add(ResourceRequirement::exact(
                id(1),
                ResourceKind::logical_qubits(),
                1,
            ))
            .expect("requirement should succeed");

        assert_eq!(
            requirements.as_slice()[0].id(),
            id(2)
        );

        assert_eq!(
            requirements.as_slice()[1].id(),
            id(1)
        );
    }
}