//! Deterministic execution infrastructure for Zamani Quantum Error Correction.
//!
//! # Ownership
//!
//! `deterministic.rs` owns execution-time determinism:
//!
//! - deterministic execution mode;
//! - reproducible seeds;
//! - deterministic sequences;
//! - canonical ordering;
//! - deterministic worker assignment;
//! - deterministic reductions;
//! - execution fingerprints;
//! - deterministic runtime context.
//!
//! It does NOT own:
//!
//! - resource limits (`limits.rs`);
//! - resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`);
//! - cancellation state (`cancellation.rs`);
//! - configuration composition (`configuration.rs`);
//! - authorization (`capabilities.rs`);
//! - decoder algorithms;
//! - telemetry transport;
//! - checkpoint serialization.
//!
//! # Integration contract
//!
//! ```text
//! configuration.rs
//!        │
//!        │ validated policy
//!        ▼
//! deterministic.rs
//!        │
//!        ├── decoder.rs
//!        ├── decoding_graph.rs
//!        ├── streaming.rs
//!        ├── partition.rs
//!        ├── distributed.rs
//!        ├── simulation.rs
//!        ├── checkpoint.rs
//!        └── replay.rs
//!
//! cancellation.rs is consulted at execution boundaries.
//! limits.rs remains the authoritative resource-policy owner.
//! resources.rs remains the authoritative runtime-accounting owner.
//! ```
//!
//! # Determinism guarantee
//!
//! For the same:
//!
//! - QEC configuration;
//! - algorithm identity/version;
//! - input;
//! - seed;
//! - logical worker count;
//! - canonical ordering rules;
//!
//! deterministic execution must produce the same observable logical result.
//!
//! This does not claim that arbitrary floating-point hardware is
//! bit-for-bit identical. Strict numerical reproducibility requires callers
//! to use the deterministic numerical policy and canonical reductions.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and uses only stable standard
//! library facilities.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use super::cancellation::CancellationToken;
use super::configuration::{
    FloatingPointMode,
    QecConfig,
};
use super::errors::{QecError, QecResult};

// ============================================================================
// Public constants
// ============================================================================

/// Deterministic execution API version.
pub const DETERMINISTIC_API_VERSION: &str = "3.0.0";

/// Size of a deterministic fingerprint.
pub const FINGERPRINT_SIZE: usize = 32;

/// Stable default seed.
///
/// This seed is only used when deterministic execution is explicitly enabled
/// without an application-provided seed.
pub const DEFAULT_DETERMINISTIC_SEED: u64 = 0x5A4D_414E_4951_4543;

// ============================================================================
// Errors
// ============================================================================

/// Errors local to deterministic execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeterministicError {
    ArithmeticOverflow {
        operation: &'static str,
    },

    InvalidConfiguration {
        reason: String,
    },

    ConfigurationValidation {
        message: String,
    },

    InvalidIdentifier {
        value: u64,
    },

    DuplicateIdentifier {
        value: u64,
    },

    EmptyInput {
        operation: &'static str,
    },

    SequenceExhausted,

    InvalidWorkerCount {
        workers: usize,
    },

    InvalidWorkerId {
        worker_id: usize,
        workers: usize,
    },

    InvalidPartitionCount {
        partitions: usize,
    },

    IndexOverflow,

    InvalidFloatingPoint {
        operation: &'static str,
    },

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
                write!(f, "empty input for deterministic operation: {operation}")
            }

            Self::SequenceExhausted => {
                f.write_str("deterministic sequence exhausted")
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
                write!(
                    f,
                    "invalid deterministic partition count: {partitions}"
                )
            }

            Self::IndexOverflow => {
                f.write_str("deterministic index conversion overflow")
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
                    operation:
                        super::errors::NumericalOperation::FloatingPoint,
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
                    feature: "deterministic_execution".to_owned(),
                    message: reason,
                }
            }

            DeterministicError::SequenceExhausted => {
                QecError::InternalInvariantViolation {
                    invariant:
                        "deterministic_sequence_must_not_wrap".to_owned(),
                    message: "deterministic sequence exhausted".to_owned(),
                }
            }

            DeterministicError::InvariantViolation { invariant } => {
                QecError::InternalInvariantViolation {
                    invariant: invariant.to_owned(),
                    message:
                        "deterministic execution invariant violated"
                            .to_owned(),
                }
            }

            other => QecError::InvalidInput {
                message: other.to_string(),
            },
        }
    }
}

