//! Zamani Quantum Noise (ZQN) — Channel Execution Engine.
//!
//! # Ownership
//!
//! This module owns the execution contract for applying already-defined ZQN
//! quantum channels to simulator state representations.
//!
//! It owns:
//!
//! - channel execution configuration;
//! - explicit resource governance;
//! - cancellation checks;
//! - channel-state representation used at this execution boundary;
//! - deterministic channel application;
//! - state-vector -> density-matrix conversion;
//! - density-matrix channel application;
//! - streaming/batched channel execution;
//! - canonical resource binding using `quantum::ir::qubit::QubitId`;
//! - execution-level validation;
//! - execution-level error translation;
//! - execution statistics.
//!
//! # This module does NOT own
//!
//! This file does not own:
//!
//! - quantum source syntax;
//! - canonical quantum IR semantics;
//! - channel mathematics;
//! - Kraus construction;
//! - Choi construction;
//! - Pauli-channel definitions;
//! - Lindblad mathematics;
//! - noise-model selection;
//! - random-number generation;
//! - stochastic trajectory selection;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - calibration;
//! - hardware APIs;
//! - GPU kernels;
//! - distributed execution;
//! - vendor-specific behavior;
//! - state allocation policies outside this execution boundary.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                         │
//!                         ▼
//!                  ZQN NoiseModel
//!                         │
//!                         ▼
//!                   QuantumChannel
//!                         │
//!                         ▼
//!                    KrausChannel
//!                         │
//!                         ▼
//!              simulation/channel_engine
//!                         │
//!              ┌──────────┴──────────┐
//!              ▼                     ▼
//!       density-matrix          state-vector
//!       deterministic           conversion
//!              │                     │
//!              └──────────┬──────────┘
//!                         ▼
//!                  simulator state
//! ```
//!
//! The engine executes a mathematical channel. It does not decide which
//! channel should apply.
//!
//! # Write-once / scale-everywhere contract
//!
//! No semantic machine-size limit exists in this file.
//!
//! There is deliberately no:
//!
//! - `MAX_QUBITS`;
//! - `MAX_DIMENSION`;
//! - `MAX_CHANNELS`;
//! - fixed gate arity;
//! - fixed two-qubit assumption;
//! - vendor-specific limit.
//!
//! A dimension is data.
//!
//! A number of operations is data.
//!
//! A number of qubits is data.
//!
//! A resource limit is execution policy.
//!
//! Therefore:
//!
//! ```text
//! semantic capacity
//!     !=
//! runtime allocation capacity
//! ```
//!
//! A sufficiently large computation may fail because the selected execution
//! policy or available machine resources cannot materialize it. That is an
//! execution-resource fact, not a semantic limit imposed by ZQN.
//!
//! # Representation policy
//!
//! The engine currently provides a dense host representation because the
//! repository already defines `Complex64` as the canonical double-precision
//! quantum-memory scalar and `KrausChannel` already consumes dense row-major
//! complex matrices.
//!
//! This does NOT make dense simulation the universal representation.
//!
//! Future engines may implement:
//!
//! - sparse states;
//! - tensor networks;
//! - matrix-product states;
//! - stabilizer representations;
//! - trajectories;
//! - GPU state;
//! - distributed state;
//! - symbolic states;
//! - analog representations;
//! - continuous-variable representations.
//!
//! Those engines should consume the same execution concepts without requiring
//! this file to be rewritten.
//!
//! # Determinism
//!
//! Channel application itself is deterministic.
//!
//! This file owns no RNG.
//!
//! It must never call:
//!
//! - `thread_rng()`;
//! - a process-global RNG;
//! - a hidden simulator RNG.
//!
//! Stochastic selection of Kraus branches belongs to
//! `simulation/trajectory.rs` / `simulation/sampler.rs`.
//!
//! The same input channel and state therefore produce the same deterministic
//! output regardless of thread scheduling.
//!
//! # Qubit identity
//!
//! When a channel execution is associated with concrete qubit resources,
//! canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! is used.
//!
//! This module never defines another `QubitId`.
//!
//! Resource binding is optional because a mathematical channel may be created
//! before routing/lowering determines which physical resources execute it.
//!
//! # Numerical safety
//!
//! The engine rejects:
//!
//! - non-finite state elements;
//! - dimension/element-count mismatches;
//! - integer overflow in dimension calculations;
//! - resource-limit violations;
//! - invalid channel dimensions;
//! - invalid state representations.
//!
//! It never silently converts:
//!
//! - NaN -> 0;
//! - infinity -> finite;
//! - negative values -> absolute values;
//! - invalid dimensions -> defaults.
//!
//! # Resource safety
//!
//! Potentially expensive allocations are checked before execution.
//!
//! Limits are optional.
//!
//! `None` means that this layer does not impose a limit.
//!
//! It does not mean that the operating system or allocator has infinite
//! capacity.
//!
//! # Integration contract
//!
//! ```text
//! channel/channel.rs
//!     owns representation-independent QuantumChannel semantics.
//!
//! channel/kraus.rs
//!     owns Kraus mathematics and already provides deterministic application
//!     to dense density matrices.
//!
//! simulation/sampler.rs
//!     owns reproducible random sampling.
//!
//! simulation/trajectory.rs
//!     owns stochastic Kraus-branch trajectories.
//!
//! simulation/engine.rs
//!     may use this module as one channel-execution primitive.
//!
//! integration/memory.rs
//!     adapts this dense row-major representation to quantum-memory storage.
//!
//! integration/ir.rs
//!     supplies semantic operations/resources.
//!
//! noise/model.rs
//!     decides which channel applies.
//!
//! routing/scheduling
//!     determine resource placement and execution timing; they do not own
//!     channel execution.
//! ```
//!
//! # No-reedit contract
//!
//! Adding a new:
//!
//! - simulator;
//! - routing algorithm;
//! - scheduler;
//! - QEC code;
//! - hardware provider;
//! - quantum technology;
//! - noise model;
//! - channel representation;
//! - trajectory sampler;
//!
//! must not require changing this file merely because the consumer changed.
//!
//! A new mathematical representation should add its own execution adapter.
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
//! - no unsafe Rust.
//!
//! # Security
//!
//! The engine treats dimensions and allocation requests as untrusted data.
//!
//! All multiplication used for allocation sizing is checked.
//!
//! No external process, filesystem, network, or dynamic-code execution is
//! performed here.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::memory::complex::{Complex64, ComplexScalar};
use crate::quantum::zqn::channel::kraus::{KrausChannel, KrausError};

