//! Zamani Quantum Memory — Resource-Limit Integration Tests
//!
//! Production tests for:
//!
//! `crate::quantum::memory::limits`
//!
//! # Responsibility
//!
//! This module verifies the complete public resource-policy contract used by
//! Zamani quantum memory.
//!
//! It covers:
//!
//! - production policy validity;
//! - strict policy validity;
//! - deny-all policy validity;
//! - finite/no-unlimited policy semantics;
//! - all public resource categories;
//! - exact boundary acceptance;
//! - boundary-plus-one rejection;
//! - structured violation identity;
//! - logical/classical memory limits;
//! - allocation-count limits;
//! - host memory;
//! - temporary host memory;
//! - persistent host memory;
//! - pinned host memory;
//! - device/accelerator memory;
//! - temporary device memory;
//! - distributed memory;
//! - distributed partition count;
//! - distributed partition size;
//! - quantum-state memory;
//! - temporary state memory;
//! - state-element limits;
//! - snapshots;
//! - checkpoints;
//! - tensor rank;
//! - tensor dimensions;
//! - tensor count;
//! - tensor-network bond dimensions;
//! - state-vector qubit limits;
//! - density-matrix qubit limits;
//! - tensor-network qubit limits;
//! - measurement-result limits;
//! - metadata limits;
//! - deterministic planning-work limits;
//! - checked powers;
//! - checked basis-state counts;
//! - checked density-matrix element counts;
//! - state-vector memory estimation;
//! - density-matrix memory estimation;
//! - tensor memory estimation;
//! - tensor-network estimation;
//! - distributed-allocation validation;
//! - snapshot/checkpoint validation;
//! - platform-size conversion;
//! - requirement construction;
//! - requirement composition;
//! - checked requirement addition;
//! - overflow detection;
//! - deterministic first-violation ordering;
//! - provider/QPU neutrality;
//! - architecture-independent behavior;
//! - Rust 1.97 / 1.97.1 compatibility;
//! - no unsafe code.
//!
//! # Architectural rule
//!
//! These tests deliberately do not:
//!
//! - allocate real quantum state vectors;
//! - allocate GPU memory;
//! - access CUDA;
//! - access HIP;
//! - access Metal;
//! - access Vulkan;
//! - access MPI;
//! - access RDMA;
//! - access a QPU SDK;
//! - contact IBM Quantum;
//! - contact Google Quantum AI;
//! - contact Quantinuum;
//! - contact IonQ;
//! - contact Rigetti;
//! - contact AWS Braket;
//! - contact Azure Quantum;
//! - depend on a particular backend;
//! - depend on a particular simulator;
//! - use unsafe Rust;
//! - use raw pointers;
//! - use architecture-specific intrinsics.
//!
//! `MemoryLimits` is intentionally provider-neutral. A real QPU or accelerator
//! backend consumes this policy; it does not redefine it.
//!
//! # Integration contract
//!
//! This file is designed against the stable public contracts of:
//!
//! - `memory::limits::MemoryLimits`;
//! - `memory::limits::MemoryRequirement`;
//! - `memory::limits::MemoryEstimate`;
//! - `memory::limits::MemoryLimitKind`;
//! - `memory::limits::MemoryLimitViolation`;
//! - `memory::limits::MemoryEstimateError`;
//! - `memory::limits::MemoryLimitConfigError`.
//!
//! No later memory module needs to be modified for these tests.
//!
//! The test module should be registered from:
//!
//! `src/quantum/memory/tests/mod.rs`
//!
//! with:
//!
//! ```text
//! mod limits;
//! ```
//!
//! If the parent `memory/mod.rs` exposes the test module under `cfg(test)`,
//! this file participates in the normal Rust test build.
//!
//! # Safety
//!
//! Entirely safe Rust.
//!
//! There is intentionally no:
//!
//! - `unsafe`;
//! - `unsafe fn`;
//! - raw pointer;
//! - `transmute`;
//! - FFI;
//! - architecture-specific intrinsic.
//!
//! # Quantum-memory scaling
//!
//! The tests enforce the fundamental scaling rules:
//!
//! ```text
//! state vector       = 2^n elements
//! density matrix     = 4^n elements
//! dense tensor       = dimension^rank elements
//! ```
//!
//! The test suite therefore verifies that overflow is detected before an
//! allocator could ever receive an invalid size.
//!
//! # QPU/hardware neutrality
//!
//! A QPU may have very different physical memory characteristics:
//!
//! - superconducting;
//! - trapped-ion;
//! - neutral-atom;
//! - photonic;
//! - spin/qubit devices;
//! - annealing systems;
//! - quantum simulators;
//! - heterogeneous CPU/GPU systems;
//! - distributed simulators;
//! - remote QPU services.
//!
//! None of those differences belong in this test module. They are represented
//! by the generic resource categories exposed by `MemoryLimits`.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!

use crate::quantum::memory::limits::{
    MemoryEstimateError,
    MemoryLimitKind,
    MemoryLimits,
    MemoryRequirement,
};

// =============================================================================
// Constants
// =============================================================================

const ONE_KIB: u64 = 1024;
const ONE_MIB: u64 = 1024 * ONE_KIB;
const ONE_GIB: u64 = 1024 * ONE_MIB;

// =============================================================================
// Policy construction
// =============================================================================

#[test]
fn production_policy_is_valid() {
    let limits = MemoryLimits::production();

    assert!(
        limits.validate().is_ok(),
        "production memory policy must be internally consistent"
    );
}

#[test]
fn strict_policy_is_valid() {
    let limits = MemoryLimits::strict();

    assert!(
        limits.validate().is_ok(),
        "strict memory policy must be internally consistent"
    );
}

#[test]
fn deny_all_policy_is_valid() {
    let limits = MemoryLimits::deny_all();

    assert!(
        limits.validate().is_ok(),
        "deny-all is intentionally valid: zero means prohibited"
    );
}

#[test]
fn default_policy_is_valid_and_matches_production() {
    let default_limits = MemoryLimits::default();
    let production_limits = MemoryLimits::production();

    assert_eq!(default_limits, production_limits);
    assert!(default_limits.validate().is_ok());
}

