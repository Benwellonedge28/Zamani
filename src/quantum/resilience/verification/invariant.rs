//! Zamani Quantum Resilience — Verification Invariants
//!
//! Path:
//!     crate::quantum::resilience::verification::invariant
//!
//! # Purpose
//!
//! This module defines the low-level, provider-independent invariants used by
//! quantum resilience verification.
//!
//! It answers one question:
//!
//! > Has a resilience action preserved the structural properties that must
//! > remain true before an adapted/recovered execution can be trusted?
//!
//! This module deliberately does NOT:
//!
//! - define another `QuantumCircuit`;
//! - define another `QuantumOperation`;
//! - define another `Gate`;
//! - define another `QubitId`;
//! - define another `PhysicalQubitId`;
//! - implement routing;
//! - implement scheduling;
//! - implement optimization;
//! - implement QEC;
//! - implement hardware discovery;
//! - implement backend execution;
//! - define provider-specific rules;
//! - assume a fixed number of qubits;
//! - assume a fixed number of operations;
//! - impose an architectural machine-size limit.
//!
//! Canonical quantum identities remain owned by:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! The canonical IR remains owned by `crate::quantum::ir`.
//!
//! # Architectural role
//!
//! ```text
//! canonical IR / execution result
//!             │
//!             ▼
//!      invariant inputs
//!             │
//!             ▼
//!    ┌──────────────────┐
//!    │   this module    │
//!    │                  │
//!    │ invariant checks │
//!    └────────┬─────────┘
//!             │
//!             ▼
//!       InvariantReport
//!             │
//!             ▼
//! semantic/result/acceptance verification
//! ```
//!
//! This module is intentionally lower-level than `semantic.rs`,
//! `result.rs`, and `acceptance.rs`.
//!
//! Those modules may compose these structural invariants with stronger
//! semantic and result-level verification.
//!
//! # Write once, scale everywhere
//!
//! No invariant in this module contains a fixed hardware size.
//!
//! In particular, this module MUST NOT contain architectural constants such
//! as:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_OPERATIONS
//!     MAX_DEPTH
//!
//! Concrete execution limits belong to:
//!
//! - target capabilities;
//! - execution policies;
//! - resource budgets;
//! - security policies;
//! - host/runtime availability.
//!
//! An empty collection is therefore valid unless a caller explicitly requires
//! at least one resource.
//!
//! # Important semantic boundary
//!
//! Structural identity preservation is not the same thing as proving that a
//! quantum state or probability distribution is correct.
//!
//! This module can establish properties such as:
//!
//! - logical identities are preserved;
//! - identities are unique where uniqueness is required;
//! - required resources are present;
//! - forbidden resources are absent;
//! - mappings are injective where required;
//! - mappings contain no unknown logical resources;
//! - mappings contain no duplicate physical resources;
//! - required resource relationships are preserved;
//! - invariant evaluation is deterministic.
//!
//! It does NOT prove:
//!
//! - amplitudes are correct;
//! - a physical quantum state equals another state;
//! - a noisy result is statistically equivalent;
//! - a decoder is correct;
//! - a circuit transformation is semantically equivalent.
//!
//! Those responsibilities belong to higher verification layers.
//!
//! # Canonical qubit identity
//!
//! All logical qubit identifiers in this module use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! All physical qubit identifiers use:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! No resilience-specific qubit identity is introduced.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//! - standard library only
//!
//! # Integration contract
//!
//! This file intentionally depends only on canonical IR identity types and
//! standard-library facilities.
//!
//! Higher-level modules integrate with it by constructing `InvariantInput`
//! and invoking `InvariantVerifier`.
//!
//! Consequently, changes to routing, scheduling, hardware, QEC, optimization,
//! execution, or backend implementations do not require this file to be
//! modified.
//!
//! The public types in this file are the stable verification vocabulary.
//!
//! ---------------------------------------------------------------------------
//! Safety
//! ---------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

// ---------------------------------------------------------------------------
// Imports
// ---------------------------------------------------------------------------

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Public result type
// ============================================================================

/// Result type returned by invariant verification.
pub type InvariantResult<T> = Result<T, InvariantError>;

// ============================================================================
// Invariant identifier
// ============================================================================

/// Stable identifier for a structural resilience invariant.
///
/// Numeric identifiers are intentionally stable and provider-independent.
/// They are not tied to a particular hardware architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum InvariantId {
    /// The logical resource namespace must not contain duplicates.
    UniqueLogicalQubits = 1,

    /// The physical resource namespace must not contain duplicates.
    UniquePhysicalQubits = 2,

    /// Every logical resource required by a mapping must exist.
    MappingReferencesKnownLogicalQubits = 3,

    /// Every physical resource in a mapping must be unique when an injective
    /// placement is required.
    MappingIsInjective = 4,

    /// A logical-to-physical mapping must preserve its logical domain.
    MappingPreservesLogicalDomain = 5,

    /// A mapping must not contain an unexpected physical target.
    MappingContainsOnlyDeclaredPhysicalQubits = 6,

    /// A required logical resource must not disappear during adaptation.
    RequiredLogicalQubitsPreserved = 7,

    /// A protected logical resource must not disappear during adaptation.
    ProtectedLogicalQubitsPreserved = 8,

    /// A forbidden logical resource must not appear.
    ForbiddenLogicalQubitsAbsent = 9,

    /// A required physical resource must not disappear.
    RequiredPhysicalQubitsPreserved = 10,

    /// A forbidden physical resource must not appear.
    ForbiddenPhysicalQubitsAbsent = 11,

    /// A collection must contain only valid canonical identifiers.
    ValidIdentifiers = 12,

    /// A comparison must preserve cardinality where cardinality preservation
    /// is explicitly required by the caller.
    CardinalityPreserved = 13,

    /// The invariant input must be internally coherent.
    InputCoherent = 14,

    /// A mapping must be deterministic and contain no ambiguous assignment.
    MappingUnambiguous = 15,
}

