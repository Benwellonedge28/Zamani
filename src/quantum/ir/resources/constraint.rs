//! Zamani Quantum IR — Resource Constraints
//!
//! This module defines declarative relationships between quantum resource
//! requirements.
//!
//! # Architectural role
//!
//! `constraint.rs` answers:
//!
//! > Under what relationships must semantic resources coexist, differ,
//! > dominate, depend on, or be selected together?
//!
//! A [`ResourceConstraint`] is a semantic declaration. It is NOT an allocator,
//! router, scheduler, hardware selector, simulator, or backend instruction.
//!
//! The canonical dependency direction is:
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ▼
//! resources::resource
//!        │
//!        ▼
//! resources::requirement
//!        │
//!        ▼
//! resources::constraint
//!        │
//!        ├── validation
//!        ├── target compatibility
//!        ├── routing
//!        └── scheduling
//! ```
//!
//! Downstream systems may consume constraints, but this module must remain
//! independent of those systems.
//!
//! # Universal-program principle
//!
//! Zamani quantum programs are written at the semantic level and may be
//! lowered to different machines and machine sizes.
//!
//! This module therefore contains:
//!
//! - no maximum qubit count;
//! - no fixed register size;
//! - no fixed topology;
//! - no vendor identifiers;
//! - no hardware backend identifiers;
//! - no architecture-specific assumptions;
//! - no sentinel such as `u64::MAX` for infinity;
//! - no use of `usize` as semantic identity.
//!
//! A finite integer in this module is a value, not an architectural limit.
//!
//! `Unbounded` is represented by the resource algebra in `resource.rs`; this
//! module does not reinterpret a large integer as infinity.
//!
//! # Responsibility boundary
//!
//! This module OWNS:
//!
//! - constraint identity;
//! - constraint strength;
//! - constraint relation;
//! - constraint operands;
//! - constraint scope;
//! - constraint composition;
//! - constraint validation;
//! - deterministic constraint collections;
//! - declarative satisfaction checking.
//!
//! This module DOES NOT OWN:
//!
//! - resource definitions;
//! - resource capacities;
//! - resource allocation;
//! - physical qubit selection;
//! - routing;
//! - scheduling;
//! - calibration;
//! - backend execution;
//! - capability discovery;
//! - optimization algorithms.
//!
//! Those responsibilities belong to their respective IR/downstream modules.
//!
//! # Canonical dependencies
//!
//! Resource primitives are owned by:
//!
//! `quantum::ir::resources::resource`
//!
//! Semantic requirements are owned by:
//!
//! `quantum::ir::resources::requirement`
//!
//! Logical qubit identity is owned by:
//!
//! `quantum::ir::qubit`
//!
//! New code must use the canonical qubit path:
//!
//! `quantum::ir::qubit::QubitId`
//!
//! No duplicate qubit identity is defined here.
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
//! `resource.rs` provides the primitive resource vocabulary.
//!
//! `requirement.rs` provides semantic resource demands.
//!
//! `constraint.rs` provides relationships between those demands.
//!
//! `capability.rs` may use constraints when expressing conditional resource
//! availability, but `constraint.rs` must not depend on a capability
//! implementation.
//!
//! `topology.rs` may consume locality-related constraints, but this module
//! must not know how a topology is represented or routed.
//!
//! `mapping.rs` may consume qubit-related constraints, but this module must
//! not perform logical-to-physical mapping.
//!
//! `hardware` may evaluate these constraints against concrete resources.
//!
//! # Determinism
//!
//! [`ResourceConstraints`] preserves insertion order.
//!
//! It does not expose hash-map iteration order as semantic order.
//!
//! Canonical serialization layers may impose their own canonical ordering,
//! but that ordering is outside this module.
//!
//! # Constraint semantics
//!
//! A constraint is one of:
//!
//! - equality;
//! - inequality;
//! - lower/upper bound;
//! - dependency;
//! - implication;
//! - mutual exclusion;
//! - co-requirement;
//! - co-location;
//! - separation;
//! - distinctness;
//! - ratio;
//! - custom extension.
//!
//! The relation itself is target-independent.
//!
//! # Important semantic rule
//!
//! A constraint does not allocate resources.
//!
//! For example:
//!
//! ```text
//! logical qubit q0 must be distinct from logical qubit q1
//! ```
//!
//! does NOT mean that `constraint.rs` chooses physical qubits.
//!
//! It only records the semantic requirement.
//!
//! # Overflow policy
//!
//! Any arithmetic performed by this module is checked.
//!
//! Overflow is an explicit error.
//!
//! Wrapping arithmetic is never used for semantic quantities.
//!
//! # Forward compatibility
//!
//! [`ConstraintRelation::Custom`] and [`ConstraintOperand::Custom`] allow
//! future dialects to extend the constraint system without modifying the
//! canonical core for every new quantum technology.
//!
//! Unknown/custom data is explicit rather than silently discarded.
//!
//! # File completion contract
//!
//! This file is complete when:
//!
//! 1. all public constraint concepts are represented here;
//! 2. no resource primitive is duplicated here;
//! 3. no qubit identity is duplicated here;
//! 4. no hardware implementation appears here;
//! 5. validation is deterministic;
//! 6. arithmetic is checked;
//! 7. collections preserve semantic insertion order;
//! 8. custom extensions are explicit;
//! 9. the module compiles independently against the contracts of
//!    `resource.rs`, `requirement.rs`, and `qubit.rs`;
//! 10. downstream modules can consume it without modifying this file.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::requirement::{
    RequirementMode,
    ResourceRequirement,
    ResourceRequirementId,
};
use super::resource::{
    ResourceCapacity,
    ResourceError,
    ResourceKind,
    ResourceQuantity,
    ResourceRange,
    ResourceScope,
};
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Constraint identity
// =============================================================================

