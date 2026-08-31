//! Zamani Quantum IR — Reset Semantics
//!
//! Canonical, hardware-independent representation of quantum reset.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This module answers:
//!
//!     "What quantum reset does the program semantically request?"
//!
//! It does NOT answer:
//!
//! - which physical qubit performs the reset;
//! - which reset pulse is used;
//! - which calibration is used;
//! - whether reset is active or passive;
//! - whether measurement is used internally;
//! - which control/readout channel is used;
//! - which hardware vendor is used;
//! - when reset executes;
//! - how reset is routed;
//! - how reset is scheduled;
//! - how reset is optimized;
//! - how reset is simulated;
//! - how reset is transported to a QPU.
//!
//! Those responsibilities belong to downstream layers.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani quantum program is written once at the semantic level.
//!
//! Therefore this module contains:
//!
//! - no fixed maximum qubit count;
//! - no fixed register size;
//! - no vendor-specific reset mechanism;
//! - no hardware topology;
//! - no pulse implementation;
//! - no simulator state;
//! - no backend API;
//! - no fixed batch-reset ceiling.
//!
//! Any resource/security limits are supplied by an explicit policy layer such
//! as `QuantumIrLimits`. They are not semantic properties of reset.
//!
//! ============================================================================
//! RESET SEMANTICS
//! ============================================================================
//!
//! Canonical quantum reset means:
//!
//!     target subsystem -> computational ground state |0>
//!
//! At the semantic level, for a target density operator `rho`, the logical
//! postcondition is equivalent to preparation of the target in |0>.
//!
//! The IR does NOT prescribe how this is achieved physically.
//!
//! A backend may implement reset using, for example:
//!
//! - native active reset;
//! - measurement followed by conditional correction;
//! - optical re-preparation;
//! - ion re-initialization;
//! - dissipative reset;
//! - calibrated pulse sequences;
//! - another implementation that satisfies the semantic contract.
//!
//! ============================================================================
//! IMPORTANT DISTINCTION: RESET VS INITIALIZATION
//! ============================================================================
//!
//! `Reset` owns the canonical semantic operation:
//!
//!     reset q -> |0>
//!
//! Arbitrary state preparation belongs to the initialization/state-preparation
//! layer.
//!
//! This distinction prevents `reset.rs` from becoming a second, incompatible
//! state-preparation system.
//!
//! If a future IR operation needs to prepare:
//!
//!     |1>
//!     |+>
//!     arbitrary |psi>
//!     encoded logical state
//!     stabilizer state
//!     multi-qubit entangled state
//!
//! it should use the appropriate initialization/preparation model rather than
//! changing the meaning of canonical reset.
//!
//! ============================================================================
//! LOGICAL VS PHYSICAL IDENTITY
//! ============================================================================
//!
//! Reset targets MUST use the canonical:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This file intentionally does NOT define another qubit identifier.
//!
//! A `QubitId` identifies semantic logical-program identity.
//!
//! Physical placement belongs to mapping/routing/hardware.
//!
//! ============================================================================
//! SINGLE VS BATCH RESET
//! ============================================================================
//!
//! The semantic model supports:
//!
//!     reset q0
//!
//! and:
//!
//!     reset q0, q1, q2, ...
//!
//! without imposing a fixed maximum.
//!
//! A batch reset means that every listed logical target receives the same
//! canonical reset postcondition.
//!
//! It does NOT imply a hardware implementation that resets all targets
//! simultaneously.
//!
//! Parallelism is a scheduling/resource decision.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Source operand order is preserved in `targets()` because preserving source
//! order is useful for diagnostics and provenance.
//!
//! Canonical target order is separately available through:
//!
//!     canonical_targets()
//!
//! which returns logical qubits in deterministic `QubitId` order.
//!
//! This allows serialization/hashing layers to canonicalize without silently
//! changing the source-level representation.
//!
//! ============================================================================
//! VALIDATION BOUNDARY
//! ============================================================================
//!
//! This module validates properties that can be established locally:
//!
//! - at least one reset target exists;
//! - no duplicate logical qubit occurs;
//! - target IDs are valid within an explicitly supplied logical namespace;
//! - reset semantics are canonical;
//! - no physical/hardware information is embedded;
//! - source target ordering is preserved.
//!
//! Whole-program validation belongs to `validation.rs`.
//!
//! Hardware capability validation belongs to hardware/target validation.
//!
//! Scheduling validation belongs to scheduling.
//!
//! ============================================================================
//! NO HIDDEN EXECUTION
//! ============================================================================
//!
//! Constructing a `Reset` never mutates a qubit, simulator, hardware device,
//! memory provider, or backend.
//!
//! It creates semantic IR only.
//!
//! ============================================================================
//! SERIALIZATION CONTRACT
//! ============================================================================
//!
//! The semantic serialization identity is:
//!
//!     zamani.quantum.ir.quantum.reset
//!
//! Schema versioning is explicit.
//!
//! The target list is semantically significant.
//!
//! `canonical_targets()` exists for canonical serializers that need stable
//! ordering independent of source ordering.
//!
//! ============================================================================
//! HASHING CONTRACT
//! ============================================================================
//!
//! This module does not implement a cryptographic hash.
//!
//! The canonical hashing layer owns hashing.
//!
//! It must hash:
//!
//! - reset semantic schema identity/version;
//! - reset operation semantics;
//! - canonical logical target set;
//! - semantically relevant attributes introduced by future extensions.
//!
//! It must not hash:
//!
//! - memory addresses;
//! - process identifiers;
//! - allocation order;
//! - hardware pointers;
//! - nondeterministic metadata.
//!
//! ============================================================================
//! EXTENSIBILITY
//! ============================================================================
//!
//! Future reset-related implementation details must not be added here merely
//! because a backend needs them.
//!
//! Examples that belong downstream:
//!
//!     NativeReset
//!     ActiveResetPulse
//!     ResetCalibration
//!     ResetChannel
//!     ResetLatency
//!     ReadoutBasedReset
//!
//! Those belong to target/hardware/pulse/backend dialects.
//!
//! The semantic reset contract remains stable.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! `quantum::ir::qubit`
//!     Supplies canonical `QubitId`.
//!
//! `quantum::ir::gate`
//!     Standard `GateKind::Reset` represents the standard one-qubit reset
//!     vocabulary. This module provides the richer semantic reset contract
//!     used by the universal instruction/program layer.
//!
//! `quantum::ir::instruction`
//!     May represent a reset instruction by referencing/embedding `Reset`.
//!     It should not redefine reset semantics.
//!
//! `quantum::ir::operation`
//!     Owns the universal operation container and may carry this reset body.
//!
//! `quantum::ir::program`
//!     Owns program ordering, regions and namespace declarations.
//!
//! `quantum::ir::validation`
//!     Performs whole-program validation, including namespace validation.
//!
//! `quantum::ir::limits`
//!     Supplies explicit resource/security policy limits. This file does not
//!     contain architectural qubit limits.
//!
//! `quantum::ir::serialization`
//!     Owns canonical persistence.
//!
//! `quantum::ir::hash`
//!     Owns canonical content hashing.
//!
//! `quantum::ir::analysis`
//!     Reads reset targets and reset dependencies.
//!
//! `quantum::ir::scheduling`
//!     Determines actual execution timing and parallelism.
//!
//! `quantum::hardware`
//!     Determines whether and how a target can implement reset.
//!
//! `quantum::memory`
//!     Provides downstream execution/state-management contracts.
//!
//! `quantum::simulator`
//!     Interprets reset mathematically without becoming part of this IR.
//!
//! `quantum::qec`
//!     May consume reset operations as part of logical/fault-tolerant
//!     workflows without changing canonical reset semantics.
//!
//! `quantum::backend`
//!     Lowers the semantic reset into target-specific execution.
//!
//! ============================================================================
//! FILE-COMPLETION GUARANTEE
//! ============================================================================
//!
//! This file owns:
//!
//! - reset schema identity;
//! - canonical reset semantic state;
//! - reset target representation;
//! - single-target construction;
//! - batch-target construction;
//! - deterministic target access;
//! - duplicate detection;
//! - namespace validation;
//! - semantic equality;
//! - canonical target ordering;
//! - reset-local errors;
//! - reset-local result types;
//! - reset-local tests.
//!
//! Later modules should consume this contract rather than changing what
//! canonical reset means.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// SCHEMA
// ============================================================================

