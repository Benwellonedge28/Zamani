//! Zamani Quantum Benchmarking — Deterministic Randomness
//!
//! This module provides the canonical random-number infrastructure used by
//! quantum benchmark circuit generators.
//!
//! # Architectural role
//!
//! `random.rs` is deliberately independent of:
//!
//! - quantum IR
//! - hardware backends
//! - benchmark protocols
//! - circuit generators
//! - statistics
//! - execution
//! - reporting
//!
//! Its only responsibility is to provide a safe, deterministic, explicitly
//! seeded source of pseudo-randomness for benchmark generation.
//!
//! The dependency direction is therefore:
//!
//! ```text
//! generators/random.rs
//!        │
//!        ├──────────────► generators/clifford.rs
//!        ├──────────────► generators/pauli.rs
//!        ├──────────────► generators/random_circuits.rs
//!        ├──────────────► generators/mirror_circuits.rs
//!        ├──────────────► generators/qv.rs
//!        └──────────────► future benchmark generators
//! ```
//!
//! No module in this file depends on those future modules.
//!
//! # Reproducibility contract
//!
//! Benchmark reproducibility requires more than "use a random seed". A
//! benchmark must be able to record:
//!
//! - the seed;
//! - the random algorithm identifier;
//! - the stream/domain identifier;
//! - the generator version;
//! - the benchmark configuration.
//!
//! This module therefore uses a deliberately versioned deterministic
//! SplitMix64-based generator.
//!
//! The algorithm identifier is:
//!
//! `splitmix64-zamani-v1`
//!
//! It is part of the public reproducibility contract. It MUST NOT be silently
//! changed in a future release.
//!
//! If Zamani changes the random algorithm, it must introduce a new algorithm
//! identifier, for example:
//!
//! `splitmix64-zamani-v2`
//!
//! Existing benchmark results can then remain reproducible.
//!
//! # Cryptographic warning
//!
//! This generator is NOT cryptographically secure.
//!
//! It is intended for:
//!
//! - randomized benchmarking;
//! - random circuit generation;
//! - Clifford/Pauli selection;
//! - randomized workload construction;
//! - reproducible simulation experiments;
//! - deterministic regression fixtures.
//!
//! It MUST NOT be used for:
//!
//! - cryptographic keys;
//! - authentication tokens;
//! - security nonces;
//! - secrets;
//! - cryptographic randomness.
//!
//! For cryptographic randomness, use the appropriate cryptographic RNG from
//! the security subsystem.
//!
//! # Entropy
//!
//! `BenchmarkSeed::from_entropy()` uses the operating-system-backed
//! `rand::rngs::OsRng` to obtain an initial seed. Once that seed is created,
//! all benchmark generation is deterministic.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! Future generators MUST:
//!
//! 1. receive a `RandomStream` explicitly;
//! 2. never call a global RNG;
//! 3. never use `thread_rng()` directly;
//! 4. never derive randomness from system time;
//! 5. never use an implicit global mutable RNG;
//! 6. record the root `BenchmarkSeed` in benchmark provenance;
//! 7. use `derive()` or `fork()` to create independent logical streams;
//! 8. keep benchmark-domain labels stable once published.
//!
//! Example:
//!
//! ```text
//! let root = BenchmarkSeed::from_u64(42);
//! let qv_stream = RandomStream::from_seed(root).derive("quantum-volume");
//! let circuit_stream = qv_stream.fork(0);
//!
//! let permutation = circuit_stream.shuffle(&mut qubits);
//! ```
//!
//! The exact quantum circuit generation remains owned by the caller.

use std::fmt;
use std::str::FromStr;

use rand::rngs::OsRng;
use rand::RngCore;

// =============================================================================
// Public constants
// =============================================================================

/// Stable identifier for the deterministic benchmark RNG algorithm.
///
/// This identifier is part of benchmark provenance and reproducibility.
pub const RANDOM_ALGORITHM_ID: &str = "splitmix64-zamani-v1";

/// Version of the deterministic random API.
pub const RANDOM_API_VERSION: u32 = 1;

/// Number of bytes in a Zamani benchmark seed.
pub const SEED_BYTES: usize = 32;

/// Number of 64-bit words in a Zamani benchmark seed.
pub const SEED_WORDS: usize = 4;

