
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Provider-Neutral Memory Allocation
//!
//! Path: `src/classical/memory/allocation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a target-independent allocation boundary for classical computing.
//! Allocation requests are checked against explicit policy, resource
//! accounting is reserved before backend allocation, and successful
//! allocations retain ownership of their backend storage until released.
//!
//! # Authority
//!
//! - `grammar/memory/allocation.g4` owns allocation-intent syntax.
//! - `grammar/memory/memory.g4` owns common memory syntax.
//! - `grammar/memory/deallocation.g4` owns deallocation-intent syntax.
//! - `grammar/specification/semantic-model.md` owns normative semantics.
//! - `src/zuir/ids.rs` owns canonical ZUIR identifiers.
//! - `src/zuir/ir/ownership.rs` owns canonical ownership contracts.
//! - `src/zuir/ir/region.rs` owns canonical structural IR regions.
//! - Semantic analysis and verification own lifetime and borrow validity.
//! - The selected allocation backend owns the actual storage mechanism.
//!
//! # Owns
//!
//! - Allocation requests and allocation-specific errors.
//! - Configurable allocation budgets.
//! - Thread-safe accounting for live allocations.
//! - The provider-neutral backend contract.
//! - Safe host allocation using fallible capacity reservation.
//! - Allocation handles that retain and release backend storage.
//!
//! # Does not own
//!
//! - Source grammar, AST nodes, or canonical IR.
//! - Ownership, borrowing, lifetime inference, or alias analysis.
//! - Canonical ZUIR identity types.
//! - Address-space semantics or physical addresses.
//! - Device discovery, placement, scheduling, or distributed coordination.
//! - Quantum-state representations or quantum-specific memory policies.
//! - A universal allocator for every target.
//! - Runtime-specific collection, tensor, or numerical semantics.
//!
//! # Resource and scalability contract
//!
//! There are no built-in maximum allocation sizes, allocation counts,
//! hardware counts, or device counts. Optional budgets are supplied by the
//! caller. If a budget is absent, this layer does not impose that budget.
//!
//! Actual limits still exist: host address-space limits, backend capabilities,
//! available resources, integer representation, and configured policy.
//! Requests that cannot be represented or satisfied fail explicitly.
//!
//! This API cannot guarantee recovery from process-wide allocator aborts or
//! operating-system termination. Fallible reservation handles reportable
//! capacity failures without using unsafe Rust.
//!
//! # Integration contract
//!
//! - `memory/mod.rs` declares `pub mod allocation;`.
//! - `memory/ownership.rs` remains the classical adapter for canonical
//!   ownership contracts; this module does not replace it.
//! - `memory/borrowing.rs`, `regions.rs`, and the lifetime verifier establish
//!   semantic validity before memory operations are executed.
//! - A future `address_space.rs` may map resolved semantic spaces to backend
//!   identifiers. This module treats space names as opaque identifiers.
//! - Lowering converts validated allocation intent into an `AllocationRequest`
//!   only when the target's requirements and capabilities have been resolved.
//! - A backend must be selected explicitly. This module does not silently
//!   substitute host storage for an unsupported device or distributed space.
//! - The existing `src/quantum/memory/allocator.rs` remains a separate
//!   quantum-memory implementation until a deliberate shared-allocator
//!   refactor establishes one canonical cross-domain API.
//!
//! # Failure and ownership guarantees
//!
//! - Invalid requests do not change accounting.
//! - Budget failures do not call the backend.
//! - Backend failures roll back the reservation.
//! - A backend returning a mismatched allocation size is rejected.
//! - An allocation handle is not `Clone`.
//! - Dropping a live handle releases storage and updates accounting.
//! - Accounting uses checked arithmetic; overflow is an error.
//! - Poisoned accounting locks are reported by fallible public operations.
//!
//! # Freeze rule
//!
//! Keep this module below semantic analysis and above backend implementations.
//! Any future unification with `quantum::memory::allocator` must be an
//! explicit compatibility migration, not a second implicit allocator path.

use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use thiserror::Error;

/// Result type for allocation operations.
pub type AllocationResult<T> = Result<T, AllocationError>;

