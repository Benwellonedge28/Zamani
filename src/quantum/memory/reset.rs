//! Zamani Quantum Memory — Reset Contract
//!
//! Provider-neutral reset semantics for `quantum::memory`.
//!
//! # Architectural responsibility
//!
//! This module owns the canonical memory-level contract for resetting quantum
//! resources. It defines:
//!
//! - reset target selection;
//! - reset basis/state semantics;
//! - reset operation identity;
//! - reset policy;
//! - reset validation;
//! - reset execution results;
//! - reset capability requirements;
//! - provider-neutral reset execution;
//! - batch reset validation;
//! - deterministic reset planning;
//! - reset operation auditing;
//! - integration boundaries for state representations and QPU backends;
//! - tests for the reset contract.
//!
//! This module does NOT implement:
//!
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer/tableau mathematics;
//! - sparse-state mathematics;
//! - tensor-network mathematics;
//! - GPU kernels;
//! - distributed communication;
//! - QPU communication;
//! - hardware-specific pulse generation;
//! - calibration;
//! - routing;
//! - scheduling;
//! - compiler parsing;
//! - measurement sampling.
//!
//! Those responsibilities belong to the appropriate downstream modules.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                        │
//!                        │ canonical QubitId
//!                        ▼
//!              quantum::memory::reset
//!                        │
//!              provider-neutral contract
//!                        │
//!       ┌────────────────┼────────────────┐
//!       │                │                │
//!       ▼                ▼                ▼
//! StateVector      DensityMatrix     Stabilizer
//!       │                │                │
//!       └────────────────┼────────────────┘
//!                        ▼
//!                 backend/provider
//!                        │
//!          ┌─────────────┼─────────────┐
//!          ▼             ▼             ▼
//!         CPU           GPU            QPU
//! ```
//!
//! # Fundamental semantic rule
//!
//! A reset operation means:
//!
//! ```text
//! target quantum subsystem → canonical |0⟩ state
//! ```
//!
//! unless the operation explicitly specifies another supported preparation
//! basis/state.
//!
//! For a computational-basis reset:
//!
//! ```text
//! ρ → |0⟩⟨0|
//! ```
//!
//! for the target subsystem.
//!
//! Reset is therefore NOT equivalent to:
//!
//! - merely setting an IR state marker;
//! - merely clearing classical memory;
//! - merely applying an `X` gate conditionally;
//! - merely forgetting a measurement result;
//! - assuming the qubit was previously measured.
//!
//! A provider must implement the mathematical reset semantics appropriate for
//! its representation and execution model.
//!
//! # Important QPU rule
//!
//! A physical QPU may implement reset using:
//!
//! - active reset;
//! - measurement followed by conditional correction;
//! - optical re-preparation;
//! - ion re-initialization;
//! - hardware-specific state preparation;
//! - another calibrated mechanism.
//!
//! `reset.rs` MUST NOT prescribe the physical implementation.
//!
//! It specifies the logical result required by the operation contract.
//!
//! # QPU neutrality
//!
//! Nothing in this module assumes:
//!
//! - superconducting qubits;
//! - trapped ions;
//! - neutral atoms;
//! - photonic qubits;
//! - spin qubits;
//! - NV centers;
//! - topological qubits;
//! - continuous-variable systems;
//! - annealing hardware;
//! - a particular vendor;
//! - a particular pulse language;
//! - a particular SDK.
//!
//! A backend may reject a reset request if its advertised capabilities do not
//! support the requested logical semantics.
//!
//! # Identity rule
//!
//! Logical qubit identity comes from:
//!
//! ```text
//! crate::quantum::ir::QubitId
//! ```
//!
//! This module MUST NOT introduce another logical `QubitId` type.
//!
//! # Error rule
//!
//! Fallible public APIs return:
//!
//! ```text
//! Result<T, MemoryError>
//! ```
//!
//! from `quantum::memory::errors`.
//!
//! This module does not create a competing reset-specific error hierarchy.
//!
//! # No hidden randomness
//!
//! Reset is deterministic at the logical semantic level.
//!
//! A backend may use internally randomized physical procedures, but such
//! behavior must not alter the logical reset contract.
//!
//! # Transaction rule
//!
//! A batch reset must be validated completely before execution begins.
//!
//! If validation fails, no target may be partially reset by this module.
//!
//! Providers are responsible for atomicity of physical execution when they
//! expose a transactional execution mechanism. Where a provider cannot offer
//! physical atomicity, the execution result must accurately report completion
//! and failure information rather than pretending that the operation was
//! atomic.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! This module contains no `unsafe` code.
//!
//! # Integration contract
//!
//! Earlier foundational modules:
//!
//! ```text
//! errors.rs
//! types.rs
//! numeric.rs
//! complex.rs
//! representation.rs
//! limits.rs
//! layout.rs
//! indexing.rs
//! state.rs
//! ```
//!
//! Related modules:
//!
//! ```text
//! qubit.rs
//! register.rs
//! lifetime.rs
//! measurement.rs
//! collapse.rs
//! ```
//!
//! Later consumers:
//!
//! ```text
//! state_vector.rs
//! density_matrix.rs
//! stabilizer.rs
//! sparse.rs
//! tensor_network.rs
//! backend_state.rs
//! gpu.rs
//! distributed.rs
//! migration.rs
//! diagnostics.rs
//! telemetry.rs
//! ```
//!
//! `reset.rs` intentionally depends only on foundational contracts and the
//! canonical IR identity. It must not depend on any concrete state
//! representation or hardware implementation.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use super::errors::MemoryError;