/// Stable semantic schema identifier for canonical reset.
pub const RESET_SCHEMA_ID: &str = "zamani.quantum.ir.quantum.reset";

/// Major semantic version of the reset contract.
///
/// This is the reset schema version, not the complete Quantum IR version.
pub const RESET_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type returned by reset construction and local validation.
pub type ResetResult<T> = Result<T, ResetError>;

// ============================================================================
// ERROR MODEL
// ============================================================================

/// Errors produced by local reset construction and validation.
///
/// These errors remain local to the reset semantic module.
///
/// Whole-program validation may translate them into the canonical `IrError`
/// model owned by `quantum::ir::errors`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResetError {
    /// A reset operation contains no logical targets.
    EmptyTargets,

    /// A logical qubit appears more than once.
    DuplicateTarget {
        /// Duplicated canonical logical qubit.
        qubit: QubitId,
    },

    /// A target is outside an explicitly supplied logical namespace.
    ///
    /// This is a validation-policy error, not an architectural limit.
    TargetOutOfRange {
        /// Invalid logical target.
        qubit: QubitId,

        /// Number of logical qubits in the supplied namespace.
        logical_qubits: usize,
    },

    /// The target list contains an invalid structure.
    InvalidStructure {
        /// Stable explanation of the violated local invariant.
        message: &'static str,
    },
}