/// Number of bits used to construct a uniformly distributed `f64` in
/// `[0, 1)`.
const F64_MANTISSA_BITS: u32 = 53;

/// Golden-ratio increment used by SplitMix64.
///
/// This value is part of the algorithm definition and must not be changed
/// without changing `RANDOM_ALGORITHM_ID`.
const SPLITMIX_INCREMENT: u64 = 0x9E37_79B9_7F4A_7C15;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the benchmark random subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RandomError {
    /// A requested range was empty or inverted.
    InvalidRange {
        start: usize,
        end: usize,
    },

    /// A 64-bit range was empty or inverted.
    InvalidRangeU64 {
        start: u64,
        end: u64,
    },

    /// A collection was required to contain at least one element.
    EmptyCollection,

    /// The supplied seed text was malformed.
    InvalidSeed {
        reason: String,
    },

    /// Operating-system entropy could not be obtained.
    EntropyUnavailable,

    /// The stream/domain label was empty.
    EmptyDomain,

    /// The requested random operation would exceed the supported stream
    /// draw counter.
    DrawCounterOverflow,
}

impl fmt::Display for RandomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRange { start, end } => {
                write!(
                    f,
                    "invalid random range: start ({start}) must be less than end ({end})"
                )
            }

            Self::InvalidRangeU64 { start, end } => {
                write!(
                    f,
                    "invalid random u64 range: start ({start}) must be less than end ({end})"
                )
            }

            Self::EmptyCollection => {
                write!(f, "cannot select from or shuffle an empty collection")
            }

            Self::InvalidSeed { reason } => {
                write!(f, "invalid benchmark seed: {reason}")
            }

            Self::EntropyUnavailable => {
                write!(
                    f,
                    "operating-system entropy is unavailable for benchmark seed generation"
                )
            }

            Self::EmptyDomain => {
                write!(f, "random stream domain cannot be empty")
            }

            Self::DrawCounterOverflow => {
                write!(
                    f,
                    "benchmark random stream draw counter overflowed"
                )
            }
        }
    }
}

impl std::error::Error for RandomError {}

// =============================================================================
// Benchmark seed
// =============================================================================

/// A 256-bit benchmark seed.
///
/// The seed is intentionally larger than the internal SplitMix64 state.
/// This gives benchmark provenance a stable 256-bit identity while allowing
/// the deterministic generator to derive independent streams from it.
///
/// A seed is a value object and is safe to clone and copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchmarkSeed([u64; SEED_WORDS]);

impl BenchmarkSeed {
    /// Creates a seed from four explicit 64-bit words.
    ///
    /// This is the lowest-level deterministic constructor.
    pub const fn from_words(words: [u64; SEED_WORDS]) -> Self {
        Self(words)
    }

    /// Creates a deterministic seed from one 64-bit value.
    ///
    /// The remaining words are deterministically derived from the supplied
    /// value using the same stable mixing primitive used by the stream
    /// derivation system.
    pub fn from_u64(value: u64) -> Self {
        let mut state = value;

        let w0 = splitmix64_step(&mut state);
        let w1 = splitmix64_step(&mut state);
        let w2 = splitmix64_step(&mut state);
        let w3 = splitmix64_step(&mut state);

        Self([w0, w1, w2, w3])
    }

    /// Creates a seed from exactly 32 bytes in big-endian word order.
    pub fn from_bytes(bytes: [u8; SEED_BYTES]) -> Self {
        let mut words = [0u64; SEED_WORDS];

        let mut i = 0;

        while i < SEED_WORDS {
            let offset = i * 8;

            words[i] = u64::from_be_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]);