impl InvariantId {
    /// Returns a stable machine-readable code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::UniqueLogicalQubits => "RES-INV-001",
            Self::UniquePhysicalQubits => "RES-INV-002",
            Self::MappingReferencesKnownLogicalQubits => "RES-INV-003",
            Self::MappingIsInjective => "RES-INV-004",
            Self::MappingPreservesLogicalDomain => "RES-INV-005",
            Self::MappingContainsOnlyDeclaredPhysicalQubits => "RES-INV-006",
            Self::RequiredLogicalQubitsPreserved => "RES-INV-007",
            Self::ProtectedLogicalQubitsPreserved => "RES-INV-008",
            Self::ForbiddenLogicalQubitsAbsent => "RES-INV-009",
            Self::RequiredPhysicalQubitsPreserved => "RES-INV-010",
            Self::ForbiddenPhysicalQubitsAbsent => "RES-INV-011",
            Self::ValidIdentifiers => "RES-INV-012",
            Self::CardinalityPreserved => "RES-INV-013",
            Self::InputCoherent => "RES-INV-014",
            Self::MappingUnambiguous => "RES-INV-015",
        }
    }

    /// Returns the canonical short name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::UniqueLogicalQubits => "unique_logical_qubits",
            Self::UniquePhysicalQubits => "unique_physical_qubits",
            Self::MappingReferencesKnownLogicalQubits => {
                "mapping_references_known_logical_qubits"
            }
            Self::MappingIsInjective => "mapping_is_injective",
            Self::MappingPreservesLogicalDomain => "mapping_preserves_logical_domain",
            Self::MappingContainsOnlyDeclaredPhysicalQubits => {
                "mapping_contains_only_declared_physical_qubits"
            }
            Self::RequiredLogicalQubitsPreserved => "required_logical_qubits_preserved",
            Self::ProtectedLogicalQubitsPreserved => {
                "protected_logical_qubits_preserved"
            }
            Self::ForbiddenLogicalQubitsAbsent => "forbidden_logical_qubits_absent",
            Self::RequiredPhysicalQubitsPreserved => {
                "required_physical_qubits_preserved"
            }
            Self::ForbiddenPhysicalQubitsAbsent => {
                "forbidden_physical_qubits_absent"
            }
            Self::ValidIdentifiers => "valid_identifiers",
            Self::CardinalityPreserved => "cardinality_preserved",
            Self::InputCoherent => "input_coherent",
            Self::MappingUnambiguous => "mapping_unambiguous",
        }
    }
}

impl fmt::Display for InvariantId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.code())
    }
}

// ============================================================================
// Verification severity
// ============================================================================

/// Severity of an invariant violation.
///
/// Severity does not itself decide recovery. That remains a policy decision
/// in the resilience policy/acceptance layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InvariantSeverity {
    /// Informational condition that does not invalidate the result.
    Info,

    /// The result may be usable only under an explicit degraded policy.
    Warning,

    /// The invariant violation prevents normal acceptance.
    Error,

    /// The violation indicates a potentially unsafe semantic/resource state.
    Critical,
}

impl InvariantSeverity {
    /// Returns whether this severity is blocking under the default policy.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Error | Self::Critical)
    }
}

impl fmt::Display for InvariantSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => formatter.write_str("info"),
            Self::Warning => formatter.write_str("warning"),
            Self::Error => formatter.write_str("error"),
            Self::Critical => formatter.write_str("critical"),
        }
    }
}

// ============================================================================
// Verification mode
// ============================================================================

/// Controls which invariant set is evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvariantMode {
    /// Check only the minimum structural safety invariants.
    Basic,

    /// Check structural invariants plus declared resource constraints.
    Standard,

    /// Check all invariants represented by the supplied input.
    Strict,
}

impl Default for InvariantMode {
    fn default() -> Self {
        Self::Standard
    }
}

// ============================================================================
// Verification policy
// ============================================================================

/// Policy controlling structural invariant verification.
///
/// This is deliberately independent from resilience recovery policy.
///
/// It describes *what must be checked*, not *what recovery action should be
/// performed after a failure*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantPolicy {
    mode: InvariantMode,

    require_unique_logical_qubits: bool,
    require_unique_physical_qubits: bool,
    require_injective_mapping: bool,
    require_mapping_domain_preservation: bool,
    require_mapping_physical_domain: bool,
    require_cardinality_preservation: bool,
}

impl Default for InvariantPolicy {
    fn default() -> Self {
        Self {
            mode: InvariantMode::Standard,
            require_unique_logical_qubits: true,
            require_unique_physical_qubits: true,
            require_injective_mapping: true,
            require_mapping_domain_preservation: true,
            require_mapping_physical_domain: true,
            require_cardinality_preservation: false,
        }
    }
}

