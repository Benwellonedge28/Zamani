//! Deterministic execution infrastructure for Zamani QEC.
//!
//! This module provides deterministic building blocks for quantum-error-
//! correction algorithms, simulations, distributed decoding, checkpointing,
//! metrics, and reproducibility.
//!
//! # Determinism contract
//!
//! Given identical:
//!
//! * code/topology;
//! * noise model;
//! * syndrome stream;
//! * configuration;
//! * seed;
//! * algorithm/version;
//!
//! deterministic execution must produce identical:
//!
//! * event ordering;
//! * pseudo-random decisions;
//! * reductions;
//! * correction ordering;
//! * logical outcome;
//! * execution fingerprint.
//!
//! Determinism is deliberately separated from scheduling. Parallel execution
//! may change physical execution order, but observable QEC results must not
//! depend on that order.
//!
//! # Design goals
//!
//! * no unsafe code;
//! * no process-global mutable state;
//! * no dependence on wall-clock time for deterministic decisions;
//! * checked arithmetic;
//! * canonical ordering;
//! * stable hashing;
//! * reproducible pseudo-random generation;
//! * deterministic floating-point reduction helpers;
//! * deterministic partition/worker assignment;
//! * explicit configuration;
//! * structured errors;
//! * no panics for externally supplied values;
//! * suitable for streaming, partitioned and distributed QEC.
//!
//! # Important limitation
//!
//! Determinism does not mean that floating-point hardware is magically
//! identical across every CPU/GPU architecture. Algorithms requiring
//! cross-platform bit-for-bit reproducibility should use the deterministic
//! integer/fixed-point primitives provided here or an explicitly specified
//! numerical representation.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Deterministic subsystem API version.
pub const DETERMINISTIC_API_VERSION: &str = "1.0.0";

/// Fixed-width output size of the deterministic execution fingerprint.
pub const FINGERPRINT_SIZE: usize = 32;

/// Internal mixing constants.
const MIX_1: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_2: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_3: u64 = 0x94D0_49BB_1331_11EB;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by deterministic execution infrastructure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeterministicError {
    /// A requested operation would overflow.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// A deterministic seed or configuration is invalid.
    InvalidConfiguration {
        reason: &'static str,
    },

    /// An externally supplied collection contains an invalid identifier.
    InvalidIdentifier {
        value: u64,
    },

    /// A duplicate identifier was encountered where uniqueness is required.
    DuplicateIdentifier {
        value: u64,
    },

    /// A reduction was requested for an empty collection when at least one
    /// element is required.
    EmptyInput {
        operation: &'static str,
    },

    /// A sequence number cannot be advanced safely.
    SequenceExhausted,

    /// A worker count is invalid.
    InvalidWorkerCount {
        workers: usize,
    },

    /// A worker identifier is outside the configured worker range.
    InvalidWorkerId {
        worker_id: usize,
        workers: usize,
    },

    /// A deterministic operation cannot represent its requested index.
    IndexOverflow,

    /// A floating-point value cannot participate in deterministic arithmetic.
    InvalidFloatingPoint {
        operation: &'static str,
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

            Self::InvalidIdentifier { value } => {
                write!(f, "invalid deterministic identifier: {value}")
            }

            Self::DuplicateIdentifier { value } => {
                write!(f, "duplicate deterministic identifier: {value}")
            }

            Self::EmptyInput { operation } => {
                write!(f, "empty input is not valid for deterministic operation: {operation}")
            }

            Self::SequenceExhausted => {
                write!(f, "deterministic sequence exhausted")
            }

            Self::InvalidWorkerCount { workers } => {
                write!(f, "invalid deterministic worker count: {workers}")
            }

            Self::InvalidWorkerId { worker_id, workers } => {
                write!(
                    f,
                    "invalid worker id {worker_id}; configured workers: {workers}"
                )
            }

            Self::IndexOverflow => {
                write!(f, "deterministic index overflow")
            }

            Self::InvalidFloatingPoint { operation } => {
                write!(f, "invalid floating-point value for deterministic operation: {operation}")
            }
        }
    }
}

impl std::error::Error for DeterministicError {}

// -----------------------------------------------------------------------------
// Determinism policy
// -----------------------------------------------------------------------------