            i += 1;
        }

        Self(words)
    }

    /// Generates a fresh benchmark seed from operating-system entropy.
    ///
    /// Once returned, the seed is fully deterministic and can be persisted
    /// in benchmark provenance.
    pub fn from_entropy() -> Result<Self, RandomError> {
        let mut bytes = [0u8; SEED_BYTES];

        OsRng
            .try_fill_bytes(&mut bytes)
            .map_err(|_| RandomError::EntropyUnavailable)?;

        Ok(Self::from_bytes(bytes))
    }

    /// Returns the seed as four 64-bit words.
    pub const fn words(self) -> [u64; SEED_WORDS] {
        self.0
    }

    /// Returns the seed as exactly 32 bytes.
    pub fn to_bytes(self) -> [u8; SEED_BYTES] {
        let mut bytes = [0u8; SEED_BYTES];

        let mut i = 0;

        while i < SEED_WORDS {
            let encoded = self.0[i].to_be_bytes();
            let offset = i * 8;

            bytes[offset..offset + 8].copy_from_slice(&encoded);

            i += 1;
        }

        bytes
    }

    /// Returns a canonical lowercase hexadecimal representation.
    pub fn to_hex(self) -> String {
        let bytes = self.to_bytes();

        let mut output = String::with_capacity(SEED_BYTES * 2);

        for byte in bytes {
            output.push(hex_digit(byte >> 4));
            output.push(hex_digit(byte & 0x0f));
        }

        output
    }

    /// Derives a deterministic child seed from this seed and a domain.
    ///
    /// Domain separation is essential for benchmark reproducibility.
    ///
    /// For example, QV should not consume the same logical stream as a
    /// Clifford generator merely because both happen to start from seed 42.
    pub fn derive_domain(self, domain: &str) -> Result<Self, RandomError> {
        if domain.is_empty() {
            return Err(RandomError::EmptyDomain);
        }

        let mut state = seed_to_state(self);

        mix_bytes_into_state(&mut state, domain.as_bytes());

        let mut words = [0u64; SEED_WORDS];

        let mut i = 0;

        while i < SEED_WORDS {
            words[i] = splitmix64_step(&mut state);
            i += 1;
        }

        Ok(Self(words))
    }
}

impl Default for BenchmarkSeed {
    /// The default is deliberately deterministic.
    ///
    /// Production applications should normally use an explicit seed or
    /// `from_entropy()` and record the resulting value.
    fn default() -> Self {
        Self::from_u64(0)
    }
}

impl fmt::Display for BenchmarkSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl FromStr for BenchmarkSeed {
    type Err = RandomError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != SEED_BYTES * 2 {
            return Err(RandomError::InvalidSeed {
                reason: format!(
                    "expected {} hexadecimal characters, got {}",
                    SEED_BYTES * 2,
                    value.len()
                ),
            });
        }

        let bytes = value.as_bytes();
        let mut decoded = [0u8; SEED_BYTES];

        let mut i = 0;

        while i < SEED_BYTES {
            let high = hex_value(bytes[i * 2]).ok_or_else(|| {
                RandomError::InvalidSeed {
                    reason: format!(
                        "invalid hexadecimal character at position {}",
                        i * 2
                    ),
                }
            })?;

            let low = hex_value(bytes[i * 2 + 1]).ok_or_else(|| {
                RandomError::InvalidSeed {
                    reason: format!(
                        "invalid hexadecimal character at position {}",
                        i * 2 + 1
                    ),
                }
            })?;

            decoded[i] = (high << 4) | low;

            i += 1;
        }

        Ok(Self::from_bytes(decoded))
    }
}

// =============================================================================
// Random stream
// =============================================================================

/// A deterministic, independently reproducible random stream.
///
/// A stream contains:
///
/// - its root seed;
/// - its domain seed;
/// - its current internal state;
/// - its draw count.
///
/// The state is intentionally private so callers cannot accidentally violate
/// the generator's reproducibility invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomStream {
    seed: BenchmarkSeed,
    state: u64,
    draws: u64,
}

impl RandomStream {
    /// Creates the root deterministic stream for a benchmark.
    pub fn from_seed(seed: BenchmarkSeed) -> Self {
        let state = seed_to_state(seed);

        Self {
            seed,
            state,
            draws: 0,
        }
    }

    /// Creates a root stream using OS entropy.
    ///
    /// The generated seed should immediately be stored in benchmark
    /// provenance.
    pub fn from_entropy() -> Result<Self, RandomError> {
        Ok(Self::from_seed(BenchmarkSeed::from_entropy()?))
    }

    /// Returns the seed associated with this stream.
    pub const fn seed(&self) -> BenchmarkSeed {
        self.seed
    }

