//! Deterministic execution infrastructure for Zamani Quantum Error Correction.
//!
//! This module is the execution-level implementation of the determinism policy
//! declared by `configuration.rs`.
//!
//! Architectural contract:
//!
//! ```text
//! QecConfig
//!     │
//!     ├── QecLimits
//!     ├── DeterminismConfig
//!     ├── ParallelismConfig
//!     └── NumericalPolicy
//!          │
//!          ▼
//! DeterministicContext
//!     │
//!     ├── deterministic scheduling
//!     ├── deterministic worker assignment
//!     ├── deterministic ordering
//!     ├── deterministic reductions
//!     ├── reproducible RNG
//!     ├── canonical execution fingerprint
//!     ├── cancellation checkpoints
//!     └── resource-aware validation
//!          │
//!          ▼
//! Decoder / Streaming / Partition / Distributed / Simulation / Checkpoint
//! ```
//!
//! Determinism means that identical logical inputs and configuration produce
//! identical observable QEC results, regardless of worker execution order.
//!
//! It does NOT mean that arbitrary floating-point hardware is bit-for-bit
//! identical. Cross-platform numerical reproducibility must use the explicit
//! deterministic numerical primitives supplied here.
//!
//! This module deliberately:
//!
//! * contains no unsafe code;
//! * contains no process-global mutable state;
//! * never uses wall-clock time for deterministic ordering;
//! * uses checked arithmetic at policy boundaries;
//! * uses canonical ordering;
//! * uses explicit seeds;
//! * integrates with `QecConfig`;
//! * integrates with `CancellationToken`;
//! * respects `QecLimits`;
//! * exposes `QecError` at the public error boundary;
//! * does not silently convert resource-limited execution into success.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use super::cancellation::CancellationToken;
use super::configuration::{
    DeterminismConfig as QecDeterminismConfig,
    FloatingPointMode,
    QecConfig,
};
use super::errors::{QecError, QecResult};

// ============================================================================
// Version / constants
// ============================================================================

/// Deterministic execution API version.
pub const DETERMINISTIC_API_VERSION: &str = "2.0.0";

/// Size of an execution fingerprint.
pub const FINGERPRINT_SIZE: usize = 32;

/// Default deterministic seed used only when deterministic execution is
/// explicitly enabled without an externally supplied seed.
pub const DEFAULT_DETERMINISTIC_SEED: u64 = 0x5A4D_414E_4951_4543;

const MIX_1: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_2: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_3: u64 = 0x94D0_49BB_1331_11EB;

// ============================================================================
// Errors
// ============================================================================

/// Errors specific to deterministic execution infrastructure.
///
/// Public QEC APIs should normally expose these as `QecError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeterministicError {
    /// Arithmetic overflow occurred.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// Deterministic configuration is invalid.
    InvalidConfiguration {
        reason: String,
    },

    /// Configuration validation failed.
    ConfigurationValidation {
        message: String,
    },

    /// An identifier is invalid.
    InvalidIdentifier {
        value: u64,
    },

    /// An identifier that must be unique was duplicated.
    DuplicateIdentifier {
        value: u64,
    },

    /// An operation requiring input received none.
    EmptyInput {
        operation: &'static str,
    },

    /// A deterministic sequence cannot advance.
    SequenceExhausted,

    /// Worker count is invalid.
    InvalidWorkerCount {
        workers: usize,
    },

    /// Worker identifier is invalid.
    InvalidWorkerId {
        worker_id: usize,
        workers: usize,
    },

    /// A partition count is invalid.
    InvalidPartitionCount {
        partitions: usize,
    },

    /// Integer index conversion failed.
    IndexOverflow,

    /// Non-finite floating-point input.
    InvalidFloatingPoint {
        operation: &'static str,
    },

    /// A required deterministic invariant was violated.
    InvariantViolation {
        invariant: &'static str,
    },
}

impl fmt::Display for DeterministicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArithmeticOverflow { operation } => {
                write!(f, "deterministic arithmetic overflow: {operation}")
            }

            Self::InvalidConfiguration { reason } => {
                write!(f, "invalid deterministic configuration: {reason}")
            }

            Self::ConfigurationValidation { message } => {
                write!(f, "QEC configuration validation failed: {message}")
            }

            Self::InvalidIdentifier { value } => {
                write!(f, "invalid deterministic identifier: {value}")
            }

            Self::DuplicateIdentifier { value } => {
                write!(f, "duplicate deterministic identifier: {value}")
            }

            Self::EmptyInput { operation } => {
                write!(
                    f,
                    "empty input is invalid for deterministic operation: {operation}"
                )
            }

            Self::SequenceExhausted => {
                write!(f, "deterministic sequence exhausted")
            }

            Self::InvalidWorkerCount { workers } => {
                write!(f, "invalid deterministic worker count: {workers}")
            }

            Self::InvalidWorkerId {
                worker_id,
                workers,
            } => {
                write!(
                    f,
                    "invalid worker id {worker_id}; configured workers: {workers}"
                )
            }

            Self::InvalidPartitionCount { partitions } => {
                write!(f, "invalid deterministic partition count: {partitions}")
            }

            Self::IndexOverflow => {
                write!(f, "deterministic index conversion overflow")
            }

            Self::InvalidFloatingPoint { operation } => {
                write!(
                    f,
                    "invalid floating-point value for deterministic operation: {operation}"
                )
            }

            Self::InvariantViolation { invariant } => {
                write!(
                    f,
                    "deterministic invariant violated: {invariant}"
                )
            }
        }
    }
}

