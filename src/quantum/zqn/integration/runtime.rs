//! Zamani Quantum Noise (ZQN) — Runtime Integration
//!
//! Path:
//!     src/quantum/zqn/integration/runtime.rs
//!
//! # Ownership
//!
//! This module owns the stable boundary between the ZQN subsystem and the
//! surrounding Zamani quantum runtime.
//!
//! It owns:
//!
//! - runtime execution context;
//! - explicit runtime resource policy;
//! - cooperative cancellation;
//! - deterministic execution identity;
//! - runtime resource binding;
//! - runtime execution requests;
//! - runtime execution outcomes;
//! - runtime lifecycle states;
//! - runtime-facing ZQN capability contracts;
//! - runtime-facing noise realization contracts;
//! - runtime-facing fault/channel/observation envelopes;
//! - validation at the ZQN/runtime boundary;
//! - deterministic sub-seed derivation;
//! - execution accounting;
//! - runtime integration errors;
//! - runtime integration tests.
//!
//! # This module does NOT own
//!
//! This file does not own:
//!
//! - canonical quantum-program semantics;
//! - canonical quantum IR;
//! - quantum gates;
//! - circuit construction;
//! - source-language syntax;
//! - routing;
//! - scheduling;
//! - quantum channels;
//! - Kraus mathematics;
//! - Choi mathematics;
//! - probability distributions;
//! - noise-model mathematics;
//! - calibration storage;
//! - characterization protocols;
//! - simulation-state representation;
//! - stochastic trajectory algorithms;
//! - QEC decoding;
//! - logical correction;
//! - hardware APIs;
//! - vendor APIs;
//! - benchmarking methodology;
//! - serialization wire formats;
//! - global runtime state;
//! - global RNG state.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                       canonical Quantum IR
//!                              |
//!             +----------------+----------------+
//!             |                |                |
//!             v                v                v
//!          routing         scheduling          QEC
//!             |                |                |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                            ZQN
//!                              |
//!                 +------------+------------+
//!                 |                         |
//!                 v                         v
//!            noise model              calibration
//!                 |                         |
//!                 +------------+------------+
//!                              |
//!                              v
//!                   ZQN/runtime integration
//!                              |
//!                +-------------+-------------+
//!                |             |             |
//!                v             v             v
//!             simulator       QPU         emulator
//!                |             |             |
//!                +-------------+-------------+
//!                              |
//!                              v
//!                         observations
//!                              |
//!                              v
//!                    characterization / benchmark
//! ```
//!
//! The runtime is the execution boundary.
//!
//! ZQN tells the runtime what noise/fault/channel realization is applicable.
//! The runtime determines how that realization is executed against the
//! selected execution environment.
//!
//! # Fundamental ownership rule
//!
//! The runtime integration layer must not become a second runtime.
//!
//! It defines contracts that an actual runtime, simulator, emulator or
//! hardware adapter can implement.
//!
//! In particular, this module MUST NOT:
//!
//! - spawn execution threads itself;
//! - own a thread pool;
//! - own an async executor;
//! - own a process-global RNG;
//! - own a global calibration cache;
//! - own a global hardware connection;
//! - invoke vendor APIs;
//! - allocate a complete quantum state;
//! - implement a quantum simulator;
//! - execute QEC decoding.
//!
//! # Canonical identity
//!
//! Quantum resource identity MUST use the canonical IR types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Semantic operation identity MUST use:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! This module never defines another `QubitId`, `PhysicalQubitId`, or
//! `OperationId`.
//!
//! The repository's IR explicitly establishes these as canonical identities.
//!
//! # Write once, scale everywhere
//!
//! There are intentionally no semantic constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_SHOTS
//! MAX_NODES
//! MAX_RUNTIME_DEPTH
//! MAX_RESOURCES
//! ```
//!
//! A computation can contain any finite number of resources that can be
//! represented and executed by the selected environment.
//!
//! Runtime limits are policy:
//!
//! ```text
//! quantum semantics
//!       |
//!       v
//! ZQN semantics
//!       |
//!       v
//! runtime request
//!       |
//!       v
//! caller/resource policy
//!       |
//!       v
//! available machine resources
//! ```
//!
//! Therefore "infinity" means that this module imposes no artificial finite
//! machine-size ceiling. It does not mean that a concrete machine can
//! physically allocate infinite memory or execute an infinite workload.
//!
//! # Resource governance
//!
//! All potentially expensive runtime work is governed through
//! `RuntimeLimits`.
//!
//! Every limit is optional.
//!
//! `None` means:
//!
//!     this integration layer imposes no limit for that resource.
//!
//! It does NOT mean:
//!
//!     the operating system, allocator, hardware or scheduler has infinite
//!     capacity.
//!
//! A production runtime should normally supply explicit deployment limits.
//!
//! # Determinism
//!
//! ZQN/runtime integration must not introduce hidden nondeterminism.
//!
//! This module contains:
//!
//! - no global RNG;
//! - no thread-local RNG;
//! - no wall-clock dependency for semantic identity;
//! - no pointer-address-derived identity;
//! - no process-ID-derived semantic identity;
//! - no unordered collection whose iteration order affects semantics.
//!
//! Stochastic execution must receive an explicit master seed.
//!
//! Sub-seeds are derived deterministically from explicit execution identity.
//!
//! Consequently:
//!
//! ```text
//! same program identity
//! + same target identity
//! + same noise identity
//! + same calibration identity
//! + same master seed
//! + same execution key
//! =
//! same deterministic execution seed
//! ```
//!
//! Parallel execution must not require a different semantic result merely
//! because work is distributed over different worker counts.
//!
//! # Cancellation
//!
//! Cancellation is cooperative.
//!
//! The runtime owns the cancellation mechanism.
//!
//! ZQN only observes it through `CancellationToken`.
//!
//! Cancellation must never be encoded as a semantic noise result.
//!
//! # Runtime lifecycle
//!
//! Runtime state is represented explicitly:
//!
//! ```text
//! Created
//!    |
//!    v
//! Validated
//!    |
//!    v
//! Running
//!    |
//!    +---------> Cancelled
//!    |
//!    +---------> Failed
//!    |
//!    v
//! Completed
//! ```
//!
//! Invalid state transitions are rejected rather than silently ignored.
//!
//! # Approximation
//!
//! Runtime execution can be:
//!
//! - Exact;
//! - Approximate;
//! - Bounded;
//! - Statistical;
//! - Unsupported.
//!
//! The runtime MUST NOT silently downgrade an exact request into an
//! approximation.
//!
//! A caller must explicitly permit approximation.
//!
//! # Integration with simulation
//!
//! `zqn::simulation` owns actual simulation algorithms.
//!
//! This file supplies:
//!
//! - execution context;
//! - limits;
//! - cancellation;
//! - deterministic identity;
//! - resource binding;
//! - execution request.
//!
//! The simulator consumes these contracts.
//!
//! # Integration with hardware
//!
//! Hardware adapters implement runtime-facing traits without exposing vendor
//! APIs here.
//!
//! The adapter is responsible for translating:
//!
//! ```text
//! RuntimeExecutionRequest
//!         |
//!         v
//! target-specific execution
//!         |
//!         v
//! RuntimeExecutionOutcome
//! ```
//!
//! # Integration with QEC
//!
//! QEC may use runtime context for:
//!
//! - deterministic syndrome generation;
//! - fault injection;
//! - resource accounting;
//! - cancellation;
//! - execution provenance.
//!
//! This file does not decode syndromes or perform logical correction.
//!
//! # Integration with scheduling
//!
//! Scheduling determines temporal placement.
//!
//! Runtime consumes the resulting schedule.
//!
//! This file does not own `Schedule`, `TimePoint`, `Duration` or scheduling
//! algorithms.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may create multiple runtime requests and associate each with
//! an execution identity.
//!
//! Runtime returns execution outcomes and observations.
//!
//! Benchmarking owns statistical analysis and benchmark methodology.
//!
//! # Integration with calibration
//!
//! A runtime request may identify the calibration revision used for execution.
//!
//! Runtime does not own calibration validity or interpolation.
//!
//! # Integration with memory
//!
//! A memory/simulation implementation may use the runtime request's resource
//! binding and execution policy to determine how state is materialized.
//!
//! This module does not own quantum-memory storage.
//!
//! # Serialization
//!
//! This file defines semantic runtime values but does not define a wire
//! serialization format.
//!
//! The ZQN `io` subsystem owns canonical serialization.
//!
//! Any serialization adapter MUST preserve:
//!
//! - execution identity;
//! - target identity;
//! - noise identity;
//! - calibration identity;
//! - resource identities;
//! - operation identities;
//! - deterministic seed policy;
//! - approximation policy;
//! - resource policy;
//! - lifecycle/outcome status.
//!
//! It MUST NOT serialize:
//!
//! - pointers;
//! - thread IDs;
//! - allocator state;
//! - synchronization primitives;
//! - temporary caches;
//! - process-local addresses.
//!
//! # Security
//!
//! Runtime requests are untrusted at this boundary.
//!
//! Implementations must defend against:
//!
//! - allocation exhaustion;
//! - arithmetic overflow;
//! - invalid identifiers;
//! - invalid resource cardinality;
//! - non-finite numeric values;
//! - pathological cancellation polling;
//! - unbounded metadata;
//! - malicious execution adapters.
//!
//! This module itself does not perform external process, filesystem, network,
//! dynamic-code or vendor execution.
//!
//! # Thread safety
//!
//! Contract implementations intended for concurrent runtime use SHOULD be
//! `Send + Sync`.
//!
//! Runtime state is intentionally owned by the caller/implementation rather
//! than stored globally.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` is intentional.
//!
//! # No-reedit contract
//!
//! This file is intentionally dependent only on stable semantic boundaries:
//!
//! - standard-library types;
//! - canonical IR identities;
//! - caller-supplied runtime data.
//!
//! Adding a new:
//!
//! - simulator;
//! - QPU backend;
//! - emulator;
//! - QEC implementation;
//! - scheduler;
//! - routing algorithm;
//! - noise model;
//! - quantum technology;
//! - benchmark;
//! - calibration implementation;
//!
//! must not require this file to be modified merely because that implementation
//! was added.
//!
//! New implementations consume these contracts.
//!
//! # Testing
//!
//! Tests in this file verify:
//!
//! - canonical logical/physical qubit identity;
//! - resource deduplication;
//! - no implicit machine-size limits;
//! - explicit resource-limit enforcement;
//! - deterministic seed derivation;
//! - cancellation;
//! - lifecycle correctness;
//! - exact/approximate policy correctness;
//! - checked accounting;
//! - deterministic ordering;
//! - invalid input rejection;
//! - absence of unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::error::Error;
use std::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