    /// Returns the stable algorithm identifier.
    pub const fn algorithm_id(&self) -> &'static str {
        RANDOM_ALGORITHM_ID
    }

    /// Returns the random API version.
    pub const fn api_version(&self) -> u32 {
        RANDOM_API_VERSION
    }

    /// Returns the number of random draws consumed by this stream.
    pub const fn draws(&self) -> u64 {
        self.draws
    }

    /// Derives an independent deterministic stream for a named domain.
    ///
    /// This operation does not consume the parent stream.
    ///
    /// Therefore:
    ///
    /// ```text
    /// root.derive("qv")
    /// ```
    ///
    /// always produces the same child stream for the same root seed.
    pub fn derive(&self, domain: &str) -> Result<Self, RandomError> {
        let child_seed = self.seed.derive_domain(domain)?;

        Ok(Self::from_seed(child_seed))
    }

    /// Derives an indexed child stream.
    ///
    /// This is the preferred mechanism for parallel benchmark experiments:
    ///
    /// ```text
    /// sequence 0
    /// sequence 1
    /// sequence 2
    /// ...
    /// ```
    ///
    /// Each stream is independently reproducible.
    pub fn fork(&self, index: u64) -> Self {
        let mut state = seed_to_state(self.seed);

        mix_u64_into_state(&mut state, index);

        let mut words = [0u64; SEED_WORDS];

        let mut i = 0;

        while i < SEED_WORDS {
            words[i] = splitmix64_step(&mut state);
            i += 1;
        }

        Self::from_seed(BenchmarkSeed::from_words(words))
    }

    /// Returns the next 64-bit random value.
    pub fn next_u64(&mut self) -> Result<u64, RandomError> {
        self.advance_draw_counter()?;

        Ok(splitmix64_step(&mut self.state))
    }

    /// Returns the next 32-bit random value.
    pub fn next_u32(&mut self) -> Result<u32, RandomError> {
        Ok(self.next_u64()? as u32)
    }

    /// Returns one random bit.
    pub fn next_bool(&mut self) -> Result<bool, RandomError> {
        Ok((self.next_u64()? & 1) != 0)
    }

    /// Returns a uniformly distributed floating-point number in `[0, 1)`.
    ///
    /// Exactly 53 random bits are used to construct the significand.
    pub fn next_f64(&mut self) -> Result<f64, RandomError> {
        let value = self.next_u64()?;

        let mantissa = value >> (64 - F64_MANTISSA_BITS);

        Ok((mantissa as f64) / ((1u64 << F64_MANTISSA_BITS) as f64))
    }

    /// Returns a uniformly distributed `usize` in `[start, end)`.
    ///
    /// Rejection sampling is used instead of `%` so that arbitrary ranges do
    /// not introduce modulo bias.
    pub fn range_usize(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<usize, RandomError> {
        if start >= end {
            return Err(RandomError::InvalidRange { start, end });
        }

        let span = end - start;

        if span == 1 {
            return Ok(start);
        }

        let span_u64 = u64::try_from(span).map_err(|_| {
            RandomError::InvalidRange { start, end }
        })?;

        let value = self.range_u64(0, span_u64)?;

        let offset = usize::try_from(value).map_err(|_| {
            RandomError::InvalidRange { start, end }
        })?;

        Ok(start + offset)
    }

    /// Returns a uniformly distributed `u64` in `[start, end)`.
    ///
    /// Rejection sampling eliminates modulo bias.
    pub fn range_u64(
        &mut self,
        start: u64,
        end: u64,
    ) -> Result<u64, RandomError> {
        if start >= end {
            return Err(RandomError::InvalidRangeU64 { start, end });
        }

        let span = end - start;

        // If the range spans all possible u64 values, subtraction cannot
        // represent it as a positive u64. This branch is impossible because
        // start < end, but keeping the normal algorithm below handles every
        // representable half-open interval.
        let threshold = span.wrapping_neg() % span;

        loop {
            let value = self.next_u64()?;

            if value >= threshold {
                return Ok(start + (value % span));
            }
        }
    }

    /// Selects one element uniformly from a non-empty slice.
    pub fn choose<'a, T>(
        &mut self,
        values: &'a [T],
    ) -> Result<&'a T, RandomError> {
        if values.is_empty() {
            return Err(RandomError::EmptyCollection);
        }

        let index = self.range_usize(0, values.len())?;

        Ok(&values[index])
    }

    /// Selects one mutable element uniformly from a non-empty slice.
    pub fn choose_mut<'a, T>(
        &mut self,
        values: &'a mut [T],
    ) -> Result<&'a mut T, RandomError> {
        if values.is_empty() {
            return Err(RandomError::EmptyCollection);
        }

        let index = self.range_usize(0, values.len())?;

        Ok(&mut values[index])
    }

    /// Shuffles a slice in-place using Fisher-Yates.
    ///
    /// The algorithm is deterministic for a given stream state and produces
    /// an unbiased permutation under the underlying uniform integer sampler.
    pub fn shuffle<T>(
        &mut self,
        values: &mut [T],
    ) -> Result<(), RandomError> {
        if values.len() <= 1 {
            return Ok(());
        }

        let mut i = values.len();

        while i > 1 {
            i -= 1;

            let j = self.range_usize(0, i + 1)?;

            values.swap(i, j);
        }

        Ok(())
    }

    /// Creates a deterministic permutation of indices `0..length`.
    pub fn permutation(
        &mut self,
        length: usize,
    ) -> Result<Vec<usize>, RandomError> {
        let mut values: Vec<usize> = (0..length).collect();

        self.shuffle(&mut values)?;

        Ok(values)
    }

    /// Creates a vector containing `count` independent random `u64` values.
    pub fn values(
        &mut self,
        count: usize,
    ) -> Result<Vec<u64>, RandomError> {
        let mut values = Vec::with_capacity(count);

        for _ in 0..count {
            values.push(self.next_u64()?);
        }

        Ok(values)
    }

    /// Advances the stream by `count` generated values.
    ///
    /// This is intentionally implemented by consuming the generator rather
    /// than by exposing its internal state transition, preserving the
    /// generator's exact algorithmic semantics.
    pub fn advance(&mut self, count: u64) -> Result<(), RandomError> {
        if count > u64::MAX - self.draws {
            return Err(RandomError::DrawCounterOverflow);
        }

        for _ in 0..count {
            splitmix64_step(&mut self.state);
        }

        self.draws += count;

        Ok(())
    }

    /// Returns a copy of this stream positioned at the current state.
    pub fn checkpoint(&self) -> Self {
        self.clone()
    }

    /// Restores this stream from a previously captured checkpoint.
    pub fn restore(&mut self, checkpoint: &Self) {
        *self = checkpoint.clone();
    }

    fn advance_draw_counter(&mut self) -> Result<(), RandomError> {
        if self.draws == u64::MAX {
            return Err(RandomError::DrawCounterOverflow);
        }

        self.draws += 1;

        Ok(())
    }
}

