//! Zamani Quantum Error Correction (QEC) subsystem.
//!
//! Production-grade fault-tolerance infrastructure for:
//!
//! - stabilizer and Pauli algebra;
//! - syndrome generation and streaming;
//! - decoding graphs;
//! - MWPM and Union-Find decoding;
//! - logical-error classification;
//! - surface codes;
//! - configurable noise models;
//! - Pauli-frame tracking;
//! - simulation and threshold experiments;
//! - resource-aware execution;
//! - sparse and streaming representations;
//! - deterministic execution;
//! - cancellation and checkpointing;
//! - partitioned and distributed decoding;
//! - CPU, parallel CPU, GPU, accelerator and QPU backends;
//! - capability-based authorization;
//! - configuration/version compatibility;
//! - telemetry, metrics and observability;
//! - security, fuzzing, property and regression testing.
//!
//! ## Architectural boundary
//!
//! ```text
//!                 UNTRUSTED INPUT
//!                       |
//!                       v
//!                +--------------+
//!                | Configuration|
//!                +------+-------+
//!                       |
//!                       v
//!                +--------------+
//!                |  Validation  |
//!                +------+-------+
//!                       |
//!                       v
//!              +-------------------+
//!              | Resource Manager  |
//!              +---------+---------+
//!                        |
//!                        v
//!              +-------------------+
//!              | Syndrome Stream   |
//!              +---------+---------+
//!                        |
//!                        v
//!              +-------------------+
//!              | Detection Events  |
//!              +---------+---------+
//!                        |
//!                        v
//!              +-------------------+
//!              | Sparse Graph      |
//!              +---------+---------+
//!                        |
//!              +---------+---------+
//!              |                   |
//!              v                   v
//!             MWPM             Union-Find
//!              |                   |
//!              +---------+---------+
//!                        |
//!                        v
//!                 Pauli Frame
//!                        |
//!                        v
//!                Logical Outcome
//! ```
//!
//! ## Execution architecture
//!
//! QEC algorithms are independent of the physical execution environment.
//!
//! ```text
//!                         QEC Algorithm
//!                              |
//!              +---------------+---------------+
//!              |               |               |
//!              v               v               v
//!             CPU            GPU             QPU
//!              |               |               |
//!       Parallel CPU     Accelerator      Quantum Device
//!              |               |               |
//!              +---------------+---------------+
//!                              |
//!                              v
//!                    Distributed Execution
//! ```
//!
//! QPU access is capability-controlled. Merely running a decoder does not
//! grant access to a physical quantum processor.
//!
//! ## Resource model
//!
//! Zamani does not promise literally infinite memory or execution time.
//!
//! Instead, arbitrarily large QEC workloads are supported through:
//!
//! - bounded resource policies;
//! - sparse representations;
//! - streaming;
//! - partitioning;
//! - distributed execution;
//! - checkpointing;
//! - cancellation;
//! - deterministic failure when configured limits are exceeded.
//!
//! ## Security boundary
//!
//! ```text
//! External / Untrusted QEC Data
//!             |
//!             v
//!       Configuration
//!             |
//!             v
//!        Validation
//!             |
//!             v
//!       Capability Check
//!             |
//!             v
//!       Resource Check
//!             |
//!             v
//!       QEC Execution
//! ```
//!
//! No QEC module should silently bypass validation, resource accounting,
//! capability checks, or cancellation policies.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// -----------------------------------------------------------------------------
// Core QEC algorithms and mathematical models
// -----------------------------------------------------------------------------

pub mod decoder;
pub mod decoding_graph;
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

// -----------------------------------------------------------------------------
// Production infrastructure
// -----------------------------------------------------------------------------

pub mod validation;
pub mod limits;
pub mod errors;
pub mod resources;
pub mod metrics;
pub mod telemetry;
pub mod cancellation;
pub mod deterministic;
pub mod checkpoint;
pub mod streaming;
pub mod partition;
pub mod distributed;
pub mod scheduler;
pub mod memory;
pub mod arithmetic;
pub mod sparse;
pub mod cache;

