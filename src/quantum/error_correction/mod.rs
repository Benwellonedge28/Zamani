//! Zamani Quantum Error Correction (QEC) subsystem.
//!
//! # Root integration module
//!
//! This file is the public module registry and architectural boundary for the
//! complete QEC subsystem. It contains no decoder algorithms, numerical
//! implementations, QPU implementations, persistence engines, or execution
//! algorithms.
//!
//! Its responsibilities are:
//!
//! - declare every QEC production module;
//! - declare the QEC test registry;
//! - expose canonical public contracts;
//! - define subsystem identity;
//! - define the public API version identity;
//! - define execution-environment classification;
//! - expose the compile-time capability inventory;
//! - expose the fail-closed QPU access state;
//! - provide a bounded, hardware-independent self-check;
//! - document the dependency direction and integration contract.
//!
//! # Rust compatibility
//!
//! This module is written for the repository's Rust 1.97.1 toolchain.
//!
//! No unstable language features are required.
//!
//! # Architectural rule
//!
//! `mod.rs` is an integration boundary, not an implementation layer.
//!
//! ```text
//!                         QEC ROOT
//!                            │
//!              ┌─────────────┼─────────────┐
//!              │             │             │
//!              ▼             ▼             ▼
//!        FOUNDATION       CONTROL       DATA/MATH
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            │
//!                            ▼
//!                         DECODERS
//!                            │
//!              ┌─────────────┼─────────────┐
//!              ▼             ▼             ▼
//!          CLASSICAL       SCALE          QPU
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                    RESULTS / RECOVERY
//!                            │
//!                            ▼
//!                     VERIFICATION
//! ```
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! arithmetic
//!     │
//! errors / version / limits
//!     │
//! cancellation / deterministic
//!     │
//! memory / resources
//!     │
//! validation / capabilities / configuration
//!     │
//! representations
//!     │
//! surface-code mathematics
//!     │
//! decoding graph
//!     │
//! decoder contract
//!     │
//! decoder implementations
//!     │
//! canonical decoder result
//!     │
//! Pauli frame / logical equivalence
//!     │
//! execution / streaming / partition / distributed / QPU
//!     │
//! checkpoint / cache / replay
//!     │
//! statistical / verification
//! ```
//!
//! The root module does not enforce this dependency graph through Rust module
//! ordering. Individual modules remain responsible for respecting the
//! architectural contracts documented in their own files.
//!
//! # Resource model
//!
//! ```text
//! limits.rs
//!     = what is permitted
//!
//! resource_estimator.rs
//!     = what is expected to be required
//!
//! memory.rs
//!     = allocation enforcement
//!
//! resources.rs
//!     = runtime accounting
//!
//! scheduler.rs
//!     = admission/execution lifecycle
//! ```
//!
//! No module may silently introduce an independent production-wide resource
//! policy.
//!
//! # Security model
//!
//! Capability authorization belongs to `capabilities.rs`.
//!
//! Execution-environment classification in this file is informational and
//! does not grant authorization.
//!
//! In particular:
//!
//! ```text
//! ExecutionEnvironment::Qpu
//!         !=
//! QPU authorization
//! ```
//!
//! QPU access must remain explicitly authorized through the capability layer.
//!
//! # QPU boundary
//!
//! ```text
//! SurfaceCode
//!      │
//!      ▼
//! validation
//!      │
//!      ▼
//! capability authorization
//!      │
//!      ▼
//! resource admission
//!      │
//!      ▼
//! qpu_adapter
//!      │
//!      ▼
//! QPU execution boundary
//!      │
//!      ▼
//! syndrome_extractor
//!      │
//!      ▼
//! syndrome
//!      │
//!      ▼
//! decoder
//!      │
//!      ▼
//! decoder_result
//!      │
//!      ▼
//! pauli_frame / logical_equivalence
//! ```
//!
//! Mathematical verification must never require physical QPU access.
//!
//! # Determinism
//!
//! Deterministic execution is owned by `deterministic.rs`.
//!
//! This root module only exposes deterministic subsystem metadata and stable
//! execution-environment ordering.
//!
//! # Error boundary
//!
//! `errors.rs` owns the canonical QEC error boundary.
//!
//! The root module does not duplicate decoder, numerical, QPU, resource,
//! checkpoint, or version error types.
//!
//! # Canonical result boundary
//!
//! `decoder_result.rs` owns `DecodeResult`.
//!
//! `decoder.rs` owns the decoder execution contract.
//!
//! Concrete decoders such as MWPM and Union-Find must return the canonical
//! result type rather than defining competing result structures.
//!
//! # Testing
//!
//! The complete QEC test suite is registered through `tests/mod.rs`.
//!
//! Mathematical verification, determinism, resource, decoder, security,
//! fault-injection, scalability and regression tests remain separate from
//! this root module.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/* ========================================================================= */
/* FOUNDATION                                                                */
/* ========================================================================= */

