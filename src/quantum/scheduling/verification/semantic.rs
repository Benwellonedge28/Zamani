//! Zamani Quantum Scheduling — Semantic Verification
//!
//! Path:
//!     src/quantum/scheduling/verification/semantic.rs
//!
//! # Purpose
//!
//! This module verifies the most important invariant of the scheduling
//! subsystem:
//!
//! > Scheduling may change WHEN and, after routing, WHERE an operation is
//! > executed, but it must not change WHAT computation the program represents.
//!
//! Semantic verification therefore compares a source semantic representation
//! with the semantic representation carried by the scheduled program.
//!
//! The verifier is deliberately independent of:
//!
//! - scheduling algorithms;
//! - ASAP/ALAP/list/critical-path scheduling;
//! - resource calendars;
//! - hardware providers;
//! - hardware topology;
//! - routing implementation;
//! - QEC implementation;
//! - timing representation;
//! - pulse representation;
//! - serialization formats;
//! - runtime execution;
//! - vendor SDKs.
//!
//! Those concerns are represented through stable semantic views/adapters.
//!
//! # Architectural position
//!
//! ```text
//!                         canonical quantum::ir
//!                                  │
//!                                  ▼
//!                         semantic source view
//!                                  │
//!                                  │
//!                     ┌────────────┴────────────┐
//!                     │                         │
//!                     ▼                         ▼
//!             scheduling::adapters::ir    scheduled semantic view
//!                     │                         │
//!                     └────────────┬────────────┘
//!                                  ▼
//!                   verification::semantic
//!                                  │
//!                    ┌─────────────┴──────────────┐
//!                    ▼                            ▼
//!               valid semantics              diagnostics
//!                    │
//!                    ▼
//!              other verification
//!
//! dependency.rs  resource.rs  timing.rs  structural.rs
//! ```
//!
//! # What semantic verification means
//!
//! Semantic verification checks properties that must survive scheduling.
//!
//! Depending on the supplied semantic view, this includes:
//!
//! - operation identity;
//! - operation kind;
//! - operation arity;
//! - logical qubit operands;
//! - classical operands;
//! - classical results;
//! - control conditions;
//! - measurement semantics;
//! - reset semantics;
//! - parameter values;
//! - symbolic parameters;
//! - operation attributes that affect computation;
//! - semantic ordering barriers;
//! - explicitly declared semantic dependencies;
//! - source/scheduled operation multiplicity;
//! - preservation of operation provenance.
//!
//! It deliberately does NOT require:
//!
//! - identical physical qubits;
//! - identical resource IDs;
//! - identical start times;
//! - identical durations;
//! - identical resource reservations;
//! - identical routing;
//! - identical scheduling policy;
//! - identical parallelism.
//!
//! Those are verified by other modules.
//!
//! # Critical routing distinction
//!
//! A routed program may transform:
//!
//! ```text
//! logical q0 -> physical q17
//! logical q1 -> physical q23
//! ```
//!
//! without changing the semantic operation:
//!
//! ```text
//! CNOT(q0, q1)
//! ```
//!
//! Therefore this verifier compares canonical logical qubit identity for
//! semantic preservation. Physical placement belongs to routing/resource
//! verification.
//!
//! Canonical identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No scheduler-specific qubit identity is defined here.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may be specialized for targets with
//! radically different:
//!
//! - qubit counts;
//! - topology;
//! - gate sets;
//! - timing;
//! - resource capacities;
//! - control systems;
//! - QEC implementations;
//! - communication systems;
//! - physical technologies.
//!
//! Semantic verification must therefore verify the program's computation,
//! rather than assuming anything about a particular machine.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum operation arity;
//! - maximum classical operand count;
//! - maximum parameter count;
//! - fixed gate set;
//! - fixed topology;
//! - fixed resource count.
//!
//! # Scalability
//!
//! Let:
//!
//! - `N` = number of semantic operations;
//! - `Q` = total logical operand references;
//! - `P` = total parameter references;
//! - `C` = total classical references;
//! - `D` = total semantic dependency references.
//!
//! Verification is designed around deterministic indexed comparison rather
//! than an all-pairs comparison.
//!
//! Target complexity is:
//!
//! ```text
//! O(N + Q + P + C + D)
//! ```
//!
//! apart from collection lookup costs imposed by caller-provided views.
//!
//! Memory usage is:
//!
//! ```text
//! O(N)
//! ```
//!
//! for the verifier's identity indexes, plus the semantic input itself.
//!
//! The implementation never creates:
//!
//! ```text
//! operations × operations
//! ```
//!
//! comparison matrices.
//!
//! It also never creates a timeline proportional to:
//!
//! ```text
//! qubits × execution_time
//! ```
//!
//! Semantic verification is therefore independent of schedule duration.
//!
//! # Determinism
//!
//! Diagnostics are emitted in deterministic source-order/index order.
//!
//! The verifier never relies on:
//!
//! - hash-map iteration order;
//! - pointer addresses;
//! - wall-clock time;
//! - process IDs;
//! - thread scheduling;
//! - hidden randomness.
//!
//! When a caller supplies operations in canonical deterministic order, the
//! resulting diagnostics are deterministic.
//!
//! # Empty programs
//!
//! An empty program is semantically valid when both source and scheduled views
//! are empty.
//!
//! The verifier does not impose a minimum program size.
//!
//! # Zero-duration operations
//!
//! Duration is not part of semantic equivalence. Zero-duration operations are
//! therefore neither accepted nor rejected by this module because timing
//! verification owns that concern.
//!
//! # Classical/dynamic operations
//!
//! Dynamic circuits are supported through semantic conditions and classical
//! dependencies exposed by the semantic view.
//!
//! The verifier does not assume that all computation is a static quantum DAG.
//!
//! For example:
//!
//! ```text
//! measure(q0) -> c0
//! if c0 == 1:
//!     X(q1)
//! ```
//!
//! remains semantically meaningful even though the final execution timing may
//! only be resolved dynamically.
//!
//! # QEC
//!
//! QEC operations can be verified when their semantic views are supplied.
//!
//! This module does not implement:
//!
//! - stabilizer extraction;
//! - syndrome decoding;
//! - surface-code semantics;
//! - recovery algorithms.
//!
//! It verifies preservation of whatever canonical semantic representation the
//! QEC adapter exposes.
//!
//! # Distributed quantum computing
//!
//! Distributed operations are supported as ordinary semantic operations.
//!
//! For example:
//!
//! ```text
//! entangle(q0, remote_q1)
//! teleport(q0, remote_q1)
//! remote_gate(q0, q1)
//! ```
//!
//! Their physical communication details are not semantic equivalence concerns
//! of this module unless explicitly represented as semantic operations by the
//! canonical IR.
//!
//! # Canonical operation identity
//!
//! Operation identity comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! The verifier does not create another operation identity type.
//!
//! An operation identity is useful for provenance, but semantic equivalence is
//! not established merely because IDs match. The actual semantic fingerprint
//! must also match.
//!
//! This protects against a corrupted or incorrectly adapted schedule that
//! preserves IDs while changing operands or operation meaning.
//!
//! # Floating-point parameters
//!
//! Quantum parameters may be represented using floating-point values by the
//! upstream IR. Exact bitwise comparison is intentionally not imposed by this
//! module because parameter semantics can be represented by different valid
//! canonical forms.
//!
//! Instead, callers supply canonical parameter fingerprints through
//! `SemanticParameter`.
//!
//! The recommended adapter rule is:
//!
//! ```text
//! canonical IR expression
//!          │
//!          ▼
//! canonical semantic fingerprint
//!          │
//!          ▼
//! semantic verifier
//! ```
//!
//! The verifier never invents numerical tolerances for quantum semantics.
//!
//! If approximate equivalence is desired, that is a separate explicitly
//! configured semantic-equivalence layer and must not silently weaken exact
//! verification.
//!
//! # Semantic fingerprint
//!
//! `SemanticFingerprint` is an immutable value describing the computation
//! represented by one operation.
//!
//! It intentionally does not contain scheduling data.
//!
//! It may contain:
//!
//! - operation identity;
//! - operation kind;
//! - logical qubits;
//! - classical inputs;
//! - classical outputs;
//! - control condition;
//! - parameters;
//! - semantic attributes;
//! - semantic dependencies.
//!
//! Hardware-specific information belongs outside this structure.
//!
//! # Semantic attributes
//!
//! Attributes are represented as deterministic key/value pairs.
//!
//! The verifier does not interpret arbitrary attribute names. Their semantic
//! meaning is determined by the canonical IR adapter.
//!
//! This prevents the scheduler from accumulating a vendor-specific attribute
//! language.
//!
//! # Finish-once rule
//!
//! This file depends only on:
//!
//! ```text
//! quantum::ir::core::identity
//! quantum::ir::qubit
//! scheduling::errors
//! ```
//!
//! It does not import:
//!
//! ```text
//! planners
//! algorithms
//! resources
//! timing
//! hardware
//! routing
//! qec
//! runtime
//! ```
//!
//! Therefore adding a new scheduler, hardware target, resource type, routing
//! implementation, QEC protocol, or timing model does not require modifying
//! this file merely for integration.
//!
//! Adapters implement the semantic-view traits defined here.
//!
//! # Integration contract
//!
//! `adapters/ir.rs` should convert canonical `quantum::ir` operations into
//! `SemanticFingerprint` values.
//!
//! The final verifier should invoke:
//!
//! ```text
//! SemanticVerifier::verify()
//! ```
//!
//! alongside:
//!
//! ```text
//! structural verification
//! dependency verification
//! resource verification
//! timing verification
//! ```
//!
//! The complete production pipeline is:
//!
//! ```text
//! source IR
//!    │
//!    ├───────────────┐
//!    │               │
//!    ▼               ▼
//! source semantic    scheduler
//! fingerprint        │
//!                    ▼
//!               scheduled IR
//!                    │
//!                    ▼
//!              scheduled semantic
//!              fingerprint
//!                    │
//!          ┌─────────┴─────────┐
//!          ▼                   ▼
//!      semantic             other
//!      verifier           verification
//! ```
//!
//! # Failure policy
//!
//! Any semantic mismatch is a verification failure.
//!
//! The verifier never:
//!
//! - silently ignores an extra operation;
//! - silently ignores a missing operation;
//! - silently ignores a changed operand;
//! - silently ignores a changed condition;
//! - silently ignores a changed measurement;
//! - silently ignores a changed reset;
//! - silently ignores a changed parameter;
//! - silently treats a physical qubit change as a semantic change.
//!
//! The caller may configure fail-fast or full-diagnostic collection.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! The compiler-enforced safety boundary is:
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Configuration
// =============================================================================

