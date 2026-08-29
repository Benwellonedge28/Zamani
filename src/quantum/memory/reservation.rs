//! Zamani Quantum Memory — Transactional Resource Reservations
//!
//! This module provides the transactional reservation layer used by the
//! quantum-memory subsystem before physical allocation takes place.
//!
//! # Architectural position
//!
//! `reservation.rs` deliberately sits between resource-policy validation and
//! concrete allocation:
//!
//! ```text
//! MemoryLimits / policy
//!          │
//!          ▼
//!   ReservationRequest
//!          │
//!          ▼
//!   ReservationManager
//!          │
//!     ┌────┴────┐
//!     │         │
//!   commit   rollback
//!     │         │
//!     ▼         ▼
//! MemoryAllocator
//!
//! ```
//!
//! The reservation layer does NOT allocate memory.
//!
//! It guarantees that a caller can atomically reserve a declared amount of
//! one or more resource classes before asking a later allocator to perform
//! the actual allocation.
//!
//! # Why this is separate from `allocator.rs`
//!
//! Quantum-memory requirements can be extremely large and may span multiple
//! resource domains:
//!
//! - host RAM;
//! - pinned host RAM;
//! - accelerator/device memory;
//! - unified memory;
//! - distributed memory;
//! - temporary working memory;
//! - persistent/checkpoint memory.
//!
//! An allocation may therefore require several resources simultaneously.
//! Reservation must happen before any irreversible allocation so that failure
//! of one resource cannot leave the system partially committed.
//!
//! # Transactional lifecycle
//!
//! ```text
//!                    reserve()
//!                       │
//!                       ▼
//!                   Reserved
//!                   /      \
//!             commit()    rollback()
//!                │            │
//!                ▼            ▼
//!             Committed    Released
//!                │
//!             release()
//!                │
//!                ▼
//!             Released
//! ```
//!
//! Dropping an uncommitted reservation automatically rolls it back.
//!
//! Dropping a committed reservation automatically releases its committed
//! resources. This makes the reservation token an RAII ownership object and
//! prevents leaked reservations when an operation returns early.
//!
//! # Hardware neutrality
//!
//! No QPU vendor appears in this module.
//!
//! IBM, Rigetti, Quantinuum, IonQ, superconducting systems, trapped-ion
//! systems, neutral-atom systems, photonic systems, simulators, GPUs, CPUs,
//! distributed simulators, and future hardware providers can all consume the
//! same reservation contract.
//!
//! Hardware-specific capacity discovery belongs to `hardware` and concrete
//! memory providers. Those components translate their capacities into the
//! generic resource classes defined here.
//!
//! # Dependency rule
//!
//! This module intentionally depends only on the Rust standard library.
//!
//! It does not depend on:
//!
//! - `allocator.rs`;
//! - `budget.rs`;
//! - `pool.rs`;
//! - state representations;
//! - Quantum IR;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - hardware adapters.
//!
//! Later modules integrate by consuming the public types and traits defined
//! here. `reservation.rs` therefore does not need to be edited merely because
//! those later modules are implemented.
//!
//! # Safety
//!
//! - No `unsafe`.
//! - No raw pointers.
//! - No global mutable state.
//! - No hidden allocation performed by the reservation operation itself.
//! - All arithmetic uses checked operations.
//! - All shared mutable state is protected by `std::sync::Mutex`.
//! - Poisoned mutexes are converted into an explicit error.
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

// =============================================================================
// Imports
// =============================================================================

use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

// =============================================================================
// Constants
// =============================================================================

/// Number of independently tracked reservation resource classes.
///
/// Keeping this fixed makes multi-resource reservation atomic without needing
/// a heap-allocated map and prevents callers from creating arbitrary resource
/// classes that the rest of the memory subsystem does not understand.
pub const RESOURCE_KIND_COUNT: usize = 7;

// =============================================================================
// Resource kind
// =============================================================================

/// A resource domain that can be reserved by the quantum-memory subsystem.
///
/// These categories are deliberately generic. They describe where/how memory
/// is accounted for, not which hardware vendor provides it.
///
/// A future allocator may map these categories to:
///
/// - CPU NUMA nodes;
/// - CUDA/HIP/Metal/Vulkan device memory;
/// - pinned host memory;
/// - unified memory;
/// - distributed memory;
/// - remote execution memory;
/// - simulator-specific arenas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ResourceKind {
    /// Ordinary host-resident memory.
    Host = 0,

    /// Page-locked/pinned host memory used for accelerator transfers.
    PinnedHost = 1,

    /// Accelerator/device-local memory.
    Device = 2,

    /// Unified/shared address-space memory.
    Unified = 3,

    /// Memory owned by a distributed execution partition.
    Distributed = 4,

    /// Short-lived working memory.
    Temporary = 5,

    /// Long-lived memory used for persistent state, snapshots, or checkpoints.
    Persistent = 6,
}

impl ResourceKind {
    /// Returns every supported resource kind in canonical order.
    pub const ALL: [Self; RESOURCE_KIND_COUNT] = [
        Self::Host,
        Self::PinnedHost,
        Self::Device,
        Self::Unified,
        Self::Distributed,
        Self::Temporary,
        Self::Persistent,
    ];

    /// Returns the zero-based array index associated with this resource.
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Returns a stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::Temporary => "temporary",
            Self::Persistent => "persistent",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Resource quantities
// =============================================================================

/// A complete multi-domain resource quantity.
///
/// Every entry is expressed in bytes.
///
/// The array representation is intentional:
///
/// - fixed-size;
/// - allocation-free;
/// - deterministic;
/// - easy to compare;
/// - suitable for atomic multi-resource accounting.
///
/// `ResourceAmounts` is not tied to a particular allocator or hardware API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ResourceAmounts {
    amounts: [usize; RESOURCE_KIND_COUNT],
}

impl ResourceAmounts {
    /// Creates an all-zero quantity.
    pub const fn zero() -> Self {
        Self {
            amounts: [0; RESOURCE_KIND_COUNT],
        }
    }

    /// Creates a quantity containing one resource amount.
    pub const fn single(kind: ResourceKind, bytes: usize) -> Self {
        let mut amounts = [0; RESOURCE_KIND_COUNT];
        amounts[kind.index()] = bytes;

        Self { amounts }
    }

    /// Returns the amount assigned to `kind`.
    #[inline]
    pub const fn get(&self, kind: ResourceKind) -> usize {
        self.amounts[kind.index()]
    }