impl std::error::Error for DeterministicError {}

impl From<DeterministicError> for QecError {
    fn from(error: DeterministicError) -> Self {
        match error {
            DeterministicError::ArithmeticOverflow { operation } => {
                QecError::NumericalFailure {
                    operation: super::errors::NumericalOperation::Arithmetic,
                    message: format!(
                        "deterministic arithmetic overflow: {operation}"
                    ),
                }
            }

            DeterministicError::InvalidFloatingPoint { operation } => {
                QecError::NumericalFailure {
                    operation: super::errors::NumericalOperation::FloatingPoint,
                    message: format!(
                        "invalid floating-point value: {operation}"
                    ),
                }
            }

            DeterministicError::ConfigurationValidation { message } => {
                QecError::InvalidInput { message }
            }

            DeterministicError::InvalidConfiguration { reason } => {
                QecError::UnsupportedConfiguration {
                    feature: "deterministic_execution".to_string(),
                    message: reason,
                }
            }

            DeterministicError::SequenceExhausted => {
                QecError::InternalInvariantViolation {
                    invariant: "deterministic_sequence_must_not_wrap",
                    message: "deterministic sequence exhausted".to_string(),
                }
            }

            DeterministicError::InvariantViolation { invariant } => {
                QecError::InternalInvariantViolation {
                    invariant: invariant.to_string(),
                    message: "deterministic execution invariant violated"
                        .to_string(),
                }
            }

            other => QecError::InvalidInput {
                message: other.to_string(),
            },
        }
    }
}

/// Result type for internal deterministic operations.
pub type DeterministicResult<T> = Result<T, DeterministicError>;

// ============================================================================
// Determinism mode
// ============================================================================

/// Execution-level determinism mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterminismMode {
    /// Determinism is disabled.
    Disabled,

    /// Deterministic scheduling, ordering and reductions are required.
    Deterministic,

    /// Determinism is enabled and numerical operations are additionally
    /// validated strictly.
    Strict,
}

impl Default for DeterminismMode {
    fn default() -> Self {
        Self::Disabled
    }
}

// ============================================================================
// Deterministic runtime configuration
// ============================================================================

/// Runtime configuration derived from the repository's `QecConfig`.
///
/// This is intentionally separate from `configuration::DeterminismConfig`.
/// `configuration.rs` owns policy; this structure contains the validated
/// execution state needed by the deterministic engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicRuntimeConfig {
    /// Execution mode.
    pub mode: DeterminismMode,

    /// Reproducible root seed.
    pub seed: u64,

    /// Number of logical workers.
    pub worker_count: usize,

    /// Whether scheduling must be canonical.
    pub deterministic_scheduling: bool,

    /// Whether reductions must be canonical.
    pub deterministic_reductions: bool,

    /// Whether serialized ordering must be canonical.
    pub deterministic_serialization: bool,

    /// Whether execution fingerprints are required.
    pub require_fingerprint: bool,
}

impl Default for DeterministicRuntimeConfig {
    fn default() -> Self {
        Self {
            mode: DeterminismMode::Disabled,
            seed: DEFAULT_DETERMINISTIC_SEED,
            worker_count: 1,
            deterministic_scheduling: true,
            deterministic_reductions: true,
            deterministic_serialization: true,
            require_fingerprint: true,
        }
    }
}

impl DeterministicRuntimeConfig {
    /// Creates deterministic runtime configuration from `QecConfig`.
    pub fn from_qec_config(
        config: &QecConfig,
    ) -> DeterministicResult<Self> {
        config
            .validate()
            .map_err(|error| {
                DeterministicError::ConfigurationValidation {
                    message: error.to_string(),
                }
            })?;

        let policy = &config.determinism;

        let mode = if !policy.enabled {
            DeterminismMode::Disabled
        } else if config.numerical.floating_point_mode
            == FloatingPointMode::Strict
        {
            DeterminismMode::Strict
        } else {
            DeterminismMode::Deterministic
        };

        let worker_count = usize::try_from(
            config.parallelism.max_workers,
        )
        .map_err(|_| DeterministicError::IndexOverflow)?;

        if worker_count == 0 {
            return Err(
                DeterministicError::InvalidWorkerCount {
                    workers: worker_count,
                },
            );
        }

        let seed = policy
            .seed
            .unwrap_or(DEFAULT_DETERMINISTIC_SEED);

        let runtime = Self {
            mode,
            seed,
            worker_count,
            deterministic_scheduling:
                policy.deterministic_scheduling,
            deterministic_reductions:
                policy.deterministic_reductions,
            deterministic_serialization:
                policy.deterministic_serialization,
            require_fingerprint: policy.enabled,
        };

        runtime.validate()?;
        Ok(runtime)
    }

    /// Validates the runtime configuration.
    pub fn validate(&self) -> DeterministicResult<()> {
        if self.worker_count == 0 {
            return Err(
                DeterministicError::InvalidWorkerCount {
                    workers: 0,
                },
            );
        }

        if self.mode != DeterminismMode::Disabled {
            if !self.deterministic_scheduling {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic scheduling"
                                .to_string(),
                    },
                );
            }