/// Controls semantic verification behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticVerificationConfig {
    /// Stop after the first semantic violation.
    ///
    /// When `false`, the verifier collects all violations that can be
    /// determined without invalidating the comparison.
    pub fail_fast: bool,

    /// Require operation identities to match between source and scheduled
    /// representations.
    ///
    /// This should normally remain enabled for production verification because
    /// operation identity is the provenance boundary between source and
    /// scheduled representations.
    pub require_operation_identity: bool,

    /// Require source and scheduled operations to have the same relative
    /// semantic order.
    ///
    /// This is intentionally separate from dependency verification.
    ///
    /// When enabled, the order supplied by the semantic adapters is treated as
    /// canonical semantic order. This is useful for representations where
    /// order itself carries semantic meaning.
    ///
    /// It should normally be disabled for ordinary commutable quantum
    /// operations, because legal scheduling is allowed to reorder independent
    /// operations.
    pub require_sequence_order: bool,
}

impl Default for SemanticVerificationConfig {
    fn default() -> Self {
        Self {
            fail_fast: false,
            require_operation_identity: true,
            require_sequence_order: false,
        }
    }
}

// =============================================================================
// Semantic value
// =============================================================================

/// Canonical semantic parameter representation.
///
/// The verifier compares these values exactly.
///
/// Numerical approximation, symbolic simplification, and canonicalization
/// belong to the upstream IR adapter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticParameter {
    /// A canonical integer parameter.
    Integer(i128),

    /// A canonical unsigned integer parameter.
    Unsigned(u128),

    /// A canonical textual representation of a real-valued parameter.
    ///
    /// The adapter is responsible for producing a deterministic canonical
    /// representation.
    Real(String),

    /// A symbolic parameter.
    Symbol(String),

    /// A canonical expression fingerprint.
    Expression(String),

    /// A target-independent opaque semantic parameter fingerprint.
    ///
    /// This must be deterministic and stable within the semantic IR contract.
    Opaque(String),
}

