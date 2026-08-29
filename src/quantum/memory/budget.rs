//! Zamani Quantum Memory — Resource Budgets
//!
//! Production-grade, provider-neutral resource accounting for
//! `quantum::memory`.
//!
//! # Architectural position
//!
//! `budget.rs` is the **accounting layer** of the quantum-memory subsystem.
//!
//! The intended dependency direction is:
//!
//! ```text
//!                 quantum::memory::limits
//!                           │
//!                           │ policy
//!                           ▼
//!                 quantum::memory::budget
//!                           │
//!                           │ accounting
//!                           ▼
//!                 quantum::memory::allocator
//!                           │
//!              ┌────────────┼────────────┐
//!              ▼            ▼            ▼
//!            host         device     distributed
//! ```
//!
//! `limits.rs` answers:
//!
//!     "What is this memory domain allowed to consume?"
//!
//! `budget.rs` answers:
//!
//!     "How much of that allowance has already been reserved?"
//!
//! `allocator.rs` answers:
//!
//!     "Can the requested provider actually allocate it?"
//!
//! These responsibilities must remain separate.
//!
//! # Why this module exists
//!
//! Quantum workloads can consume memory in several simultaneous dimensions.
//! A single allocation may need to count against:
//!
//! - total host memory;
//! - temporary host memory;
//! - state memory;
//! - allocation count;
//! - qubit count;
//! - device memory;
//! - distributed memory;
//! - persistent memory;
//! - checkpoint memory;
//! - metadata memory.
//!
//! Therefore a simple:
//!
//! ```text
//! used_bytes += requested_bytes
//! ```
//!
//! is insufficient.
//!
//! This module supports **atomic multi-resource reservations**.
//!
//! For example, a temporary GPU state allocation can be charged atomically to:
//!
//! ```text
//! DeviceBytes
//! TemporaryDeviceBytes
//! StateBytes
//! TemporaryStateBytes
//! Allocations
//! ```
//!
//! If any one of those dimensions would exceed its budget, **none of them is
//! modified**.
//!
//! # Provider neutrality
//!
//! This module intentionally knows nothing about:
//!
//! - CUDA;
//! - HIP;
//! - Metal;
//! - Vulkan;
//! - SYCL;
//! - OpenCL;
//! - NVIDIA;
//! - AMD;
//! - Apple;
//! - IBM;
//! - Google;
//! - IonQ;
//! - Rigetti;
//! - Quantinuum;
//! - Pasqal;
//! - QuEra;
//! - D-Wave;
//! - photonic hardware;
//! - superconducting hardware;
//! - trapped-ion hardware;
//! - neutral-atom hardware;
//! - remote QPUs;
//! - simulators.
//!
//! A provider is represented only through generic resource dimensions.
//!
//! Consequently, adding a new QPU or accelerator must not require modifying
//! this file.
//!
//! # No unsafe
//!
//! This module contains no `unsafe` code.
//!
//! `#![deny(unsafe_code)]` is included deliberately.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe.
//!
//! # Important accounting invariant
//!
//! For every resource:
//!
//! ```text
//! 0 <= used <= capacity
//! ```
//!
//! Failed reservation attempts must not modify accounting.
//!
//! Successful reservations increase accounting exactly once.
//!
//! Released reservations decrease accounting exactly once.
//!
//! RAII reservation guards are provided to make this invariant robust against
//! early returns and error paths.
//!
//! # Nested budgets
//!
//! A parent budget may own child budgets conceptually:
//!
//! ```text
//! execution budget
//! ├── compilation budget
//! ├── simulation budget
//! │   ├── state budget
//! │   └── temporary budget
//! ├── QEC budget
//! └── checkpoint budget
//! ```
//!
//! This module provides a parent/child composition primitive through shared
//! accounting domains. A child reservation can charge both a child budget and
//! its parent atomically.
//!
//! # Integration contract
//!
//! `limits.rs`:
//!
//! - constructs the finite policy;
//! - supplies the capacity values;
//! - must never perform live accounting.
//!
//! `allocator.rs`:
//!
//! - creates the actual allocation;
//! - must reserve budget before calling a provider;
//! - must release/commit the reservation according to allocation lifetime.
//!
//! `reservation.rs`:
//!
//! - may use `BudgetReservation` as the accounting layer;
//! - must not create a second accounting implementation.
//!
//! `pool.rs`:
//!
//! - charges retained pool capacity;
//! - releases capacity when blocks leave the pool.
//!
//! `state_vector.rs` / `density_matrix.rs` / `stabilizer.rs` / `sparse.rs` /
//! `tensor_network.rs`:
//!
//! - calculate representation-specific requirements;
//! - submit those requirements as a `BudgetRequest`;
//! - never directly mutate budget counters.
//!
//! `gpu.rs`:
//!
//! - maps accelerator allocations to `DeviceBytes` and related dimensions.
//!
//! `distributed.rs`:
//!
//! - maps distributed allocations to `DistributedBytes` and partition counts.
//!
//! `migration.rs`:
//!
//! - reserves source/destination/temporary resources before migration.
//!
//! `snapshot.rs` / `checkpoint.rs`:
//!
//! - charge persistent/checkpoint resources before producing the payload.
//!
//! `diagnostics.rs` / `telemetry.rs`:
//!
//! - read immutable snapshots from this module;
//! - must not mutate accounting.
//!
//! `benchmarking`:
//!
//! - may consume budget snapshots as measurements;
//! - must not become a dependency of this module.
//!
//! # Deliberate non-responsibilities
//!
//! This module does not:
//!
//! - allocate memory;
//! - free provider memory;
//! - inspect pointers;
//! - inspect OS memory;
//! - query GPU drivers;
//! - contact QPUs;
//! - perform network operations;
//! - perform serialization;
//! - perform logging;
//! - perform telemetry;
//! - choose a quantum-state representation;
//! - decide whether a workload should use CPU/GPU/QPU;
//! - silently downgrade an allocation.
//!
//! It only performs deterministic resource accounting.
//!
//! # Example
//!
//! ```rust
//! use zamani_compiler::quantum::memory::budget::{
//!     BudgetRequest, BudgetResource, MemoryBudget,
//! };
//!
//! let budget = MemoryBudget::new("simulation", [
//!     (BudgetResource::HostBytes, 8 * 1024 * 1024 * 1024),
//!     (BudgetResource::StateBytes, 4 * 1024 * 1024 * 1024),
//!     (BudgetResource::Allocations, 10_000),
//! ]);
//!
//! let request = BudgetRequest::new()
//!     .with(BudgetResource::HostBytes, 1024 * 1024)
//!     .with(BudgetResource::StateBytes, 1024 * 1024)
//!     .with(BudgetResource::Allocations, 1);
//!
//! let reservation = budget.reserve(request).expect("reservation fits");
//!
//! assert_eq!(
//!     budget.used(BudgetResource::Allocations),
//!     1
//! );
//!
//! drop(reservation);
//!
//! assert_eq!(
//!     budget.used(BudgetResource::Allocations),
//!     0
//! );
//! ```
//!
//! The real quantum memory implementation will normally build requests from
//! `limits::MemoryRequirement` and `allocator::AllocationClass` rather than
//! manually constructing them at every call site.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