// -----------------------------------------------------------------------------
// Execution, security and compatibility
// -----------------------------------------------------------------------------

pub mod backend;
pub mod capabilities;
pub mod configuration;
pub mod version;

// -----------------------------------------------------------------------------
// Stable high-level API
// -----------------------------------------------------------------------------

pub use decoder::{
    validate_correction,
    validate_correction_for_syndrome,
    validate_syndrome,
    single_qubit_error,
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

// -----------------------------------------------------------------------------
// Subsystem metadata
// -----------------------------------------------------------------------------

/// Public QEC subsystem API version.
///
/// This is intentionally separate from the overall Zamani project version.
pub const QEC_API_VERSION: &str = "2.0.0";

/// QEC subsystem name.
pub const QEC_SUBSYSTEM_NAME: &str = "zamani.quantum.error_correction";

/// QEC architecture identifier.
pub const QEC_ARCHITECTURE: &str = "resource-safe-scalable-qec";

/// Returns the QEC subsystem API version.
pub const fn api_version() -> &'static str {
    QEC_API_VERSION
}

// -----------------------------------------------------------------------------
// Execution model
// -----------------------------------------------------------------------------

/// Execution environments supported by the QEC subsystem.
///
/// A QEC algorithm must not assume that execution occurs on a CPU.
/// QPU access is explicitly represented and capability controlled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionEnvironment {
    /// Single-threaded CPU execution.
    Cpu,

    /// Multi-threaded CPU execution.
    ParallelCpu,

    /// GPU execution.
    Gpu,

    /// Generic hardware accelerator.
    Accelerator,

    /// Local quantum processing unit.
    Qpu,

    /// Multiple execution resources coordinated by Zamani.
    Distributed,
}

impl ExecutionEnvironment {
    /// Returns `true` when this environment represents quantum hardware.
    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns `true` when this environment is capable of classical
    /// computation.
    pub const fn is_classical(self) -> bool {
        matches!(
            self,
            Self::Cpu
                | Self::ParallelCpu
                | Self::Gpu
                | Self::Accelerator
        )
    }

    /// Returns `true` when execution may involve multiple workers/devices.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }
}

// -----------------------------------------------------------------------------
// QEC capabilities
// -----------------------------------------------------------------------------

/// High-level capabilities compiled into this QEC subsystem.
///
/// These describe subsystem functionality, not authorization.
///
/// Authorization must be handled by [`capabilities`] and the active
/// capability policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QecCapabilities {
    // Mathematical capabilities.
    pub stabilizer_algebra: bool,
    pub syndrome_generation: bool,
    pub decoding_graph: bool,
    pub mwpm: bool,
    pub union_find: bool,
    pub noise_models: bool,
    pub pauli_frame: bool,
    pub logical_operators: bool,
    pub simulation: bool,
    pub surface_code: bool,
    pub decoder_interface: bool,

    // Production infrastructure.
    pub validation: bool,
    pub resource_limits: bool,
    pub resource_accounting: bool,
    pub metrics: bool,
    pub telemetry: bool,
    pub cancellation: bool,
    pub deterministic_execution: bool,
    pub checkpointing: bool,
    pub streaming: bool,
    pub partitioning: bool,
    pub distributed_execution: bool,
    pub scheduling: bool,
    pub memory_management: bool,
    pub safe_arithmetic: bool,
    pub sparse_data: bool,
    pub caching: bool,

    // Execution.
    pub cpu_backend: bool,
    pub parallel_cpu_backend: bool,
    pub gpu_backend: bool,
    pub accelerator_backend: bool,
    pub qpu_backend: bool,

    // Security / compatibility.
    pub capability_security: bool,
    pub configuration_management: bool,
    pub versioning: bool,
}