    /// Returns true when every resource quantity is zero.
    pub const fn is_zero(&self) -> bool {
        let mut index = 0;

        while index < RESOURCE_KIND_COUNT {
            if self.amounts[index] != 0 {
                return false;
            }

            index += 1;
        }

        true
    }

    /// Returns the total quantity across all resource classes.
    ///
    /// This is primarily diagnostic. Different resource domains are not
    /// physically interchangeable, so callers must not use this value for
    /// capacity decisions.
    pub fn checked_total(&self) -> Result<usize, ReservationError> {
        let mut total = 0usize;

        for kind in ResourceKind::ALL {
            total = total.checked_add(self.get(kind)).ok_or(
                ReservationError::ArithmeticOverflow {
                    operation: ArithmeticOperation::Total,
                    resource: Some(kind),
                },
            )?;
        }

        Ok(total)
    }

    /// Returns an iterator over all resource classes and their quantities.
    pub fn iter(&self) -> impl Iterator<Item = (ResourceKind, usize)> + '_ {
        ResourceKind::ALL
            .iter()
            .copied()
            .map(|kind| (kind, self.get(kind)))
    }

    /// Returns true when this quantity is less than or equal to `capacity` in
    /// every resource domain.
    pub fn fits_within(&self, capacity: &Self) -> bool {
        ResourceKind::ALL
            .iter()
            .copied()
            .all(|kind| self.get(kind) <= capacity.get(kind))
    }

    /// Adds two quantities using checked arithmetic.
    pub fn checked_add(&self, other: &Self) -> Result<Self, ReservationError> {
        let mut amounts = [0usize; RESOURCE_KIND_COUNT];

        for kind in ResourceKind::ALL {
            amounts[kind.index()] = self
                .get(kind)
                .checked_add(other.get(kind))
                .ok_or(ReservationError::ArithmeticOverflow {
                    operation: ArithmeticOperation::Add,
                    resource: Some(kind),
                })?;
        }

        Ok(Self { amounts })
    }

    /// Subtracts `other` from this quantity using checked arithmetic.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, ReservationError> {
        let mut amounts = [0usize; RESOURCE_KIND_COUNT];

        for kind in ResourceKind::ALL {
            amounts[kind.index()] = self
                .get(kind)
                .checked_sub(other.get(kind))
                .ok_or(ReservationError::ArithmeticUnderflow {
                    operation: ArithmeticOperation::Subtract,
                    resource: Some(kind),
                })?;
        }

        Ok(Self { amounts })
    }

    /// Sets one resource quantity.
    ///
    /// This is a `const` constructor-style method so callers can build
    /// quantities without introducing a mutable collection.
    pub const fn with(mut self, kind: ResourceKind, bytes: usize) -> Self {
        self.amounts[kind.index()] = bytes;
        self
    }

    /// Returns the underlying fixed-size quantities.
    ///
    /// The array is copied, so callers cannot mutate manager-owned state.
    pub const fn as_array(&self) -> [usize; RESOURCE_KIND_COUNT] {
        self.amounts
    }
}

// =============================================================================
// Reservation request
// =============================================================================

/// Immutable declaration of resources required by one operation.
///
/// A request may span multiple domains.
///
/// Example:
///
/// - host: 1 GiB;
/// - device: 8 GiB;
/// - temporary: 512 MiB.
///
/// Reservation of all requested domains succeeds or none of them becomes
/// reserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservationRequest {
    amounts: ResourceAmounts,
}

impl ReservationRequest {
    /// Creates a request containing no resources.
    pub const fn empty() -> Self {
        Self {
            amounts: ResourceAmounts::zero(),
        }
    }

    /// Creates a request for one resource domain.
    pub const fn single(kind: ResourceKind, bytes: usize) -> Self {
        Self {
            amounts: ResourceAmounts::single(kind, bytes),
        }
    }

    /// Creates a request from resource amounts.
    pub const fn from_amounts(amounts: ResourceAmounts) -> Self {
        Self { amounts }
    }

    /// Adds one resource requirement.
    ///
    /// Returns an error on arithmetic overflow.
    pub fn try_with(
        self,
        kind: ResourceKind,
        bytes: usize,
    ) -> Result<Self, ReservationError> {
        let addition = ResourceAmounts::single(kind, bytes);

        Ok(Self {
            amounts: self.amounts.checked_add(&addition)?,
        })
    }

    /// Returns the resource amounts.
    pub const fn amounts(&self) -> ResourceAmounts {
        self.amounts
    }

    /// Returns the amount requested from one resource domain.
    pub const fn amount(&self, kind: ResourceKind) -> usize {
        self.amounts.get(kind)
    }

    /// Returns true if this request reserves nothing.
    pub const fn is_empty(&self) -> bool {
        self.amounts.is_zero()
    }

    /// Validates that this request is structurally usable.
    ///
    /// An empty request is rejected because a successful reservation of zero
    /// resources is almost always an integration bug and provides no useful
    /// ownership guarantee.
    pub const fn validate(&self) -> Result<(), ReservationError> {
        if self.is_empty() {
            return Err(ReservationError::EmptyRequest);
        }

        Ok(())
    }
}

// =============================================================================
// Reservation identity
// =============================================================================

/// Stable identity of one reservation transaction.
///
/// IDs are generated monotonically by a `ReservationManager`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReservationId(u64);

impl ReservationId {
    /// Returns the raw numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ReservationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reservation-{}", self.0)
    }
}

// =============================================================================
// Reservation metadata
// =============================================================================

/// Optional immutable metadata attached to a reservation.
///
/// Metadata is intentionally compact and owned by the reservation token.
///
/// It is useful for diagnostics and integration with:
///
/// - simulators;
/// - QPU execution;
/// - QEC;
/// - GPU execution;
/// - distributed execution;
/// - snapshots/checkpoints.
///
/// The reservation layer does not interpret the metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ReservationMetadata {
    /// Optional caller-defined operation identifier.
    operation: Option<String>,

    /// Optional caller-defined subsystem/component identifier.
    subsystem: Option<String>,
}

impl ReservationMetadata {
    /// Creates empty metadata.
    pub const fn empty() -> Self {
        Self {
            operation: None,
            subsystem: None,
        }
    }

    /// Creates metadata with an operation label.
    pub fn operation(value: impl Into<String>) -> Self {
        Self {
            operation: Some(value.into()),
            subsystem: None,
        }
    }