/// Controls how strongly deterministic execution is enforced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterminismMode {
    /// Deterministic ordering and random decisions.
    Deterministic,

    /// Deterministic execution plus strict validation of deterministic
    /// numerical operations.
    Strict,

    /// Determinism is disabled at the execution-policy level.
    ///
    /// This does not make this module's primitives nondeterministic. It tells
    /// higher-level components that they are permitted to use optimized
    /// nondeterministic execution.
    Disabled,
}

impl Default for DeterminismMode {
    fn default() -> Self {
        Self::Deterministic
    }
}

/// Configuration governing deterministic execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicConfig {
    /// Deterministic execution mode.
    pub mode: DeterminismMode,

    /// Root seed used for reproducible pseudo-random decisions.
    pub seed: u64,

    /// Stable algorithm identifier.
    pub algorithm_id: u64,

    /// Algorithm version encoded as a stable integer.
    pub algorithm_version: u64,

    /// Number of logical workers.
    pub worker_count: usize,

    /// Whether worker assignment must be deterministic.
    pub deterministic_worker_assignment: bool,

    /// Whether reduction ordering must be deterministic.
    pub deterministic_reduction: bool,

    /// Whether fingerprints are required.
    pub require_fingerprint: bool,
}

impl Default for DeterministicConfig {
    fn default() -> Self {
        Self {
            mode: DeterminismMode::Deterministic,
            seed: 0,
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
    /// Validates the complete deterministic configuration.
    pub fn validate(&self) -> Result<(), DeterministicError> {
        if self.worker_count == 0 {
            return Err(DeterministicError::InvalidWorkerCount {
                workers: self.worker_count,
            });
        }

        if self.mode == DeterminismMode::Strict
            && (!self.deterministic_worker_assignment || !self.deterministic_reduction)
        {
            return Err(DeterministicError::InvalidConfiguration {
                reason: "strict mode requires deterministic worker assignment and reduction",
            });
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Deterministic sequence
// -----------------------------------------------------------------------------

/// Monotonically increasing deterministic sequence.
///
/// Useful for:
///
/// * syndrome events;
/// * decoder iterations;
/// * partitions;
/// * checkpoint generations;
/// * telemetry event IDs.
///
/// It never derives ordering from wall-clock time.
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
    /// Creates a sequence starting at zero.
    pub const fn new() -> Self {
        Self { next: 0 }
    }

    /// Creates a sequence beginning at a specific value.
    pub const fn from(start: u64) -> Self {
        Self { next: start }
    }

    /// Returns the next sequence number.
    pub fn next(&mut self) -> Result<u64, DeterministicError> {
        let value = self.next;

        self.next = self
            .next
            .checked_add(1)
            .ok_or(DeterministicError::SequenceExhausted)?;

        Ok(value)
    }

    /// Returns the next value without consuming it.
    pub const fn peek(&self) -> u64 {
        self.next
    }
}

// -----------------------------------------------------------------------------
// Deterministic RNG
// -----------------------------------------------------------------------------

/// Reproducible pseudo-random generator.
///
/// This is deliberately not intended for cryptography.
///
/// QEC simulations should use this generator when reproducibility is required
/// rather than using ambient/global randomness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    /// Creates a generator from a seed.
    pub const fn new(seed: u64) -> Self {
        Self {
            state: splitmix64(seed),
        }
    }

    /// Returns the current internal state.
    pub const fn state(&self) -> u64 {
        self.state
    }

    /// Restores a generator from a previously captured state.
    pub const fn from_state(state: u64) -> Self {
        Self { state }
    }

    /// Generates the next 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        self.state = splitmix64(self.state.wrapping_add(MIX_1));
        self.state
    }

    /// Generates a reproducible boolean.
    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) != 0
    }

    /// Generates a value in `[0, upper)`.
    pub fn next_bounded(&mut self, upper: u64) -> Result<u64, DeterministicError> {
        if upper == 0 {
            return Err(DeterministicError::InvalidConfiguration {
                reason: "random upper bound must be greater than zero",
            });
        }

        // Rejection sampling avoids modulo bias.
        let zone = u64::MAX - (u64::MAX % upper);

        loop {
            let value = self.next_u64();

            if value < zone {
                return Ok(value % upper);
            }
        }
    }