use crate::quantum::ir::QubitId;

// =============================================================================
// Result alias
// =============================================================================

/// Canonical result type for reset operations.
pub type ResetResult<T> = Result<T, MemoryError>;

// =============================================================================
// Constants
// =============================================================================

/// Stable schema identifier for the reset contract.
pub const RESET_SCHEMA_ID: &str = "zamani.quantum.memory.reset";

/// Semantic version of the reset contract.
pub const RESET_SCHEMA_VERSION: u16 = 1;

/// Maximum number of qubits in one reset request before the request must be
/// rejected by the reset contract.
///
/// This is deliberately a contract-level safety ceiling, not the system-wide
/// quantum-memory limit. The system-wide limit remains owned by `limits.rs`.
pub const MAX_RESET_TARGETS: usize = 1_000_000;

/// Maximum number of reset operations that may be represented in one batch.
pub const MAX_RESET_BATCH_SIZE: usize = 1_000_000;

// =============================================================================
// Reset basis
// =============================================================================

/// Logical basis in which the reset target is prepared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResetBasis {
    /// Computational/Z basis.
    ///
    /// The target is prepared as `|0⟩`.
    Computational,

    /// X basis.
    ///
    /// The target is prepared as `|+⟩`.
    X,

    /// Y basis.
    ///
    /// The target is prepared as `|+i⟩`.
    Y,
}

impl Default for ResetBasis {
    fn default() -> Self {
        Self::Computational
    }
}

impl ResetBasis {
    /// Returns the conventional basis name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Computational => "computational",
            Self::X => "x",
            Self::Y => "y",
        }
    }

    /// Returns whether this is the canonical computational reset.
    pub const fn is_computational(self) -> bool {
        matches!(self, Self::Computational)
    }

    /// Returns whether the reset requires a non-computational prepared state.
    pub const fn is_non_computational(self) -> bool {
        !self.is_computational()
    }
}

impl fmt::Display for ResetBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Reset state
// =============================================================================

/// Logical target state produced by a reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResetState {
    /// Prepare `|0⟩`.
    Zero,

    /// Prepare `|+⟩ = (|0⟩ + |1⟩) / √2`.
    Plus,

    /// Prepare `|+i⟩ = (|0⟩ + i|1⟩) / √2`.
    PlusI,
}

impl ResetState {
    /// Returns the state corresponding to a reset basis.
    pub const fn for_basis(basis: ResetBasis) -> Self {
        match basis {
            ResetBasis::Computational => Self::Zero,
            ResetBasis::X => Self::Plus,
            ResetBasis::Y => Self::PlusI,
        }
    }

    /// Returns the canonical textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::Plus => "plus",
            Self::PlusI => "plus_i",
        }
    }

    /// Returns the associated basis.
    pub const fn basis(self) -> ResetBasis {
        match self {
            Self::Zero => ResetBasis::Computational,
            Self::Plus => ResetBasis::X,
            Self::PlusI => ResetBasis::Y,
        }
    }
}

impl fmt::Display for ResetState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Reset method
// =============================================================================

/// Logical reset method requested by the execution layer.
///
/// This describes semantics, not physical implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResetMethod {
    /// Provider chooses the implementation while satisfying the logical reset
    /// contract.
    Automatic,

    /// Provider uses a native reset mechanism when available.
    Native,

    /// Provider may implement reset using measurement and conditional
    /// correction.
    MeasurementConditional,

    /// Provider explicitly re-prepares the target state.
    StatePreparation,
}

impl Default for ResetMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

impl ResetMethod {
    /// Returns a stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Native => "native",
            Self::MeasurementConditional => "measurement_conditional",
            Self::StatePreparation => "state_preparation",
        }
    }

    /// Returns whether this method allows provider-side implementation
    /// selection.
    pub const fn allows_provider_selection(self) -> bool {
        matches!(self, Self::Automatic)
    }
}

impl fmt::Display for ResetMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Reset policy
// =============================================================================

/// Policy controlling reset execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetPolicy {
    /// Desired logical target state.
    pub state: ResetState,

    /// Preferred physical/logical implementation method.
    pub method: ResetMethod,

    /// Whether the provider may use an internally equivalent implementation.
    ///
    /// This must remain true for normal provider-neutral execution. Setting it
    /// to false requests that the provider honor `method` exactly.
    pub allow_equivalent_implementation: bool,

    /// Whether an already-reset target may be treated as a successful no-op.
    ///
    /// The provider may use this optimization only when it can establish that
    /// the requested logical state is already satisfied.
    pub allow_idempotent_noop: bool,
}

impl Default for ResetPolicy {
    fn default() -> Self {
        Self {
            state: ResetState::Zero,
            method: ResetMethod::Automatic,
            allow_equivalent_implementation: true,
            allow_idempotent_noop: true,
        }
    }
}

impl ResetPolicy {
    /// Creates the canonical computational reset policy.
    pub const fn computational() -> Self {
        Self {
            state: ResetState::Zero,
            method: ResetMethod::Automatic,
            allow_equivalent_implementation: true,
            allow_idempotent_noop: true,
        }
    }