/// Checked numerical operations and numerical validation.
pub mod arithmetic;

/// Canonical QEC error boundary.
pub mod errors;

/// Canonical version and artifact compatibility contracts.
pub mod version;

/// Global workload/resource policy.
pub mod limits;

/// Cooperative cancellation and deadlines.
pub mod cancellation;

/// Deterministic execution context and ordering.
pub mod deterministic;

/// Allocation and memory-reservation enforcement.
pub mod memory;

/// Runtime resource accounting and snapshots.
pub mod resources;

/* ========================================================================= */
/* CONTROL PLANE                                                             */
/* ========================================================================= */

/// Input and structural validation.
pub mod validation;

/// Capability-based authorization.
pub mod capabilities;

/// Complete validated QEC configuration.
pub mod configuration;

/// Structured QEC execution metrics.
pub mod metrics;

/// Privacy-aware observability and telemetry.
pub mod telemetry;

/* ========================================================================= */
/* DATA REPRESENTATIONS                                                      */
/* ========================================================================= */

/// Sparse data structures used throughout QEC.
pub mod sparse;

/// Stabilizer and Pauli algebra.
pub mod stabilizer;

/// Syndrome representation and validation.
pub mod syndrome;

/// Logical-state and logical-outcome classification.
pub mod logical;

/// Pauli-frame state tracking.
pub mod pauli_frame;

/// Formal stabilizer/logical equivalence analysis.
pub mod logical_equivalence;

/* ========================================================================= */
/* CODE AND GRAPH MATHEMATICS                                                */
/* ========================================================================= */

/// Surface-code mathematical topology.
pub mod surface_code;

/// Code-distance verification.
pub mod distance;

/// Sparse decoding graph construction and validation.
pub mod decoding_graph;

/* ========================================================================= */
/* DECODER CONTRACT AND IMPLEMENTATIONS                                      */
/* ========================================================================= */

/// Common decoder execution contract.
pub mod decoder;

/// Canonical result returned by every decoder.
pub mod decoder_result;

/// Minimum-weight perfect matching decoder.
pub mod mwpm;

/// Union-Find decoder.
pub mod union_find;

/* ========================================================================= */
/* PHYSICAL / SIMULATION EXECUTION                                           */
/* ========================================================================= */

/// Execution backend abstraction.
pub mod backend;

/// Configurable physical noise models.
pub mod noise;

/// Statistical QEC simulation.
pub mod simulation;

/// Surface-code circuit/execution integration.
pub mod surface_coder;

/* ========================================================================= */
/* LARGE-SCALE EXECUTION                                                     */
/* ========================================================================= */

/// Bounded incremental/streaming decoding.
pub mod streaming;

/// Partitioned decoding and boundary reconciliation.
pub mod partition;

/// Distributed classical decoding.
pub mod distributed;

/// Execution scheduling and lifecycle management.
pub mod scheduler;

/* ========================================================================= */
/* STATE, RECOVERY AND REPRODUCTION                                          */
/* ========================================================================= */

/// Resource estimation before execution/allocation.
pub mod resource_estimator;

/// Checkpoint creation, validation and recovery.
pub mod checkpoint;

/// Validated reusable computation/cache layer.
pub mod cache;

/// Deterministic replay/reproduction layer.
pub mod replay;

/* ========================================================================= */
/* QPU BOUNDARY                                                              */
/* ========================================================================= */

/// Explicit QPU execution boundary.
pub mod qpu_adapter;

/// Conversion of QPU measurements into validated syndrome events.
pub mod syndrome_extractor;

/* ========================================================================= */
/* STATISTICS AND VERIFICATION                                               */
/* ========================================================================= */

/// Confidence intervals, stopping criteria and statistical contracts.
pub mod statistical;

/// Cross-layer mathematical and execution verification.
pub mod verification;