    /// Generates a reproducible `f64` in `[0, 1)`.
    ///
    /// The construction uses the top 53 bits and therefore avoids generating
    /// exactly 1.0.
    pub fn next_unit_f64(&mut self) -> f64 {
        let value = self.next_u64() >> 11;
        value as f64 / ((1_u64 << 53) as f64)
    }
}

// -----------------------------------------------------------------------------
// Stable hashing
// -----------------------------------------------------------------------------

/// Stable non-cryptographic hash state.
///
/// Rust's default hashers are intentionally not specified as a cross-process
/// stable serialization format. This hasher exists for reproducibility.
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
    /// Creates a new stable hash state.
    pub const fn new() -> Self {
        Self {
            state: 0x243F_6A88_85A3_08D3,
        }
    }

    /// Creates a hash state with an explicit seed.
    pub const fn with_seed(seed: u64) -> Self {
        Self {
            state: seed ^ 0x243F_6A88_85A3_08D3,
        }
    }

    /// Mixes raw bytes into the hash.
    pub fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state ^= u64::from(*byte);
            self.state = self.state.wrapping_mul(MIX_2);
            self.state ^= self.state >> 29;
            self.state = self.state.rotate_left(17);
        }
    }

    /// Mixes a `u64`.
    pub fn update_u64(&mut self, value: u64) {
        self.update(&value.to_le_bytes());
    }

    /// Mixes a signed integer.
    pub fn update_i64(&mut self, value: i64) {
        self.update_u64(value as u64);
    }

    /// Mixes a boolean.
    pub fn update_bool(&mut self, value: bool) {
        self.update(&[u8::from(value)]);
    }

    /// Mixes a string using its UTF-8 representation.
    pub fn update_str(&mut self, value: &str) {
        self.update_u64(value.len() as u64);
        self.update(value.as_bytes());
    }

    /// Returns the final 64-bit digest.
    pub fn finish_u64(&self) -> u64 {
        avalanche(self.state)
    }

    /// Returns a fixed-size fingerprint.
    pub fn finish(&self) -> ExecutionFingerprint {
        ExecutionFingerprint::from_seed(self.finish_u64())
    }
}

// -----------------------------------------------------------------------------
// Execution fingerprint
// -----------------------------------------------------------------------------

/// Fixed-size reproducibility fingerprint for a QEC execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionFingerprint([u8; FINGERPRINT_SIZE]);

impl ExecutionFingerprint {
    fn from_seed(seed: u64) -> Self {
        let mut output = [0_u8; FINGERPRINT_SIZE];

        let mut state = seed;

        for chunk in output.chunks_exact_mut(8) {
            state = splitmix64(state.wrapping_add(MIX_3));
            chunk.copy_from_slice(&state.to_le_bytes());
        }

        Self(output)
    }

    /// Returns the raw fingerprint bytes.
    pub const fn as_bytes(&self) -> &[u8; FINGERPRINT_SIZE] {
        &self.0
    }

    /// Returns a lowercase hexadecimal representation.
    pub fn to_hex(&self) -> String {
        let mut output = String::with_capacity(FINGERPRINT_SIZE * 2);

        for byte in self.0 {
            output.push(hex_digit(byte >> 4));
            output.push(hex_digit(byte & 0x0f));
        }

        output
    }
}

impl fmt::Display for ExecutionFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

// -----------------------------------------------------------------------------
// Deterministic execution context
// -----------------------------------------------------------------------------

/// Reproducibility context shared by QEC operations.
///
/// A context should be created once for a logical QEC job and passed through
/// the decoding pipeline.
#[derive(Debug, Clone)]
pub struct DeterministicContext {
    config: DeterministicConfig,
    rng: DeterministicRng,
    sequence: DeterministicSequence,
    hasher: StableHasher,
}