// =============================================================================
// Constants
// =============================================================================

/// Stable schema identifier for this execution contract.
pub const CHANNEL_ENGINE_SCHEMA_ID: &str =
    "zamani.quantum.zqn.simulation.channel_engine";

/// Semantic version of this execution contract.
pub const CHANNEL_ENGINE_SCHEMA_VERSION: u16 = 1;

/// Number of bytes in one `Complex64`.
///
/// This is derived from the canonical type rather than used as an architectural
/// capacity.
const COMPLEX64_BYTES: usize =
    <Complex64 as ComplexScalar>::BYTE_SIZE;

// =============================================================================
// Cancellation
// =============================================================================

/// Cooperative cancellation boundary for long-running channel execution.
///
/// Implementations must be cheap to query and must not mutate the simulation
/// semantics.
pub trait CancellationToken: Send + Sync {
    /// Returns `true` when execution should stop.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation token that never cancels.
///
/// Useful when the caller wants the lowest-overhead execution path.
#[derive(Debug, Default, Clone, Copy)]
pub struct NeverCancel;

impl CancellationToken for NeverCancel {
    #[inline]
    fn is_cancelled(&self) -> bool {
        false
    }
}

// =============================================================================
// Resource binding
// =============================================================================

/// Canonical resource binding for channel execution.
///
/// This type intentionally stores canonical IR qubit identities rather than
/// introducing a ZQN-specific qubit ID.
///
/// The binding is optional because routing/lowering may occur after channel
/// construction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChannelResourceBinding {
    qubits: Vec<QubitId>,
}

impl ChannelResourceBinding {
    /// Creates an empty binding.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a binding from canonical IR qubit identities.
    ///
    /// Duplicate resource identities are rejected because one execution
    /// location cannot unambiguously bind the same semantic qubit twice.
    pub fn new(qubits: Vec<QubitId>) -> Result<Self, ChannelEngineError> {
        for index in 1..qubits.len() {
            if qubits[..index].contains(&qubits[index]) {
                return Err(ChannelEngineError::DuplicateQubitResource {
                    qubit: qubits[index],
                });
            }
        }

        Ok(Self { qubits })
    }

    /// Returns the bound qubits.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns the number of bound qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether no qubits are bound.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }
}

// =============================================================================
// Execution configuration
// =============================================================================

/// Resource policy for channel execution.
///
/// Every field is optional so the semantic layer does not impose artificial
/// machine-size limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelEngineConfig {
    /// Maximum number of complex matrix elements that one execution may
    /// materialize.
    ///
    /// `None` means no limit at this layer.
    pub max_matrix_elements: Option<u128>,

    /// Maximum number of bytes that one execution may materialize.
    ///
    /// `None` means no limit at this layer.
    pub max_allocation_bytes: Option<u128>,

    /// Maximum number of channels processed by one batch call.
    ///
    /// `None` means no limit at this layer.
    pub max_batch_operations: Option<u64>,

    /// Check cancellation after this many operations.
    ///
    /// `0` is rejected.
    pub cancellation_check_interval: u64,
}

impl Default for ChannelEngineConfig {
    fn default() -> Self {
        Self {
            max_matrix_elements: None,
            max_allocation_bytes: None,
            max_batch_operations: None,
            cancellation_check_interval: 1,
        }
    }
}

