//! Zamani Quantum Memory — Allocation Integration Tests
//!
//! Production integration tests for:
//!
//! `quantum::memory::allocator`
//!
//! # Responsibility
//!
//! This test module verifies the public allocation contract without reaching
//! into allocator internals.
//!
//! It verifies:
//!
//! - provider-neutral allocation;
//! - host allocation;
//! - device/accelerator provider registration;
//! - unified-memory provider registration;
//! - distributed-memory provider registration;
//! - backend/QPU-native provider registration;
//! - allocation ownership;
//! - explicit release;
//! - drop-based release;
//! - allocation accounting;
//! - peak accounting;
//! - allocation identity;
//! - allocator cloning/shared ownership;
//! - provider replacement safety;
//! - zero-byte rejection;
//! - resource-limit enforcement;
//! - allocation-count enforcement;
//! - state-memory accounting;
//! - state-element accounting;
//! - provider allocation failure rollback;
//! - provider byte-length validation;
//! - provider location validation;
//! - invalid provider identifiers;
//! - concurrent allocations;
//! - deterministic accounting after concurrency;
//! - no vendor-specific assumptions.
//!
//! # Architectural rule
//!
//! These tests must test the public contract, not private implementation
//! details.
//!
//! They therefore intentionally do not:
//!
//! - access `AllocatorInner`;
//! - access allocator mutexes;
//! - access private allocation records;
//! - inspect raw pointers;
//! - use `unsafe`;
//! - depend on CUDA;
//! - depend on HIP;
//! - depend on Metal;
//! - depend on Vulkan;
//! - depend on MPI;
//! - depend on any QPU SDK;
//! - perform network I/O;
//! - require physical quantum hardware.
//!
//! Hardware providers are represented by safe test providers implementing the
//! same public `MemoryProvider` contract used by real providers.
//!
//! # Integration contract
//!
//! This file is designed against the following stable memory APIs:
//!
//! - `memory::allocator::MemoryAllocator`
//! - `memory::allocator::MemoryProvider`
//! - `memory::allocator::ProviderAllocation`
//! - `memory::allocator::AllocationRequest`
//! - `memory::allocator::AllocationClass`
//! - `memory::allocator::MemoryLocation`
//! - `memory::allocator::MemoryLocationKind`
//! - `memory::limits::MemoryLimits`
//! - `memory::types::{AllocationId, ByteCount, MemoryId}`
//! - `memory::errors::MemoryError`
//!
//! Later state, migration, GPU, distributed, QPU, routing, scheduling and
//! hardware modules must consume these contracts rather than creating
//! alternate allocation APIs.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//!
//! # Safety
//!
//! This entire test module is safe Rust.
//!
//! There is deliberately no `unsafe` block, `unsafe fn`, raw pointer, FFI,
//! transmute, or architecture-specific intrinsic.
//!
//! This is important because `quantum::memory` promises that its public
//! allocation boundary does not expose unsafe memory ownership to higher
//! quantum layers.

// =============================================================================
// Imports
// =============================================================================

use std::sync::Arc;
use std::thread;

use crate::quantum::memory::allocator::{
    AllocationClass,
    AllocationRequest,
    HostMemoryProvider,
    MemoryAllocator,
    MemoryLocation,
    MemoryLocationKind,
    MemoryProvider,
    ProviderAllocation,
};
use crate::quantum::memory::errors::MemoryError;
use crate::quantum::memory::limits::MemoryLimits;
use crate::quantum::memory::types::{
    AllocationId,
    ByteCount,
    MemoryId,
};

// =============================================================================
// Test constants
// =============================================================================

const TEST_MEMORY_ID: u64 = 1;
const TEST_BYTES: u64 = 4096;
const TEST_STATE_ELEMENTS: u64 = 256;
const TEST_QUBITS: u64 = 8;

// =============================================================================
// Test allocation provider
// =============================================================================

/// Safe provider allocation used to test provider-neutral allocation.
///
/// The object deliberately stores metadata only rather than allocating a
/// large backing buffer. This allows device, distributed and backend-native
/// allocation tests to run on ordinary developer machines without requiring
/// those technologies to be installed.
#[derive(Debug)]
struct TestProviderAllocation {
    bytes: u64,
    location: MemoryLocation,
    label: &'static str,
}

