//! Zamani Quantum Memory — Provider-Neutral Copy-on-Write State Handles
//!
//! `copy_on_write.rs` provides the ownership layer used when multiple logical
//! execution branches need to refer to the same quantum-state resource while
//! delaying a provider-level state fork until a branch actually mutates it.
//!
//! # Critical quantum rule
//!
//! This module implements software ownership semantics, not physical quantum
//! cloning.
//!
//! Cloning [`CopyOnWriteState`] only clones a Rust handle. It does not copy the
//! quantum state and it does not create a second physical QPU state.
//!
//! When a shared handle is mutated, this module asks the underlying
//! [`QuantumState`] implementation to perform its provider-defined `fork()`.
//! Providers that cannot fork their state, including many live QPU/backend
//! resources, therefore return their canonical [`MemoryError`].
//!
//! This distinction is mandatory because arbitrary unknown quantum states
//! cannot be physically cloned. The no-cloning theorem is a physical
//! constraint; this module never attempts to bypass it.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! execution / runtime
//!      │
//!      ▼
//! quantum::memory::state
//!      │
//!      ▼
//! CopyOnWriteState
//!      │
//!      ├── shared immutable handle
//!      │
//!      └── first mutation
//!             │
//!             ├── unique owner → mutate in place
//!             │
//!             └── shared owner → QuantumState::fork()
//!                                  │
//!                                  ├── simulator/state representation
//!                                  │      → independent fork when supported
//!                                  │
//!                                  └── QPU/backend-native state
//!                                         → provider-defined result/error
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - shared ownership of provider-neutral quantum states;
//! - cheap logical branching;
//! - lazy provider-level state forking;
//! - safe mutation boundaries;
//! - thread-safe handle access;
//! - ownership diagnostics;
//! - read-only state inspection;
//! - provider-neutral operation application;
//! - explicit eager forking;
//! - distinction between software branching and quantum cloning.
//!
//! It does NOT own:
//!
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer mathematics;
//! - sparse-state algorithms;
//! - tensor-network algorithms;
//! - GPU APIs;
//! - distributed communication;
//! - QPU vendor APIs;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - compiler syntax;
//! - measurement randomness.
//!
//! Those remain in their respective subsystems.
//!
//! # Integration contract
//!
//! This module is intentionally implementable after the foundational contracts
//! in:
//!
//! - `errors.rs`;
//! - `types.rs`;
//! - `state.rs`.
//!
//! It does not require later modules such as `snapshot.rs`, `migration.rs`,
//! `gpu.rs`, or `distributed.rs`.
//!
//! Later modules can consume this contract without modifying this file:
//!
//! - `measurement.rs` can use `with_mut()` for branch-local collapse/reset;
//! - `migration.rs` can use `with_mut()` for branch-local migration;
//! - `snapshot.rs` can use `with_state()` without detaching;
//! - `checkpoint.rs` can retain independent logical handles;
//! - `diagnostics.rs` can inspect sharing and metadata;
//! - runtime/executor code can use `apply_operation()`;
//! - simulators can use cheap branches for speculative execution;
//! - QEC can use branches where its representation supports provider-level
//!   forking;
//! - backend-native QPU states remain valid because unsupported fork operations
//!   are propagated rather than fabricated.
//!
//! # Concurrency model
//!
//! `CopyOnWriteState` is `Send + Sync` because the canonical `QuantumState`
//! trait is `Send + Sync` and access is protected by `RwLock`.
//!
//! The lock protects the Rust provider object. It does not replace provider-
//! specific GPU, QPU, distributed-memory, or backend synchronization.
//!
//! Closure-based access is intentional. No public API returns a long-lived
//! mutable reference to the provider object.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! # Memory behavior
//!
//! Handle cloning is O(1) with respect to the number of quantum-state elements.
//! A provider-level state fork occurs only when a shared handle is mutated.
//!
//! The underlying provider remains responsible for checking memory limits
//! before performing the fork. This module never assumes that a fork is cheap.
//! A state-vector fork can be exponentially expensive in qubit count.
//!
//! # Determinism
//!
//! This module creates no RNG and performs no measurement. Measurement
//! randomness remains owned by the measurement/execution layer.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::sync::{Arc, RwLock};