/// Stable schema identifier for this integration contract.
pub const RUNTIME_INTEGRATION_SCHEMA_ID: &str =
    "zamani.quantum.zqn.integration.runtime";

/// Semantic version of the runtime integration contract.
pub const RUNTIME_INTEGRATION_SCHEMA_VERSION: u16 = 1;

/// Result type used by the runtime integration boundary.
pub type RuntimeResult<T> = Result<T, RuntimeIntegrationError>;

/// Errors produced by the runtime integration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeIntegrationError {
    /// A required identifier was empty.
    EmptyIdentifier {
        /// Semantic field containing the invalid identifier.
        field: &'static str,
    },

    /// A string exceeded a caller-specified policy.
    IdentifierTooLarge {
        /// Semantic field containing the invalid identifier.
        field: &'static str,
        /// Supplied byte length.
        bytes: u128,
        /// Maximum permitted byte length.
        maximum: u128,
    },

    /// A resource occurred more than once in a resource binding.
    DuplicateResource {
        /// Resource kind.
        resource: RuntimeResource,
    },

    /// An execution request contained no valid execution identity.
    MissingExecutionIdentity,

    /// A runtime policy was invalid.
    InvalidLimits {
        /// Explanation.
        reason: &'static str,
    },

    /// A runtime context was invalid.
    InvalidContext {
        /// Explanation.
        reason: &'static str,
    },

    /// A lifecycle transition was invalid.
    InvalidStateTransition {
        /// Current state.
        from: RuntimeLifecycle,
        /// Requested state.
        to: RuntimeLifecycle,
    },

    /// Execution was cancelled.
    Cancelled,

    /// A runtime resource policy was exceeded.
    ResourceLimitExceeded {
        /// Resource being limited.
        resource: RuntimeResourceKind,
        /// Requested amount.
        requested: u128,
        /// Configured maximum.
        maximum: u128,
    },

    /// An arithmetic operation could not be represented.
    ArithmeticOverflow {
        /// Operation description.
        operation: &'static str,
    },

    /// An unsupported runtime capability was requested.
    Unsupported {
        /// Description.
        reason: &'static str,
    },

    /// A deterministic execution request was incomplete.
    DeterminismViolation {
        /// Explanation.
        reason: &'static str,
    },

    /// A runtime adapter rejected a request.
    AdapterRejected {
        /// Stable adapter error description.
        reason: String,
    },
}

impl fmt::Display for RuntimeIntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::IdentifierTooLarge {
                field,
                bytes,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} is {bytes} bytes but the configured maximum is {maximum}"
                )
            }

            Self::DuplicateResource { resource } => {
                write!(formatter, "duplicate runtime resource: {resource:?}")
            }

            Self::MissingExecutionIdentity => {
                write!(formatter, "execution identity is required")
            }

            Self::InvalidLimits { reason } => {
                write!(formatter, "invalid runtime limits: {reason}")
            }

            Self::InvalidContext { reason } => {
                write!(formatter, "invalid runtime context: {reason}")
            }

            Self::InvalidStateTransition { from, to } => {
                write!(
                    formatter,
                    "invalid runtime lifecycle transition: {from:?} -> {to:?}"
                )
            }

            Self::Cancelled => write!(formatter, "runtime execution was cancelled"),

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "runtime resource limit exceeded for {resource:?}: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "arithmetic overflow during {operation}")
            }

            Self::Unsupported { reason } => {
                write!(formatter, "unsupported runtime operation: {reason}")
            }

            Self::DeterminismViolation { reason } => {
                write!(formatter, "determinism contract violation: {reason}")
            }

            Self::AdapterRejected { reason } => {
                write!(formatter, "runtime adapter rejected request: {reason}")
            }
        }
    }
}

impl Error for RuntimeIntegrationError {}