impl ProviderAllocation for TestProviderAllocation {
    fn byte_len(&self) -> u64 {
        self.bytes
    }

    fn location(&self) -> MemoryLocation {
        self.location.clone()
    }

    fn resource_label(&self) -> Option<&str> {
        Some(self.label)
    }
}

/// Safe configurable provider used to emulate any provider-neutral memory
/// location.
///
/// This is intentionally generic. The test does not contain an IBM, Google,
/// IonQ, Quantinuum, Rigetti, CUDA, HIP, Metal, Vulkan, MPI, or other vendor
/// implementation.
///
/// A real provider only needs to implement the same public trait.
#[derive(Debug)]
struct TestProvider {
    id: &'static str,
    location: MemoryLocation,
    can_allocate: bool,
    reported_bytes: Option<u64>,
    reported_location: Option<MemoryLocation>,
    allocation_error: Option<&'static str>,
}

impl TestProvider {
    fn available(
        id: &'static str,
        location: MemoryLocation,
    ) -> Self {
        Self {
            id,
            location,
            can_allocate: true,
            reported_bytes: None,
            reported_location: None,
            allocation_error: None,
        }
    }

    fn unavailable(
        id: &'static str,
        location: MemoryLocation,
    ) -> Self {
        Self {
            id,
            location,
            can_allocate: false,
            reported_bytes: None,
            reported_location: None,
            allocation_error: None,
        }
    }

    fn failing(
        id: &'static str,
        location: MemoryLocation,
        message: &'static str,
    ) -> Self {
        Self {
            id,
            location,
            can_allocate: true,
            reported_bytes: None,
            reported_location: None,
            allocation_error: Some(message),
        }
    }

    fn wrong_size(
        id: &'static str,
        location: MemoryLocation,
        reported_bytes: u64,
    ) -> Self {
        Self {
            id,
            location,
            can_allocate: true,
            reported_bytes: Some(reported_bytes),
            reported_location: None,
            allocation_error: None,
        }
    }

    fn wrong_location(
        id: &'static str,
        location: MemoryLocation,
        reported_location: MemoryLocation,
    ) -> Self {
        Self {
            id,
            location,
            can_allocate: true,
            reported_bytes: None,
            reported_location: Some(reported_location),
            allocation_error: None,
        }
    }
}

impl MemoryProvider for TestProvider {
    fn provider_id(&self) -> &str {
        self.id
    }

    fn location(&self) -> MemoryLocation {
        self.location.clone()
    }

    fn can_allocate(&self, _bytes: u64) -> bool {
        self.can_allocate
    }

    fn allocate(
        &self,
        bytes: u64,
    ) -> Result<Box<dyn ProviderAllocation>, MemoryError> {
        if let Some(message) = self.allocation_error {
            return Err(MemoryError::BackendRejected {
                reason: message.to_owned(),
            });
        }

        let reported_bytes =
            self.reported_bytes.unwrap_or(bytes);

        let reported_location = self
            .reported_location
            .clone()
            .unwrap_or_else(|| self.location.clone());

        Ok(Box::new(TestProviderAllocation {
            bytes: reported_bytes,
            location: reported_location,
            label: self.id,
        }))
    }
}

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a production allocator with the safe host provider installed.
fn production_allocator() -> MemoryAllocator {
    MemoryAllocator::production(MemoryId::new(TEST_MEMORY_ID))
        .expect("production memory limits must be valid")
        .with_host_provider()
        .expect("host provider registration must succeed")
}

/// Creates an allocator with the restrictive policy.
///
/// The strict policy is useful for testing rejection before a provider
/// allocation occurs.
fn strict_allocator() -> MemoryAllocator {
    MemoryAllocator::new(
        MemoryId::new(TEST_MEMORY_ID),
        MemoryLimits::strict(),
    )
    .expect("strict memory limits must be valid")
    .with_host_provider()
    .expect("host provider registration must succeed")
}

/// Returns whether an error represents an allocation-limit violation.
fn is_memory_limit_error(error: &MemoryError) -> bool {
    matches!(
        error,
        MemoryError::MemoryLimitExceeded { .. }
    )
}