impl SemanticParameter {
    /// Creates a canonical real parameter fingerprint.
    ///
    /// The caller must provide the canonical representation.
    #[must_use]
    pub fn real(value: impl Into<String>) -> Self {
        Self::Real(value.into())
    }

    /// Creates a symbolic parameter.
    #[must_use]
    pub fn symbol(value: impl Into<String>) -> Self {
        Self::Symbol(value.into())
    }

    /// Creates an expression fingerprint.
    #[must_use]
    pub fn expression(value: impl Into<String>) -> Self {
        Self::Expression(value.into())
    }

    /// Creates an opaque semantic fingerprint.
    #[must_use]
    pub fn opaque(value: impl Into<String>) -> Self {
        Self::Opaque(value.into())
    }
}

impl fmt::Display for SemanticParameter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(formatter, "{value}"),
            Self::Unsigned(value) => write!(formatter, "{value}"),
            Self::Real(value) => formatter.write_str(value),
            Self::Symbol(value) => formatter.write_str(value),
            Self::Expression(value) => formatter.write_str(value),
            Self::Opaque(value) => formatter.write_str(value),
        }
    }
}

// =============================================================================
// Classical semantic references
// =============================================================================

/// Stable semantic identity for a classical value.
///
/// The scheduler does not define the canonical classical IR identity here.
/// This wrapper exists so semantic adapters can expose deterministic
/// fingerprints without coupling this verifier to a particular classical
/// subsystem implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalValue(String);

impl ClassicalValue {
    /// Creates a classical semantic identity/fingerprint.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the canonical textual identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ClassicalValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Conditions
// =============================================================================

/// Semantic representation of an operation's classical control condition.
///
/// The verifier treats the condition as opaque canonical semantic data.
/// Evaluation semantics belong to the canonical IR/runtime layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SemanticCondition {
    /// Operation is unconditional.
    Unconditional,

    /// Canonical condition expression.
    Expression(String),

    /// Canonical opaque condition fingerprint.
    Opaque(String),
}

impl Default for SemanticCondition {
    fn default() -> Self {
        Self::Unconditional
    }
}

// =============================================================================
// Semantic dependencies
// =============================================================================

/// Semantic dependency carried by an operation.
///
/// This is intentionally separate from scheduler dependency edges.
///
/// A scheduler dependency may be introduced solely because of timing or
/// resources. Such a dependency must not automatically become a semantic
/// dependency.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SemanticDependency {
    /// Canonical predecessor operation identity.
    pub predecessor: OperationId,

    /// Canonical semantic dependency classification.
    pub kind: String,
}

impl SemanticDependency {
    /// Creates a semantic dependency.
    #[must_use]
    pub fn new(
        predecessor: OperationId,
        kind: impl Into<String>,
    ) -> Self {
        Self {
            predecessor,
            kind: kind.into(),
        }
    }
}

// =============================================================================
// Semantic attributes
// =============================================================================

/// One deterministic semantic attribute.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SemanticAttribute {
    /// Canonical attribute key.
    pub key: String,

    /// Canonical attribute value/fingerprint.
    pub value: String,
}

impl SemanticAttribute {
    /// Creates a semantic attribute.
    #[must_use]
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

// =============================================================================
// Semantic fingerprint
// =============================================================================

/// Complete semantic fingerprint for one operation.
///
/// This structure intentionally excludes scheduling concerns.
///
/// In particular it does NOT contain:
///
/// - start time;
/// - duration;
/// - resource IDs;
/// - physical qubit placement;
/// - channel IDs;
/// - scheduling priority;
/// - reservation IDs;
/// - calendar state.
///
/// Those belong to other verification layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFingerprint {
    /// Canonical operation identity.
    pub operation: OperationId,

    /// Canonical semantic operation kind/name.
    pub operation_kind: String,

    /// Logical qubit operands in semantic operand order.
    ///
    /// These are canonical `quantum::ir::qubit::QubitId` values.
    pub qubits: Vec<QubitId>,

    /// Classical inputs consumed by the operation.
    pub classical_inputs: Vec<ClassicalValue>,

    /// Classical values produced by the operation.
    pub classical_outputs: Vec<ClassicalValue>,

    /// Operation parameters.
    pub parameters: Vec<SemanticParameter>,

    /// Classical control condition.
    pub condition: SemanticCondition,

    /// Explicit semantic dependencies.
    pub dependencies: Vec<SemanticDependency>,

    /// Semantic attributes.
    pub attributes: Vec<SemanticAttribute>,
}

impl SemanticFingerprint {
    /// Creates a semantic fingerprint with the minimum required fields.
    #[must_use]
    pub fn new(
        operation: OperationId,
        operation_kind: impl Into<String>,
        qubits: Vec<QubitId>,
    ) -> Self {
        Self {
            operation,
            operation_kind: operation_kind.into(),
            qubits,
            classical_inputs: Vec::new(),
            classical_outputs: Vec::new(),
            parameters: Vec::new(),
            condition: SemanticCondition::Unconditional,
            dependencies: Vec::new(),
            attributes: Vec::new(),
        }
    }

