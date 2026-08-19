//! Zamani Quantum Error Correction (QEC) subsystem.
//!
//! Production-oriented fault-tolerance infrastructure for:
//!
//! - Pauli and stabilizer algebra;
//! - syndrome generation;
//! - decoding graphs;
//! - MWPM and Union-Find decoding;
//! - logical-operator classification;
//! - code-distance verification;
//! - surface codes;
//! - configurable noise models;
//! - Pauli-frame tracking;
//! - simulation and threshold experiments;
//! - resource-aware execution;
//! - sparse representations;
//! - streaming;
//! - partitioned and distributed decoding;
//! - cancellation;
//! - deterministic execution;
//! - checkpointing;
//! - scheduling;
//! - memory accounting;
//! - caching;
//! - CPU/GPU/accelerator execution;
//! - explicit QPU execution boundaries;
//! - capability-based authorization;
//! - configuration and version compatibility;
//! - telemetry and metrics;
//! - mathematical, property, fuzz, security and regression testing.
//!
//! # Architectural contract
//!
//! ```text
//! UNTRUSTED INPUT
//!       |
//!       v
//! CONFIGURATION
//!       |
//!       v
//! VALIDATION
//!       |
//!       v
//! CAPABILITY CHECK
//!       |
//!       v
//! RESOURCE PREFLIGHT
//!       |
//!       v
//! MEMORY RESERVATION
//!       |
//!       v
//! CANCELLATION + DETERMINISM
//!       |
//!       v
//! SYNDROME / DETECTION EVENTS
//!       |
//!       v
//! SPARSE DECODING GRAPH
//!       |
//!       +------------------+
//!       |                  |
//!       v                  v
//!     MWPM             UNION-FIND
//!       |                  |
//!       +--------+---------+
//!                |
//!                v
//!           CORRECTION
//!                |
//!                v
//!           PAULI FRAME
//!                |
//!                v
//!       LOGICAL EQUIVALENCE
//!                |
//!                v
//!        LOGICAL OUTCOME
//! ```
//!
//! QPU execution is a separate physical-execution boundary:
//!
//! ```text
//! QEC CODE
//!    |
//!    v
//! SYNDROME-EXTRACTION CIRCUIT
//!    |
//!    v
//! QPU
//!    |
//!    v
//! MEASUREMENTS
//!    |
//!    v
//! SYNDROME VALIDATION
//!    |
//!    v
//! DETECTION EVENTS
//!    |
//!    v
//! DECODER
//!    |
//!    v
//! PAULI FRAME
//!    |
//!    v
//! LOGICAL OUTCOME
//! ```
//!
//! Mathematical verification must never require physical QPU access.
//!
//! # Scalability contract
//!
//! Zamani does not promise literally infinite memory or execution time.
//!
//! The scalability target is arbitrarily large workloads subject to explicit
//! resource limits, using sparse, streaming, partitioned, distributed and
//! resumable execution.
//!
//! Resource exhaustion must produce a deterministic error rather than an
//! uncontrolled allocation, panic or process termination.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/* -------------------------------------------------------------------------- */
/* Core mathematical QEC modules                                             */
/* -------------------------------------------------------------------------- */

pub mod decoder;
pub mod decoding_graph;
pub mod distance;
pub mod logical;
pub mod mwpm;
pub mod noise;
pub mod pauli_frame;
pub mod simulation;
pub mod stabilizer;
pub mod surface_code;
pub mod surface_coder;
pub mod syndrome;
pub mod union_find;

/* -------------------------------------------------------------------------- */
/* Resource, safety and execution infrastructure                             */
/* -------------------------------------------------------------------------- */

pub mod arithmetic;
pub mod backend;
pub mod cache;
pub mod cancellation;
pub mod capabilities;
pub mod checkpoint;
pub mod configuration;
pub mod deterministic;
pub mod distributed;
pub mod errors;
pub mod limits;
pub mod memory;
pub mod metrics;
pub mod partition;
pub mod resources;
pub mod scheduler;
pub mod sparse;
pub mod streaming;
pub mod telemetry;
pub mod validation;
pub mod version;

/* -------------------------------------------------------------------------- */
/* Canonical public API                                                      */
/* -------------------------------------------------------------------------- */

pub use decoder::{
    single_qubit_error,
    validate_correction,
    validate_correction_for_syndrome,
    validate_syndrome,
    x_error,
    y_error,
    z_error,
    Correction,
    DecodeResult,
    Decoder,
    DecoderError,
    DecoderId,
    DecoderRegistry,
    DecoderStatistics,
    IdentityDecoder,
    StabilizerDecoder,
    SyndromeClass,
};

pub use distance::{
    compute_distance,
    distance,
    find_logical_operator_of_weight,
    validate_distance,
    CodeDistance,
    DistanceError,
};