            if !self.deterministic_reductions {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic reductions"
                                .to_string(),
                    },
                );
            }

            if !self.deterministic_serialization {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic serialization"
                                .to_string(),
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Backward-compatible deterministic configuration
// ============================================================================

/// Standalone deterministic configuration.
///
/// Prefer `DeterministicRuntimeConfig::from_qec_config()` for production
/// execution. This type remains useful for unit tests and low-level primitives.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicConfig {
    pub mode: DeterminismMode,
    pub seed: u64,
    pub algorithm_id: u64,
    pub algorithm_version: u64,
    pub worker_count: usize,
    pub deterministic_worker_assignment: bool,
    pub deterministic_reduction: bool,
    pub require_fingerprint: bool,
}

impl Default for DeterministicConfig {
    fn default() -> Self {
        Self {
            mode: DeterminismMode::Deterministic,
            seed: DEFAULT_DETERMINISTIC_SEED,
            algorithm_id: 0,
            algorithm_version: 1,
            worker_count: 1,
            deterministic_worker_assignment: true,
            deterministic_reduction: true,
            require_fingerprint: true,
        }
    }
}

impl DeterministicConfig {
    pub fn validate(&self) -> DeterministicResult<()> {
        if self.worker_count == 0 {
            return Err(
                DeterministicError::InvalidWorkerCount {
                    workers: 0,
                },
            );
        }

        if self.mode != DeterminismMode::Disabled {
            if !self.deterministic_worker_assignment {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic worker assignment"
                                .to_string(),
                    },
                );
            }

            if !self.deterministic_reduction {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic reduction"
                                .to_string(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Converts the standalone configuration into runtime configuration.
    pub fn runtime(&self) -> DeterministicResult<DeterministicRuntimeConfig> {
        let runtime = DeterministicRuntimeConfig {
            mode: self.mode,
            seed: self.seed,
            worker_count: self.worker_count,
            deterministic_scheduling:
                self.deterministic_worker_assignment,
            deterministic_reductions:
                self.deterministic_reduction,
            deterministic_serialization: true,
            require_fingerprint: self.require_fingerprint,
        };

        runtime.validate()?;
        Ok(runtime)
    }
}

// ============================================================================
// Deterministic sequence
// ============================================================================

/// Monotonically increasing deterministic sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeterministicSequence {
    next: u64,
}

impl Default for DeterministicSequence {
    fn default() -> Self {
        Self::new()
    }
}

impl DeterministicSequence {
    pub const fn new() -> Self {
        Self { next: 0 }
    }

    pub const fn from(start: u64) -> Self {
        Self { next: start }
    }

    pub fn next(&mut self) -> DeterministicResult<u64> {
        let value = self.next;

        self.next = self
            .next
            .checked_add(1)
            .ok_or(DeterministicError::SequenceExhausted)?;

        Ok(value)
    }

    pub const fn peek(&self) -> u64 {
        self.next
    }
}

// ============================================================================
// Reproducible RNG
// ============================================================================

/// Reproducible non-cryptographic pseudo-random generator.
///
/// Never use this generator for secrets, credentials, cryptographic keys,
/// capability tokens, or security decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    pub const fn new(seed: u64) -> Self {
        Self {
            state: splitmix64(seed),
        }
    }

    pub const fn from_state(state: u64) -> Self {
        Self { state }
    }

    pub const fn state(&self) -> u64 {
        self.state
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state =
            splitmix64(self.state.wrapping_add(MIX_1));
        self.state
    }

    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) != 0
    }

    /// Uniform value in `[0, upper)`.
    pub fn next_bounded(
        &mut self,
        upper: u64,
    ) -> DeterministicResult<u64> {
        if upper == 0 {
            return Err(
                DeterministicError::InvalidConfiguration {
                    reason:
                        "random upper bound must be greater than zero"
                            .to_string(),
                },
            );
        }

        /*
         * Rejection sampling avoids modulo bias.
         *
         * The previous implementation used:
         *
         *   u64::MAX - (u64::MAX % upper)
         *
         * which excludes a slightly incorrect range at the boundary.
         *
         * Using `wrapping_neg()` gives the exact largest multiple zone.
         */
        let threshold =
            upper.wrapping_neg() % upper;

        loop {
            let value = self.next_u64();

            if value >= threshold {
                return Ok(value % upper);
            }
        }
    }

    /// Generates a reproducible finite value in `[0, 1)`.
    pub fn next_unit_f64(&mut self) -> f64 {
        let value = self.next_u64() >> 11;
        value as f64 / ((1_u64 << 53) as f64)
    }
}

// ============================================================================
// Stable deterministic hashing
// ============================================================================

/// Stable non-cryptographic hashing state.
///
/// This is intended for execution identity and reproducibility, not
/// cryptographic integrity. Checkpoint integrity belongs to `checkpoint.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StableHasher {
    state: u64,
}

impl Default for StableHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl StableHasher {
    pub const fn new() -> Self {
        Self {
            state: 0x243F_6A88_85A3_08D3,
        }
    }

    pub const fn with_seed(seed: u64) -> Self {
        Self {
            state: seed ^ 0x243F_6A88_85A3_08D3,
        }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state ^= u64::from(*byte);
            self.state = self.state.wrapping_mul(MIX_2);
            self.state ^= self.state >> 29;
            self.state = self.state.rotate_left(17);
        }
    }

    pub fn update_u8(&mut self, value: u8) {
        self.update(&[value]);
    }

    pub fn update_bool(&mut self, value: bool) {
        self.update_u8(u8::from(value));
    }

    pub fn update_u64(&mut self, value: u64) {
        self.update(&value.to_le_bytes());
    }

    pub fn update_i64(&mut self, value: i64) {
        self.update_u64(value as u64);
    }

    pub fn update_str(&mut self, value: &str) {
        self.update_u64(value.len() as u64);
        self.update(value.as_bytes());
    }

    pub fn finish_u64(&self) -> u64 {
        avalanche(self.state)
    }

    pub fn finish(&self) -> ExecutionFingerprint {
        ExecutionFingerprint::from_seed(
            self.finish_u64(),
        )
    }
}