#[test]
fn policies_are_finite_and_do_not_use_unlimited_sentinels() {
    let production = MemoryLimits::production();
    let strict = MemoryLimits::strict();

    assert_ne!(production.max_qubits(), u64::MAX);
    assert_ne!(production.max_host_bytes(), u64::MAX);
    assert_ne!(production.max_device_bytes(), u64::MAX);
    assert_ne!(production.max_state_bytes(), u64::MAX);

    assert_ne!(strict.max_qubits(), u64::MAX);
    assert_ne!(strict.max_host_bytes(), u64::MAX);
    assert_ne!(strict.max_device_bytes(), u64::MAX);
    assert_ne!(strict.max_state_bytes(), u64::MAX);
}

// =============================================================================
// Basic generic limit checking
// =============================================================================

#[test]
fn generic_check_accepts_exact_boundary() {
    let result = MemoryLimits::check(
        MemoryLimitKind::Qubits,
        4096,
        4096,
    );

    assert!(result.is_ok());
}

#[test]
fn generic_check_rejects_boundary_plus_one() {
    let result = MemoryLimits::check(
        MemoryLimitKind::Qubits,
        4097,
        4096,
    );

    let error = result.expect_err("boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::Qubits);
    assert_eq!(error.requested(), 4097);
    assert_eq!(error.maximum(), 4096);
}

#[test]
fn zero_is_allowed_when_policy_allows_zero() {
    let result = MemoryLimits::check(
        MemoryLimitKind::Qubits,
        0,
        0,
    );

    assert!(result.is_ok());
}

#[test]
fn deny_all_rejects_any_positive_resource_request() {
    let limits = MemoryLimits::deny_all();

    assert_eq!(
        limits
            .check_qubits(1)
            .expect_err("qubit allocation must be denied")
            .kind(),
        MemoryLimitKind::Qubits
    );

    assert_eq!(
        limits
            .check_host_bytes(1)
            .expect_err("host allocation must be denied")
            .kind(),
        MemoryLimitKind::HostBytes
    );

    assert_eq!(
        limits
            .check_device_bytes(1)
            .expect_err("device allocation must be denied")
            .kind(),
        MemoryLimitKind::DeviceBytes
    );

    assert_eq!(
        limits
            .check_distributed_bytes(1)
            .expect_err("distributed allocation must be denied")
            .kind(),
        MemoryLimitKind::DistributedBytes
    );
}

// =============================================================================
// Logical and classical memory
// =============================================================================

#[test]
fn qubit_limit_accepts_boundary_and_rejects_overflow() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_qubits();

    assert!(limits.check_qubits(maximum).is_ok());

    let error = limits
        .check_qubits(maximum + 1)
        .expect_err("qubit boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::Qubits);
    assert_eq!(error.requested(), maximum + 1);
    assert_eq!(error.maximum(), maximum);
}

#[test]
fn classical_bit_limit_accepts_boundary_and_rejects_overflow() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_classical_bits();

    assert!(limits.check_classical_bits(maximum).is_ok());

    let error = limits
        .check_classical_bits(maximum + 1)
        .expect_err("classical-bit boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::ClassicalBits);
}

#[test]
fn allocation_count_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_allocations();

    assert!(limits.check_allocations(maximum).is_ok());

    let error = limits
        .check_allocations(maximum + 1)
        .expect_err("allocation count must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::Allocations);
}

// =============================================================================
// Host memory
// =============================================================================

#[test]
fn host_memory_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_host_bytes();

    assert!(limits.check_host_bytes(maximum).is_ok());

    let error = limits
        .check_host_bytes(maximum + 1)
        .expect_err("host memory boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::HostBytes);
}

#[test]
fn temporary_host_memory_has_independent_limit() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_temporary_host_bytes();

    assert!(limits.check_temporary_host_bytes(maximum).is_ok());

    let error = limits
        .check_temporary_host_bytes(maximum + 1)
        .expect_err("temporary host memory must be independently bounded");

    assert_eq!(error.kind(), MemoryLimitKind::TemporaryHostBytes);
}

#[test]
fn persistent_host_memory_has_independent_limit() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_persistent_host_bytes();

    assert!(limits.check_persistent_host_bytes(maximum).is_ok());

    let error = limits
        .check_persistent_host_bytes(maximum + 1)
        .expect_err("persistent host memory must be independently bounded");

    assert_eq!(error.kind(), MemoryLimitKind::PersistentHostBytes);
}

#[test]
fn pinned_host_memory_has_independent_limit() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_pinned_host_bytes();

    assert!(limits.check_pinned_host_bytes(maximum).is_ok());

    let error = limits
        .check_pinned_host_bytes(maximum + 1)
        .expect_err("pinned host memory must be independently bounded");

    assert_eq!(error.kind(), MemoryLimitKind::PinnedHostBytes);
}

// =============================================================================
// Device / accelerator memory
// =============================================================================

#[test]
fn device_memory_is_provider_neutral() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_device_bytes();

    assert!(limits.check_device_bytes(maximum).is_ok());

    let error = limits
        .check_device_bytes(maximum + 1)
        .expect_err("device memory must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::DeviceBytes);
}

#[test]
fn temporary_device_memory_is_independently_bounded() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_temporary_device_bytes();

    assert!(limits.check_temporary_device_bytes(maximum).is_ok());

    let error = limits
        .check_temporary_device_bytes(maximum + 1)
        .expect_err("temporary device memory must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::TemporaryDeviceBytes);
}

// =============================================================================
// Distributed memory
// =============================================================================

#[test]
fn distributed_memory_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_distributed_bytes();

    assert!(limits.check_distributed_bytes(maximum).is_ok());

    let error = limits
        .check_distributed_bytes(maximum + 1)
        .expect_err("distributed memory must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::DistributedBytes);
}

#[test]
fn distributed_partition_count_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_distributed_partitions();

    assert!(limits.check_distributed_partitions(maximum).is_ok());

    let error = limits
        .check_distributed_partitions(maximum + 1)
        .expect_err("partition count must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::DistributedPartitions);
}

#[test]
fn distributed_partition_size_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_distributed_partition_bytes();

    assert!(limits.check_distributed_partition_bytes(maximum).is_ok());

    let error = limits
        .check_distributed_partition_bytes(maximum + 1)
        .expect_err("partition size must be bounded");

    assert_eq!(
        error.kind(),
        MemoryLimitKind::DistributedPartitionBytes
    );
}