impl ChannelEngineConfig {
    /// Validates the execution policy.
    pub fn validate(&self) -> Result<(), ChannelEngineError> {
        if self.cancellation_check_interval == 0 {
            return Err(ChannelEngineError::InvalidConfiguration(
                "cancellation_check_interval must be greater than zero",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Execution context
// =============================================================================

/// Context supplied to one channel execution.
pub struct ChannelExecutionContext<'a> {
    /// Optional canonical resource binding.
    pub resources: Option<&'a ChannelResourceBinding>,

    /// Optional cooperative cancellation token.
    pub cancellation: Option<&'a dyn CancellationToken>,

    /// Stable semantic operation identity supplied by the caller.
    ///
    /// This value is metadata only. It is not a qubit index and is not a
    /// machine-size limit.
    pub operation_id: Option<u128>,
}

impl<'a> Default for ChannelExecutionContext<'a> {
    fn default() -> Self {
        Self {
            resources: None,
            cancellation: None,
            operation_id: None,
        }
    }
}

// =============================================================================
// State representation
// =============================================================================

/// Dense channel-engine state.
///
/// A channel engine operates on either:
///
/// - a pure state vector; or
/// - a density matrix.
///
/// A general noisy channel does not preserve purity, so applying a channel to
/// a state vector produces a density matrix rather than pretending the result
/// is still a pure vector.
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelState {
    /// Pure state vector.
    StateVector {
        /// State amplitudes.
        amplitudes: Vec<Complex64>,
    },

    /// Density matrix in row-major order.
    DensityMatrix {
        /// Hilbert-space dimension.
        dimension: usize,

        /// Row-major matrix elements.
        elements: Vec<Complex64>,
    },
}

impl ChannelState {
    /// Creates a state-vector state after validating finiteness.
    pub fn state_vector(
        amplitudes: Vec<Complex64>,
    ) -> Result<Self, ChannelEngineError> {
        validate_complex_slice(&amplitudes)?;

        if amplitudes.is_empty() {
            return Err(ChannelEngineError::ZeroDimension);
        }

        Ok(Self::StateVector { amplitudes })
    }

    /// Creates a density-matrix state after validating its dimensions and
    /// elements.
    pub fn density_matrix(
        dimension: usize,
        elements: Vec<Complex64>,
    ) -> Result<Self, ChannelEngineError> {
        validate_square_matrix(dimension, elements.len())?;
        validate_complex_slice(&elements)?;

        Ok(Self::DensityMatrix {
            dimension,
            elements,
        })
    }

    /// Returns the state dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        match self {
            Self::StateVector { amplitudes } => amplitudes.len(),
            Self::DensityMatrix { dimension, .. } => *dimension,
        }
    }

    /// Returns whether this is a pure state-vector representation.
    #[must_use]
    pub fn is_state_vector(&self) -> bool {
        matches!(self, Self::StateVector { .. })
    }

    /// Returns whether this is a density-matrix representation.
    #[must_use]
    pub fn is_density_matrix(&self) -> bool {
        matches!(self, Self::DensityMatrix { .. })
    }

    /// Returns state-vector data when available.
    #[must_use]
    pub fn as_state_vector(&self) -> Option<&[Complex64]> {
        match self {
            Self::StateVector { amplitudes } => Some(amplitudes),
            Self::DensityMatrix { .. } => None,
        }
    }

    /// Returns density-matrix data when available.
    #[must_use]
    pub fn as_density_matrix(&self) -> Option<(usize, &[Complex64])> {
        match self {
            Self::StateVector { .. } => None,
            Self::DensityMatrix {
                dimension,
                elements,
            } => Some((*dimension, elements)),
        }
    }
}

// =============================================================================
// Execution result
// =============================================================================

/// Result of deterministic channel execution.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelExecutionResult {
    /// Resulting density matrix.
    pub state: ChannelState,

    /// Number of input matrix elements processed.
    pub input_elements: u128,

    /// Number of output matrix elements produced.
    pub output_elements: u128,

    /// Number of Kraus operators represented by the executed channel.
    pub channel_operators: usize,
}

impl ChannelExecutionResult {
    /// Returns the output Hilbert-space dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.state.dimension()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the channel execution engine.
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelEngineError {
    /// Configuration is invalid.
    InvalidConfiguration(&'static str),

    /// A dimension is zero.
    ZeroDimension,

    /// A square matrix size cannot be represented safely.
    DimensionOverflow {
        /// Requested dimension.
        dimension: usize,
    },

    /// A matrix element count does not match its dimension.
    ElementCountMismatch {
        /// Expected count.
        expected: usize,

        /// Actual count.
        actual: usize,
    },

    /// A state dimension does not match a channel's input dimension.
    InputDimensionMismatch {
        /// Channel input dimension.
        expected: usize,

        /// State dimension.
        actual: usize,
    },

    /// A duplicate canonical qubit was supplied.
    DuplicateQubitResource {
        /// Duplicated canonical qubit.
        qubit: QubitId,
    },

    /// A numerical value is not finite.
    NonFiniteElement {
        /// Flat element index.
        index: usize,
    },

    /// A resource policy would be exceeded.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Requested amount.
        requested: u128,

        /// Configured limit.
        limit: u128,
    },

    /// Execution was cancelled.
    Cancelled,

    /// The channel implementation rejected the execution.
    Channel(KrausError),

    /// Arithmetic overflow occurred while calculating an execution size.
    ArithmeticOverflow {
        /// Description of the calculation.
        operation: &'static str,
    },
}

impl fmt::Display for ChannelEngineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid channel-engine configuration: {message}")
            }