impl InvariantPolicy {
    /// Creates the default production policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            mode: InvariantMode::Standard,
            require_unique_logical_qubits: true,
            require_unique_physical_qubits: true,
            require_injective_mapping: true,
            require_mapping_domain_preservation: true,
            require_mapping_physical_domain: true,
            require_cardinality_preservation: false,
        }
    }

    /// Returns the verification mode.
    #[must_use]
    pub const fn mode(&self) -> InvariantMode {
        self.mode
    }

    /// Sets the verification mode.
    #[must_use]
    pub const fn with_mode(mut self, mode: InvariantMode) -> Self {
        self.mode = mode;
        self
    }

    /// Enables or disables logical uniqueness checking.
    #[must_use]
    pub const fn with_unique_logical_qubits(mut self, required: bool) -> Self {
        self.require_unique_logical_qubits = required;
        self
    }

    /// Enables or disables physical uniqueness checking.
    #[must_use]
    pub const fn with_unique_physical_qubits(mut self, required: bool) -> Self {
        self.require_unique_physical_qubits = required;
        self
    }

    /// Enables or disables injective mapping checking.
    #[must_use]
    pub const fn with_injective_mapping(mut self, required: bool) -> Self {
        self.require_injective_mapping = required;
        self
    }

    /// Enables or disables logical mapping-domain preservation.
    #[must_use]
    pub const fn with_mapping_domain_preservation(mut self, required: bool) -> Self {
        self.require_mapping_domain_preservation = required;
        self
    }

    /// Enables or disables physical mapping-domain validation.
    #[must_use]
    pub const fn with_physical_domain_validation(mut self, required: bool) -> Self {
        self.require_mapping_physical_domain = required;
        self
    }

    /// Enables or disables cardinality preservation.
    #[must_use]
    pub const fn with_cardinality_preservation(mut self, required: bool) -> Self {
        self.require_cardinality_preservation = required;
        self
    }

    /// Returns whether logical uniqueness is required.
    #[must_use]
    pub const fn requires_unique_logical_qubits(&self) -> bool {
        self.require_unique_logical_qubits
    }

    /// Returns whether physical uniqueness is required.
    #[must_use]
    pub const fn requires_unique_physical_qubits(&self) -> bool {
        self.require_unique_physical_qubits
    }

    /// Returns whether mapping injectivity is required.
    #[must_use]
    pub const fn requires_injective_mapping(&self) -> bool {
        self.require_injective_mapping
    }

    /// Returns whether logical mapping-domain preservation is required.
    #[must_use]
    pub const fn requires_mapping_domain_preservation(&self) -> bool {
        self.require_mapping_domain_preservation
    }

    /// Returns whether physical mapping-domain validation is required.
    #[must_use]
    pub const fn requires_physical_domain_validation(&self) -> bool {
        self.require_mapping_physical_domain
    }

    /// Returns whether cardinality preservation is required.
    #[must_use]
    pub const fn requires_cardinality_preservation(&self) -> bool {
        self.require_cardinality_preservation
    }
}

// ============================================================================
// Resource mapping
// ============================================================================

/// A logical-to-physical resource mapping.
///
/// This is intentionally only a verification value.
///
/// It does NOT implement routing or placement.
///
/// Routing remains owned by `quantum::routing`.
///
/// `BTreeMap` provides deterministic iteration and deterministic verification
/// order without relying on hash-map iteration order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LogicalPhysicalMapping {
    entries: BTreeMap<QubitId, PhysicalQubitId>,
}

impl LogicalPhysicalMapping {
    /// Creates an empty mapping.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a mapping from an iterator.
    ///
    /// Duplicate logical keys are rejected instead of silently replacing an
    /// existing mapping.
    pub fn from_iter<I>(entries: I) -> InvariantResult<Self>
    where
        I: IntoIterator<Item = (QubitId, PhysicalQubitId)>,
    {
        let mut mapping = Self::new();

        for (logical, physical) in entries {
            mapping.insert(logical, physical)?;
        }

        Ok(mapping)
    }

    /// Inserts one logical-to-physical association.
    ///
    /// A logical qubit may have exactly one physical assignment within one
    /// mapping snapshot.
    pub fn insert(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> InvariantResult<()> {
        if self.entries.contains_key(&logical) {
            return Err(InvariantError::DuplicateLogicalMapping { logical });
        }

        self.entries.insert(logical, physical);
        Ok(())
    }

    /// Returns the physical assignment for a logical qubit.
    #[must_use]
    pub fn get(&self, logical: QubitId) -> Option<PhysicalQubitId> {
        self.entries.get(&logical).copied()
    }

    /// Returns the number of mapping entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the mapping is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns deterministic logical/physical entries.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&QubitId, &PhysicalQubitId)> {
        self.entries.iter()
    }

    /// Returns the logical domain.
    pub fn logical_ids(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.entries.keys().copied()
    }

    /// Returns the physical codomain.
    pub fn physical_ids(&self) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.entries.values().copied()
    }
}

// ============================================================================
// Invariant input
// ============================================================================

/// Immutable input to invariant verification.
///
/// Higher-level verification code should construct this value from canonical
/// IR, routing, hardware, QEC, and execution observations.
///
/// This type intentionally contains only data necessary for structural
/// verification.
///
/// It does not own or mutate a `QuantumCircuit`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InvariantInput {
    /// Logical qubits represented by the computation.
    logical_qubits: Vec<QubitId>,

    /// Physical qubits available/declared for the verification context.
    physical_qubits: Vec<PhysicalQubitId>,

    /// Optional logical-to-physical mapping.
    mapping: Option<LogicalPhysicalMapping>,

    /// Logical qubits that must survive adaptation.
    required_logical_qubits: BTreeSet<QubitId>,

    /// Logical qubits whose preservation is explicitly security/correctness
    /// critical.
    protected_logical_qubits: BTreeSet<QubitId>,

    /// Logical qubits that must not occur in the adapted execution context.
    forbidden_logical_qubits: BTreeSet<QubitId>,

    /// Physical qubits that must remain available.
    required_physical_qubits: BTreeSet<PhysicalQubitId>,

    /// Physical qubits that must not be used.
    forbidden_physical_qubits: BTreeSet<PhysicalQubitId>,

    /// Whether the caller explicitly requires the logical resource count to
    /// remain unchanged.
    preserve_logical_cardinality: bool,

    /// Whether the caller explicitly requires the physical resource count to
    /// remain unchanged.
    preserve_physical_cardinality: bool,
}