/* ========================================================================= */
/* PUBLIC SUBSYSTEM IDENTITY                                                 */
/* ========================================================================= */

/// Stable public identifier of the QEC subsystem.
pub const QEC_SUBSYSTEM_NAME: &str =
    "zamani.quantum.error_correction";

/// Canonical public QEC API version.
///
/// This string intentionally matches the canonical `Version` contract in
/// `version.rs`.
pub const QEC_API_VERSION: &str = "3.0.0";

/// Stable architecture identifier.
pub const QEC_ARCHITECTURE: &str =
    "resource-safe-scalable-validated-qec";

/// Returns the public QEC API version.
#[must_use]
pub const fn api_version() -> &'static str {
    QEC_API_VERSION
}

/* ========================================================================= */
/* CANONICAL PUBLIC RE-EXPORTS                                               */
/* ========================================================================= */

// Error boundary.
pub use errors::{
    DecoderKind,
    NumericalOperation,
    QecError,
    QecResult,
    ResourceKind,
};

// Version boundary.
pub use version::{
    ArtifactKind,
    ArtifactHeader,
    ExecutionTarget,
    FeatureFlags,
    Version,
    VersionManifest,
    CURRENT_ALGORITHM_VERSION,
    CURRENT_BACKEND_VERSION,
    CURRENT_CACHE_VERSION,
    CURRENT_CAPABILITY_VERSION,
    CURRENT_CHECKPOINT_VERSION,
    CURRENT_CONFIGURATION_VERSION,
    CURRENT_DECODER_OUTPUT_VERSION,
    CURRENT_DECODER_RESULT_VERSION,
    CURRENT_DISTRIBUTED_VERSION,
    CURRENT_GRAPH_VERSION,
    CURRENT_NOISE_MODEL_VERSION,
    CURRENT_PARTITION_VERSION,
    CURRENT_QPU_EXECUTION_VERSION,
    CURRENT_QPU_INTERFACE_VERSION,
    CURRENT_QEC_VERSION,
    CURRENT_REPLAY_VERSION,
    CURRENT_SIMULATION_VERSION,
    CURRENT_STREAMING_VERSION,
    CURRENT_SYNDROME_VERSION,
};

// Decoder contract/result boundary.
//
// `DecodeResult` is deliberately re-exported from `decoder_result`, not
// `decoder`, because decoder.rs owns execution while decoder_result.rs owns
// the result contract.
pub use decoder::{
    DecodeContext,
    Decoder,
    DecoderRegistry,
};

pub use decoder_result::{
    Correction,
    DecodeMetadata,
    DecodeResourceUsage,
    DecodeResult,
    DecodeTermination,
    DecodeWitness,
    DecoderId,
};

// Mathematical public primitives.
pub use logical::LogicalOutcome;

pub use stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
    StabilizerGenerator,
    StabilizerGroup,
};

/* ========================================================================= */
/* EXECUTION ENVIRONMENT                                                     */
/* ========================================================================= */

/// Execution environment represented by the QEC subsystem.
///
/// This is a classification only. It does not grant authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionEnvironment {
    /// Single-threaded classical CPU execution.
    Cpu,

    /// Parallel classical CPU execution.
    ParallelCpu,

    /// GPU execution.
    Gpu,

    /// Generic hardware accelerator.
    Accelerator,

    /// Physical quantum processing unit.
    Qpu,

    /// Distributed classical execution.
    Distributed,
}

impl ExecutionEnvironment {
    /// Returns `true` only for QPU execution.
    #[must_use]
    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether the environment is classical.
    #[must_use]
    pub const fn is_classical(self) -> bool {
        matches!(
            self,
            Self::Cpu
                | Self::ParallelCpu
                | Self::Gpu
                | Self::Accelerator
        )
    }

    /// Returns whether the environment is distributed.
    #[must_use]
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }
}

/// Stable ordering of all execution environments represented by QEC.
#[must_use]
pub const fn supported_execution_environments(
) -> &'static [ExecutionEnvironment] {
    &[
        ExecutionEnvironment::Cpu,
        ExecutionEnvironment::ParallelCpu,
        ExecutionEnvironment::Gpu,
        ExecutionEnvironment::Accelerator,
        ExecutionEnvironment::Qpu,
        ExecutionEnvironment::Distributed,
    ]
}