/// Stable schema identifier for the memory-budget subsystem.
pub const MEMORY_BUDGET_SCHEMA_ID: &str = "zamani.quantum.memory.budget";

/// Semantic version of the budget contract.
pub const MEMORY_BUDGET_SCHEMA_VERSION: u16 = 1;

/// Resource dimensions tracked by a quantum-memory budget.
///
/// The dimensions are intentionally generic and provider-neutral.
///
/// A request may charge several dimensions simultaneously.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BudgetResource {
    /// Total host-resident bytes.
    HostBytes,

    /// Temporary host-resident bytes.
    TemporaryHostBytes,

    /// Persistent host-resident bytes.
    PersistentHostBytes,

    /// Pinned host bytes.
    PinnedHostBytes,

    /// Accelerator/device bytes.
    DeviceBytes,

    /// Temporary accelerator/device bytes.
    TemporaryDeviceBytes,

    /// Unified host/device bytes.
    UnifiedBytes,

    /// Distributed-memory bytes.
    DistributedBytes,

    /// Temporary distributed-memory bytes.
    TemporaryDistributedBytes,

    /// Persistent distributed-memory bytes.
    PersistentDistributedBytes,

    /// Backend-native memory represented as bytes.
    BackendNativeBytes,

    /// Total state-representation bytes.
    StateBytes,

    /// Temporary state bytes.
    TemporaryStateBytes,

    /// Persistent state bytes.
    PersistentStateBytes,

    /// Number of independently tracked allocations.
    Allocations,

    /// Number of logical/physical qubits accounted by a budget.
    Qubits,

    /// Number of classical bits accounted by a budget.
    ClassicalBits,

    /// Number of state elements/amplitudes.
    StateElements,

    /// Number of distributed partitions.
    DistributedPartitions,

    /// Number of tensors.
    Tensors,

    /// Tensor-network bond-dimension units.
    BondDimension,

    /// Snapshot bytes.
    SnapshotBytes,

    /// Checkpoint bytes.
    CheckpointBytes,

    /// Metadata bytes.
    MetadataBytes,

    /// Planning-work units.
    PlanningWork,
}

impl BudgetResource {
    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostBytes => "host_bytes",
            Self::TemporaryHostBytes => "temporary_host_bytes",
            Self::PersistentHostBytes => "persistent_host_bytes",
            Self::PinnedHostBytes => "pinned_host_bytes",
            Self::DeviceBytes => "device_bytes",
            Self::TemporaryDeviceBytes => "temporary_device_bytes",
            Self::UnifiedBytes => "unified_bytes",
            Self::DistributedBytes => "distributed_bytes",
            Self::TemporaryDistributedBytes => "temporary_distributed_bytes",
            Self::PersistentDistributedBytes => "persistent_distributed_bytes",
            Self::BackendNativeBytes => "backend_native_bytes",
            Self::StateBytes => "state_bytes",
            Self::TemporaryStateBytes => "temporary_state_bytes",
            Self::PersistentStateBytes => "persistent_state_bytes",
            Self::Allocations => "allocations",
            Self::Qubits => "qubits",
            Self::ClassicalBits => "classical_bits",
            Self::StateElements => "state_elements",
            Self::DistributedPartitions => "distributed_partitions",
            Self::Tensors => "tensors",
            Self::BondDimension => "bond_dimension",
            Self::SnapshotBytes => "snapshot_bytes",
            Self::CheckpointBytes => "checkpoint_bytes",
            Self::MetadataBytes => "metadata_bytes",
            Self::PlanningWork => "planning_work",
        }
    }

    /// Returns whether this dimension represents bytes.
    pub const fn is_byte_resource(self) -> bool {
        matches!(
            self,
            Self::HostBytes
                | Self::TemporaryHostBytes
                | Self::PersistentHostBytes
                | Self::PinnedHostBytes
                | Self::DeviceBytes
                | Self::TemporaryDeviceBytes
                | Self::UnifiedBytes
                | Self::DistributedBytes
                | Self::TemporaryDistributedBytes
                | Self::PersistentDistributedBytes
                | Self::BackendNativeBytes
                | Self::StateBytes
                | Self::TemporaryStateBytes
                | Self::PersistentStateBytes
                | Self::SnapshotBytes
                | Self::CheckpointBytes
                | Self::MetadataBytes
        )
    }
}