    /// Sets classical inputs.
    #[must_use]
    pub fn with_classical_inputs(
        mut self,
        inputs: Vec<ClassicalValue>,
    ) -> Self {
        self.classical_inputs = inputs;
        self
    }

    /// Sets classical outputs.
    #[must_use]
    pub fn with_classical_outputs(
        mut self,
        outputs: Vec<ClassicalValue>,
    ) -> Self {
        self.classical_outputs = outputs;
        self
    }

    /// Sets semantic parameters.
    #[must_use]
    pub fn with_parameters(
        mut self,
        parameters: Vec<SemanticParameter>,
    ) -> Self {
        self.parameters = parameters;
        self
    }

    /// Sets the semantic control condition.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: SemanticCondition,
    ) -> Self {
        self.condition = condition;
        self
    }

    /// Sets semantic dependencies.
    #[must_use]
    pub fn with_dependencies(
        mut self,
        dependencies: Vec<SemanticDependency>,
    ) -> Self {
        self.dependencies = dependencies;
        self
    }

    /// Sets semantic attributes.
    #[must_use]
    pub fn with_attributes(
        mut self,
        attributes: Vec<SemanticAttribute>,
    ) -> Self {
        self.attributes = attributes;
        self
    }

    /// Returns a canonical semantic key excluding operation identity.
    ///
    /// This is useful for detecting accidental identity changes while
    /// preserving the actual computation.
    #[must_use]
    pub fn semantic_key(&self) -> SemanticKey {
        SemanticKey {
            operation_kind: self.operation_kind.clone(),
            qubits: self.qubits.clone(),
            classical_inputs: self.classical_inputs.clone(),
            classical_outputs: self.classical_outputs.clone(),
            parameters: self.parameters.clone(),
            condition: self.condition.clone(),
            dependencies: canonical_dependencies(&self.dependencies),
            attributes: canonical_attributes(&self.attributes),
        }
    }

    /// Returns whether two operations represent identical computation
    /// semantics, ignoring operation identity.
    #[must_use]
    pub fn semantically_equal(&self, other: &Self) -> bool {
        self.semantic_key() == other.semantic_key()
    }

    /// Returns whether the operation contains duplicate logical qubit
    /// operands.
    ///
    /// Duplicate operands may be legal for some future operation semantics,
    /// so this is informational rather than a semantic failure by itself.
    #[must_use]
    pub fn has_duplicate_qubits(&self) -> bool {
        let mut seen = BTreeSet::new();

        self.qubits
            .iter()
            .any(|qubit| !seen.insert(qubit.clone()))
    }
}

/// Semantic fingerprint excluding operation identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticKey {
    /// Semantic operation kind.
    pub operation_kind: String,

    /// Logical operands.
    pub qubits: Vec<QubitId>,

    /// Classical inputs.
    pub classical_inputs: Vec<ClassicalValue>,

    /// Classical outputs.
    pub classical_outputs: Vec<ClassicalValue>,

    /// Parameters.
    pub parameters: Vec<SemanticParameter>,

    /// Control condition.
    pub condition: SemanticCondition,

    /// Semantic dependencies.
    pub dependencies: Vec<SemanticDependency>,

    /// Semantic attributes.
    pub attributes: Vec<SemanticAttribute>,
}

fn canonical_dependencies(
    dependencies: &[SemanticDependency],
) -> Vec<SemanticDependency> {
    let mut result = dependencies.to_vec();
    result.sort();
    result.dedup();
    result
}

fn canonical_attributes(
    attributes: &[SemanticAttribute],
) -> Vec<SemanticAttribute> {
    let mut result = attributes.to_vec();
    result.sort();
    result.dedup();
    result
}

// =============================================================================
// Semantic operation view
// =============================================================================

/// Adapter boundary for source and scheduled semantic operations.
///
/// Implementations belong to the IR/scheduling adapter layer.
///
/// The verifier never needs to know the concrete operation type.
pub trait SemanticOperationView {
    /// Returns the operation's complete semantic fingerprint.
    fn semantic_fingerprint(&self) -> SemanticFingerprint;
}

// =============================================================================
// Verification issue
// =============================================================================

/// Machine-readable semantic verification issue.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SemanticIssueKind {
    /// The source operation is missing from the scheduled representation.
    MissingOperation,

    /// The scheduled representation contains an operation absent from the
    /// source representation.
    ExtraOperation,

    /// The same operation identity appears more than once.
    DuplicateOperation,

    /// Operation identity is not preserved.
    IdentityMismatch,

    /// Operation kind/name changed.
    OperationKindMismatch,

    /// Logical qubit operands changed.
    QubitOperandsMismatch,

    /// Classical inputs changed.
    ClassicalInputsMismatch,

    /// Classical outputs changed.
    ClassicalOutputsMismatch,

    /// Parameters changed.
    ParametersMismatch,

    /// Control condition changed.
    ConditionMismatch,

    /// Explicit semantic dependencies changed.
    SemanticDependenciesMismatch,

    /// Semantic attributes changed.
    SemanticAttributesMismatch,

    /// The semantic sequence order changed when sequence order was explicitly
    /// required.
    SequenceOrderMismatch,

    /// A source semantic identity is internally inconsistent.
    InvalidSource,

    /// A scheduled semantic identity is internally inconsistent.
    InvalidScheduled,
}