// =============================================================================
// Basic host allocation
// =============================================================================

#[test]
fn host_allocation_succeeds_through_public_allocator_contract() {
    let allocator = production_allocator();

    let allocation = allocator
        .allocate_host(
            ByteCount::new(TEST_BYTES),
            AllocationClass::Temporary,
        )
        .expect("host allocation should succeed");

    assert!(allocation.is_live());
    assert_eq!(allocation.byte_len(), TEST_BYTES);
    assert_eq!(allocation.provider_id(), "zamani.host");

    assert_eq!(
        allocation.request().location,
        MemoryLocation::Host
    );

    assert_eq!(
        allocation.request().class,
        AllocationClass::Temporary
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        TEST_BYTES
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        1
    );
}

// =============================================================================
// Drop/release semantics
// =============================================================================

#[test]
fn dropping_allocation_releases_provider_memory_and_accounting() {
    let allocator = production_allocator();

    {
        let allocation = allocator
            .allocate_host(
                ByteCount::new(TEST_BYTES),
                AllocationClass::Temporary,
            )
            .expect("allocation should succeed");

        assert!(allocation.is_live());

        assert_eq!(
            allocator.allocated_bytes().expect("accounting"),
            TEST_BYTES
        );

        assert_eq!(
            allocator.allocation_count().expect("accounting"),
            1
        );
    }

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

#[test]
fn explicit_release_releases_allocation_immediately() {
    let allocator = production_allocator();

    let allocation = allocator
        .allocate_host(
            ByteCount::new(TEST_BYTES),
            AllocationClass::Temporary,
        )
        .expect("allocation should succeed");

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        TEST_BYTES
    );

    allocation.release();

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

// =============================================================================
// Peak accounting
// =============================================================================

#[test]
fn peak_accounting_survives_release() {
    let allocator = production_allocator();

    {
        let first = allocator
            .allocate_host(
                ByteCount::new(TEST_BYTES),
                AllocationClass::Temporary,
            )
            .expect("first allocation");

        let second = allocator
            .allocate_host(
                ByteCount::new(TEST_BYTES),
                AllocationClass::Temporary,
            )
            .expect("second allocation");

        assert_eq!(
            allocator.allocated_bytes().expect("accounting"),
            TEST_BYTES * 2
        );

        assert_eq!(
            allocator.peak_allocated_bytes().expect("accounting"),
            TEST_BYTES * 2
        );

        drop(second);
        drop(first);
    }

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.peak_allocated_bytes().expect("accounting"),
        TEST_BYTES * 2
    );
}

// =============================================================================
// Allocation identity
// =============================================================================

#[test]
fn allocations_receive_nonzero_unique_ids() {
    let allocator = production_allocator();

    let first = allocator
        .allocate_host(
            ByteCount::new(TEST_BYTES),
            AllocationClass::Temporary,
        )
        .expect("first allocation");

    let second = allocator
        .allocate_host(
            ByteCount::new(TEST_BYTES),
            AllocationClass::Temporary,
        )
        .expect("second allocation");

    let first_id = first.id();
    let second_id = second.id();

    assert_ne!(
        first_id,
        AllocationId::new(0),
        "zero is reserved for invalid allocation identity"
    );

    assert_ne!(
        second_id,
        AllocationId::new(0),
        "zero is reserved for invalid allocation identity"
    );

    assert_ne!(
        first_id,
        second_id,
        "live allocations must never share identity"
    );
}

// =============================================================================
// Shared allocator ownership
// =============================================================================

#[test]
fn cloned_allocators_share_memory_domain_and_accounting() {
    let allocator = production_allocator();
    let clone = allocator.clone();

    assert_eq!(
        allocator.memory_id(),
        clone.memory_id()
    );

    let allocation = clone
        .allocate_host(
            ByteCount::new(TEST_BYTES),
            AllocationClass::Temporary,
        )
        .expect("allocation through clone");

    assert_eq!(
        allocator.allocated_bytes().expect("shared accounting"),
        TEST_BYTES
    );

    assert_eq!(
        clone.allocated_bytes().expect("shared accounting"),
        TEST_BYTES
    );

    drop(allocation);

    assert_eq!(
        allocator.allocated_bytes().expect("shared accounting"),
        0
    );

    assert_eq!(
        clone.allocated_bytes().expect("shared accounting"),
        0
    );
}