    /// Creates metadata with both operation and subsystem labels.
    pub fn new(
        operation: Option<String>,
        subsystem: Option<String>,
    ) -> Self {
        Self {
            operation,
            subsystem,
        }
    }

    /// Returns the optional operation label.
    pub fn operation_name(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the optional subsystem label.
    pub fn subsystem_name(&self) -> Option<&str> {
        self.subsystem.as_deref()
    }
}

// =============================================================================
// Reservation state
// =============================================================================

/// Lifecycle state of a reservation token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReservationState {
    /// Resources have been reserved but not physically committed.
    Reserved,

    /// Resources have been committed to the caller's allocation lifecycle.
    Committed,

    /// Resources have been returned to the manager.
    Released,
}

impl fmt::Display for ReservationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reserved => f.write_str("reserved"),
            Self::Committed => f.write_str("committed"),
            Self::Released => f.write_str("released"),
        }
    }
}

// =============================================================================
// Arithmetic operation
// =============================================================================

/// Identifies a checked arithmetic operation that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOperation {
    /// Addition overflow.
    Add,

    /// Subtraction underflow.
    Subtract,

    /// Total calculation overflow.
    Total,

    /// Reservation ID increment overflow.
    ReservationId,
}

impl fmt::Display for ArithmeticOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => f.write_str("addition"),
            Self::Subtract => f.write_str("subtraction"),
            Self::Total => f.write_str("total"),
            Self::ReservationId => f.write_str("reservation id"),
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by transactional memory reservation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservationError {
    /// A reservation request contains no resources.
    EmptyRequest,

    /// The requested resource exceeds the configured capacity.
    CapacityExceeded {
        /// Resource that could not satisfy the request.
        resource: ResourceKind,

        /// Currently committed amount.
        committed: usize,

        /// Currently reserved amount.
        reserved: usize,

        /// Newly requested amount.
        requested: usize,

        /// Total configured capacity.
        capacity: usize,
    },

    /// Internal arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: ArithmeticOperation,

        /// Resource involved, if applicable.
        resource: Option<ResourceKind>,
    },

    /// Internal arithmetic underflow occurred.
    ArithmeticUnderflow {
        /// Operation that underflowed.
        operation: ArithmeticOperation,

        /// Resource involved, if applicable.
        resource: Option<ResourceKind>,
    },

    /// The manager's synchronization primitive was poisoned.
    SynchronizationPoisoned,

    /// A reservation ID could not be generated.
    ReservationIdExhausted,

    /// A token operation was attempted after release.
    AlreadyReleased {
        /// Reservation involved.
        reservation: ReservationId,
    },

    /// Commit was requested for a reservation that was already committed.
    AlreadyCommitted {
        /// Reservation involved.
        reservation: ReservationId,
    },

    /// Rollback was requested for a reservation that was already released.
    AlreadyRolledBack {
        /// Reservation involved.
        reservation: ReservationId,
    },

    /// A token no longer belongs to the manager that created it.
    InvalidToken,

    /// An operation was attempted on an invalid lifecycle transition.
    InvalidTransition {
        /// Reservation involved.
        reservation: ReservationId,

        /// Current state.
        current: ReservationState,

        /// Requested operation.
        operation: &'static str,
    },
}

impl fmt::Display for ReservationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRequest => {
                f.write_str("quantum memory reservation request is empty")
            }

            Self::CapacityExceeded {
                resource,
                committed,
                reserved,
                requested,
                capacity,
            } => write!(
                f,
                "quantum memory reservation exceeds {resource} capacity: \
                 committed={committed}, reserved={reserved}, \
                 requested={requested}, capacity={capacity}"
            ),

            Self::ArithmeticOverflow {
                operation,
                resource,
            } => {
                write!(f, "arithmetic overflow during {operation}")?;

                if let Some(resource) = resource {
                    write!(f, " for resource {resource}")?;
                }

                Ok(())
            }

            Self::ArithmeticUnderflow {
                operation,
                resource,
            } => {
                write!(f, "arithmetic underflow during {operation}")?;

                if let Some(resource) = resource {
                    write!(f, " for resource {resource}")?;
                }

                Ok(())
            }

            Self::SynchronizationPoisoned => {
                f.write_str("quantum memory reservation state is poisoned")
            }

            Self::ReservationIdExhausted => {
                f.write_str("quantum memory reservation id space exhausted")
            }

            Self::AlreadyReleased { reservation } => {
                write!(f, "{reservation} has already been released")
            }

            Self::AlreadyCommitted { reservation } => {
                write!(f, "{reservation} has already been committed")
            }

            Self::AlreadyRolledBack { reservation } => {
                write!(f, "{reservation} has already been rolled back")
            }

            Self::InvalidToken => {
                f.write_str("reservation token does not belong to this manager")
            }

            Self::InvalidTransition {
                reservation,
                current,
                operation,
            } => write!(
                f,
                "invalid reservation lifecycle transition for {reservation}: \
                 operation={operation}, current_state={current}"
            ),
        }
    }
}

impl std::error::Error for ReservationError {}

// =============================================================================
// Capacity
// =============================================================================

/// Capacity available to the reservation manager.
///
/// Capacity is specified independently for every resource class.
///
/// A capacity of zero means that the resource is unavailable.
///
/// A maximum value of `usize` means the reservation layer places no smaller
/// limit on that resource; however, the concrete allocator may still impose
/// stricter limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservationCapacity {
    amounts: ResourceAmounts,
}

impl ReservationCapacity {
    /// Creates zero capacity for every resource.
    pub const fn zero() -> Self {
        Self {
            amounts: ResourceAmounts::zero(),
        }
    }

    /// Creates capacity for one resource.
    pub const fn single(kind: ResourceKind, bytes: usize) -> Self {
        Self {
            amounts: ResourceAmounts::single(kind, bytes),
        }
    }

    /// Creates capacity from resource amounts.
    pub const fn from_amounts(amounts: ResourceAmounts) -> Self {
        Self { amounts }
    }

    /// Returns the capacity for one resource.
    pub const fn amount(&self, kind: ResourceKind) -> usize {
        self.amounts.get(kind)
    }

    /// Returns all capacity amounts.
    pub const fn amounts(&self) -> ResourceAmounts {
        self.amounts
    }