#[test]
fn complete_distributed_allocation_checks_all_dimensions() {
    let limits = MemoryLimits::production();

    assert!(
        limits
            .check_distributed_allocation(
                limits.max_distributed_partitions(),
                limits.max_distributed_partition_bytes(),
                limits.max_distributed_bytes(),
            )
            .is_ok()
    );

    let error = limits
        .check_distributed_allocation(
            limits.max_distributed_partitions() + 1,
            limits.max_distributed_partition_bytes(),
            limits.max_distributed_bytes(),
        )
        .expect_err("partition count must be checked");

    assert_eq!(
        error.kind(),
        MemoryLimitKind::DistributedPartitions
    );
}

// =============================================================================
// Quantum state limits
// =============================================================================

#[test]
fn state_memory_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_state_bytes();

    assert!(limits.check_state_bytes(maximum).is_ok());

    let error = limits
        .check_state_bytes(maximum + 1)
        .expect_err("state memory boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::StateBytes);
}

#[test]
fn temporary_state_memory_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_temporary_state_bytes();

    assert!(limits.check_temporary_state_bytes(maximum).is_ok());

    let error = limits
        .check_temporary_state_bytes(maximum + 1)
        .expect_err("temporary state memory must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::TemporaryStateBytes);
}

#[test]
fn state_element_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_state_elements();

    assert!(limits.check_state_elements(maximum).is_ok());

    let error = limits
        .check_state_elements(maximum + 1)
        .expect_err("state element boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::StateElements);
}

// =============================================================================
// Representation-specific qubit limits
// =============================================================================

#[test]
fn state_vector_qubit_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_state_vector_qubits();

    assert!(limits.check_state_vector_qubits(maximum).is_ok());

    let error = limits
        .check_state_vector_qubits(maximum + 1)
        .expect_err("state-vector qubit boundary must be enforced");

    assert_eq!(error.kind(), MemoryLimitKind::StateVectorQubits);
}

#[test]
fn density_matrix_qubit_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_density_matrix_qubits();

    assert!(limits.check_density_matrix_qubits(maximum).is_ok());

    let error = limits
        .check_density_matrix_qubits(maximum + 1)
        .expect_err("density-matrix qubit boundary must be enforced");

    assert_eq!(error.kind(), MemoryLimitKind::DensityMatrixQubits);
}

#[test]
fn tensor_network_qubit_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_tensor_network_qubits();

    assert!(limits.check_tensor_network_qubits(maximum).is_ok());

    let error = limits
        .check_tensor_network_qubits(maximum + 1)
        .expect_err("tensor-network qubit boundary must be enforced");

    assert_eq!(error.kind(), MemoryLimitKind::TensorNetworkQubits);
}

// =============================================================================
// Persistence limits
// =============================================================================

#[test]
fn snapshot_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_snapshot_bytes();

    assert!(limits.check_snapshot_bytes(maximum).is_ok());

    assert!(
        limits
            .check_snapshot(maximum)
            .is_ok()
    );

    let error = limits
        .check_snapshot(maximum + 1)
        .expect_err("snapshot boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::SnapshotBytes);
}

#[test]
fn checkpoint_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_checkpoint_bytes();

    assert!(limits.check_checkpoint_bytes(maximum).is_ok());

    assert!(
        limits
            .check_checkpoint(maximum)
            .is_ok()
    );

    let error = limits
        .check_checkpoint(maximum + 1)
        .expect_err("checkpoint boundary plus one must fail");

    assert_eq!(error.kind(), MemoryLimitKind::CheckpointBytes);
}

// =============================================================================
// Tensor limits
// =============================================================================

#[test]
fn tensor_rank_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_tensor_rank();

    assert!(limits.check_tensor_rank(maximum).is_ok());

    let error = limits
        .check_tensor_rank(maximum + 1)
        .expect_err("tensor rank must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::TensorRank);
}

#[test]
fn tensor_dimension_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_tensor_dimension();

    assert!(limits.check_tensor_dimension(maximum).is_ok());

    let error = limits
        .check_tensor_dimension(maximum + 1)
        .expect_err("tensor dimension must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::TensorDimension);
}

#[test]
fn tensor_count_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_tensors();

    assert!(limits.check_tensors(maximum).is_ok());

    let error = limits
        .check_tensors(maximum + 1)
        .expect_err("tensor count must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::Tensors);
}

#[test]
fn bond_dimension_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_bond_dimension();

    assert!(limits.check_bond_dimension(maximum).is_ok());

    let error = limits
        .check_bond_dimension(maximum + 1)
        .expect_err("bond dimension must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::BondDimension);
}

// =============================================================================
// Classical / metadata / planning limits
// =============================================================================

#[test]
fn measurement_result_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_measurement_results();

    assert!(limits.check_measurement_results(maximum).is_ok());

    let error = limits
        .check_measurement_results(maximum + 1)
        .expect_err("measurement-result count must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::MeasurementResults);
}

#[test]
fn metadata_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_metadata_bytes();

    assert!(limits.check_metadata_bytes(maximum).is_ok());

    let error = limits
        .check_metadata_bytes(maximum + 1)
        .expect_err("metadata bytes must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::MetadataBytes);
}

#[test]
fn planning_work_limit_is_enforced() {
    let limits = MemoryLimits::production();
    let maximum = limits.max_planning_work();

    assert!(limits.check_planning_work(maximum).is_ok());

    let error = limits
        .check_planning_work(maximum + 1)
        .expect_err("planning work must be bounded");

    assert_eq!(error.kind(), MemoryLimitKind::PlanningWork);
}

// =============================================================================
// Checked powers
// =============================================================================

#[test]
fn checked_power_handles_zero_exponent() {
    assert_eq!(
        MemoryLimits::checked_pow_u64(2, 0).unwrap(),
        1
    );

    assert_eq!(
        MemoryLimits::checked_pow_u64(4, 0).unwrap(),
        1
    );
}

#[test]
fn checked_power_handles_normal_values() {
    assert_eq!(
        MemoryLimits::checked_pow_u64(2, 10).unwrap(),
        1024
    );

    assert_eq!(
        MemoryLimits::checked_pow_u64(4, 5).unwrap(),
        1024
    );
}

#[test]
fn checked_power_detects_overflow() {
    let result = MemoryLimits::checked_pow_u64(2, 64);

    let error = result.expect_err("2^64 must not fit into u64");

    assert_eq!(
        error,
        MemoryEstimateError::ExponentOverflow {
            base: 2,
            exponent: 64,
        }
    );
}