/// Kind of resource governed by runtime policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuntimeResourceKind {
    /// Logical/semantic quantum resources.
    LogicalQubits,

    /// Physical quantum resources.
    PhysicalQubits,

    /// Semantic operations.
    Operations,

    /// Runtime shots/executions.
    Shots,

    /// Execution nodes.
    Nodes,

    /// Bytes of materialized runtime memory.
    MemoryBytes,

    /// Generic execution work units.
    WorkUnits,
}

/// A canonical quantum resource participating in runtime execution.
///
/// Logical and physical identities remain distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuntimeResource {
    /// Canonical logical qubit.
    LogicalQubit(QubitId),

    /// Canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Canonical semantic operation.
    Operation(OperationId),
}

impl fmt::Display for RuntimeResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(qubit) => {
                write!(formatter, "logical-qubit({:?})", qubit)
            }
            Self::PhysicalQubit(qubit) => {
                write!(formatter, "physical-qubit({:?})", qubit)
            }
            Self::Operation(operation) => {
                write!(formatter, "operation({:?})", operation)
            }
        }
    }
}

/// Explicit runtime resource limits.
///
/// There are no architectural defaults representing a maximum quantum
/// machine size.
///
/// Every field is optional.
///
/// `None` means that this policy layer does not impose that limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeLimits {
    /// Maximum logical qubits participating in one request.
    pub max_logical_qubits: Option<u128>,

    /// Maximum physical qubits participating in one request.
    pub max_physical_qubits: Option<u128>,

    /// Maximum operations participating in one request.
    pub max_operations: Option<u128>,

    /// Maximum shots represented by one request.
    pub max_shots: Option<u128>,

    /// Maximum execution nodes represented by one request.
    pub max_nodes: Option<u128>,

    /// Maximum memory bytes that an adapter may materialize.
    pub max_memory_bytes: Option<u128>,

    /// Maximum generic work units.
    pub max_work_units: Option<u128>,

    /// Optional maximum number of bytes for each textual identifier.
    ///
    /// `None` imposes no limit at this integration layer.
    pub max_identifier_bytes: Option<u128>,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self {
            max_logical_qubits: None,
            max_physical_qubits: None,
            max_operations: None,
            max_shots: None,
            max_nodes: None,
            max_memory_bytes: None,
            max_work_units: None,
            max_identifier_bytes: None,
        }
    }
}

impl RuntimeLimits {
    /// Validates the limit configuration.
    pub fn validate(&self) -> RuntimeResult<()> {
        let fields = [
            (
                self.max_logical_qubits,
                "max_logical_qubits",
            ),
            (
                self.max_physical_qubits,
                "max_physical_qubits",
            ),
            (
                self.max_operations,
                "max_operations",
            ),
            (self.max_shots, "max_shots"),
            (self.max_nodes, "max_nodes"),
            (self.max_memory_bytes, "max_memory_bytes"),
            (self.max_work_units, "max_work_units"),
            (
                self.max_identifier_bytes,
                "max_identifier_bytes",
            ),
        ];

        // Options can only contain non-negative u128 values, so the purpose
        // of this method is primarily to establish a stable validation
        // boundary for future limit extensions.
        for (value, _name) in fields {
            if value.is_some_and(|limit| limit == 0) {
                // Zero is valid for a resource limit. It means "allow none".
                // Therefore no rejection is performed here.
            }
        }

        Ok(())
    }