    /// Returns a copy with one resource capacity changed.
    pub const fn with(mut self, kind: ResourceKind, bytes: usize) -> Self {
        self.amounts.amounts[kind.index()] = bytes;
        self
    }
}

// =============================================================================
// Internal manager state
// =============================================================================

#[derive(Debug)]
struct ReservationLedger {
    capacity: ReservationCapacity,
    committed: ResourceAmounts,
    reserved: ResourceAmounts,
    next_id: u64,
}

impl ReservationLedger {
    fn new(capacity: ReservationCapacity) -> Self {
        Self {
            capacity,
            committed: ResourceAmounts::zero(),
            reserved: ResourceAmounts::zero(),
            next_id: 1,
        }
    }

    fn next_id(&mut self) -> Result<ReservationId, ReservationError> {
        let id = self.next_id;

        self.next_id = self.next_id.checked_add(1).ok_or(
            ReservationError::ReservationIdExhausted,
        )?;

        Ok(ReservationId(id))
    }

    fn validate_request(
        &self,
        request: &ReservationRequest,
    ) -> Result<(), ReservationError> {
        request.validate()?;

        for kind in ResourceKind::ALL {
            let requested = request.amount(kind);

            if requested == 0 {
                continue;
            }

            let committed = self.committed.get(kind);
            let reserved = self.reserved.get(kind);

            let used = committed.checked_add(reserved).ok_or(
                ReservationError::ArithmeticOverflow {
                    operation: ArithmeticOperation::Add,
                    resource: Some(kind),
                },
            )?;

            let projected = used.checked_add(requested).ok_or(
                ReservationError::ArithmeticOverflow {
                    operation: ArithmeticOperation::Add,
                    resource: Some(kind),
                },
            )?;

            let capacity = self.capacity.amount(kind);

            if projected > capacity {
                return Err(ReservationError::CapacityExceeded {
                    resource: kind,
                    committed,
                    reserved,
                    requested,
                    capacity,
                });
            }
        }

        Ok(())
    }

    fn reserve(
        &mut self,
        request: &ReservationRequest,
    ) -> Result<ReservationId, ReservationError> {
        self.validate_request(request)?;

        let id = self.next_id()?;

        self.reserved = self.reserved.checked_add(&request.amounts())?;

        Ok(id)
    }

    fn commit(
        &mut self,
        request: &ReservationRequest,
    ) -> Result<(), ReservationError> {
        self.reserved = self.reserved.checked_sub(&request.amounts())?;
        self.committed = self.committed.checked_add(&request.amounts())?;

        Ok(())
    }

    fn rollback(
        &mut self,
        request: &ReservationRequest,
    ) -> Result<(), ReservationError> {
        self.reserved = self.reserved.checked_sub(&request.amounts())?;

        Ok(())
    }

    fn release_committed(
        &mut self,
        request: &ReservationRequest,
    ) -> Result<(), ReservationError> {
        self.committed = self.committed.checked_sub(&request.amounts())?;

        Ok(())
    }
}

// =============================================================================
// Manager
// =============================================================================

/// Thread-safe transactional reservation manager.
///
/// A manager owns resource accounting but does not own physical memory.
///
/// Multiple execution threads can share the same manager. Each reservation
/// operation holds the mutex only for the small accounting transaction.
///
/// No memory allocation occurs during the accounting operation itself.
///
/// # Example
///
/// ```
/// use zamani_quantum_memory_reservation::{
///     ReservationCapacity,
///     ReservationManager,
///     ReservationRequest,
///     ResourceKind,
/// };
///
/// let capacity = ReservationCapacity::single(
///     ResourceKind::Host,
///     1024,
/// );
///
/// let manager = ReservationManager::new(capacity);
///
/// let request = ReservationRequest::single(
///     ResourceKind::Host,
///     512,
/// );
///
/// let reservation = manager.reserve(request).unwrap();
/// reservation.commit().unwrap();
///
/// assert_eq!(
///     manager.committed().unwrap().amount(ResourceKind::Host),
///     512
/// );
///
/// reservation.release().unwrap();
/// assert_eq!(
///     manager.committed().unwrap().amount(ResourceKind::Host),
///     0
/// );
/// ```
#[derive(Clone, Debug)]
pub struct ReservationManager {
    inner: Arc<Mutex<ReservationLedger>>,
}

impl ReservationManager {
    /// Creates a new reservation manager.
    pub fn new(capacity: ReservationCapacity) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ReservationLedger::new(capacity))),
        }
    }

    /// Creates a manager with effectively unrestricted capacity.
    ///
    /// This is primarily useful for environments where another policy layer
    /// already guarantees capacity.
    ///
    /// Concrete production code should normally construct capacity explicitly
    /// from the actual memory policy.
    pub const fn unlimited_capacity() -> ReservationCapacity {
        ReservationCapacity::from_amounts(ResourceAmounts {
            amounts: [usize::MAX; RESOURCE_KIND_COUNT],
        })
    }

    /// Creates a manager using the maximum representable capacity.
    pub fn unlimited() -> Self {
        Self::new(Self::unlimited_capacity())
    }

    /// Returns the configured capacity.
    pub fn capacity(&self) -> Result<ReservationCapacity, ReservationError> {
        let ledger = self.lock()?;
        Ok(ledger.capacity)
    }

    /// Attempts to reserve all resources in `request` atomically.
    ///
    /// If any requested resource cannot fit, no resource is reserved.
    pub fn reserve(
        &self,
        request: ReservationRequest,
    ) -> Result<ReservationToken, ReservationError> {
        let mut ledger = self.lock()?;

        let id = ledger.reserve(&request)?;

        Ok(ReservationToken {
            manager: Arc::clone(&self.inner),
            id,
            request,
            metadata: ReservationMetadata::empty(),
            state: ReservationState::Reserved,
        })
    }

    /// Reserves resources with caller-supplied metadata.
    pub fn reserve_with_metadata(
        &self,
        request: ReservationRequest,
        metadata: ReservationMetadata,
    ) -> Result<ReservationToken, ReservationError> {
        let mut ledger = self.lock()?;

        let id = ledger.reserve(&request)?;

        Ok(ReservationToken {
            manager: Arc::clone(&self.inner),
            id,
            request,
            metadata,
            state: ReservationState::Reserved,
        })
    }

    /// Returns currently reserved resources.
    pub fn reserved(&self) -> Result<ResourceAmounts, ReservationError> {
        Ok(self.lock()?.reserved)
    }

    /// Returns currently committed resources.
    pub fn committed(&self) -> Result<ResourceAmounts, ReservationError> {
        Ok(self.lock()?.committed)
    }

    /// Returns currently available resources.
    ///
    /// Available = capacity - committed - reserved.
    pub fn available(&self) -> Result<ResourceAmounts, ReservationError> {
        let ledger = self.lock()?;

        let used = ledger.committed.checked_add(&ledger.reserved)?;

        ledger.capacity.amounts().checked_sub(&used)
    }

    /// Returns true when `request` could currently be reserved.
    ///
    /// This is a non-reserving probe. Another thread may consume the capacity
    /// immediately after this call, so callers that need a guarantee must call
    /// [`ReservationManager::reserve`].
    pub fn can_reserve(
        &self,
        request: ReservationRequest,
    ) -> Result<bool, ReservationError> {
        let ledger = self.lock()?;

        match ledger.validate_request(&request) {
            Ok(()) => Ok(true),
            Err(ReservationError::CapacityExceeded { .. }) => Ok(false),
            Err(error) => Err(error),
        }
    }

    /// Returns a diagnostic snapshot of the manager.
    pub fn snapshot(&self) -> Result<ReservationSnapshot, ReservationError> {
        let ledger = self.lock()?;

        Ok(ReservationSnapshot {
            capacity: ledger.capacity,
            reserved: ledger.reserved,
            committed: ledger.committed,
        })
    }

    fn lock(
        &self,
    ) -> Result<MutexGuard<'_, ReservationLedger>, ReservationError> {
        self.inner
            .lock()
            .map_err(|_| ReservationError::SynchronizationPoisoned)
    }
}