// =============================================================================
// Random source trait
// =============================================================================

/// Common interface consumed by future benchmark generators.
///
/// The trait deliberately exposes only operations that benchmark generators
/// actually need. It prevents those generators from depending directly on the
/// internal SplitMix64 implementation.
///
/// This also permits future deterministic random implementations to be
/// substituted in tests or future benchmark versions.
pub trait RandomSource {
    /// Returns the next 64-bit random value.
    fn next_u64(&mut self) -> Result<u64, RandomError>;

    /// Returns a uniformly distributed `usize` in `[start, end)`.
    fn range_usize(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<usize, RandomError>;

    /// Returns a uniformly distributed `u64` in `[start, end)`.
    fn range_u64(
        &mut self,
        start: u64,
        end: u64,
    ) -> Result<u64, RandomError>;

    /// Returns a random boolean.
    fn next_bool(&mut self) -> Result<bool, RandomError>;

    /// Returns a random `f64` in `[0, 1)`.
    fn next_f64(&mut self) -> Result<f64, RandomError>;
}

impl RandomSource for RandomStream {
    fn next_u64(&mut self) -> Result<u64, RandomError> {
        RandomStream::next_u64(self)
    }

    fn range_usize(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<usize, RandomError> {
        RandomStream::range_usize(self, start, end)
    }

    fn range_u64(
        &mut self,
        start: u64,
        end: u64,
    ) -> Result<u64, RandomError> {
        RandomStream::range_u64(self, start, end)
    }

    fn next_bool(&mut self) -> Result<bool, RandomError> {
        RandomStream::next_bool(self)
    }

    fn next_f64(&mut self) -> Result<f64, RandomError> {
        RandomStream::next_f64(self)
    }
}

// =============================================================================
// Deterministic algorithm implementation
// =============================================================================

/// One SplitMix64 state transition.
///
/// This function defines part of the stable `splitmix64-zamani-v1` contract.
///
/// Do not alter it without introducing a new algorithm identifier.
#[inline]
fn splitmix64_step(state: &mut u64) -> u64 {
    *state = state.wrapping_add(SPLITMIX_INCREMENT);

    let mut z = *state;

    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);