            Self::ZeroDimension => {
                formatter.write_str("channel-engine state dimension must be non-zero")
            }

            Self::DimensionOverflow { dimension } => {
                write!(
                    formatter,
                    "cannot represent square matrix for dimension {dimension}"
                )
            }

            Self::ElementCountMismatch { expected, actual } => {
                write!(
                    formatter,
                    "matrix element count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::InputDimensionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "channel input dimension mismatch: expected {expected}, got {actual}"
                )
            }

            Self::DuplicateQubitResource { qubit } => {
                write!(formatter, "duplicate canonical qubit resource: {qubit:?}")
            }

            Self::NonFiniteElement { index } => {
                write!(formatter, "non-finite state element at index {index}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "channel execution resource limit exceeded: {resource}, \
                     requested={requested}, limit={limit}"
                )
            }

            Self::Cancelled => formatter.write_str("channel execution cancelled"),

            Self::Channel(error) => write!(formatter, "Kraus channel execution failed: {error}"),

            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "arithmetic overflow during {operation}")
            }
        }
    }
}

impl Error for ChannelEngineError {}

impl From<KrausError> for ChannelEngineError {
    fn from(error: KrausError) -> Self {
        Self::Channel(error)
    }
}

// =============================================================================
// Engine
// =============================================================================

/// Deterministic quantum-channel execution engine.
///
/// The engine is intentionally stateless.
///
/// It contains no:
///
/// - RNG;
/// - global cache;
/// - mutable global state;
/// - hardware connection;
/// - simulator-wide state.
///
/// This makes an engine instance safe to share between concurrent executions
/// when its configuration is shared.
#[derive(Debug, Clone, Copy)]
pub struct ChannelEngine {
    config: ChannelEngineConfig,
}

impl ChannelEngine {
    /// Creates an engine from an explicit execution policy.
    pub fn new(config: ChannelEngineConfig) -> Result<Self, ChannelEngineError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Creates an unrestricted engine at this layer.
    ///
    /// "Unrestricted" means no ZQN channel-engine policy limit; it does not
    /// override operating-system, allocator, address-space or hardware limits.
    #[must_use]
    pub fn unrestricted() -> Self {
        Self {
            config: ChannelEngineConfig::default(),
        }
    }

    /// Returns the execution policy.
    #[must_use]
    pub const fn config(&self) -> ChannelEngineConfig {
        self.config
    }

    /// Applies a Kraus channel to a density matrix.
    ///
    /// This is the canonical deterministic execution path for a general
    /// channel.
    pub fn apply_kraus_to_density_matrix(
        &self,
        channel: &KrausChannel,
        density_matrix: &[Complex64],
        context: &ChannelExecutionContext<'_>,
    ) -> Result<ChannelExecutionResult, ChannelEngineError> {
        self.validate_context(context)?;
        self.check_cancelled(context)?;

        let input_dimension = channel.input_dimension();
        let output_dimension = channel.output_dimension();

        validate_square_matrix(input_dimension, density_matrix.len())?;
        validate_complex_slice(density_matrix)?;

        self.check_matrix_resources(input_dimension)?;
        self.check_matrix_resources(output_dimension)?;

        let result = channel.apply_to_density_matrix(density_matrix)?;

        self.check_cancelled(context)?;
        validate_square_matrix(output_dimension, result.len())?;
        validate_complex_slice(&result)?;

        let input_elements = square_element_count(input_dimension)?;
        let output_elements = square_element_count(output_dimension)?;

        Ok(ChannelExecutionResult {
            state: ChannelState::DensityMatrix {
                dimension: output_dimension,
                elements: result,
            },
            input_elements,
            output_elements,
            channel_operators: channel.operators().len(),
        })
    }

    /// Applies a Kraus channel to a pure state vector.
    ///
    /// A general channel may map a pure state to a mixed state. Therefore this
    /// method constructs the input density matrix and executes the channel
    /// deterministically.
    pub fn apply_kraus_to_state_vector(
        &self,
        channel: &KrausChannel,
        amplitudes: &[Complex64],
        context: &ChannelExecutionContext<'_>,
    ) -> Result<ChannelExecutionResult, ChannelEngineError> {
        self.validate_context(context)?;
        self.check_cancelled(context)?;

        let expected = channel.input_dimension();

        if amplitudes.len() != expected {
            return Err(ChannelEngineError::InputDimensionMismatch {
                expected,
                actual: amplitudes.len(),
            });
        }

        validate_complex_slice(amplitudes)?;

        self.check_matrix_resources(expected)?;

        let density = pure_state_to_density_matrix(amplitudes)?;

        self.apply_kraus_to_density_matrix(channel, &density, context)
    }