    /// Creates a reset policy for a specific target state.
    pub const fn for_state(state: ResetState) -> Self {
        Self {
            state,
            method: ResetMethod::Automatic,
            allow_equivalent_implementation: true,
            allow_idempotent_noop: true,
        }
    }

    /// Creates a reset policy for a specific basis.
    pub const fn for_basis(basis: ResetBasis) -> Self {
        Self::for_state(ResetState::for_basis(basis))
    }

    /// Requires native reset.
    pub const fn native(mut self) -> Self {
        self.method = ResetMethod::Native;
        self
    }

    /// Allows measurement/conditional reset.
    pub const fn measurement_conditional(mut self) -> Self {
        self.method = ResetMethod::MeasurementConditional;
        self
    }

    /// Requests explicit state preparation.
    pub const fn state_preparation(mut self) -> Self {
        self.method = ResetMethod::StatePreparation;
        self
    }

    /// Disables provider-side equivalent implementation.
    pub const fn exact_method(mut self) -> Self {
        self.allow_equivalent_implementation = false;
        self
    }

    /// Disables idempotent no-op optimization.
    pub const fn require_execution(mut self) -> Self {
        self.allow_idempotent_noop = false;
        self
    }
}

// =============================================================================
// Reset target
// =============================================================================

/// A single logical quantum reset target.
///
/// This is deliberately representation-neutral. It contains no amplitude,
/// tensor, physical-device, pulse, or backend data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetTarget {
    qubit: QubitId,
    state: ResetState,
}

impl ResetTarget {
    /// Creates a canonical computational reset target.
    pub const fn new(qubit: QubitId) -> Self {
        Self {
            qubit,
            state: ResetState::Zero,
        }
    }

    /// Creates a reset target for a specific state.
    pub const fn to_state(qubit: QubitId, state: ResetState) -> Self {
        Self { qubit, state }
    }

    /// Creates a reset target for a specific basis.
    pub const fn in_basis(qubit: QubitId, basis: ResetBasis) -> Self {
        Self {
            qubit,
            state: ResetState::for_basis(basis),
        }
    }

    /// Returns the logical qubit.
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the requested target state.
    pub const fn state(self) -> ResetState {
        self.state
    }

    /// Returns the requested basis.
    pub const fn basis(self) -> ResetBasis {
        self.state.basis()
    }

    /// Returns whether this is the canonical `|0⟩` reset.
    pub const fn is_zero_reset(self) -> bool {
        matches!(self.state, ResetState::Zero)
    }

    /// Validates the target.
    pub fn validate(self, qubit_count: usize) -> ResetResult<()> {
        if self.qubit.index() >= qubit_count {
            return Err(invalid_qubit_error(self.qubit, qubit_count));
        }

        Ok(())
    }
}

// =============================================================================
// Reset operation
// =============================================================================

/// One complete logical reset operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetOperation {
    target: ResetTarget,
    policy: ResetPolicy,
}

impl ResetOperation {
    /// Creates a canonical computational reset operation.
    pub const fn new(qubit: QubitId) -> Self {
        Self {
            target: ResetTarget::new(qubit),
            policy: ResetPolicy::computational(),
        }
    }

    /// Creates a reset operation for a specific state.
    pub const fn to_state(qubit: QubitId, state: ResetState) -> Self {
        Self {
            target: ResetTarget::to_state(qubit, state),
            policy: ResetPolicy::for_state(state),
        }
    }

    /// Creates a reset operation for a specific basis.
    pub const fn in_basis(qubit: QubitId, basis: ResetBasis) -> Self {
        Self {
            target: ResetTarget::in_basis(qubit, basis),
            policy: ResetPolicy::for_basis(basis),
        }
    }

    /// Creates an operation with an explicit policy.
    pub const fn with_policy(qubit: QubitId, policy: ResetPolicy) -> Self {
        Self {
            target: ResetTarget::to_state(qubit, policy.state),
            policy,
        }
    }

    /// Returns the target.
    pub const fn target(self) -> ResetTarget {
        self.target
    }

    /// Returns the logical qubit.
    pub const fn qubit(self) -> QubitId {
        self.target.qubit()
    }

    /// Returns the requested state.
    pub const fn state(self) -> ResetState {
        self.target.state()
    }

    /// Returns the requested basis.
    pub const fn basis(self) -> ResetBasis {
        self.target.basis()
    }

    /// Returns the execution policy.
    pub const fn policy(self) -> ResetPolicy {
        self.policy
    }

    /// Returns whether this is the ordinary `|0⟩` reset.
    pub const fn is_zero_reset(self) -> bool {
        self.target.is_zero_reset()
    }