impl fmt::Display for ResetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTargets => {
                formatter.write_str(
                    "reset requires at least one logical qubit target",
                )
            }

            Self::DuplicateTarget { qubit } => {
                write!(
                    formatter,
                    "reset contains duplicate logical qubit {qubit}"
                )
            }

            Self::TargetOutOfRange {
                qubit,
                logical_qubits,
            } => {
                write!(
                    formatter,
                    "logical reset target {qubit} is outside logical namespace 0..{logical_qubits}"
                )
            }

            Self::InvalidStructure { message } => {
                write!(
                    formatter,
                    "invalid reset structure: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ResetError {}

// ============================================================================
// RESET SEMANTIC STATE
// ============================================================================

/// Canonical semantic state produced by reset.
///
/// Canonical reset means preparation of the computational ground state.
///
/// This enum deliberately contains only the semantic state of canonical reset.
/// Arbitrary state preparation belongs to the initialization/state-preparation
/// layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResetState {
    /// Prepare the target logical qubit in computational ground state `|0⟩`.
    Zero,
}

impl Default for ResetState {
    fn default() -> Self {
        Self::Zero
    }
}

impl ResetState {
    /// Returns a stable schema-level name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zero => "zero",
        }
    }

    /// Returns whether this is the canonical computational reset state.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Zero)
    }
}

impl fmt::Display for ResetState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RESET IMPLEMENTATION PREFERENCE
// ============================================================================

/// Optional implementation preference attached to reset semantics.
///
/// This is deliberately a preference rather than a hardware implementation.
///
/// The default is [`ResetMethod::Automatic`].
///
/// A backend may reject an explicit method if its capability contract does
/// not support that requested implementation strategy.
///
/// Canonical semantic meaning remains unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResetMethod {
    /// Allow the target compiler/backend to choose any implementation that
    /// satisfies the canonical reset postcondition.
    Automatic,

    /// Request a target-native reset operation when supported.
    Native,

    /// Permit reset implemented using measurement and classical feedback.
    MeasurementConditional,

    /// Request explicit state re-preparation.
    StatePreparation,
}

impl Default for ResetMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

impl ResetMethod {
    /// Returns a stable schema-level identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Native => "native",
            Self::MeasurementConditional => "measurement_conditional",
            Self::StatePreparation => "state_preparation",
        }
    }

    /// Returns whether the method leaves implementation selection to the
    /// downstream target.
    #[must_use]
    pub const fn is_automatic(self) -> bool {
        matches!(self, Self::Automatic)
    }

    /// Returns whether the method requests an explicit implementation class.
    #[must_use]
    pub const fn is_explicit(self) -> bool {
        !self.is_automatic()
    }
}

impl fmt::Display for ResetMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RESET POLICY
// ============================================================================

/// Semantic/execution preference associated with a reset.
///
/// This structure intentionally contains no hardware identifiers.
///
/// It is safe to carry through the canonical IR because all fields describe
/// semantic intent or compilation preference rather than physical realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetPolicy {
    /// Semantic state that reset must establish.
    state: ResetState,

    /// Preferred implementation class.
    method: ResetMethod,

    /// Whether a downstream implementation may use a semantically equivalent
    /// implementation rather than the preferred method.
    allow_equivalent_implementation: bool,

    /// Whether a downstream implementation may omit physical work when it can
    /// prove that the reset postcondition is already satisfied.
    ///
    /// This does not change semantic meaning.
    allow_idempotent_noop: bool,
}