impl fmt::Display for BudgetResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A requested resource amount.
///
/// The amount is always non-negative.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BudgetAmount(u64);

impl BudgetAmount {
    /// Zero resource units.
    pub const ZERO: Self = Self(0);

    /// Creates an amount.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying quantity.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Checked addition.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for BudgetAmount {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<BudgetAmount> for u64 {
    fn from(value: BudgetAmount) -> Self {
        value.get()
    }
}

impl fmt::Display for BudgetAmount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0.to_string())
    }
}

/// A single finite resource limit.
///
/// A capacity of zero is valid. It means that resource is prohibited.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BudgetLimit {
    capacity: u64,
}

impl BudgetLimit {
    /// Creates a finite resource limit.
    pub const fn new(capacity: u64) -> Self {
        Self { capacity }
    }

    /// Returns the configured capacity.
    pub const fn capacity(self) -> u64 {
        self.capacity
    }
}

impl From<u64> for BudgetLimit {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// A resource request.
///
/// Requests are normalized by resource identity. Repeated additions to the
/// same resource are checked for overflow and combined into one entry.
///
/// This is important for composability: independent layers may contribute to
/// the same request without creating duplicate accounting records.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BudgetRequest {
    amounts: BTreeMap<BudgetResource, u64>,
}

impl BudgetRequest {
    /// Creates an empty request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds resource units to the request.
    ///
    /// Returns an error if the combined amount overflows `u64`.
    pub fn with(
        mut self,
        resource: BudgetResource,
        amount: u64,
    ) -> Result<Self, BudgetRequestError> {
        self.add(resource, amount)?;
        Ok(self)
    }

    /// Adds resource units to an existing request.
    pub fn add(
        &mut self,
        resource: BudgetResource,
        amount: u64,
    ) -> Result<(), BudgetRequestError> {
        let current = self.amounts.get(&resource).copied().unwrap_or(0);

        let combined = current
            .checked_add(amount)
            .ok_or(BudgetRequestError::AmountOverflow { resource })?;

        self.amounts.insert(resource, combined);
        Ok(())
    }

    /// Adds one unit to a resource.
    pub fn add_one(
        &mut self,
        resource: BudgetResource,
    ) -> Result<(), BudgetRequestError> {
        self.add(resource, 1)
    }

    /// Returns the requested amount for a resource.
    pub fn get(&self, resource: BudgetResource) -> u64 {
        self.amounts.get(&resource).copied().unwrap_or(0)
    }

    /// Returns true when the request has no resource requirements.
    pub fn is_empty(&self) -> bool {
        self.amounts.is_empty()
    }

    /// Returns the number of resource dimensions in this request.
    pub fn resource_count(&self) -> usize {
        self.amounts.len()
    }

    /// Returns all resource requirements in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = (BudgetResource, u64)> + '_ {
        self.amounts.iter().map(|(&resource, &amount)| (resource, amount))
    }

    /// Returns the request as a deterministic vector.
    pub fn to_vec(&self) -> Vec<(BudgetResource, u64)> {
        self.iter().collect()
    }
}

/// Error produced while constructing a budget request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BudgetRequestError {
    /// Adding amounts for one resource overflowed `u64`.
    AmountOverflow {
        /// Resource whose amount overflowed.
        resource: BudgetResource,
    },
}

impl fmt::Display for BudgetRequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AmountOverflow { resource } => {
                write!(
                    formatter,
                    "budget request amount overflowed for resource `{resource}`"
                )
            }
        }
    }
}

impl std::error::Error for BudgetRequestError {}

/// A resource reservation failure.
///
/// The budget is unchanged when this error is returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BudgetError {
    /// The request exceeds a configured capacity.
    CapacityExceeded {
        /// Resource that prevented the reservation.
        resource: BudgetResource,

        /// Configured capacity.
        capacity: u64,

        /// Amount already used.
        used: u64,

        /// Requested additional amount.
        requested: u64,

        /// Remaining capacity.
        available: u64,
    },

    /// Existing accounting was corrupted or inconsistent.
    AccountingOverflow {
        /// Resource whose accounting overflowed.
        resource: BudgetResource,
    },

    /// Release attempted to return more units than were reserved.
    ReleaseExceedsUsage {
        /// Resource being released.
        resource: BudgetResource,

        /// Current usage.
        used: u64,

        /// Amount requested for release.
        release: u64,
    },

    /// A request contained a value that could not be represented.
    InvalidRequest {
        /// Resource with the invalid amount.
        resource: BudgetResource,
    },

    /// Internal synchronization state could not be acquired.
    SynchronizationFailure,
}