/* ========================================================================= */
/* SUBSYSTEM CAPABILITY INVENTORY                                            */
/* ========================================================================= */

/// Compile-time inventory of functionality exposed by the QEC subsystem.
///
/// This structure is an inventory, not an authorization token.
///
/// Authorization remains owned by `capabilities.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QecCapabilities {
    /* Mathematics */
    pub stabilizer_algebra: bool,
    pub syndrome_generation: bool,
    pub decoding_graph: bool,
    pub distance_verification: bool,
    pub logical_operators: bool,
    pub logical_equivalence: bool,
    pub mwpm: bool,
    pub union_find: bool,
    pub noise_models: bool,
    pub pauli_frame: bool,
    pub surface_code: bool,
    pub simulation: bool,
    pub decoder_interface: bool,

    /* Safety/control */
    pub validation: bool,
    pub resource_limits: bool,
    pub resource_estimation: bool,
    pub resource_accounting: bool,
    pub memory_management: bool,
    pub safe_arithmetic: bool,
    pub cancellation: bool,
    pub deterministic_execution: bool,

    /* Scale */
    pub sparse_data: bool,
    pub streaming: bool,
    pub partitioning: bool,
    pub distributed_execution: bool,
    pub scheduling: bool,

    /* Recovery/reproduction */
    pub checkpointing: bool,
    pub caching: bool,
    pub replay: bool,

    /* Observability/statistics */
    pub metrics: bool,
    pub telemetry: bool,
    pub statistical_analysis: bool,
    pub verification: bool,

    /* Backends */
    pub cpu_backend: bool,
    pub parallel_cpu_backend: bool,
    pub gpu_backend: bool,
    pub accelerator_backend: bool,
    pub qpu_boundary: bool,

    /* Compatibility/security */
    pub capability_security: bool,
    pub configuration_management: bool,
    pub versioning: bool,
}

/// Complete functionality inventory for the current QEC subsystem.
pub const QEC_CAPABILITIES: QecCapabilities = QecCapabilities {
    stabilizer_algebra: true,
    syndrome_generation: true,
    decoding_graph: true,
    distance_verification: true,
    logical_operators: true,
    logical_equivalence: true,
    mwpm: true,
    union_find: true,
    noise_models: true,
    pauli_frame: true,
    surface_code: true,
    simulation: true,
    decoder_interface: true,

    validation: true,
    resource_limits: true,
    resource_estimation: true,
    resource_accounting: true,
    memory_management: true,
    safe_arithmetic: true,
    cancellation: true,
    deterministic_execution: true,

    sparse_data: true,
    streaming: true,
    partitioning: true,
    distributed_execution: true,
    scheduling: true,

    checkpointing: true,
    caching: true,
    replay: true,

    metrics: true,
    telemetry: true,
    statistical_analysis: true,
    verification: true,

    cpu_backend: true,
    parallel_cpu_backend: true,
    gpu_backend: true,
    accelerator_backend: true,
    qpu_boundary: true,

    capability_security: true,
    configuration_management: true,
    versioning: true,
};

/// Returns the current QEC subsystem capability inventory.
#[must_use]
pub const fn capabilities() -> QecCapabilities {
    QEC_CAPABILITIES
}

impl QecCapabilities {
    /// Returns whether the subsystem exposes the specified execution class.
    #[must_use]
    pub const fn supports_execution(
        self,
        environment: ExecutionEnvironment,
    ) -> bool {
        match environment {
            ExecutionEnvironment::Cpu => self.cpu_backend,
            ExecutionEnvironment::ParallelCpu => {
                self.parallel_cpu_backend
            }
            ExecutionEnvironment::Gpu => self.gpu_backend,
            ExecutionEnvironment::Accelerator => {
                self.accelerator_backend
            }
            ExecutionEnvironment::Qpu => self.qpu_boundary,
            ExecutionEnvironment::Distributed => {
                self.distributed_execution
            }
        }
    }
}

/* ========================================================================= */
/* QPU FAIL-CLOSED STATE                                                     */
/* ========================================================================= */

/// Authorization state at the QPU boundary.
///
/// This does not contain credentials and does not perform hardware access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QpuAccess {
    /// QPU execution is denied.
    Denied,

    /// QPU exists conceptually, but capability authorization is required.
    RequiresCapability,

    /// Explicit QPU capability authorization has been granted.
    Authorized,
}