// =============================================================================
// Zero allocation
// =============================================================================

#[test]
fn zero_byte_allocations_are_rejected() {
    let allocator = production_allocator();

    let result = allocator.allocate_host(
        ByteCount::new(0),
        AllocationClass::Temporary,
    );

    assert!(
        matches!(
            result,
            Err(MemoryError::InvalidArgument { .. })
        ),
        "zero-byte allocations must be rejected before provider allocation"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

// =============================================================================
// State allocation
// =============================================================================

#[test]
fn state_allocations_account_for_bytes_elements_and_qubits() {
    let allocator = production_allocator();

    let allocation = allocator
        .allocate_state(
            ByteCount::new(TEST_BYTES),
            MemoryLocation::Host,
            TEST_QUBITS,
            TEST_STATE_ELEMENTS,
        )
        .expect("state allocation should succeed");

    let accounting =
        allocator.accounting().expect("allocation accounting");

    assert_eq!(
        accounting.state_bytes,
        TEST_BYTES
    );

    assert_eq!(
        accounting.state_elements,
        TEST_STATE_ELEMENTS
    );

    assert_eq!(
        accounting.allocations,
        1
    );

    assert_eq!(
        allocation.request().qubits,
        TEST_QUBITS
    );

    assert_eq!(
        allocation.request().state_elements,
        TEST_STATE_ELEMENTS
    );
}

#[test]
fn state_allocation_rejects_qubits_without_state_elements() {
    let allocator = production_allocator();

    let result = allocator.allocate(
        AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            MemoryLocation::Host,
            AllocationClass::State,
        )
        .with_qubits(TEST_QUBITS),
    );

    assert!(
        matches!(
            result,
            Err(MemoryError::InvalidArgument { .. })
        ),
        "state allocations must declare their state-element count"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

// =============================================================================
// Resource-limit enforcement
// =============================================================================

#[test]
fn strict_host_limit_is_checked_before_provider_allocation() {
    let allocator = strict_allocator();

    // Strict host capacity is 4 GiB. Requesting one byte more must fail
    // during policy validation rather than attempting a physical allocation.
    let request = AllocationRequest::new(
        ByteCount::new(4 * 1024 * 1024 * 1024 + 1),
        MemoryLocation::Host,
        AllocationClass::Persistent,
    );

    let result = allocator.allocate(request);

    assert!(
        result
            .as_ref()
            .err()
            .is_some_and(is_memory_limit_error),
        "request exceeding policy must fail before provider allocation"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

// =============================================================================
// Provider registration
// =============================================================================

#[test]
fn device_provider_can_be_registered_without_vendor_specific_core_logic() {
    let allocator = production_allocator();

    let location = MemoryLocation::Device {
        device_id: 0,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::available(
                "test.accelerator",
                location.clone(),
            ),
        ))
        .expect("device provider registration");

    assert!(
        allocator
            .has_provider(&location)
            .expect("provider lookup")
    );

    let allocation = allocator
        .allocate(AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location.clone(),
            AllocationClass::Temporary,
        ))
        .expect("device allocation");

    assert_eq!(
        allocation.provider_id(),
        "test.accelerator"
    );

    assert_eq!(
        allocation
            .provider_allocation()
            .expect("provider allocation")
            .location(),
        location
    );

    assert_eq!(
        allocation
            .provider_allocation()
            .expect("provider allocation")
            .byte_len(),
        TEST_BYTES
    );
}

#[test]
fn unified_memory_provider_is_supported_by_the_generic_contract() {
    let allocator = production_allocator();

    let location = MemoryLocation::Unified {
        device_id: 7,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::available(
                "test.unified",
                location.clone(),
            ),
        ))
        .expect("unified provider registration");

    let allocation = allocator
        .allocate(AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location.clone(),
            AllocationClass::Temporary,
        ))
        .expect("unified allocation");

    assert_eq!(
        allocation
            .provider_allocation()
            .expect("provider allocation")
            .location(),
        location
    );
}

#[test]
fn distributed_memory_provider_is_supported_by_the_generic_contract() {
    let allocator = production_allocator();

    let location = MemoryLocation::Distributed {
        domain_id: 11,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::available(
                "test.distributed",
                location.clone(),
            ),
        ))
        .expect("distributed provider registration");

    let allocation = allocator
        .allocate(AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location.clone(),
            AllocationClass::Persistent,
        ))
        .expect("distributed allocation");

    assert_eq!(
        allocation
            .provider_allocation()
            .expect("provider allocation")
            .location(),
        location
    );
}