impl QecCapabilities {
    /// Capabilities provided by the current QEC implementation.
    pub const CURRENT: Self = Self {
        stabilizer_algebra: true,
        syndrome_generation: true,
        decoding_graph: true,
        mwpm: true,
        union_find: true,
        noise_models: true,
        pauli_frame: true,
        logical_operators: true,
        simulation: true,
        surface_code: true,
        decoder_interface: true,

        validation: true,
        resource_limits: true,
        resource_accounting: true,
        metrics: true,
        telemetry: true,
        cancellation: true,
        deterministic_execution: true,
        checkpointing: true,
        streaming: true,
        partitioning: true,
        distributed_execution: true,
        scheduling: true,
        memory_management: true,
        safe_arithmetic: true,
        sparse_data: true,
        caching: true,

        cpu_backend: true,
        parallel_cpu_backend: true,
        gpu_backend: true,
        accelerator_backend: true,
        qpu_backend: true,

        capability_security: true,
        configuration_management: true,
        versioning: true,
    };

    /// Returns whether a particular execution environment is represented.
    pub const fn supports_execution(
        self,
        environment: ExecutionEnvironment,
    ) -> bool {
        match environment {
            ExecutionEnvironment::Cpu => self.cpu_backend,
            ExecutionEnvironment::ParallelCpu => self.parallel_cpu_backend,
            ExecutionEnvironment::Gpu => self.gpu_backend,
            ExecutionEnvironment::Accelerator => self.accelerator_backend,
            ExecutionEnvironment::Qpu => self.qpu_backend,
            ExecutionEnvironment::Distributed => {
                self.distributed_execution
            }
        }
    }
}

/// Returns the capabilities exposed by this QEC subsystem.
pub const fn capabilities() -> QecCapabilities {
    QecCapabilities::CURRENT
}

// -----------------------------------------------------------------------------
// QPU safety boundary
// -----------------------------------------------------------------------------

/// QPU execution safety state.
///
/// This is deliberately a small, dependency-free boundary type. Actual QPU
/// authorization, device discovery and transport belong in [`backend`] and
/// [`capabilities`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QpuAccess {
    /// No QPU access is available.
    Denied,

    /// QPU access exists but requires capability authorization.
    RequiresCapability,

    /// QPU access has been authorized.
    Authorized,
}

impl QpuAccess {
    /// Returns whether QPU operations may proceed.
    pub const fn is_authorized(self) -> bool {
        matches!(self, Self::Authorized)
    }
}

// -----------------------------------------------------------------------------
// Structural health check
// -----------------------------------------------------------------------------

/// Performs a lightweight deterministic QEC self-check.
///
/// The check intentionally does not:
///
/// - access a QPU;
/// - access a GPU;
/// - allocate unbounded memory;
/// - perform distributed execution;
/// - execute a decoder;
/// - access the network;
/// - modify persistent state.
///
/// It verifies the fundamental identity-syndrome invariant:
///
/// ```text
/// identity Pauli
///      ↓
/// stabilizer syndrome
///      ↓
/// trivial syndrome
/// ```
pub fn self_check() -> Result<(), QecSelfCheckError> {
    let stabilizers =
        StabilizerGroup::new(1)
            .map_err(QecSelfCheckError::Stabilizer)?;

    stabilizers
        .validate()
        .map_err(QecSelfCheckError::Stabilizer)?;

    let identity = PauliString::identity(1);

    let syndrome =
        stabilizers
            .syndrome(&identity)
            .map_err(QecSelfCheckError::Stabilizer)?;

    if !syndrome.is_trivial() {
        return Err(
            QecSelfCheckError::InvalidIdentitySyndrome,
        );
    }

    Ok(())
}

/// Errors returned by [`self_check`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QecSelfCheckError {
    Stabilizer(StabilizerError),
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

// -----------------------------------------------------------------------------
// Compile-time architecture checks
// -----------------------------------------------------------------------------