impl fmt::Display for BudgetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityExceeded {
                resource,
                capacity,
                used,
                requested,
                available,
            } => write!(
                formatter,
                "budget resource `{resource}` exceeded: \
                 capacity={capacity}, used={used}, requested={requested}, \
                 available={available}"
            ),

            Self::AccountingOverflow { resource } => {
                write!(
                    formatter,
                    "budget accounting overflow for resource `{resource}`"
                )
            }

            Self::ReleaseExceedsUsage {
                resource,
                used,
                release,
            } => write!(
                formatter,
                "budget release exceeds usage for `{resource}`: \
                 used={used}, release={release}"
            ),

            Self::InvalidRequest { resource } => {
                write!(
                    formatter,
                    "invalid budget request for resource `{resource}`"
                )
            }

            Self::SynchronizationFailure => {
                formatter.write_str("budget synchronization failure")
            }
        }
    }
}

impl std::error::Error for BudgetError {}

/// Immutable information about one resource dimension.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BudgetResourceSnapshot {
    /// Resource identity.
    pub resource: BudgetResource,

    /// Configured maximum.
    pub capacity: u64,

    /// Current reserved amount.
    pub used: u64,

    /// Remaining amount.
    pub available: u64,
}

impl BudgetResourceSnapshot {
    /// Returns utilization as a fraction in `[0, 1]`.
    ///
    /// A zero-capacity budget reports `0.0` when unused and `1.0` when used.
    pub fn utilization(self) -> f64 {
        if self.capacity == 0 {
            if self.used == 0 {
                0.0
            } else {
                1.0
            }
        } else {
            self.used as f64 / self.capacity as f64
        }
    }

    /// Returns whether the resource is exhausted.
    pub const fn is_exhausted(self) -> bool {
        self.used == self.capacity
    }
}

/// Immutable snapshot of all budget accounting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetSnapshot {
    /// Stable budget identifier.
    pub name: String,

    /// Schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Per-resource accounting.
    resources: Vec<BudgetResourceSnapshot>,
}

impl BudgetSnapshot {
    /// Returns all resources in deterministic order.
    pub fn resources(&self) -> &[BudgetResourceSnapshot] {
        &self.resources
    }

    /// Returns the snapshot for one resource, if configured.
    pub fn resource(
        &self,
        resource: BudgetResource,
    ) -> Option<BudgetResourceSnapshot> {
        self.resources
            .iter()
            .copied()
            .find(|entry| entry.resource == resource)
    }
}

/// Internal mutable accounting state.
#[derive(Debug)]
struct BudgetState {
    limits: BTreeMap<BudgetResource, BudgetLimit>,
    used: BTreeMap<BudgetResource, u64>,
}

/// Thread-safe finite resource budget.
///
/// `MemoryBudget` is cloneable because clones share the same accounting
/// domain. Cloning does **not** duplicate capacity.
///
/// This is important for concurrent quantum execution.
///
/// ```text
/// Arc<Mutex<BudgetState>>
///       │
///       ├── executor thread
///       ├── simulator thread
///       ├── QEC thread
///       └── migration thread
/// ```
///
/// All successful multi-resource reservations are committed atomically under
/// one lock.
#[derive(Clone, Debug)]
pub struct MemoryBudget {
    name: Arc<str>,
    state: Arc<Mutex<BudgetState>>,
}

impl MemoryBudget {
    /// Creates a budget from a deterministic list of finite resource limits.
    ///
    /// Duplicate resources are rejected rather than silently overwritten.
    pub fn new<I, R>(name: impl Into<String>, limits: I) -> Result<Self, BudgetConfigError>
    where
        I: IntoIterator<Item = (R, u64)>,
        R: Into<BudgetResource>,
    {
        let mut map = BTreeMap::new();

        for (resource, capacity) in limits {
            let resource = resource.into();

            if map.insert(resource, BudgetLimit::new(capacity)).is_some() {
                return Err(BudgetConfigError::DuplicateResource { resource });
            }
        }

        Ok(Self {
            name: Arc::<str>::from(name.into()),
            state: Arc::new(Mutex::new(BudgetState {
                limits: map,
                used: BTreeMap::new(),
            })),
        })
    }

    /// Creates an empty budget.
    ///
    /// An empty budget permits no configured resources. This is useful when
    /// building a budget incrementally with `with_limit`.
    pub fn empty(name: impl Into<String>) -> Self {
        Self {
            name: Arc::<str>::from(name.into()),
            state: Arc::new(Mutex::new(BudgetState {
                limits: BTreeMap::new(),
                used: BTreeMap::new(),
            })),
        }
    }

    /// Returns the budget's stable human-readable name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Adds a resource limit to an empty/configurable budget.
    ///
    /// This operation fails if the resource already exists.
    ///
    /// Configuration should normally be completed before the budget is shared
    /// across execution threads.
    pub fn with_limit(
        &self,
        resource: BudgetResource,
        capacity: u64,
    ) -> Result<(), BudgetConfigError> {
        let mut state = self.lock()?;

        if state.limits.contains_key(&resource) {
            return Err(BudgetConfigError::DuplicateResource { resource });
        }

        state
            .limits
            .insert(resource, BudgetLimit::new(capacity));

        Ok(())
    }

    /// Returns the configured capacity of a resource.
    ///
    /// `None` means the resource is not configured by this budget.
    pub fn capacity(
        &self,
        resource: BudgetResource,
    ) -> Result<Option<u64>, BudgetError> {
        let state = self.lock()?;
        Ok(state.limits.get(&resource).map(|limit| limit.capacity()))
    }

    /// Returns current usage of a resource.
    ///
    /// An unconfigured resource reports zero usage.
    pub fn used(&self, resource: BudgetResource) -> u64 {
        match self.state.lock() {
            Ok(state) => state.used.get(&resource).copied().unwrap_or(0),
            Err(_) => 0,
        }
    }