/// Optional operational budgets for a memory-allocation domain.
///
/// `None` means that this policy does not set a limit for that metric.
/// It does not mean that the host or backend has unlimited resources.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AllocationLimits {
    /// Maximum aggregate live bytes tracked by this allocator.
    pub max_live_bytes: Option<u128>,

    /// Maximum number of simultaneously live allocations.
    pub max_live_allocations: Option<u128>,
}

impl AllocationLimits {
    /// Construct a policy with no additional allocator-level budgets.
    ///
    /// Backend and platform constraints still apply.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_live_bytes: None,
            max_live_allocations: None,
        }
    }

    /// Construct a policy with an explicit live-byte budget.
    #[must_use]
    pub const fn with_max_live_bytes(mut self, limit: u128) -> Self {
        self.max_live_bytes = Some(limit);
        self
    }

    /// Construct a policy with an explicit live-allocation budget.
    #[must_use]
    pub const fn with_max_live_allocations(mut self, limit: u128) -> Self {
        self.max_live_allocations = Some(limit);
        self
    }
}

/// Snapshot of the allocation accounting maintained by this allocator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AllocationAccounting {
    /// Current aggregate live bytes.
    pub live_bytes: u128,

    /// Current number of live allocation handles.
    pub live_allocations: u128,

    /// Highest aggregate live-byte count observed.
    pub peak_live_bytes: u128,

    /// Highest live-allocation count observed.
    pub peak_live_allocations: u128,
}

/// A validated request to allocate storage.
///
/// `space` and `class` are open-world identifiers. Their meanings are
/// established by semantic analysis, address-space resolution, and the
/// selected backend rather than by a closed hardware enum in this module.
///
/// `bytes` is a byte quantity, not a semantic ceiling or a guarantee that
/// the requested storage can be provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationRequest {
    bytes: u128,
    space: String,
    class: String,
    label: Option<String>,
}

impl AllocationRequest {
    /// Create an allocation request.
    ///
    /// Space and class identifiers must be nonempty after trimming.
    /// Zero-byte allocations are permitted by this API.
    pub fn new(
        bytes: u128,
        space: impl Into<String>,
        class: impl Into<String>,
    ) -> AllocationResult<Self> {
        let space = space.into();
        let class = class.into();

        if space.trim().is_empty() {
            return Err(AllocationError::InvalidRequest {
                reason: "memory-space identifier must not be empty".to_owned(),
            });
        }

        if class.trim().is_empty() {
            return Err(AllocationError::InvalidRequest {
                reason: "allocation-class identifier must not be empty"
                    .to_owned(),
            });
        }

        Ok(Self {
            bytes,
            space,
            class,
            label: None,
        })
    }

    /// Create a request for the built-in host backend.
    pub fn host(bytes: u128) -> Self {
        Self {
            bytes,
            space: "host".to_owned(),
            class: "ordinary".to_owned(),
            label: None,
        }
    }

    /// Add a safe diagnostic label.
    ///
    /// Labels must not contain credentials, secrets, raw addresses, or
    /// provider-private handles.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Requested byte count.
    #[must_use]
    pub const fn bytes(&self) -> u128 {
        self.bytes
    }

    /// Opaque resolved memory-space identifier.
    #[must_use]
    pub fn space(&self) -> &str {
        &self.space
    }

    /// Opaque semantic allocation-class identifier.
    #[must_use]
    pub fn class(&self) -> &str {
        &self.class
    }

    /// Optional diagnostic label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Errors returned by the allocation boundary.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AllocationError {
    /// Request metadata is invalid.
    #[error("invalid allocation request: {reason}")]
    InvalidRequest {
        /// Explanation of the invalid field or combination.
        reason: String,
    },