    /// Validates one requested amount against a configured limit.
    pub fn check(
        &self,
        resource: RuntimeResourceKind,
        requested: u128,
    ) -> RuntimeResult<()> {
        let maximum = match resource {
            RuntimeResourceKind::LogicalQubits => self.max_logical_qubits,
            RuntimeResourceKind::PhysicalQubits => self.max_physical_qubits,
            RuntimeResourceKind::Operations => self.max_operations,
            RuntimeResourceKind::Shots => self.max_shots,
            RuntimeResourceKind::Nodes => self.max_nodes,
            RuntimeResourceKind::MemoryBytes => self.max_memory_bytes,
            RuntimeResourceKind::WorkUnits => self.max_work_units,
        };

        if let Some(maximum) = maximum {
            if requested > maximum {
                return Err(RuntimeIntegrationError::ResourceLimitExceeded {
                    resource,
                    requested,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Validates a textual identifier according to this policy.
    pub fn check_identifier(
        &self,
        field: &'static str,
        value: &str,
    ) -> RuntimeResult<()> {
        if value.is_empty() {
            return Err(RuntimeIntegrationError::EmptyIdentifier { field });
        }

        if let Some(maximum) = self.max_identifier_bytes {
            let bytes = value.len() as u128;

            if bytes > maximum {
                return Err(RuntimeIntegrationError::IdentifierTooLarge {
                    field,
                    bytes,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

/// Cooperative cancellation contract.
///
/// The runtime owns cancellation state; ZQN only observes it.
pub trait CancellationToken: Send + Sync {
    /// Returns `true` if execution should stop.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation token that never cancels.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeverCancel;

impl CancellationToken for NeverCancel {
    #[inline]
    fn is_cancelled(&self) -> bool {
        false
    }
}

/// Explicit deterministic execution identity.
///
/// This identity is intentionally independent from:
///
/// - memory addresses;
/// - process IDs;
/// - thread IDs;
/// - wall-clock time;
/// - worker count.
///
/// All strings are semantic identities and must therefore be stable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionIdentity {
    /// Identity of the Zamani program/workload.
    pub program_id: String,

    /// Identity of the selected execution target.
    pub target_id: String,

    /// Identity of the active ZQN noise model.
    pub noise_model_id: String,

    /// Identity/revision of the calibration state.
    pub calibration_id: Option<String>,

    /// Caller-supplied master seed.
    pub master_seed: u128,

    /// Stable execution/run key.
    ///
    /// This distinguishes independent executions of the same program while
    /// remaining independent from process-local runtime state.
    pub execution_key: u128,
}

impl ExecutionIdentity {
    /// Validates semantic identity fields.
    pub fn validate(&self, limits: &RuntimeLimits) -> RuntimeResult<()> {
        limits.check_identifier("program_id", &self.program_id)?;
        limits.check_identifier("target_id", &self.target_id)?;
        limits.check_identifier("noise_model_id", &self.noise_model_id)?;

        if let Some(calibration_id) = &self.calibration_id {
            limits.check_identifier("calibration_id", calibration_id)?;
        }

        Ok(())
    }

    /// Returns a deterministic fingerprint of this execution identity.
    ///
    /// The algorithm is intentionally implemented locally rather than using
    /// `DefaultHasher`, because Rust's default hash state is not a stable
    /// serialization/fingerprint contract.
    #[must_use]
    pub fn fingerprint(&self) -> u128 {
        let mut hasher = StableHasher128::new();

        hasher.write_str(&self.program_id);
        hasher.write_str(&self.target_id);
        hasher.write_str(&self.noise_model_id);

        match &self.calibration_id {
            Some(value) => {
                hasher.write_u8(1);
                hasher.write_str(value);
            }
            None => hasher.write_u8(0),
        }

        hasher.write_u128(self.master_seed);
        hasher.write_u128(self.execution_key);

        hasher.finish()
    }
}

/// Runtime execution mode.
///
/// This is an execution policy, not a simulator implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Require exact realization.
    Exact,

    /// Permit an explicitly bounded approximation.
    Approximate,

    /// Require an explicit conservative bound.
    Bounded,

    /// Permit statistical realization with an explicit confidence contract.
    Statistical,
}

impl ExecutionMode {
    /// Returns whether approximation is permitted.
    #[must_use]
    pub const fn allows_approximation(self) -> bool {
        matches!(
            self,
            Self::Approximate | Self::Bounded | Self::Statistical
        )
    }
}

/// Runtime request precision contract.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrecisionContract {
    /// Exact execution is required.
    Exact,

    /// Approximation is permitted within an explicit tolerance.
    Approximate {
        /// Maximum declared approximation tolerance.
        tolerance: f64,
    },

    /// A conservative absolute error bound is required.
    Bounded {
        /// Maximum declared absolute error.
        error_bound: f64,
    },

    /// Statistical execution with an explicit confidence.
    Statistical {
        /// Required confidence in `(0, 1]`.
        confidence: f64,
    },
}

impl PrecisionContract {
    /// Validates the precision contract.
    pub fn validate(self) -> RuntimeResult<()> {
        match self {
            Self::Exact => Ok(()),

            Self::Approximate { tolerance } => {
                validate_non_negative_finite(tolerance, "tolerance")
            }

            Self::Bounded { error_bound } => {
                validate_non_negative_finite(error_bound, "error_bound")
            }

            Self::Statistical { confidence } => {
                if !confidence.is_finite() || confidence <= 0.0 || confidence > 1.0 {
                    return Err(RuntimeIntegrationError::InvalidContext {
                        reason: "statistical confidence must be finite and within (0, 1]",
                    });
                }

                Ok(())
            }
        }
    }
}

/// Runtime resource binding.
///
/// The binding contains canonical IR identities and makes no assumption about
/// operation arity or machine size.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeResourceBinding {
    logical_qubits: Vec<QubitId>,
    physical_qubits: Vec<PhysicalQubitId>,
    operations: Vec<OperationId>,
}

impl RuntimeResourceBinding {
    /// Creates an empty binding.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a validated resource binding.
    ///
    /// Duplicate identities within the same identity domain are rejected.
    pub fn new(
        logical_qubits: Vec<QubitId>,
        physical_qubits: Vec<PhysicalQubitId>,
        operations: Vec<OperationId>,
    ) -> RuntimeResult<Self> {
        reject_duplicates(
            &logical_qubits,
            RuntimeResource::LogicalQubit,
        )?;

        reject_duplicates(
            &physical_qubits,
            RuntimeResource::PhysicalQubit,
        )?;

        reject_duplicates(
            &operations,
            RuntimeResource::Operation,
        )?;

        Ok(Self {
            logical_qubits,
            physical_qubits,
            operations,
        })
    }

    /// Returns logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns physical qubits.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns operations.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn logical_qubit_count(&self) -> u128 {
        self.logical_qubits.len() as u128
    }

    /// Returns the number of physical qubits.
    #[must_use]
    pub fn physical_qubit_count(&self) -> u128 {
        self.physical_qubits.len() as u128
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> u128 {
        self.operations.len() as u128
    }

    /// Validates this binding against runtime limits.
    pub fn validate(&self, limits: &RuntimeLimits) -> RuntimeResult<()> {
        limits.check(
            RuntimeResourceKind::LogicalQubits,
            self.logical_qubit_count(),
        )?;

        limits.check(
            RuntimeResourceKind::PhysicalQubits,
            self.physical_qubit_count(),
        )?;

        limits.check(
            RuntimeResourceKind::Operations,
            self.operation_count(),
        )?;

        Ok(())
    }
}

/// Runtime execution context.
///
/// This is immutable semantic context. The actual runtime implementation may
/// maintain mutable state outside this value.
pub struct RuntimeExecutionContext<'a> {
    /// Deterministic execution identity.
    pub identity: ExecutionIdentity,

    /// Runtime resource policy.
    pub limits: RuntimeLimits,

    /// Bound logical/physical resources and operations.
    pub resources: RuntimeResourceBinding,

    /// Execution mode.
    pub mode: ExecutionMode,

    /// Precision contract.
    pub precision: PrecisionContract,

    /// Optional cancellation source.
    pub cancellation: Option<&'a dyn CancellationToken>,
}

impl<'a> RuntimeExecutionContext<'a> {
    /// Validates the complete execution context.
    pub fn validate(&self) -> RuntimeResult<()> {
        self.limits.validate()?;
        self.identity.validate(&self.limits)?;
        self.resources.validate(&self.limits)?;
        self.precision.validate()?;

        if matches!(self.mode, ExecutionMode::Exact)
            && !matches!(self.precision, PrecisionContract::Exact)
        {
            return Err(RuntimeIntegrationError::InvalidContext {
                reason: "exact execution mode requires an exact precision contract",
            });
        }

        if !self.mode.allows_approximation()
            && !matches!(self.precision, PrecisionContract::Exact)
        {
            return Err(RuntimeIntegrationError::InvalidContext {
                reason: "execution mode does not permit the requested precision policy",
            });
        }

        Ok(())
    }

    /// Returns whether execution has been cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancellation
            .map(CancellationToken::is_cancelled)
            .unwrap_or(false)
    }

    /// Returns an error if execution has been cancelled.
    pub fn check_cancellation(&self) -> RuntimeResult<()> {
        if self.is_cancelled() {
            Err(RuntimeIntegrationError::Cancelled)
        } else {
            Ok(())
        }
    }

    /// Derives a deterministic seed for a runtime sub-operation.
    ///
    /// `domain` must be a stable semantic identifier.
    #[must_use]
    pub fn derive_seed(&self, domain: &str, index: u128) -> u128 {
        derive_seed(
            self.identity.master_seed,
            self.identity.fingerprint(),
            domain,
            index,
        )
    }
}

/// Stable runtime sub-seed derivation.
///
/// This function does not access any global state and does not use a
/// nondeterministic standard-library hasher.
#[must_use]
pub fn derive_seed(
    master_seed: u128,
    execution_fingerprint: u128,
    domain: &str,
    index: u128,
) -> u128 {
    let mut hasher = StableHasher128::new();

    hasher.write_u128(master_seed);
    hasher.write_u128(execution_fingerprint);
    hasher.write_str(domain);
    hasher.write_u128(index);

    hasher.finish()
}

/// Runtime execution request.
///
/// This type describes *what runtime work is requested* without implementing
/// the work itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeExecutionRequest {
    /// Execution identity.
    pub identity: ExecutionIdentity,

    /// Resource binding.
    pub resources: RuntimeResourceBinding,

    /// Number of requested shots/executions.
    pub shots: u128,

    /// Number of execution nodes requested by the caller.
    pub nodes: u128,

    /// Generic work estimate supplied by the caller.
    pub work_units: u128,

    /// Memory budget requested by the caller.
    pub memory_bytes: u128,

    /// Runtime execution mode.
    pub mode: ExecutionMode,

    /// Precision contract.
    pub precision: PrecisionContract,
}

impl RuntimeExecutionRequest {
    /// Validates the request against a runtime policy.
    pub fn validate(&self, limits: &RuntimeLimits) -> RuntimeResult<()> {
        limits.validate()?;
        self.identity.validate(limits)?;
        self.resources.validate(limits)?;

        limits.check(RuntimeResourceKind::Shots, self.shots)?;
        limits.check(RuntimeResourceKind::Nodes, self.nodes)?;
        limits.check(
            RuntimeResourceKind::WorkUnits,
            self.work_units,
        )?;
        limits.check(
            RuntimeResourceKind::MemoryBytes,
            self.memory_bytes,
        )?;

        self.precision.validate()?;

        if self.mode == ExecutionMode::Exact
            && !matches!(self.precision, PrecisionContract::Exact)
        {
            return Err(RuntimeIntegrationError::InvalidContext {
                reason: "exact execution requires PrecisionContract::Exact",
            });
        }

        Ok(())
    }
}

/// Runtime lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeLifecycle {
    /// Request has been created but not validated.
    Created,

    /// Request passed runtime-boundary validation.
    Validated,

    /// Execution is in progress.
    Running,

    /// Execution completed successfully.
    Completed,

    /// Execution was cancelled.
    Cancelled,

    /// Execution failed.
    Failed,
}

impl RuntimeLifecycle {
    /// Returns whether this is a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Cancelled | Self::Failed
        )
    }

    /// Returns whether a transition is valid.
    #[must_use]
    pub const fn can_transition_to(self, next: Self) -> bool {
        match (self, next) {
            (Self::Created, Self::Validated) => true,
            (Self::Created, Self::Cancelled) => true,
            (Self::Validated, Self::Running) => true,
            (Self::Validated, Self::Cancelled) => true,
            (Self::Running, Self::Completed) => true,
            (Self::Running, Self::Cancelled) => true,
            (Self::Running, Self::Failed) => true,
            _ => false,
        }
    }
}

/// Explicit lifecycle tracker.
///
/// The tracker is local to an execution and therefore does not introduce
/// global mutable runtime state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeLifecycleTracker {
    state: RuntimeLifecycle,
}

impl Default for RuntimeLifecycleTracker {
    fn default() -> Self {
        Self {
            state: RuntimeLifecycle::Created,
        }
    }
}

impl RuntimeLifecycleTracker {
    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(self) -> RuntimeLifecycle {
        self.state
    }