#[test]
fn backend_native_provider_is_supported_without_vendor_lock_in() {
    let allocator = production_allocator();

    let location = MemoryLocation::BackendNative {
        provider: "test.qpu.provider".to_owned(),
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::available(
                "test.qpu.provider",
                location.clone(),
            ),
        ))
        .expect("backend-native provider registration");

    let allocation = allocator
        .allocate(AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location.clone(),
            AllocationClass::State,
        )
        .with_qubits(TEST_QUBITS)
        .with_state_elements(TEST_STATE_ELEMENTS))
        .expect("backend-native allocation");

    assert_eq!(
        allocation.provider_id(),
        "test.qpu.provider"
    );

    assert_eq!(
        allocation
            .provider_allocation()
            .expect("provider allocation")
            .location(),
        location
    );
}

// =============================================================================
// Provider failure and rollback
// =============================================================================

#[test]
fn provider_failure_rolls_back_accounting() {
    let allocator = production_allocator();

    let location = MemoryLocation::Device {
        device_id: 1,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::failing(
                "test.failing-provider",
                location.clone(),
                "simulated provider allocation failure",
            ),
        ))
        .expect("provider registration");

    let result = allocator.allocate(
        AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location,
            AllocationClass::Temporary,
        ),
    );

    assert!(
        matches!(
            result,
            Err(MemoryError::BackendRejected { .. })
        ),
        "provider failure must be returned to the caller"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0,
        "failed provider allocation must not leak accounting"
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0,
        "failed provider allocation must not leave a live allocation"
    );
}

#[test]
fn provider_reported_size_mismatch_is_rejected_and_rolled_back() {
    let allocator = production_allocator();

    let location = MemoryLocation::Device {
        device_id: 2,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::wrong_size(
                "test.wrong-size",
                location.clone(),
                TEST_BYTES + 1,
            ),
        ))
        .expect("provider registration");

    let result = allocator.allocate(
        AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location,
            AllocationClass::Temporary,
        ),
    );

    assert!(
        matches!(
            result,
            Err(MemoryError::AllocationFailed { .. })
        ),
        "provider size mismatch must be rejected"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

#[test]
fn provider_reported_location_mismatch_is_rejected_and_rolled_back() {
    let allocator = production_allocator();

    let requested_location = MemoryLocation::Device {
        device_id: 3,
    };

    let actual_location = MemoryLocation::Device {
        device_id: 4,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::wrong_location(
                "test.wrong-location",
                requested_location.clone(),
                actual_location,
            ),
        ))
        .expect("provider registration");

    let result = allocator.allocate(
        AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            requested_location,
            AllocationClass::Temporary,
        ),
    );

    assert!(
        matches!(
            result,
            Err(MemoryError::BackendRejected { .. })
        ),
        "provider location mismatch must be rejected"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

// =============================================================================
// Provider availability
// =============================================================================

#[test]
fn provider_that_cannot_allocate_is_rejected_without_accounting_leak() {
    let allocator = production_allocator();

    let location = MemoryLocation::Device {
        device_id: 5,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::unavailable(
                "test.unavailable",
                location.clone(),
            ),
        ))
        .expect("provider registration");

    let result = allocator.allocate(
        AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location,
            AllocationClass::Temporary,
        ),
    );

    assert!(
        matches!(
            result,
            Err(MemoryError::AllocationFailed { .. })
        ),
        "provider refusing the requested size must fail"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        0
    );
}

// =============================================================================
// Provider identity validation
// =============================================================================

#[test]
fn empty_provider_identifier_is_rejected() {
    let allocator = production_allocator();

    let provider = TestProvider::available(
        "",
        MemoryLocation::Device {
            device_id: 0,
        },
    );

    let result =
        allocator.register_provider(Arc::new(provider));

    assert!(
        matches!(
            result,
            Err(MemoryError::InvalidArgument { .. })
        ),
        "provider identity is part of the public provider contract"
    );
}