// =============================================================================
// Reservation snapshot
// =============================================================================

/// Point-in-time diagnostic view of reservation accounting.
///
/// The snapshot owns copies of the accounting values and therefore remains
/// valid after the manager changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservationSnapshot {
    /// Configured capacity.
    pub capacity: ReservationCapacity,

    /// Resources currently reserved but not committed.
    pub reserved: ResourceAmounts,

    /// Resources currently committed.
    pub committed: ResourceAmounts,
}

impl ReservationSnapshot {
    /// Returns resources currently available.
    pub fn available(&self) -> Result<ResourceAmounts, ReservationError> {
        let used = self.committed.checked_add(&self.reserved)?;

        self.capacity.amounts().checked_sub(&used)
    }
}

// =============================================================================
// Reservation token
// =============================================================================

/// RAII ownership token for a resource reservation.
///
/// A token is the only normal mechanism through which a reservation moves
/// through its lifecycle.
///
/// The token can be:
///
/// - committed;
/// - rolled back;
/// - released.
///
/// If the token is dropped while still reserved, the reservation is rolled
/// back automatically.
///
/// If the token is dropped after commit, the committed resources are released
/// automatically.
///
/// This provides strong cleanup guarantees during error propagation.
///
/// # Integration with `allocator.rs`
///
/// The intended later integration is:
///
/// ```text
/// ReservationManager
///        │
///        ▼
/// reserve(request)
///        │
///        ▼
/// ReservationToken
///        │
///        ├── physical allocation
///        │
///        └── commit()
/// ```
///
/// If physical allocation fails:
///
/// ```text
/// ReservationToken dropped
///        │
///        ▼
/// automatic rollback
/// ```
///
/// The reservation layer therefore does not need to know anything about the
/// allocator implementation.
#[derive(Debug)]
pub struct ReservationToken {
    manager: Arc<Mutex<ReservationLedger>>,
    id: ReservationId,
    request: ReservationRequest,
    metadata: ReservationMetadata,
    state: ReservationState,
}

impl ReservationToken {
    /// Returns the reservation identifier.
    pub const fn id(&self) -> ReservationId {
        self.id
    }

    /// Returns the requested resources.
    pub const fn request(&self) -> ReservationRequest {
        self.request
    }

    /// Returns the resource amounts.
    pub const fn amounts(&self) -> ResourceAmounts {
        self.request.amounts()
    }

    /// Returns the current lifecycle state.
    pub const fn state(&self) -> ReservationState {
        self.state
    }

    /// Returns immutable reservation metadata.
    pub const fn metadata(&self) -> &ReservationMetadata {
        &self.metadata
    }

    /// Returns true if the token is currently reserved.
    pub const fn is_reserved(&self) -> bool {
        matches!(self.state, ReservationState::Reserved)
    }

    /// Returns true if the token has been committed.
    pub const fn is_committed(&self) -> bool {
        matches!(self.state, ReservationState::Committed)
    }

    /// Returns true if the token has been released.
    pub const fn is_released(&self) -> bool {
        matches!(self.state, ReservationState::Released)
    }

    /// Commits the reservation.
    ///
    /// This moves accounting from `reserved` to `committed`.
    ///
    /// Physical allocation is deliberately outside this operation.
    ///
    /// The caller should invoke this only after the corresponding concrete
    /// allocation has successfully been established.
    pub fn commit(&mut self) -> Result<(), ReservationError> {
        match self.state {
            ReservationState::Reserved => {
                let mut ledger = self.lock()?;
                ledger.commit(&self.request)?;
                self.state = ReservationState::Committed;
                Ok(())
            }

            ReservationState::Committed => {
                Err(ReservationError::AlreadyCommitted {
                    reservation: self.id,
                })
            }

            ReservationState::Released => {
                Err(ReservationError::InvalidTransition {
                    reservation: self.id,
                    current: self.state,
                    operation: "commit",
                })
            }
        }
    }

    /// Rolls back a reserved transaction.
    ///
    /// This is the explicit form of the automatic rollback performed by
    /// `Drop`.
    pub fn rollback(&mut self) -> Result<(), ReservationError> {
        match self.state {
            ReservationState::Reserved => {
                let mut ledger = self.lock()?;
                ledger.rollback(&self.request)?;
                self.state = ReservationState::Released;
                Ok(())
            }

            ReservationState::Committed => {
                Err(ReservationError::InvalidTransition {
                    reservation: self.id,
                    current: self.state,
                    operation: "rollback",
                })
            }

            ReservationState::Released => {
                Err(ReservationError::AlreadyRolledBack {
                    reservation: self.id,
                })
            }
        }
    }