// ============================================================================
// Execution fingerprint
// ============================================================================

/// Fixed-size deterministic execution fingerprint.
///
/// This is an execution identity, not a cryptographic authentication token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionFingerprint(
    [u8; FINGERPRINT_SIZE],
);

impl ExecutionFingerprint {
    fn from_seed(seed: u64) -> Self {
        let mut output = [0_u8; FINGERPRINT_SIZE];
        let mut state = seed;

        for chunk in output.chunks_exact_mut(8) {
            state =
                splitmix64(state.wrapping_add(MIX_3));

            chunk.copy_from_slice(
                &state.to_le_bytes(),
            );
        }

        Self(output)
    }

    pub const fn as_bytes(
        &self,
    ) -> &[u8; FINGERPRINT_SIZE] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        let mut output =
            String::with_capacity(FINGERPRINT_SIZE * 2);

        for byte in self.0 {
            output.push(hex_digit(byte >> 4));
            output.push(hex_digit(byte));
        }

        output
    }
}

impl fmt::Display for ExecutionFingerprint {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

// ============================================================================
// Deterministic execution context
// ============================================================================

/// Execution-scoped deterministic state.
///
/// One context should normally be created per QEC job and passed through the
/// complete execution pipeline.
#[derive(Debug, Clone)]
pub struct DeterministicContext {
    config: DeterministicConfig,
    rng: DeterministicRng,
    sequence: DeterministicSequence,
    hasher: StableHasher,
}

impl DeterministicContext {
    pub fn new(
        config: DeterministicConfig,
    ) -> DeterministicResult<Self> {
        config.validate()?;

        let mut hasher =
            StableHasher::with_seed(config.seed);

        hasher.update_str(
            DETERMINISTIC_API_VERSION,
        );
        hasher.update_u64(config.seed);
        hasher.update_u64(config.algorithm_id);
        hasher.update_u64(config.algorithm_version);
        hasher.update_u64(
            config.worker_count as u64,
        );
        hasher.update_bool(
            config.deterministic_worker_assignment,
        );
        hasher.update_bool(
            config.deterministic_reduction,
        );

        let rng = DeterministicRng::new(
            derive_seed(
                config.seed,
                config.algorithm_id,
                config.algorithm_version,
            ),
        );

        Ok(Self {
            config,
            rng,
            sequence: DeterministicSequence::new(),
            hasher,
        })
    }

    /// Creates an execution context directly from the repository's validated
    /// QecConfig.
    pub fn from_qec_config(
        config: &QecConfig,
    ) -> DeterministicResult<Self> {
        let runtime =
            DeterministicRuntimeConfig::from_qec_config(
                config,
            )?;

        let standalone =
            DeterministicConfig {
                mode: runtime.mode,
                seed: runtime.seed,
                algorithm_id: 0,
                algorithm_version: 1,
                worker_count:
                    runtime.worker_count,
                deterministic_worker_assignment:
                    runtime.deterministic_scheduling,
                deterministic_reduction:
                    runtime.deterministic_reductions,
                require_fingerprint:
                    runtime.require_fingerprint,
            };

        Self::new(standalone)
    }

    pub const fn config(
        &self,
    ) -> &DeterministicConfig {
        &self.config
    }

    pub fn next_sequence(
        &mut self,
    ) -> DeterministicResult<u64> {
        self.sequence.next()
    }

    pub fn next_sequence_checked(
        &mut self,
        cancellation: Option<&CancellationToken>,
    ) -> QecResult<u64> {
        check_cancellation(cancellation)?;
        self.sequence.next().map_err(QecError::from)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn next_bool(&mut self) -> bool {
        self.rng.next_bool()
    }

    pub fn next_bounded(
        &mut self,
        upper: u64,
    ) -> DeterministicResult<u64> {
        self.rng.next_bounded(upper)
    }

    pub fn next_bounded_checked(
        &mut self,
        upper: u64,
        cancellation: Option<&CancellationToken>,
    ) -> QecResult<u64> {
        check_cancellation(cancellation)?;

        self.rng
            .next_bounded(upper)
            .map_err(QecError::from)
    }

    pub fn record_bytes(
        &mut self,
        bytes: &[u8],
    ) {
        record_bytes(&mut self.hasher, bytes);
    }

    pub fn record_u64(
        &mut self,
        value: u64,
    ) {
        self.hasher.update_u64(value);
    }

    pub fn record_bool(
        &mut self,
        value: bool,
    ) {
        self.hasher.update_bool(value);
    }

    pub fn record_str(
        &mut self,
        value: &str,
    ) {
        self.hasher.update_str(value);
    }

    pub fn fingerprint(
        &self,
    ) -> ExecutionFingerprint {
        self.hasher.finish()
    }

    pub const fn sequence_position(
        &self,
    ) -> u64 {
        self.sequence.peek()
    }

    pub const fn rng_state(
        &self,
    ) -> u64 {
        self.rng.state()
    }

    pub const fn worker_count(
        &self,
    ) -> usize {
        self.config.worker_count
    }

    pub fn check_cancellation(
        &self,
        cancellation: Option<&CancellationToken>,
    ) -> QecResult<()> {
        check_cancellation(cancellation)
    }
}

// ============================================================================
// Deterministic events
// ============================================================================

/// Canonically sortable QEC event.
///
/// Ordering is based only on deterministic metadata and payload ordering.
/// Arrival time and thread execution order are deliberately absent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicEvent<T> {
    pub id: u64,
    pub round: u64,
    pub partition: u64,
    pub payload: T,
}

impl<T> DeterministicEvent<T> {
    pub const fn new(
        id: u64,
        round: u64,
        partition: u64,
        payload: T,
    ) -> Self {
        Self {
            id,
            round,
            partition,
            payload,
        }
    }