pub use errors::{
    DecoderKind,
    NumericalOperation,
    QecError,
    QecResult,
    ResourceKind,
};

pub use stabilizer::{
    commutes_with_stabilizer_group,
    logical_operators_anticommute,
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
    StabilizerGenerator,
    StabilizerGroup,
    Syndrome as StabilizerSyndrome,
};

/* -------------------------------------------------------------------------- */
/* Subsystem identity and version                                            */
/* -------------------------------------------------------------------------- */

/// Public QEC subsystem API version.
pub const QEC_API_VERSION: &str = "2.1.0";

/// Stable subsystem identifier.
pub const QEC_SUBSYSTEM_NAME: &str =
    "zamani.quantum.error_correction";

/// Architecture identifier.
pub const QEC_ARCHITECTURE: &str =
    "resource-safe-scalable-validated-qec";

/// Returns the QEC API version.
pub const fn api_version() -> &'static str {
    QEC_API_VERSION
}

/* -------------------------------------------------------------------------- */
/* Execution environments                                                    */
/* -------------------------------------------------------------------------- */

/// Physical/classical environments supported by the QEC subsystem.
///
/// This is an execution classification, not an authorization mechanism.
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

    /// Physical or externally managed quantum processing unit.
    Qpu,

    /// Multi-worker/distributed execution.
    Distributed,
}

impl ExecutionEnvironment {
    /// Returns `true` only for QPU execution.
    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether the environment performs classical computation.
    pub const fn is_classical(self) -> bool {
        matches!(
            self,
            Self::Cpu
                | Self::ParallelCpu
                | Self::Gpu
                | Self::Accelerator
        )
    }

    /// Returns whether the environment represents distributed execution.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }
}

/* -------------------------------------------------------------------------- */
/* QEC capability inventory                                                  */
/* -------------------------------------------------------------------------- */

/// Compile-time capability inventory.
///
/// These fields describe functionality exposed by the subsystem. They are
/// deliberately separate from authorization, which belongs to
/// `capabilities.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QecCapabilities {
    /* Mathematical layer */
    pub stabilizer_algebra: bool,
    pub syndrome_generation: bool,
    pub decoding_graph: bool,
    pub distance_verification: bool,
    pub logical_operators: bool,
    pub mwpm: bool,
    pub union_find: bool,
    pub noise_models: bool,
    pub pauli_frame: bool,
    pub surface_code: bool,
    pub simulation: bool,
    pub decoder_interface: bool,

    /* Resource/safety layer */
    pub validation: bool,
    pub resource_limits: bool,
    pub resource_accounting: bool,
    pub memory_management: bool,
    pub safe_arithmetic: bool,
    pub cancellation: bool,
    pub deterministic_execution: bool,

    /* Large-scale execution */
    pub sparse_data: bool,
    pub streaming: bool,
    pub partitioning: bool,
    pub distributed_execution: bool,
    pub scheduling: bool,
    pub checkpointing: bool,
    pub caching: bool,

    /* Observability */
    pub metrics: bool,
    pub telemetry: bool,

    /* Execution backends */
    pub cpu_backend: bool,
    pub parallel_cpu_backend: bool,
    pub gpu_backend: bool,
    pub accelerator_backend: bool,
    pub qpu_backend: bool,

    /* Security/compatibility */
    pub capability_security: bool,
    pub configuration_management: bool,
    pub versioning: bool,
}

impl QecCapabilities {
    /// Capabilities compiled into this subsystem.
    pub const CURRENT: Self = Self {
        stabilizer_algebra: true,
        syndrome_generation: true,
        decoding_graph: true,
        distance_verification: true,
        logical_operators: true,
        mwpm: true,
        union_find: true,
        noise_models: true,
        pauli_frame: true,
        surface_code: true,
        simulation: true,
        decoder_interface: true,

        validation: true,
        resource_limits: true,
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

        metrics: true,
        telemetry: true,

        cpu_backend: true,
        parallel_cpu_backend: true,
        gpu_backend: true,
        accelerator_backend: true,
        qpu_backend: true,

        capability_security: true,
        configuration_management: true,
        versioning: true,
    };

    /// Returns whether this subsystem exposes an execution environment.
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
            ExecutionEnvironment::Qpu => self.qpu_backend,
            ExecutionEnvironment::Distributed => {
                self.distributed_execution
            }
        }
    }
}

/// Returns the current QEC capability inventory.
pub const fn capabilities() -> QecCapabilities {
    QecCapabilities::CURRENT
}

/* -------------------------------------------------------------------------- */
/* QPU safety boundary                                                       */
/* -------------------------------------------------------------------------- */