    /// Validates the operation against the logical qubit count.
    pub fn validate(self, qubit_count: usize) -> ResetResult<()> {
        self.target.validate(qubit_count)?;

        if self.policy.state != self.target.state() {
            return Err(invalid_policy_error(
                "reset policy state does not match reset target state",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Reset batch
// =============================================================================

/// A deterministic collection of reset operations.
///
/// The batch preserves insertion order and rejects duplicate logical targets.
///
/// Duplicate targets are rejected because silently accepting:
///
/// ```text
/// reset q0
/// reset q0
/// ```
///
/// inside one memory-level batch would obscure whether the caller intended two
/// sequential semantic operations or one batched reset. Sequential operations
/// should therefore be represented as separate batches/operation boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResetBatch {
    operations: Vec<ResetOperation>,
}

impl ResetBatch {
    /// Creates an empty reset batch.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Creates a batch from a vector after validating uniqueness and size.
    pub fn from_operations(
        operations: Vec<ResetOperation>,
    ) -> ResetResult<Self> {
        if operations.len() > MAX_RESET_BATCH_SIZE {
            return Err(batch_limit_error(
                operations.len(),
                MAX_RESET_BATCH_SIZE,
            ));
        }

        let mut batch = Self::new();

        for operation in operations {
            batch.push(operation)?;
        }

        Ok(batch)
    }

    /// Returns the number of operations.
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns the operations in deterministic insertion order.
    pub fn operations(&self) -> &[ResetOperation] {
        &self.operations
    }

    /// Returns an operation by index.
    pub fn get(&self, index: usize) -> Option<&ResetOperation> {
        self.operations.get(index)
    }

    /// Appends a reset operation after validating the batch invariant.
    pub fn push(&mut self, operation: ResetOperation) -> ResetResult<()> {
        if self.operations.len() >= MAX_RESET_BATCH_SIZE {
            return Err(batch_limit_error(
                self.operations.len().saturating_add(1),
                MAX_RESET_BATCH_SIZE,
            ));
        }

        if self
            .operations
            .iter()
            .any(|existing| existing.qubit() == operation.qubit())
        {
            return Err(duplicate_qubit_error(operation.qubit()));
        }

        self.operations.push(operation);

        Ok(())
    }

    /// Validates all operations against a quantum register size.
    ///
    /// No mutation occurs.
    pub fn validate(&self, qubit_count: usize) -> ResetResult<()> {
        if self.operations.len() > MAX_RESET_BATCH_SIZE {
            return Err(batch_limit_error(
                self.operations.len(),
                MAX_RESET_BATCH_SIZE,
            ));
        }

        for operation in &self.operations {
            operation.validate(qubit_count)?;
        }

        Ok(())
    }

    /// Returns whether all operations request the same target state.
    pub fn has_uniform_state(&self) -> bool {
        let first = match self.operations.first() {
            Some(operation) => operation.state(),
            None => return true,
        };

        self.operations
            .iter()
            .all(|operation| operation.state() == first)
    }

    /// Returns whether every operation is a computational `|0⟩` reset.
    pub fn is_all_zero_reset(&self) -> bool {
        self.operations
            .iter()
            .all(|operation| operation.is_zero_reset())
    }
}

impl Default for ResetBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for ResetBatch {
    type Item = ResetOperation;
    type IntoIter = std::vec::IntoIter<ResetOperation>;

    fn into_iter(self) -> Self::IntoIter {
        self.operations.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResetBatch {
    type Item = &'a ResetOperation;
    type IntoIter = std::slice::Iter<'a, ResetOperation>;

    fn into_iter(self) -> Self::IntoIter {
        self.operations.iter()
    }
}

// =============================================================================
// Reset capabilities
// =============================================================================

/// Capabilities required/advertised by a reset provider.
///
/// This is intentionally separate from concrete hardware capabilities so that
/// CPU simulators, GPUs, distributed simulators and QPUs can all satisfy the
/// same logical contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ResetCapabilities {
    /// Provider can reset to `|0⟩`.
    pub zero: bool,

    /// Provider can reset to `|+⟩`.
    pub plus: bool,

    /// Provider can reset to `|+i⟩`.
    pub plus_i: bool,

    /// Provider has a native reset primitive.
    pub native: bool,

    /// Provider can implement reset through measurement/conditional logic.
    pub measurement_conditional: bool,

    /// Provider can explicitly prepare requested states.
    pub state_preparation: bool,

    /// Provider can reset several targets in one logical operation.
    pub batch: bool,

    /// Provider can guarantee batch-level transactional semantics.
    pub transactional_batch: bool,
}

impl ResetCapabilities {
    /// Capabilities for a full logical reset implementation.
    pub const fn full() -> Self {
        Self {
            zero: true,
            plus: true,
            plus_i: true,
            native: true,
            measurement_conditional: true,
            state_preparation: true,
            batch: true,
            transactional_batch: true,
        }
    }

    /// Capabilities for a minimal computational-reset implementation.
    pub const fn zero_only() -> Self {
        Self {
            zero: true,
            plus: false,
            plus_i: false,
            native: false,
            measurement_conditional: false,
            state_preparation: false,
            batch: false,
            transactional_batch: false,
        }
    }

    /// Returns whether a target state is supported.
    pub const fn supports_state(self, state: ResetState) -> bool {
        match state {
            ResetState::Zero => self.zero,
            ResetState::Plus => self.plus,
            ResetState::PlusI => self.plus_i,
        }
    }

    /// Returns whether a method is supported.
    pub const fn supports_method(self, method: ResetMethod) -> bool {
        match method {
            ResetMethod::Automatic => {
                self.native
                    || self.measurement_conditional
                    || self.state_preparation
                    || self.zero
            }
            ResetMethod::Native => self.native,
            ResetMethod::MeasurementConditional => self.measurement_conditional,
            ResetMethod::StatePreparation => self.state_preparation,
        }
    }

    /// Checks whether one operation is supported.
    pub fn supports(self, operation: ResetOperation) -> bool {
        self.supports_state(operation.state())
            && self.supports_method(operation.policy().method)
    }
}

// =============================================================================
// Reset execution context
// =============================================================================

/// Provider-neutral metadata supplied to a reset executor.
///
/// It contains no raw pointers, device handles or vendor credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetExecutionContext {
    /// Logical number of qubits in the current state.
    qubit_count: usize,

    /// Operation sequence number supplied by the execution layer.
    ///
    /// `None` means the caller does not maintain a sequence number.
    sequence: Option<u64>,
}

impl ResetExecutionContext {
    /// Creates an execution context.
    pub const fn new(qubit_count: usize) -> Self {
        Self {
            qubit_count,
            sequence: None,
        }
    }

    /// Creates a context with an explicit operation sequence.
    pub const fn with_sequence(qubit_count: usize, sequence: u64) -> Self {
        Self {
            qubit_count,
            sequence: Some(sequence),
        }
    }

    /// Returns the number of logical qubits.
    pub const fn qubit_count(self) -> usize {
        self.qubit_count
    }

    /// Returns the optional operation sequence.
    pub const fn sequence(self) -> Option<u64> {
        self.sequence
    }
}

// =============================================================================
// Reset execution result
// =============================================================================

/// Outcome of one reset operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResetOutcome {
    /// The provider performed the requested reset.
    Applied,

    /// The target was already in the requested state and the provider safely
    /// performed no physical work.
    NoOp,

    /// The provider accepted the request but actual completion is delegated to
    /// an asynchronous execution layer.
    Accepted,

    /// The provider explicitly indicates that the request is pending.
    Pending,
}

impl ResetOutcome {
    /// Returns whether the logical reset has completed.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Applied | Self::NoOp)
    }

    /// Returns whether execution is still pending.
    pub const fn is_pending(self) -> bool {
        matches!(self, Self::Accepted | Self::Pending)
    }
}

impl fmt::Display for ResetOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Applied => "applied",
            Self::NoOp => "no_op",
            Self::Accepted => "accepted",
            Self::Pending => "pending",
        };