    z ^ (z >> 31)
}

/// Converts the 256-bit seed into the initial 64-bit generator state.
#[inline]
fn seed_to_state(seed: BenchmarkSeed) -> u64 {
    let words = seed.words();

    let mut state = SPLITMIX_INCREMENT;

    let mut i = 0;

    while i < SEED_WORDS {
        mix_u64_into_state(&mut state, words[i]);
        i += 1;
    }

    state
}

/// Mixes a 64-bit value into a deterministic stream state.
#[inline]
fn mix_u64_into_state(state: &mut u64, value: u64) {
    *state ^= value.wrapping_add(SPLITMIX_INCREMENT);

    *state = state
        .wrapping_mul(0xBF58_476D_1CE4_E5B9)
        .rotate_left(27);

    *state ^= *state >> 29;
}

/// Mixes arbitrary domain bytes into a deterministic state.
///
/// This is a deliberately specified lightweight domain-separation function.
/// It is NOT a cryptographic hash.
fn mix_bytes_into_state(state: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *state ^= u64::from(*byte);

        *state = state
            .wrapping_mul(0x1000_0000_01B3)
            .rotate_left(5);

        *state ^= *state >> 32;
    }

    // Domain terminator prevents simple concatenation ambiguity for callers
    // that construct multiple domains.
    mix_u64_into_state(state, bytes.len() as u64);
}

// =============================================================================
// Hexadecimal helpers
// =============================================================================

#[inline]
fn hex_digit(value: u8) -> char {
    match value & 0x0f {
        0..=9 => (b'0' + (value & 0x0f)) as char,
        value => (b'a' + (value - 10)) as char,
    }
}