/// Stable semantic identity of a resource constraint.
///
/// This identity is local to an IR constraint collection. It is not a memory
/// address, hardware identifier, qubit identifier, or backend job identifier.
///
/// `u64` is deliberately used instead of `usize` so semantic identity does
/// not depend on host pointer width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceConstraintId(u64);

impl ResourceConstraintId {
    /// Creates a constraint identifier from a stable numeric value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable numeric value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ResourceConstraintId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "constraint:{}", self.0)
    }
}

// =============================================================================
// Constraint strength
// =============================================================================

/// Semantic strength of a constraint.
///
/// Strength describes how a downstream compiler should interpret the
/// declaration. It does not itself select a fallback implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintStrength {
    /// The constraint is semantically mandatory.
    Required,

    /// The constraint is preferred but may be violated if a valid lowering
    /// explicitly chooses an alternative.
    Preferred,

    /// The constraint is advisory metadata.
    Advisory,
}

impl Default for ConstraintStrength {
    fn default() -> Self {
        Self::Required
    }
}

impl ConstraintStrength {
    /// Returns whether this constraint is mandatory.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Returns whether this constraint is preferred.
    #[must_use]
    pub const fn is_preferred(self) -> bool {
        matches!(self, Self::Preferred)
    }

    /// Returns whether this constraint is advisory.
    #[must_use]
    pub const fn is_advisory(self) -> bool {
        matches!(self, Self::Advisory)
    }

    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Preferred => "preferred",
            Self::Advisory => "advisory",
        }
    }
}

impl fmt::Display for ConstraintStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Constraint scope
// =============================================================================

/// Semantic scope to which a constraint applies.
///
/// Scope is intentionally independent of physical hardware topology.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintScope {
    /// Applies to the complete semantic program.
    Program,

    /// Applies to a semantic resource group.
    ResourceGroup(String),

    /// Applies to one logical qubit.
    LogicalQubit(QubitId),

    /// Applies to a named semantic region.
    Region(String),

    /// Applies to a named semantic operation group.
    OperationGroup(String),

    /// Extension-defined scope.
    Custom(String),
}

impl Default for ConstraintScope {
    fn default() -> Self {
        Self::Program
    }
}

impl ConstraintScope {
    /// Creates a resource-group scope.
    pub fn resource_group(name: impl Into<String>) -> Result<Self, ConstraintError> {
        let name = name.into();

        validate_non_empty("resource group", &name)?;

        Ok(Self::ResourceGroup(name))
    }

    /// Creates a named region scope.
    pub fn region(name: impl Into<String>) -> Result<Self, ConstraintError> {
        let name = name.into();

        validate_non_empty("region", &name)?;

        Ok(Self::Region(name))
    }

    /// Creates an operation-group scope.
    pub fn operation_group(name: impl Into<String>) -> Result<Self, ConstraintError> {
        let name = name.into();

        validate_non_empty("operation group", &name)?;

        Ok(Self::OperationGroup(name))
    }

    /// Creates a custom scope.
    pub fn custom(name: impl Into<String>) -> Result<Self, ConstraintError> {
        let name = name.into();

        validate_non_empty("custom scope", &name)?;

        Ok(Self::Custom(name))
    }
}