#[test]
fn empty_backend_native_namespace_is_rejected() {
    let allocator = production_allocator();

    let location = MemoryLocation::BackendNative {
        provider: String::new(),
    };

    let provider =
        TestProvider::available(
            "invalid-backend-provider",
            location,
        );

    let result =
        allocator.register_provider(Arc::new(provider));

    assert!(
        matches!(
            result,
            Err(MemoryError::InvalidArgument { .. })
        ),
        "backend-native namespaces must be non-empty"
    );
}

// =============================================================================
// Provider replacement safety
// =============================================================================

#[test]
fn provider_replacement_is_rejected_while_allocations_are_live() {
    let allocator = production_allocator();

    let location = MemoryLocation::Device {
        device_id: 8,
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::available(
                "test.original-provider",
                location.clone(),
            ),
        ))
        .expect("initial provider registration");

    let allocation = allocator
        .allocate(AllocationRequest::new(
            ByteCount::new(TEST_BYTES),
            location.clone(),
            AllocationClass::Temporary,
        ))
        .expect("allocation");

    let replacement =
        TestProvider::available(
            "test.replacement-provider",
            location.clone(),
        );

    let result =
        allocator.register_provider(Arc::new(replacement));

    assert!(
        matches!(
            result,
            Err(MemoryError::ConcurrencyConflict { .. })
        ),
        "provider replacement must not occur underneath a live allocation"
    );

    drop(allocation);

    allocator
        .register_provider(Arc::new(
            TestProvider::available(
                "test.replacement-provider",
                location,
            ),
        ))
        .expect("replacement should succeed after all allocations are released");
}

// =============================================================================
// Memory-location classification
// =============================================================================

#[test]
fn memory_location_classification_is_provider_neutral() {
    assert_eq!(
        MemoryLocation::Host.kind(),
        MemoryLocationKind::Host
    );

    assert_eq!(
        MemoryLocation::PinnedHost.kind(),
        MemoryLocationKind::PinnedHost
    );

    assert_eq!(
        MemoryLocation::Device { device_id: 1 }.kind(),
        MemoryLocationKind::Device
    );

    assert_eq!(
        MemoryLocation::Unified { device_id: 2 }.kind(),
        MemoryLocationKind::Unified
    );

    assert_eq!(
        MemoryLocation::Distributed { domain_id: 3 }.kind(),
        MemoryLocationKind::Distributed
    );

    assert_eq!(
        MemoryLocation::BackendNative {
            provider: "qpu".to_owned(),
        }
        .kind(),
        MemoryLocationKind::BackendNative
    );
}

// =============================================================================
// Allocation class semantics
// =============================================================================

#[test]
fn allocation_classes_remain_explicit() {
    assert!(
        AllocationClass::Temporary.is_temporary()
    );

    assert!(
        !AllocationClass::Persistent.is_temporary()
    );

    assert!(
        AllocationClass::State.is_state()
    );

    assert!(
        !AllocationClass::Temporary.is_state()
    );
}

// =============================================================================
// Concurrent allocation
// =============================================================================

#[test]
fn concurrent_allocations_preserve_accounting() {
    let allocator = Arc::new(production_allocator());

    const THREADS: usize = 8;
    const ALLOCATIONS_PER_THREAD: usize = 16;

    let mut handles = Vec::with_capacity(THREADS);

    for _ in 0..THREADS {
        let allocator = Arc::clone(&allocator);

        handles.push(thread::spawn(move || {
            let mut allocations = Vec::with_capacity(
                ALLOCATIONS_PER_THREAD
            );

            for _ in 0..ALLOCATIONS_PER_THREAD {
                let allocation = allocator
                    .allocate_host(
                        ByteCount::new(TEST_BYTES),
                        AllocationClass::Temporary,
                    )
                    .expect("concurrent allocation");

                allocations.push(allocation);
            }

            allocations
        }));
    }

    let mut all_allocations = Vec::new();

    for handle in handles {
        let allocations =
            handle.join().expect("allocation thread must not panic");

        all_allocations.extend(allocations);
    }

    let expected_count =
        (THREADS * ALLOCATIONS_PER_THREAD) as u64;

    let expected_bytes =
        expected_count * TEST_BYTES;

    assert_eq!(
        allocator
            .allocation_count()
            .expect("accounting"),
        expected_count
    );

    assert_eq!(
        allocator
            .allocated_bytes()
            .expect("accounting"),
        expected_bytes
    );

    assert_eq!(
        allocator
            .peak_allocated_bytes()
            .expect("accounting"),
        expected_bytes
    );

    drop(all_allocations);

    assert_eq!(
        allocator
            .allocation_count()
            .expect("accounting"),
        0
    );

    assert_eq!(
        allocator
            .allocated_bytes()
            .expect("accounting"),
        0
    );
}