#[inline]
fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algorithm_identity_is_stable() {
        assert_eq!(
            RANDOM_ALGORITHM_ID,
            "splitmix64-zamani-v1"
        );

        assert_eq!(RANDOM_API_VERSION, 1);
    }

    #[test]
    fn seed_from_u64_is_deterministic() {
        let first = BenchmarkSeed::from_u64(42);
        let second = BenchmarkSeed::from_u64(42);

        assert_eq!(first, second);
    }

    #[test]
    fn different_seeds_are_distinguishable() {
        let first = BenchmarkSeed::from_u64(1);
        let second = BenchmarkSeed::from_u64(2);

        assert_ne!(first, second);
    }

    #[test]
    fn seed_byte_round_trip_is_exact() {
        let original = BenchmarkSeed::from_words([
            0x0123_4567_89AB_CDEF,
            0x1020_3040_5060_7080,
            0xFEDC_BA98_7654_3210,
            0xFFE1_D2C3_B4A5_9687,
        ]);

        let bytes = original.to_bytes();
        let restored = BenchmarkSeed::from_bytes(bytes);

        assert_eq!(original, restored);
    }

    #[test]
    fn seed_hex_round_trip_is_exact() {
        let original = BenchmarkSeed::from_u64(123_456_789);

        let encoded = original.to_hex();
        let restored: BenchmarkSeed =
            encoded.parse().expect("valid seed must parse");

        assert_eq!(original, restored);
    }

    #[test]
    fn invalid_seed_length_is_rejected() {
        let result = "abcd".parse::<BenchmarkSeed>();

        assert!(matches!(
            result,
            Err(RandomError::InvalidSeed { .. })
        ));
    }

    #[test]
    fn invalid_seed_character_is_rejected() {
        let value = "z".repeat(SEED_BYTES * 2);

        let result = value.parse::<BenchmarkSeed>();

        assert!(matches!(
            result,
            Err(RandomError::InvalidSeed { .. })
        ));
    }

    #[test]
    fn root_stream_is_deterministic() {
        let seed = BenchmarkSeed::from_u64(42);

        let mut first = RandomStream::from_seed(seed);
        let mut second = RandomStream::from_seed(seed);

        for _ in 0..1000 {
            assert_eq!(
                first.next_u64().unwrap(),
                second.next_u64().unwrap()
            );
        }

        assert_eq!(first.draws(), 1000);
        assert_eq!(second.draws(), 1000);
    }

    #[test]
    fn stream_clone_is_deterministic() {
        let seed = BenchmarkSeed::from_u64(42);

        let mut original = RandomStream::from_seed(seed);

        let _ = original.next_u64().unwrap();
        let _ = original.next_u64().unwrap();

        let mut clone = original.clone();

        for _ in 0..100 {
            assert_eq!(
                original.next_u64().unwrap(),
                clone.next_u64().unwrap()
            );
        }
    }

    #[test]
    fn domain_derivation_is_deterministic() {
        let root = RandomStream::from_seed(
            BenchmarkSeed::from_u64(42),
        );

        let mut first = root.derive("quantum-volume").unwrap();
        let mut second = root.derive("quantum-volume").unwrap();

        for _ in 0..100 {
            assert_eq!(
                first.next_u64().unwrap(),
                second.next_u64().unwrap()
            );
        }
    }

    #[test]
    fn different_domains_are_independent() {
        let root = RandomStream::from_seed(
            BenchmarkSeed::from_u64(42),
        );

        let mut qv = root.derive("quantum-volume").unwrap();
        let mut rb = root.derive("randomized-benchmarking").unwrap();

        let mut equal_count = 0;

        for _ in 0..100 {
            if qv.next_u64().unwrap() == rb.next_u64().unwrap() {
                equal_count += 1;
            }
        }

        // This is a sanity check rather than a statistical proof.
        assert!(equal_count < 5);
    }

    #[test]
    fn empty_domain_is_rejected() {
        let root = RandomStream::from_seed(
            BenchmarkSeed::from_u64(42),
        );

        let result = root.derive("");

        assert_eq!(
            result,
            Err(RandomError::EmptyDomain)
        );
    }

    #[test]
    fn fork_is_deterministic() {
        let root = RandomStream::from_seed(
            BenchmarkSeed::from_u64(42),
        );

        let mut first = root.fork(7);
        let mut second = root.fork(7);

        for _ in 0..100 {
            assert_eq!(
                first.next_u64().unwrap(),
                second.next_u64().unwrap()
            );
        }
    }

    #[test]
    fn different_forks_are_distinct() {
        let root = RandomStream::from_seed(
            BenchmarkSeed::from_u64(42),
        );

        let mut first = root.fork(0);
        let mut second = root.fork(1);

        assert_ne!(
            first.next_u64().unwrap(),
            second.next_u64().unwrap()
        );
    }

    #[test]
    fn range_is_within_bounds() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        for _ in 0..10_000 {
            let value = stream.range_usize(10, 20).unwrap();

            assert!(value >= 10);
            assert!(value < 20);
        }
    }

    #[test]
    fn u64_range_is_within_bounds() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        for _ in 0..10_000 {
            let value = stream
                .range_u64(1_000, 2_000)
                .unwrap();

            assert!(value >= 1_000);
            assert!(value < 2_000);
        }
    }

    #[test]
    fn invalid_ranges_are_rejected() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        assert!(matches!(
            stream.range_usize(10, 10),
            Err(RandomError::InvalidRange { .. })
        ));

        assert!(matches!(
            stream.range_usize(20, 10),
            Err(RandomError::InvalidRange { .. })
        ));

        assert!(matches!(
            stream.range_u64(10, 10),
            Err(RandomError::InvalidRangeU64 { .. })
        ));

        assert!(matches!(
            stream.range_u64(20, 10),
            Err(RandomError::InvalidRangeU64 { .. })
        ));
    }

    #[test]
    fn singleton_range_is_exact() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        for _ in 0..100 {
            assert_eq!(
                stream.range_usize(7, 8).unwrap(),
                7
            );
        }
    }

    #[test]
    fn random_f64_is_in_half_open_unit_interval() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        for _ in 0..10_000 {
            let value = stream.next_f64().unwrap();

            assert!(value >= 0.0);
            assert!(value < 1.0);
            assert!(value.is_finite());
        }
    }

    #[test]
    fn choose_returns_an_element() {
        let values = [10, 20, 30, 40];

        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        for _ in 0..100 {
            let value = *stream.choose(&values).unwrap();

            assert!(values.contains(&value));
        }
    }

    #[test]
    fn empty_choose_is_rejected() {
        let values: [u64; 0] = [];

        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        assert_eq!(
            stream.choose(&values),
            Err(RandomError::EmptyCollection)
        );
    }

    #[test]
    fn shuffle_is_deterministic() {
        let original: Vec<usize> =
            (0..100).collect();

        let mut first = original.clone();
        let mut second = original.clone();

        let seed = BenchmarkSeed::from_u64(123);

        let mut first_stream =
            RandomStream::from_seed(seed);

        let mut second_stream =
            RandomStream::from_seed(seed);

        first_stream.shuffle(&mut first).unwrap();
        second_stream.shuffle(&mut second).unwrap();

        assert_eq!(first, second);
        assert_ne!(first, original);

        let mut sorted = first.clone();
        sorted.sort_unstable();

        assert_eq!(sorted, original);
    }

    #[test]
    fn permutation_contains_every_index_once() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        let permutation =
            stream.permutation(100).unwrap();

        assert_eq!(permutation.len(), 100);

        let mut sorted = permutation.clone();
        sorted.sort_unstable();

        assert_eq!(
            sorted,
            (0..100).collect::<Vec<_>>()
        );
    }

    #[test]
    fn empty_permutation_is_valid() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        let permutation =
            stream.permutation(0).unwrap();

        assert!(permutation.is_empty());
    }

    #[test]
    fn one_element_shuffle_is_unchanged() {
        let mut values = vec![42];

        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        stream.shuffle(&mut values).unwrap();

        assert_eq!(values, vec![42]);
    }

    #[test]
    fn advance_matches_consumption() {
        let seed = BenchmarkSeed::from_u64(999);

        let mut consumed =
            RandomStream::from_seed(seed);

        let mut advanced =
            RandomStream::from_seed(seed);

        for _ in 0..100 {
            consumed.next_u64().unwrap();
        }

        advanced.advance(100).unwrap();

        assert_eq!(
            consumed.next_u64().unwrap(),
            advanced.next_u64().unwrap()
        );

        assert_eq!(
            consumed.draws(),
            advanced.draws()
        );
    }

    #[test]
    fn checkpoint_and_restore_reproduce_sequence() {
        let seed = BenchmarkSeed::from_u64(123);

        let mut stream =
            RandomStream::from_seed(seed);

        let checkpoint = stream.checkpoint();

        let first = stream.next_u64().unwrap();
        let second = stream.next_u64().unwrap();

        stream.restore(&checkpoint);

        assert_eq!(
            stream.next_u64().unwrap(),
            first
        );

        assert_eq!(
            stream.next_u64().unwrap(),
            second
        );
    }

    #[test]
    fn random_source_trait_is_usable() {
        fn consume(
            source: &mut dyn RandomSource,
        ) -> Result<u64, RandomError> {
            source.next_u64()
        }

        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        let _ = consume(&mut stream).unwrap();
    }

    #[test]
    fn generated_values_are_not_constant() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        let first = stream.next_u64().unwrap();

        let mut changed = false;

        for _ in 0..100 {
            if stream.next_u64().unwrap() != first {
                changed = true;
                break;
            }
        }

        assert!(changed);
    }

    #[test]
    fn draw_counter_is_incremented_once_per_value() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        assert_eq!(stream.draws(), 0);

        stream.next_u64().unwrap();

        assert_eq!(stream.draws(), 1);

        stream.next_bool().unwrap();

        assert_eq!(stream.draws(), 2);

        stream.next_f64().unwrap();

        assert_eq!(stream.draws(), 3);
    }

    #[test]
    fn bulk_values_have_requested_length() {
        let mut stream = RandomStream::from_seed(
            BenchmarkSeed::from_u64(123),
        );

        let values = stream.values(1_000).unwrap();

        assert_eq!(values.len(), 1_000);
        assert_eq!(stream.draws(), 1_000);
    }

    #[test]
    fn seed_display_is_canonical() {
        let seed = BenchmarkSeed::from_words([
            0,
            0,
            0,
            0,
        ]);

        let text = seed.to_string();

        assert_eq!(
            text.len(),
            SEED_BYTES * 2
        );

        assert!(text
            .chars()
            .all(|character| {
                character.is_ascii_hexdigit()
                    && !character.is_ascii_uppercase()
            }));
    }
}