use super::errors::MemoryError;
use super::state::{
    ComplexAmplitude,
    QuantumState,
    StateCapabilities,
    StateConsistency,
    StateExecutionDomain,
    StateLifecycle,
    StateMetadata,
    StateOperation,
    StateOperationResult,
    StateProbability,
    StateResult,
    StateStorageLocation,
};
use super::types::{ByteCount, QubitCount, StateId};

/// Internal shared state container.
///
/// The quantum-state object itself remains owned by this container. The
/// `RwLock` provides safe concurrent read access and exclusive provider-object
/// mutation.
struct SharedState {
    state: Box<dyn QuantumState>,
}

impl SharedState {
    fn new(state: Box<dyn QuantumState>) -> Self {
        Self { state }
    }
}

/// Thread-safe clone-on-write handle around a provider-neutral quantum state.
///
/// # Important
///
/// `CopyOnWriteState::clone()` does **not** clone the quantum state.
///
/// It creates another logical Rust handle pointing at the same state resource.
///
/// If multiple handles exist and one handle needs to mutate its state, the
/// provider's [`QuantumState::fork`] implementation is called first.
///
/// Therefore:
///
/// ```text
/// let branch_a = state.clone();
/// ```
///
/// means:
///
/// ```text
/// shared software handle
/// ```
///
/// while:
///
/// ```text
/// branch_a.apply_operation(...)
/// ```
///
/// may cause:
///
/// ```text
/// provider state fork
///        ↓
/// branch_a gets independent provider state
///        ↓
/// operation executes on branch_a only
/// ```
///
/// A provider that cannot fork returns its canonical `MemoryError`.
#[derive(Clone)]
pub struct CopyOnWriteState {
    inner: Arc<RwLock<SharedState>>,
}

impl CopyOnWriteState {
    /// Wraps an existing provider-neutral state.
    ///
    /// The state is validated before being exposed through the COW handle.
    pub fn new(state: Box<dyn QuantumState>) -> StateResult<Self> {
        state.validate_invariants()?;

        Ok(Self {
            inner: Arc::new(RwLock::new(SharedState::new(state))),
        })
    }

    /// Returns the number of logical Rust handles sharing this state resource.
    ///
    /// This is an ownership diagnostic only. It is not the number of quantum
    /// states, QPUs, physical qubits, or backend replicas.
    pub fn strong_handle_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    /// Returns whether this handle is the sole logical owner.
    pub fn is_unique(&self) -> bool {
        Arc::strong_count(&self.inner) == 1
    }

    /// Returns whether this handle shares its state with another handle.
    pub fn is_shared(&self) -> bool {
        !self.is_unique()
    }

    /// Executes a read-only closure against the underlying quantum state.
    ///
    /// The closure cannot mutate the provider and cannot retain the borrowed
    /// state beyond the call.
    pub fn with_state<R>(
        &self,
        operation: impl FnOnce(&dyn QuantumState) -> R,
    ) -> StateResult<R> {
        let guard = self.read_lock()?;
        Ok(operation(guard.state.as_ref()))
    }

    /// Returns a cloned metadata value.
    ///
    /// This clones metadata only. It never clones the quantum-state payload.
    pub fn metadata(&self) -> StateResult<StateMetadata> {
        self.with_state(|state| state.metadata().clone())
    }

    /// Returns the provider-neutral state identifier.
    pub fn state_id(&self) -> StateResult<StateId> {
        self.with_state(|state| state.state_id())
    }

    /// Returns the number of logical qubits represented.
    pub fn qubit_count(&self) -> StateResult<QubitCount> {
        self.with_state(|state| state.qubit_count())
    }

    /// Returns the currently declared memory consumption.
    pub fn memory_bytes(&self) -> StateResult<ByteCount> {
        self.with_state(|state| state.memory_bytes())
    }

    /// Returns the representation name.
    pub fn representation(&self) -> StateResult<String> {
        self.with_state(|state| state.representation().to_owned())
    }

    /// Returns the storage location.
    pub fn storage_location(&self) -> StateResult<StateStorageLocation> {
        self.with_state(|state| state.storage_location())
    }

    /// Returns the execution domain.
    pub fn execution_domain(&self) -> StateResult<StateExecutionDomain> {
        self.with_state(|state| state.execution_domain())
    }