impl DeterministicContext {
    /// Creates and validates a deterministic execution context.
    pub fn new(config: DeterministicConfig) -> Result<Self, DeterministicError> {
        config.validate()?;

        let mut hasher = StableHasher::with_seed(config.seed);
        hasher.update_u64(config.algorithm_id);
        hasher.update_u64(config.algorithm_version);
        hasher.update_u64(config.worker_count as u64);

        Ok(Self {
            rng: DeterministicRng::new(derive_seed(
                config.seed,
                config.algorithm_id,
                config.algorithm_version,
            )),
            sequence: DeterministicSequence::new(),
            hasher,
            config,
        })
    }

    /// Returns the immutable configuration.
    pub const fn config(&self) -> &DeterministicConfig {
        &self.config
    }

    /// Returns the next deterministic sequence number.
    pub fn next_sequence(&mut self) -> Result<u64, DeterministicError> {
        self.sequence.next()
    }

    /// Returns the next reproducible random value.
    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    /// Returns the next reproducible random boolean.
    pub fn next_bool(&mut self) -> bool {
        self.rng.next_bool()
    }

    /// Returns the next reproducible bounded random value.
    pub fn next_bounded(&mut self, upper: u64) -> Result<u64, DeterministicError> {
        self.rng.next_bounded(upper)
    }

    /// Records bytes into the execution fingerprint.
    pub fn record_bytes(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    /// Records a stable integer.
    pub fn record_u64(&mut self, value: u64) {
        self.hasher.update_u64(value);
    }

    /// Records a string.
    pub fn record_str(&mut self, value: &str) {
        self.hasher.update_str(value);
    }

    /// Produces the current execution fingerprint.
    pub fn fingerprint(&self) -> ExecutionFingerprint {
        self.hasher.finish()
    }

    /// Returns the current deterministic sequence position.
    pub const fn sequence_position(&self) -> u64 {
        self.sequence.peek()
    }

    /// Returns the current RNG state.
    pub const fn rng_state(&self) -> u64 {
        self.rng.state()
    }
}

// -----------------------------------------------------------------------------
// Canonical deterministic ordering
// -----------------------------------------------------------------------------

/// A deterministic sortable event.
///
/// QEC events should have a stable ordering independent of arrival order when
/// parallel or distributed processing is used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicEvent<T> {
    /// Stable event identifier.
    pub id: u64,

    /// Measurement/round number.
    pub round: u64,

    /// Optional partition identifier.
    pub partition: u64,

    /// Payload.
    pub payload: T,
}

impl<T> DeterministicEvent<T> {
    /// Constructs a deterministic event.
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

    /// Converts the event into its payload while retaining deterministic
    /// metadata.
    pub fn map<U, F>(self, function: F) -> DeterministicEvent<U>
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
    fn cmp(&self, other: &Self) -> Ordering {
        self.round
            .cmp(&other.round)
            .then_with(|| self.partition.cmp(&other.partition))
            .then_with(|| self.id.cmp(&other.id))
            .then_with(|| self.payload.cmp(&other.payload))
    }
}

impl<T> PartialOrd for DeterministicEvent<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Sorts QEC events into canonical order.
pub fn canonicalize_events<T>(
    events: &mut [DeterministicEvent<T>],
) where
    T: Ord,
{
    events.sort_unstable();
}

// -----------------------------------------------------------------------------
// Deterministic worker assignment
// -----------------------------------------------------------------------------

/// Computes the deterministic worker responsible for an identifier.
///
/// The same identifier and worker count always produce the same worker.
pub fn assign_worker(
    identifier: u64,
    workers: usize,
) -> Result<usize, DeterministicError> {
    if workers == 0 {
        return Err(DeterministicError::InvalidWorkerCount { workers });
    }

    let mixed = splitmix64(identifier);

    Ok((mixed % workers as u64) as usize)
}