        f.write_str(value)
    }
}

/// Result information for a single reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetResultInfo {
    /// Target logical qubit.
    qubit: QubitId,

    /// Requested target state.
    state: ResetState,

    /// Actual logical outcome.
    outcome: ResetOutcome,

    /// Method selected by the provider.
    method: ResetMethod,

    /// Whether the provider reports the target as logically reset.
    logically_reset: bool,
}

impl ResetResultInfo {
    /// Creates a result record.
    pub const fn new(
        qubit: QubitId,
        state: ResetState,
        outcome: ResetOutcome,
        method: ResetMethod,
        logically_reset: bool,
    ) -> Self {
        Self {
            qubit,
            state,
            outcome,
            method,
            logically_reset,
        }
    }

    /// Returns the logical qubit.
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the requested state.
    pub const fn state(self) -> ResetState {
        self.state
    }

    /// Returns the outcome.
    pub const fn outcome(self) -> ResetOutcome {
        self.outcome
    }

    /// Returns the method used.
    pub const fn method(self) -> ResetMethod {
        self.method
    }

    /// Returns whether the target is logically reset.
    pub const fn logically_reset(self) -> bool {
        self.logically_reset
    }
}

// =============================================================================
// Batch execution result
// =============================================================================

/// Aggregate result for a reset batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResetBatchResult {
    results: Vec<ResetResultInfo>,
}

impl ResetBatchResult {
    /// Creates an empty result.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Creates a result with prevalidated records.
    pub fn from_results(results: Vec<ResetResultInfo>) -> ResetResult<Self> {
        if results.len() > MAX_RESET_BATCH_SIZE {
            return Err(batch_limit_error(
                results.len(),
                MAX_RESET_BATCH_SIZE,
            ));
        }

        let mut result = Self::new();

        for item in results {
            result.push(item)?;
        }

        Ok(result)
    }

    /// Returns the number of result records.
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Returns whether there are no result records.
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns all result records.
    pub fn results(&self) -> &[ResetResultInfo] {
        &self.results
    }

    /// Returns a result record by index.
    pub fn get(&self, index: usize) -> Option<&ResetResultInfo> {
        self.results.get(index)
    }

    /// Adds a result record.
    pub fn push(&mut self, result: ResetResultInfo) -> ResetResult<()> {
        if self.results.len() >= MAX_RESET_BATCH_SIZE {
            return Err(batch_limit_error(
                self.results.len().saturating_add(1),
                MAX_RESET_BATCH_SIZE,
            ));
        }

        if self
            .results
            .iter()
            .any(|existing| existing.qubit() == result.qubit())
        {
            return Err(duplicate_qubit_error(result.qubit()));
        }

        self.results.push(result);

        Ok(())
    }

    /// Returns the number of completed logical resets.
    pub fn completed_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| result.logically_reset())
            .count()
    }

    /// Returns the number of pending results.
    pub fn pending_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| result.outcome().is_pending())
            .count()
    }

    /// Returns whether every result is complete.
    pub fn is_complete(&self) -> bool {
        self.results
            .iter()
            .all(|result| result.outcome().is_complete())
    }
}