// =============================================================================
// Constraint relation
// =============================================================================

/// Declarative relationship between resource operands.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintRelation {
    /// Two operands must represent equal semantic quantities/identities.
    Equal,

    /// The left operand must differ from the right operand.
    NotEqual,

    /// Left quantity must be less than right quantity.
    LessThan,

    /// Left quantity must be less than or equal to right quantity.
    LessThanOrEqual,

    /// Left quantity must be greater than right quantity.
    GreaterThan,

    /// Left quantity must be greater than or equal to right quantity.
    GreaterThanOrEqual,

    /// If the first operand exists, the second must also exist.
    Requires,

    /// If the first operand exists, the second must not exist.
    Excludes,

    /// Both operands must be selected/available together.
    Together,

    /// Both operands must not be selected/assigned to the same resource.
    Separate,

    /// Operands must refer to distinct resources.
    Distinct,

    /// Operands must share a compatible resource location/group.
    CoLocated,

    /// A requirement is active only when another operand is active.
    Implies,

    /// A requirement is active when another operand is not active.
    ImpliedBy,

    /// A finite ratio relationship.
    Ratio,

    /// A target-independent custom relation.
    Custom(String),
}

impl ConstraintRelation {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Equal => "equal",
            Self::NotEqual => "not_equal",
            Self::LessThan => "less_than",
            Self::LessThanOrEqual => "less_than_or_equal",
            Self::GreaterThan => "greater_than",
            Self::GreaterThanOrEqual => "greater_than_or_equal",
            Self::Requires => "requires",
            Self::Excludes => "excludes",
            Self::Together => "together",
            Self::Separate => "separate",
            Self::Distinct => "distinct",
            Self::CoLocated => "co_located",
            Self::Implies => "implies",
            Self::ImpliedBy => "implied_by",
            Self::Ratio => "ratio",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Creates a custom relation.
    pub fn custom(value: impl Into<String>) -> Result<Self, ConstraintError> {
        let value = value.into();

        validate_non_empty("custom relation", &value)?;

        Ok(Self::Custom(value))
    }

    /// Returns whether this relation requires two operands.
    #[must_use]
    pub const fn requires_two_operands(&self) -> bool {
        true
    }
}

impl fmt::Display for ConstraintRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Constraint operand
// =============================================================================

/// Operand of a resource constraint.
///
/// The operand is intentionally abstract. It can refer to a requirement,
/// resource kind, concrete capacity, logical qubit, or symbolic semantic
/// quantity.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintOperand {
    /// References a semantic resource requirement.
    Requirement(ResourceRequirementId),

    /// References a concrete resource kind.
    ResourceKind(ResourceKind),

    /// References a logical qubit.
    LogicalQubit(QubitId),

    /// References a resource scope.
    Scope(ConstraintScope),

    /// A finite semantic quantity.
    Quantity(u64),

    /// An explicitly unbounded semantic quantity.
    Unbounded,

    /// A resource range.
    Range(ResourceRange),

    /// A concrete resource capacity.
    Capacity(ResourceCapacity),

    /// A symbolic semantic resource name.
    Symbol(String),

    /// Extension-defined operand.
    Custom(String),
}

impl ConstraintOperand {
    /// Creates a requirement operand.
    #[must_use]
    pub const fn requirement(id: ResourceRequirementId) -> Self {
        Self::Requirement(id)
    }

    /// Creates a logical-qubit operand.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a finite quantity operand.
    #[must_use]
    pub const fn quantity(value: u64) -> Self {
        Self::Quantity(value)
    }

    /// Creates an explicitly unbounded operand.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Creates a symbolic operand.
    pub fn symbol(value: impl Into<String>) -> Result<Self, ConstraintError> {
        let value = value.into();

        validate_non_empty("constraint symbol", &value)?;

        Ok(Self::Symbol(value))
    }

    /// Creates a custom operand.
    pub fn custom(value: impl Into<String>) -> Result<Self, ConstraintError> {
        let value = value.into();

        validate_non_empty("custom constraint operand", &value)?;

        Ok(Self::Custom(value))
    }
}

// =============================================================================
// Ratio
// =============================================================================

/// Exact positive ratio used by [`ConstraintRelation::Ratio`].
///
/// The denominator must be non-zero.
///
/// The ratio is represented as integers instead of floating point so that
/// constraint semantics remain deterministic across platforms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRatio {
    numerator: u64,
    denominator: u64,
}