    /// The selected backend does not support the requested memory space.
    #[error(
        "backend {backend:?} does not support memory space {space:?}"
    )]
    UnsupportedSpace {
        /// Selected backend identifier.
        backend: String,
        /// Requested memory-space identifier.
        space: String,
    },

    /// A configured operational budget would be exceeded.
    #[error(
        "allocation budget exceeded for {resource}: current={current}, \
         requested={requested}, limit={limit}"
    )]
    BudgetExceeded {
        /// Budget metric.
        resource: &'static str,
        /// Current accounted quantity.
        current: u128,
        /// Requested additional quantity.
        requested: u128,
        /// Configured maximum.
        limit: u128,
    },

    /// Checked arithmetic could not represent an accounting operation.
    #[error("allocation accounting overflow during {operation}")]
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },

    /// A backend could not satisfy the request.
    #[error(
        "backend {backend:?} failed to allocate {requested_bytes} bytes: \
         {reason}"
    )]
    BackendFailure {
        /// Backend identifier.
        backend: String,
        /// Requested number of bytes.
        requested_bytes: u128,
        /// Safe diagnostic explanation.
        reason: String,
    },

    /// A backend returned storage with an unexpected byte length.
    #[error(
        "backend returned {actual_bytes} bytes for a {requested_bytes}-byte \
         request"
    )]
    BackendSizeMismatch {
        /// Requested byte count.
        requested_bytes: u128,
        /// Actual byte count reported by the backend.
        actual_bytes: u128,
    },

    /// The request cannot be represented by the selected host platform.
    #[error(
        "requested byte count {requested_bytes} cannot be represented by \
         this host platform"
    )]
    PlatformLimit {
        /// Requested byte count.
        requested_bytes: u128,
    },

    /// The internal accounting lock was poisoned.
    #[error("allocation accounting lock is poisoned")]
    AccountingPoisoned,

    /// Internal accounting invariants were violated.
    #[error("allocation accounting invariant failed: {reason}")]
    AccountingInvariant {
        /// Description of the invariant failure.
        reason: &'static str,
    },
}

/// Opaque storage owned by an allocation handle.
///
/// Backends may implement this trait for host buffers, accelerator resources,
/// distributed registrations, or opaque remote resources. The interface does
/// not expose raw pointers or assume that storage is host-addressable.
pub trait AllocationStorage: Send + Sync + 'static {
    /// Actual byte count represented by this storage.
    fn byte_len(&self) -> u128;

    /// Resolved memory-space identifier.
    fn space(&self) -> &str;
}

/// Safe backend contract for allocating opaque storage.
///
/// Implementations must either return a storage object matching the request
/// or return an error. They must not expose unsafe operations through this
/// interface.
pub trait AllocationBackend: Send + Sync + 'static {
    /// Stable backend identifier for diagnostics.
    fn backend_id(&self) -> &str;

    /// Whether this backend can attempt the requested memory space.
    ///
    /// This is a capability query, not a guarantee of available capacity.
    fn supports_space(&self, space: &str) -> bool;

    /// Allocate storage for a validated request.
    fn allocate(
        &self,
        request: &AllocationRequest,
    ) -> AllocationResult<Box<dyn AllocationStorage>>;
}

struct AllocatorInner {
    backend: Arc<dyn AllocationBackend>,
    limits: AllocationLimits,
    accounting: Mutex<AllocationAccounting>,
}

impl fmt::Debug for AllocatorInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AllocatorInner")
            .field("backend", &self.backend.backend_id())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}

/// Cloneable allocator facade sharing one backend and accounting domain.
///
/// Clones share accounting and budgets. Each successful allocation is owned
/// independently by its returned handle.
#[derive(Clone)]
pub struct MemoryAllocator {
    inner: Arc<AllocatorInner>,
}

impl fmt::Debug for MemoryAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryAllocator")
            .field("backend", &self.inner.backend.backend_id())
            .field("limits", &self.inner.limits)
            .finish()
    }
}

impl MemoryAllocator {
    /// Construct an allocator with an explicit backend and budget policy.
    #[must_use]
    pub fn new(
        backend: Arc<dyn AllocationBackend>,
        limits: AllocationLimits,
    ) -> Self {
        Self {
            inner: Arc::new(AllocatorInner {
                backend,
                limits,
                accounting: Mutex::new(AllocationAccounting::default()),
            }),
        }
    }

    /// Construct an allocator using the safe standard-library host backend.
    #[must_use]
    pub fn host(limits: AllocationLimits) -> Self {
        Self::new(Arc::new(HostAllocationBackend), limits)
    }