    pub fn map<U, F>(
        self,
        function: F,
    ) -> DeterministicEvent<U>
    where
        F: FnOnce(T) -> U,
    {
        DeterministicEvent {
            id: self.id,
            round: self.round,
            partition: self.partition,
            payload: function(self.payload),
        }
    }
}

impl<T> Ord for DeterministicEvent<T>
where
    T: Ord,
{
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.round
            .cmp(&other.round)
            .then_with(|| {
                self.partition
                    .cmp(&other.partition)
            })
            .then_with(|| self.id.cmp(&other.id))
            .then_with(|| {
                self.payload.cmp(&other.payload)
            })
    }
}

impl<T> PartialOrd for DeterministicEvent<T>
where
    T: Ord,
{
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Canonically sorts events.
///
/// This is the required boundary before deterministic reductions of
/// concurrently produced events.
pub fn canonicalize_events<T>(
    events: &mut [DeterministicEvent<T>],
) where
    T: Ord,
{
    events.sort_unstable();
}

/// Canonically sorts events while checking cancellation.
pub fn canonicalize_events_checked<T>(
    events: &mut [DeterministicEvent<T>],
    cancellation: Option<&CancellationToken>,
) -> QecResult<()>
where
    T: Ord,
{
    check_cancellation(cancellation)?;

    events.sort_unstable();

    check_cancellation(cancellation)
}

// ============================================================================
// Worker / partition assignment
// ============================================================================

/// Deterministically assigns an identifier to a worker.
///
/// This assignment is a routing decision only. The decoder must still
/// canonicalize results before reducing them.
pub fn assign_worker(
    identifier: u64,
    workers: usize,
) -> DeterministicResult<usize> {
    if workers == 0 {
        return Err(
            DeterministicError::InvalidWorkerCount {
                workers,
            },
        );
    }

    let mixed = splitmix64(identifier);

    Ok((mixed % workers as u64) as usize)
}

/// Assigns a worker while respecting a configured worker limit.
pub fn assign_worker_checked(
    identifier: u64,
    workers: usize,
    configured_max_workers: u32,
) -> QecResult<usize> {
    if workers == 0 {
        return Err(QecError::InvalidInput {
            message:
                "worker count must be greater than zero"
                    .to_string(),
        });
    }

    let configured =
        usize::try_from(configured_max_workers)
            .map_err(|_| {
                QecError::InvalidInput {
                    message:
                        "configured worker limit cannot be represented"
                            .to_string(),
                }
            })?;

    if workers > configured {
        return Err(QecError::ResourceLimitExceeded {
            resource:
                super::errors::ResourceKind::Parallelism,
            requested: workers as u128,
            current: 0,
            limit: configured as u128,
            message:
                "deterministic worker request exceeds configured QEC parallelism limit"
                    .to_string(),
        });
    }

    assign_worker(identifier, workers)
        .map_err(QecError::from)
}

pub fn validate_worker(
    worker_id: usize,
    workers: usize,
) -> DeterministicResult<()> {
    if workers == 0 {
        return Err(
            DeterministicError::InvalidWorkerCount {
                workers,
            },
        );
    }

    if worker_id >= workers {
        return Err(
            DeterministicError::InvalidWorkerId {
                worker_id,
                workers,
            },
        );
    }

    Ok(())
}

pub fn assign_partition(
    identifier: u64,
    partitions: usize,
) -> DeterministicResult<usize> {
    if partitions == 0 {
        return Err(
            DeterministicError::InvalidPartitionCount {
                partitions,
            },
        );
    }

    assign_worker(identifier, partitions)
}

// ============================================================================
// Deterministic reductions
// ============================================================================

/// Reduces values in canonical input order.
///
/// Callers must canonicalize parallel/distributed results before invoking
/// this function.
pub fn deterministic_reduce<T, F>(
    values: &[T],
    mut initial: T,
    mut operation: F,
) -> T
where
    F: FnMut(T, &T) -> T,
{
    for value in values {
        initial = operation(initial, value);
    }

    initial
}

/// Checked deterministic reduction with cancellation.
pub fn deterministic_reduce_checked<T, F>(
    values: &[T],
    mut initial: T,
    mut operation: F,
    cancellation: Option<&CancellationToken>,
) -> QecResult<T>
where
    F: FnMut(T, &T) -> T,
{
    for value in values {
        check_cancellation(cancellation)?;
        initial = operation(initial, value);
    }

    Ok(initial)
}

/// Deterministically sums finite floating-point values.
///
/// Neumaier compensation reduces numerical sensitivity while preserving a
/// fixed evaluation order.
pub fn deterministic_sum_f64(
    values: &[f64],
) -> DeterministicResult<f64> {
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;

    for value in values {
        if !value.is_finite() {
            return Err(
                DeterministicError::InvalidFloatingPoint {
                    operation:
                        "deterministic_sum_f64 input",
                },
            );
        }

        let corrected = *value - compensation;
        let temporary = sum + corrected;

        compensation =
            (temporary - sum) - corrected;

        sum = temporary;
    }

    if !sum.is_finite() {
        return Err(
            DeterministicError::InvalidFloatingPoint {
                operation:
                    "deterministic_sum_f64 result",
            },
        );
    }

    Ok(sum)
}

/// Deterministic mean.
pub fn deterministic_mean_f64(
    values: &[f64],
) -> DeterministicResult<f64> {
    if values.is_empty() {
        return Err(
            DeterministicError::EmptyInput {
                operation:
                    "deterministic_mean_f64",
            },
        );
    }

    let sum =
        deterministic_sum_f64(values)?;

    let count =
        values.len() as f64;

    let result = sum / count;

    if !result.is_finite() {
        return Err(
            DeterministicError::InvalidFloatingPoint {
                operation:
                    "deterministic_mean_f64 result",
            },
        );
    }

    Ok(result)
}

/// Deterministically reduces floating-point values with cancellation support.
pub fn deterministic_sum_f64_checked(
    values: &[f64],
    cancellation: Option<&CancellationToken>,
) -> QecResult<f64> {
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;

    for value in values {
        check_cancellation(cancellation)?;

        if !value.is_finite() {
            return Err(
                DeterministicError::InvalidFloatingPoint {
                    operation:
                        "deterministic_sum_f64_checked input",
                }
                .into(),
            );
        }

        let corrected = *value - compensation;
        let temporary = sum + corrected;

        compensation =
            (temporary - sum) - corrected;

        sum = temporary;
    }

    if !sum.is_finite() {
        return Err(
            DeterministicError::InvalidFloatingPoint {
                operation:
                    "deterministic_sum_f64_checked result",
            }
            .into(),
        );
    }

    Ok(sum)
}

// ============================================================================
// Deterministic maps
// ============================================================================

/// Deterministic map backed by `BTreeMap`.
///
/// Never replace this with an unordered map when iteration order contributes
/// to decoder output, checkpoint identity, metrics or replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicMap<K, V> {
    values: BTreeMap<K, V>,
}