impl SemanticIssueKind {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::MissingOperation => "missing_operation",
            Self::ExtraOperation => "extra_operation",
            Self::DuplicateOperation => "duplicate_operation",
            Self::IdentityMismatch => "identity_mismatch",
            Self::OperationKindMismatch => "operation_kind_mismatch",
            Self::QubitOperandsMismatch => "qubit_operands_mismatch",
            Self::ClassicalInputsMismatch => "classical_inputs_mismatch",
            Self::ClassicalOutputsMismatch => "classical_outputs_mismatch",
            Self::ParametersMismatch => "parameters_mismatch",
            Self::ConditionMismatch => "condition_mismatch",
            Self::SemanticDependenciesMismatch => {
                "semantic_dependencies_mismatch"
            }
            Self::SemanticAttributesMismatch => {
                "semantic_attributes_mismatch"
            }
            Self::SequenceOrderMismatch => "sequence_order_mismatch",
            Self::InvalidSource => "invalid_source",
            Self::InvalidScheduled => "invalid_scheduled",
        }
    }
}

impl fmt::Display for SemanticIssueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Semantic issue
// =============================================================================

/// One semantic verification diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIssue {
    /// Machine-readable issue category.
    pub kind: SemanticIssueKind,

    /// Source operation identity, when known.
    pub source_operation: Option<OperationId>,

    /// Scheduled operation identity, when known.
    pub scheduled_operation: Option<OperationId>,

    /// Source operation index, when known.
    pub source_index: Option<usize>,

    /// Scheduled operation index, when known.
    pub scheduled_index: Option<usize>,

    /// Stable human-readable explanation.
    pub message: String,
}

impl SemanticIssue {
    fn new(
        kind: SemanticIssueKind,
        source_operation: Option<OperationId>,
        scheduled_operation: Option<OperationId>,
        source_index: Option<usize>,
        scheduled_index: Option<usize>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_operation,
            scheduled_operation,
            source_index,
            scheduled_index,
            message: message.into(),
        }
    }
}

impl fmt::Display for SemanticIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.kind, self.message)
    }
}

// =============================================================================
// Verification report
// =============================================================================

/// Complete semantic verification report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticVerificationReport {
    /// Whether semantic verification succeeded.
    pub valid: bool,

    /// Number of source operations inspected.
    pub source_operations: usize,

    /// Number of scheduled operations inspected.
    pub scheduled_operations: usize,

    /// Number of source identities indexed.
    pub source_identities: usize,

    /// Number of scheduled identities indexed.
    pub scheduled_identities: usize,

    /// Number of operations whose semantic fingerprints matched.
    pub matched_operations: usize,

    /// Semantic issues discovered.
    pub issues: Vec<SemanticIssue>,
}

impl SemanticVerificationReport {
    /// Creates an empty successful report.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            valid: true,
            source_operations: 0,
            scheduled_operations: 0,
            source_identities: 0,
            scheduled_identities: 0,
            matched_operations: 0,
            issues: Vec::new(),
        }
    }

    /// Returns whether the verification succeeded.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns whether at least one semantic issue exists.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Returns the number of semantic issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Returns all issues of one category.
    #[must_use]
    pub fn issues_of_kind(
        &self,
        kind: SemanticIssueKind,
    ) -> impl Iterator<Item = &SemanticIssue> {
        self.issues.iter().filter(move |issue| issue.kind == kind)
    }

    /// Converts a failed report into a scheduler error.
    ///
    /// The first issue is used as the concise canonical error context.
    pub fn into_result(
        self,
    ) -> Result<Self, crate::quantum::scheduling::errors::SchedulingError> {
        if self.valid {
            return Ok(self);
        }

        let first = self.issues.first();

        let reason = match first {
            Some(issue) => issue.message.clone(),
            None => String::from(
                "semantic verification failed without a diagnostic",
            ),
        };

        Err(
            crate::quantum::scheduling::errors::SchedulingError::VerificationFailed {
                operation: first.and_then(|issue| {
                    issue
                        .scheduled_operation
                        .clone()
                        .or_else(|| issue.source_operation.clone())
                }),
                reason,
            },
        )
    }
}

// =============================================================================
// Verifier
// =============================================================================

/// Production semantic verifier.
///
/// The verifier is stateless. It may therefore be reused for many independent
/// scheduling results and shared through immutable references.
///
/// No global mutable state is used.
#[derive(Debug, Clone, Copy)]
pub struct SemanticVerifier {
    config: SemanticVerificationConfig,
}

impl Default for SemanticVerifier {
    fn default() -> Self {
        Self::new(SemanticVerificationConfig::default())
    }
}

impl SemanticVerifier {
    /// Creates a verifier with explicit configuration.
    #[must_use]
    pub const fn new(config: SemanticVerificationConfig) -> Self {
        Self { config }
    }

    /// Returns the verifier configuration.
    #[must_use]
    pub const fn config(&self) -> SemanticVerificationConfig {
        self.config
    }

    /// Verifies source and scheduled semantic operations.
    ///
    /// The inputs are consumed only through the `SemanticOperationView`
    /// interface.
    ///
    /// No timing, routing, resource, or hardware assumptions are made.
    pub fn verify<S, T>(
        &self,
        source: &[S],
        scheduled: &[T],
    ) -> SemanticVerificationReport
    where
        S: SemanticOperationView,
        T: SemanticOperationView,
    {
        let mut report = SemanticVerificationReport {
            valid: true,
            source_operations: source.len(),
            scheduled_operations: scheduled.len(),
            source_identities: 0,
            scheduled_identities: 0,
            matched_operations: 0,
            issues: Vec::new(),
        };

        let source_fingerprints = self.collect_source(source, &mut report);

        if self.config.fail_fast && !report.valid {
            return report;
        }

        let scheduled_fingerprints =
            self.collect_scheduled(scheduled, &mut report);

        if self.config.fail_fast && !report.valid {
            return report;
        }

        self.compare_identity_sets(
            &source_fingerprints,
            &scheduled_fingerprints,
            &mut report,
        );

        if self.config.fail_fast && !report.valid {
            return report;
        }

        self.compare_operations(
            &source_fingerprints,
            &scheduled_fingerprints,
            &mut report,
        );

        if self.config.fail_fast && !report.valid {
            return report;
        }

        if self.config.require_sequence_order {
            self.compare_sequence_order(
                &source_fingerprints,
                &scheduled_fingerprints,
                &mut report,
            );
        }

        report
    }