/// Validates a worker identifier.
pub fn validate_worker(
    worker_id: usize,
    workers: usize,
) -> Result<(), DeterministicError> {
    if workers == 0 {
        return Err(DeterministicError::InvalidWorkerCount { workers });
    }

    if worker_id >= workers {
        return Err(DeterministicError::InvalidWorkerId {
            worker_id,
            workers,
        });
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Deterministic partition assignment
// -----------------------------------------------------------------------------

/// Deterministically assigns an item to a partition.
pub fn assign_partition(
    identifier: u64,
    partitions: usize,
) -> Result<usize, DeterministicError> {
    assign_worker(identifier, partitions)
}

// -----------------------------------------------------------------------------
// Deterministic reductions
// -----------------------------------------------------------------------------

/// Deterministically reduces values in slice order.
///
/// This is intended for associative integer operations. For floating-point
/// values use [`deterministic_sum_f64`] instead.
pub fn deterministic_reduce<T, F>(
    values: &[T],
    mut initial: T,
    mut operation: F,
) -> T
where
    T: Clone,
    F: FnMut(T, &T) -> T,
{
    for value in values {
        initial = operation(initial, value);
    }

    initial
}

/// Deterministically sums finite `f64` values in their supplied order.
///
/// The function rejects NaN and infinities because allowing them to silently
/// propagate can make decoder results depend on execution order.
pub fn deterministic_sum_f64(
    values: &[f64],
) -> Result<f64, DeterministicError> {
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;

    for value in values {
        if !value.is_finite() {
            return Err(DeterministicError::InvalidFloatingPoint {
                operation: "deterministic_sum_f64",
            });
        }

        // Neumaier-style compensated summation.
        let corrected = *value - compensation;
        let temporary = sum + corrected;
        compensation = (temporary - sum) - corrected;
        sum = temporary;
    }

    if !sum.is_finite() {
        return Err(DeterministicError::InvalidFloatingPoint {
            operation: "deterministic_sum_f64 result",
        });
    }

    Ok(sum)
}

/// Deterministically computes the arithmetic mean.
pub fn deterministic_mean_f64(
    values: &[f64],
) -> Result<f64, DeterministicError> {
    if values.is_empty() {
        return Err(DeterministicError::EmptyInput {
            operation: "deterministic_mean_f64",
        });
    }

    let sum = deterministic_sum_f64(values)?;

    let count = values.len() as f64;

    let result = sum / count;

    if !result.is_finite() {
        return Err(DeterministicError::InvalidFloatingPoint {
            operation: "deterministic_mean_f64 result",
        });
    }

    Ok(result)
}

// -----------------------------------------------------------------------------
// Canonical maps
// -----------------------------------------------------------------------------

/// Deterministic key/value accumulator.
///
/// `BTreeMap` is used deliberately because iteration order is defined by key
/// ordering rather than hash-map implementation details.
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
    /// Creates an empty deterministic map.
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts a value.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.values.insert(key, value)
    }

    /// Returns a value.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.values.get(key)
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Iterates in canonical key order.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.values.iter()
    }
}

// -----------------------------------------------------------------------------
// Canonical serialization helpers
// -----------------------------------------------------------------------------

/// Appends a length-prefixed byte slice to a stable hash.
///
/// Length-prefixing prevents concatenation ambiguity:
///
/// ```text
/// ["ab", "c"] != ["a", "bc"]
/// ```
pub fn record_bytes(
    hasher: &mut StableHasher,
    bytes: &[u8],
) {
    hasher.update_u64(bytes.len() as u64);
    hasher.update(bytes);
}

/// Records a deterministic event into a hash.
pub fn record_event<T>(
    hasher: &mut StableHasher,
    event: &DeterministicEvent<T>,
) where
    T: Hash,
{
    hasher.update_u64(event.id);
    hasher.update_u64(event.round);
    hasher.update_u64(event.partition);

    let mut payload_hasher = StableHasher::new();

    // `Hash` implementations supplied by the standard library are stable for
    // primitive values but are not a general-purpose wire-format contract.
    // This method is therefore intended for internal deterministic metadata,
    // not persistent cross-language serialization.
    event.payload.hash(&mut HashAdapter(&mut payload_hasher));

    hasher.update_u64(payload_hasher.finish_u64());
}

/// Adapter allowing the standard `Hash` trait to feed our stable hasher.
struct HashAdapter<'a>(&'a mut StableHasher);