impl<K, V> Default for DeterministicMap<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> DeterministicMap<K, V>
where
    K: Ord,
{
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        key: K,
        value: V,
    ) -> Option<V> {
        self.values.insert(key, value)
    }

    pub fn get(
        &self,
        key: &K,
    ) -> Option<&V> {
        self.values.get(key)
    }

    pub fn remove(
        &mut self,
        key: &K,
    ) -> Option<V> {
        self.values.remove(key)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&K, &V)> {
        self.values.iter()
    }
}

// ============================================================================
// Canonical recording
// ============================================================================

/// Records bytes using length-prefixing.
pub fn record_bytes(
    hasher: &mut StableHasher,
    bytes: &[u8],
) {
    hasher.update_u64(bytes.len() as u64);
    hasher.update(bytes);
}

/// Records a deterministic event using its stable metadata.
///
/// Payload bytes should preferably be recorded separately using an explicit
/// canonical representation. This function intentionally does not depend on
/// Rust's `Hash` implementation because `Hash` is not a cross-language wire
/// format.
pub fn record_event_metadata<T>(
    hasher: &mut StableHasher,
    event: &DeterministicEvent<T>,
) {
    hasher.update_u64(event.id);
    hasher.update_u64(event.round);
    hasher.update_u64(event.partition);
}

// ============================================================================
// Checked arithmetic
// ============================================================================

pub fn checked_mul_u64(
    left: u64,
    right: u64,
) -> DeterministicResult<u64> {
    left.checked_mul(right)
        .ok_or(
            DeterministicError::ArithmeticOverflow {
                operation: "u64 multiplication",
            },
        )
}

pub fn checked_add_u64(
    left: u64,
    right: u64,
) -> DeterministicResult<u64> {
    left.checked_add(right)
        .ok_or(
            DeterministicError::ArithmeticOverflow {
                operation: "u64 addition",
            },
        )
}

pub fn checked_sub_u64(
    left: u64,
    right: u64,
) -> DeterministicResult<u64> {
    left.checked_sub(right)
        .ok_or(
            DeterministicError::ArithmeticOverflow {
                operation: "u64 subtraction",
            },
        )
}

pub fn usize_to_u64(
    value: usize,
) -> DeterministicResult<u64> {
    u64::try_from(value)
        .map_err(|_| {
            DeterministicError::IndexOverflow
        })
}

// ============================================================================
// Cancellation integration
// ============================================================================

/// Canonical cancellation boundary used by deterministic operations.
///
/// Expensive deterministic operations should call this periodically rather
/// than relying only on their caller to check cancellation.
pub fn check_cancellation(
    cancellation: Option<&CancellationToken>,
) -> QecResult<()> {
    if let Some(token) = cancellation {
        token.check()?;
    }

    Ok(())
}

// ============================================================================
// Execution fingerprint
// ============================================================================