impl QpuAccess {
    /// Returns whether physical QPU execution is authorized.
    #[must_use]
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::Authorized)
    }
}

/* ========================================================================= */
/* BOUNDED HARDWARE-INDEPENDENT SELF CHECK                                   */
/* ========================================================================= */

/// Errors returned by the root QEC structural self-check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QecSelfCheckError {
    /// Surface-code construction or validation failed.
    SurfaceCodeInvariant,

    /// The public subsystem metadata is inconsistent.
    MetadataInvariant,
}

impl core::fmt::Display for QecSelfCheckError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::SurfaceCodeInvariant => {
                formatter.write_str(
                    "QEC surface-code invariant failed",
                )
            }
            Self::MetadataInvariant => {
                formatter.write_str(
                    "QEC subsystem metadata invariant failed",
                )
            }
        }
    }
}

impl std::error::Error for QecSelfCheckError {}

/// Performs a small, deterministic, hardware-independent QEC self-check.
///
/// The check deliberately:
///
/// - performs no network access;
/// - accesses no physical QPU;
/// - accesses no credentials;
/// - creates no distributed job;
/// - requires no GPU;
/// - uses only a bounded distance-3 surface-code fixture.
///
/// It verifies that the public root integration is connected to the
/// mathematical surface-code layer.
pub fn self_check() -> Result<(), QecSelfCheckError> {
    if QEC_SUBSYSTEM_NAME.is_empty()
        || QEC_API_VERSION.is_empty()
        || QEC_ARCHITECTURE.is_empty()
    {
        return Err(QecSelfCheckError::MetadataInvariant);
    }

    let code = surface_code::SurfaceCode::new(3)
        .map_err(|_| QecSelfCheckError::SurfaceCodeInvariant)?;

    code.validate()
        .map_err(|_| QecSelfCheckError::SurfaceCodeInvariant)?;

    code.validate_logical_operators()
        .map_err(|_| QecSelfCheckError::SurfaceCodeInvariant)?;

    Ok(())
}

/* ========================================================================= */
/* TEST REGISTRY                                                             */
/* ========================================================================= */

/// Complete focused QEC test registry.
///
/// The individual test files are declared by `tests/mod.rs`; keeping that
/// registry below the production declarations ensures tests never become
/// production dependencies.
#[cfg(test)]
mod tests;

/* ========================================================================= */
/* ROOT INTEGRATION TESTS                                                    */
/* ========================================================================= */

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn subsystem_identity_is_stable() {
        assert_eq!(
            QEC_SUBSYSTEM_NAME,
            "zamani.quantum.error_correction"
        );

        assert_eq!(
            QEC_API_VERSION,
            "3.0.0"
        );

        assert_eq!(
            QEC_ARCHITECTURE,
            "resource-safe-scalable-validated-qec"
        );
    }

    #[test]
    fn all_execution_environments_are_registered() {
        let environments =
            supported_execution_environments();

        assert_eq!(environments.len(), 6);

        for environment in environments {
            assert!(
                capabilities()
                    .supports_execution(*environment),
                "missing capability inventory entry for {:?}",
                environment
            );
        }
    }

    #[test]
    fn qpu_is_not_authorization() {
        assert!(
            ExecutionEnvironment::Qpu.is_qpu()
        );

        assert!(
            !ExecutionEnvironment::Qpu.is_classical()
        );

        assert!(
            !QpuAccess::Denied.is_authorized()
        );

        assert!(
            !QpuAccess::RequiresCapability.is_authorized()
        );

        assert!(
            QpuAccess::Authorized.is_authorized()
        );
    }

    #[test]
    fn root_self_check_is_hardware_independent() {
        assert!(
            self_check().is_ok(),
            "QEC root self-check failed"
        );
    }

    #[test]
    fn canonical_decoder_result_is_reexported() {
        let _ = core::mem::size_of::<DecodeResult>();
        let _ = core::mem::size_of::<Correction>();
        let _ = core::mem::size_of::<DecodeTermination>();
        let _ = core::mem::size_of::<DecoderId>();
    }

    #[test]
    fn canonical_version_contract_is_reexported() {
        assert_eq!(
            CURRENT_QEC_VERSION,
            Version::current()
        );

        assert_eq!(
            CURRENT_DECODER_RESULT_VERSION,
            ArtifactKind::DecoderResult.current_version()
        );
    }
}