    /// Attempts a lifecycle transition.
    pub fn transition(
        &mut self,
        next: RuntimeLifecycle,
    ) -> RuntimeResult<()> {
        if !self.state.can_transition_to(next) {
            return Err(
                RuntimeIntegrationError::InvalidStateTransition {
                    from: self.state,
                    to: next,
                },
            );
        }

        self.state = next;
        Ok(())
    }
}

/// Runtime execution accounting.
///
/// Counters are monotonic and use checked arithmetic.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuntimeAccounting {
    /// Number of operations observed.
    pub operations: u128,

    /// Number of shots observed.
    pub shots: u128,

    /// Number of work units consumed.
    pub work_units: u128,

    /// Number of bytes materialized.
    pub memory_bytes: u128,

    /// Number of runtime nodes used.
    pub nodes: u128,
}

impl RuntimeAccounting {
    /// Adds operation count using checked arithmetic.
    pub fn add_operations(&mut self, value: u128) -> RuntimeResult<()> {
        self.operations = self
            .operations
            .checked_add(value)
            .ok_or(RuntimeIntegrationError::ArithmeticOverflow {
                operation: "operation accounting",
            })?;

        Ok(())
    }

    /// Adds shot count using checked arithmetic.
    pub fn add_shots(&mut self, value: u128) -> RuntimeResult<()> {
        self.shots = self
            .shots
            .checked_add(value)
            .ok_or(RuntimeIntegrationError::ArithmeticOverflow {
                operation: "shot accounting",
            })?;

        Ok(())
    }

    /// Adds work units using checked arithmetic.
    pub fn add_work_units(&mut self, value: u128) -> RuntimeResult<()> {
        self.work_units = self
            .work_units
            .checked_add(value)
            .ok_or(RuntimeIntegrationError::ArithmeticOverflow {
                operation: "work accounting",
            })?;

        Ok(())
    }

    /// Adds memory usage using checked arithmetic.
    pub fn add_memory_bytes(&mut self, value: u128) -> RuntimeResult<()> {
        self.memory_bytes = self
            .memory_bytes
            .checked_add(value)
            .ok_or(RuntimeIntegrationError::ArithmeticOverflow {
                operation: "memory accounting",
            })?;

        Ok(())
    }

    /// Adds node usage using checked arithmetic.
    pub fn add_nodes(&mut self, value: u128) -> RuntimeResult<()> {
        self.nodes = self
            .nodes
            .checked_add(value)
            .ok_or(RuntimeIntegrationError::ArithmeticOverflow {
                operation: "node accounting",
            })?;

        Ok(())
    }

    /// Validates the accounting against runtime limits.
    pub fn validate(&self, limits: &RuntimeLimits) -> RuntimeResult<()> {
        limits.check(
            RuntimeResourceKind::Operations,
            self.operations,
        )?;
        limits.check(RuntimeResourceKind::Shots, self.shots)?;
        limits.check(
            RuntimeResourceKind::WorkUnits,
            self.work_units,
        )?;
        limits.check(
            RuntimeResourceKind::MemoryBytes,
            self.memory_bytes,
        )?;
        limits.check(RuntimeResourceKind::Nodes, self.nodes)?;

        Ok(())
    }
}

/// Runtime execution outcome status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeOutcomeStatus {
    /// Execution completed normally.
    Completed,

    /// Execution was cooperatively cancelled.
    Cancelled,

    /// Execution failed.
    Failed,
}

/// Runtime execution outcome.
///
/// This is intentionally an envelope rather than a simulator/hardware result
/// type. Concrete execution systems can attach their own result through an
/// external adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeExecutionOutcome {
    /// Execution identity.
    pub identity: ExecutionIdentity,

    /// Final lifecycle status.
    pub status: RuntimeOutcomeStatus,

    /// Execution accounting.
    pub accounting: RuntimeAccounting,

    /// Stable deterministic execution fingerprint.
    pub execution_fingerprint: u128,

    /// Optional adapter-defined result identity.
    pub result_id: Option<String>,

    /// Optional stable observation identity.
    pub observation_id: Option<String>,
}

impl RuntimeExecutionOutcome {
    /// Constructs a successful outcome.
    pub fn completed(
        identity: ExecutionIdentity,
        accounting: RuntimeAccounting,
    ) -> Self {
        let execution_fingerprint = identity.fingerprint();

        Self {
            identity,
            status: RuntimeOutcomeStatus::Completed,
            accounting,
            execution_fingerprint,
            result_id: None,
            observation_id: None,
        }
    }

    /// Constructs a cancelled outcome.
    pub fn cancelled(
        identity: ExecutionIdentity,
        accounting: RuntimeAccounting,
    ) -> Self {
        let execution_fingerprint = identity.fingerprint();

        Self {
            identity,
            status: RuntimeOutcomeStatus::Cancelled,
            accounting,
            execution_fingerprint,
            result_id: None,
            observation_id: None,
        }
    }

    /// Constructs a failed outcome.
    pub fn failed(
        identity: ExecutionIdentity,
        accounting: RuntimeAccounting,
    ) -> Self {
        let execution_fingerprint = identity.fingerprint();

        Self {
            identity,
            status: RuntimeOutcomeStatus::Failed,
            accounting,
            execution_fingerprint,
            result_id: None,
            observation_id: None,
        }
    }