/// Creates a reproducibility fingerprint from the complete execution identity.
///
/// The caller is responsible for supplying canonical digests of:
///
/// * code/topology;
/// * noise configuration;
/// * syndrome stream;
/// * decoder configuration;
/// * resource configuration.
///
/// Resource policy is deliberately included because changing resource policy
/// can change whether an execution terminates successfully.
pub fn execution_fingerprint(
    config: &DeterministicConfig,
    code_digest: u64,
    noise_digest: u64,
    syndrome_digest: u64,
    decoder_digest: u64,
    resource_digest: u64,
) -> DeterministicResult<ExecutionFingerprint> {
    config.validate()?;

    let mut hasher =
        StableHasher::with_seed(config.seed);

    hasher.update_str(
        DETERMINISTIC_API_VERSION,
    );

    hasher.update_u64(config.seed);
    hasher.update_u64(config.algorithm_id);
    hasher.update_u64(config.algorithm_version);
    hasher.update_u64(
        config.worker_count as u64,
    );

    hasher.update_bool(
        config.deterministic_worker_assignment,
    );
    hasher.update_bool(
        config.deterministic_reduction,
    );

    hasher.update_u64(code_digest);
    hasher.update_u64(noise_digest);
    hasher.update_u64(syndrome_digest);
    hasher.update_u64(decoder_digest);
    hasher.update_u64(resource_digest);

    Ok(hasher.finish())
}

/// QecConfig-aware execution fingerprint.
///
/// This should be preferred by high-level QEC execution code.
pub fn execution_fingerprint_from_qec_config(
    config: &QecConfig,
    code_digest: u64,
    noise_digest: u64,
    syndrome_digest: u64,
    decoder_digest: u64,
) -> DeterministicResult<ExecutionFingerprint> {
    let runtime =
        DeterministicRuntimeConfig::from_qec_config(
            config,
        )?;

    let mut hasher =
        StableHasher::with_seed(runtime.seed);

    hasher.update_str(
        DETERMINISTIC_API_VERSION,
    );

    hasher.update_u64(runtime.seed);
    hasher.update_u64(
        runtime.worker_count as u64,
    );
    hasher.update_bool(
        runtime.deterministic_scheduling,
    );
    hasher.update_bool(
        runtime.deterministic_reductions,
    );
    hasher.update_bool(
        runtime.deterministic_serialization,
    );

    /*
     * Include relevant numerical policy because changing floating-point
     * behavior can legitimately change a decoder's numerical result.
     */
    hasher.update_u64(
        config
            .numerical
            .probability_epsilon
            .to_bits(),
    );

    hasher.update_u64(
        config
            .numerical
            .weight_epsilon
            .to_bits(),
    );

    hasher.update_u64(code_digest);
    hasher.update_u64(noise_digest);
    hasher.update_u64(syndrome_digest);
    hasher.update_u64(decoder_digest);

    Ok(hasher.finish())
}

// ============================================================================
// Utility functions
// ============================================================================

fn splitmix64(
    mut value: u64,
) -> u64 {
    value = value.wrapping_add(MIX_1);

    value = (value ^ (value >> 30))
        .wrapping_mul(MIX_2);

    value = (value ^ (value >> 27))
        .wrapping_mul(MIX_3);

    value ^ (value >> 31)
}

fn derive_seed(
    seed: u64,
    algorithm_id: u64,
    algorithm_version: u64,
) -> u64 {
    let mut value = seed;

    value ^=
        algorithm_id.rotate_left(17);

    value = splitmix64(value);

    value ^=
        algorithm_version.rotate_left(31);

    splitmix64(value)
}

fn avalanche(
    mut value: u64,
) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(MIX_2);

    value ^= value >> 27;
    value = value.wrapping_mul(MIX_3);

    value ^ (value >> 31)
}