impl Default for ResetPolicy {
    fn default() -> Self {
        Self::canonical()
    }
}

impl ResetPolicy {
    /// Creates the canonical provider-neutral reset policy.
    #[must_use]
    pub const fn canonical() -> Self {
        Self {
            state: ResetState::Zero,
            method: ResetMethod::Automatic,
            allow_equivalent_implementation: true,
            allow_idempotent_noop: true,
        }
    }

    /// Returns the required semantic post-reset state.
    #[must_use]
    pub const fn state(self) -> ResetState {
        self.state
    }

    /// Returns the implementation preference.
    #[must_use]
    pub const fn method(self) -> ResetMethod {
        self.method
    }

    /// Returns whether an equivalent downstream implementation is permitted.
    #[must_use]
    pub const fn allows_equivalent_implementation(self) -> bool {
        self.allow_equivalent_implementation
    }

    /// Returns whether an already-satisfied reset may be optimized away by a
    /// downstream implementation that can prove the postcondition.
    #[must_use]
    pub const fn allows_idempotent_noop(self) -> bool {
        self.allow_idempotent_noop
    }

    /// Requests a native implementation.
    #[must_use]
    pub const fn native(mut self) -> Self {
        self.method = ResetMethod::Native;
        self
    }

    /// Requests measurement/conditional implementation.
    #[must_use]
    pub const fn measurement_conditional(mut self) -> Self {
        self.method = ResetMethod::MeasurementConditional;
        self
    }

    /// Requests explicit state preparation.
    #[must_use]
    pub const fn state_preparation(mut self) -> Self {
        self.method = ResetMethod::StatePreparation;
        self
    }

    /// Requires the preferred implementation method to be honored exactly.
    ///
    /// Hardware capability validation remains downstream.
    #[must_use]
    pub const fn exact_method(mut self) -> Self {
        self.allow_equivalent_implementation = false;
        self
    }

    /// Prevents a downstream implementation from treating the reset as a
    /// proven no-op.
    #[must_use]
    pub const fn require_execution(mut self) -> Self {
        self.allow_idempotent_noop = false;
        self
    }
}

// ============================================================================
// RESET TARGET
// ============================================================================

/// One logical reset target.
///
/// A reset target contains only canonical logical identity.
///
/// It deliberately contains no:
///
/// - physical qubit;
/// - topology node;
/// - pulse;
/// - calibration;
/// - channel;
/// - simulator index;
/// - backend handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResetTarget {
    qubit: QubitId,
}

impl ResetTarget {
    /// Creates a reset target for one logical qubit.
    #[must_use]
    pub const fn new(qubit: QubitId) -> Self {
        Self { qubit }
    }

    /// Returns the canonical logical qubit.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Validates this target against an explicitly supplied logical namespace.
    ///
    /// `logical_qubits` is a policy/program namespace size supplied by the
    /// caller. It is not a machine-size constant.
    pub fn validate(self, logical_qubits: usize) -> ResetResult<()> {
        if self.qubit.index() >= logical_qubits {
            return Err(ResetError::TargetOutOfRange {
                qubit: self.qubit,
                logical_qubits,
            });
        }

        Ok(())
    }
}

impl From<QubitId> for ResetTarget {
    fn from(qubit: QubitId) -> Self {
        Self::new(qubit)
    }
}

impl From<ResetTarget> for QubitId {
    fn from(target: ResetTarget) -> Self {
        target.qubit()
    }
}

impl fmt::Display for ResetTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "reset({})", self.qubit)
    }
}

// ============================================================================
// RESET OPERATION
// ============================================================================

/// Canonical semantic reset operation.
///
/// A `Reset` describes one or more logical qubits that must be prepared in
/// the computational ground state.
///
/// It does not execute anything.
///
/// # Examples
///
/// Single target:
///
/// ```
/// # use crate::quantum::ir::qubit::QubitId;
/// # use crate::quantum::ir::quantum::reset::Reset;
/// let reset = Reset::single(QubitId::new(0)).unwrap();
/// assert_eq!(reset.len(), 1);
/// ```
///
/// Batch target:
///
/// ```
/// # use crate::quantum::ir::qubit::QubitId;
/// # use crate::quantum::ir::quantum::reset::Reset;
/// let reset = Reset::many(vec![
///     QubitId::new(0),
///     QubitId::new(4),
///     QubitId::new(9),
/// ]).unwrap();
/// assert_eq!(reset.len(), 3);
/// ```
///
/// The number of targets is determined by program data and available
/// resources. There is no semantic fixed maximum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reset {
    targets: Vec<QubitId>,
    policy: ResetPolicy,
}