    /// Adds an external result identity.
    ///
    /// Validation is performed by `set_result_id`.
    pub fn set_result_id(
        &mut self,
        result_id: String,
        limits: &RuntimeLimits,
    ) -> RuntimeResult<()> {
        limits.check_identifier("result_id", &result_id)?;
        self.result_id = Some(result_id);
        Ok(())
    }

    /// Adds an observation identity.
    pub fn set_observation_id(
        &mut self,
        observation_id: String,
        limits: &RuntimeLimits,
    ) -> RuntimeResult<()> {
        limits.check_identifier(
            "observation_id",
            &observation_id,
        )?;
        self.observation_id = Some(observation_id);
        Ok(())
    }
}

/// Runtime-facing capability declaration.
///
/// This describes what a runtime adapter can do without naming a vendor or
/// concrete implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeCapabilities {
    /// Supports deterministic execution under an explicit seed.
    pub deterministic_execution: bool,

    /// Supports cooperative cancellation.
    pub cancellation: bool,

    /// Supports logical-resource execution.
    pub logical_resources: bool,

    /// Supports physical-resource binding.
    pub physical_resources: bool,

    /// Supports arbitrary operation cardinality.
    pub variable_operation_arity: bool,

    /// Supports exact execution when requested.
    pub exact_execution: bool,

    /// Supports explicitly declared approximations.
    pub approximation: bool,

    /// Supports bounded approximations.
    pub bounded_execution: bool,

    /// Supports statistical execution.
    pub statistical_execution: bool,

    /// Supports concurrent execution.
    pub concurrent_execution: bool,

    /// Supports distributed execution.
    pub distributed_execution: bool,
}

impl RuntimeCapabilities {
    /// Returns a conservative capability set for an adapter that only
    /// guarantees deterministic local execution.
    #[must_use]
    pub const fn deterministic_local() -> Self {
        Self {
            deterministic_execution: true,
            cancellation: true,
            logical_resources: true,
            physical_resources: true,
            variable_operation_arity: true,
            exact_execution: true,
            approximation: true,
            bounded_execution: true,
            statistical_execution: true,
            concurrent_execution: false,
            distributed_execution: false,
        }
    }

    /// Validates a request against the declared capabilities.
    pub fn validate_request(
        &self,
        request: &RuntimeExecutionRequest,
    ) -> RuntimeResult<()> {
        if !self.logical_resources
            && !request.resources.logical_qubits().is_empty()
        {
            return Err(RuntimeIntegrationError::Unsupported {
                reason: "runtime does not support logical quantum resources",
            });
        }

        if !self.physical_resources
            && !request.resources.physical_qubits().is_empty()
        {
            return Err(RuntimeIntegrationError::Unsupported {
                reason: "runtime does not support physical quantum resources",
            });
        }

        if !self.variable_operation_arity {
            return Err(RuntimeIntegrationError::Unsupported {
                reason: "runtime does not advertise variable operation arity",
            });
        }

        if request.mode == ExecutionMode::Exact
            && !self.exact_execution
        {
            return Err(RuntimeIntegrationError::Unsupported {
                reason: "runtime does not support exact execution",
            });
        }

        match request.precision {
            PrecisionContract::Exact => {
                if !self.exact_execution {
                    return Err(RuntimeIntegrationError::Unsupported {
                        reason: "exact precision is not supported",
                    });
                }
            }

            PrecisionContract::Approximate { .. } => {
                if !self.approximation {
                    return Err(RuntimeIntegrationError::Unsupported {
                        reason: "approximate execution is not supported",
                    });
                }
            }

            PrecisionContract::Bounded { .. } => {
                if !self.bounded_execution {
                    return Err(RuntimeIntegrationError::Unsupported {
                        reason: "bounded execution is not supported",
                    });
                }
            }

            PrecisionContract::Statistical { .. } => {
                if !self.statistical_execution {
                    return Err(RuntimeIntegrationError::Unsupported {
                        reason: "statistical execution is not supported",
                    });
                }
            }
        }

        Ok(())
    }
}

/// Stable descriptor for a runtime adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDescriptor {
    /// Stable adapter identifier.
    pub runtime_id: String,

    /// Runtime implementation version.
    pub version: String,

    /// Capability declaration.
    pub capabilities: RuntimeCapabilities,
}

impl RuntimeDescriptor {
    /// Validates descriptor identifiers.
    pub fn validate(
        &self,
        limits: &RuntimeLimits,
    ) -> RuntimeResult<()> {
        limits.check_identifier("runtime_id", &self.runtime_id)?;
        limits.check_identifier("runtime_version", &self.version)?;

        Ok(())
    }
}

/// Runtime adapter contract.
///
/// This is the central integration trait.
///
/// A simulator, emulator or hardware runtime can implement it without
/// requiring this module to know the implementation details.
pub trait RuntimeAdapter: Send + Sync {
    /// Returns stable runtime metadata and capabilities.
    fn descriptor(&self) -> RuntimeResult<RuntimeDescriptor>;

    /// Validates a request before execution.
    fn validate(
        &self,
        request: &RuntimeExecutionRequest,
        context: &RuntimeExecutionContext<'_>,
    ) -> RuntimeResult<()> {
        request.validate(&context.limits)?;
        context.validate()?;

        let descriptor = self.descriptor()?;

        descriptor.validate(&context.limits)?;
        descriptor
            .capabilities
            .validate_request(request)?;

        Ok(())
    }

    /// Executes the request.
    ///
    /// The adapter owns actual execution.
    ///
    /// It MUST:
    ///
    /// - honor the supplied limits;
    /// - honor cancellation;
    /// - preserve execution identity;
    /// - never introduce hidden semantic randomness;
    /// - report explicit failure instead of silently changing semantics.
    fn execute(
        &self,
        request: &RuntimeExecutionRequest,
        context: &RuntimeExecutionContext<'_>,
    ) -> RuntimeResult<RuntimeExecutionOutcome>;
}

/// Thin ZQN runtime facade.
///
/// This type does not own an executor or global state. It merely validates and
/// delegates to a caller-provided runtime adapter.
pub struct ZqnRuntime<'a, A: RuntimeAdapter + ?Sized> {
    adapter: &'a A,
}

impl<'a, A: RuntimeAdapter + ?Sized> ZqnRuntime<'a, A> {
    /// Creates a runtime facade around an adapter.
    #[must_use]
    pub const fn new(adapter: &'a A) -> Self {
        Self { adapter }
    }

    /// Returns the underlying adapter.
    #[must_use]
    pub const fn adapter(&self) -> &'a A {
        self.adapter
    }

    /// Validates an execution request without executing it.
    pub fn validate(
        &self,
        request: &RuntimeExecutionRequest,
        context: &RuntimeExecutionContext<'_>,
    ) -> RuntimeResult<()> {
        self.adapter.validate(request, context)
    }

    /// Executes an already validated runtime request.
    pub fn execute(
        &self,
        request: &RuntimeExecutionRequest,
        context: &RuntimeExecutionContext<'_>,
    ) -> RuntimeResult<RuntimeExecutionOutcome> {
        self.adapter.validate(request, context)?;
        context.check_cancellation()?;

        self.adapter.execute(request, context)
    }
}