    /// Releases a reservation regardless of whether it is reserved or
    /// committed.
    ///
    /// For a reserved token this is equivalent to rollback.
    ///
    /// For a committed token this returns committed accounting to the
    /// available capacity.
    pub fn release(&mut self) -> Result<(), ReservationError> {
        match self.state {
            ReservationState::Reserved => {
                let mut ledger = self.lock()?;
                ledger.rollback(&self.request)?;
                self.state = ReservationState::Released;
                Ok(())
            }

            ReservationState::Committed => {
                let mut ledger = self.lock()?;
                ledger.release_committed(&self.request)?;
                self.state = ReservationState::Released;
                Ok(())
            }

            ReservationState::Released => {
                Err(ReservationError::AlreadyReleased {
                    reservation: self.id,
                })
            }
        }
    }

    fn lock(
        &self,
    ) -> Result<MutexGuard<'_, ReservationLedger>, ReservationError> {
        self.manager
            .lock()
            .map_err(|_| ReservationError::SynchronizationPoisoned)
    }
}

impl Drop for ReservationToken {
    fn drop(&mut self) {
        match self.state {
            ReservationState::Reserved => {
                if let Ok(mut ledger) = self.manager.lock() {
                    let _ = ledger.rollback(&self.request);
                }
            }

            ReservationState::Committed => {
                if let Ok(mut ledger) = self.manager.lock() {
                    let _ = ledger.release_committed(&self.request);
                }
            }

            ReservationState::Released => {}
        }

        self.state = ReservationState::Released;
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn host_capacity(bytes: usize) -> ReservationCapacity {
        ReservationCapacity::single(ResourceKind::Host, bytes)
    }

    fn host_request(bytes: usize) -> ReservationRequest {
        ReservationRequest::single(ResourceKind::Host, bytes)
    }

    #[test]
    fn resource_kind_indices_are_unique_and_contiguous() {
        for (expected, kind) in ResourceKind::ALL.iter().enumerate() {
            assert_eq!(kind.index(), expected);
        }
    }

    #[test]
    fn resource_kind_names_are_stable() {
        assert_eq!(ResourceKind::Host.as_str(), "host");
        assert_eq!(ResourceKind::PinnedHost.as_str(), "pinned_host");
        assert_eq!(ResourceKind::Device.as_str(), "device");
        assert_eq!(ResourceKind::Unified.as_str(), "unified");
        assert_eq!(ResourceKind::Distributed.as_str(), "distributed");
        assert_eq!(ResourceKind::Temporary.as_str(), "temporary");
        assert_eq!(ResourceKind::Persistent.as_str(), "persistent");
    }

    #[test]
    fn zero_resource_amounts_are_empty() {
        let amounts = ResourceAmounts::zero();

        assert!(amounts.is_zero());
        assert_eq!(
            amounts.get(ResourceKind::Host),
            0
        );
        assert_eq!(
            amounts.get(ResourceKind::Device),
            0
        );
    }

    #[test]
    fn resource_amounts_can_hold_multiple_domains() {
        let amounts = ResourceAmounts::zero()
            .with(ResourceKind::Host, 100)
            .with(ResourceKind::Device, 200)
            .with(ResourceKind::Temporary, 50);

        assert_eq!(amounts.get(ResourceKind::Host), 100);
        assert_eq!(amounts.get(ResourceKind::Device), 200);
        assert_eq!(amounts.get(ResourceKind::Temporary), 50);
        assert_eq!(
            amounts.checked_total().unwrap(),
            350
        );
    }

    #[test]
    fn resource_amount_addition_is_checked() {
        let lhs = ResourceAmounts::single(
            ResourceKind::Host,
            usize::MAX,
        );

        let rhs = ResourceAmounts::single(
            ResourceKind::Host,
            1,
        );

        let error = lhs.checked_add(&rhs).unwrap_err();

        assert_eq!(
            error,
            ReservationError::ArithmeticOverflow {
                operation: ArithmeticOperation::Add,
                resource: Some(ResourceKind::Host),
            }
        );
    }

    #[test]
    fn resource_amount_subtraction_is_checked() {
        let lhs = ResourceAmounts::single(
            ResourceKind::Host,
            10,
        );

        let rhs = ResourceAmounts::single(
            ResourceKind::Host,
            11,
        );

        let error = lhs.checked_sub(&rhs).unwrap_err();

        assert_eq!(
            error,
            ReservationError::ArithmeticUnderflow {
                operation: ArithmeticOperation::Subtract,
                resource: Some(ResourceKind::Host),
            }
        );
    }

    #[test]
    fn empty_request_is_rejected() {
        let manager = ReservationManager::new(host_capacity(1024));

        let result = manager.reserve(ReservationRequest::empty());

        assert_eq!(
            result.unwrap_err(),
            ReservationError::EmptyRequest
        );
    }

    #[test]
    fn reservation_succeeds_within_capacity() {
        let manager = ReservationManager::new(host_capacity(1024));

        let reservation = manager
            .reserve(host_request(512))
            .unwrap();

        assert!(reservation.is_reserved());

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            512
        );

        assert_eq!(
            manager.committed().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn reservation_exceeding_capacity_is_rejected() {
        let manager = ReservationManager::new(host_capacity(1024));

        let result = manager.reserve(host_request(1025));

        match result {
            Err(ReservationError::CapacityExceeded {
                resource,
                committed,
                reserved,
                requested,
                capacity,
            }) => {
                assert_eq!(resource, ResourceKind::Host);
                assert_eq!(committed, 0);
                assert_eq!(reserved, 0);
                assert_eq!(requested, 1025);
                assert_eq!(capacity, 1024);
            }

            other => panic!(
                "unexpected reservation result: {other:?}"
            ),
        }

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn multiple_reservations_consume_capacity() {
        let manager = ReservationManager::new(host_capacity(1000));

        let first = manager
            .reserve(host_request(400))
            .unwrap();

        let second = manager
            .reserve(host_request(500))
            .unwrap();

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            900
        );

        drop(first);
        drop(second);

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn reservation_drop_rolls_back() {
        let manager = ReservationManager::new(host_capacity(1000));

        {
            let _reservation = manager
                .reserve(host_request(600))
                .unwrap();

            assert_eq!(
                manager.reserved().unwrap().amount(ResourceKind::Host),
                600
            );
        }

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            0
        );

        assert_eq!(
            manager.committed().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn commit_moves_reserved_to_committed() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(600))
            .unwrap();

        reservation.commit().unwrap();

        assert!(reservation.is_committed());

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            0
        );

        assert_eq!(
            manager.committed().unwrap().amount(ResourceKind::Host),
            600
        );
    }

    #[test]
    fn committed_token_drop_releases_committed_capacity() {
        let manager = ReservationManager::new(host_capacity(1000));

        {
            let mut reservation = manager
                .reserve(host_request(600))
                .unwrap();

            reservation.commit().unwrap();

            assert_eq!(
                manager.committed().unwrap().amount(ResourceKind::Host),
                600
            );
        }

        assert_eq!(
            manager.committed().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn explicit_release_of_committed_reservation_works() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(600))
            .unwrap();

        reservation.commit().unwrap();
        reservation.release().unwrap();

        assert!(reservation.is_released());

        assert_eq!(
            manager.committed().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn explicit_rollback_releases_reserved_capacity() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(600))
            .unwrap();

        reservation.rollback().unwrap();

        assert!(reservation.is_released());

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn double_commit_is_rejected() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(100))
            .unwrap();

        reservation.commit().unwrap();

        let error = reservation.commit().unwrap_err();

        assert_eq!(
            error,
            ReservationError::AlreadyCommitted {
                reservation: reservation.id(),
            }
        );
    }