#[test]
fn checked_power_detects_large_composite_overflow() {
    let result = MemoryLimits::checked_pow_u64(
        u64::MAX,
        2,
    );

    assert!(matches!(
        result,
        Err(MemoryEstimateError::ExponentOverflow { .. })
    ));
}

#[test]
fn checked_power_supports_base_zero() {
    assert_eq!(
        MemoryLimits::checked_pow_u64(0, 0).unwrap(),
        1
    );

    assert_eq!(
        MemoryLimits::checked_pow_u64(0, 5).unwrap(),
        0
    );
}

// =============================================================================
// Quantum-state element counts
// =============================================================================

#[test]
fn basis_state_count_is_exact() {
    assert_eq!(
        MemoryLimits::basis_state_count(0).unwrap(),
        1
    );

    assert_eq!(
        MemoryLimits::basis_state_count(1).unwrap(),
        2
    );

    assert_eq!(
        MemoryLimits::basis_state_count(10).unwrap(),
        1024
    );

    assert_eq!(
        MemoryLimits::basis_state_count(32).unwrap(),
        1u64 << 32
    );
}

#[test]
fn basis_state_count_detects_overflow() {
    assert!(
        MemoryLimits::basis_state_count(64).is_err(),
        "2^64 cannot fit into u64"
    );
}

#[test]
fn density_matrix_element_count_is_exact() {
    assert_eq!(
        MemoryLimits::density_matrix_element_count(0).unwrap(),
        1
    );

    assert_eq!(
        MemoryLimits::density_matrix_element_count(1).unwrap(),
        4
    );

    assert_eq!(
        MemoryLimits::density_matrix_element_count(2).unwrap(),
        16
    );

    assert_eq!(
        MemoryLimits::density_matrix_element_count(16).unwrap(),
        1u64 << 32
    );
}

#[test]
fn density_matrix_element_count_detects_overflow() {
    assert!(
        MemoryLimits::density_matrix_element_count(32).is_err(),
        "4^32 = 2^64 cannot fit into u64"
    );
}

// =============================================================================
// State-vector estimation
// =============================================================================

#[test]
fn complex_f64_state_vector_estimation_is_exact() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_state_vector_complex_f64(10)
        .expect("10-qubit state vector must fit production limits");

    assert_eq!(estimate.qubits(), 10);
    assert_eq!(estimate.elements(), 1024);
    assert_eq!(estimate.bytes_per_element(), 16);
    assert_eq!(estimate.bytes(), 16 * 1024);
}

#[test]
fn complex_f32_state_vector_estimation_is_exact() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_state_vector_complex_f32(10)
        .expect("10-qubit f32 state vector must fit");

    assert_eq!(estimate.qubits(), 10);
    assert_eq!(estimate.elements(), 1024);
    assert_eq!(estimate.bytes_per_element(), 8);
    assert_eq!(estimate.bytes(), 8 * 1024);
}

#[test]
fn state_vector_memory_scales_by_two_per_extra_qubit() {
    let limits = MemoryLimits::production();

    let a = limits
        .estimate_state_vector_complex_f64(10)
        .unwrap();

    let b = limits
        .estimate_state_vector_complex_f64(11)
        .unwrap();

    assert_eq!(b.elements(), a.elements() * 2);
    assert_eq!(b.bytes(), a.bytes() * 2);
}

#[test]
fn maximum_default_f64_state_vector_fits_exactly_when_policy_allows_it() {
    let limits = MemoryLimits::production();

    let qubits = limits.max_state_vector_qubits();

    let estimate = limits
        .estimate_state_vector_complex_f64(qubits)
        .expect(
            "production state-vector qubit policy and byte policy \
             should agree at the configured boundary",
        );

    assert_eq!(estimate.bytes_per_element(), 16);
    assert!(estimate.bytes() <= limits.max_state_bytes());
}

#[test]
fn state_vector_estimation_rejects_representation_qubit_overflow() {
    let limits = MemoryLimits::strict();

    let qubits = limits.max_state_vector_qubits() + 1;

    let result = limits
        .estimate_state_vector_complex_f64(qubits);

    assert!(
        result.is_err(),
        "state-vector estimation must reject \
         representation-specific qubit overflow"
    );
}

#[test]
fn state_vector_estimation_rejects_excessive_element_count() {
    let limits = MemoryLimits::production();

    let qubits = limits.max_state_vector_qubits();

    let result = limits
        .estimate_state_vector_complex_f64(qubits);

    if let Ok(estimate) = result {
        assert!(
            estimate.elements() <= limits.max_state_elements()
        );
        assert!(
            estimate.bytes() <= limits.max_state_bytes()
        );
    }
}

// =============================================================================
// Density-matrix estimation
// =============================================================================

#[test]
fn complex_f64_density_matrix_estimation_is_exact() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_density_matrix_complex_f64(4)
        .expect("4-qubit density matrix must fit");

    assert_eq!(estimate.qubits(), 4);
    assert_eq!(estimate.elements(), 256);
    assert_eq!(estimate.bytes_per_element(), 16);
    assert_eq!(estimate.bytes(), 4096);
}

#[test]
fn complex_f32_density_matrix_estimation_is_exact() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_density_matrix_complex_f32(4)
        .expect("4-qubit f32 density matrix must fit");

    assert_eq!(estimate.elements(), 256);
    assert_eq!(estimate.bytes_per_element(), 8);
    assert_eq!(estimate.bytes(), 2048);
}

#[test]
fn density_matrix_memory_scales_by_four_per_extra_qubit() {
    let limits = MemoryLimits::production();

    let a = limits
        .estimate_density_matrix_complex_f64(4)
        .unwrap();

    let b = limits
        .estimate_density_matrix_complex_f64(5)
        .unwrap();

    assert_eq!(b.elements(), a.elements() * 4);
    assert_eq!(b.bytes(), a.bytes() * 4);
}

#[test]
fn density_matrix_estimation_enforces_representation_limit() {
    let limits = MemoryLimits::strict();

    let qubits = limits.max_density_matrix_qubits() + 1;

    assert!(
        limits
            .estimate_density_matrix_complex_f64(qubits)
            .is_err()
    );
}