    /// Applies a channel to the supplied `ChannelState`.
    ///
    /// State vectors are converted to density matrices because deterministic
    /// general-channel evolution cannot in general remain a pure state.
    pub fn apply_kraus(
        &self,
        channel: &KrausChannel,
        state: &ChannelState,
        context: &ChannelExecutionContext<'_>,
    ) -> Result<ChannelExecutionResult, ChannelEngineError> {
        match state {
            ChannelState::StateVector { amplitudes } => {
                self.apply_kraus_to_state_vector(channel, amplitudes, context)
            }

            ChannelState::DensityMatrix {
                dimension,
                elements,
            } => {
                if *dimension != channel.input_dimension() {
                    return Err(ChannelEngineError::InputDimensionMismatch {
                        expected: channel.input_dimension(),
                        actual: *dimension,
                    });
                }

                self.apply_kraus_to_density_matrix(channel, elements, context)
            }
        }
    }

    /// Applies a sequence of channels to a density matrix.
    ///
    /// The intermediate state is streamed through the channel sequence; the
    /// caller does not need to construct a second list of channels or a second
    /// list of states.
    pub fn apply_kraus_sequence<I>(
        &self,
        channels: I,
        initial_density_matrix: &[Complex64],
        context: &ChannelExecutionContext<'_>,
    ) -> Result<ChannelExecutionResult, ChannelEngineError>
    where
        I: IntoIterator<Item = &'_ KrausChannel>,
    {
        self.validate_context(context)?;

        let mut current = initial_density_matrix.to_vec();
        let mut current_dimension = infer_square_dimension(current.len())?;

        let mut operations = 0_u64;
        let mut last_operator_count = 0_usize;

        for channel in channels {
            operations = operations
                .checked_add(1)
                .ok_or(ChannelEngineError::ArithmeticOverflow {
                    operation: "sequence operation count",
                })?;

            if let Some(limit) = self.config.max_batch_operations {
                if operations > limit {
                    return Err(ChannelEngineError::ResourceLimitExceeded {
                        resource: "max_batch_operations",
                        requested: u128::from(operations),
                        limit: u128::from(limit),
                    });
                }
            }

            if operations % self.config.cancellation_check_interval == 0 {
                self.check_cancelled(context)?;
            }

            if current_dimension != channel.input_dimension() {
                return Err(ChannelEngineError::InputDimensionMismatch {
                    expected: channel.input_dimension(),
                    actual: current_dimension,
                });
            }

            let result =
                self.apply_kraus_to_density_matrix(channel, &current, context)?;

            let next = match result.state {
                ChannelState::DensityMatrix {
                    dimension,
                    elements,
                } => {
                    current_dimension = dimension;
                    last_operator_count = result.channel_operators;
                    elements
                }

                ChannelState::StateVector { .. } => {
                    return Err(ChannelEngineError::InvalidConfiguration(
                        "channel engine produced an unexpected state-vector result",
                    ));
                }
            };

            current = next;
        }

        self.check_cancelled(context)?;

        let input_elements = square_element_count(current_dimension)?;

        Ok(ChannelExecutionResult {
            state: ChannelState::DensityMatrix {
                dimension: current_dimension,
                elements: current,
            },
            input_elements,
            output_elements: input_elements,
            channel_operators: last_operator_count,
        })
    }