    /// Returns the provider's capability set.
    pub fn capabilities(&self) -> StateResult<StateCapabilities> {
        self.with_state(|state| state.capabilities())
    }

    /// Returns whether the provider advertises provider-level state forking.
    ///
    /// This does not perform a fork.
    pub fn can_fork(&self) -> StateResult<bool> {
        self.with_state(|state| {
            state
                .capabilities()
                .contains(StateCapabilities::FORK)
        })
    }

    /// Returns the current state lifecycle.
    pub fn lifecycle(&self) -> StateResult<StateLifecycle> {
        self.with_state(|state| state.lifecycle())
    }

    /// Returns the current state consistency status.
    pub fn consistency(&self) -> StateResult<StateConsistency> {
        self.with_state(|state| state.consistency())
    }

    /// Returns whether the underlying state is ready for execution.
    pub fn is_ready(&self) -> StateResult<bool> {
        self.with_state(|state| state.is_ready())
    }

    /// Returns whether the underlying state is backend/QPU-native.
    pub fn is_backend_native(&self) -> StateResult<bool> {
        self.with_state(|state| state.is_backend_native())
    }

    /// Reads an amplitude when the provider exposes amplitude semantics.
    ///
    /// `Ok(None)` means the representation/provider does not expose amplitudes.
    /// It does not mean that the amplitude is zero.
    pub fn amplitude(
        &self,
        basis_index: usize,
    ) -> StateResult<Option<ComplexAmplitude>> {
        self.with_state(|state| state.amplitude(basis_index))?
    }

    /// Reads a computational-basis probability when supported.
    pub fn probability(
        &self,
        basis_index: usize,
    ) -> StateResult<Option<StateProbability>> {
        self.with_state(|state| state.probability(basis_index))?
    }

    /// Validates an operation without detaching or mutating the state.
    pub fn validate_operation(
        &self,
        operation: &dyn StateOperation,
    ) -> StateResult<()> {
        self.with_state(|state| state.validate_operation(operation))?
    }

    /// Applies a provider-neutral state operation using clone-on-write
    /// semantics.
    ///
    /// The operation is validated before detachment so an invalid operation
    /// never causes an unnecessary state fork.
    ///
    /// Once shared ownership requires detachment:
    ///
    /// 1. the provider is asked to fork;
    /// 2. the new state is invariant-validated;
    /// 3. the fork replaces this handle's shared resource;
    /// 4. the operation is validated again against the private state;
    /// 5. the operation executes;
    /// 6. state invariants are validated after execution.
    ///
    /// If the provider cannot fork, the original shared state remains
    /// untouched and the provider's error is returned.
    pub fn apply_operation(
        &mut self,
        operation: &dyn StateOperation,
    ) -> StateResult<StateOperationResult> {
        self.validate_operation(operation)?;
        self.ensure_private()?;

        let mut guard = self.write_lock()?;

        guard.state.validate_operation(operation)?;

        let result = guard.state.apply_operation(operation)?;

        guard.state.validate_invariants()?;

        Ok(result)
    }

    /// Provides exclusive mutable access with clone-on-write semantics.
    ///
    /// If this handle is shared, the provider-level fork occurs before the
    /// closure runs.
    ///
    /// If the closure fails, the detached branch remains owned by this handle,
    /// while the original branch remains unchanged.
    ///
    /// If the closure succeeds, invariants are checked before the result is
    /// returned.
    pub fn with_mut<R>(
        &mut self,
        operation: impl FnOnce(&mut dyn QuantumState) -> StateResult<R>,
    ) -> StateResult<R> {
        self.ensure_private()?;

        let mut guard = self.write_lock()?;

        let result = operation(guard.state.as_mut());

        match result {
            Ok(value) => {
                guard.state.validate_invariants()?;
                Ok(value)
            }
            Err(error) => {
                // A provider implementation must not leave a state in an
                // invalid invariant state after reporting an operation error.
                // Validate nevertheless so corruption is not silently carried
                // forward.
                guard.state.validate_invariants()?;
                Err(error)
            }
        }
    }