impl Reset {
    /// Creates a single canonical computational reset.
    ///
    /// This is the most direct constructor and corresponds to the ordinary
    /// semantic `reset q` operation.
    pub fn single(qubit: QubitId) -> ResetResult<Self> {
        Self::with_targets(vec![qubit])
    }

    /// Creates a batch reset using the canonical computational reset policy.
    ///
    /// Duplicate targets are rejected rather than silently removed.
    pub fn many(targets: Vec<QubitId>) -> ResetResult<Self> {
        Self::with_targets(targets)
    }

    /// Creates a reset with explicit policy.
    pub fn with_policy(
        qubit: QubitId,
        policy: ResetPolicy,
    ) -> ResetResult<Self> {
        Self::with_targets_and_policy(vec![qubit], policy)
    }

    /// Creates a batch reset with explicit policy.
    pub fn many_with_policy(
        targets: Vec<QubitId>,
        policy: ResetPolicy,
    ) -> ResetResult<Self> {
        Self::with_targets_and_policy(targets, policy)
    }

    /// Creates a reset from a vector of canonical logical targets.
    pub fn with_targets(targets: Vec<QubitId>) -> ResetResult<Self> {
        Self::with_targets_and_policy(
            targets,
            ResetPolicy::canonical(),
        )
    }

    /// Creates a reset from a vector of targets and an explicit policy.
    pub fn with_targets_and_policy(
        targets: Vec<QubitId>,
        policy: ResetPolicy,
    ) -> ResetResult<Self> {
        Self::validate_target_slice(&targets)?;

        Ok(Self { targets, policy })
    }

    /// Returns the number of logical reset targets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether the reset has no targets.
    ///
    /// Valid `Reset` values are never empty. This method exists for generic
    /// collection-oriented APIs and defensive callers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns all logical targets in source/IR order.
    ///
    /// The returned order is never silently modified.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns the first target.
    ///
    /// A valid `Reset` always contains at least one target.
    #[must_use]
    pub fn first_target(&self) -> QubitId {
        self.targets[0]
    }

    /// Returns whether this is exactly one target.
    #[must_use]
    pub fn is_single_target(&self) -> bool {
        self.targets.len() == 1
    }

    /// Returns whether this reset targets multiple logical qubits.
    #[must_use]
    pub fn is_batch(&self) -> bool {
        self.targets.len() > 1
    }

    /// Returns the reset semantic policy.
    #[must_use]
    pub const fn policy(&self) -> ResetPolicy {
        self.policy
    }

    /// Returns the semantic post-reset state.
    #[must_use]
    pub const fn state(&self) -> ResetState {
        self.policy.state()
    }

    /// Returns the implementation preference.
    #[must_use]
    pub const fn method(&self) -> ResetMethod {
        self.policy.method()
    }

    /// Returns whether equivalent downstream implementations are allowed.
    #[must_use]
    pub const fn allows_equivalent_implementation(&self) -> bool {
        self.policy.allows_equivalent_implementation()
    }

    /// Returns whether a downstream implementation may prove and use an
    /// idempotent no-op.
    #[must_use]
    pub const fn allows_idempotent_noop(&self) -> bool {
        self.policy.allows_idempotent_noop()
    }

    /// Returns the targets in deterministic canonical `QubitId` order.
    ///
    /// This does not mutate the reset and does not change source ordering.
    ///
    /// The returned vector is suitable for canonical serialization/hashing
    /// layers that require order-independent target representation.
    #[must_use]
    pub fn canonical_targets(&self) -> Vec<QubitId> {
        let mut targets = self.targets.clone();
        targets.sort_unstable();
        targets
    }

    /// Returns whether two resets target exactly the same logical qubit set.
    ///
    /// Source ordering is ignored.
    #[must_use]
    pub fn targets_equivalent(&self, other: &Self) -> bool {
        self.canonical_targets() == other.canonical_targets()
    }