// =============================================================================
// Concurrent allocation identity
// =============================================================================

#[test]
fn concurrent_allocations_receive_unique_ids() {
    let allocator = Arc::new(production_allocator());

    const THREADS: usize = 8;
    const ALLOCATIONS_PER_THREAD: usize = 8;

    let mut handles = Vec::with_capacity(THREADS);

    for _ in 0..THREADS {
        let allocator = Arc::clone(&allocator);

        handles.push(thread::spawn(move || {
            let mut ids = Vec::with_capacity(
                ALLOCATIONS_PER_THREAD
            );

            let mut allocations = Vec::with_capacity(
                ALLOCATIONS_PER_THREAD
            );

            for _ in 0..ALLOCATIONS_PER_THREAD {
                let allocation = allocator
                    .allocate_host(
                        ByteCount::new(TEST_BYTES),
                        AllocationClass::Temporary,
                    )
                    .expect("allocation");

                ids.push(allocation.id());
                allocations.push(allocation);
            }

            (ids, allocations)
        }));
    }

    let mut ids = Vec::new();
    let mut allocations = Vec::new();

    for handle in handles {
        let (thread_ids, thread_allocations) =
            handle.join().expect("thread must not panic");

        ids.extend(thread_ids);
        allocations.extend(thread_allocations);
    }

    for (index, first) in ids.iter().enumerate() {
        for second in ids.iter().skip(index + 1) {
            assert_ne!(
                first,
                second,
                "allocation identities must be globally unique"
            );
        }
    }

    drop(allocations);

    assert_eq!(
        allocator
            .allocation_count()
            .expect("accounting"),
        0
    );
}

// =============================================================================
// Host provider identity
// =============================================================================

#[test]
fn built_in_host_provider_has_stable_identity() {
    let provider = HostMemoryProvider::new();

    assert_eq!(
        provider.provider_id(),
        "zamani.host"
    );

    assert_eq!(
        provider.location(),
        MemoryLocation::Host
    );

    assert!(
        provider.can_allocate(TEST_BYTES),
        "normal small host allocation must be supported"
    );
}

// =============================================================================
// Remaining capacity
// =============================================================================

#[test]
fn remaining_host_capacity_decreases_and_is_restored() {
    let allocator = production_allocator();

    let before = allocator
        .remaining_host_bytes()
        .expect("remaining host capacity");

    let allocation = allocator
        .allocate_host(
            ByteCount::new(TEST_BYTES),
            AllocationClass::Temporary,
        )
        .expect("allocation");

    let during = allocator
        .remaining_host_bytes()
        .expect("remaining host capacity");

    assert_eq!(
        before - during,
        TEST_BYTES
    );

    drop(allocation);

    let after = allocator
        .remaining_host_bytes()
        .expect("remaining host capacity");

    assert_eq!(
        before,
        after
    );
}

// =============================================================================
// State accounting rollback
// =============================================================================

#[test]
fn failed_state_provider_allocation_does_not_leak_state_accounting() {
    let allocator = production_allocator();

    let location = MemoryLocation::BackendNative {
        provider: "test.failed-state".to_owned(),
    };

    allocator
        .register_provider(Arc::new(
            TestProvider::failing(
                "test.failed-state",
                location.clone(),
                "state allocation failure",
            ),
        ))
        .expect("provider registration");

    let result = allocator.allocate_state(
        ByteCount::new(TEST_BYTES),
        location,
        TEST_QUBITS,
        TEST_STATE_ELEMENTS,
    );

    assert!(
        result.is_err(),
        "provider failure must propagate"
    );

    let accounting =
        allocator.accounting().expect("accounting");

    assert_eq!(
        accounting.state_bytes,
        0
    );

    assert_eq!(
        accounting.state_elements,
        0
    );

    assert_eq!(
        accounting.allocations,
        0
    );
}