    #[test]
    fn release_after_release_is_rejected() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(100))
            .unwrap();

        reservation.release().unwrap();

        let error = reservation.release().unwrap_err();

        assert_eq!(
            error,
            ReservationError::AlreadyReleased {
                reservation: reservation.id(),
            }
        );
    }

    #[test]
    fn rollback_after_commit_is_rejected() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(100))
            .unwrap();

        reservation.commit().unwrap();

        let error = reservation.rollback().unwrap_err();

        assert_eq!(
            error,
            ReservationError::InvalidTransition {
                reservation: reservation.id(),
                current: ReservationState::Committed,
                operation: "rollback",
            }
        );
    }

    #[test]
    fn commit_after_release_is_rejected() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(100))
            .unwrap();

        reservation.release().unwrap();

        let error = reservation.commit().unwrap_err();

        assert_eq!(
            error,
            ReservationError::InvalidTransition {
                reservation: reservation.id(),
                current: ReservationState::Released,
                operation: "commit",
            }
        );
    }

    #[test]
    fn available_capacity_is_correct() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut first = manager
            .reserve(host_request(200))
            .unwrap();

        let second = manager
            .reserve(host_request(300))
            .unwrap();

        first.commit().unwrap();

        let available = manager.available().unwrap();

        assert_eq!(
            available.amount(ResourceKind::Host),
            500
        );

        drop(second);

        let available = manager.available().unwrap();

        assert_eq!(
            available.amount(ResourceKind::Host),
            800
        );

        drop(first);
    }

    #[test]
    fn can_reserve_is_only_a_probe() {
        let manager = ReservationManager::new(host_capacity(1000));

        assert!(
            manager
                .can_reserve(host_request(1000))
                .unwrap()
        );

        assert!(
            !manager
                .can_reserve(host_request(1001))
                .unwrap()
        );

        let reservation = manager
            .reserve(host_request(500))
            .unwrap();

        assert!(
            !manager
                .can_reserve(host_request(501))
                .unwrap()
        );

        drop(reservation);
    }

    #[test]
    fn multi_resource_reservation_is_atomic() {
        let capacity = ReservationCapacity::zero()
            .with(ResourceKind::Host, 1000)
            .with(ResourceKind::Device, 500);

        let manager = ReservationManager::new(capacity);

        let request = ReservationRequest::from_amounts(
            ResourceAmounts::zero()
                .with(ResourceKind::Host, 800)
                .with(ResourceKind::Device, 600),
        );

        let result = manager.reserve(request);

        assert!(result.is_err());

        let snapshot = manager.snapshot().unwrap();

        assert_eq!(
            snapshot.reserved.amount(ResourceKind::Host),
            0
        );

        assert_eq!(
            snapshot.reserved.amount(ResourceKind::Device),
            0
        );
    }

    #[test]
    fn multi_resource_reservation_succeeds() {
        let capacity = ReservationCapacity::zero()
            .with(ResourceKind::Host, 1000)
            .with(ResourceKind::Device, 500)
            .with(ResourceKind::Temporary, 250);

        let manager = ReservationManager::new(capacity);

        let request = ReservationRequest::from_amounts(
            ResourceAmounts::zero()
                .with(ResourceKind::Host, 800)
                .with(ResourceKind::Device, 400)
                .with(ResourceKind::Temporary, 200),
        );

        let reservation = manager.reserve(request).unwrap();

        let amounts = reservation.amounts();

        assert_eq!(
            amounts.get(ResourceKind::Host),
            800
        );

        assert_eq!(
            amounts.get(ResourceKind::Device),
            400
        );

        assert_eq!(
            amounts.get(ResourceKind::Temporary),
            200
        );
    }

    #[test]
    fn metadata_is_preserved() {
        let manager = ReservationManager::new(host_capacity(1000));

        let metadata = ReservationMetadata::new(
            Some("state-vector-allocation".to_owned()),
            Some("simulator".to_owned()),
        );

        let reservation = manager
            .reserve_with_metadata(
                host_request(100),
                metadata,
            )
            .unwrap();

        assert_eq!(
            reservation.metadata().operation_name(),
            Some("state-vector-allocation")
        );

        assert_eq!(
            reservation.metadata().subsystem_name(),
            Some("simulator")
        );
    }

    #[test]
    fn reservation_ids_are_unique() {
        let manager = ReservationManager::new(host_capacity(10_000));

        let first = manager
            .reserve(host_request(100))
            .unwrap();

        let second = manager
            .reserve(host_request(100))
            .unwrap();

        assert_ne!(first.id(), second.id());

        assert_eq!(first.id().get(), 1);
        assert_eq!(second.id().get(), 2);
    }

    #[test]
    fn reservation_ids_are_manager_local() {
        let first_manager = ReservationManager::new(host_capacity(1000));
        let second_manager = ReservationManager::new(host_capacity(1000));

        let first = first_manager
            .reserve(host_request(100))
            .unwrap();

        let second = second_manager
            .reserve(host_request(100))
            .unwrap();

        assert_eq!(first.id().get(), 1);
        assert_eq!(second.id().get(), 1);
    }

    #[test]
    fn reservation_manager_is_cloneable() {
        let manager = ReservationManager::new(host_capacity(1000));
        let clone = manager.clone();

        let reservation = clone
            .reserve(host_request(500))
            .unwrap();

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            500
        );

        drop(reservation);
    }

    #[test]
    fn manager_clone_shares_accounting_state() {
        let manager = ReservationManager::new(host_capacity(1000));
        let clone = manager.clone();

        let mut reservation = manager
            .reserve(host_request(300))
            .unwrap();

        reservation.commit().unwrap();

        assert_eq!(
            clone.committed().unwrap().amount(ResourceKind::Host),
            300
        );

        reservation.release().unwrap();

        assert_eq!(
            clone.committed().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn unlimited_capacity_accepts_large_valid_requests() {
        let manager = ReservationManager::unlimited();

        let request = ReservationRequest::single(
            ResourceKind::Host,
            usize::MAX,
        );

        let reservation = manager.reserve(request).unwrap();

        assert_eq!(
            reservation.amounts().get(ResourceKind::Host),
            usize::MAX
        );
    }

    #[test]
    fn snapshot_is_independent_of_future_changes() {
        let manager = ReservationManager::new(host_capacity(1000));

        let reservation = manager
            .reserve(host_request(400))
            .unwrap();

        let snapshot = manager.snapshot().unwrap();

        assert_eq!(
            snapshot.reserved.amount(ResourceKind::Host),
            400
        );

        drop(reservation);

        assert_eq!(
            snapshot.reserved.amount(ResourceKind::Host),
            400
        );

        assert_eq!(
            manager.reserved().unwrap().amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn request_builder_is_checked() {
        let request = ReservationRequest::single(
            ResourceKind::Host,
            usize::MAX,
        );

        let result = request.try_with(
            ResourceKind::Host,
            1,
        );

        assert!(matches!(
            result,
            Err(ReservationError::ArithmeticOverflow {
                operation: ArithmeticOperation::Add,
                resource: Some(ResourceKind::Host),
            })
        ));
    }

    #[test]
    fn capacity_zero_rejects_nonzero_request() {
        let manager = ReservationManager::new(
            ReservationCapacity::zero(),
        );

        let result = manager.reserve(host_request(1));

        assert!(matches!(
            result,
            Err(ReservationError::CapacityExceeded {
                resource: ResourceKind::Host,
                ..
            })
        ));
    }

    #[test]
    fn separate_resource_capacities_are_independent() {
        let capacity = ReservationCapacity::zero()
            .with(ResourceKind::Host, 100)
            .with(ResourceKind::Device, 1000);

        let manager = ReservationManager::new(capacity);

        let host = manager
            .reserve(
                ReservationRequest::single(
                    ResourceKind::Host,
                    100,
                ),
            )
            .unwrap();

        let device = manager
            .reserve(
                ReservationRequest::single(
                    ResourceKind::Device,
                    1000,
                ),
            )
            .unwrap();

        assert_eq!(
            manager.reserved().unwrap().get(ResourceKind::Host),
            100
        );

        assert_eq!(
            manager.reserved().unwrap().get(ResourceKind::Device),
            1000
        );

        drop(host);
        drop(device);
    }

    #[test]
    fn concurrent_reservations_never_exceed_capacity() {
        use std::sync::Arc;
        use std::thread;

        let manager = Arc::new(
            ReservationManager::new(host_capacity(1000)),
        );

        let mut handles = Vec::new();

        for _ in 0..16 {
            let manager = Arc::clone(&manager);

            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    if let Ok(reservation) =
                        manager.reserve(host_request(100))
                    {
                        // The reservation is intentionally dropped here,
                        // exercising automatic rollback under concurrency.
                        drop(reservation);
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(
            manager
                .reserved()
                .unwrap()
                .amount(ResourceKind::Host),
            0
        );

        assert_eq!(
            manager
                .committed()
                .unwrap()
                .amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn concurrent_commits_and_releases_preserve_accounting() {
        use std::sync::Arc;
        use std::thread;

        let manager = Arc::new(
            ReservationManager::new(host_capacity(10_000)),
        );

        let mut handles = Vec::new();

        for _ in 0..8 {
            let manager = Arc::clone(&manager);

            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    if let Ok(mut reservation) =
                        manager.reserve(host_request(10))
                    {
                        if reservation.commit().is_ok() {
                            let _ = reservation.release();
                        }
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(
            manager
                .reserved()
                .unwrap()
                .amount(ResourceKind::Host),
            0
        );

        assert_eq!(
            manager
                .committed()
                .unwrap()
                .amount(ResourceKind::Host),
            0
        );
    }

    #[test]
    fn request_amounts_are_copyable() {
        let request = host_request(123);

        let first = request.amounts();
        let second = request.amounts();

        assert_eq!(first, second);
    }

    #[test]
    fn committed_resources_block_new_reservations() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(1000))
            .unwrap();

        reservation.commit().unwrap();

        let result = manager.reserve(host_request(1));

        assert!(matches!(
            result,
            Err(ReservationError::CapacityExceeded {
                resource: ResourceKind::Host,
                committed: 1000,
                reserved: 0,
                requested: 1,
                capacity: 1000,
            })
        );

        reservation.release().unwrap();
    }

    #[test]
    fn rollback_restores_capacity_for_reuse() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(1000))
            .unwrap();

        reservation.rollback().unwrap();

        let second = manager
            .reserve(host_request(1000))
            .unwrap();

        assert_eq!(
            second.amounts().get(ResourceKind::Host),
            1000
        );
    }

    #[test]
    fn release_restores_committed_capacity_for_reuse() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut first = manager
            .reserve(host_request(1000))
            .unwrap();

        first.commit().unwrap();
        first.release().unwrap();

        let second = manager
            .reserve(host_request(1000))
            .unwrap();

        assert_eq!(
            second.amounts().get(ResourceKind::Host),
            1000
        );
    }

    #[test]
    fn state_transitions_are_explicit() {
        let manager = ReservationManager::new(host_capacity(1000));

        let mut reservation = manager
            .reserve(host_request(100))
            .unwrap();

        assert_eq!(
            reservation.state(),
            ReservationState::Reserved
        );

        reservation.commit().unwrap();

        assert_eq!(
            reservation.state(),
            ReservationState::Committed
        );

        reservation.release().unwrap();

        assert_eq!(
            reservation.state(),
            ReservationState::Released
        );
    }
}