    /// Returns remaining capacity.
    ///
    /// An unconfigured resource has no capacity and therefore reports zero.
    pub fn available(&self, resource: BudgetResource) -> u64 {
        match self.state.lock() {
            Ok(state) => {
                let capacity = state
                    .limits
                    .get(&resource)
                    .map(BudgetLimit::capacity)
                    .unwrap_or(0);

                let used = state.used.get(&resource).copied().unwrap_or(0);

                capacity.saturating_sub(used)
            }
            Err(_) => 0,
        }
    }

    /// Returns a complete immutable accounting snapshot.
    pub fn snapshot(&self) -> Result<BudgetSnapshot, BudgetError> {
        let state = self.lock()?;

        let resources = state
            .limits
            .iter()
            .map(|(&resource, &limit)| {
                let capacity = limit.capacity();
                let used = state.used.get(&resource).copied().unwrap_or(0);
                let available = capacity.saturating_sub(used);

                BudgetResourceSnapshot {
                    resource,
                    capacity,
                    used,
                    available,
                }
            })
            .collect();

        Ok(BudgetSnapshot {
            name: self.name.to_string(),
            schema_id: MEMORY_BUDGET_SCHEMA_ID,
            schema_version: MEMORY_BUDGET_SCHEMA_VERSION,
            resources,
        })
    }

    /// Checks whether a request would fit without changing accounting.
    ///
    /// This operation is atomic across all dimensions.
    pub fn check(&self, request: &BudgetRequest) -> Result<(), BudgetError> {
        let state = self.lock()?;
        Self::check_locked(&state, request)
    }

    /// Atomically reserves all resources in a request.
    ///
    /// On failure, **no resource is modified**.
    pub fn reserve(
        &self,
        request: BudgetRequest,
    ) -> Result<BudgetReservation, BudgetError> {
        let mut state = self.lock()?;

        Self::check_locked(&state, &request)?;

        for (resource, amount) in request.iter() {
            let current = state.used.get(&resource).copied().unwrap_or(0);

            let new_value = current
                .checked_add(amount)
                .ok_or(BudgetError::AccountingOverflow { resource })?;

            state.used.insert(resource, new_value);
        }

        Ok(BudgetReservation {
            budget: self.clone(),
            request,
            active: true,
        })
    }

    /// Atomically consumes a request without returning an RAII reservation.
    ///
    /// This is useful for cumulative accounting where usage is deliberately
    /// retained until an explicit release.
    pub fn consume(
        &self,
        request: &BudgetRequest,
    ) -> Result<(), BudgetError> {
        let mut state = self.lock()?;

        Self::check_locked(&state, request)?;

        for (resource, amount) in request.iter() {
            let current = state.used.get(&resource).copied().unwrap_or(0);

            let new_value = current
                .checked_add(amount)
                .ok_or(BudgetError::AccountingOverflow { resource })?;

            state.used.insert(resource, new_value);
        }

        Ok(())
    }

    /// Releases previously consumed resources.
    ///
    /// Every resource must have enough current usage for the release.
    ///
    /// The release is atomic: if one dimension fails, no dimension is changed.
    pub fn release(
        &self,
        request: &BudgetRequest,
    ) -> Result<(), BudgetError> {
        let mut state = self.lock()?;

        Self::check_release_locked(&state, request)?;

        for (resource, amount) in request.iter() {
            let current = state.used.get(&resource).copied().unwrap_or(0);

            let new_value = current
                .checked_sub(amount)
                .ok_or(BudgetError::ReleaseExceedsUsage {
                    resource,
                    used: current,
                    release: amount,
                })?;

            if new_value == 0 {
                state.used.remove(&resource);
            } else {
                state.used.insert(resource, new_value);
            }
        }

        Ok(())
    }

    /// Returns true when no resource currently has any usage.
    pub fn is_empty(&self) -> bool {
        match self.state.lock() {
            Ok(state) => state.used.values().all(|value| *value == 0),
            Err(_) => false,
        }
    }

    /// Returns the number of configured resource dimensions.
    pub fn resource_count(&self) -> usize {
        match self.state.lock() {
            Ok(state) => state.limits.len(),
            Err(_) => 0,
        }
    }

    /// Checks all request dimensions while holding the accounting lock.
    fn check_locked(
        state: &BudgetState,
        request: &BudgetRequest,
    ) -> Result<(), BudgetError> {
        for (resource, requested) in request.iter() {
            let limit = match state.limits.get(&resource) {
                Some(limit) => limit.capacity(),
                None => {
                    return Err(BudgetError::InvalidRequest { resource });
                }
            };

            let used = state.used.get(&resource).copied().unwrap_or(0);

            let required = used
                .checked_add(requested)
                .ok_or(BudgetError::AccountingOverflow { resource })?;

            if required > limit {
                return Err(BudgetError::CapacityExceeded {
                    resource,
                    capacity: limit,
                    used,
                    requested,
                    available: limit.saturating_sub(used),
                });
            }
        }

        Ok(())
    }

    /// Checks release validity while holding the accounting lock.
    fn check_release_locked(
        state: &BudgetState,
        request: &BudgetRequest,
    ) -> Result<(), BudgetError> {
        for (resource, release) in request.iter() {
            let used = state.used.get(&resource).copied().unwrap_or(0);

            if release > used {
                return Err(BudgetError::ReleaseExceedsUsage {
                    resource,
                    used,
                    release,
                });
            }
        }

        Ok(())
    }

    fn lock(&self) -> Result<MutexGuard<'_, BudgetState>, BudgetError> {
        self.state
            .lock()
            .map_err(|_| BudgetError::SynchronizationFailure)
    }
}