    /// Forces this handle to become independent.
    ///
    /// If it is already unique, this is a no-op.
    ///
    /// If it is shared, the underlying provider's `fork()` operation is used.
    pub fn detach(&mut self) -> StateResult<()> {
        self.ensure_private()
    }

    /// Creates a cheap logical execution branch.
    ///
    /// This is intentionally equivalent to `Clone`, and does not invoke
    /// `QuantumState::fork()`.
    pub fn branch(&self) -> Self {
        self.clone()
    }

    /// Eagerly asks the provider to create an independent state resource.
    ///
    /// Unlike [`Self::branch`], this can allocate substantial resources and can
    /// fail for backend-native/QPU states.
    pub fn fork_now(&self) -> StateResult<Self> {
        let guard = self.read_lock()?;

        let forked = guard.state.fork()?;

        drop(guard);

        Self::new(forked)
    }

    /// Returns whether two handles refer to the same software-owned state
    /// resource.
    ///
    /// This does not compare quantum-state amplitudes or physical quantum
    /// states.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    /// Alias for [`Self::with_state`] intended for infrastructure/diagnostic
    /// code.
    pub fn read<R>(
        &self,
        operation: impl FnOnce(&dyn QuantumState) -> R,
    ) -> StateResult<R> {
        self.with_state(operation)
    }

    /// Returns the current logical handle count.
    pub fn handle_count(&self) -> usize {
        self.strong_handle_count()
    }

    /// Performs the provider-level detachment required before mutation.
    ///
    /// This method is private so all public mutation paths share exactly the
    /// same ownership semantics.
    fn ensure_private(&mut self) -> StateResult<()> {
        if self.is_unique() {
            return Ok(());
        }

        let forked = {
            let guard = self.read_lock()?;
            guard.state.fork()?
        };

        forked.validate_invariants()?;

        self.inner = Arc::new(RwLock::new(SharedState::new(forked)));

        Ok(())
    }

    /// Acquires a read lock and converts poisoning into the canonical memory
    /// error model.
    fn read_lock(
        &self,
    ) -> StateResult<std::sync::RwLockReadGuard<'_, SharedState>> {
        self.inner.read().map_err(|_| {
            MemoryError::invariant_violation(
                "quantum memory copy-on-write state lock was poisoned",
            )
        })
    }

    /// Acquires a write lock and converts poisoning into the canonical memory
    /// error model.
    fn write_lock(
        &self,
    ) -> StateResult<std::sync::RwLockWriteGuard<'_, SharedState>> {
        self.inner.write().map_err(|_| {
            MemoryError::invariant_violation(
                "quantum memory copy-on-write state lock was poisoned",
            )
        })
    }
}

impl std::fmt::Debug for CopyOnWriteState {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let handle_count = self.strong_handle_count();

        match self.metadata() {
            Ok(metadata) => formatter
                .debug_struct("CopyOnWriteState")
                .field("state_id", &metadata.state_id)
                .field("representation", &metadata.representation)
                .field("qubit_count", &metadata.qubit_count)
                .field("memory_bytes", &metadata.memory_bytes)
                .field("storage_location", &metadata.storage_location)
                .field("execution_domain", &metadata.execution_domain)
                .field("handle_count", &handle_count)
                .finish(),

            Err(_) => formatter
                .debug_struct("CopyOnWriteState")
                .field("handle_count", &handle_count)
                .field("state", &"unavailable")
                .finish(),
        }
    }
}

/// Describes what a mutation would require from the COW handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyOnWriteAction {
    /// The handle is unique and can mutate its provider state in place.
    MutateInPlace,

    /// The handle is shared and requires a provider-level fork before
    /// mutation.
    ForkThenMutate,
}

impl CopyOnWriteAction {
    /// Determines the required mutation action without performing it.
    pub fn for_handle(handle: &CopyOnWriteState) -> Self {
        if handle.is_unique() {
            Self::MutateInPlace
        } else {
            Self::ForkThenMutate
        }
    }
}

/// Returns the mutation action that would currently be required.
pub fn planned_action(handle: &CopyOnWriteState) -> CopyOnWriteAction {
    CopyOnWriteAction::for_handle(handle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_variants_are_distinct() {
        assert_ne!(
            CopyOnWriteAction::MutateInPlace,
            CopyOnWriteAction::ForkThenMutate
        );
    }
}