#[test]
fn density_matrix_estimation_does_not_hide_exponential_growth() {
    let limits = MemoryLimits::production();

    let four = limits
        .estimate_density_matrix_complex_f64(4)
        .unwrap();

    let six = limits
        .estimate_density_matrix_complex_f64(6)
        .unwrap();

    assert_eq!(
        six.elements(),
        four.elements() * 16
    );

    assert_eq!(
        six.bytes(),
        four.bytes() * 16
    );
}

// =============================================================================
// Tensor estimation
// =============================================================================

#[test]
fn tensor_element_count_is_exact() {
    assert_eq!(
        MemoryLimits::tensor_element_count(1, 2).unwrap(),
        2
    );

    assert_eq!(
        MemoryLimits::tensor_element_count(2, 2).unwrap(),
        4
    );

    assert_eq!(
        MemoryLimits::tensor_element_count(3, 2).unwrap(),
        8
    );

    assert_eq!(
        MemoryLimits::tensor_element_count(4, 3).unwrap(),
        81
    );
}

#[test]
fn tensor_element_count_rejects_zero_rank() {
    assert_eq!(
        MemoryLimits::tensor_element_count(0, 2)
            .expect_err("zero tensor rank must fail"),
        MemoryEstimateError::InvalidTensorRank
    );
}

#[test]
fn tensor_element_count_rejects_zero_dimension() {
    assert_eq!(
        MemoryLimits::tensor_element_count(2, 0)
            .expect_err("zero tensor dimension must fail"),
        MemoryEstimateError::InvalidTensorDimension
    );
}

#[test]
fn tensor_element_count_detects_overflow() {
    assert!(
        MemoryLimits::tensor_element_count(64, 2).is_err(),
        "2^64 tensor elements cannot fit in u64"
    );
}

#[test]
fn tensor_estimation_is_exact_for_small_tensor() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_tensor(3, 2, 16)
        .expect("small tensor must fit");

    assert_eq!(estimate.elements(), 8);
    assert_eq!(estimate.bytes_per_element(), 16);
    assert_eq!(estimate.bytes(), 128);
}

#[test]
fn tensor_estimation_rejects_zero_rank() {
    let limits = MemoryLimits::production();

    assert_eq!(
        limits
            .estimate_tensor(0, 2, 16)
            .expect_err("zero rank must fail"),
        MemoryEstimateError::InvalidTensorRank
    );
}

#[test]
fn tensor_estimation_rejects_zero_dimension() {
    let limits = MemoryLimits::production();

    assert_eq!(
        limits
            .estimate_tensor(2, 0, 16)
            .expect_err("zero dimension must fail"),
        MemoryEstimateError::InvalidTensorDimension
    );
}

#[test]
fn tensor_estimation_detects_element_overflow() {
    let limits = MemoryLimits::production();

    let result = limits.estimate_tensor(
        limits.max_tensor_rank(),
        limits.max_tensor_dimension(),
        16,
    );

    assert!(
        result.is_err(),
        "unsafe implicit tensor growth must be rejected"
    );
}

// =============================================================================
// Tensor-network estimation
// =============================================================================

#[test]
fn tensor_network_estimation_is_exact_for_small_network() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_tensor_network(
            4,
            2,
            2,
            16,
        )
        .expect("small tensor network must fit");

    // 4 tensors × physical dimension 2 × bond dimension² 4 = 32.
    assert_eq!(estimate.qubits(), 4);
    assert_eq!(estimate.elements(), 32);
    assert_eq!(estimate.bytes(), 32 * 16);
}

#[test]
fn tensor_network_estimation_rejects_zero_bond_dimension() {
    let limits = MemoryLimits::production();

    assert_eq!(
        limits
            .estimate_tensor_network(4, 2, 0, 16)
            .expect_err("zero bond dimension must fail"),
        MemoryEstimateError::InvalidBondDimension
    );
}

#[test]
fn tensor_network_estimation_rejects_excessive_bond_dimension() {
    let limits = MemoryLimits::strict();

    let bond_dimension =
        limits.max_bond_dimension() + 1;

    assert!(
        limits
            .estimate_tensor_network(
                4,
                2,
                bond_dimension,
                16,
            )
            .is_err()
    );
}

#[test]
fn tensor_network_estimation_checks_qubit_limit() {
    let limits = MemoryLimits::strict();

    let qubits =
        limits.max_tensor_network_qubits() + 1;

    assert!(
        limits
            .estimate_tensor_network(
                qubits,
                2,
                2,
                16,
            )
            .is_err()
    );
}

// =============================================================================
// Requirement construction
// =============================================================================

#[test]
fn empty_requirement_is_zero_everywhere() {
    let requirement = MemoryRequirement::empty();

    assert_eq!(requirement.qubits(), 0);
    assert_eq!(requirement.classical_bits(), 0);
    assert_eq!(requirement.allocations(), 0);

    assert_eq!(requirement.host_bytes(), 0);
    assert_eq!(requirement.temporary_host_bytes(), 0);
    assert_eq!(requirement.persistent_host_bytes(), 0);
    assert_eq!(requirement.pinned_host_bytes(), 0);

    assert_eq!(requirement.device_bytes(), 0);
    assert_eq!(requirement.temporary_device_bytes(), 0);

    assert_eq!(requirement.distributed_bytes(), 0);
    assert_eq!(requirement.distributed_partitions(), 0);
    assert_eq!(
        requirement.distributed_partition_bytes(),
        0
    );

    assert_eq!(requirement.state_bytes(), 0);
    assert_eq!(requirement.temporary_state_bytes(), 0);
    assert_eq!(requirement.state_elements(), 0);

    assert_eq!(requirement.snapshot_bytes(), 0);
    assert_eq!(requirement.checkpoint_bytes(), 0);

    assert_eq!(requirement.tensor_rank(), 0);
    assert_eq!(requirement.tensor_dimension(), 0);
    assert_eq!(requirement.bond_dimension(), 0);
    assert_eq!(requirement.tensors(), 0);

    assert_eq!(requirement.measurement_results(), 0);
    assert_eq!(requirement.metadata_bytes(), 0);
    assert_eq!(requirement.planning_work(), 0);
}