    /// Identifier of the selected backend.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        self.inner.backend.backend_id()
    }

    /// Configured budget policy.
    #[must_use]
    pub const fn limits(&self) -> AllocationLimits {
        self.inner.limits
    }

    /// Return a consistent snapshot of current and peak accounting.
    pub fn accounting(&self) -> AllocationResult<AllocationAccounting> {
        let state = self.lock_accounting()?;
        Ok(*state)
    }

    /// Allocate storage after checking policy and reserving accounting.
    ///
    /// Reservation happens before the backend call. A backend failure or
    /// size mismatch rolls the reservation back before returning the error.
    pub fn allocate(
        &self,
        request: AllocationRequest,
    ) -> AllocationResult<MemoryAllocation> {
        if !self.inner.backend.supports_space(request.space()) {
            return Err(AllocationError::UnsupportedSpace {
                backend: self.inner.backend.backend_id().to_owned(),
                space: request.space().to_owned(),
            });
        }

        self.reserve(request.bytes())?;

        let storage = match self.inner.backend.allocate(&request) {
            Ok(storage) => storage,
            Err(error) => {
                self.release_accounting(request.bytes());
                return Err(error);
            }
        };

        let actual_bytes = storage.byte_len();
        if actual_bytes != request.bytes() {
            drop(storage);
            self.release_accounting(request.bytes());

            return Err(AllocationError::BackendSizeMismatch {
                requested_bytes: request.bytes(),
                actual_bytes,
            });
        }

        if storage.space() != request.space() {
            let actual_space = storage.space().to_owned();
            drop(storage);
            self.release_accounting(request.bytes());

            return Err(AllocationError::InvalidRequest {
                reason: format!(
                    "backend returned storage in space {actual_space:?}, \
                     expected {:?}",
                    request.space()
                ),
            });
        }

        Ok(MemoryAllocation {
            storage: Some(storage),
            accounting: Arc::clone(&self.inner),
            bytes: request.bytes(),
            label: request.label().map(str::to_owned),
            accounted: true,
        })
    }

    fn lock_accounting(
        &self,
    ) -> AllocationResult<MutexGuard<'_, AllocationAccounting>> {
        self.inner
            .accounting
            .lock()
            .map_err(|_| AllocationError::AccountingPoisoned)
    }

    fn reserve(&self, bytes: u128) -> AllocationResult<()> {
        let mut state = self.lock_accounting()?;

        if let Some(limit) = self.inner.limits.max_live_allocations {
            if state.live_allocations >= limit {
                return Err(AllocationError::BudgetExceeded {
                    resource: "live allocations",
                    current: state.live_allocations,
                    requested: 1,
                    limit,
                });
            }
        }

        let new_bytes = state.live_bytes.checked_add(bytes).ok_or(
            AllocationError::ArithmeticOverflow {
                operation: "live-byte reservation",
            },
        )?;

        let new_allocations = state.live_allocations.checked_add(1).ok_or(
            AllocationError::ArithmeticOverflow {
                operation: "live-allocation reservation",
            },
        )?;

        if let Some(limit) = self.inner.limits.max_live_bytes {
            if new_bytes > limit {
                return Err(AllocationError::BudgetExceeded {
                    resource: "live bytes",
                    current: state.live_bytes,
                    requested: bytes,
                    limit,
                });
            }
        }

        state.live_bytes = new_bytes;
        state.live_allocations = new_allocations;
        state.peak_live_bytes = state.peak_live_bytes.max(new_bytes);
        state.peak_live_allocations =
            state.peak_live_allocations.max(new_allocations);

        Ok(())
    }

    fn release_accounting(&self, bytes: u128) {
        // This path is used during rollback. It must not panic while handling
        // another allocation error.
        if let Ok(mut state) = self.inner.accounting.lock() {
            if let Some(live_bytes) = state.live_bytes.checked_sub(bytes) {
                if let Some(live_allocations) =
                    state.live_allocations.checked_sub(1)
                {
                    state.live_bytes = live_bytes;
                    state.live_allocations = live_allocations;
                }
            }
        }
    }
}

/// An owned allocation handle.
///
/// This type intentionally does not implement `Clone`: ownership of the
/// underlying resource must remain explicit. Dropping the handle releases
/// its storage and decrements accounting.
pub struct MemoryAllocation {
    storage: Option<Box<dyn AllocationStorage>>,
    accounting: Arc<AllocatorInner>,
    bytes: u128,
    label: Option<String>,
    accounted: bool,
}