/// Internal deterministic result.
pub type DeterministicResult<T> = Result<T, DeterministicError>;

// ============================================================================
// Determinism mode
// ============================================================================

/// Execution-level determinism mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterminismMode {
    /// Deterministic execution is disabled.
    Disabled,

    /// Canonical scheduling, ordering and reductions are required.
    Deterministic,

    /// Deterministic execution plus strict numerical validation.
    Strict,
}

impl Default for DeterminismMode {
    fn default() -> Self {
        Self::Disabled
    }
}

/// Compatibility alias used by older QEC tests/callers.
pub type DeterministicMode = DeterminismMode;

// ============================================================================
// Runtime configuration
// ============================================================================

/// Validated execution-time deterministic configuration.
///
/// `configuration.rs` remains the owner of user-facing policy.
/// This structure is the execution representation of that policy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicRuntimeConfig {
    pub mode: DeterminismMode,
    pub seed: u64,
    pub worker_count: usize,
    pub deterministic_scheduling: bool,
    pub deterministic_reductions: bool,
    pub deterministic_serialization: bool,
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
            require_fingerprint: false,
        }
    }
}

impl DeterministicRuntimeConfig {
    /// Builds execution state from the canonical `QecConfig`.
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

        let worker_count = config.parallelism.max_workers;

        if worker_count == 0 {
            return Err(
                DeterministicError::InvalidWorkerCount {
                    workers: 0,
                },
            );
        }

        let runtime = Self {
            mode,
            seed: policy
                .seed
                .unwrap_or(DEFAULT_DETERMINISTIC_SEED),
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

    /// Validates execution invariants.
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
                                .to_owned(),
                    },
                );
            }

            if !self.deterministic_reductions {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic reductions"
                                .to_owned(),
                    },
                );
            }

            if !self.deterministic_serialization {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic serialization"
                                .to_owned(),
                    },
                );
            }
        }

        Ok(())
    }

    pub const fn is_enabled(&self) -> bool {
        !matches!(self.mode, DeterminismMode::Disabled)
    }

    pub const fn is_strict(&self) -> bool {
        matches!(self.mode, DeterminismMode::Strict)
    }
}

// ============================================================================
// Standalone configuration
// ============================================================================

/// Low-level deterministic configuration.
///
/// This is useful for mathematical/unit-level callers that do not need a
/// complete `QecConfig`.
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
                                .to_owned(),
                    },
                );
            }

            if !self.deterministic_reduction {
                return Err(
                    DeterministicError::InvalidConfiguration {
                        reason:
                            "deterministic execution requires deterministic reduction"
                                .to_owned(),
                    },
                );
            }
        }

        Ok(())
    }

    pub fn runtime(
        &self,
    ) -> DeterministicResult<DeterministicRuntimeConfig> {
        self.validate()?;

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

/// Overflow-safe monotonically increasing sequence.
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

    pub const fn peek(&self) -> u64 {
        self.next
    }

    pub fn next(&mut self) -> DeterministicResult<u64> {
        let value = self.next;

        self.next = self
            .next
            .checked_add(1)
            .ok_or(DeterministicError::SequenceExhausted)?;

        Ok(value)
    }

    pub fn reset(&mut self) {
        self.next = 0;
    }
}

// ============================================================================
// Stable ordering
// ============================================================================

/// Canonical ordering helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StableOrdering;

impl StableOrdering {
    pub const fn new() -> Self {
        Self
    }