/// Stable 128-bit FNV-1a-style hasher.
///
/// This is used only for deterministic identity derivation.
///
/// It is NOT a cryptographic hash and MUST NOT be used for:
///
/// - authentication;
/// - signatures;
/// - password hashing;
/// - encryption;
/// - authorization.
///
/// Its purpose is stable semantic seed derivation without relying on
/// implementation-defined/randomized hash state.
#[derive(Debug, Clone, Copy)]
struct StableHasher128 {
    state: u128,
}

impl StableHasher128 {
    const OFFSET: u128 =
        0x6c62272e07bb014262b821756295c58d;

    const PRIME: u128 =
        0x0000000001000000000000000000013b;

    #[must_use]
    const fn new() -> Self {
        Self {
            state: Self::OFFSET,
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.state ^= value as u128;
        self.state = self.state.wrapping_mul(Self::PRIME);
    }

    fn write_u128(&mut self, value: u128) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    fn write_str(&mut self, value: &str) {
        self.write_u128(value.len() as u128);

        for byte in value.as_bytes() {
            self.write_u8(*byte);
        }
    }

    #[must_use]
    const fn finish(self) -> u128 {
        self.state
    }
}

fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> RuntimeResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(RuntimeIntegrationError::InvalidContext {
            reason: if field == "tolerance" {
                "tolerance must be finite and non-negative"
            } else {
                "error bound must be finite and non-negative"
            },
        });
    }

    Ok(())
}