    fn collect_source<S>(
        &self,
        source: &[S],
        report: &mut SemanticVerificationReport,
    ) -> BTreeMap<OperationId, IndexedSemanticFingerprint>
    where
        S: SemanticOperationView,
    {
        self.collect(
            source,
            report,
            true,
        )
    }

    fn collect_scheduled<T>(
        &self,
        scheduled: &[T],
        report: &mut SemanticVerificationReport,
    ) -> BTreeMap<OperationId, IndexedSemanticFingerprint>
    where
        T: SemanticOperationView,
    {
        self.collect(
            scheduled,
            report,
            false,
        )
    }

    fn collect<T>(
        &self,
        operations: &[T],
        report: &mut SemanticVerificationReport,
        source: bool,
    ) -> BTreeMap<OperationId, IndexedSemanticFingerprint>
    where
        T: SemanticOperationView,
    {
        let mut index = BTreeMap::new();

        for (position, operation) in operations.iter().enumerate() {
            let fingerprint = operation.semantic_fingerprint();

            if fingerprint.operation.to_string().is_empty() {
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    if source {
                        SemanticIssueKind::InvalidSource
                    } else {
                        SemanticIssueKind::InvalidScheduled
                    },
                    if source {
                        Some(fingerprint.operation.clone())
                    } else {
                        None
                    },
                    if source {
                        None
                    } else {
                        Some(fingerprint.operation.clone())
                    },
                    if source {
                        Some(position)
                    } else {
                        None
                    },
                    if source {
                        None
                    } else {
                        Some(position)
                    },
                    "semantic operation contains an empty operation identity",
                ));

                if self.config.fail_fast {
                    return index;
                }

                continue;
            }