impl InvariantInput {
    /// Creates an empty invariant input.
    ///
    /// Empty is a valid structural state. Whether a non-empty computation is
    /// required is a higher-level program/execution concern.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the logical qubit collection.
    #[must_use]
    pub fn with_logical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.logical_qubits = qubits.into_iter().collect();
        self
    }

    /// Sets the physical qubit collection.
    #[must_use]
    pub fn with_physical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        self.physical_qubits = qubits.into_iter().collect();
        self
    }

    /// Sets the logical-to-physical mapping.
    #[must_use]
    pub fn with_mapping(mut self, mapping: LogicalPhysicalMapping) -> Self {
        self.mapping = Some(mapping);
        self
    }

    /// Declares logical qubits that must be preserved.
    #[must_use]
    pub fn with_required_logical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.required_logical_qubits = qubits.into_iter().collect();
        self
    }

    /// Declares protected logical qubits.
    #[must_use]
    pub fn with_protected_logical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.protected_logical_qubits = qubits.into_iter().collect();
        self
    }

    /// Declares forbidden logical qubits.
    #[must_use]
    pub fn with_forbidden_logical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.forbidden_logical_qubits = qubits.into_iter().collect();
        self
    }

    /// Declares physical qubits that must be preserved.
    #[must_use]
    pub fn with_required_physical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        self.required_physical_qubits = qubits.into_iter().collect();
        self
    }

    /// Declares physical qubits that are forbidden.
    #[must_use]
    pub fn with_forbidden_physical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        self.forbidden_physical_qubits = qubits.into_iter().collect();
        self
    }

    /// Requires logical cardinality preservation.
    #[must_use]
    pub fn preserve_logical_cardinality(mut self, required: bool) -> Self {
        self.preserve_logical_cardinality = required;
        self
    }

    /// Requires physical cardinality preservation.
    #[must_use]
    pub fn preserve_physical_cardinality(mut self, required: bool) -> Self {
        self.preserve_physical_cardinality = required;
        self
    }

    /// Returns the logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns the physical qubits.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns the mapping, if present.
    #[must_use]
    pub fn mapping(&self) -> Option<&LogicalPhysicalMapping> {
        self.mapping.as_ref()
    }

    /// Returns required logical qubits.
    #[must_use]
    pub fn required_logical_qubits(&self) -> &BTreeSet<QubitId> {
        &self.required_logical_qubits
    }

    /// Returns protected logical qubits.
    #[must_use]
    pub fn protected_logical_qubits(&self) -> &BTreeSet<QubitId> {
        &self.protected_logical_qubits
    }

    /// Returns forbidden logical qubits.
    #[must_use]
    pub fn forbidden_logical_qubits(&self) -> &BTreeSet<QubitId> {
        &self.forbidden_logical_qubits
    }

    /// Returns required physical qubits.
    #[must_use]
    pub fn required_physical_qubits(&self) -> &BTreeSet<PhysicalQubitId> {
        &self.required_physical_qubits
    }

    /// Returns forbidden physical qubits.
    #[must_use]
    pub fn forbidden_physical_qubits(&self) -> &BTreeSet<PhysicalQubitId> {
        &self.forbidden_physical_qubits
    }

    /// Returns whether logical cardinality must be preserved.
    #[must_use]
    pub const fn preserves_logical_cardinality(&self) -> bool {
        self.preserve_logical_cardinality
    }

    /// Returns whether physical cardinality must be preserved.
    #[must_use]
    pub const fn preserves_physical_cardinality(&self) -> bool {
        self.preserve_physical_cardinality
    }
}

// ============================================================================
// Violation location
// ============================================================================

/// Location associated with an invariant violation.
///
/// These are resilience verification locations, not replacements for canonical
/// IR or hardware resource identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InvariantLocation {
    /// No narrower location is applicable.
    Global,

    /// A logical qubit is implicated.
    LogicalQubit(QubitId),

    /// A physical qubit is implicated.
    PhysicalQubit(PhysicalQubitId),

    /// A logical/physical mapping entry is implicated.
    Mapping {
        /// Logical identity.
        logical: QubitId,

        /// Physical identity.
        physical: PhysicalQubitId,
    },
}

impl fmt::Display for InvariantLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => formatter.write_str("global"),
            Self::LogicalQubit(id) => write!(formatter, "logical:{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical:{id}"),
            Self::Mapping { logical, physical } => {
                write!(formatter, "mapping:{logical}->{physical}")
            }
        }
    }
}

// ============================================================================
// Violation
// ============================================================================

/// One deterministic invariant violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantViolation {
    invariant: InvariantId,
    severity: InvariantSeverity,
    location: InvariantLocation,
    message: String,
}

impl InvariantViolation {
    fn new(
        invariant: InvariantId,
        severity: InvariantSeverity,
        location: InvariantLocation,
        message: impl Into<String>,
    ) -> Self {
        Self {
            invariant,
            severity,
            location,
            message: message.into(),
        }
    }

    /// Returns the invariant identifier.
    #[must_use]
    pub const fn invariant(&self) -> InvariantId {
        self.invariant
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> InvariantSeverity {
        self.severity
    }

    /// Returns the location.
    #[must_use]
    pub const fn location(&self) -> InvariantLocation {
        self.location
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the stable machine-readable invariant code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.invariant.code()
    }
}

// ============================================================================
// Verification report
// ============================================================================

/// Complete deterministic invariant verification report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantReport {
    mode: InvariantMode,
    checked: Vec<InvariantId>,
    violations: Vec<InvariantViolation>,
}

impl InvariantReport {
    fn new(mode: InvariantMode) -> Self {
        Self {
            mode,
            checked: Vec::new(),
            violations: Vec::new(),
        }
    }

    fn check(&mut self, invariant: InvariantId) {
        if !self.checked.contains(&invariant) {
            self.checked.push(invariant);
        }
    }

    fn violation(
        &mut self,
        invariant: InvariantId,
        severity: InvariantSeverity,
        location: InvariantLocation,
        message: impl Into<String>,
    ) {
        self.violations.push(InvariantViolation::new(
            invariant,
            severity,
            location,
            message,
        ));
    }

    /// Returns the verification mode.
    #[must_use]
    pub const fn mode(&self) -> InvariantMode {
        self.mode
    }

    /// Returns the checked invariant identifiers.
    #[must_use]
    pub fn checked(&self) -> &[InvariantId] {
        &self.checked
    }

    /// Returns all violations in deterministic evaluation order.
    #[must_use]
    pub fn violations(&self) -> &[InvariantViolation] {
        &self.violations
    }

    /// Returns whether every checked invariant passed.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }

    /// Returns whether at least one blocking violation exists.
    #[must_use]
    pub fn is_blocking(&self) -> bool {
        self.violations
            .iter()
            .any(|violation| violation.severity().is_blocking())
    }

    /// Returns the number of violations.
    #[must_use]
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    /// Returns the number of checked invariants.
    #[must_use]
    pub fn checked_count(&self) -> usize {
        self.checked.len()
    }