impl fmt::Debug for MemoryAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryAllocation")
            .field("bytes", &self.bytes)
            .field("space", &self.space())
            .field("label", &self.label)
            .field("is_live", &self.is_live())
            .finish()
    }
}

impl MemoryAllocation {
    /// Requested and accounted byte count.
    #[must_use]
    pub const fn byte_len(&self) -> u128 {
        self.bytes
    }

    /// Whether this handle still owns its allocation.
    #[must_use]
    pub fn is_live(&self) -> bool {
        self.storage.is_some()
    }

    /// Resolved storage space, if the allocation remains live.
    #[must_use]
    pub fn space(&self) -> Option<&str> {
        self.storage.as_ref().map(|storage| storage.space())
    }

    /// Safe diagnostic label, if supplied.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Borrow the opaque storage interface while this handle is live.
    ///
    /// Access to actual bytes or provider-specific operations belongs to
    /// backend-specific safe interfaces, not this generic boundary.
    #[must_use]
    pub fn storage(&self) -> Option<&dyn AllocationStorage> {
        self.storage.as_deref()
    }

    /// Release the allocation immediately.
    ///
    /// This consumes the handle. Storage is dropped before accounting is
    /// decremented, so the resource is not reported as free while it is
    /// still owned by this handle.
    pub fn release(mut self) {
        self.release_inner();
    }

    fn release_inner(&mut self) {
        if self.storage.take().is_some() && self.accounted {
            release_shared_accounting(&self.accounting, self.bytes);
            self.accounted = false;
        }
    }
}

impl Drop for MemoryAllocation {
    fn drop(&mut self) {
        self.release_inner();
    }
}

fn release_shared_accounting(inner: &AllocatorInner, bytes: u128) {
    // Destructors must not panic. If the accounting mutex has been poisoned,
    // retain process safety; the next explicit accounting operation reports
    // the poison rather than inventing a repaired value.
    if let Ok(mut state) = inner.accounting.lock() {
        if let (Some(live_bytes), Some(live_allocations)) = (
            state.live_bytes.checked_sub(bytes),
            state.live_allocations.checked_sub(1),
        ) {
            state.live_bytes = live_bytes;
            state.live_allocations = live_allocations;
        }
    }
}

/// Safe host-memory backend implemented with standard-library containers.
///
/// This backend supports only the resolved `"host"` space. Other spaces must
/// be handled by explicitly registered backend implementations.
#[derive(Debug, Default)]
pub struct HostAllocationBackend;

impl AllocationBackend for HostAllocationBackend {
    fn backend_id(&self) -> &str {
        "zamani.classical.host"
    }

    fn supports_space(&self, space: &str) -> bool {
        space == "host"
    }

    fn allocate(
        &self,
        request: &AllocationRequest,
    ) -> AllocationResult<Box<dyn AllocationStorage>> {
        let length = usize::try_from(request.bytes()).map_err(|_| {
            AllocationError::PlatformLimit {
                requested_bytes: request.bytes(),
            }
        })?;

        let mut bytes = Vec::<u8>::new();
        bytes.try_reserve_exact(length).map_err(|error| {
            AllocationError::BackendFailure {
                backend: self.backend_id().to_owned(),
                requested_bytes: request.bytes(),
                reason: error.to_string(),
            }
        })?;

        // Capacity has been reserved fallibly; resizing within that capacity
        // initializes the requested bytes without another capacity growth.
        bytes.resize(length, 0);

        Ok(Box::new(HostAllocationStorage { bytes }))
    }
}

struct HostAllocationStorage {
    bytes: Vec<u8>,
}

impl AllocationStorage for HostAllocationStorage {
    fn byte_len(&self) -> u128 {
        self.bytes.len() as u128
    }

    fn space(&self) -> &str {
        "host"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_and_releases_host_storage() {
        let allocator = MemoryAllocator::host(AllocationLimits::unbounded());

        let allocation = allocator
            .allocate(AllocationRequest::host(128))
            .expect("host allocation");

        assert!(allocation.is_live());
        assert_eq!(allocation.byte_len(), 128);
        assert_eq!(allocation.space(), Some("host"));

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 128);
        assert_eq!(snapshot.live_allocations, 1);

        allocation.release();

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
        assert_eq!(snapshot.peak_live_bytes, 128);
    }