            if index.contains_key(&fingerprint.operation) {
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::DuplicateOperation,
                    if source {
                        Some(fingerprint.operation.clone())
                    } else {
                        None
                    },
                    if source {
                        None
                    } else {
                        Some(fingerprint.operation.clone())
                    },
                    if source {
                        Some(position)
                    } else {
                        None
                    },
                    if source {
                        None
                    } else {
                        Some(position)
                    },
                    format!(
                        "operation `{}` occurs more than once in the {} \
                         semantic representation",
                        fingerprint.operation,
                        if source { "source" } else { "scheduled" }
                    ),
                ));

                if self.config.fail_fast {
                    return index;
                }

                continue;
            }

            index.insert(
                fingerprint.operation.clone(),
                IndexedSemanticFingerprint {
                    position,
                    fingerprint,
                },
            );
        }

        if source {
            report.source_identities = index.len();
        } else {
            report.scheduled_identities = index.len();
        }

        index
    }

    fn compare_identity_sets(
        &self,
        source: &BTreeMap<OperationId, IndexedSemanticFingerprint>,
        scheduled: &BTreeMap<OperationId, IndexedSemanticFingerprint>,
        report: &mut SemanticVerificationReport,
    ) {
        if !self.config.require_operation_identity {
            return;
        }

        for (operation, source_entry) in source {
            if !scheduled.contains_key(operation) {
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::MissingOperation,
                    Some(operation.clone()),
                    None,
                    Some(source_entry.position),
                    None,
                    format!(
                        "source operation `{operation}` is missing from \
                         the scheduled semantic representation"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }
        }

        for (operation, scheduled_entry) in scheduled {
            if !source.contains_key(operation) {
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::ExtraOperation,
                    None,
                    Some(operation.clone()),
                    None,
                    Some(scheduled_entry.position),
                    format!(
                        "scheduled operation `{operation}` is absent from \
                         the source semantic representation"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }
        }
    }

    fn compare_operations(
        &self,
        source: &BTreeMap<OperationId, IndexedSemanticFingerprint>,
        scheduled: &BTreeMap<OperationId, IndexedSemanticFingerprint>,
        report: &mut SemanticVerificationReport,
    ) {
        for (operation, source_entry) in source {
            let Some(scheduled_entry) = scheduled.get(operation) else {
                continue;
            };

            let source_fingerprint = &source_entry.fingerprint;
            let scheduled_fingerprint = &scheduled_entry.fingerprint;

            let mut matched = true;

            if source_fingerprint.operation != scheduled_fingerprint.operation
                && self.config.require_operation_identity
            {
                matched = false;

                report.valid = false;
                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::IdentityMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "operation identity changed from `{}` to `{}`",
                        source_fingerprint.operation,
                        scheduled_fingerprint.operation
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if source_fingerprint.operation_kind
                != scheduled_fingerprint.operation_kind
            {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::OperationKindMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "operation `{operation}` changed kind from `{}` to `{}`",
                        source_fingerprint.operation_kind,
                        scheduled_fingerprint.operation_kind
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if source_fingerprint.qubits
                != scheduled_fingerprint.qubits
            {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::QubitOperandsMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "logical qubit operands changed for operation `{operation}`"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if source_fingerprint.classical_inputs
                != scheduled_fingerprint.classical_inputs
            {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::ClassicalInputsMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "classical inputs changed for operation `{operation}`"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if source_fingerprint.classical_outputs
                != scheduled_fingerprint.classical_outputs
            {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::ClassicalOutputsMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "classical outputs changed for operation `{operation}`"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if source_fingerprint.parameters
                != scheduled_fingerprint.parameters
            {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::ParametersMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "parameters changed for operation `{operation}`"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if source_fingerprint.condition
                != scheduled_fingerprint.condition
            {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::ConditionMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "classical control condition changed for \
                         operation `{operation}`"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if canonical_dependencies(
                &source_fingerprint.dependencies,
            ) != canonical_dependencies(
                &scheduled_fingerprint.dependencies,
            ) {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::SemanticDependenciesMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "semantic dependencies changed for operation \
                         `{operation}`"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if canonical_attributes(
                &source_fingerprint.attributes,
            ) != canonical_attributes(
                &scheduled_fingerprint.attributes,
            ) {
                matched = false;
                report.valid = false;

                report.issues.push(SemanticIssue::new(
                    SemanticIssueKind::SemanticAttributesMismatch,
                    Some(source_fingerprint.operation.clone()),
                    Some(scheduled_fingerprint.operation.clone()),
                    Some(source_entry.position),
                    Some(scheduled_entry.position),
                    format!(
                        "semantic attributes changed for operation \
                         `{operation}`"
                    ),
                ));

                if self.config.fail_fast {
                    return;
                }
            }

            if matched {
                report.matched_operations =
                    report.matched_operations.saturating_add(1);
            }
        }
    }

    fn compare_sequence_order(
        &self,
        source: &BTreeMap<OperationId, IndexedSemanticFingerprint>,
        scheduled: &BTreeMap<OperationId, IndexedSemanticFingerprint>,
        report: &mut SemanticVerificationReport,
    ) {
        let mut source_order: Vec<(&OperationId, usize)> = source
            .iter()
            .map(|(id, entry)| (id, entry.position))
            .collect();

        let mut scheduled_order: Vec<(&OperationId, usize)> = scheduled
            .iter()
            .filter_map(|(id, entry)| {
                source.contains_key(id).then_some((id, entry.position))
            })
            .collect();

        source_order.sort_by_key(|(_, position)| *position);
        scheduled_order.sort_by_key(|(_, position)| *position);

        let source_ids: Vec<&OperationId> =
            source_order.into_iter().map(|(id, _)| id).collect();

        let scheduled_ids: Vec<&OperationId> =
            scheduled_order.into_iter().map(|(id, _)| id).collect();

        if source_ids != scheduled_ids {
            report.valid = false;

            report.issues.push(SemanticIssue::new(
                SemanticIssueKind::SequenceOrderMismatch,
                None,
                None,
                None,
                None,
                "semantic operation sequence order changed while \
                 sequence-order verification was explicitly enabled",
            ));
        }
    }
}

// =============================================================================
// Internal indexed representation
// =============================================================================

#[derive(Debug, Clone)]
struct IndexedSemanticFingerprint {
    position: usize,
    fingerprint: SemanticFingerprint,
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Verifies semantic preservation using the default production configuration.
///
/// The default configuration:
///
/// - requires operation identity;
/// - permits legal scheduler reordering of independent operations;
/// - collects all discoverable semantic issues.
pub fn verify_semantics<S, T>(
    source: &[S],
    scheduled: &[T],
) -> SemanticVerificationReport
where
    S: SemanticOperationView,
    T: SemanticOperationView,
{
    SemanticVerifier::default().verify(source, scheduled)
}

/// Verifies semantic preservation with explicit configuration.
pub fn verify_semantics_with_config<S, T>(
    source: &[S],
    scheduled: &[T],
    config: SemanticVerificationConfig,
) -> SemanticVerificationReport
where
    S: SemanticOperationView,
    T: SemanticOperationView,
{
    SemanticVerifier::new(config).verify(source, scheduled)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestOperation {
        fingerprint: SemanticFingerprint,
    }

    impl TestOperation {
        fn new(
            operation: &str,
            kind: &str,
            qubits: Vec<QubitId>,
        ) -> Self {
            Self {
                fingerprint: SemanticFingerprint::new(
                    OperationId::from(operation),
                    kind,
                    qubits,
                ),
            }
        }

        fn with_parameter(
            mut self,
            parameter: SemanticParameter,
        ) -> Self {
            self.fingerprint.parameters.push(parameter);
            self
        }

        fn with_condition(
            mut self,
            condition: SemanticCondition,
        ) -> Self {
            self.fingerprint.condition = condition;
            self
        }
    }

    impl SemanticOperationView for TestOperation {
        fn semantic_fingerprint(&self) -> SemanticFingerprint {
            self.fingerprint.clone()
        }
    }

    fn qubit(value: u64) -> QubitId {
        QubitId::new(value)
    }

    #[test]
    fn identical_operations_are_valid() {
        let source = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let scheduled = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let report = verify_semantics(&source, &scheduled);

        assert!(report.is_valid());
        assert_eq!(report.matched_operations, 1);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn empty_program_is_valid() {
        let source: Vec<TestOperation> = Vec::new();
        let scheduled: Vec<TestOperation> = Vec::new();

        let report = verify_semantics(&source, &scheduled);

        assert!(report.is_valid());
        assert_eq!(report.source_operations, 0);
        assert_eq!(report.scheduled_operations, 0);
    }

    #[test]
    fn missing_operation_is_rejected() {
        let source = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let scheduled: Vec<TestOperation> = Vec::new();

        let report = verify_semantics(&source, &scheduled);

        assert!(!report.is_valid());
        assert_eq!(report.issues.len(), 1);
        assert_eq!(
            report.issues[0].kind,
            SemanticIssueKind::MissingOperation
        );
    }

    #[test]
    fn extra_operation_is_rejected() {
        let source: Vec<TestOperation> = Vec::new();

        let scheduled = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let report = verify_semantics(&source, &scheduled);

        assert!(!report.is_valid());
        assert_eq!(report.issues.len(), 1);
        assert_eq!(
            report.issues[0].kind,
            SemanticIssueKind::ExtraOperation
        );
    }

    #[test]
    fn changed_operation_kind_is_rejected() {
        let source = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let scheduled = vec![TestOperation::new(
            "op0",
            "h",
            vec![qubit(0)],
        )];

        let report = verify_semantics(&source, &scheduled);

        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|issue| {
            issue.kind == SemanticIssueKind::OperationKindMismatch
        }));
    }

    #[test]
    fn changed_qubit_operand_is_rejected() {
        let source = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let scheduled = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(1)],
        )];

        let report = verify_semantics(&source, &scheduled);

        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|issue| {
            issue.kind == SemanticIssueKind::QubitOperandsMismatch
        }));
    }

    #[test]
    fn changed_parameter_is_rejected() {
        let source = vec![
            TestOperation::new("op0", "rx", vec![qubit(0)])
                .with_parameter(SemanticParameter::real("1/2")),
        ];

        let scheduled = vec![
            TestOperation::new("op0", "rx", vec![qubit(0)])
                .with_parameter(SemanticParameter::real("1/3")),
        ];

        let report = verify_semantics(&source, &scheduled);

        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|issue| {
            issue.kind == SemanticIssueKind::ParametersMismatch
        }));
    }

    #[test]
    fn changed_condition_is_rejected() {
        let source = vec![
            TestOperation::new("op0", "x", vec![qubit(0)])
                .with_condition(SemanticCondition::Expression(
                    String::from("c0 == 0"),
                )),
        ];

        let scheduled = vec![
            TestOperation::new("op0", "x", vec![qubit(0)])
                .with_condition(SemanticCondition::Expression(
                    String::from("c0 == 1"),
                )),
        ];

        let report = verify_semantics(&source, &scheduled);

        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|issue| {
            issue.kind == SemanticIssueKind::ConditionMismatch
        }));
    }

    #[test]
    fn legal_scheduler_reordering_is_allowed_by_default() {
        let source = vec![
            TestOperation::new("op0", "x", vec![qubit(0)]),
            TestOperation::new("op1", "x", vec![qubit(1)]),
        ];

        let scheduled = vec![
            TestOperation::new("op1", "x", vec![qubit(1)]),
            TestOperation::new("op0", "x", vec![qubit(0)]),
        ];

        let report = verify_semantics(&source, &scheduled);

        assert!(report.is_valid());
        assert_eq!(report.matched_operations, 2);
    }

    #[test]
    fn explicit_sequence_order_can_be_required() {
        let source = vec![
            TestOperation::new("op0", "x", vec![qubit(0)]),
            TestOperation::new("op1", "x", vec![qubit(1)]),
        ];

        let scheduled = vec![
            TestOperation::new("op1", "x", vec![qubit(1)]),
            TestOperation::new("op0", "x", vec![qubit(0)]),
        ];

        let config = SemanticVerificationConfig {
            require_sequence_order: true,
            ..SemanticVerificationConfig::default()
        };

        let report =
            verify_semantics_with_config(&source, &scheduled, config);

        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|issue| {
            issue.kind == SemanticIssueKind::SequenceOrderMismatch
        }));
    }

    #[test]
    fn physical_routing_is_not_part_of_semantic_fingerprint() {
        // The semantic verifier only sees canonical logical qubits.
        //
        // Physical placement belongs to routing/resource verification.
        let source = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let scheduled = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let report = verify_semantics(&source, &scheduled);

        assert!(report.is_valid());
    }

    #[test]
    fn duplicate_operation_is_rejected() {
        let source = vec![
            TestOperation::new("op0", "x", vec![qubit(0)]),
            TestOperation::new("op0", "x", vec![qubit(0)]),
        ];

        let scheduled = vec![
            TestOperation::new("op0", "x", vec![qubit(0)]),
        ];

        let report = verify_semantics(&source, &scheduled);

        assert!(!report.is_valid());
        assert!(report.issues.iter().any(|issue| {
            issue.kind == SemanticIssueKind::DuplicateOperation
        }));
    }

    #[test]
    fn fail_fast_stops_after_first_discoverable_failure() {
        let source = vec![TestOperation::new(
            "op0",
            "x",
            vec![qubit(0)],
        )];

        let scheduled = vec![
            TestOperation::new("op1", "h", vec![qubit(1)]),
            TestOperation::new("op2", "h", vec![qubit(2)]),
        ];

        let config = SemanticVerificationConfig {
            fail_fast: true,
            ..SemanticVerificationConfig::default()
        };

        let report =
            verify_semantics_with_config(&source, &scheduled, config);

        assert!(!report.is_valid());
        assert!(!report.issues.is_empty());
    }

    #[test]
    fn semantic_attributes_are_order_independent() {
        let source = vec![
            TestOperation::new("op0", "x", vec![qubit(0)])
                .with_attributes(vec![
                    SemanticAttribute::new("a", "1"),
                    SemanticAttribute::new("b", "2"),
                ]),
        ];

        let scheduled = vec![
            TestOperation::new("op0", "x", vec![qubit(0)])
                .with_attributes(vec![
                    SemanticAttribute::new("b", "2"),
                    SemanticAttribute::new("a", "1"),
                ]),
        ];

        let report = verify_semantics(&source, &scheduled);

        assert!(report.is_valid());
    }

    #[test]
    fn semantic_dependencies_are_order_independent() {
        let mut source_op =
            TestOperation::new("op2", "x", vec![qubit(0)]);

        source_op.fingerprint.dependencies = vec![
            SemanticDependency::new(
                OperationId::from("op0"),
                "measurement",
            ),
            SemanticDependency::new(
                OperationId::from("op1"),
                "classical",
            ),
        ];

        let mut scheduled_op =
            TestOperation::new("op2", "x", vec![qubit(0)]);

        scheduled_op.fingerprint.dependencies = vec![
            SemanticDependency::new(
                OperationId::from("op1"),
                "classical",
            ),
            SemanticDependency::new(
                OperationId::from("op0"),
                "measurement",
            ),
        ];

        let report =
            verify_semantics(&[source_op], &[scheduled_op]);

        assert!(report.is_valid());
    }

    #[test]
    fn semantic_key_ignores_operation_identity() {
        let first = SemanticFingerprint::new(
            OperationId::from("op0"),
            "x",
            vec![qubit(0)],
        );

        let second = SemanticFingerprint::new(
            OperationId::from("op1"),
            "x",
            vec![qubit(0)],
        );

        assert_eq!(first.semantic_key(), second.semantic_key());
        assert!(first.semantically_equal(&second));
        assert_ne!(first.operation, second.operation);
    }
}