    /// Returns a stable ordering for primitive identifiers.
    pub fn compare_u64(left: u64, right: u64) -> Ordering {
        left.cmp(&right)
    }

    /// Sorts values using their total `Ord` implementation.
    pub fn sort<T: Ord>(values: &mut [T]) {
        values.sort();
    }

    /// Returns a deterministically ordered copy.
    pub fn sorted<T: Ord + Clone>(values: &[T]) -> Vec<T> {
        let mut result = values.to_vec();
        result.sort();
        result
    }
}

impl Default for StableOrdering {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Deterministic reduction
// ============================================================================

/// Canonical reduction helper.
///
/// Floating-point callers should use this only when the selected numerical
/// policy permits it. Exact integer reductions are preferred for QEC counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeterministicReduction;

impl DeterministicReduction {
    pub const fn new() -> Self {
        Self
    }

    pub fn sum_u64(values: &[u64]) -> DeterministicResult<u64> {
        let mut ordered = values.to_vec();
        ordered.sort_unstable();

        let mut result = 0_u64;

        for value in ordered {
            result = result.checked_add(value).ok_or(
                DeterministicError::ArithmeticOverflow {
                    operation: "u64 deterministic reduction",
                },
            )?;
        }

        Ok(result)
    }

    pub fn xor_u64(values: &[u64]) -> u64 {
        let mut ordered = values.to_vec();
        ordered.sort_unstable();

        ordered
            .into_iter()
            .fold(0_u64, |acc, value| acc ^ value)
    }

    pub fn count<T>(values: &[T]) -> usize {
        values.len()
    }
}

impl Default for DeterministicReduction {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Deterministic worker assignment
// ============================================================================

/// Maps logical work identifiers to workers independently of arrival order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkerAssignment {
    worker_count: usize,
}

impl WorkerAssignment {
    pub fn new(worker_count: usize) -> DeterministicResult<Self> {
        if worker_count == 0 {
            return Err(
                DeterministicError::InvalidWorkerCount {
                    workers: 0,
                },
            );
        }

        Ok(Self { worker_count })
    }

    pub const fn worker_count(&self) -> usize {
        self.worker_count
    }

    pub fn worker_for(&self, job_id: u64) -> usize {
        (job_id % self.worker_count as u64) as usize
    }