impl ResourceRatio {
    /// Creates a positive or zero numerator ratio.
    ///
    /// The denominator must be non-zero.
    pub const fn new(
        numerator: u64,
        denominator: u64,
    ) -> Result<Self, ConstraintError> {
        if denominator == 0 {
            return Err(ConstraintError::ZeroRatioDenominator);
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.numerator
    }

    /// Returns the denominator.
    #[must_use]
    pub const fn denominator(self) -> u64 {
        self.denominator
    }

    /// Returns whether this ratio is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }
}

impl fmt::Display for ResourceRatio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.numerator, self.denominator)
    }
}

// =============================================================================
// Constraint
// =============================================================================

/// One declarative relationship between semantic resource operands.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceConstraint {
    id: ResourceConstraintId,
    relation: ConstraintRelation,
    left: ConstraintOperand,
    right: ConstraintOperand,
    strength: ConstraintStrength,
    scope: ConstraintScope,
    label: Option<String>,
    ratio: Option<ResourceRatio>,
}

impl ResourceConstraint {
    /// Creates a required binary constraint.
    #[must_use]
    pub const fn new(
        id: ResourceConstraintId,
        relation: ConstraintRelation,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self {
            id,
            relation,
            left,
            right,
            strength: ConstraintStrength::Required,
            scope: ConstraintScope::Program,
            label: None,
            ratio: None,
        }
    }