    /// Returns the highest severity observed.
    #[must_use]
    pub fn maximum_severity(&self) -> Option<InvariantSeverity> {
        self.violations
            .iter()
            .map(InvariantViolation::severity)
            .max()
    }
}

// ============================================================================
// Verification errors
// ============================================================================

/// Errors preventing invariant verification from being meaningfully
/// performed.
///
/// An invariant violation is represented inside `InvariantReport` rather than
/// as this error type.
///
/// This distinction is important:
//!
//! - `Ok(report)` means verification completed;
//! - `report.is_valid()` tells whether invariants passed;
//! - `Err(error)` means verification itself could not be completed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvariantError {
    /// The same logical qubit appeared more than once where uniqueness was
    /// required.
    DuplicateLogicalQubit {
        /// Duplicated canonical logical identity.
        qubit: QubitId,
    },

    /// The same physical qubit appeared more than once where uniqueness was
    /// required.
    DuplicatePhysicalQubit {
        /// Duplicated canonical physical identity.
        qubit: PhysicalQubitId,
    },

    /// A logical qubit received multiple physical mappings.
    DuplicateLogicalMapping {
        /// Canonical logical identity.
        logical: QubitId,
    },

    /// Input declares a required and forbidden resource simultaneously.
    ContradictoryLogicalRequirement {
        /// Contradictory logical identity.
        qubit: QubitId,
    },

    /// Input declares a required and forbidden physical resource
    /// simultaneously.
    ContradictoryPhysicalRequirement {
        /// Contradictory physical identity.
        qubit: PhysicalQubitId,
    },

    /// Verification input is structurally incoherent.
    IncoherentInput {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for InvariantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateLogicalQubit { qubit } => {
                write!(formatter, "duplicate logical qubit: {qubit}")
            }
            Self::DuplicatePhysicalQubit { qubit } => {
                write!(formatter, "duplicate physical qubit: {qubit}")
            }
            Self::DuplicateLogicalMapping { logical } => {
                write!(formatter, "duplicate logical mapping for {logical}")
            }
            Self::ContradictoryLogicalRequirement { qubit } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is both required/protected and forbidden"
                )
            }
            Self::ContradictoryPhysicalRequirement { qubit } => {
                write!(
                    formatter,
                    "physical qubit {qubit} is both required and forbidden"
                )
            }
            Self::IncoherentInput { message } => {
                write!(formatter, "incoherent invariant input: {message}")
            }
        }
    }
}

impl std::error::Error for InvariantError {}

// ============================================================================
// Invariant verifier
// ============================================================================

/// Stateless production invariant verifier.
///
/// The verifier owns no mutable global state and therefore scales across
/// independent executions, devices, processes, and distributed workers.
///
/// It is intentionally cheap to construct:
///
/// ```text
/// let verifier = InvariantVerifier::production();
/// ```
///
/// Verification state exists only in the returned `InvariantReport`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantVerifier {
    policy: InvariantPolicy,
}

impl Default for InvariantVerifier {
    fn default() -> Self {
        Self::production()
    }
}