    #[test]
    fn dropping_handle_releases_accounting() {
        let allocator = MemoryAllocator::host(AllocationLimits::unbounded());

        {
            let _allocation = allocator
                .allocate(AllocationRequest::host(32))
                .expect("host allocation");

            assert_eq!(
                allocator.accounting().expect("accounting").live_bytes,
                32
            );
        }

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    #[test]
    fn byte_budget_is_enforced_before_backend_allocation() {
        let limits =
            AllocationLimits::unbounded().with_max_live_bytes(10);
        let allocator = MemoryAllocator::host(limits);

        let error = allocator
            .allocate(AllocationRequest::host(11))
            .expect_err("budget should reject allocation");

        assert!(matches!(
            error,
            AllocationError::BudgetExceeded {
                resource: "live bytes",
                ..
            }
        ));

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    #[test]
    fn allocation_count_budget_is_enforced() {
        let limits =
            AllocationLimits::unbounded().with_max_live_allocations(1);
        let allocator = MemoryAllocator::host(limits);

        let first = allocator
            .allocate(AllocationRequest::host(1))
            .expect("first allocation");

        let error = allocator
            .allocate(AllocationRequest::host(1))
            .expect_err("second allocation should exceed budget");

        assert!(matches!(
            error,
            AllocationError::BudgetExceeded {
                resource: "live allocations",
                ..
            }
        ));

        drop(first);

        let second = allocator
            .allocate(AllocationRequest::host(1))
            .expect("allocation after release");
        assert!(second.is_live());
    }

    #[test]
    fn failed_budget_request_does_not_change_accounting() {
        let limits =
            AllocationLimits::unbounded().with_max_live_bytes(5);
        let allocator = MemoryAllocator::host(limits);

        let first = allocator
            .allocate(AllocationRequest::host(4))
            .expect("first allocation");

        assert!(allocator
            .allocate(AllocationRequest::host(2))
            .is_err());

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 4);
        assert_eq!(snapshot.live_allocations, 1);

        drop(first);

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    #[test]
    fn zero_byte_allocation_is_supported() {
        let allocator = MemoryAllocator::host(AllocationLimits::unbounded());

        let allocation = allocator
            .allocate(AllocationRequest::host(0))
            .expect("zero-byte allocation");

        assert!(allocation.is_live());
        assert_eq!(allocation.byte_len(), 0);
        assert_eq!(
            allocator.accounting().expect("accounting").live_allocations,
            1
        );

        drop(allocation);

        assert_eq!(
            allocator.accounting().expect("accounting").live_allocations,
            0
        );
    }

    #[test]
    fn unknown_spaces_are_not_silently_mapped_to_host() {
        let allocator = MemoryAllocator::host(AllocationLimits::unbounded());
        let request = AllocationRequest::new(16, "device::future", "ordinary")
            .expect("valid open-world request");

        let error = allocator
            .allocate(request)
            .expect_err("host backend cannot satisfy device space");

        assert!(matches!(
            error,
            AllocationError::UnsupportedSpace { .. }
        ));

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    #[test]
    fn invalid_request_metadata_is_rejected() {
        assert!(AllocationRequest::new(1, " ", "ordinary").is_err());
        assert!(AllocationRequest::new(1, "host", "").is_err());
    }

    #[test]
    fn allocator_clones_share_accounting() {
        let allocator = MemoryAllocator::host(AllocationLimits::unbounded());
        let clone = allocator.clone();

        let allocation = clone
            .allocate(AllocationRequest::host(64))
            .expect("allocation");

        assert_eq!(
            allocator.accounting().expect("accounting").live_bytes,
            64
        );

        drop(allocation);

        assert_eq!(
            allocator.accounting().expect("accounting").live_bytes,
            0
        );
    }

    #[test]
    fn peak_accounting_is_retained_after_release() {
        let allocator = MemoryAllocator::host(AllocationLimits::unbounded());

        let first = allocator
            .allocate(AllocationRequest::host(10))
            .expect("first allocation");
        drop(first);

        let second = allocator
            .allocate(AllocationRequest::host(25))
            .expect("second allocation");
        drop(second);

        let snapshot = allocator.accounting().expect("accounting");
        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.peak_live_bytes, 25);
        assert_eq!(snapshot.peak_live_allocations, 1);
    }
}