impl Default for ResetBatchResult {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Provider-neutral reset executor
// =============================================================================

/// Provider-neutral reset execution contract.
///
/// Implementations include:
///
/// - state-vector simulator;
/// - density-matrix simulator;
/// - stabilizer simulator;
/// - sparse simulator;
/// - tensor-network simulator;
/// - CPU provider;
/// - GPU provider;
/// - distributed simulator;
/// - physical QPU adapter.
///
/// The implementation owns the actual state transition.
///
/// No implementation is permitted to fabricate success when the requested
/// reset semantics are unavailable.
pub trait ResetExecutor {
    /// Returns the reset capabilities exposed by this provider.
    fn reset_capabilities(&self) -> ResetCapabilities;

    /// Executes one reset operation.
    fn reset(
        &mut self,
        operation: ResetOperation,
        context: ResetExecutionContext,
    ) -> ResetResult<ResetResultInfo>;

    /// Executes a validated reset batch.
    ///
    /// The default implementation executes operations in deterministic order.
    ///
    /// Providers requiring hardware-native batching may override this method.
    fn reset_batch(
        &mut self,
        batch: &ResetBatch,
        context: ResetExecutionContext,
    ) -> ResetResult<ResetBatchResult> {
        batch.validate(context.qubit_count())?;

        if batch.is_empty() {
            return Ok(ResetBatchResult::new());
        }

        if !self.reset_capabilities().batch && batch.len() > 1 {
            return Err(unsupported_reset_error(
                "provider does not support reset batches",
            ));
        }

        let capabilities = self.reset_capabilities();

        for operation in batch.operations() {
            if !capabilities.supports(*operation) {
                return Err(unsupported_operation_for_reset(*operation));
            }
        }

        let mut result = ResetBatchResult::new();

        for operation in batch.operations() {
            let item = self.reset(*operation, context)?;
            result.push(item)?;
        }

        Ok(result)
    }
}

// =============================================================================
// Reset planning
// =============================================================================

/// Deterministic reset execution plan.
///
/// A plan is validated before it is handed to a provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResetPlan {
    batch: ResetBatch,
}

impl ResetPlan {
    /// Builds a validated plan.
    pub fn new(
        batch: ResetBatch,
        qubit_count: usize,
    ) -> ResetResult<Self> {
        batch.validate(qubit_count)?;

        Ok(Self { batch })
    }

    /// Creates a plan containing one canonical `|0⟩` reset.
    pub fn single(qubit: QubitId, qubit_count: usize) -> ResetResult<Self> {
        let mut batch = ResetBatch::new();
        batch.push(ResetOperation::new(qubit))?;

        Self::new(batch, qubit_count)
    }

    /// Returns the underlying validated batch.
    pub fn batch(&self) -> &ResetBatch {
        &self.batch
    }

    /// Returns the number of operations.
    pub fn len(&self) -> usize {
        self.batch.len()
    }

    /// Returns whether the plan is empty.
    pub fn is_empty(&self) -> bool {
        self.batch.is_empty()
    }