    /// Creates an equality constraint.
    #[must_use]
    pub const fn equal(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::Equal, left, right)
    }

    /// Creates a non-equality constraint.
    #[must_use]
    pub const fn not_equal(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::NotEqual, left, right)
    }

    /// Creates a less-than constraint.
    #[must_use]
    pub const fn less_than(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::LessThan, left, right)
    }

    /// Creates a less-than-or-equal constraint.
    #[must_use]
    pub const fn less_than_or_equal(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::LessThanOrEqual, left, right)
    }

    /// Creates a greater-than constraint.
    #[must_use]
    pub const fn greater_than(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::GreaterThan, left, right)
    }

    /// Creates a greater-than-or-equal constraint.
    #[must_use]
    pub const fn greater_than_or_equal(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::GreaterThanOrEqual, left, right)
    }

    /// Creates a requirement dependency.
    #[must_use]
    pub const fn requires(
        id: ResourceConstraintId,
        requirement: ResourceRequirementId,
        dependency: ResourceRequirementId,
    ) -> Self {
        Self::new(
            id,
            ConstraintRelation::Requires,
            ConstraintOperand::Requirement(requirement),
            ConstraintOperand::Requirement(dependency),
        )
    }

    /// Creates an exclusion relationship.
    #[must_use]
    pub const fn excludes(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::Excludes, left, right)
    }

    /// Creates a co-location relationship.
    #[must_use]
    pub const fn co_located(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::CoLocated, left, right)
    }

    /// Creates a separation relationship.
    #[must_use]
    pub const fn separate(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::Separate, left, right)
    }

    /// Creates a distinct-resource relationship.
    #[must_use]
    pub const fn distinct(
        id: ResourceConstraintId,
        left: ConstraintOperand,
        right: ConstraintOperand,
    ) -> Self {
        Self::new(id, ConstraintRelation::Distinct, left, right)
    }

    /// Creates an implication.
    #[must_use]
    pub const fn implies(
        id: ResourceConstraintId,
        condition: ConstraintOperand,
        consequence: ConstraintOperand,
    ) -> Self {
        Self::new(
            id,
            ConstraintRelation::Implies,
            condition,
            consequence,
        )
    }

    /// Returns the constraint identity.
    #[must_use]
    pub const fn id(&self) -> ResourceConstraintId {
        self.id
    }

    /// Returns the constraint relation.
    #[must_use]
    pub const fn relation(&self) -> &ConstraintRelation {
        &self.relation
    }

    /// Returns the left operand.
    #[must_use]
    pub const fn left(&self) -> &ConstraintOperand {
        &self.left
    }

    /// Returns the right operand.
    #[must_use]
    pub const fn right(&self) -> &ConstraintOperand {
        &self.right
    }

    /// Returns the constraint strength.
    #[must_use]
    pub const fn strength(&self) -> ConstraintStrength {
        self.strength
    }

    /// Returns the constraint scope.
    #[must_use]
    pub const fn scope(&self) -> &ConstraintScope {
        &self.scope
    }

    /// Returns the optional semantic label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the optional ratio.
    #[must_use]
    pub const fn ratio(&self) -> Option<ResourceRatio> {
        self.ratio
    }

    /// Sets the constraint strength.
    #[must_use]
    pub const fn with_strength(
        mut self,
        strength: ConstraintStrength,
    ) -> Self {
        self.strength = strength;
        self
    }

    /// Marks the constraint as preferred.
    #[must_use]
    pub const fn preferred(self) -> Self {
        self.with_strength(ConstraintStrength::Preferred)
    }

    /// Marks the constraint as advisory.
    #[must_use]
    pub const fn advisory(self) -> Self {
        self.with_strength(ConstraintStrength::Advisory)
    }

    /// Sets the semantic scope.
    #[must_use]
    pub fn with_scope(mut self, scope: ConstraintScope) -> Self {
        self.scope = scope;
        self
    }

    /// Associates this constraint with one logical qubit.
    #[must_use]
    pub const fn for_logical_qubit(mut self, qubit: QubitId) -> Self {
        self.scope = ConstraintScope::LogicalQubit(qubit);
        self
    }

    /// Adds a semantic label.
    ///
    /// The label is never interpreted as a hardware identifier.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Associates a ratio with this constraint.
    ///
    /// The relation must be `Ratio`.
    pub fn with_ratio(
        mut self,
        ratio: ResourceRatio,
    ) -> Result<Self, ConstraintError> {
        if !matches!(self.relation, ConstraintRelation::Ratio) {
            return Err(ConstraintError::RatioOnNonRatioConstraint);
        }

        self.ratio = Some(ratio);

        Ok(self)
    }

    /// Validates this constraint's local invariants.
    pub fn validate(&self) -> Result<(), ConstraintError> {
        validate_operand(&self.left)?;
        validate_operand(&self.right)?;
        validate_scope(&self.scope)?;

        if let Some(label) = self.label() {
            validate_non_empty("constraint label", label)?;
        }

        if matches!(self.relation, ConstraintRelation::Ratio)
            && self.ratio.is_none()
        {
            return Err(ConstraintError::MissingRatio);
        }

        if !matches!(self.relation, ConstraintRelation::Ratio)
            && self.ratio.is_some()
        {
            return Err(ConstraintError::RatioOnNonRatioConstraint);
        }

        Ok(())
    }

    /// Attempts to evaluate the constraint when both operands resolve to
    /// finite numeric quantities.
    ///
    /// Returns `None` when the relation cannot be decided from finite
    /// quantities alone.
    #[must_use]
    pub fn evaluate_finite(
        &self,
        left: u64,
        right: u64,
    ) -> Option<bool> {
        match self.relation {
            ConstraintRelation::Equal => Some(left == right),
            ConstraintRelation::NotEqual => Some(left != right),
            ConstraintRelation::LessThan => Some(left < right),
            ConstraintRelation::LessThanOrEqual => Some(left <= right),
            ConstraintRelation::GreaterThan => Some(left > right),
            ConstraintRelation::GreaterThanOrEqual => Some(left >= right),
            ConstraintRelation::Ratio => {
                let ratio = self.ratio?;

                let lhs = (left as u128)
                    .checked_mul(ratio.denominator as u128)?;

                let rhs = (right as u128)
                    .checked_mul(ratio.numerator as u128)?;

                Some(lhs == rhs)
            }
            ConstraintRelation::Requires
            | ConstraintRelation::Excludes
            | ConstraintRelation::Together
            | ConstraintRelation::Separate
            | ConstraintRelation::Distinct
            | ConstraintRelation::CoLocated
            | ConstraintRelation::Implies
            | ConstraintRelation::ImpliedBy
            | ConstraintRelation::Custom(_) => None,
        }
    }

    /// Returns whether a capacity satisfies a direct requirement operand.
    ///
    /// This method only performs declarative compatibility checking. It does
    /// not allocate the resource.
    #[must_use]
    pub fn accepts_capacity(
        &self,
        requirement: &ResourceRequirement,
        capacity: &ResourceCapacity,
    ) -> bool {
        if self.strength == ConstraintStrength::Advisory {
            return true;
        }

        requirement.satisfies(capacity)
    }
}

// =============================================================================
// Constraint collection
// =============================================================================

/// Deterministic collection of resource constraints.
///
/// Insertion order is preserved.
///
/// Duplicate identities are rejected so that one constraint ID never silently
/// changes meaning.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourceConstraints {
    constraints: Vec<ResourceConstraint>,
}