fn reject_duplicates<T, F>(
    values: &[T],
    to_resource: F,
) -> RuntimeResult<()>
where
    T: PartialEq + Copy,
    F: Fn(T) -> RuntimeResource,
{
    for index in 1..values.len() {
        let current = values[index];

        if values[..index].contains(&current) {
            return Err(
                RuntimeIntegrationError::DuplicateResource {
                    resource: to_resource(current),
                },
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCancellation {
        cancelled: bool,
    }

    impl CancellationToken for TestCancellation {
        fn is_cancelled(&self) -> bool {
            self.cancelled
        }
    }

    #[derive(Debug)]
    struct TestAdapter;

    impl RuntimeAdapter for TestAdapter {
        fn descriptor(&self) -> RuntimeResult<RuntimeDescriptor> {
            Ok(RuntimeDescriptor {
                runtime_id: String::from("test-runtime"),
                version: String::from("1"),
                capabilities: RuntimeCapabilities::deterministic_local(),
            })
        }

        fn execute(
            &self,
            request: &RuntimeExecutionRequest,
            context: &RuntimeExecutionContext<'_>,
        ) -> RuntimeResult<RuntimeExecutionOutcome> {
            context.check_cancellation()?;

            let mut accounting = RuntimeAccounting::default();

            accounting.add_operations(
                request.resources.operation_count(),
            )?;

            accounting.add_shots(request.shots)?;
            accounting.add_nodes(request.nodes)?;
            accounting.add_work_units(request.work_units)?;
            accounting.add_memory_bytes(request.memory_bytes)?;

            accounting.validate(&context.limits)?;

            Ok(RuntimeExecutionOutcome::completed(
                request.identity.clone(),
                accounting,
            ))
        }
    }

    fn identity() -> ExecutionIdentity {
        ExecutionIdentity {
            program_id: String::from("program"),
            target_id: String::from("target"),
            noise_model_id: String::from("noise"),
            calibration_id: Some(String::from("calibration")),
            master_seed: 42,
            execution_key: 7,
        }
    }

    #[test]
    fn canonical_logical_and_physical_qubits_are_preserved() {
        let binding = RuntimeResourceBinding::new(
            vec![QubitId::new(0), QubitId::new(100)],
            vec![
                PhysicalQubitId::new(4),
                PhysicalQubitId::new(999),
            ],
            Vec::new(),
        )
        .expect("valid binding");

        assert_eq!(
            binding.logical_qubits()[0],
            QubitId::new(0)
        );

        assert_eq!(
            binding.logical_qubits()[1],
            QubitId::new(100)
        );

        assert_eq!(
            binding.physical_qubits()[0],
            PhysicalQubitId::new(4)
        );

        assert_eq!(
            binding.physical_qubits()[1],
            PhysicalQubitId::new(999)
        );
    }

    #[test]
    fn duplicate_logical_qubits_are_rejected() {
        let result = RuntimeResourceBinding::new(
            vec![QubitId::new(1), QubitId::new(1)],
            Vec::new(),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(RuntimeIntegrationError::DuplicateResource {
                resource: RuntimeResource::LogicalQubit(_)
            })
        ));
    }

    #[test]
    fn duplicate_physical_qubits_are_rejected() {
        let result = RuntimeResourceBinding::new(
            Vec::new(),
            vec![
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(1),
            ],
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(RuntimeIntegrationError::DuplicateResource {
                resource: RuntimeResource::PhysicalQubit(_)
            })
        ));
    }

    #[test]
    fn duplicate_operations_are_rejected() {
        let operation = OperationId::new(17);

        let result = RuntimeResourceBinding::new(
            Vec::new(),
            Vec::new(),
            vec![operation, operation],
        );

        assert!(matches!(
            result,
            Err(RuntimeIntegrationError::DuplicateResource {
                resource: RuntimeResource::Operation(_)
            })
        ));
    }

    #[test]
    fn no_implicit_machine_size_limit_exists() {
        let limits = RuntimeLimits::default();

        let binding = RuntimeResourceBinding::new(
            vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
                QubitId::new(3),
            ],
            vec![
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(3),
            ],
            Vec::new(),
        )
        .expect("valid binding");

        binding
            .validate(&limits)
            .expect("unlimited policy accepts resources");
    }

    #[test]
    fn explicit_limits_are_enforced() {
        let limits = RuntimeLimits {
            max_logical_qubits: Some(2),
            ..RuntimeLimits::default()
        };

        let binding = RuntimeResourceBinding::new(
            vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
            ],
            Vec::new(),
            Vec::new(),
        )
        .expect("identity set is structurally valid");

        let result = binding.validate(&limits);

        assert!(matches!(
            result,
            Err(RuntimeIntegrationError::ResourceLimitExceeded {
                resource: RuntimeResourceKind::LogicalQubits,
                requested: 3,
                maximum: 2,
            })
        ));
    }

    #[test]
    fn deterministic_seed_is_stable() {
        let identity = identity();

        let first = identity.fingerprint();
        let second = identity.fingerprint();

        assert_eq!(first, second);

        let seed_a = derive_seed(
            identity.master_seed,
            first,
            "operation",
            10,
        );

        let seed_b = derive_seed(
            identity.master_seed,
            first,
            "operation",
            10,
        );

        assert_eq!(seed_a, seed_b);
    }

    #[test]
    fn changing_seed_domain_changes_subseed() {
        let identity = identity();
        let fingerprint = identity.fingerprint();

        let first =
            derive_seed(identity.master_seed, fingerprint, "operation", 1);

        let second =
            derive_seed(identity.master_seed, fingerprint, "operation", 2);

        assert_ne!(first, second);
    }

    #[test]
    fn changing_domain_changes_subseed() {
        let identity = identity();
        let fingerprint = identity.fingerprint();

        let first =
            derive_seed(identity.master_seed, fingerprint, "gate", 1);

        let second =
            derive_seed(identity.master_seed, fingerprint, "measurement", 1);

        assert_ne!(first, second);
    }

    #[test]
    fn lifecycle_follows_valid_execution_path() {
        let mut tracker = RuntimeLifecycleTracker::default();

        assert_eq!(
            tracker.state(),
            RuntimeLifecycle::Created
        );

        tracker
            .transition(RuntimeLifecycle::Validated)
            .expect("Created -> Validated");

        tracker
            .transition(RuntimeLifecycle::Running)
            .expect("Validated -> Running");

        tracker
            .transition(RuntimeLifecycle::Completed)
            .expect("Running -> Completed");

        assert!(tracker.state().is_terminal());
    }

    #[test]
    fn invalid_lifecycle_transition_is_rejected() {
        let mut tracker = RuntimeLifecycleTracker::default();

        let result =
            tracker.transition(RuntimeLifecycle::Completed);

        assert!(matches!(
            result,
            Err(
                RuntimeIntegrationError::InvalidStateTransition {
                    from: RuntimeLifecycle::Created,
                    to: RuntimeLifecycle::Completed,
                }
            )
        ));
    }

    #[test]
    fn cancellation_is_observed() {
        let cancellation = TestCancellation {
            cancelled: true,
        };

        let context = RuntimeExecutionContext {
            identity: identity(),
            limits: RuntimeLimits::default(),
            resources: RuntimeResourceBinding::empty(),
            mode: ExecutionMode::Exact,
            precision: PrecisionContract::Exact,
            cancellation: Some(&cancellation),
        };

        assert!(matches!(
            context.check_cancellation(),
            Err(RuntimeIntegrationError::Cancelled)
        ));
    }

    #[test]
    fn never_cancel_never_cancels() {
        let token = NeverCancel;

        assert!(!token.is_cancelled());
    }

    #[test]
    fn exact_mode_requires_exact_precision() {
        let request = RuntimeExecutionRequest {
            identity: identity(),
            resources: RuntimeResourceBinding::empty(),
            shots: 1,
            nodes: 1,
            work_units: 1,
            memory_bytes: 0,
            mode: ExecutionMode::Exact,
            precision: PrecisionContract::Approximate {
                tolerance: 0.01,
            },
        };

        let result = request.validate(&RuntimeLimits::default());

        assert!(matches!(
            result,
            Err(RuntimeIntegrationError::InvalidContext { .. })
        ));
    }

    #[test]
    fn approximate_precision_rejects_nan() {
        let result =
            PrecisionContract::Approximate {
                tolerance: f64::NAN,
            }
            .validate();

        assert!(result.is_err());
    }

    #[test]
    fn approximate_precision_rejects_negative_values() {
        let result =
            PrecisionContract::Approximate {
                tolerance: -1.0,
            }
            .validate();

        assert!(result.is_err());
    }

    #[test]
    fn bounded_precision_rejects_infinity() {
        let result =
            PrecisionContract::Bounded {
                error_bound: f64::INFINITY,
            }
            .validate();

        assert!(result.is_err());
    }

    #[test]
    fn statistical_confidence_is_checked() {
        assert!(
            PrecisionContract::Statistical {
                confidence: 0.95
            }
            .validate()
            .is_ok()
        );

        assert!(
            PrecisionContract::Statistical {
                confidence: 0.0
            }
            .validate()
            .is_err()
        );

        assert!(
            PrecisionContract::Statistical {
                confidence: 1.1
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn accounting_uses_checked_arithmetic() {
        let mut accounting = RuntimeAccounting {
            operations: u128::MAX,
            ..RuntimeAccounting::default()
        };

        let result = accounting.add_operations(1);

        assert!(matches!(
            result,
            Err(RuntimeIntegrationError::ArithmeticOverflow {
                operation: "operation accounting"
            })
        ));
    }

    #[test]
    fn runtime_facade_delegates_to_adapter() {
        let adapter = TestAdapter;
        let runtime = ZqnRuntime::new(&adapter);

        let request = RuntimeExecutionRequest {
            identity: identity(),
            resources: RuntimeResourceBinding::empty(),
            shots: 1,
            nodes: 1,
            work_units: 1,
            memory_bytes: 0,
            mode: ExecutionMode::Exact,
            precision: PrecisionContract::Exact,
        };

        let context = RuntimeExecutionContext {
            identity: identity(),
            limits: RuntimeLimits::default(),
            resources: RuntimeResourceBinding::empty(),
            mode: ExecutionMode::Exact,
            precision: PrecisionContract::Exact,
            cancellation: None,
        };

        let outcome = runtime
            .execute(&request, &context)
            .expect("adapter execution");

        assert_eq!(
            outcome.status,
            RuntimeOutcomeStatus::Completed
        );

        assert_eq!(outcome.accounting.shots, 1);
    }

    #[test]
    fn result_identifier_obeys_limits() {
        let mut outcome =
            RuntimeExecutionOutcome::completed(
                identity(),
                RuntimeAccounting::default(),
            );

        let limits = RuntimeLimits {
            max_identifier_bytes: Some(4),
            ..RuntimeLimits::default()
        };

        assert!(
            outcome
                .set_result_id(String::from("ok"), &limits)
                .is_ok()
        );

        assert!(
            outcome
                .set_result_id(
                    String::from("too-large"),
                    &limits
                )
                .is_err()
        );
    }

    #[test]
    fn runtime_descriptor_is_validated() {
        let descriptor = RuntimeDescriptor {
            runtime_id: String::from("runtime"),
            version: String::from("1"),
            capabilities:
                RuntimeCapabilities::deterministic_local(),
        };

        assert!(
            descriptor
                .validate(&RuntimeLimits::default())
                .is_ok()
        );
    }

    #[test]
    fn runtime_context_derives_stable_subseed() {
        let context = RuntimeExecutionContext {
            identity: identity(),
            limits: RuntimeLimits::default(),
            resources: RuntimeResourceBinding::empty(),
            mode: ExecutionMode::Exact,
            precision: PrecisionContract::Exact,
            cancellation: None,
        };

        let first = context.derive_seed("shot", 7);
        let second = context.derive_seed("shot", 7);

        assert_eq!(first, second);
    }

    #[test]
    fn runtime_context_rejects_exact_mode_with_approximation() {
        let context = RuntimeExecutionContext {
            identity: identity(),
            limits: RuntimeLimits::default(),
            resources: RuntimeResourceBinding::empty(),
            mode: ExecutionMode::Exact,
            precision: PrecisionContract::Approximate {
                tolerance: 0.001,
            },
            cancellation: None,
        };

        assert!(context.validate().is_err());
    }

    #[test]
    fn runtime_context_accepts_explicit_approximation() {
        let context = RuntimeExecutionContext {
            identity: identity(),
            limits: RuntimeLimits::default(),
            resources: RuntimeResourceBinding::empty(),
            mode: ExecutionMode::Approximate,
            precision: PrecisionContract::Approximate {
                tolerance: 0.001,
            },
            cancellation: None,
        };

        assert!(context.validate().is_ok());
    }

    #[test]
    fn runtime_outcome_preserves_identity() {
        let identity = identity();

        let outcome = RuntimeExecutionOutcome::completed(
            identity.clone(),
            RuntimeAccounting::default(),
        );

        assert_eq!(outcome.identity, identity);
        assert_eq!(
            outcome.execution_fingerprint,
            identity.fingerprint()
        );
    }
}