impl InvariantVerifier {
    /// Creates a verifier using the production policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            policy: InvariantPolicy::production(),
        }
    }

    /// Creates a verifier using a caller-supplied policy.
    #[must_use]
    pub const fn with_policy(policy: InvariantPolicy) -> Self {
        Self { policy }
    }

    /// Returns the policy.
    #[must_use]
    pub const fn policy(&self) -> &InvariantPolicy {
        &self.policy
    }

    /// Verifies the supplied structural invariants.
    ///
    /// This method never mutates the input.
    ///
    /// A structurally invalid input returns a report containing violations.
    /// An `Err` is reserved for an input that cannot be meaningfully
    /// interpreted as an invariant context.
    pub fn verify(&self, input: &InvariantInput) -> InvariantResult<InvariantReport> {
        let mut report = InvariantReport::new(self.policy.mode());

        self.check_input_coherence(input, &mut report)?;

        self.check_logical_uniqueness(input, &mut report);
        self.check_physical_uniqueness(input, &mut report);
        self.check_required_logical_qubits(input, &mut report);
        self.check_protected_logical_qubits(input, &mut report);
        self.check_forbidden_logical_qubits(input, &mut report);
        self.check_required_physical_qubits(input, &mut report);
        self.check_forbidden_physical_qubits(input, &mut report);

        if let Some(mapping) = input.mapping() {
            self.check_mapping(input, mapping, &mut report);
        }

        self.check_cardinality(input, &mut report);

        Ok(report)
    }

    // ------------------------------------------------------------------------
    // Input coherence
    // ------------------------------------------------------------------------

    fn check_input_coherence(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) -> InvariantResult<()> {
        report.check(InvariantId::InputCoherent);

        for qubit in input.protected_logical_qubits() {
            if input.forbidden_logical_qubits().contains(qubit) {
                return Err(InvariantError::ContradictoryLogicalRequirement {
                    qubit: *qubit,
                });
            }
        }

        for qubit in input.required_logical_qubits() {
            if input.forbidden_logical_qubits().contains(qubit) {
                return Err(InvariantError::ContradictoryLogicalRequirement {
                    qubit: *qubit,
                });
            }
        }

        for qubit in input.required_physical_qubits() {
            if input.forbidden_physical_qubits().contains(qubit) {
                return Err(InvariantError::ContradictoryPhysicalRequirement {
                    qubit: *qubit,
                });
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Logical uniqueness
    // ------------------------------------------------------------------------

    fn check_logical_uniqueness(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        if !self.policy.requires_unique_logical_qubits() {
            return;
        }

        report.check(InvariantId::UniqueLogicalQubits);

        let mut seen = BTreeSet::new();

        for qubit in input.logical_qubits() {
            if !seen.insert(*qubit) {
                report.violation(
                    InvariantId::UniqueLogicalQubits,
                    InvariantSeverity::Critical,
                    InvariantLocation::LogicalQubit(*qubit),
                    format!("logical qubit {qubit} occurs more than once"),
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Physical uniqueness
    // ------------------------------------------------------------------------

    fn check_physical_uniqueness(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        if !self.policy.requires_unique_physical_qubits() {
            return;
        }

        report.check(InvariantId::UniquePhysicalQubits);

        let mut seen = BTreeSet::new();

        for qubit in input.physical_qubits() {
            if !seen.insert(*qubit) {
                report.violation(
                    InvariantId::UniquePhysicalQubits,
                    InvariantSeverity::Critical,
                    InvariantLocation::PhysicalQubit(*qubit),
                    format!("physical qubit {qubit} occurs more than once"),
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Required logical resources
    // ------------------------------------------------------------------------

    fn check_required_logical_qubits(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        report.check(InvariantId::RequiredLogicalQubitsPreserved);

        let actual: BTreeSet<QubitId> =
            input.logical_qubits().iter().copied().collect();

        for qubit in input.required_logical_qubits() {
            if !actual.contains(qubit) {
                report.violation(
                    InvariantId::RequiredLogicalQubitsPreserved,
                    InvariantSeverity::Critical,
                    InvariantLocation::LogicalQubit(*qubit),
                    format!("required logical qubit {qubit} is absent"),
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Protected logical resources
    // ------------------------------------------------------------------------

    fn check_protected_logical_qubits(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        report.check(InvariantId::ProtectedLogicalQubitsPreserved);

        let actual: BTreeSet<QubitId> =
            input.logical_qubits().iter().copied().collect();

        for qubit in input.protected_logical_qubits() {
            if !actual.contains(qubit) {
                report.violation(
                    InvariantId::ProtectedLogicalQubitsPreserved,
                    InvariantSeverity::Critical,
                    InvariantLocation::LogicalQubit(*qubit),
                    format!("protected logical qubit {qubit} is absent"),
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Forbidden logical resources
    // ------------------------------------------------------------------------

    fn check_forbidden_logical_qubits(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        report.check(InvariantId::ForbiddenLogicalQubitsAbsent);

        let actual: BTreeSet<QubitId> =
            input.logical_qubits().iter().copied().collect();

        for qubit in input.forbidden_logical_qubits() {
            if actual.contains(qubit) {
                report.violation(
                    InvariantId::ForbiddenLogicalQubitsAbsent,
                    InvariantSeverity::Critical,
                    InvariantLocation::LogicalQubit(*qubit),
                    format!("forbidden logical qubit {qubit} is present"),
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Required physical resources
    // ------------------------------------------------------------------------

    fn check_required_physical_qubits(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        report.check(InvariantId::RequiredPhysicalQubitsPreserved);

        let actual: BTreeSet<PhysicalQubitId> =
            input.physical_qubits().iter().copied().collect();

        for qubit in input.required_physical_qubits() {
            if !actual.contains(qubit) {
                report.violation(
                    InvariantId::RequiredPhysicalQubitsPreserved,
                    InvariantSeverity::Critical,
                    InvariantLocation::PhysicalQubit(*qubit),
                    format!("required physical qubit {qubit} is absent"),
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Forbidden physical resources
    // ------------------------------------------------------------------------

    fn check_forbidden_physical_qubits(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        report.check(InvariantId::ForbiddenPhysicalQubitsAbsent);

        let actual: BTreeSet<PhysicalQubitId> =
            input.physical_qubits().iter().copied().collect();

        for qubit in input.forbidden_physical_qubits() {
            if actual.contains(qubit) {
                report.violation(
                    InvariantId::ForbiddenPhysicalQubitsAbsent,
                    InvariantSeverity::Critical,
                    InvariantLocation::PhysicalQubit(*qubit),
                    format!("forbidden physical qubit {qubit} is present"),
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // Mapping
    // ------------------------------------------------------------------------

    fn check_mapping(
        &self,
        input: &InvariantInput,
        mapping: &LogicalPhysicalMapping,
        report: &mut InvariantReport,
    ) {
        report.check(InvariantId::MappingUnambiguous);

        let logical_domain: BTreeSet<QubitId> =
            input.logical_qubits().iter().copied().collect();

        let physical_domain: BTreeSet<PhysicalQubitId> =
            input.physical_qubits().iter().copied().collect();

        // --------------------------------------------------------------------
        // Logical domain
        // --------------------------------------------------------------------

        if self.policy.requires_mapping_domain_preservation() {
            report.check(InvariantId::MappingReferencesKnownLogicalQubits);
            report.check(InvariantId::MappingPreservesLogicalDomain);

            for (logical, physical) in mapping.iter() {
                if !logical_domain.contains(logical) {
                    report.violation(
                        InvariantId::MappingReferencesKnownLogicalQubits,
                        InvariantSeverity::Critical,
                        InvariantLocation::Mapping {
                            logical: *logical,
                            physical: *physical,
                        },
                        format!(
                            "mapping references logical qubit {logical}, \
                             which is absent from the declared logical domain"
                        ),
                    );
                }
            }

            if mapping.len() != logical_domain.len() {
                report.violation(
                    InvariantId::MappingPreservesLogicalDomain,
                    InvariantSeverity::Error,
                    InvariantLocation::Global,
                    format!(
                        "mapping contains {} logical entries but the logical \
                         domain contains {} resources",
                        mapping.len(),
                        logical_domain.len()
                    ),
                );
            }
        }

        // --------------------------------------------------------------------
        // Physical domain
        // --------------------------------------------------------------------

        if self.policy.requires_physical_domain_validation() {
            report.check(InvariantId::MappingContainsOnlyDeclaredPhysicalQubits);

            for (logical, physical) in mapping.iter() {
                if !physical_domain.contains(physical) {
                    report.violation(
                        InvariantId::MappingContainsOnlyDeclaredPhysicalQubits,
                        InvariantSeverity::Critical,
                        InvariantLocation::Mapping {
                            logical: *logical,
                            physical: *physical,
                        },
                        format!(
                            "mapping assigns logical qubit {logical} to \
                             undeclared physical qubit {physical}"
                        ),
                    );
                }
            }
        }

        // --------------------------------------------------------------------
        // Injectivity
        // --------------------------------------------------------------------

        if self.policy.requires_injective_mapping() {
            report.check(InvariantId::MappingIsInjective);

            let mut physical_to_logical: BTreeMap<
                PhysicalQubitId,
                QubitId,
            > = BTreeMap::new();

            for (logical, physical) in mapping.iter() {
                if let Some(previous_logical) =
                    physical_to_logical.insert(*physical, *logical)
                {
                    report.violation(
                        InvariantId::MappingIsInjective,
                        InvariantSeverity::Critical,
                        InvariantLocation::Mapping {
                            logical: *logical,
                            physical: *physical,
                        },
                        format!(
                            "physical qubit {physical} is assigned to both \
                             logical qubits {previous_logical} and {logical}"
                        ),
                    );
                }
            }
        }

        // --------------------------------------------------------------------
        // Required mappings
        // --------------------------------------------------------------------

        for logical in input.required_logical_qubits() {
            if let Some(physical) = mapping.get(*logical) {
                if input.forbidden_physical_qubits().contains(&physical) {
                    report.violation(
                        InvariantId::ForbiddenPhysicalQubitsAbsent,
                        InvariantSeverity::Critical,
                        InvariantLocation::Mapping {
                            logical: *logical,
                            physical,
                        },
                        format!(
                            "required logical qubit {logical} maps to \
                             forbidden physical qubit {physical}"
                        ),
                    );
                }
            }
        }

        // --------------------------------------------------------------------
        // Protected mappings
        // --------------------------------------------------------------------

        for logical in input.protected_logical_qubits() {
            if let Some(physical) = mapping.get(*logical) {
                if input.forbidden_physical_qubits().contains(&physical) {
                    report.violation(
                        InvariantId::ForbiddenPhysicalQubitsAbsent,
                        InvariantSeverity::Critical,
                        InvariantLocation::Mapping {
                            logical: *logical,
                            physical,
                        },
                        format!(
                            "protected logical qubit {logical} maps to \
                             forbidden physical qubit {physical}"
                        ),
                    );
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // Cardinality
    // ------------------------------------------------------------------------

    fn check_cardinality(
        &self,
        input: &InvariantInput,
        report: &mut InvariantReport,
    ) {
        if self.policy.requires_cardinality_preservation()
            || input.preserves_logical_cardinality()
        {
            report.check(InvariantId::CardinalityPreserved);

            if let Some(mapping) = input.mapping() {
                if mapping.len() != input.logical_qubits().len() {
                    report.violation(
                        InvariantId::CardinalityPreserved,
                        InvariantSeverity::Error,
                        InvariantLocation::Global,
                        format!(
                            "logical cardinality is {} while mapping \
                             cardinality is {}",
                            input.logical_qubits().len(),
                            mapping.len()
                        ),
                    );
                }
            }
        }

        if input.preserves_physical_cardinality() {
            report.check(InvariantId::CardinalityPreserved);

            if let Some(mapping) = input.mapping() {
                let physical_count = mapping
                    .physical_ids()
                    .collect::<BTreeSet<_>>()
                    .len();

                if physical_count != input.physical_qubits().len() {
                    report.violation(
                        InvariantId::CardinalityPreserved,
                        InvariantSeverity::Error,
                        InvariantLocation::Global,
                        format!(
                            "physical cardinality is {} while mapping \
                             uses {} distinct physical resources",
                            input.physical_qubits().len(),
                            physical_count
                        ),
                    );
                }
            }
        }
    }
}

// ============================================================================
// Convenience API
// ============================================================================

/// Verifies invariants using the production policy.
///
/// This is the preferred simple entry point for callers that do not require
/// a custom verification policy.
pub fn verify(input: &InvariantInput) -> InvariantResult<InvariantReport> {
    InvariantVerifier::production().verify(input)
}

/// Verifies invariants with an explicit policy.
pub fn verify_with_policy(
    input: &InvariantInput,
    policy: &InvariantPolicy,
) -> InvariantResult<InvariantReport> {
    InvariantVerifier::with_policy(policy.clone()).verify(input)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn physical(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn empty_input_is_structurally_valid() {
        let input = InvariantInput::new();

        let report = verify(&input).expect("verification must complete");

        assert!(report.is_valid());
        assert!(report.violations().is_empty());
    }

    #[test]
    fn logical_qubits_must_be_unique() {
        let input = InvariantInput::new()
            .with_logical_qubits([logical(0), logical(1), logical(0)]);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant() == InvariantId::UniqueLogicalQubits
                && violation.location()
                    == InvariantLocation::LogicalQubit(logical(0))
        }));
    }

    #[test]
    fn physical_qubits_must_be_unique() {
        let input = InvariantInput::new()
            .with_physical_qubits([physical(0), physical(1), physical(0)]);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant() == InvariantId::UniquePhysicalQubits
        }));
    }

    #[test]
    fn required_logical_qubit_must_be_present() {
        let input = InvariantInput::new()
            .with_logical_qubits([logical(0)])
            .with_required_logical_qubits([logical(0), logical(1)]);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant()
                == InvariantId::RequiredLogicalQubitsPreserved
                && violation.location()
                    == InvariantLocation::LogicalQubit(logical(1))
        }));
    }

    #[test]
    fn protected_logical_qubit_must_be_present() {
        let input = InvariantInput::new()
            .with_logical_qubits([logical(0)])
            .with_protected_logical_qubits([logical(1)]);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant()
                == InvariantId::ProtectedLogicalQubitsPreserved
        }));
    }

    #[test]
    fn forbidden_logical_qubit_must_be_absent() {
        let input = InvariantInput::new()
            .with_logical_qubits([logical(0), logical(1)])
            .with_forbidden_logical_qubits([logical(1)]);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant()
                == InvariantId::ForbiddenLogicalQubitsAbsent
        }));
    }

    #[test]
    fn valid_mapping_passes() {
        let mapping = LogicalPhysicalMapping::from_iter([
            (logical(0), physical(7)),
            (logical(1), physical(2)),
        ])
        .expect("mapping must be constructible");

        let input = InvariantInput::new()
            .with_logical_qubits([logical(0), logical(1)])
            .with_physical_qubits([physical(2), physical(7)])
            .with_mapping(mapping);

        let report = verify(&input).expect("verification must complete");

        assert!(report.is_valid(), "{:?}", report.violations());
    }

    #[test]
    fn mapping_cannot_reference_unknown_logical_qubit() {
        let mapping =
            LogicalPhysicalMapping::from_iter([(logical(99), physical(1))])
                .expect("mapping must be constructible");

        let input = InvariantInput::new()
            .with_logical_qubits([logical(0)])
            .with_physical_qubits([physical(1)])
            .with_mapping(mapping);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant()
                == InvariantId::MappingReferencesKnownLogicalQubits
        }));
    }

    #[test]
    fn mapping_cannot_reference_unknown_physical_qubit() {
        let mapping =
            LogicalPhysicalMapping::from_iter([(logical(0), physical(99))])
                .expect("mapping must be constructible");

        let input = InvariantInput::new()
            .with_logical_qubits([logical(0)])
            .with_physical_qubits([physical(1)])
            .with_mapping(mapping);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant()
                == InvariantId::MappingContainsOnlyDeclaredPhysicalQubits
        }));
    }

    #[test]
    fn mapping_must_be_injective_by_default() {
        let mapping = LogicalPhysicalMapping::from_iter([
            (logical(0), physical(1)),
            (logical(1), physical(1)),
        ])
        .expect("mapping must be constructible");

        let input = InvariantInput::new()
            .with_logical_qubits([logical(0), logical(1)])
            .with_physical_qubits([physical(1)])
            .with_mapping(mapping);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant() == InvariantId::MappingIsInjective
        }));
    }

    #[test]
    fn duplicate_logical_mapping_is_rejected_at_construction() {
        let result = LogicalPhysicalMapping::from_iter([
            (logical(0), physical(1)),
            (logical(0), physical(2)),
        ]);

        assert!(matches!(
            result,
            Err(InvariantError::DuplicateLogicalMapping {
                logical
            }) if logical == logical(0)
        ));
    }

    #[test]
    fn forbidden_physical_resource_is_rejected() {
        let input = InvariantInput::new()
            .with_physical_qubits([physical(0), physical(1)])
            .with_forbidden_physical_qubits([physical(1)]);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant()
                == InvariantId::ForbiddenPhysicalQubitsAbsent
        }));
    }

    #[test]
    fn required_physical_resource_must_be_present() {
        let input = InvariantInput::new()
            .with_physical_qubits([physical(0)])
            .with_required_physical_qubits([physical(0), physical(1)]);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant()
                == InvariantId::RequiredPhysicalQubitsPreserved
        }));
    }

    #[test]
    fn contradictory_logical_requirements_are_rejected() {
        let input = InvariantInput::new()
            .with_required_logical_qubits([logical(1)])
            .with_forbidden_logical_qubits([logical(1)]);

        let result = verify(&input);

        assert!(matches!(
            result,
            Err(InvariantError::ContradictoryLogicalRequirement {
                qubit
            }) if qubit == logical(1)
        ));
    }

    #[test]
    fn contradictory_physical_requirements_are_rejected() {
        let input = InvariantInput::new()
            .with_required_physical_qubits([physical(1)])
            .with_forbidden_physical_qubits([physical(1)]);

        let result = verify(&input);

        assert!(matches!(
            result,
            Err(InvariantError::ContradictoryPhysicalRequirement {
                qubit
            }) if qubit == physical(1)
        ));
    }

    #[test]
    fn cardinality_can_be_required_explicitly() {
        let mapping =
            LogicalPhysicalMapping::from_iter([(logical(0), physical(0))])
                .expect("mapping must be constructible");

        let input = InvariantInput::new()
            .with_logical_qubits([logical(0), logical(1)])
            .with_physical_qubits([physical(0)])
            .with_mapping(mapping)
            .preserve_logical_cardinality(true);

        let report = verify(&input).expect("verification must complete");

        assert!(!report.is_valid());

        assert!(report.violations().iter().any(|violation| {
            violation.invariant() == InvariantId::CardinalityPreserved
        }));
    }

    #[test]
    fn deterministic_verification_produces_equal_reports() {
        let mapping = LogicalPhysicalMapping::from_iter([
            (logical(0), physical(4)),
            (logical(1), physical(7)),
        ])
        .expect("mapping must be constructible");

        let input = InvariantInput::new()
            .with_logical_qubits([logical(0), logical(1)])
            .with_physical_qubits([physical(4), physical(7)])
            .with_mapping(mapping);

        let first = verify(&input).expect("verification must complete");
        let second = verify(&input).expect("verification must complete");

        assert_eq!(first, second);
    }

    #[test]
    fn custom_policy_can_disable_injective_mapping() {
        let mapping = LogicalPhysicalMapping::from_iter([
            (logical(0), physical(1)),
            (logical(1), physical(1)),
        ])
        .expect("mapping must be constructible");

        let input = InvariantInput::new()
            .with_logical_qubits([logical(0), logical(1)])
            .with_physical_qubits([physical(1)])
            .with_mapping(mapping);

        let policy =
            InvariantPolicy::production().with_injective_mapping(false);

        let report =
            verify_with_policy(&input, &policy).expect("verification must complete");

        assert!(!report
            .violations()
            .iter()
            .any(|violation| {
                violation.invariant() == InvariantId::MappingIsInjective
            }));
    }

    #[test]
    fn mapping_iteration_is_deterministic() {
        let mapping = LogicalPhysicalMapping::from_iter([
            (logical(9), physical(2)),
            (logical(1), physical(8)),
            (logical(4), physical(3)),
        ])
        .expect("mapping must be constructible");

        let values: Vec<QubitId> = mapping.logical_ids().collect();

        assert_eq!(
            values,
            vec![logical(1), logical(4), logical(9)]
        );
    }
}