impl Hasher for HashAdapter<'_> {
    fn finish(&self) -> u64 {
        self.0.finish_u64()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    fn write_u8(&mut self, value: u8) {
        self.write(&[value]);
    }

    fn write_u16(&mut self, value: u16) {
        self.write(&value.to_le_bytes());
    }

    fn write_u32(&mut self, value: u32) {
        self.write(&value.to_le_bytes());
    }

    fn write_u64(&mut self, value: u64) {
        self.write(&value.to_le_bytes());
    }

    fn write_u128(&mut self, value: u128) {
        self.write(&value.to_le_bytes());
    }

    fn write_usize(&mut self, value: usize) {
        self.write_u64(value as u64);
    }

    fn write_i8(&mut self, value: i8) {
        self.write(&value.to_le_bytes());
    }

    fn write_i16(&mut self, value: i16) {
        self.write(&value.to_le_bytes());
    }

    fn write_i32(&mut self, value: i32) {
        self.write(&value.to_le_bytes());
    }

    fn write_i64(&mut self, value: i64) {
        self.write(&value.to_le_bytes());
    }

    fn write_i128(&mut self, value: i128) {
        self.write(&value.to_le_bytes());
    }

    fn write_isize(&mut self, value: isize) {
        self.write_i64(value as i64);
    }
}

// -----------------------------------------------------------------------------
// Checked deterministic arithmetic
// -----------------------------------------------------------------------------

/// Checked multiplication of resource quantities.
pub fn checked_mul_u64(
    left: u64,
    right: u64,
) -> Result<u64, DeterministicError> {
    left.checked_mul(right)
        .ok_or(DeterministicError::ArithmeticOverflow {
            operation: "u64 multiplication",
        })
}

/// Checked addition of resource quantities.
pub fn checked_add_u64(
    left: u64,
    right: u64,
) -> Result<u64, DeterministicError> {
    left.checked_add(right)
        .ok_or(DeterministicError::ArithmeticOverflow {
            operation: "u64 addition",
        })
}

/// Checked conversion of a `usize` to `u64`.
pub fn usize_to_u64(
    value: usize,
) -> Result<u64, DeterministicError> {
    u64::try_from(value).map_err(|_| DeterministicError::IndexOverflow)
}

// -----------------------------------------------------------------------------
// Deterministic execution identity
// -----------------------------------------------------------------------------

/// Builds an execution fingerprint from the fundamental QEC inputs.
///
/// Callers should record canonical representations of:
///
/// * code topology;
/// * stabilizers;
/// * logical operators;
/// * noise configuration;
/// * syndrome stream;
/// * decoder configuration;
/// * resource configuration.
///
/// The returned fingerprint can then be attached to metrics/checkpoints.
pub fn execution_fingerprint(
    config: &DeterministicConfig,
    code_digest: u64,
    noise_digest: u64,
    syndrome_digest: u64,
    decoder_digest: u64,
) -> Result<ExecutionFingerprint, DeterministicError> {
    config.validate()?;

    let mut hasher = StableHasher::with_seed(config.seed);

    hasher.update_str(DETERMINISTIC_API_VERSION);
    hasher.update_u64(config.algorithm_id);
    hasher.update_u64(config.algorithm_version);
    hasher.update_u64(config.worker_count as u64);

    hasher.update_u64(code_digest);
    hasher.update_u64(noise_digest);
    hasher.update_u64(syndrome_digest);
    hasher.update_u64(decoder_digest);

    Ok(hasher.finish())
}

// -----------------------------------------------------------------------------
// Utility functions
// -----------------------------------------------------------------------------

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(MIX_1);
    value = (value ^ (value >> 30)).wrapping_mul(MIX_2);
    value = (value ^ (value >> 27)).wrapping_mul(MIX_3);
    value ^ (value >> 31)
}

fn derive_seed(
    seed: u64,
    algorithm_id: u64,
    algorithm_version: u64,
) -> u64 {
    let mut value = seed;

    value ^= algorithm_id.rotate_left(17);
    value = splitmix64(value);

    value ^= algorithm_version.rotate_left(31);
    splitmix64(value)
}

fn avalanche(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(MIX_2);

    value ^= value >> 27;
    value = value.wrapping_mul(MIX_3);

    value ^ (value >> 31)
}