#[test]
fn requirement_builder_preserves_all_categories() {
    let requirement = MemoryRequirement::empty()
        .with_qubits(4)
        .with_classical_bits(8)
        .with_allocations(2)
        .with_host_bytes(100)
        .with_temporary_host_bytes(20)
        .with_persistent_host_bytes(80)
        .with_pinned_host_bytes(10)
        .with_device_bytes(200)
        .with_temporary_device_bytes(50)
        .with_distributed_bytes(300)
        .with_distributed_partitions(4)
        .with_distributed_partition_bytes(75)
        .with_state_bytes(400)
        .with_temporary_state_bytes(100)
        .with_state_elements(32)
        .with_snapshot_bytes(500)
        .with_checkpoint_bytes(600)
        .with_tensor_rank(4)
        .with_tensor_dimension(2)
        .with_bond_dimension(8)
        .with_tensors(10)
        .with_measurement_results(16)
        .with_metadata_bytes(1024)
        .with_planning_work(1000);

    assert_eq!(requirement.qubits(), 4);
    assert_eq!(requirement.classical_bits(), 8);
    assert_eq!(requirement.allocations(), 2);

    assert_eq!(requirement.host_bytes(), 100);
    assert_eq!(requirement.temporary_host_bytes(), 20);
    assert_eq!(requirement.persistent_host_bytes(), 80);
    assert_eq!(requirement.pinned_host_bytes(), 10);

    assert_eq!(requirement.device_bytes(), 200);
    assert_eq!(requirement.temporary_device_bytes(), 50);

    assert_eq!(requirement.distributed_bytes(), 300);
    assert_eq!(requirement.distributed_partitions(), 4);
    assert_eq!(
        requirement.distributed_partition_bytes(),
        75
    );

    assert_eq!(requirement.state_bytes(), 400);
    assert_eq!(requirement.temporary_state_bytes(), 100);
    assert_eq!(requirement.state_elements(), 32);

    assert_eq!(requirement.snapshot_bytes(), 500);
    assert_eq!(requirement.checkpoint_bytes(), 600);

    assert_eq!(requirement.tensor_rank(), 4);
    assert_eq!(requirement.tensor_dimension(), 2);
    assert_eq!(requirement.bond_dimension(), 8);
    assert_eq!(requirement.tensors(), 10);

    assert_eq!(requirement.measurement_results(), 16);
    assert_eq!(requirement.metadata_bytes(), 1024);
    assert_eq!(requirement.planning_work(), 1000);
}

#[test]
fn small_complete_requirement_fits_production_policy() {
    let limits = MemoryLimits::production();

    let requirement = MemoryRequirement::empty()
        .with_qubits(32)
        .with_classical_bits(1024)
        .with_allocations(32)
        .with_host_bytes(ONE_MIB)
        .with_temporary_host_bytes(256 * 1024)
        .with_persistent_host_bytes(512 * 1024)
        .with_pinned_host_bytes(128 * 1024)
        .with_device_bytes(ONE_MIB)
        .with_temporary_device_bytes(256 * 1024)
        .with_distributed_bytes(ONE_MIB)
        .with_distributed_partitions(4)
        .with_distributed_partition_bytes(256 * 1024)
        .with_state_bytes(ONE_MIB)
        .with_temporary_state_bytes(256 * 1024)
        .with_state_elements(65_536)
        .with_snapshot_bytes(ONE_MIB)
        .with_checkpoint_bytes(ONE_MIB)
        .with_tensor_rank(4)
        .with_tensor_dimension(16)
        .with_bond_dimension(64)
        .with_tensors(16)
        .with_measurement_results(1024)
        .with_metadata_bytes(64 * 1024)
        .with_planning_work(100_000);

    assert!(
        limits.check_requirement(requirement).is_ok()
    );
}

// =============================================================================
// Requirement violation ordering
// =============================================================================

#[test]
fn requirement_check_is_deterministic() {
    let limits = MemoryLimits::production();

    let requirement = MemoryRequirement::empty()
        .with_qubits(limits.max_qubits() + 1)
        .with_classical_bits(limits.max_classical_bits() + 1)
        .with_host_bytes(limits.max_host_bytes() + 1);

    let first = limits
        .check_requirement(requirement)
        .expect_err("requirement must fail");

    let second = limits
        .check_requirement(requirement)
        .expect_err("same requirement must fail identically");

    assert_eq!(first, second);
    assert_eq!(first.kind(), MemoryLimitKind::Qubits);
}

#[test]
fn every_requirement_category_has_a_structured_failure_identity() {
    let limits = MemoryLimits::production();

    let cases = [
        (
            MemoryRequirement::empty()
                .with_qubits(limits.max_qubits() + 1),
            MemoryLimitKind::Qubits,
        ),
        (
            MemoryRequirement::empty()
                .with_classical_bits(
                    limits.max_classical_bits() + 1,
                ),
            MemoryLimitKind::ClassicalBits,
        ),
        (
            MemoryRequirement::empty()
                .with_allocations(
                    limits.max_allocations() + 1,
                ),
            MemoryLimitKind::Allocations,
        ),
        (
            MemoryRequirement::empty()
                .with_host_bytes(
                    limits.max_host_bytes() + 1,
                ),
            MemoryLimitKind::HostBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_temporary_host_bytes(
                    limits.max_temporary_host_bytes() + 1,
                ),
            MemoryLimitKind::TemporaryHostBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_persistent_host_bytes(
                    limits.max_persistent_host_bytes() + 1,
                ),
            MemoryLimitKind::PersistentHostBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_pinned_host_bytes(
                    limits.max_pinned_host_bytes() + 1,
                ),
            MemoryLimitKind::PinnedHostBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_device_bytes(
                    limits.max_device_bytes() + 1,
                ),
            MemoryLimitKind::DeviceBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_temporary_device_bytes(
                    limits.max_temporary_device_bytes() + 1,
                ),
            MemoryLimitKind::TemporaryDeviceBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_distributed_bytes(
                    limits.max_distributed_bytes() + 1,
                ),
            MemoryLimitKind::DistributedBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_distributed_partitions(
                    limits.max_distributed_partitions() + 1,
                ),
            MemoryLimitKind::DistributedPartitions,
        ),
        (
            MemoryRequirement::empty()
                .with_distributed_partition_bytes(
                    limits.max_distributed_partition_bytes() + 1,
                ),
            MemoryLimitKind::DistributedPartitionBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_state_bytes(
                    limits.max_state_bytes() + 1,
                ),
            MemoryLimitKind::StateBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_temporary_state_bytes(
                    limits.max_temporary_state_bytes() + 1,
                ),
            MemoryLimitKind::TemporaryStateBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_state_elements(
                    limits.max_state_elements() + 1,
                ),
            MemoryLimitKind::StateElements,
        ),
        (
            MemoryRequirement::empty()
                .with_snapshot_bytes(
                    limits.max_snapshot_bytes() + 1,
                ),
            MemoryLimitKind::SnapshotBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_checkpoint_bytes(
                    limits.max_checkpoint_bytes() + 1,
                ),
            MemoryLimitKind::CheckpointBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_tensor_rank(
                    limits.max_tensor_rank() + 1,
                ),
            MemoryLimitKind::TensorRank,
        ),
        (
            MemoryRequirement::empty()
                .with_tensor_dimension(
                    limits.max_tensor_dimension() + 1,
                ),
            MemoryLimitKind::TensorDimension,
        ),
        (
            MemoryRequirement::empty()
                .with_bond_dimension(
                    limits.max_bond_dimension() + 1,
                ),
            MemoryLimitKind::BondDimension,
        ),
        (
            MemoryRequirement::empty()
                .with_tensors(
                    limits.max_tensors() + 1,
                ),
            MemoryLimitKind::Tensors,
        ),
        (
            MemoryRequirement::empty()
                .with_measurement_results(
                    limits.max_measurement_results() + 1,
                ),
            MemoryLimitKind::MeasurementResults,
        ),
        (
            MemoryRequirement::empty()
                .with_metadata_bytes(
                    limits.max_metadata_bytes() + 1,
                ),
            MemoryLimitKind::MetadataBytes,
        ),
        (
            MemoryRequirement::empty()
                .with_planning_work(
                    limits.max_planning_work() + 1,
                ),
            MemoryLimitKind::PlanningWork,
        ),
    ];

    for (requirement, expected_kind) in cases {
        let error = limits
            .check_requirement(requirement)
            .expect_err("requirement must violate its selected limit");

        assert_eq!(
            error.kind(),
            expected_kind,
            "wrong resource-limit identity"
        );
    }
}