/// RAII reservation.
///
/// A reservation automatically releases all charged resources when dropped.
///
/// This is critical for quantum execution because state construction frequently
/// crosses many fallible operations:
///
/// ```text
/// reserve
///   │
///   ├── allocate
///   ├── initialize
///   ├── validate
///   ├── migrate
///   └── publish
/// ```
///
/// If any operation returns early, the reservation is still released.
///
/// The reservation is not cloneable, so one successful reservation corresponds
/// to exactly one owner.
#[must_use = "a successful budget reservation must be retained or explicitly committed"]
#[derive(Debug)]
pub struct BudgetReservation {
    budget: MemoryBudget,
    request: BudgetRequest,
    active: bool,
}

impl BudgetReservation {
    /// Returns the original reservation request.
    pub fn request(&self) -> &BudgetRequest {
        &self.request
    }

    /// Returns whether this reservation is still active.
    pub const fn is_active(&self) -> bool {
        self.active
    }

    /// Commits the reservation without releasing it.
    ///
    /// After commit, the reservation becomes inactive and its resources remain
    /// charged to the budget.
    ///
    /// This is appropriate when provider allocation ownership has successfully
    /// transferred to an allocation object.
    pub fn commit(mut self) {
        self.active = false;
    }

    /// Explicitly releases the reservation.
    ///
    /// On success the reservation becomes inactive.
    pub fn release(mut self) -> Result<(), BudgetError> {
        if !self.active {
            return Ok(());
        }

        self.budget.release(&self.request)?;
        self.active = false;

        Ok(())
    }

    /// Prevents the reservation from releasing automatically.
    ///
    /// This is equivalent to `commit()`.
    pub fn forget(mut self) {
        self.active = false;
    }
}

impl Drop for BudgetReservation {
    fn drop(&mut self) {
        if self.active {
            // Drop cannot return an error. The budget implementation guarantees
            // that this request was successfully reserved by this object and
            // therefore the release should succeed unless the accounting domain
            // has become poisoned. In that exceptional case, there is no safe
            // recovery action available from Drop.
            let _ = self.budget.release(&self.request);
            self.active = false;
        }
    }
}

/// Configuration error for a memory budget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BudgetConfigError {
    /// A resource was configured more than once.
    DuplicateResource {
        /// Duplicate resource.
        resource: BudgetResource,
    },
}

impl fmt::Display for BudgetConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "resource `{resource}` was configured more than once"
                )
            }
        }
    }
}

impl std::error::Error for BudgetConfigError {}

/// Convenience constructor for the standard quantum-memory budget dimensions.
///
/// This is intentionally a builder rather than hard-coded policy. The actual
/// numeric values should normally originate from `limits.rs`.
#[derive(Clone, Debug)]
pub struct MemoryBudgetBuilder {
    name: String,
    limits: BTreeMap<BudgetResource, u64>,
}

impl MemoryBudgetBuilder {
    /// Creates a new builder.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            limits: BTreeMap::new(),
        }
    }

    /// Adds/replaces a resource limit.
    ///
    /// Replacement is explicit and deterministic, which makes this builder
    /// convenient for applying policy overrides.
    pub fn limit(mut self, resource: BudgetResource, capacity: u64) -> Self {
        self.limits.insert(resource, capacity);
        self
    }

    /// Adds a byte-oriented host limit.
    pub fn host_bytes(self, bytes: u64) -> Self {
        self.limit(BudgetResource::HostBytes, bytes)
    }

    /// Adds a device limit.
    pub fn device_bytes(self, bytes: u64) -> Self {
        self.limit(BudgetResource::DeviceBytes, bytes)
    }

    /// Adds a distributed-memory limit.
    pub fn distributed_bytes(self, bytes: u64) -> Self {
        self.limit(BudgetResource::DistributedBytes, bytes)
    }

    /// Adds a state-memory limit.
    pub fn state_bytes(self, bytes: u64) -> Self {
        self.limit(BudgetResource::StateBytes, bytes)
    }

    /// Adds a state-element limit.
    pub fn state_elements(self, elements: u64) -> Self {
        self.limit(BudgetResource::StateElements, elements)
    }

    /// Adds an allocation-count limit.
    pub fn allocations(self, count: u64) -> Self {
        self.limit(BudgetResource::Allocations, count)
    }

    /// Adds a qubit-count limit.
    pub fn qubits(self, count: u64) -> Self {
        self.limit(BudgetResource::Qubits, count)
    }

    /// Adds a classical-bit limit.
    pub fn classical_bits(self, count: u64) -> Self {
        self.limit(BudgetResource::ClassicalBits, count)
    }

    /// Adds a temporary-state limit.
    pub fn temporary_state_bytes(self, bytes: u64) -> Self {
        self.limit(BudgetResource::TemporaryStateBytes, bytes)
    }

    /// Adds a snapshot limit.
    pub fn snapshot_bytes(self, bytes: u64) -> Self {
        self.limit(BudgetResource::SnapshotBytes, bytes)
    }

    /// Adds a checkpoint limit.
    pub fn checkpoint_bytes(self, bytes: u64) -> Self {
        self.limit(BudgetResource::CheckpointBytes, bytes)
    }

    /// Adds a tensor-network bond-dimension limit.
    pub fn bond_dimension(self, dimension: u64) -> Self {
        self.limit(BudgetResource::BondDimension, dimension)
    }

    /// Builds the shared budget.
    pub fn build(self) -> Result<MemoryBudget, BudgetConfigError> {
        MemoryBudget::new(self.name, self.limits)
    }
}