/// Returns the execution environments supported by the compiled subsystem.
pub const fn supported_execution_environments()
    -> &'static [ExecutionEnvironment]
{
    &[
        ExecutionEnvironment::Cpu,
        ExecutionEnvironment::ParallelCpu,
        ExecutionEnvironment::Gpu,
        ExecutionEnvironment::Accelerator,
        ExecutionEnvironment::Qpu,
        ExecutionEnvironment::Distributed,
    ]
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn api_version_is_present() {
        assert!(!QEC_API_VERSION.is_empty());
    }

    #[test]
    fn subsystem_name_is_present() {
        assert!(!QEC_SUBSYSTEM_NAME.is_empty());
    }

    #[test]
    fn capabilities_are_enabled() {
        let caps = capabilities();

        assert!(caps.stabilizer_algebra);
        assert!(caps.syndrome_generation);
        assert!(caps.decoding_graph);
        assert!(caps.mwpm);
        assert!(caps.union_find);
        assert!(caps.noise_models);
        assert!(caps.pauli_frame);
        assert!(caps.logical_operators);
        assert!(caps.simulation);
        assert!(caps.surface_code);
        assert!(caps.decoder_interface);

        assert!(caps.validation);
        assert!(caps.resource_limits);
        assert!(caps.resource_accounting);
        assert!(caps.metrics);
        assert!(caps.telemetry);
        assert!(caps.cancellation);
        assert!(caps.deterministic_execution);
        assert!(caps.checkpointing);
        assert!(caps.streaming);
        assert!(caps.partitioning);
        assert!(caps.distributed_execution);
        assert!(caps.scheduling);
        assert!(caps.memory_management);
        assert!(caps.safe_arithmetic);
        assert!(caps.sparse_data);
        assert!(caps.caching);

        assert!(caps.cpu_backend);
        assert!(caps.parallel_cpu_backend);
        assert!(caps.gpu_backend);
        assert!(caps.accelerator_backend);
        assert!(caps.qpu_backend);

        assert!(caps.capability_security);
        assert!(caps.configuration_management);
        assert!(caps.versioning);
    }

    #[test]
    fn all_execution_environments_are_declared() {
        let environments =
            supported_execution_environments();

        assert_eq!(environments.len(), 6);

        assert!(
            environments.contains(
                &ExecutionEnvironment::Cpu
            )
        );

        assert!(
            environments.contains(
                &ExecutionEnvironment::ParallelCpu
            )
        );

        assert!(
            environments.contains(
                &ExecutionEnvironment::Gpu
            )
        );

        assert!(
            environments.contains(
                &ExecutionEnvironment::Accelerator
            )
        );

        assert!(
            environments.contains(
                &ExecutionEnvironment::Qpu
            )
        );

        assert!(
            environments.contains(
                &ExecutionEnvironment::Distributed
            )
        );
    }

    #[test]
    fn qpu_is_distinguished_from_classical_execution() {
        assert!(
            ExecutionEnvironment::Qpu.is_qpu()
        );

        assert!(
            !ExecutionEnvironment::Qpu.is_classical()
        );

        assert!(
            !ExecutionEnvironment::Cpu.is_qpu()
        );

        assert!(
            ExecutionEnvironment::Cpu.is_classical()
        );
    }

    #[test]
    fn qpu_requires_explicit_authorization() {
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
    fn self_check_passes() {
        assert!(
            self_check().is_ok()
        );
    }

    #[test]
    fn identity_has_trivial_syndrome() {
        let group =
            StabilizerGroup::new(2)
                .expect("valid stabilizer group");

        let identity =
            PauliString::identity(2);

        let syndrome =
            group
                .syndrome(&identity)
                .expect("identity syndrome");

        assert!(
            syndrome.is_trivial()
        );
    }

    #[test]
    fn correction_uses_shared_pauli_model() {
        let correction =
            Correction::identity(3);

        assert_eq!(
            correction
                .operator()
                .num_qubits(),
            3
        );

        assert!(
            correction.is_identity()
        );
    }
}