    pub fn validate_worker(
        &self,
        worker_id: usize,
    ) -> DeterministicResult<()> {
        if worker_id >= self.worker_count {
            return Err(
                DeterministicError::InvalidWorkerId {
                    worker_id,
                    workers: self.worker_count,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Reproducible random stream
// ============================================================================

/// Small deterministic pseudo-random generator.
///
/// This is intentionally not a cryptographic RNG. It is for reproducible
/// simulation, scheduling and test generation. Security-sensitive randomness
/// must use an appropriate cryptographic source elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReproducibleRng {
    state: u64,
}

impl ReproducibleRng {
    pub const fn from_seed(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                DEFAULT_DETERMINISTIC_SEED
            } else {
                seed
            },
        }
    }

    pub const fn seed(&self) -> u64 {
        self.state
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;

        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;

        self.state = x;

        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    pub fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    pub fn next_bounded(
        &mut self,
        upper_exclusive: u64,
    ) -> DeterministicResult<u64> {
        if upper_exclusive == 0 {
            return Err(
                DeterministicError::InvalidConfiguration {
                    reason:
                        "bounded deterministic RNG requires non-zero upper bound"
                            .to_owned(),
                },
            );
        }

        Ok(self.next_u64() % upper_exclusive)
    }
}

// ============================================================================
// Fingerprint
// ============================================================================

/// Fixed-size deterministic execution fingerprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionFingerprint([u8; FINGERPRINT_SIZE]);

impl ExecutionFingerprint {
    pub const fn zero() -> Self {
        Self([0; FINGERPRINT_SIZE])
    }

    pub const fn bytes(&self) -> &[u8; FINGERPRINT_SIZE] {
        &self.0
    }

    pub fn from_bytes(bytes: [u8; FINGERPRINT_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn hex(&self) -> String {
        let mut output =
            String::with_capacity(FINGERPRINT_SIZE * 2);

        for byte in self.0 {
            use fmt::Write;
            let _ = write!(output, "{byte:02x}");
        }

        output
    }
}

impl fmt::Display for ExecutionFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hex())
    }
}

// ============================================================================
// Deterministic context
// ============================================================================

/// Execution context passed to deterministic QEC algorithms.
///
/// This is the primary integration object for decoders, streaming,
/// partitioning, distributed execution and simulation.
#[derive(Debug, Clone)]
pub struct DeterministicContext {
    runtime: DeterministicRuntimeConfig,
    sequence: DeterministicSequence,
    rng: ReproducibleRng,
    worker_assignment: WorkerAssignment,
}

impl DeterministicContext {
    pub fn new(
        runtime: DeterministicRuntimeConfig,
    ) -> DeterministicResult<Self> {
        runtime.validate()?;

        let worker_assignment =
            WorkerAssignment::new(runtime.worker_count)?;

        Ok(Self {
            rng: ReproducibleRng::from_seed(runtime.seed),
            sequence: DeterministicSequence::new(),
            worker_assignment,
            runtime,
        })
    }

    pub fn from_qec_config(
        config: &QecConfig,
    ) -> DeterministicResult<Self> {
        Self::new(
            DeterministicRuntimeConfig::from_qec_config(config)?,
        )
    }

    pub const fn runtime(
        &self,
    ) -> &DeterministicRuntimeConfig {
        &self.runtime
    }

    pub const fn mode(&self) -> DeterminismMode {
        self.runtime.mode
    }

    pub const fn seed(&self) -> u64 {
        self.runtime.seed
    }

    pub const fn worker_count(&self) -> usize {
        self.runtime.worker_count
    }

    pub const fn is_enabled(&self) -> bool {
        !matches!(
            self.runtime.mode,
            DeterminismMode::Disabled
        )
    }

    pub const fn is_strict(&self) -> bool {
        matches!(
            self.runtime.mode,
            DeterminismMode::Strict
        )
    }

    pub fn next_sequence(
        &mut self,
    ) -> DeterministicResult<u64> {
        self.sequence.next()
    }

    pub const fn sequence_position(&self) -> u64 {
        self.sequence.peek()
    }

    pub fn worker_for(&self, job_id: u64) -> usize {
        self.worker_assignment.worker_for(job_id)
    }

    pub fn validate_worker(
        &self,
        worker_id: usize,
    ) -> DeterministicResult<()> {
        self.worker_assignment.validate_worker(worker_id)
    }

    pub fn random_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn random_bool(&mut self) -> bool {
        self.rng.next_bool()
    }

    pub fn random_bounded(
        &mut self,
        upper_exclusive: u64,
    ) -> DeterministicResult<u64> {
        self.rng.next_bounded(upper_exclusive)
    }

    /// Checks cancellation at a deterministic execution boundary.
    pub fn check_cancellation(
        &self,
        cancellation: &CancellationToken,
    ) -> QecResult<()> {
        cancellation.check()
    }

    /// Creates a fingerprint from deterministic execution metadata.
    ///
    /// This is deliberately a stable internal fingerprint rather than a
    /// cryptographic security primitive.
    pub fn fingerprint(
        &self,
        algorithm_id: u64,
        algorithm_version: u64,
        input_digest: &[u8],
    ) -> ExecutionFingerprint {
        let mut values = Vec::with_capacity(
            input_digest.len() + 40,
        );

        values.extend_from_slice(
            DETERMINISTIC_API_VERSION.as_bytes(),
        );

        values.extend_from_slice(
            &self.runtime.seed.to_le_bytes(),
        );

        values.extend_from_slice(
            &self.runtime.worker_count.to_le_bytes(),
        );

        values.extend_from_slice(
            &algorithm_id.to_le_bytes(),
        );

        values.extend_from_slice(
            &algorithm_version.to_le_bytes(),
        );

        values.extend_from_slice(input_digest);

        stable_fingerprint(&values)
    }
}

// ============================================================================
// Stable fingerprint implementation
// ============================================================================

fn stable_fingerprint(
    bytes: &[u8],
) -> ExecutionFingerprint {
    let mut output = [0_u8; FINGERPRINT_SIZE];

    let mut lanes = [
        0x243F_6A88_85A3_08D3_u64,
        0x1319_8A2E_0370_7344_u64,
        0xA409_3822_299F_31D0_u64,
        0x082E_FA98_EC4E_6C89_u64,
    ];

    for (index, byte) in bytes.iter().enumerate() {
        let lane = index & 3;

        lanes[lane] ^= (*byte as u64)
            .wrapping_add(index as u64);

        lanes[lane] = lanes[lane]
            .rotate_left(13)
            .wrapping_mul(
                0x9E37_79B9_7F4A_7C15_u64,
            );
    }

    for lane in &mut lanes {
        *lane ^= *lane >> 30;
        *lane = lane.wrapping_mul(
            0xBF58_476D_1CE4_E5B9_u64,
        );
        *lane ^= *lane >> 27;
        *lane = lane.wrapping_mul(
            0x94D0_49BB_1331_11EB_u64,
        );
        *lane ^= *lane >> 31;
    }

    for (index, lane) in lanes.iter().enumerate() {
        let start = index * 8;

        output[start..start + 8]
            .copy_from_slice(
                &lane.to_le_bytes(),
            );
    }

    ExecutionFingerprint(output)
}

// ============================================================================
// Canonical map helper
// ============================================================================

/// Builds a canonical map independent of insertion order.
pub fn canonical_map<K, V, I>(
    entries: I,
) -> DeterministicResult<BTreeMap<K, V>>
where
    K: Ord,
    I: IntoIterator<Item = (K, V)>,
{
    let mut map = BTreeMap::new();

    for (key, value) in entries {
        if map.insert(key, value).is_some() {
            return Err(
                DeterministicError::InvariantViolation {
                    invariant:
                        "canonical_map_keys_must_be_unique",
                },
            );
        }
    }

    Ok(map)
}

// ============================================================================
// Canonical ordering helpers
// ============================================================================

/// Returns identifiers in canonical order.
pub fn canonical_ids(
    ids: &[u64],
) -> DeterministicResult<Vec<u64>> {
    let mut result = ids.to_vec();
    result.sort_unstable();

    for window in result.windows(2) {
        if window[0] == window[1] {
            return Err(
                DeterministicError::DuplicateIdentifier {
                    value: window[0],
                },
            );
        }
    }

    Ok(result)
}

/// Canonically sorts `(identifier, value)` pairs.
pub fn canonical_pairs<T>(
    pairs: &[(u64, T)],
) -> DeterministicResult<Vec<(u64, T)>>
where
    T: Clone,
{
    let mut result = pairs.to_vec();

    result.sort_by_key(|(id, _)| *id);

    for window in result.windows(2) {
        if window[0].0 == window[1].0 {
            return Err(
                DeterministicError::DuplicateIdentifier {
                    value: window[0].0,
                },
            );
        }
    }

    Ok(result)
}

// ============================================================================
// Deterministic numeric validation
// ============================================================================

/// Validates a floating-point value for deterministic execution.
pub fn validate_f64(
    value: f64,
    operation: &'static str,
) -> DeterministicResult<f64> {
    if !value.is_finite() {
        return Err(
            DeterministicError::InvalidFloatingPoint {
                operation,
            },
        );
    }

    Ok(value)
}

/// Deterministically compares two finite floating-point values.
pub fn compare_f64(
    left: f64,
    right: f64,
) -> DeterministicResult<Ordering> {
    validate_f64(left, "comparison")?;
    validate_f64(right, "comparison")?;

    left.partial_cmp(&right).ok_or(
        DeterministicError::InvalidFloatingPoint {
            operation: "comparison",
        },
    )
}

// ============================================================================
// Deterministic partition assignment
// ============================================================================

/// Deterministically assigns partitions to workers.
pub fn assign_partition(
    partition_id: usize,
    partitions: usize,
    workers: usize,
) -> DeterministicResult<usize> {
    if partitions == 0 {
        return Err(
            DeterministicError::InvalidPartitionCount {
                partitions: 0,
            },
        );
    }

    if workers == 0 {
        return Err(
            DeterministicError::InvalidWorkerCount {
                workers: 0,
            },
        );
    }

    if partition_id >= partitions {
        return Err(
            DeterministicError::InvalidIdentifier {
                value: partition_id as u64,
            },
        );
    }

    Ok(partition_id % workers)
}

// ============================================================================
// Deterministic cancellation boundary
// ============================================================================

/// Checks cancellation without changing deterministic state.
#[inline]
pub fn check_cancellation(
    token: &CancellationToken,
) -> QecResult<()> {
    token.check()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_starts_at_zero() {
        let mut sequence = DeterministicSequence::new();

        assert_eq!(sequence.peek(), 0);
        assert_eq!(sequence.next().unwrap(), 0);
        assert_eq!(sequence.peek(), 1);
    }

    #[test]
    fn sequence_is_reproducible() {
        let mut first = DeterministicSequence::new();
        let mut second = DeterministicSequence::new();

        for _ in 0..1024 {
            assert_eq!(
                first.next().unwrap(),
                second.next().unwrap()
            );
        }
    }

    #[test]
    fn sequence_detects_overflow() {
        let mut sequence =
            DeterministicSequence::from(u64::MAX);

        assert_eq!(
            sequence.next().unwrap(),
            u64::MAX
        );

        assert_eq!(
            sequence.next(),
            Err(DeterministicError::SequenceExhausted)
        );
    }

    #[test]
    fn worker_assignment_is_stable() {
        let assignment =
            WorkerAssignment::new(4).unwrap();

        assert_eq!(assignment.worker_for(0), 0);
        assert_eq!(assignment.worker_for(1), 1);
        assert_eq!(assignment.worker_for(4), 0);
        assert_eq!(assignment.worker_for(9), 1);
    }

    #[test]
    fn rng_is_reproducible() {
        let mut first =
            ReproducibleRng::from_seed(1234);

        let mut second =
            ReproducibleRng::from_seed(1234);

        for _ in 0..1024 {
            assert_eq!(
                first.next_u64(),
                second.next_u64()
            );
        }
    }

    #[test]
    fn canonical_ids_reject_duplicates() {
        let result =
            canonical_ids(&[3, 1, 2, 1]);

        assert!(matches!(
            result,
            Err(DeterministicError::DuplicateIdentifier {
                value: 1
            })
        ));
    }

    #[test]
    fn canonical_ids_are_order_independent() {
        let first =
            canonical_ids(&[9, 3, 7, 1]).unwrap();

        let second =
            canonical_ids(&[1, 7, 3, 9]).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn finite_float_validation_works() {
        assert!(validate_f64(1.0, "test").is_ok());
        assert!(validate_f64(f64::NAN, "test").is_err());
        assert!(validate_f64(f64::INFINITY, "test").is_err());
    }

    #[test]
    fn fingerprint_is_reproducible() {
        let runtime =
            DeterministicRuntimeConfig::default();

        let context =
            DeterministicContext::new(runtime)
                .unwrap();

        let first =
            context.fingerprint(1, 2, b"input");

        let second =
            context.fingerprint(1, 2, b"input");

        assert_eq!(first, second);
    }

    #[test]
    fn partition_assignment_is_stable() {
        assert_eq!(
            assign_partition(0, 16, 4).unwrap(),
            0
        );

        assert_eq!(
            assign_partition(5, 16, 4).unwrap(),
            1
        );

        assert_eq!(
            assign_partition(15, 16, 4).unwrap(),
            3
        );
    }
}