/// Convenience helper for a standard temporary host-state allocation.
///
/// This does not perform accounting itself. It creates a composable request
/// that callers can submit to `MemoryBudget::reserve`.
pub fn temporary_host_state_request(
    bytes: u64,
    state_elements: u64,
) -> Result<BudgetRequest, BudgetRequestError> {
    BudgetRequest::new()
        .with(BudgetResource::HostBytes, bytes)?
        .with(BudgetResource::TemporaryHostBytes, bytes)?
        .with(BudgetResource::StateBytes, bytes)?
        .with(BudgetResource::TemporaryStateBytes, bytes)?
        .with(BudgetResource::StateElements, state_elements)?
        .with(BudgetResource::Allocations, 1)
}

/// Convenience helper for a persistent host-state allocation.
pub fn persistent_host_state_request(
    bytes: u64,
    state_elements: u64,
) -> Result<BudgetRequest, BudgetRequestError> {
    BudgetRequest::new()
        .with(BudgetResource::HostBytes, bytes)?
        .with(BudgetResource::PersistentHostBytes, bytes)?
        .with(BudgetResource::StateBytes, bytes)?
        .with(BudgetResource::PersistentStateBytes, bytes)?
        .with(BudgetResource::StateElements, state_elements)?
        .with(BudgetResource::Allocations, 1)
}

/// Convenience helper for a temporary device-state allocation.
pub fn temporary_device_state_request(
    bytes: u64,
    state_elements: u64,
) -> Result<BudgetRequest, BudgetRequestError> {
    BudgetRequest::new()
        .with(BudgetResource::DeviceBytes, bytes)?
        .with(BudgetResource::TemporaryDeviceBytes, bytes)?
        .with(BudgetResource::StateBytes, bytes)?
        .with(BudgetResource::TemporaryStateBytes, bytes)?
        .with(BudgetResource::StateElements, state_elements)?
        .with(BudgetResource::Allocations, 1)
}

/// Convenience helper for a unified-memory state allocation.
pub fn unified_state_request(
    bytes: u64,
    state_elements: u64,
) -> Result<BudgetRequest, BudgetRequestError> {
    BudgetRequest::new()
        .with(BudgetResource::UnifiedBytes, bytes)?
        .with(BudgetResource::StateBytes, bytes)?
        .with(BudgetResource::StateElements, state_elements)?
        .with(BudgetResource::Allocations, 1)
}