impl ResourceConstraints {
    /// Creates an empty constraint collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    /// Returns the number of constraints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Adds a constraint.
    ///
    /// Duplicate identities are rejected.
    pub fn push(
        &mut self,
        constraint: ResourceConstraint,
    ) -> Result<(), ConstraintError> {
        constraint.validate()?;

        if self
            .constraints
            .iter()
            .any(|existing| existing.id() == constraint.id())
        {
            return Err(ConstraintError::DuplicateConstraintId(
                constraint.id(),
            ));
        }

        self.constraints.push(constraint);

        Ok(())
    }

    /// Returns a constraint by identity.
    #[must_use]
    pub fn get(
        &self,
        id: ResourceConstraintId,
    ) -> Option<&ResourceConstraint> {
        self.constraints.iter().find(|constraint| constraint.id() == id)
    }

    /// Returns an iterator over constraints in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &ResourceConstraint> {
        self.constraints.iter()
    }

    /// Returns the constraints as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[ResourceConstraint] {
        &self.constraints
    }

    /// Removes a constraint by identity.
    ///
    /// Returns the removed constraint when present.
    pub fn remove(
        &mut self,
        id: ResourceConstraintId,
    ) -> Option<ResourceConstraint> {
        let index = self
            .constraints
            .iter()
            .position(|constraint| constraint.id() == id)?;

        Some(self.constraints.remove(index))
    }

    /// Validates every constraint.
    ///
    /// The first error is returned.
    pub fn validate(&self) -> Result<(), ConstraintError> {
        for constraint in &self.constraints {
            constraint.validate()?;
        }

        Ok(())
    }

    /// Returns all constraints with required strength.
    pub fn required(
        &self,
    ) -> impl Iterator<Item = &ResourceConstraint> {
        self.constraints
            .iter()
            .filter(|constraint| constraint.strength().is_required())
    }

    /// Returns all preferred constraints.
    pub fn preferred(
        &self,
    ) -> impl Iterator<Item = &ResourceConstraint> {
        self.constraints
            .iter()
            .filter(|constraint| constraint.strength().is_preferred())
    }

    /// Returns all advisory constraints.
    pub fn advisory(
        &self,
    ) -> impl Iterator<Item = &ResourceConstraint> {
        self.constraints
            .iter()
            .filter(|constraint| constraint.strength().is_advisory())
    }
}

impl IntoIterator for ResourceConstraints {
    type Item = ResourceConstraint;
    type IntoIter = std::vec::IntoIter<ResourceConstraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResourceConstraints {
    type Item = &'a ResourceConstraint;
    type IntoIter = std::slice::Iter<'a, ResourceConstraint>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.iter()
    }
}

// =============================================================================
// Constraint errors
// =============================================================================

/// Errors produced by resource-constraint construction and validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintError {
    /// A required textual value was empty.
    EmptyValue {
        field: &'static str,
    },

    /// A range or quantity relation is malformed.
    InvalidRelation,

    /// A resource range is invalid.
    InvalidRange,

    /// A constraint has an empty intersection.
    EmptyIntersection,

    /// A ratio denominator was zero.
    ZeroRatioDenominator,

    /// A ratio was attached to a non-ratio constraint.
    RatioOnNonRatioConstraint,

    /// A ratio constraint does not contain a ratio.
    MissingRatio,

    /// A constraint identity already exists.
    DuplicateConstraintId(ResourceConstraintId),

    /// Arithmetic overflow occurred while evaluating a constraint.
    ArithmeticOverflow,

    /// A resource operation could not be evaluated because its semantic
    /// quantity was indeterminate.
    IndeterminateOperation,

    /// A finite quantity cannot satisfy the requested subtraction.
    InsufficientQuantity,

    /// A lower-level resource error.
    Resource(ResourceError),

    /// A requirement reference is malformed or unavailable.
    InvalidRequirementReference,

    /// A constraint contains an unsupported operand combination.
    UnsupportedOperandCombination,
}