fn hex_digit(value: u8) -> char {
    match value & 0x0f {
        0..=9 => (b'0' + (value & 0x0f)) as char,
        10..=15 => (b'a' + ((value & 0x0f) - 10)) as char,
        _ => unreachable!(),
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_are_valid() {
        let config = DeterministicConfig::default();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn zero_workers_are_rejected() {
        let config = DeterministicConfig {
            worker_count: 0,
            ..Default::default()
        };

        assert!(matches!(
            config.validate(),
            Err(DeterministicError::InvalidWorkerCount {
                workers: 0
            })
        ));
    }

    #[test]
    fn strict_mode_rejects_nondeterministic_reduction() {
        let config = DeterministicConfig {
            mode: DeterminismMode::Strict,
            deterministic_reduction: false,
            ..Default::default()
        };

        assert!(matches!(
            config.validate(),
            Err(DeterministicError::InvalidConfiguration { .. })
        ));
    }

    #[test]
    fn deterministic_rng_repeats_exactly() {
        let mut first = DeterministicRng::new(12345);
        let mut second = DeterministicRng::new(12345);

        for _ in 0..1024 {
            assert_eq!(
                first.next_u64(),
                second.next_u64()
            );
        }
    }

    #[test]
    fn different_seeds_produce_different_sequences() {
        let mut first = DeterministicRng::new(1);
        let mut second = DeterministicRng::new(2);

        assert_ne!(
            first.next_u64(),
            second.next_u64()
        );
    }

    #[test]
    fn rng_state_can_be_restored() {
        let mut rng = DeterministicRng::new(99);

        let _ = rng.next_u64();
        let state = rng.state();

        let expected = rng.next_u64();

        let mut restored =
            DeterministicRng::from_state(state);

        assert_eq!(
            expected,
            restored.next_u64()
        );
    }

    #[test]
    fn bounded_rng_respects_upper_bound() {
        let mut rng = DeterministicRng::new(123);

        for _ in 0..10_000 {
            let value =
                rng.next_bounded(17)
                    .expect("valid bound");

            assert!(value < 17);
        }
    }

    #[test]
    fn zero_rng_bound_is_rejected() {
        let mut rng = DeterministicRng::new(1);

        assert!(matches!(
            rng.next_bounded(0),
            Err(DeterministicError::InvalidConfiguration { .. })
        ));
    }

    #[test]
    fn sequence_is_monotonic() {
        let mut sequence = DeterministicSequence::new();

        assert_eq!(sequence.next().unwrap(), 0);
        assert_eq!(sequence.next().unwrap(), 1);
        assert_eq!(sequence.next().unwrap(), 2);
        assert_eq!(sequence.peek(), 3);
    }

    #[test]
    fn stable_hash_is_repeatable() {
        let mut first = StableHasher::new();
        first.update_str("zamani");
        first.update_u64(42);

        let mut second = StableHasher::new();
        second.update_str("zamani");
        second.update_u64(42);

        assert_eq!(
            first.finish_u64(),
            second.finish_u64()
        );

        assert_eq!(
            first.finish(),
            second.finish()
        );
    }

    #[test]
    fn fingerprints_are_repeatable() {
        let config = DeterministicConfig {
            seed: 42,
            algorithm_id: 7,
            algorithm_version: 3,
            worker_count: 4,
            ..Default::default()
        };

        let first = execution_fingerprint(
            &config,
            100,
            200,
            300,
            400,
        )
        .unwrap();

        let second = execution_fingerprint(
            &config,
            100,
            200,
            300,
            400,
        )
        .unwrap();

        assert_eq!(first, second);
        assert_eq!(
            first.to_hex().len(),
            FINGERPRINT_SIZE * 2
        );
    }

    #[test]
    fn changing_syndrome_changes_fingerprint() {
        let config = DeterministicConfig::default();

        let first = execution_fingerprint(
            &config,
            1,
            2,
            3,
            4,
        )
        .unwrap();

        let second = execution_fingerprint(
            &config,
            1,
            2,
            4,
            4,
        )
        .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn events_have_canonical_order() {
        let mut events = vec![
            DeterministicEvent::new(2, 1, 0, 10_u64),
            DeterministicEvent::new(1, 0, 0, 20_u64),
            DeterministicEvent::new(1, 1, 0, 30_u64),
            DeterministicEvent::new(1, 1, 1, 40_u64),
        ];

        canonicalize_events(&mut events);

        assert_eq!(events[0].round, 0);
        assert_eq!(events[1].id, 1);
        assert_eq!(events[1].partition, 0);
        assert_eq!(events[2].id, 2);
        assert_eq!(events[3].partition, 1);
    }

    #[test]
    fn worker_assignment_is_repeatable() {
        for id in 0..10_000 {
            let first =
                assign_worker(id, 17).unwrap();

            let second =
                assign_worker(id, 17).unwrap();

            assert_eq!(first, second);
            assert!(first < 17);
        }
    }

    #[test]
    fn partition_assignment_is_repeatable() {
        for id in 0..1000 {
            assert_eq!(
                assign_partition(id, 8).unwrap(),
                assign_partition(id, 8).unwrap()
            );
        }
    }

    #[test]
    fn invalid_worker_id_is_rejected() {
        assert!(matches!(
            validate_worker(8, 8),
            Err(DeterministicError::InvalidWorkerId {
                worker_id: 8,
                workers: 8
            })
        ));
    }

    #[test]
    fn deterministic_sum_is_stable() {
        let values = [
            1.0,
            2.0,
            3.0,
            4.0,
        ];

        assert_eq!(
            deterministic_sum_f64(&values).unwrap(),
            10.0
        );
    }

    #[test]
    fn deterministic_mean_is_stable() {
        let values = [
            2.0,
            4.0,
            6.0,
        ];

        assert_eq!(
            deterministic_mean_f64(&values).unwrap(),
            4.0
        );
    }

    #[test]
    fn nan_is_rejected() {
        assert!(matches!(
            deterministic_sum_f64(&[1.0, f64::NAN]),
            Err(DeterministicError::InvalidFloatingPoint { .. })
        ));
    }

    #[test]
    fn infinity_is_rejected() {
        assert!(matches!(
            deterministic_sum_f64(&[1.0, f64::INFINITY]),
            Err(DeterministicError::InvalidFloatingPoint { .. })
        ));
    }

    #[test]
    fn empty_mean_is_rejected() {
        assert!(matches!(
            deterministic_mean_f64(&[]),
            Err(DeterministicError::EmptyInput { .. })
        ));
    }

    #[test]
    fn deterministic_map_is_ordered() {
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
    fn checked_arithmetic_rejects_overflow() {
        assert!(matches!(
            checked_add_u64(u64::MAX, 1),
            Err(DeterministicError::ArithmeticOverflow { .. })
        ));

        assert!(matches!(
            checked_mul_u64(u64::MAX, 2),
            Err(DeterministicError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn context_is_reproducible() {
        let config = DeterministicConfig {
            seed: 123,
            algorithm_id: 77,
            algorithm_version: 4,
            worker_count: 4,
            ..Default::default()
        };

        let mut first =
            DeterministicContext::new(config.clone())
                .unwrap();

        let mut second =
            DeterministicContext::new(config)
                .unwrap();

        for _ in 0..100 {
            assert_eq!(
                first.next_sequence().unwrap(),
                second.next_sequence().unwrap()
            );

            assert_eq!(
                first.next_u64(),
                second.next_u64()
            );
        }
    }

    #[test]
    fn context_fingerprint_changes_when_recording_changes() {
        let config = DeterministicConfig::default();

        let mut first =
            DeterministicContext::new(config.clone())
                .unwrap();

        let mut second =
            DeterministicContext::new(config)
                .unwrap();

        first.record_str("syndrome-A");
        second.record_str("syndrome-B");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn event_mapping_preserves_identity() {
        let event =
            DeterministicEvent::new(
                10,
                20,
                30,
                40_u64,
            );

        let mapped =
            event.map(|value| value * 2);

        assert_eq!(mapped.id, 10);
        assert_eq!(mapped.round, 20);
        assert_eq!(mapped.partition, 30);
        assert_eq!(mapped.payload, 80);
    }

    #[test]
    fn hex_fingerprint_is_lowercase() {
        let fingerprint =
            ExecutionFingerprint::from_seed(42);

        let hex =
            fingerprint.to_hex();

        assert_eq!(
            hex.len(),
            64
        );

        assert!(
            hex.chars().all(|character| {
                character.is_ascii_hexdigit()
                    && !character.is_ascii_uppercase()
            })
        );
    }
}