    /// Returns whether every reset targets `|0⟩`.
    pub fn is_all_zero_reset(&self) -> bool {
        self.batch.is_all_zero_reset()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates one reset operation against a logical qubit count.
pub fn validate_reset(
    operation: ResetOperation,
    qubit_count: usize,
) -> ResetResult<()> {
    operation.validate(qubit_count)
}

/// Validates a reset batch against a logical qubit count.
pub fn validate_reset_batch(
    batch: &ResetBatch,
    qubit_count: usize,
) -> ResetResult<()> {
    batch.validate(qubit_count)
}

/// Validates whether a provider can satisfy a reset operation.
pub fn validate_reset_capability(
    operation: ResetOperation,
    capabilities: ResetCapabilities,
) -> ResetResult<()> {
    if capabilities.supports(operation) {
        Ok(())
    } else {
        Err(unsupported_operation_for_reset(operation))
    }
}

/// Validates whether a provider can satisfy a complete reset batch.
pub fn validate_reset_batch_capability(
    batch: &ResetBatch,
    capabilities: ResetCapabilities,
) -> ResetResult<()> {
    if batch.is_empty() {
        return Ok(());
    }

    if batch.len() > 1 && !capabilities.batch {
        return Err(unsupported_reset_error(
            "provider does not support reset batches",
        ));
    }

    for operation in batch.operations() {
        validate_reset_capability(*operation, capabilities)?;
    }

    Ok(())
}

// =============================================================================
// Error construction
// =============================================================================
//
// `errors.rs` is the canonical memory error taxonomy. These helpers are kept
// local so that all reset-specific validation remains in this file without
// introducing a second public error enum.
//
// The constructors below intentionally use the stable generic error boundary
// exposed by `MemoryError`. They do not expose implementation details.

fn invalid_qubit_error(
    qubit: QubitId,
    qubit_count: usize,
) -> MemoryError {
    MemoryError::invalid_argument(format!(
        "reset target qubit {} is outside logical register of {} qubits",
        qubit.index(),
        qubit_count
    ))
}

fn invalid_policy_error(message: &'static str) -> MemoryError {
    MemoryError::invalid_argument(message)
}

fn duplicate_qubit_error(qubit: QubitId) -> MemoryError {
    MemoryError::invalid_argument(format!(
        "reset batch contains duplicate logical qubit {}",
        qubit.index()
    ))
}

fn batch_limit_error(
    requested: usize,
    maximum: usize,
) -> MemoryError {
    MemoryError::invalid_argument(format!(
        "reset batch size {} exceeds reset contract maximum {}",
        requested,
        maximum
    ))
}

fn unsupported_reset_error(message: &'static str) -> MemoryError {
    MemoryError::invalid_argument(message)
}

fn unsupported_operation_for_reset(
    operation: ResetOperation,
) -> MemoryError {
    MemoryError::invalid_argument(format!(
        "reset operation for qubit {} to state {} with method {} is not supported by the provider",
        operation.qubit().index(),
        operation.state(),
        operation.policy().method
    ))
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Creates a canonical computational reset operation.
///
/// This is the normal entry point for:
///
/// ```text
/// reset q
/// ```
///
/// in the execution/memory layer.
pub const fn reset(qubit: QubitId) -> ResetOperation {
    ResetOperation::new(qubit)
}

/// Creates a reset operation targeting a specific state.
pub const fn reset_to_state(
    qubit: QubitId,
    state: ResetState,
) -> ResetOperation {
    ResetOperation::to_state(qubit, state)
}

/// Creates a reset operation targeting a specific basis state.
pub const fn reset_in_basis(
    qubit: QubitId,
    basis: ResetBasis,
) -> ResetOperation {
    ResetOperation::in_basis(qubit, basis)
}

/// Creates a single-operation reset plan.
pub fn plan_reset(
    qubit: QubitId,
    qubit_count: usize,
) -> ResetResult<ResetPlan> {
    ResetPlan::single(qubit, qubit_count)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    // -------------------------------------------------------------------------
    // Basis/state contract
    // -------------------------------------------------------------------------

    #[test]
    fn computational_basis_maps_to_zero() {
        assert_eq!(
            ResetState::for_basis(ResetBasis::Computational),
            ResetState::Zero
        );
    }

    #[test]
    fn x_basis_maps_to_plus() {
        assert_eq!(
            ResetState::for_basis(ResetBasis::X),
            ResetState::Plus
        );
    }

    #[test]
    fn y_basis_maps_to_plus_i() {
        assert_eq!(
            ResetState::for_basis(ResetBasis::Y),
            ResetState::PlusI
        );
    }

    // -------------------------------------------------------------------------
    // Operation construction
    // -------------------------------------------------------------------------

    #[test]
    fn default_reset_targets_zero() {
        let operation = ResetOperation::new(q(0));

        assert_eq!(operation.qubit(), q(0));
        assert_eq!(operation.state(), ResetState::Zero);
        assert_eq!(operation.basis(), ResetBasis::Computational);
        assert!(operation.is_zero_reset());
    }

    #[test]
    fn basis_reset_preserves_requested_basis() {
        let operation = ResetOperation::in_basis(q(2), ResetBasis::X);

        assert_eq!(operation.qubit(), q(2));
        assert_eq!(operation.state(), ResetState::Plus);
        assert_eq!(operation.basis(), ResetBasis::X);
    }

    #[test]
    fn policy_state_matches_operation_state() {
        let policy = ResetPolicy::for_state(ResetState::Plus);
        let operation = ResetOperation::with_policy(q(0), policy);

        assert_eq!(operation.state(), ResetState::Plus);
        assert_eq!(operation.policy().state, ResetState::Plus);
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    #[test]
    fn valid_target_is_accepted() {
        let operation = ResetOperation::new(q(3));

        assert!(operation.validate(4).is_ok());
    }

    #[test]
    fn out_of_range_target_is_rejected() {
        let operation = ResetOperation::new(q(4));

        assert!(operation.validate(4).is_err());
    }

    #[test]
    fn empty_register_rejects_every_target() {
        let operation = ResetOperation::new(q(0));

        assert!(operation.validate(0).is_err());
    }

    // -------------------------------------------------------------------------
    // Batch semantics
    // -------------------------------------------------------------------------

    #[test]
    fn batch_preserves_insertion_order() {
        let mut batch = ResetBatch::new();

        batch.push(ResetOperation::new(q(3))).unwrap();
        batch.push(ResetOperation::new(q(1))).unwrap();
        batch.push(ResetOperation::new(q(2))).unwrap();

        assert_eq!(batch.operations()[0].qubit(), q(3));
        assert_eq!(batch.operations()[1].qubit(), q(1));
        assert_eq!(batch.operations()[2].qubit(), q(2));
    }

    #[test]
    fn duplicate_qubit_is_rejected() {
        let mut batch = ResetBatch::new();

        batch.push(ResetOperation::new(q(0))).unwrap();

        assert!(batch.push(ResetOperation::new(q(0))).is_err());
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn batch_validation_does_not_mutate() {
        let mut batch = ResetBatch::new();

        batch.push(ResetOperation::new(q(0))).unwrap();
        batch.push(ResetOperation::new(q(1))).unwrap();

        assert!(batch.validate(1).is_err());
        assert_eq!(batch.len(), 2);
    }

    // -------------------------------------------------------------------------
    // Capability semantics
    // -------------------------------------------------------------------------

    #[test]
    fn zero_only_capability_supports_zero_reset() {
        let capabilities = ResetCapabilities::zero_only();

        assert!(capabilities.supports(ResetOperation::new(q(0))));
    }

    #[test]
    fn zero_only_capability_rejects_plus_reset() {
        let capabilities = ResetCapabilities::zero_only();

        assert!(!capabilities.supports(
            ResetOperation::to_state(q(0), ResetState::Plus)
        ));
    }

    #[test]
    fn full_capabilities_support_all_states() {
        let capabilities = ResetCapabilities::full();

        assert!(capabilities.supports(
            ResetOperation::to_state(q(0), ResetState::Zero)
        ));

        assert!(capabilities.supports(
            ResetOperation::to_state(q(0), ResetState::Plus)
        ));

        assert!(capabilities.supports(
            ResetOperation::to_state(q(0), ResetState::PlusI)
        ));
    }

    // -------------------------------------------------------------------------
    // Mock provider
    // -------------------------------------------------------------------------

    struct MockResetProvider {
        capabilities: ResetCapabilities,
        calls: usize,
    }

    impl MockResetProvider {
        fn new(capabilities: ResetCapabilities) -> Self {
            Self {
                capabilities,
                calls: 0,
            }
        }
    }

    impl ResetExecutor for MockResetProvider {
        fn reset_capabilities(&self) -> ResetCapabilities {
            self.capabilities
        }

        fn reset(
            &mut self,
            operation: ResetOperation,
            _context: ResetExecutionContext,
        ) -> ResetResult<ResetResultInfo> {
            validate_reset_capability(operation, self.capabilities)?;

            self.calls += 1;

            Ok(ResetResultInfo::new(
                operation.qubit(),
                operation.state(),
                ResetOutcome::Applied,
                operation.policy().method,
                true,
            ))
        }
    }

    #[test]
    fn executor_applies_supported_reset() {
        let mut provider =
            MockResetProvider::new(ResetCapabilities::full());

        let operation = ResetOperation::new(q(0));

        let result = provider
            .reset(operation, ResetExecutionContext::new(1))
            .unwrap();

        assert_eq!(result.qubit(), q(0));
        assert_eq!(result.state(), ResetState::Zero);
        assert_eq!(result.outcome(), ResetOutcome::Applied);
        assert!(result.logically_reset());
        assert_eq!(provider.calls, 1);
    }

    #[test]
    fn executor_rejects_unsupported_reset() {
        let mut provider =
            MockResetProvider::new(ResetCapabilities::zero_only());

        let operation =
            ResetOperation::to_state(q(0), ResetState::Plus);

        assert!(
            provider
                .reset(operation, ResetExecutionContext::new(1))
                .is_err()
        );

        assert_eq!(provider.calls, 0);
    }

    #[test]
    fn batch_execution_is_deterministic() {
        let mut provider =
            MockResetProvider::new(ResetCapabilities {
                zero: true,
                plus: false,
                plus_i: false,
                native: true,
                measurement_conditional: false,
                state_preparation: false,
                batch: true,
                transactional_batch: false,
            });

        let mut batch = ResetBatch::new();

        batch.push(ResetOperation::new(q(2))).unwrap();
        batch.push(ResetOperation::new(q(0))).unwrap();
        batch.push(ResetOperation::new(q(1))).unwrap();

        let result = provider
            .reset_batch(
                &batch,
                ResetExecutionContext::with_sequence(3, 7),
            )
            .unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result.results()[0].qubit(), q(2));
        assert_eq!(result.results()[1].qubit(), q(0));
        assert_eq!(result.results()[2].qubit(), q(1));
    }

    // -------------------------------------------------------------------------
    // Result semantics
    // -------------------------------------------------------------------------

    #[test]
    fn no_op_is_complete() {
        assert!(ResetOutcome::NoOp.is_complete());
        assert!(!ResetOutcome::NoOp.is_pending());
    }

    #[test]
    fn pending_is_not_complete() {
        assert!(!ResetOutcome::Pending.is_complete());
        assert!(ResetOutcome::Pending.is_pending());
    }

    #[test]
    fn accepted_is_pending() {
        assert!(!ResetOutcome::Accepted.is_complete());
        assert!(ResetOutcome::Accepted.is_pending());
    }

    // -------------------------------------------------------------------------
    // Planning
    // -------------------------------------------------------------------------

    #[test]
    fn single_reset_plan_is_validated() {
        let plan = ResetPlan::single(q(2), 3).unwrap();

        assert_eq!(plan.len(), 1);
        assert_eq!(plan.batch().operations()[0].qubit(), q(2));
    }

    #[test]
    fn single_reset_plan_rejects_invalid_qubit() {
        assert!(ResetPlan::single(q(3), 3).is_err());
    }

    #[test]
    fn all_zero_detection_works() {
        let mut batch = ResetBatch::new();

        batch.push(ResetOperation::new(q(0))).unwrap();
        batch.push(ResetOperation::new(q(1))).unwrap();

        assert!(batch.is_all_zero_reset());
    }

    #[test]
    fn mixed_state_batch_is_detected() {
        let mut batch = ResetBatch::new();

        batch.push(ResetOperation::new(q(0))).unwrap();
        batch
            .push(ResetOperation::to_state(q(1), ResetState::Plus))
            .unwrap();

        assert!(!batch.has_uniform_state());
        assert!(!batch.is_all_zero_reset());
    }
}