impl fmt::Display for ConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => {
                write!(formatter, "{field} must not be empty")
            }
            Self::InvalidRelation => {
                formatter.write_str("invalid resource constraint relation")
            }
            Self::InvalidRange => {
                formatter.write_str("invalid resource constraint range")
            }
            Self::EmptyIntersection => {
                formatter.write_str(
                    "resource constraint ranges have an empty intersection",
                )
            }
            Self::ZeroRatioDenominator => {
                formatter.write_str("resource ratio denominator must not be zero")
            }
            Self::RatioOnNonRatioConstraint => {
                formatter.write_str(
                    "a ratio can only be attached to a ratio constraint",
                )
            }
            Self::MissingRatio => {
                formatter.write_str(
                    "ratio constraint requires an explicit ratio",
                )
            }
            Self::DuplicateConstraintId(id) => {
                write!(formatter, "duplicate resource constraint ID {id}")
            }
            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "resource constraint arithmetic overflow",
                )
            }
            Self::IndeterminateOperation => {
                formatter.write_str(
                    "resource constraint operation is indeterminate",
                )
            }
            Self::InsufficientQuantity => {
                formatter.write_str(
                    "resource quantity is insufficient for the requested operation",
                )
            }
            Self::Resource(error) => {
                write!(formatter, "resource error: {error}")
            }
            Self::InvalidRequirementReference => {
                formatter.write_str("invalid resource requirement reference")
            }
            Self::UnsupportedOperandCombination => {
                formatter.write_str(
                    "unsupported resource constraint operand combination",
                )
            }
        }
    }
}

impl std::error::Error for ConstraintError {}

impl From<ResourceError> for ConstraintError {
    fn from(error: ResourceError) -> Self {
        Self::Resource(error)
    }
}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), ConstraintError> {
    if value.trim().is_empty() {
        return Err(ConstraintError::EmptyValue { field });
    }

    Ok(())
}

fn validate_scope(scope: &ConstraintScope) -> Result<(), ConstraintError> {
    match scope {
        ConstraintScope::Program => Ok(()),
        ConstraintScope::ResourceGroup(value)
        | ConstraintScope::Region(value)
        | ConstraintScope::OperationGroup(value)
        | ConstraintScope::Custom(value) => {
            validate_non_empty("constraint scope", value)
        }
        ConstraintScope::LogicalQubit(_) => Ok(()),
    }
}

fn validate_operand(
    operand: &ConstraintOperand,
) -> Result<(), ConstraintError> {
    match operand {
        ConstraintOperand::Requirement(_) => Ok(()),
        ConstraintOperand::ResourceKind(_) => Ok(()),
        ConstraintOperand::LogicalQubit(_) => Ok(()),
        ConstraintOperand::Scope(scope) => validate_scope(scope),
        ConstraintOperand::Quantity(_) => Ok(()),
        ConstraintOperand::Unbounded => Ok(()),
        ConstraintOperand::Range(range) => {
            validate_resource_range(*range)
        }
        ConstraintOperand::Capacity(capacity) => {
            validate_capacity(capacity)
        }
        ConstraintOperand::Symbol(value)
        | ConstraintOperand::Custom(value) => {
            validate_non_empty("constraint operand", value)
        }
    }
}

fn validate_resource_range(
    range: ResourceRange,
) -> Result<(), ConstraintError> {
    if let ResourceQuantity::Finite(maximum) = range.max() {
        if range.min() > maximum {
            return Err(ConstraintError::InvalidRange);
        }
    }

    Ok(())
}