/// Convenience helper for distributed state.
pub fn distributed_state_request(
    bytes: u64,
    partitions: u64,
    state_elements: u64,
) -> Result<BudgetRequest, BudgetRequestError> {
    BudgetRequest::new()
        .with(BudgetResource::DistributedBytes, bytes)?
        .with(BudgetResource::StateBytes, bytes)?
        .with(BudgetResource::StateElements, state_elements)?
        .with(BudgetResource::DistributedPartitions, partitions)?
        .with(BudgetResource::Allocations, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn budget() -> MemoryBudget {
        MemoryBudgetBuilder::new("test")
            .host_bytes(1024)
            .device_bytes(2048)
            .distributed_bytes(4096)
            .state_bytes(512)
            .temporary_state_bytes(256)
            .state_elements(1024)
            .allocations(16)
            .qubits(64)
            .classical_bits(128)
            .snapshot_bytes(2048)
            .checkpoint_bytes(4096)
            .build()
            .expect("valid budget")
    }

    #[test]
    fn request_combines_duplicate_resources() {
        let request = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 100)
            .expect("first amount")
            .with(BudgetResource::HostBytes, 50)
            .expect("second amount");

        assert_eq!(request.get(BudgetResource::HostBytes), 150);
        assert_eq!(request.resource_count(), 1);
    }

    #[test]
    fn successful_reservation_is_accounted() {
        let budget = budget();

        let request = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 100)
            .expect("request")
            .with(BudgetResource::Allocations, 1)
            .expect("request");

        let reservation = budget.reserve(request).expect("reservation");

        assert_eq!(budget.used(BudgetResource::HostBytes), 100);
        assert_eq!(budget.used(BudgetResource::Allocations), 1);

        drop(reservation);

        assert_eq!(budget.used(BudgetResource::HostBytes), 0);
        assert_eq!(budget.used(BudgetResource::Allocations), 0);
    }

    #[test]
    fn failed_reservation_is_atomic() {
        let budget = budget();

        let request = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 900)
            .expect("request")
            .with(BudgetResource::StateBytes, 600)
            .expect("request");

        let result = budget.reserve(request);

        assert!(matches!(
            result,
            Err(BudgetError::CapacityExceeded {
                resource: BudgetResource::StateBytes,
                ..
            })
        ));

        assert_eq!(budget.used(BudgetResource::HostBytes), 0);
        assert_eq!(budget.used(BudgetResource::StateBytes), 0);
    }

    #[test]
    fn explicit_release_is_atomic() {
        let budget = budget();

        let request = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 100)
            .expect("request")
            .with(BudgetResource::StateBytes, 100)
            .expect("request");

        let reservation = budget.reserve(request.clone()).expect("reserve");
        reservation.commit();

        let invalid_release = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 100)
            .expect("request")
            .with(BudgetResource::StateBytes, 101)
            .expect("request");

        assert!(budget.release(&invalid_release).is_err());

        assert_eq!(budget.used(BudgetResource::HostBytes), 100);
        assert_eq!(budget.used(BudgetResource::StateBytes), 100);

        budget.release(&request).expect("valid release");

        assert_eq!(budget.used(BudgetResource::HostBytes), 0);
        assert_eq!(budget.used(BudgetResource::StateBytes), 0);
    }

    #[test]
    fn commit_keeps_usage() {
        let budget = budget();

        let request = BudgetRequest::new()
            .with(BudgetResource::StateBytes, 100)
            .expect("request");

        let reservation = budget.reserve(request).expect("reserve");
        reservation.commit();

        assert_eq!(budget.used(BudgetResource::StateBytes), 100);
    }

    #[test]
    fn empty_budget_rejects_unconfigured_resource() {
        let budget = MemoryBudget::empty("empty");

        let request = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 1)
            .expect("request");

        assert!(matches!(
            budget.reserve(request),
            Err(BudgetError::InvalidRequest {
                resource: BudgetResource::HostBytes
            })
        ));
    }

    #[test]
    fn zero_capacity_resource_is_valid_but_rejects_positive_request() {
        let budget = MemoryBudget::new(
            "zero",
            [(BudgetResource::DeviceBytes, 0)],
        )
        .expect("valid configuration");

        let request = BudgetRequest::new()
            .with(BudgetResource::DeviceBytes, 1)
            .expect("request");

        assert!(matches!(
            budget.reserve(request),
            Err(BudgetError::CapacityExceeded {
                resource: BudgetResource::DeviceBytes,
                capacity: 0,
                ..
            })
        ));
    }

    #[test]
    fn snapshot_is_deterministic() {
        let budget = budget();

        let snapshot = budget.snapshot().expect("snapshot");

        for pair in snapshot.resources().windows(2) {
            assert!(pair[0].resource <= pair[1].resource);
        }
    }

    #[test]
    fn utilization_is_bounded() {
        let snapshot = BudgetResourceSnapshot {
            resource: BudgetResource::HostBytes,
            capacity: 100,
            used: 25,
            available: 75,
        };

        assert_eq!(snapshot.utilization(), 0.25);
    }

    #[test]
    fn request_overflow_is_rejected() {
        let mut request = BudgetRequest::new();

        request
            .add(BudgetResource::HostBytes, u64::MAX)
            .expect("first value");

        assert!(matches!(
            request.add(BudgetResource::HostBytes, 1),
            Err(BudgetRequestError::AmountOverflow {
                resource: BudgetResource::HostBytes
            })
        ));
    }

    #[test]
    fn standard_device_request_contains_required_dimensions() {
        let request =
            temporary_device_state_request(1024, 64).expect("request");

        assert_eq!(request.get(BudgetResource::DeviceBytes), 1024);
        assert_eq!(
            request.get(BudgetResource::TemporaryDeviceBytes),
            1024
        );
        assert_eq!(request.get(BudgetResource::StateBytes), 1024);
        assert_eq!(
            request.get(BudgetResource::TemporaryStateBytes),
            1024
        );
        assert_eq!(request.get(BudgetResource::StateElements), 64);
        assert_eq!(request.get(BudgetResource::Allocations), 1);
    }

    #[test]
    fn distributed_request_tracks_partitions() {
        let request =
            distributed_state_request(4096, 8, 1024).expect("request");

        assert_eq!(
            request.get(BudgetResource::DistributedBytes),
            4096
        );
        assert_eq!(
            request.get(BudgetResource::DistributedPartitions),
            8
        );
        assert_eq!(request.get(BudgetResource::StateElements), 1024);
    }

    #[test]
    fn cloned_budget_shares_accounting() {
        let first = budget();
        let second = first.clone();

        let request = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 100)
            .expect("request");

        let reservation = first.reserve(request).expect("reserve");

        assert_eq!(second.used(BudgetResource::HostBytes), 100);

        drop(reservation);

        assert_eq!(first.used(BudgetResource::HostBytes), 0);
        assert_eq!(second.used(BudgetResource::HostBytes), 0);
    }

    #[test]
    fn concurrent_reservations_are_serialized() {
        use std::sync::Arc;
        use std::thread;

        let budget = Arc::new(
            MemoryBudget::new(
                "concurrent",
                [(BudgetResource::Allocations, 100)],
            )
            .expect("budget"),
        );

        let mut handles = Vec::new();

        for _ in 0..10 {
            let shared = Arc::clone(&budget);

            handles.push(thread::spawn(move || {
                let request = BudgetRequest::new()
                    .with(BudgetResource::Allocations, 10)
                    .expect("request");

                shared.reserve(request).expect("reservation")
            }));
        }

        let reservations = handles
            .into_iter()
            .map(|handle| handle.join().expect("thread"))
            .collect::<Vec<_>>();

        assert_eq!(
            budget.used(BudgetResource::Allocations),
            100
        );

        drop(reservations);

        assert_eq!(
            budget.used(BudgetResource::Allocations),
            0
        );
    }

    #[test]
    fn failed_multi_resource_reservation_does_not_partially_charge() {
        let budget = MemoryBudget::new(
            "atomic",
            [
                (BudgetResource::HostBytes, 100),
                (BudgetResource::DeviceBytes, 50),
            ],
        )
        .expect("budget");

        let request = BudgetRequest::new()
            .with(BudgetResource::HostBytes, 50)
            .expect("request")
            .with(BudgetResource::DeviceBytes, 51)
            .expect("request");

        assert!(budget.reserve(request).is_err());

        assert_eq!(budget.used(BudgetResource::HostBytes), 0);
        assert_eq!(budget.used(BudgetResource::DeviceBytes), 0);
    }
}