fn hex_digit(
    value: u8,
) -> char {
    match value & 0x0f {
        0..=9 => {
            (b'0' + (value & 0x0f)) as char
        }

        10..=15 => {
            (b'a' + ((value & 0x0f) - 10))
                as char
        }

        _ => unreachable!(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        assert!(
            DeterministicConfig::default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn zero_workers_are_rejected() {
        let config =
            DeterministicConfig {
                worker_count: 0,
                ..Default::default()
            };

        assert!(matches!(
            config.validate(),
            Err(
                DeterministicError::InvalidWorkerCount {
                    workers: 0
                }
            )
        ));
    }

    #[test]
    fn deterministic_rng_replays_exactly() {
        let mut left =
            DeterministicRng::new(12345);

        let mut right =
            DeterministicRng::new(12345);

        for _ in 0..1_000 {
            assert_eq!(
                left.next_u64(),
                right.next_u64()
            );
        }
    }

    #[test]
    fn different_seeds_produce_different_sequences() {
        let mut left =
            DeterministicRng::new(1);

        let mut right =
            DeterministicRng::new(2);

        assert_ne!(
            left.next_u64(),
            right.next_u64()
        );
    }

    #[test]
    fn bounded_rng_never_exceeds_bound() {
        let mut rng =
            DeterministicRng::new(123);

        for _ in 0..10_000 {
            let value =
                rng.next_bounded(17).unwrap();

            assert!(value < 17);
        }
    }

    #[test]
    fn zero_random_bound_is_rejected() {
        let mut rng =
            DeterministicRng::new(1);

        assert!(matches!(
            rng.next_bounded(0),
            Err(
                DeterministicError::InvalidConfiguration {
                    ..
                }
            )
        ));
    }

    #[test]
    fn sequence_is_monotonic() {
        let mut sequence =
            DeterministicSequence::new();

        assert_eq!(
            sequence.next().unwrap(),
            0
        );

        assert_eq!(
            sequence.next().unwrap(),
            1
        );

        assert_eq!(
            sequence.next().unwrap(),
            2
        );
    }

    #[test]
    fn canonical_event_order_is_independent_of_input_order() {
        let mut left = vec![
            DeterministicEvent::new(
                2, 1, 0, 20_u64
            ),
            DeterministicEvent::new(
                1, 1, 0, 10_u64
            ),
            DeterministicEvent::new(
                3, 0, 0, 30_u64
            ),
        ];

        let mut right = vec![
            left[2].clone(),
            left[0].clone(),
            left[1].clone(),
        ];

        canonicalize_events(&mut left);
        canonicalize_events(&mut right);

        assert_eq!(left, right);
    }

    #[test]
    fn worker_assignment_is_reproducible() {
        for id in 0..1_000 {
            assert_eq!(
                assign_worker(id, 8).unwrap(),
                assign_worker(id, 8).unwrap()
            );
        }
    }

    #[test]
    fn worker_assignment_rejects_zero_workers() {
        assert!(matches!(
            assign_worker(1, 0),
            Err(
                DeterministicError::InvalidWorkerCount {
                    workers: 0
                }
            )
        ));
    }

    #[test]
    fn worker_validation_rejects_out_of_range_worker() {
        assert!(matches!(
            validate_worker(8, 8),
            Err(
                DeterministicError::InvalidWorkerId {
                    worker_id: 8,
                    workers: 8
                }
            )
        ));
    }

    #[test]
    fn deterministic_reduction_is_ordered() {
        let values = [1_u64, 2, 3, 4];

        let result =
            deterministic_reduce(
                &values,
                0_u64,
                |a, b| a + b,
            );

        assert_eq!(result, 10);
    }

    #[test]
    fn deterministic_float_sum_rejects_nan() {
        let values =
            [1.0_f64, f64::NAN];

        assert!(matches!(
            deterministic_sum_f64(&values),
            Err(
                DeterministicError::InvalidFloatingPoint {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_float_sum_rejects_infinity() {
        let values =
            [1.0_f64, f64::INFINITY];

        assert!(matches!(
            deterministic_sum_f64(&values),
            Err(
                DeterministicError::InvalidFloatingPoint {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_mean_rejects_empty_input() {
        assert!(matches!(
            deterministic_mean_f64(&[]),
            Err(
                DeterministicError::EmptyInput {
                    ..
                }
            )
        ));
    }

    #[test]
    fn fingerprints_are_reproducible() {
        let config =
            DeterministicConfig {
                seed: 42,
                algorithm_id: 7,
                algorithm_version: 3,
                worker_count: 4,
                ..Default::default()
            };

        let left =
            execution_fingerprint(
                &config,
                1,
                2,
                3,
                4,
                5,
            )
            .unwrap();

        let right =
            execution_fingerprint(
                &config,
                1,
                2,
                3,
                4,
                5,
            )
            .unwrap();

        assert_eq!(left, right);
        assert_eq!(left.to_hex().len(), 64);
    }

    #[test]
    fn fingerprint_changes_when_resource_policy_changes() {
        let config =
            DeterministicConfig::default();

        let left =
            execution_fingerprint(
                &config,
                1,
                2,
                3,
                4,
                100,
            )
            .unwrap();

        let right =
            execution_fingerprint(
                &config,
                1,
                2,
                3,
                4,
                200,
            )
            .unwrap();

        assert_ne!(left, right);
    }

    #[test]
    fn qec_config_integration_works() {
        let config =
            QecConfig::deterministic_test();

        let runtime =
            DeterministicRuntimeConfig
                ::from_qec_config(&config)
                .unwrap();

        assert_ne!(
            runtime.mode,
            DeterminismMode::Disabled
        );

        assert_eq!(
            runtime.worker_count,
            1
        );

        assert!(
            runtime.deterministic_scheduling
        );

        assert!(
            runtime.deterministic_reductions
        );

        assert!(
            runtime.deterministic_serialization
        );
    }

    #[test]
    fn qec_context_uses_repository_configuration() {
        let config =
            QecConfig::deterministic_test();

        let context =
            DeterministicContext
                ::from_qec_config(&config)
                .unwrap();

        assert_eq!(
            context.worker_count(),
            1
        );

        assert!(
            context.config().require_fingerprint
        );
    }

    #[test]
    fn checked_arithmetic_rejects_overflow() {
        assert!(matches!(
            checked_mul_u64(
                u64::MAX,
                2
            ),
            Err(
                DeterministicError::ArithmeticOverflow {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_map_has_canonical_order() {
        let mut map =
            DeterministicMap::new();

        map.insert(3_u64, "c");
        map.insert(1_u64, "a");
        map.insert(2_u64, "b");

        let keys: Vec<u64> =
            map.iter()
                .map(|(key, _)| *key)
                .collect();

        assert_eq!(
            keys,
            vec![1, 2, 3]
        );
    }

    #[test]
    fn fingerprint_is_fixed_size() {
        let config =
            DeterministicConfig::default();

        let fingerprint =
            execution_fingerprint(
                &config,
                1,
                2,
                3,
                4,
                5,
            )
            .unwrap();

        assert_eq!(
            fingerprint.as_bytes().len(),
            FINGERPRINT_SIZE
        );
    }
}