// =============================================================================
// Provider-neutrality smoke test
// =============================================================================

#[test]
fn one_allocator_contract_can_represent_multiple_hardware_domains() {
    let allocator = production_allocator();

    let locations = [
        MemoryLocation::Device {
            device_id: 0,
        },
        MemoryLocation::Unified {
            device_id: 1,
        },
        MemoryLocation::Distributed {
            domain_id: 2,
        },
        MemoryLocation::BackendNative {
            provider: "generic-qpu".to_owned(),
        },
    ];

    for (index, location) in locations.iter().enumerate() {
        let provider_id = match location {
            MemoryLocation::Device { .. } => "test.device",
            MemoryLocation::Unified { .. } => "test.unified",
            MemoryLocation::Distributed { .. } => "test.distributed",
            MemoryLocation::BackendNative { .. } => "test.qpu",
            _ => unreachable!(
                "test set only contains provider-backed locations"
            ),
        };

        allocator
            .register_provider(Arc::new(
                TestProvider::available(
                    provider_id,
                    location.clone(),
                ),
            ))
            .expect("provider registration");

        let allocation = allocator
            .allocate(AllocationRequest::new(
                ByteCount::new(
                    TEST_BYTES + index as u64
                ),
                location.clone(),
                AllocationClass::Temporary,
            ))
            .expect("provider-neutral allocation");

        assert_eq!(
            allocation
                .provider_allocation()
                .expect("provider allocation")
                .location(),
            *location
        );

        assert_eq!(
            allocation.provider_id(),
            provider_id
        );

        drop(allocation);
    }

    assert_eq!(
        allocator
            .allocated_bytes()
            .expect("accounting"),
        0
    );
}

// =============================================================================
// Allocation metadata integrity
// =============================================================================

#[test]
fn allocation_metadata_remains_consistent_with_request() {
    let allocator = production_allocator();

    let request = AllocationRequest::new(
        ByteCount::new(TEST_BYTES),
        MemoryLocation::Host,
        AllocationClass::Persistent,
    )
    .with_qubits(TEST_QUBITS)
    .with_state_elements(TEST_STATE_ELEMENTS)
    .with_label("allocation-integration-test");

    let allocation = allocator
        .allocate(request.clone())
        .expect("allocation");

    assert_eq!(
        allocation.request(),
        &request
    );

    assert_eq!(
        allocation.byte_len(),
        request.byte_count()
    );

    assert_eq!(
        allocation.memory_id(),
        MemoryId::new(TEST_MEMORY_ID)
    );

    assert!(allocation.is_live());
}

// =============================================================================
// Production policy sanity
// =============================================================================

#[test]
fn production_policy_is_finite() {
    let limits = MemoryLimits::production();

    assert!(limits.max_host_bytes() > 0);
    assert!(limits.max_device_bytes() > 0);
    assert!(limits.max_distributed_bytes() > 0);
    assert!(limits.max_allocations() > 0);
    assert!(limits.max_qubits() > 0);
    assert!(limits.max_state_bytes() > 0);
    assert!(limits.max_state_elements() > 0);
}

// =============================================================================
// Compile-time safety boundary documentation test
// =============================================================================

/// This test intentionally has no runtime assertions.
///
/// Its purpose is to keep the integration contract explicit: allocation is
/// represented by safe Rust traits and opaque provider allocations rather than
/// raw pointers.
///
/// If a future implementation attempts to expose raw addresses through the
/// public allocator API, this test file should be reviewed as part of that
/// architectural change.
#[test]
fn public_allocator_boundary_is_safe_rust() {
    fn assert_provider_is_safe<T: MemoryProvider>() {}

    fn assert_allocation_is_safe<T: ProviderAllocation>() {}

    assert_provider_is_safe::<HostMemoryProvider>();
    assert_allocation_is_safe::<TestProviderAllocation>();
}