    /// Validates all targets against an explicitly supplied logical namespace.
    ///
    /// The namespace size belongs to the caller. This module does not define
    /// a maximum number of logical qubits.
    pub fn validate_namespace(
        &self,
        logical_qubits: usize,
    ) -> ResetResult<()> {
        self.validate()?;

        for qubit in &self.targets {
            if qubit.index() >= logical_qubits {
                return Err(ResetError::TargetOutOfRange {
                    qubit: *qubit,
                    logical_qubits,
                });
            }
        }

        Ok(())
    }

    /// Validates local semantic invariants.
    pub fn validate(&self) -> ResetResult<()> {
        Self::validate_target_slice(&self.targets)?;

        if !self.policy.state().is_zero() {
            return Err(ResetError::InvalidStructure {
                message: "canonical reset must establish the zero state",
            });
        }

        Ok(())
    }

    /// Returns a single target when this reset contains exactly one target.
    ///
    /// This is useful for integration with the existing standard
    /// `GateKind::Reset`, whose gate arity is one logical qubit.
    #[must_use]
    pub fn single_target(&self) -> Option<QubitId> {
        if self.targets.len() == 1 {
            Some(self.targets[0])
        } else {
            None
        }
    }

    /// Converts this reset to its logical target vector.
    ///
    /// This is intentionally a copy so downstream consumers cannot mutate the
    /// canonical reset object through an alias.
    #[must_use]
    pub fn into_targets(self) -> Vec<QubitId> {
        self.targets
    }

    /// Creates a reset from an iterator without requiring the caller to first
    /// allocate a `Vec<QubitId>`.
    ///
    /// The iterator is materialized exactly once because the IR owns its
    /// semantic operands.
    pub fn from_iter<I>(targets: I) -> ResetResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::with_targets(targets.into_iter().collect())
    }

    /// Returns an iterator over targets in source/IR order.
    pub fn iter(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.targets.iter().copied()
    }

    /// Returns an iterator over targets in canonical sorted order.
    ///
    /// This allocates a temporary vector because canonical ordering must not
    /// mutate the semantic source representation.
    pub fn iter_canonical(
        &self,
    ) -> impl Iterator<Item = QubitId> {
        self.canonical_targets().into_iter()
    }

    /// Validates a target collection without constructing a reset.
    ///
    /// This is useful to frontends and other IR builders that need preflight
    /// validation before constructing the final operation.
    pub fn validate_targets(
        targets: &[QubitId],
    ) -> ResetResult<()> {
        Self::validate_target_slice(targets)
    }

    /// Returns whether a target slice contains duplicate logical qubits.
    #[must_use]
    pub fn contains_duplicates(targets: &[QubitId]) -> bool {
        let mut seen = BTreeSet::new();

        for target in targets {
            if !seen.insert(*target) {
                return true;
            }
        }

        false
    }

    /// Returns the number of distinct logical targets in a target slice.
    #[must_use]
    pub fn distinct_target_count(
        targets: &[QubitId],
    ) -> usize {
        let mut seen = BTreeSet::new();

        for target in targets {
            seen.insert(*target);
        }

        seen.len()
    }

    fn validate_target_slice(
        targets: &[QubitId],
    ) -> ResetResult<()> {
        if targets.is_empty() {
            return Err(ResetError::EmptyTargets);
        }

        let mut seen = BTreeSet::new();

        for target in targets {
            if !seen.insert(*target) {
                return Err(ResetError::DuplicateTarget {
                    qubit: *target,
                });
            }
        }

        Ok(())
    }
}

impl TryFrom<QubitId> for Reset {
    type Error = ResetError;

    fn try_from(qubit: QubitId) -> Result<Self, Self::Error> {
        Self::single(qubit)
    }
}

impl TryFrom<Vec<QubitId>> for Reset {
    type Error = ResetError;

    fn try_from(targets: Vec<QubitId>) -> Result<Self, Self::Error> {
        Self::many(targets)
    }
}

impl From<Reset> for Vec<QubitId> {
    fn from(reset: Reset) -> Self {
        reset.into_targets()
    }
}

impl fmt::Display for Reset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("reset ");

        for (index, qubit) in self.targets.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }

            write!(formatter, "{qubit}")?;
        }

        Ok(())
    }
}

// ============================================================================
// CANONICAL RESET HELPERS
// ============================================================================

/// Creates a canonical single-qubit reset.
///
/// This convenience function is intentionally equivalent to
/// `Reset::single(qubit)`.
pub fn reset(qubit: QubitId) -> ResetResult<Reset> {
    Reset::single(qubit)
}