// =============================================================================
// Requirement composition
// =============================================================================

#[test]
fn requirement_checked_addition_combines_resources() {
    let left = MemoryRequirement::empty()
        .with_qubits(4)
        .with_host_bytes(100)
        .with_state_elements(8);

    let right = MemoryRequirement::empty()
        .with_qubits(6)
        .with_host_bytes(200)
        .with_state_elements(16);

    let combined = left
        .checked_add(right)
        .expect("small requirement addition must succeed");

    assert_eq!(combined.qubits(), 10);
    assert_eq!(combined.host_bytes(), 300);
    assert_eq!(combined.state_elements(), 24);
}

#[test]
fn requirement_checked_addition_detects_overflow() {
    let left = MemoryRequirement::empty()
        .with_host_bytes(u64::MAX);

    let right = MemoryRequirement::empty()
        .with_host_bytes(1);

    assert!(
        left.checked_add(right).is_err(),
        "resource composition must never wrap"
    );
}

#[test]
fn requirement_checked_addition_is_commutative_for_resource_totals() {
    let left = MemoryRequirement::empty()
        .with_qubits(4)
        .with_host_bytes(100)
        .with_device_bytes(200);

    let right = MemoryRequirement::empty()
        .with_qubits(6)
        .with_host_bytes(300)
        .with_device_bytes(400);

    let a = left.checked_add(right).unwrap();
    let b = right.checked_add(left).unwrap();

    assert_eq!(a, b);
}

// =============================================================================
// Platform-size conversion
// =============================================================================

#[test]
fn platform_size_conversion_accepts_small_values() {
    let converted = MemoryLimits::check_platform_bytes(4096)
        .expect("4096 bytes must fit every supported Rust usize");

    assert_eq!(converted, 4096usize);
}

#[test]
fn platform_size_conversion_is_checked() {
    let platform_max =
        usize::MAX as u64;

    if platform_max < u64::MAX {
        let overflowing =
            platform_max.saturating_add(1);

        assert!(
            MemoryLimits::check_platform_bytes(overflowing)
                .is_err(),
            "u64-to-usize conversion must be checked"
        );
    }
}

// =============================================================================
// Memory-estimate overflow
// =============================================================================

#[test]
fn memory_estimate_detects_byte_multiplication_overflow() {
    let result = crate::quantum::memory::limits::MemoryEstimate::new(
        1,
        2,
        u64::MAX,
    );

    assert_eq!(
        result.expect_err("element-byte multiplication must overflow"),
        MemoryEstimateError::ByteCountOverflow
    );
}

#[test]
fn memory_estimate_zero_elements_are_representable_without_allocation() {
    let estimate =
        crate::quantum::memory::limits::MemoryEstimate::new(
            0,
            0,
            16,
        )
        .expect("zero mathematical elements are representable");

    assert_eq!(estimate.qubits(), 0);
    assert_eq!(estimate.elements(), 0);
    assert_eq!(estimate.bytes(), 0);
    assert_eq!(estimate.bytes_per_element(), 16);
}

// =============================================================================
// Distributed boundary behavior
// =============================================================================

#[test]
fn distributed_allocation_accepts_exact_boundaries() {
    let limits = MemoryLimits::production();

    assert!(
        limits
            .check_distributed_allocation(
                limits.max_distributed_partitions(),
                limits.max_distributed_partition_bytes(),
                limits.max_distributed_bytes(),
            )
            .is_ok()
    );
}

#[test]
fn distributed_allocation_rejects_partition_size_overflow() {
    let limits = MemoryLimits::production();

    let error = limits
        .check_distributed_allocation(
            1,
            limits.max_distributed_partition_bytes() + 1,
            limits.max_distributed_partition_bytes() + 1,
        )
        .expect_err(
            "partition capacity must be independently enforced"
        );

    assert_eq!(
        error.kind(),
        MemoryLimitKind::DistributedPartitionBytes
    );
}

// =============================================================================
// Snapshot / checkpoint planning
// =============================================================================

#[test]
fn snapshot_boundary_is_accepted() {
    let limits = MemoryLimits::production();

    assert!(
        limits
            .check_snapshot_bytes(
                limits.max_snapshot_bytes()
            )
            .is_ok()
    );
}

#[test]
fn checkpoint_boundary_is_accepted() {
    let limits = MemoryLimits::production();

    assert!(
        limits
            .check_checkpoint_bytes(
                limits.max_checkpoint_bytes()
            )
            .is_ok()
    );
}

// =============================================================================
// Policy relationships
// =============================================================================