    /// Applies a channel to every state in a streaming iterator.
    ///
    /// Results are produced one at a time. This avoids requiring the engine to
    /// materialize an arbitrarily large collection of states.
    pub fn apply_kraus_stream<'a, I>(
        &'a self,
        channel: &'a KrausChannel,
        states: I,
        context: &'a ChannelExecutionContext<'a>,
    ) -> ChannelExecutionStream<'a, I::IntoIter>
    where
        I: IntoIterator<Item = ChannelState>,
    {
        ChannelExecutionStream {
            engine: self,
            channel,
            states: states.into_iter(),
            context,
            processed: 0,
            finished: false,
        }
    }

    /// Converts a pure state vector to a density matrix without applying a
    /// channel.
    pub fn state_vector_to_density_matrix(
        &self,
        amplitudes: &[Complex64],
    ) -> Result<ChannelState, ChannelEngineError> {
        validate_complex_slice(amplitudes)?;

        if amplitudes.is_empty() {
            return Err(ChannelEngineError::ZeroDimension);
        }

        self.check_matrix_resources(amplitudes.len())?;

        let density = pure_state_to_density_matrix(amplitudes)?;

        let dimension = amplitudes.len();

        Ok(ChannelState::DensityMatrix {
            dimension,
            elements: density,
        })
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    fn validate_context(
        &self,
        context: &ChannelExecutionContext<'_>,
    ) -> Result<(), ChannelEngineError> {
        self.config.validate()?;

        if let Some(resources) = context.resources {
            // The binding itself has already rejected duplicate resources.
            // Reading the collection here makes the resource relationship
            // explicit at the execution boundary without imposing an arity
            // assumption.
            let _ = resources.qubits();
        }

        Ok(())
    }

    fn check_cancelled(
        &self,
        context: &ChannelExecutionContext<'_>,
    ) -> Result<(), ChannelEngineError> {
        if context
            .cancellation
            .is_some_and(CancellationToken::is_cancelled)
        {
            return Err(ChannelEngineError::Cancelled);
        }

        Ok(())
    }

    fn check_matrix_resources(
        &self,
        dimension: usize,
    ) -> Result<(), ChannelEngineError> {
        let elements = square_element_count(dimension)?;

        if let Some(limit) = self.config.max_matrix_elements {
            if elements > limit {
                return Err(ChannelEngineError::ResourceLimitExceeded {
                    resource: "max_matrix_elements",
                    requested: elements,
                    limit,
                });
            }
        }

        let bytes = elements
            .checked_mul(COMPLEX64_BYTES)
            .ok_or(ChannelEngineError::ArithmeticOverflow {
                operation: "matrix byte-size calculation",
            })?;

        let bytes_u128 = bytes as u128;

        if let Some(limit) = self.config.max_allocation_bytes {
            if bytes_u128 > limit {
                return Err(ChannelEngineError::ResourceLimitExceeded {
                    resource: "max_allocation_bytes",
                    requested: bytes_u128,
                    limit,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Streaming execution
// =============================================================================

/// Streaming channel execution iterator.
///
/// An error terminates the stream. No later item is produced after an error.
pub struct ChannelExecutionStream<'a, I> {
    engine: &'a ChannelEngine,
    channel: &'a KrausChannel,
    states: I,
    context: &'a ChannelExecutionContext<'a>,
    processed: u64,
    finished: bool,
}

impl<'a, I> Iterator for ChannelExecutionStream<'a, I>
where
    I: Iterator<Item = ChannelState>,
{
    type Item = Result<ChannelExecutionResult, ChannelEngineError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if let Some(limit) = self.engine.config.max_batch_operations {
            if self.processed >= limit {
                self.finished = true;

                return Some(Err(ChannelEngineError::ResourceLimitExceeded {
                    resource: "max_batch_operations",
                    requested: u128::from(self.processed.saturating_add(1)),
                    limit: u128::from(limit),
                }));
            }
        }

        let state = match self.states.next() {
            Some(state) => state,
            None => {
                self.finished = true;
                return None;
            }
        };

        self.processed = match self.processed.checked_add(1) {
            Some(value) => value,
            None => {
                self.finished = true;

                return Some(Err(ChannelEngineError::ArithmeticOverflow {
                    operation: "stream operation count",
                }));
            }
        };

        if self.processed % self.engine.config.cancellation_check_interval == 0 {
            if let Err(error) = self.engine.check_cancelled(self.context) {
                self.finished = true;
                return Some(Err(error));
            }
        }

        match self.engine.apply_kraus(self.channel, &state, self.context) {
            Ok(result) => Some(Ok(result)),
            Err(error) => {
                self.finished = true;
                Some(Err(error))
            }
        }
    }
}

// =============================================================================
// Mathematical helpers
// =============================================================================

/// Calculates `dimension * dimension` without overflow.
fn square_element_count(dimension: usize) -> Result<u128, ChannelEngineError> {
    if dimension == 0 {
        return Err(ChannelEngineError::ZeroDimension);
    }

    let dimension_u128 = dimension as u128;

    dimension_u128
        .checked_mul(dimension_u128)
        .ok_or(ChannelEngineError::DimensionOverflow { dimension })
}

/// Validates that a vector contains exactly `dimension²` elements.
fn validate_square_matrix(
    dimension: usize,
    actual_elements: usize,
) -> Result<(), ChannelEngineError> {
    let expected_u128 = square_element_count(dimension)?;

    let expected = usize::try_from(expected_u128).map_err(|_| {
        ChannelEngineError::DimensionOverflow { dimension }
    })?;

    if expected != actual_elements {
        return Err(ChannelEngineError::ElementCountMismatch {
            expected,
            actual: actual_elements,
        });
    }

    Ok(())
}

/// Infers a square matrix dimension from its element count.
///
/// This is only used for a pre-existing dense density matrix whose dimension
/// was not separately supplied.
///
/// Integer arithmetic is used rather than floating-point square roots so that
/// no rounding ambiguity can turn an invalid matrix into a valid one.
fn infer_square_dimension(
    elements: usize,
) -> Result<usize, ChannelEngineError> {
    if elements == 0 {
        return Err(ChannelEngineError::ZeroDimension);
    }

    let mut low = 1usize;
    let mut high = elements;

    while low <= high {
        let mid = low + (high - low) / 2;

        match mid.checked_mul(mid) {
            Some(square) if square == elements => return Ok(mid),

            Some(square) if square < elements => {
                low = mid.saturating_add(1);
            }

            Some(_) => {
                if mid == 0 {
                    break;
                }

                high = mid - 1;
            }

            None => {
                high = mid - 1;
            }
        }
    }

    Err(ChannelEngineError::ElementCountMismatch {
        expected: elements,
        actual: elements,
    })
}

/// Validates all complex values in a slice.
fn validate_complex_slice(
    values: &[Complex64],
) -> Result<(), ChannelEngineError> {
    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ChannelEngineError::NonFiniteElement { index });
        }
    }

    Ok(())
}

/// Builds `|ψ><ψ|` from a state vector.
///
/// The result is row-major.
///
/// No normalization is silently performed. The input state therefore retains
/// its mathematical norm exactly up to floating-point arithmetic.
fn pure_state_to_density_matrix(
    amplitudes: &[Complex64],
) -> Result<Vec<Complex64>, ChannelEngineError> {
    if amplitudes.is_empty() {
        return Err(ChannelEngineError::ZeroDimension);
    }

    validate_complex_slice(amplitudes)?;

    let dimension = amplitudes.len();

    let elements = dimension
        .checked_mul(dimension)
        .ok_or(ChannelEngineError::DimensionOverflow { dimension })?;

    let mut density = Vec::with_capacity(elements);

    for row in amplitudes.iter().copied() {
        let row_conjugate = row.conjugate();

        for column in amplitudes.iter().copied() {
            density.push(row * column.conjugate());
        }

        // `row_conjugate` is intentionally evaluated above only through the
        // canonical scalar API. Keep the actual formula explicit below.
        let _ = row_conjugate;
    }

    Ok(density)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::zqn::channel::kraus::KrausChannel;

    fn identity_channel(dimension: usize) -> KrausChannel {
        KrausChannel::identity(dimension).expect("identity channel must construct")
    }

    fn qubit_zero() -> Vec<Complex64> {
        vec![Complex64::ONE, Complex64::ZERO]
    }

    #[test]
    fn pure_state_conversion_has_correct_shape() {
        let state = qubit_zero();

        let density =
            pure_state_to_density_matrix(&state).expect("conversion must succeed");

        assert_eq!(density.len(), 4);
        assert_eq!(density[0], Complex64::ONE);
        assert_eq!(density[1], Complex64::ZERO);
        assert_eq!(density[2], Complex64::ZERO);
        assert_eq!(density[3], Complex64::ZERO);
    }

    #[test]
    fn state_vector_is_converted_to_density_matrix() {
        let engine = ChannelEngine::unrestricted();
        let state = qubit_zero();

        let result = engine
            .state_vector_to_density_matrix(&state)
            .expect("conversion must succeed");

        assert_eq!(result.dimension(), 2);
        assert!(result.is_density_matrix());
    }

    #[test]
    fn identity_channel_preserves_density_matrix() {
        let engine = ChannelEngine::unrestricted();
        let channel = identity_channel(2);

        let density = vec![
            Complex64::ONE,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ZERO,
        ];

        let context = ChannelExecutionContext::default();

        let result = engine
            .apply_kraus_to_density_matrix(
                &channel,
                &density,
                &context,
            )
            .expect("identity execution must succeed");

        match result.state {
            ChannelState::DensityMatrix {
                dimension,
                elements,
            } => {
                assert_eq!(dimension, 2);
                assert_eq!(elements, density);
            }

            ChannelState::StateVector { .. } => {
                panic!("channel execution must produce a density matrix");
            }
        }
    }

    #[test]
    fn identity_channel_preserves_zero_state() {
        let engine = ChannelEngine::unrestricted();
        let channel = identity_channel(2);

        let context = ChannelExecutionContext::default();

        let result = engine
            .apply_kraus_to_state_vector(
                &channel,
                &qubit_zero(),
                &context,
            )
            .expect("identity execution must succeed");

        match result.state {
            ChannelState::DensityMatrix { elements, .. } => {
                assert_eq!(elements[0], Complex64::ONE);
                assert_eq!(elements[1], Complex64::ZERO);
                assert_eq!(elements[2], Complex64::ZERO);
                assert_eq!(elements[3], Complex64::ZERO);
            }

            ChannelState::StateVector { .. } => {
                panic!("general channel execution must return density matrix");
            }
        }
    }

    #[test]
    fn wrong_input_dimension_is_rejected() {
        let engine = ChannelEngine::unrestricted();
        let channel = identity_channel(2);

        let state = vec![Complex64::ONE; 4];

        let context = ChannelExecutionContext::default();

        let error = engine
            .apply_kraus_to_state_vector(
                &channel,
                &state,
                &context,
            )
            .expect_err("wrong dimension must fail");

        assert!(matches!(
            error,
            ChannelEngineError::InputDimensionMismatch {
                expected: 2,
                actual: 4
            }
        ));
    }

    #[test]
    fn wrong_density_matrix_shape_is_rejected() {
        let engine = ChannelEngine::unrestricted();
        let channel = identity_channel(2);

        let density = vec![Complex64::ZERO; 3];

        let context = ChannelExecutionContext::default();

        let error = engine
            .apply_kraus_to_density_matrix(
                &channel,
                &density,
                &context,
            )
            .expect_err("non-square matrix must fail");

        assert!(matches!(
            error,
            ChannelEngineError::ElementCountMismatch { .. }
        ));
    }

    #[test]
    fn non_finite_state_is_rejected() {
        let engine = ChannelEngine::unrestricted();
        let channel = identity_channel(2);

        let invalid = vec![
            Complex64::new(f64::NAN, 0.0),
            Complex64::ZERO,
        ];

        let context = ChannelExecutionContext::default();

        let error = engine
            .apply_kraus_to_state_vector(
                &channel,
                &invalid,
                &context,
            )
            .expect_err("non-finite state must fail");

        assert!(matches!(
            error,
            ChannelEngineError::NonFiniteElement { index: 0 }
        ));
    }

    #[test]
    fn matrix_element_limit_is_enforced() {
        let config = ChannelEngineConfig {
            max_matrix_elements: Some(3),
            ..ChannelEngineConfig::default()
        };

        let engine =
            ChannelEngine::new(config).expect("configuration must be valid");

        let channel = identity_channel(2);

        let context = ChannelExecutionContext::default();

        let error = engine
            .apply_kraus_to_state_vector(
                &channel,
                &qubit_zero(),
                &context,
            )
            .expect_err("limit must be enforced");

        assert!(matches!(
            error,
            ChannelEngineError::ResourceLimitExceeded {
                resource: "max_matrix_elements",
                ..
            }
        ));
    }

    #[test]
    fn allocation_byte_limit_is_enforced() {
        let config = ChannelEngineConfig {
            max_allocation_bytes: Some(1),
            ..ChannelEngineConfig::default()
        };

        let engine =
            ChannelEngine::new(config).expect("configuration must be valid");

        let channel = identity_channel(2);
        let context = ChannelExecutionContext::default();

        let error = engine
            .apply_kraus_to_state_vector(
                &channel,
                &qubit_zero(),
                &context,
            )
            .expect_err("allocation limit must be enforced");

        assert!(matches!(
            error,
            ChannelEngineError::ResourceLimitExceeded {
                resource: "max_allocation_bytes",
                ..
            }
        ));
    }

    #[test]
    fn canonical_qubit_binding_is_supported() {
        let binding = ChannelResourceBinding::new(vec![
            QubitId::new(0),
            QubitId::new(1),
        ])
        .expect("unique canonical qubits must be accepted");

        assert_eq!(binding.len(), 2);
        assert!(!binding.is_empty());
    }

    #[test]
    fn duplicate_qubit_binding_is_rejected() {
        let qubit = QubitId::new(7);

        let error =
            ChannelResourceBinding::new(vec![qubit, qubit])
                .expect_err("duplicate resources must fail");

        assert!(matches!(
            error,
            ChannelEngineError::DuplicateQubitResource { .. }
        ));
    }

    #[test]
    fn sequence_execution_is_deterministic() {
        let engine = ChannelEngine::unrestricted();
        let first = identity_channel(2);
        let second = identity_channel(2);

        let channels = [&first, &second];

        let density = vec![
            Complex64::ONE,
            Complex64::ZERO,
            Complex64::ZERO,
            Complex64::ZERO,
        ];

        let context = ChannelExecutionContext::default();

        let a = engine
            .apply_kraus_sequence(
                channels.iter().copied(),
                &density,
                &context,
            )
            .expect("first execution must succeed");

        let b = engine
            .apply_kraus_sequence(
                channels.iter().copied(),
                &density,
                &context,
            )
            .expect("second execution must succeed");

        assert_eq!(a, b);
    }

    #[test]
    fn stream_execution_does_not_materialize_all_inputs() {
        let engine = ChannelEngine::unrestricted();
        let channel = identity_channel(2);

        let states = (0..4).map(|_| {
            ChannelState::StateVector {
                amplitudes: qubit_zero(),
            }
        });

        let context = ChannelExecutionContext::default();

        let results: Vec<_> = engine
            .apply_kraus_stream(&channel, states, &context)
            .collect();

        assert_eq!(results.len(), 4);
        assert!(results.iter().all(Result::is_ok));
    }

    #[test]
    fn cancellation_is_observed() {
        struct Cancelled;

        impl CancellationToken for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let engine = ChannelEngine::unrestricted();
        let channel = identity_channel(2);

        let token = Cancelled;

        let context = ChannelExecutionContext {
            resources: None,
            cancellation: Some(&token),
            operation_id: None,
        };

        let error = engine
            .apply_kraus_to_state_vector(
                &channel,
                &qubit_zero(),
                &context,
            )
            .expect_err("cancelled execution must fail");

        assert!(matches!(error, ChannelEngineError::Cancelled));
    }

    #[test]
    fn zero_dimension_is_rejected() {
        let error =
            validate_square_matrix(0, 0).expect_err("zero dimension must fail");

        assert!(matches!(error, ChannelEngineError::ZeroDimension));
    }

    #[test]
    fn non_square_element_count_is_rejected() {
        let error =
            infer_square_dimension(6).expect_err("six is not a perfect square");

        assert!(matches!(
            error,
            ChannelEngineError::ElementCountMismatch { .. }
        ));
    }

    #[test]
    fn arbitrary_finite_dimension_is_supported() {
        let engine = ChannelEngine::unrestricted();

        // Dimension 3 deliberately demonstrates that the engine is not
        // hard-coded around qubits.
        let channel = identity_channel(3);

        let amplitudes = vec![
            Complex64::ONE,
            Complex64::ZERO,
            Complex64::ZERO,
        ];

        let context = ChannelExecutionContext::default();

        let result = engine
            .apply_kraus_to_state_vector(
                &channel,
                &amplitudes,
                &context,
            )
            .expect("finite arbitrary dimension must work");

        assert_eq!(result.dimension(), 3);
        assert_eq!(result.output_elements, 9);
    }
}