/// QPU authorization state.
///
/// This type deliberately does not perform device discovery, network I/O or
/// credential handling. Those responsibilities belong to the backend and
/// capability layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QpuAccess {
    /// QPU access is unavailable.
    Denied,

    /// A QPU exists, but explicit capability authorization is required.
    RequiresCapability,

    /// QPU access has been explicitly authorized.
    Authorized,
}

impl QpuAccess {
    /// Returns whether QPU execution is authorized.
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::Authorized)
    }
}

/* -------------------------------------------------------------------------- */
/* Supported execution environments                                         */
/* -------------------------------------------------------------------------- */

/// Returns every execution environment represented by this subsystem.
///
/// The returned order is stable for deterministic diagnostics and tests.
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

/* -------------------------------------------------------------------------- */
/* Deterministic structural self-check                                       */
/* -------------------------------------------------------------------------- */

/// Performs a bounded, hardware-independent QEC self-check.
///
/// This function deliberately does not:
///
/// - access a QPU;
/// - access a GPU;
/// - access the network;
/// - allocate unbounded memory;
/// - execute distributed jobs;
/// - mutate persistent state.
///
/// It verifies the fundamental identity/trivial-syndrome invariant.
pub fn self_check() -> Result<(), QecSelfCheckError> {
    let stabilizers =
        StabilizerGroup::new(1)
            .map_err(QecSelfCheckError::Stabilizer)?;

    stabilizers
        .validate()
        .map_err(QecSelfCheckError::Stabilizer)?;

    let identity = PauliString::identity(1);

    let syndrome = stabilizers
        .syndrome(&identity)
        .map_err(QecSelfCheckError::Stabilizer)?;

    if !syndrome.is_trivial() {
        return Err(
            QecSelfCheckError::InvalidIdentitySyndrome
        );
    }

    Ok(())
}

/// Errors produced by [`self_check`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QecSelfCheckError {
    /// Stabilizer infrastructure failed its invariant check.
    Stabilizer(StabilizerError),

    /// Identity produced a non-trivial syndrome.
    InvalidIdentitySyndrome,
}

impl std::fmt::Display for QecSelfCheckError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    f,
                    "QEC stabilizer self-check failed: {error}"
                )
            }

            Self::InvalidIdentitySyndrome => {
                write!(
                    f,
                    "identity Pauli produced a non-trivial syndrome"
                )
            }
        }
    }
}

impl std::error::Error for QecSelfCheckError {}

/* -------------------------------------------------------------------------- */
/* Test-suite registration                                                   */
/* -------------------------------------------------------------------------- */

/// The QEC test suite is kept under `tests/mod.rs`.
///
/// Keeping the registry here guarantees that the focused mathematical,
/// resource, determinism, security and regression suites are compiled as
/// part of the QEC module rather than merely existing as repository files.
#[cfg(test)]
mod tests;

/* -------------------------------------------------------------------------- */
/* Root-module tests                                                         */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn api_identity_is_stable() {
        assert_eq!(
            QEC_SUBSYSTEM_NAME,
            "zamani.quantum.error_correction"
        );

        assert_eq!(
            QEC_ARCHITECTURE,
            "resource-safe-scalable-validated-qec"
        );

        assert!(!QEC_API_VERSION.is_empty());
    }

    #[test]
    fn distance_module_is_registered() {
        let _ = compute_distance;
        let _ = distance;
        let _ = validate_distance;
    }

    #[test]
    fn canonical_error_boundary_is_exposed() {
        fn accepts_qec_result(
            _: QecResult<()>,
        ) {
        }

        let error = QecError::InvalidInput {
            message: "test".to_owned(),
        };

        accepts_qec_result(Err(error));
    }

    #[test]
    fn all_execution_environments_are_supported_by_inventory() {
        let environments =
            supported_execution_environments();

        assert_eq!(environments.len(), 6);

        for environment in environments {
            assert!(
                capabilities()
                    .supports_execution(*environment),
                "execution environment {:?} is not represented \
                 by the capability inventory",
                environment
            );
        }
    }

    #[test]
    fn qpu_is_explicit_and_not_classical() {
        assert!(ExecutionEnvironment::Qpu.is_qpu());
        assert!(!ExecutionEnvironment::Qpu.is_classical());
        assert!(!ExecutionEnvironment::Qpu.is_distributed());
    }

    #[test]
    fn qpu_access_fails_closed() {
        assert!(!QpuAccess::Denied.is_authorized());
        assert!(
            !QpuAccess::RequiresCapability.is_authorized()
        );
        assert!(
            QpuAccess::Authorized.is_authorized()
        );
    }

    #[test]
    fn identity_syndrome_invariant_holds() {
        assert!(self_check().is_ok());
    }
}