/// Creates a canonical batch reset.
///
/// This convenience function is intentionally equivalent to
/// `Reset::many(targets)`.
pub fn reset_many(
    targets: Vec<QubitId>,
) -> ResetResult<Reset> {
    Reset::many(targets)
}

/// Validates a canonical reset target against a logical namespace.
pub fn validate_target(
    qubit: QubitId,
    logical_qubits: usize,
) -> ResetResult<()> {
    ResetTarget::new(qubit).validate(logical_qubits)
}

/// Validates a complete reset against a logical namespace.
pub fn validate_reset(
    reset: &Reset,
    logical_qubits: usize,
) -> ResetResult<()> {
    reset.validate_namespace(logical_qubits)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_reset_is_canonical() {
        let reset = Reset::single(QubitId::new(7))
            .expect("single reset must be valid");

        assert_eq!(reset.len(), 1);
        assert!(reset.is_single_target());
        assert!(!reset.is_batch());
        assert_eq!(
            reset.first_target(),
            QubitId::new(7)
        );
        assert_eq!(
            reset.state(),
            ResetState::Zero
        );
        assert_eq!(
            reset.method(),
            ResetMethod::Automatic
        );
    }

    #[test]
    fn empty_reset_is_rejected() {
        let result = Reset::many(Vec::new());

        assert_eq!(
            result,
            Err(ResetError::EmptyTargets)
        );
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let result = Reset::many(vec![
            QubitId::new(1),
            QubitId::new(1),
        ]);

        assert_eq!(
            result,
            Err(ResetError::DuplicateTarget {
                qubit: QubitId::new(1),
            })
        );
    }

    #[test]
    fn duplicate_detection_is_non_mutating() {
        let targets = vec![
            QubitId::new(1),
            QubitId::new(4),
            QubitId::new(1),
        ];

        assert!(Reset::contains_duplicates(&targets));

        assert_eq!(
            targets,
            vec![
                QubitId::new(1),
                QubitId::new(4),
                QubitId::new(1),
            ]
        );
    }

    #[test]
    fn distinct_target_count_is_correct() {
        let targets = vec![
            QubitId::new(1),
            QubitId::new(4),
            QubitId::new(1),
            QubitId::new(8),
        ];

        assert_eq!(
            Reset::distinct_target_count(&targets),
            3
        );
    }

    #[test]
    fn source_order_is_preserved() {
        let reset = Reset::many(vec![
            QubitId::new(9),
            QubitId::new(2),
            QubitId::new(5),
        ])
        .expect("targets are unique");

        assert_eq!(
            reset.targets(),
            &[
                QubitId::new(9),
                QubitId::new(2),
                QubitId::new(5),
            ]
        );
    }

    #[test]
    fn canonical_order_is_deterministic() {
        let reset = Reset::many(vec![
            QubitId::new(9),
            QubitId::new(2),
            QubitId::new(5),
        ])
        .expect("targets are unique");

        assert_eq!(
            reset.canonical_targets(),
            vec![
                QubitId::new(2),
                QubitId::new(5),
                QubitId::new(9),
            ]
        );
    }

    #[test]
    fn canonical_target_equivalence_ignores_source_order() {
        let left = Reset::many(vec![
            QubitId::new(1),
            QubitId::new(7),
            QubitId::new(3),
        ])
        .expect("targets are unique");

        let right = Reset::many(vec![
            QubitId::new(3),
            QubitId::new(1),
            QubitId::new(7),
        ])
        .expect("targets are unique");

        assert!(left.targets_equivalent(&right));
    }

    #[test]
    fn namespace_validation_accepts_valid_targets() {
        let reset = Reset::many(vec![
            QubitId::new(0),
            QubitId::new(3),
            QubitId::new(9),
        ])
        .expect("targets are unique");

        assert!(
            reset.validate_namespace(10).is_ok()
        );
    }

    #[test]
    fn namespace_validation_rejects_out_of_range_target() {
        let reset = Reset::single(QubitId::new(10))
            .expect("construction itself is valid");

        assert_eq!(
            reset.validate_namespace(10),
            Err(ResetError::TargetOutOfRange {
                qubit: QubitId::new(10),
                logical_qubits: 10,
            })
        );
    }

    #[test]
    fn target_validation_is_namespace_relative() {
        let target = ResetTarget::new(QubitId::new(4));

        assert!(
            target.validate(5).is_ok()
        );

        assert_eq!(
            target.validate(4),
            Err(ResetError::TargetOutOfRange {
                qubit: QubitId::new(4),
                logical_qubits: 4,
            })
        );
    }

    #[test]
    fn reset_policy_defaults_to_provider_neutral() {
        let policy = ResetPolicy::canonical();

        assert_eq!(
            policy.state(),
            ResetState::Zero
        );
        assert_eq!(
            policy.method(),
            ResetMethod::Automatic
        );
        assert!(
            policy.allows_equivalent_implementation()
        );
        assert!(
            policy.allows_idempotent_noop()
        );
    }

    #[test]
    fn reset_policy_can_request_native_execution() {
        let policy = ResetPolicy::canonical()
            .native()
            .exact_method();

        assert_eq!(
            policy.method(),
            ResetMethod::Native
        );
        assert!(
            !policy.allows_equivalent_implementation()
        );
    }

    #[test]
    fn reset_policy_can_require_execution() {
        let policy = ResetPolicy::canonical()
            .require_execution();

        assert!(
            !policy.allows_idempotent_noop()
        );
    }

    #[test]
    fn explicit_policy_is_preserved() {
        let policy = ResetPolicy::canonical()
            .measurement_conditional()
            .require_execution();

        let reset = Reset::with_policy(
            QubitId::new(3),
            policy,
        )
        .expect("valid reset");

        assert_eq!(
            reset.method(),
            ResetMethod::MeasurementConditional
        );

        assert!(
            !reset.allows_idempotent_noop()
        );
    }

    #[test]
    fn iterator_constructor_is_supported() {
        let reset = Reset::from_iter(
            (0usize..4usize).map(QubitId::new),
        )
        .expect("unique iterator targets");

        assert_eq!(reset.len(), 4);
    }

    #[test]
    fn single_target_conversion_is_supported() {
        let reset = Reset::single(QubitId::new(12))
            .expect("valid reset");

        assert_eq!(
            reset.single_target(),
            Some(QubitId::new(12))
        );
    }

    #[test]
    fn batch_has_no_single_target() {
        let reset = Reset::many(vec![
            QubitId::new(1),
            QubitId::new(2),
        ])
        .expect("valid batch");

        assert_eq!(
            reset.single_target(),
            None
        );
    }

    #[test]
    fn conversion_to_targets_preserves_order() {
        let reset = Reset::many(vec![
            QubitId::new(8),
            QubitId::new(2),
            QubitId::new(5),
        ])
        .expect("valid reset");

        let targets: Vec<QubitId> = reset.into();

        assert_eq!(
            targets,
            vec![
                QubitId::new(8),
                QubitId::new(2),
                QubitId::new(5),
            ]
        );
    }

    #[test]
    fn display_is_deterministic() {
        let reset = Reset::many(vec![
            QubitId::new(2),
            QubitId::new(5),
        ])
        .expect("valid reset");

        assert_eq!(
            reset.to_string(),
            "reset q2, q5"
        );
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            RESET_SCHEMA_ID,
            "zamani.quantum.ir.quantum.reset"
        );

        assert_eq!(
            RESET_SCHEMA_VERSION,
            1
        );
    }

    #[test]
    fn reset_state_is_only_canonical_zero() {
        assert!(
            ResetState::Zero.is_zero()
        );

        assert_eq!(
            ResetState::Zero.as_str(),
            "zero"
        );
    }

    #[test]
    fn reset_methods_have_stable_names() {
        assert_eq!(
            ResetMethod::Automatic.as_str(),
            "automatic"
        );

        assert_eq!(
            ResetMethod::Native.as_str(),
            "native"
        );

        assert_eq!(
            ResetMethod::MeasurementConditional.as_str(),
            "measurement_conditional"
        );

        assert_eq!(
            ResetMethod::StatePreparation.as_str(),
            "state_preparation"
        );
    }

    #[test]
    fn reset_target_converts_to_qubit_id() {
        let target =
            ResetTarget::new(QubitId::new(42));

        let qubit: QubitId = target.into();

        assert_eq!(
            qubit,
            QubitId::new(42)
        );
    }

    #[test]
    fn reset_can_scale_by_data_without_semantic_ceiling() {
        let count = 10_000usize;

        let reset = Reset::from_iter(
            (0..count).map(QubitId::new),
        )
        .expect("generated targets are unique");

        assert_eq!(
            reset.len(),
            count
        );
    }
}