fn validate_capacity(
    capacity: &ResourceCapacity,
) -> Result<(), ConstraintError> {
    let _ = capacity;

    // `ResourceCapacity` owns its own semantic invariants. Constraint
    // validation does not duplicate or mutate them.
    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_id_is_stable() {
        let id = ResourceConstraintId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(id.to_string(), "constraint:42");
    }

    #[test]
    fn ratio_rejects_zero_denominator() {
        let result = ResourceRatio::new(1, 0);

        assert_eq!(
            result,
            Err(ConstraintError::ZeroRatioDenominator)
        );
    }

    #[test]
    fn ratio_is_deterministic() {
        let ratio = ResourceRatio::new(2, 3).expect("valid ratio");

        assert_eq!(ratio.numerator(), 2);
        assert_eq!(ratio.denominator(), 3);
        assert!(!ratio.is_zero());
        assert_eq!(ratio.to_string(), "2/3");
    }

    #[test]
    fn finite_comparison_is_correct() {
        let constraint = ResourceConstraint::less_than_or_equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::quantity(10),
            ConstraintOperand::quantity(20),
        );

        assert_eq!(constraint.evaluate_finite(10, 20), Some(true));
        assert_eq!(constraint.evaluate_finite(21, 20), Some(false));
    }

    #[test]
    fn equality_is_correct() {
        let constraint = ResourceConstraint::equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::quantity(10),
            ConstraintOperand::quantity(10),
        );

        assert_eq!(constraint.evaluate_finite(10, 10), Some(true));
        assert_eq!(constraint.evaluate_finite(10, 11), Some(false));
    }

    #[test]
    fn ratio_evaluation_is_exact() {
        let ratio = ResourceRatio::new(1, 2).expect("valid ratio");

        let constraint = ResourceConstraint::new(
            ResourceConstraintId::new(1),
            ConstraintRelation::Ratio,
            ConstraintOperand::quantity(2),
            ConstraintOperand::quantity(4),
        )
        .with_ratio(ratio)
        .expect("ratio is valid");

        assert_eq!(constraint.evaluate_finite(2, 4), Some(true));
        assert_eq!(constraint.evaluate_finite(3, 4), Some(false));
    }

    #[test]
    fn requirement_constraint_is_constructible() {
        let requirement_a = ResourceRequirementId::new(1);
        let requirement_b = ResourceRequirementId::new(2);

        let constraint = ResourceConstraint::requires(
            ResourceConstraintId::new(10),
            requirement_a,
            requirement_b,
        );

        assert_eq!(
            constraint.relation(),
            &ConstraintRelation::Requires
        );

        assert!(constraint.validate().is_ok());
    }

    #[test]
    fn logical_qubit_scope_uses_canonical_qubit_identity() {
        let qubit = QubitId::new(7);

        let constraint = ResourceConstraint::equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::logical_qubit(qubit),
            ConstraintOperand::logical_qubit(qubit),
        )
        .for_logical_qubit(qubit);

        assert_eq!(
            constraint.scope(),
            &ConstraintScope::LogicalQubit(qubit)
        );

        assert!(constraint.validate().is_ok());
    }

    #[test]
    fn duplicate_constraint_ids_are_rejected() {
        let mut constraints = ResourceConstraints::new();

        let first = ResourceConstraint::equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::quantity(1),
            ConstraintOperand::quantity(1),
        );

        let second = ResourceConstraint::equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::quantity(2),
            ConstraintOperand::quantity(2),
        );

        assert!(constraints.push(first).is_ok());

        assert_eq!(
            constraints.push(second),
            Err(ConstraintError::DuplicateConstraintId(
                ResourceConstraintId::new(1)
            ))
        );
    }

    #[test]
    fn insertion_order_is_preserved() {
        let mut constraints = ResourceConstraints::new();

        for id in 1..=3 {
            let constraint = ResourceConstraint::equal(
                ResourceConstraintId::new(id),
                ConstraintOperand::quantity(id),
                ConstraintOperand::quantity(id),
            );

            constraints.push(constraint).expect("unique constraint");
        }

        let ids: Vec<u64> = constraints
            .iter()
            .map(|constraint| constraint.id().value())
            .collect();

        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn custom_scope_rejects_empty_value() {
        assert_eq!(
            ConstraintScope::custom("   "),
            Err(ConstraintError::EmptyValue {
                field: "custom scope"
            })
        );
    }

    #[test]
    fn custom_relation_rejects_empty_value() {
        assert_eq!(
            ConstraintRelation::custom(""),
            Err(ConstraintError::EmptyValue {
                field: "custom relation"
            })
        );
    }

    #[test]
    fn advisory_constraint_can_be_created() {
        let constraint = ResourceConstraint::equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::quantity(1),
            ConstraintOperand::quantity(2),
        )
        .advisory();

        assert!(constraint.strength().is_advisory());
    }

    #[test]
    fn preferred_constraint_can_be_created() {
        let constraint = ResourceConstraint::equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::quantity(1),
            ConstraintOperand::quantity(2),
        )
        .preferred();

        assert!(constraint.strength().is_preferred());
    }

    #[test]
    fn ratio_requires_ratio_payload() {
        let constraint = ResourceConstraint::new(
            ResourceConstraintId::new(1),
            ConstraintRelation::Ratio,
            ConstraintOperand::quantity(1),
            ConstraintOperand::quantity(2),
        );

        assert_eq!(
            constraint.validate(),
            Err(ConstraintError::MissingRatio)
        );
    }

    #[test]
    fn non_ratio_rejects_ratio_payload() {
        let ratio = ResourceRatio::new(1, 2).expect("valid ratio");

        let constraint = ResourceConstraint::equal(
            ResourceConstraintId::new(1),
            ConstraintOperand::quantity(1),
            ConstraintOperand::quantity(2),
        );

        assert_eq!(
            constraint.with_ratio(ratio),
            Err(ConstraintError::RatioOnNonRatioConstraint)
        );
    }
}