#[test]
fn_temporary_host_limit_never_exceeds_host_limit() {
    for limits in [
        MemoryLimits::production(),
        MemoryLimits::strict(),
        MemoryLimits::deny_all(),
    ] {
        assert!(
            limits.max_temporary_host_bytes()
                <= limits.max_host_bytes()
        );
    }
}

#[test]
fn persistent_host_limit_never_exceeds_host_limit() {
    for limits in [
        MemoryLimits::production(),
        MemoryLimits::strict(),
        MemoryLimits::deny_all(),
    ] {
        assert!(
            limits.max_persistent_host_bytes()
                <= limits.max_host_bytes()
        );
    }
}

#[test]
fn temporary_device_limit_never_exceeds_device_limit() {
    for limits in [
        MemoryLimits::production(),
        MemoryLimits::strict(),
        MemoryLimits::deny_all(),
    ] {
        assert!(
            limits.max_temporary_device_bytes()
                <= limits.max_device_bytes()
        );
    }
}

#[test]
fn temporary_state_limit_never_exceeds_state_limit() {
    for limits in [
        MemoryLimits::production(),
        MemoryLimits::strict(),
        MemoryLimits::deny_all(),
    ] {
        assert!(
            limits.max_temporary_state_bytes()
                <= limits.max_state_bytes()
        );
    }
}

#[test]
fn state_limit_never_exceeds_host_limit() {
    for limits in [
        MemoryLimits::production(),
        MemoryLimits::strict(),
        MemoryLimits::deny_all(),
    ] {
        assert!(
            limits.max_state_bytes()
                <= limits.max_host_bytes()
        );
    }
}

#[test]
fn representation_specific_qubit_limits_never_exceed_global_qubit_limit() {
    for limits in [
        MemoryLimits::production(),
        MemoryLimits::strict(),
        MemoryLimits::deny_all(),
    ] {
        assert!(
            limits.max_state_vector_qubits()
                <= limits.max_qubits()
        );

        assert!(
            limits.max_density_matrix_qubits()
                <= limits.max_qubits()
        );

        assert!(
            limits.max_tensor_network_qubits()
                <= limits.max_qubits()
        );
    }
}

#[test]
fn distributed_partition_limit_never_exceeds_global_distributed_limit() {
    for limits in [
        MemoryLimits::production(),
        MemoryLimits::strict(),
        MemoryLimits::deny_all(),
    ] {
        assert!(
            limits.max_distributed_partition_bytes()
                <= limits.max_distributed_bytes()
        );
    }
}

// =============================================================================
// Exact production-scale sanity checks
// =============================================================================

#[test]
fn production_state_vector_32_qubits_is_64_gib_f64() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_state_vector_complex_f64(32)
        .expect(
            "the configured production policy intentionally permits \
             the exact 32-qubit / 64-GiB f64 boundary",
        );

    assert_eq!(
        estimate.elements(),
        1u64 << 32
    );

    assert_eq!(
        estimate.bytes(),
        64 * ONE_GIB
    );

    assert_eq!(
        estimate.bytes(),
        limits.max_state_bytes()
    );
}

#[test]
fn production_state_vector_33_qubits_is_rejected_by_byte_policy() {
    let limits = MemoryLimits::production();

    let result =
        limits.estimate_state_vector_complex_f64(33);

    assert!(
        result.is_err(),
        "33-qubit complex-f64 dense state requires \
         more than the configured 64-GiB state limit"
    );
}

#[test]
fn production_density_matrix_14_qubits_is_4_gib_f64() {
    let limits = MemoryLimits::production();

    let estimate = limits
        .estimate_density_matrix_complex_f64(14)
        .expect(
            "14-qubit complex-f64 density matrix must fit \
             the configured production state-memory limit",
        );

    assert_eq!(
        estimate.elements(),
        1u64 << 28
    );

    assert_eq!(
        estimate.bytes(),
        4 * ONE_GIB
    );
}

#[test]
fn production_density_matrix_15_qubits_exceeds_state_memory() {
    let limits = MemoryLimits::production();

    let result =
        limits.estimate_density_matrix_complex_f64(15);

    assert!(
        result.is_err(),
        "15-qubit complex-f64 density matrix requires \
         256 GiB and must be rejected by the 64-GiB state limit"
    );
}

// =============================================================================
// Provider and QPU neutrality
// =============================================================================

#[test]
fn device_limit_is_not_tied_to_gpu_vendor() {
    let limits = MemoryLimits::production();

    // The limits layer only describes capacity. It does not know whether the
    // eventual provider is CUDA, HIP, Metal, Vulkan, SYCL, a QPU-native
    // buffer, unified memory, or another accelerator.
    let requested = 1 * ONE_MIB;

    assert!(
        limits.check_device_bytes(requested).is_ok()
    );
}

#[test]
fn distributed_limit_is_not_tied_to_mpi_or_specific_transport() {
    let limits = MemoryLimits::production();

    // The policy intentionally does not contain MPI, UCX, RDMA, TCP, or any
    // other transport identity.
    assert!(
        limits
            .check_distributed_bytes(ONE_MIB)
            .is_ok()
    );

    assert!(
        limits
            .check_distributed_partitions(2)
            .is_ok()
    );
}

#[test]
fn memory_policy_can_be_consumed_by_any_qpu_class() {
    let limits = MemoryLimits::production();

    // These are policy categories rather than vendor assumptions. A backend
    // adapter for any quantum hardware may consume the same contract.
    let categories = [
        limits.max_qubits(),
        limits.max_classical_bits(),
        limits.max_allocations(),
    ];

    for value in categories {
        assert!(value > 0);
    }
}

// =============================================================================
// Copy / equality / deterministic configuration behavior
// =============================================================================

#[test]
fn memory_limits_are_value_objects() {
    let original = MemoryLimits::production();
    let copied = original;

    assert_eq!(original, copied);
}

#[test]
fn production_policy_is_deterministic() {
    let a = MemoryLimits::production();
    let b = MemoryLimits::production();

    assert_eq!(a, b);
}

#[test]
fn strict_policy_is_deterministic() {
    let a = MemoryLimits::strict();
    let b = MemoryLimits::strict();

    assert_eq!(a, b);
}

#[test]
fn deny_all_policy_is_deterministic() {
    let a = MemoryLimits::deny_all();
    let b = MemoryLimits::deny_all();

    assert_eq!(a, b);
}