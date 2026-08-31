//! Zamani Quantum IR — Pulse Sample Semantics
//!
//! Canonical, hardware-independent representation and storage semantics for
//! explicitly sampled pulse/waveform data.
//!
//! # Architectural role
//!
//! `quantum::ir::pulse::sample` defines WHAT explicitly sampled signal data is.
//!
//! It owns:
//!
//! - sample indexing;
//! - sample sequences;
//! - sample storage;
//! - deterministic sample traversal;
//! - finite-value validation;
//! - explicit sample-storage validation policies;
//! - checked sample-count arithmetic;
//! - chunk-oriented sample construction;
//! - immutable sample access;
//! - conversion boundaries between semantic sample data and storage;
//! - local structural validation.
//!
//! It does NOT own:
//!
//! - waveform mathematical definitions;
//! - pulse scheduling;
//! - hardware sample clocks;
//! - DAC/ADC configuration;
//! - physical channels;
//! - physical qubits;
//! - logical-to-physical mapping;
//! - interpolation;
//! - resampling;
//! - clipping;
//! - quantization;
//! - normalization;
//! - calibration execution;
//! - hardware execution;
//! - backend SDKs;
//! - simulator state;
//! - frontend syntax.
//!
//! Those responsibilities belong to the corresponding IR or downstream
//! subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! pulse semantics              waveform semantics
//!      |                             |
//!      +-------------+---------------+
//!                    |
//!                    v
//!              sampled data
//!                    |
//!                    v
//!              optimization
//!                    |
//!                    v
//!               scheduling
//!                    |
//!                    v
//!                hardware
//!                    |
//!                    v
//!                 backend
//! ```
//!
//! # Separation from waveform.rs
//!
//! `waveform.rs` owns the mathematical/semantic waveform definition.
//!
//! This module owns the explicit sampled representation of a waveform.
//!
//! Therefore:
//!
//! ```text
//! WaveformDefinition
//!       |
//!       | may be materialized by a downstream transformation
//!       v
//! SampleSequence
//!       |
//!       v
//! target-specific representation
//! ```
//!
//! This module MUST NOT silently evaluate a parametric waveform.
//!
//! It also MUST NOT silently:
//!
//! - interpolate samples;
//! - resample samples;
//! - reorder samples;
//! - normalize samples;
//! - clip samples;
//! - quantize samples;
//! - change sample precision.
//!
//! Such transformations must be explicit downstream operations.
//!
//! # Universal-program principle
//!
//! The number of samples is data, not an architectural limit.
//!
//! Valid programs may contain:
//!
//! ```text
//! 0 samples
//! 1 sample
//! 10 samples
//! 1_000 samples
//! 1_000_000 samples
//! N samples
//! ```
//!
//! No constant in this file defines the maximum number of samples that
//! Zamani supports.
//!
//! Actual limits are imposed only by:
//!
//! - available memory;
//! - address-space limits;
//! - caller-provided resource policies;
//! - serialization policies;
//! - compilation-service policies;
//! - target hardware capabilities.
//!
//! Those are execution/resource constraints, not semantic limits.
//!
//! # Representation
//!
//! A sample sequence stores values in source order:
//!
//! ```text
//! index 0 -> first sample
//! index 1 -> second sample
//! ...
//! index N-1 -> last sample
//! ```
//!
//! The semantic sample index is `u64`, not `usize`.
//!
//! `usize` is used only internally when interacting with Rust's contiguous
//! `Vec` storage. This prevents the host container index type from becoming
//! part of the semantic IR model.
//!
//! # Existing waveform integration
//!
//! The repository's canonical waveform module already owns `ComplexSample`.
//! This file therefore reuses:
//!
//! ```text
//! quantum::ir::pulse::waveform::ComplexSample
//! ```
//!
//! rather than creating a second incompatible complex-sample type.
//!
//! This is critical for maintaining one canonical numerical representation.
//!
//! # Qubit integration
//!
//! Sample storage itself is independent of qubits.
//!
//! A sample sequence may eventually be referenced by a pulse that targets:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! but sample storage must not contain a qubit identifier.
//!
//! Physical qubit placement belongs to mapping/routing.
//!
//! Therefore this module intentionally does not import `QubitId`.
//!
//! # Timing integration
//!
//! Sample storage does not decide how samples map to time.
//!
//! The waveform/pulse/timing layers may associate a `SampleSequence` with a
//! semantic sample rate or duration.
//!
//! This prevents a storage representation from accidentally becoming a
//! hardware-clock representation.
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
//! - no external dependencies;
//! - no unsafe code.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden at compile time.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Determinism
//!
//! Sample order is semantically significant.
//!
//! The following must always hold:
//!
//! ```text
//! samples.get(0) == first source sample
//! samples.get(1) == second source sample
//! ...
//! ```
//!
//! No unordered collection is used for sample storage.
//!
//! # Canonical serialization
//!
//! Serialization belongs to `quantum::ir::serialization`.
//!
//! This module exposes deterministic accessors and traversal but does not
//! define a second serialization format.
//!
//! A serializer should consume:
//!
//! ```text
//! SampleSequence::format()
//! SampleSequence::len()
//! SampleSequence::sample(index)
//! ```
//!
//! in ascending semantic index order.
//!
//! # Canonical hashing
//!
//! Hashing belongs to `quantum::ir::hash`.
//!
//! A canonical hash should incorporate:
//!
//! - sample representation/format;
//! - semantic sample count;
//! - sample values in ascending index order;
//! - any semantically relevant metadata owned by the enclosing waveform.
//!
//! It must not hash Rust allocation addresses or container capacity.
//!
//! # File completion contract
//!
//! This file is deliberately complete with respect to its ownership boundary.
//!
//! Later implementation of:
//!
//! - scheduling;
//! - hardware;
//! - mapping;
//! - routing;
//! - calibration;
//! - backend execution;
//! - QEC;
//! - simulators
//!
//! must not require changing the semantic meaning of this file.
//!
//! Integration is performed by consuming the public types and methods defined
//! here.
//!
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::slice;

use super::waveform::ComplexSample;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for explicit pulse sample storage.
pub const SAMPLE_SCHEMA_ID: &str = "zamani.quantum.ir.pulse.sample";

/// Semantic schema major version.
pub const SAMPLE_SCHEMA_MAJOR: u16 = 1;

/// Semantic schema minor version.
pub const SAMPLE_SCHEMA_MINOR: u16 = 0;

/// Semantic schema patch version.
pub const SAMPLE_SCHEMA_PATCH: u16 = 0;

/// Returns the sample schema version.
#[must_use]
pub const fn sample_schema_version() -> (u16, u16, u16) {
    (
        SAMPLE_SCHEMA_MAJOR,
        SAMPLE_SCHEMA_MINOR,
        SAMPLE_SCHEMA_PATCH,
    )
}

// =============================================================================
// Result
// =============================================================================

/// Result type for sample operations.
pub type SampleResult<T> = Result<T, SampleError>;

// =============================================================================
// Semantic sample index
// =============================================================================

/// Stable semantic index of an explicitly sampled value.
///
/// `SampleIndex` is intentionally backed by `u64` rather than `usize`.
///
/// This means the semantic IR does not encode the host architecture's pointer
/// width into the sample model.
///
/// The actual storage implementation may still be constrained by the host's
/// address space and available memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SampleIndex(u64);

impl SampleIndex {
    /// Creates a sample index.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying semantic index.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the first sample index.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Returns the next index, if representable.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the previous index, if one exists.
    #[must_use]
    pub const fn checked_previous(self) -> Option<Self> {
        match self.0.checked_sub(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for SampleIndex {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<SampleIndex> for u64 {
    fn from(index: SampleIndex) -> u64 {
        index.value()
    }
}

impl fmt::Display for SampleIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "sample[{}]", self.0)
    }
}

// =============================================================================
// Sample format
// =============================================================================

/// Semantic representation of stored samples.
///
/// The format describes the data representation, not a hardware DAC format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SampleFormat {
    /// Real-valued samples.
    Real,

    /// Complex in-phase/quadrature samples.
    ComplexIq,
}

impl Default for SampleFormat {
    fn default() -> Self {
        Self::Real
    }
}

impl fmt::Display for SampleFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Real => formatter.write_str("real"),
            Self::ComplexIq => formatter.write_str("complex-iq"),
        }
    }
}

// =============================================================================
// Sample value
// =============================================================================

/// Canonical sample value used by explicit sample storage.
///
/// The actual numerical complex representation remains owned by
/// `pulse::waveform::ComplexSample`.
///
/// This wrapper exists so sample storage can explicitly distinguish:
///
/// - real samples;
/// - complex IQ samples.
///
/// It does not introduce a second complex-number implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SampleValue {
    /// Real-valued sample.
    Real(f64),

    /// Complex IQ sample.
    Complex(ComplexSample),
}

impl SampleValue {
    /// Creates a finite real-valued sample.
    pub fn real(value: f64) -> SampleResult<Self> {
        ensure_finite(value)?;
        Ok(Self::Real(value))
    }

    /// Creates a finite complex sample.
    pub fn complex(value: ComplexSample) -> SampleResult<Self> {
        value
            .validate()
            .map_err(|_| SampleError::NonFiniteValue)?;

        Ok(Self::Complex(value))
    }

    /// Returns the semantic format of the value.
    #[must_use]
    pub const fn format(self) -> SampleFormat {
        match self {
            Self::Real(_) => SampleFormat::Real,
            Self::Complex(_) => SampleFormat::ComplexIq,
        }
    }

    /// Returns whether the sample contains only finite values.
    #[must_use]
    pub fn is_finite(self) -> bool {
        match self {
            Self::Real(value) => value.is_finite(),
            Self::Complex(value) => value.is_finite(),
        }
    }

    /// Validates the numerical value.
    pub fn validate(self) -> SampleResult<()> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(SampleError::NonFiniteValue)
        }
    }

    /// Returns the real component when representable as a real sample.
    ///
    /// For a complex sample this returns the I component.
    #[must_use]
    pub fn real_component(self) -> f64 {
        match self {
            Self::Real(value) => value,
            Self::Complex(value) => value.i(),
        }
    }

    /// Returns the imaginary/quadrature component.
    ///
    /// Real samples have a Q component of zero.
    #[must_use]
    pub fn imaginary_component(self) -> f64 {
        match self {
            Self::Real(_) => 0.0,
            Self::Complex(value) => value.q(),
        }
    }
}

// =============================================================================
// Sample storage policy
// =============================================================================

/// Explicit policy controlling sample-sequence validation.
///
/// These values are resource/security policies only.
///
/// They are NOT limits on Zamani's quantum-computing capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleValidationPolicy {
    /// Maximum number of samples accepted by this validation operation.
    ///
    /// `None` means no policy-level count limit.
    pub max_samples: Option<u64>,

    /// Whether an empty sample sequence is allowed.
    pub allow_empty: bool,
}

impl Default for SampleValidationPolicy {
    fn default() -> Self {
        Self {
            max_samples: None,
            allow_empty: true,
        }
    }
}

impl SampleValidationPolicy {
    /// Creates an unlimited policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_samples: None,
            allow_empty: true,
        }
    }

    /// Creates a bounded policy.
    #[must_use]
    pub const fn bounded(max_samples: u64) -> Self {
        Self {
            max_samples: Some(max_samples),
            allow_empty: true,
        }
    }

    /// Creates a policy that requires at least one sample.
    #[must_use]
    pub const fn non_empty() -> Self {
        Self {
            max_samples: None,
            allow_empty: false,
        }
    }

    /// Validates the policy.
    pub const fn validate(self) -> SampleResult<()> {
        if let Some(max_samples) = self.max_samples {
            if max_samples == 0 && self.allow_empty {
                return Err(SampleError::InvalidValidationPolicy);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Sample chunk
// =============================================================================

/// A contiguous immutable semantic sample chunk.
///
/// Chunks provide a natural integration boundary for very large sampled
/// waveforms without imposing a fixed global waveform size.
///
/// A caller may construct multiple chunks and then combine them into a
/// `SampleSequence`.
///
/// The chunk's first semantic index is explicit rather than inferred from a
/// container position.
#[derive(Debug, Clone, PartialEq)]
pub struct SampleChunk {
    start: SampleIndex,
    samples: Vec<SampleValue>,
}

impl SampleChunk {
    /// Creates a chunk beginning at `start`.
    pub fn new(
        start: SampleIndex,
        samples: Vec<SampleValue>,
    ) -> SampleResult<Self> {
        validate_values(&samples)?;

        let chunk = Self { start, samples };

        chunk.validate()?;

        Ok(chunk)
    }

    /// Creates an empty chunk.
    ///
    /// Empty chunks are structurally valid and useful while constructing
    /// streaming/chunked pipelines, but callers may reject them through an
    /// explicit validation policy.
    #[must_use]
    pub const fn empty(start: SampleIndex) -> Self {
        Self {
            start,
            samples: Vec::new(),
        }
    }

    /// Returns the first semantic index.
    #[must_use]
    pub const fn start(&self) -> SampleIndex {
        self.start
    }

    /// Returns the number of samples in this chunk.
    ///
    /// This is a semantic count represented as `u64`.
    #[must_use]
    pub fn len(&self) -> u64 {
        self.samples.len() as u64
    }

    /// Returns whether this chunk contains no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Returns the first sample, if present.
    #[must_use]
    pub fn first(&self) -> Option<SampleValue> {
        self.samples.first().copied()
    }

    /// Returns the last sample, if present.
    #[must_use]
    pub fn last(&self) -> Option<SampleValue> {
        self.samples.last().copied()
    }

    /// Returns a sample by local zero-based offset.
    #[must_use]
    pub fn get(&self, offset: u64) -> Option<SampleValue> {
        let index = usize::try_from(offset).ok()?;
        self.samples.get(index).copied()
    }

    /// Returns the semantic index immediately after the chunk.
    ///
    /// Returns `None` if the semantic index space would overflow.
    #[must_use]
    pub fn end_exclusive(&self) -> Option<SampleIndex> {
        let len = self.len();

        match self.start.value().checked_add(len) {
            Some(value) => Some(SampleIndex::new(value)),
            None => None,
        }
    }

    /// Returns an immutable iterator over samples.
    pub fn iter(&self) -> impl Iterator<Item = SampleValue> + '_ {
        self.samples.iter().copied()
    }

    /// Returns the underlying samples as an immutable slice.
    ///
    /// The slice is an implementation-storage view and must not be treated as
    /// a semantic index type.
    #[must_use]
    pub fn as_slice(&self) -> &[SampleValue] {
        &self.samples
    }

    /// Validates this chunk.
    pub fn validate(&self) -> SampleResult<()> {
        validate_values(&self.samples)?;

        let len = self.len();

        if self.start.value().checked_add(len).is_none() {
            return Err(SampleError::IndexOverflow);
        }

        Ok(())
    }

    /// Converts the chunk into its owned sample vector.
    #[must_use]
    pub fn into_samples(self) -> Vec<SampleValue> {
        self.samples
    }
}

// =============================================================================
// Sample sequence
// =============================================================================

/// Explicitly sampled pulse/waveform data.
///
/// `SampleSequence` is the canonical storage abstraction for a finite sequence
/// of explicitly materialized samples.
///
/// It does not encode:
///
/// - hardware sample rate;
/// - DAC resolution;
/// - physical channel;
/// - qubit;
/// - timing placement.
///
/// Those belong to other IR layers.
///
/// # Scaling
///
/// No fixed sample count is encoded in the type.
///
/// The underlying Rust `Vec` naturally scales with available address space and
/// memory. A caller that needs stricter limits must use
/// `SampleValidationPolicy`.
///
/// For extremely large data, callers may construct and process
/// `SampleChunk`s incrementally instead of assuming that one allocation is
/// always appropriate.
#[derive(Debug, Clone, PartialEq)]
pub struct SampleSequence {
    format: SampleFormat,
    samples: Vec<SampleValue>,
}

impl SampleSequence {
    /// Creates an empty sequence with the requested sample format.
    #[must_use]
    pub fn empty(format: SampleFormat) -> Self {
        Self {
            format,
            samples: Vec::new(),
        }
    }

    /// Creates a sequence from explicit samples.
    pub fn new(
        format: SampleFormat,
        samples: Vec<SampleValue>,
    ) -> SampleResult<Self> {
        let sequence = Self { format, samples };

        sequence.validate()?;

        Ok(sequence)
    }

    /// Creates a real-valued sequence.
    pub fn from_real(samples: Vec<f64>) -> SampleResult<Self> {
        let values = samples
            .into_iter()
            .map(SampleValue::real)
            .collect::<SampleResult<Vec<_>>>()?;

        Self::new(SampleFormat::Real, values)
    }

    /// Creates a complex IQ sequence.
    pub fn from_complex(
        samples: Vec<ComplexSample>,
    ) -> SampleResult<Self> {
        let values = samples
            .into_iter()
            .map(SampleValue::complex)
            .collect::<SampleResult<Vec<_>>>()?;

        Self::new(SampleFormat::ComplexIq, values)
    }

    /// Returns the semantic sample format.
    #[must_use]
    pub const fn format(&self) -> SampleFormat {
        self.format
    }

    /// Returns the number of samples.
    ///
    /// The semantic count is represented as `u64`.
    #[must_use]
    pub fn len(&self) -> u64 {
        self.samples.len() as u64
    }

    /// Returns whether the sequence contains no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Returns the first sample.
    #[must_use]
    pub fn first(&self) -> Option<SampleValue> {
        self.samples.first().copied()
    }

    /// Returns the last sample.
    #[must_use]
    pub fn last(&self) -> Option<SampleValue> {
        self.samples.last().copied()
    }

    /// Returns the sample at the semantic index.
    #[must_use]
    pub fn get(&self, index: SampleIndex) -> Option<SampleValue> {
        let storage_index = usize::try_from(index.value()).ok()?;
        self.samples.get(storage_index).copied()
    }

    /// Returns a sample using a raw semantic index.
    #[must_use]
    pub fn get_u64(&self, index: u64) -> Option<SampleValue> {
        self.get(SampleIndex::new(index))
    }

    /// Returns an immutable sample iterator in semantic order.
    pub fn iter(&self) -> impl Iterator<Item = (SampleIndex, SampleValue)> + '_ {
        self.samples.iter().copied().enumerate().map(|(index, value)| {
            (
                SampleIndex::new(index as u64),
                value,
            )
        })
    }

    /// Returns only sample values in semantic order.
    pub fn values(&self) -> impl Iterator<Item = SampleValue> + '_ {
        self.samples.iter().copied()
    }

    /// Returns an immutable slice of storage.
    ///
    /// This is useful for serializers and backends that explicitly operate on
    /// contiguous sample storage.
    #[must_use]
    pub fn as_slice(&self) -> &[SampleValue] {
        &self.samples
    }

    /// Returns the underlying storage capacity.
    ///
    /// Capacity is implementation information and MUST NOT be serialized as
    /// semantic IR.
    #[must_use]
    pub fn storage_capacity(&self) -> usize {
        self.samples.capacity()
    }

    /// Appends one sample.
    ///
    /// The sample format must match the sequence format.
    pub fn push(&mut self, sample: SampleValue) -> SampleResult<SampleIndex> {
        self.validate_value_format(sample)?;

        let index = self
            .samples
            .len()
            .try_into()
            .map_err(|_| SampleError::IndexOverflow)?;

        self.samples.push(sample);

        Ok(SampleIndex::new(index))
    }

    /// Appends many samples in source order.
    ///
    /// The input is validated before mutation so invalid input does not result
    /// in a partially appended batch.
    pub fn extend<I>(&mut self, samples: I) -> SampleResult<SampleIndex>
    where
        I: IntoIterator<Item = SampleValue>,
    {
        let values: Vec<SampleValue> = samples.into_iter().collect();

        for &sample in &values {
            self.validate_value_format(sample)?;
        }

        let start = self
            .samples
            .len()
            .try_into()
            .map_err(|_| SampleError::IndexOverflow)?;

        self.samples.extend(values);

        Ok(SampleIndex::new(start))
    }

    /// Appends all samples from another sequence.
    ///
    /// The formats must match.
    pub fn append_sequence(
        &mut self,
        other: &SampleSequence,
    ) -> SampleResult<SampleIndex> {
        if self.format != other.format {
            return Err(SampleError::FormatMismatch {
                expected: self.format,
                actual: other.format,
            });
        }

        let start = self
            .samples
            .len()
            .try_into()
            .map_err(|_| SampleError::IndexOverflow)?;

        self.samples.extend_from_slice(&other.samples);

        Ok(SampleIndex::new(start))
    }

    /// Creates a sequence from one contiguous chunk.
    pub fn from_chunk(
        format: SampleFormat,
        chunk: SampleChunk,
    ) -> SampleResult<Self> {
        if chunk.start() != SampleIndex::zero() {
            return Err(SampleError::NonContiguousChunk {
                expected: SampleIndex::zero(),
                actual: chunk.start(),
            });
        }

        Self::new(format, chunk.into_samples())
    }

    /// Creates a sequence by concatenating contiguous chunks.
    ///
    /// Chunks must begin exactly where the previous chunk ends.
    pub fn from_chunks<I>(
        format: SampleFormat,
        chunks: I,
    ) -> SampleResult<Self>
    where
        I: IntoIterator<Item = SampleChunk>,
    {
        let mut sequence = Self::empty(format);
        let mut expected = SampleIndex::zero();

        for chunk in chunks {
            if chunk.start() != expected {
                return Err(SampleError::NonContiguousChunk {
                    expected,
                    actual: chunk.start(),
                });
            }

            for sample in chunk.iter() {
                sequence.validate_value_format(sample)?;
            }

            sequence.samples.extend_from_slice(chunk.as_slice());

            expected = chunk
                .end_exclusive()
                .ok_or(SampleError::IndexOverflow)?;
        }

        Ok(sequence)
    }

    /// Validates the sequence using the default unlimited policy.
    pub fn validate(&self) -> SampleResult<()> {
        self.validate_with_policy(SampleValidationPolicy::default())
    }

    /// Validates the sequence using an explicit resource policy.
    pub fn validate_with_policy(
        &self,
        policy: SampleValidationPolicy,
    ) -> SampleResult<()> {
        policy.validate()?;

        let count = self.len();

        if !policy.allow_empty && count == 0 {
            return Err(SampleError::EmptySequence);
        }

        if let Some(maximum) = policy.max_samples {
            if count > maximum {
                return Err(SampleError::SampleCountLimitExceeded {
                    count,
                    maximum,
                });
            }
        }

        for &sample in &self.samples {
            self.validate_value_format(sample)?;
        }

        Ok(())
    }

    /// Returns the number of samples as a checked `usize`.
    ///
    /// This is intended only for interaction with host-language containers.
    pub fn len_usize(&self) -> usize {
        self.samples.len()
    }

    /// Returns an immutable iterator over the underlying storage.
    ///
    /// This method is primarily useful for integration with serializers and
    /// numerical processing code that already operates on slices.
    pub fn storage_iter(&self) -> slice::Iter<'_, SampleValue> {
        self.samples.iter()
    }

    /// Consumes the sequence and returns its owned storage.
    #[must_use]
    pub fn into_samples(self) -> Vec<SampleValue> {
        self.samples
    }

    fn validate_value_format(
        &self,
        sample: SampleValue,
    ) -> SampleResult<()> {
        sample.validate()?;

        if sample.format() != self.format {
            return Err(SampleError::FormatMismatch {
                expected: self.format,
                actual: sample.format(),
            });
        }

        Ok(())
    }
}

impl Default for SampleSequence {
    fn default() -> Self {
        Self::empty(SampleFormat::Real)
    }
}

// =============================================================================
// Sample statistics
// =============================================================================

/// Read-only numerical information about a sample sequence.
///
/// This structure contains derived information only. It is not part of the
/// semantic sample storage and therefore should not be serialized as canonical
/// sample data unless explicitly requested by an analysis layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SampleStatistics {
    /// Number of samples.
    pub count: u64,

    /// Maximum absolute magnitude observed.
    pub peak_magnitude: f64,

    /// Sum of squared magnitudes.
    pub energy: f64,
}

impl SampleStatistics {
    /// Calculates deterministic statistics without modifying the sequence.
    ///
    /// Arithmetic overflow is rejected rather than silently producing
    /// infinity.
    pub fn calculate(sequence: &SampleSequence) -> SampleResult<Self> {
        let mut peak = 0.0_f64;
        let mut energy = 0.0_f64;

        for sample in sequence.values() {
            let magnitude_squared = match sample {
                SampleValue::Real(value) => {
                    ensure_finite(value)?;

                    let squared = value * value;

                    if !squared.is_finite() {
                        return Err(SampleError::NumericalOverflow);
                    }

                    squared
                }

                SampleValue::Complex(value) => value
                    .magnitude_squared()
                    .map_err(|_| SampleError::NumericalOverflow)?,
            };

            let magnitude = magnitude_squared.sqrt();

            if !magnitude.is_finite() {
                return Err(SampleError::NumericalOverflow);
            }

            if magnitude > peak {
                peak = magnitude;
            }

            energy += magnitude_squared;

            if !energy.is_finite() {
                return Err(SampleError::NumericalOverflow);
            }
        }

        Ok(Self {
            count: sequence.len(),
            peak_magnitude: peak,
            energy,
        })
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by sample storage and validation.
#[derive(Debug, Clone, PartialEq)]
pub enum SampleError {
    /// A numerical sample contains NaN or infinity.
    NonFiniteValue,

    /// Numerical arithmetic overflowed to a non-finite result.
    NumericalOverflow,

    /// A semantic sample index would overflow.
    IndexOverflow,

    /// The supplied sequence is empty when the selected policy forbids it.
    EmptySequence,

    /// A caller supplied an impossible validation policy.
    InvalidValidationPolicy,

    /// A sample's representation does not match the sequence format.
    FormatMismatch {
        /// Required sequence format.
        expected: SampleFormat,

        /// Actual sample format.
        actual: SampleFormat,
    },

    /// A sample count exceeded an explicit caller-provided policy.
    SampleCountLimitExceeded {
        /// Actual number of samples.
        count: u64,

        /// Policy maximum.
        maximum: u64,
    },

    /// Chunks do not form one contiguous semantic sequence.
    NonContiguousChunk {
        /// Required next chunk start.
        expected: SampleIndex,

        /// Actual chunk start.
        actual: SampleIndex,
    },
}

impl fmt::Display for SampleError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NonFiniteValue => {
                formatter.write_str(
                    "sample contains a non-finite numerical value",
                )
            }

            Self::NumericalOverflow => {
                formatter.write_str(
                    "sample numerical operation produced a non-finite result",
                )
            }

            Self::IndexOverflow => {
                formatter.write_str(
                    "sample semantic index would overflow",
                )
            }

            Self::EmptySequence => {
                formatter.write_str(
                    "sample sequence is empty but the validation policy requires data",
                )
            }

            Self::InvalidValidationPolicy => {
                formatter.write_str(
                    "sample validation policy is internally inconsistent",
                )
            }

            Self::FormatMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "sample format mismatch: expected {expected}, got {actual}"
                )
            }

            Self::SampleCountLimitExceeded {
                count,
                maximum,
            } => {
                write!(
                    formatter,
                    "sample count {count} exceeds explicit validation limit {maximum}"
                )
            }

            Self::NonContiguousChunk {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "non-contiguous sample chunk: expected start {}, got {}",
                    expected,
                    actual
                )
            }
        }
    }
}

impl std::error::Error for SampleError {}

// =============================================================================
// Internal numerical validation
// =============================================================================

fn ensure_finite(value: f64) -> SampleResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SampleError::NonFiniteValue)
    }
}

fn validate_values(values: &[SampleValue]) -> SampleResult<()> {
    for &value in values {
        value.validate()?;
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            SAMPLE_SCHEMA_ID,
            "zamani.quantum.ir.pulse.sample"
        );

        assert_eq!(
            sample_schema_version(),
            (1, 0, 0)
        );
    }

    #[test]
    fn sample_index_is_stable() {
        let index = SampleIndex::new(42);

        assert_eq!(index.value(), 42);
        assert_eq!(
            index.checked_next(),
            Some(SampleIndex::new(43))
        );
        assert_eq!(
            index.checked_previous(),
            Some(SampleIndex::new(41))
        );
    }

    #[test]
    fn sample_index_overflow_is_checked() {
        let index = SampleIndex::new(u64::MAX);

        assert_eq!(
            index.checked_next(),
            None
        );
    }

    #[test]
    fn real_sample_rejects_nan() {
        let result = SampleValue::real(f64::NAN);

        assert_eq!(
            result,
            Err(SampleError::NonFiniteValue)
        );
    }

    #[test]
    fn real_sample_rejects_infinity() {
        let result = SampleValue::real(f64::INFINITY);

        assert_eq!(
            result,
            Err(SampleError::NonFiniteValue)
        );
    }

    #[test]
    fn real_sample_accepts_finite_value() {
        let sample = SampleValue::real(0.3)
            .expect("finite sample should be accepted");

        assert_eq!(
            sample.format(),
            SampleFormat::Real
        );

        assert!(sample.is_finite());
    }

    #[test]
    fn complex_sample_uses_canonical_waveform_type() {
        let complex = ComplexSample::new(
            0.3,
            0.2,
        )
        .expect("finite complex sample should be accepted");

        let sample = SampleValue::complex(complex)
            .expect("canonical complex sample should be accepted");

        assert_eq!(
            sample.format(),
            SampleFormat::ComplexIq
        );

        assert!(sample.is_finite());
    }

    #[test]
    fn sequence_preserves_order() {
        let mut sequence = SampleSequence::empty(
            SampleFormat::Real,
        );

        let first = sequence
            .push(
                SampleValue::real(0.1)
                    .expect("finite sample"),
            )
            .expect("push should succeed");

        let second = sequence
            .push(
                SampleValue::real(0.2)
                    .expect("finite sample"),
            )
            .expect("push should succeed");

        assert_eq!(
            first,
            SampleIndex::new(0)
        );

        assert_eq!(
            second,
            SampleIndex::new(1)
        );

        assert_eq!(
            sequence.get_u64(0),
            Some(
                SampleValue::real(0.1)
                    .expect("finite sample")
            )
        );

        assert_eq!(
            sequence.get_u64(1),
            Some(
                SampleValue::real(0.2)
                    .expect("finite sample")
            )
        );
    }

    #[test]
    fn sequence_rejects_wrong_format() {
        let mut sequence = SampleSequence::empty(
            SampleFormat::Real,
        );

        let complex = ComplexSample::new(
            0.1,
            0.2,
        )
        .expect("finite complex sample");

        let result = sequence.push(
            SampleValue::complex(complex)
                .expect("finite complex sample"),
        );

        assert_eq!(
            result,
            Err(
                SampleError::FormatMismatch {
                    expected: SampleFormat::Real,
                    actual: SampleFormat::ComplexIq,
                }
            )
        );
    }

    #[test]
    fn sequence_can_be_created_from_real_values() {
        let sequence = SampleSequence::from_real(
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
        )
        .expect("finite real sequence");

        assert_eq!(
            sequence.format(),
            SampleFormat::Real
        );

        assert_eq!(
            sequence.len(),
            5
        );

        assert!(!sequence.is_empty());
    }

    #[test]
    fn sequence_can_be_created_from_complex_values() {
        let values = vec![
            ComplexSample::new(1.0, 0.0)
                .expect("finite sample"),
            ComplexSample::new(0.0, 1.0)
                .expect("finite sample"),
        ];

        let sequence = SampleSequence::from_complex(values)
            .expect("finite complex sequence");

        assert_eq!(
            sequence.format(),
            SampleFormat::ComplexIq
        );

        assert_eq!(
            sequence.len(),
            2
        );
    }

    #[test]
    fn chunk_indices_are_semantic() {
        let chunk = SampleChunk::new(
            SampleIndex::new(10),
            vec![
                SampleValue::real(1.0)
                    .expect("finite sample"),
                SampleValue::real(2.0)
                    .expect("finite sample"),
            ],
        )
        .expect("valid chunk");

        assert_eq!(
            chunk.start(),
            SampleIndex::new(10)
        );

        assert_eq!(
            chunk.end_exclusive(),
            Some(SampleIndex::new(12))
        );

        assert_eq!(
            chunk.get(0),
            Some(
                SampleValue::real(1.0)
                    .expect("finite sample")
            )
        );

        assert_eq!(
            chunk.get(1),
            Some(
                SampleValue::real(2.0)
                    .expect("finite sample")
            )
        );
    }

    #[test]
    fn contiguous_chunks_are_combined() {
        let first = SampleChunk::new(
            SampleIndex::new(0),
            vec![
                SampleValue::real(1.0)
                    .expect("finite sample"),
                SampleValue::real(2.0)
                    .expect("finite sample"),
            ],
        )
        .expect("valid first chunk");

        let second = SampleChunk::new(
            SampleIndex::new(2),
            vec![
                SampleValue::real(3.0)
                    .expect("finite sample"),
            ],
        )
        .expect("valid second chunk");

        let sequence = SampleSequence::from_chunks(
            SampleFormat::Real,
            vec![first, second],
        )
        .expect("contiguous chunks");

        assert_eq!(
            sequence.len(),
            3
        );

        assert_eq!(
            sequence.get_u64(2),
            Some(
                SampleValue::real(3.0)
                    .expect("finite sample")
            )
        );
    }

    #[test]
    fn non_contiguous_chunks_are_rejected() {
        let first = SampleChunk::new(
            SampleIndex::new(0),
            vec![
                SampleValue::real(1.0)
                    .expect("finite sample"),
            ],
        )
        .expect("valid first chunk");

        let second = SampleChunk::new(
            SampleIndex::new(3),
            vec![
                SampleValue::real(2.0)
                    .expect("finite sample"),
            ],
        )
        .expect("valid second chunk");

        let result = SampleSequence::from_chunks(
            SampleFormat::Real,
            vec![first, second],
        );

        assert_eq!(
            result,
            Err(
                SampleError::NonContiguousChunk {
                    expected: SampleIndex::new(1),
                    actual: SampleIndex::new(3),
                }
            )
        );
    }

    #[test]
    fn unlimited_policy_has_no_architectural_sample_limit() {
        let policy = SampleValidationPolicy::unlimited();

        assert_eq!(
            policy.max_samples,
            None
        );

        assert!(policy.allow_empty);
    }

    #[test]
    fn explicit_policy_can_limit_samples() {
        let sequence = SampleSequence::from_real(
            vec![0.0, 1.0, 2.0],
        )
        .expect("valid sequence");

        let result = sequence.validate_with_policy(
            SampleValidationPolicy::bounded(2),
        );

        assert_eq!(
            result,
            Err(
                SampleError::SampleCountLimitExceeded {
                    count: 3,
                    maximum: 2,
                }
            )
        );
    }

    #[test]
    fn non_empty_policy_rejects_empty_sequence() {
        let sequence = SampleSequence::empty(
            SampleFormat::Real,
        );

        let result = sequence.validate_with_policy(
            SampleValidationPolicy::non_empty(),
        );

        assert_eq!(
            result,
            Err(SampleError::EmptySequence)
        );
    }

    #[test]
    fn statistics_are_deterministic() {
        let sequence = SampleSequence::from_real(
            vec![1.0, 2.0, 3.0],
        )
        .expect("valid sequence");

        let statistics = SampleStatistics::calculate(
            &sequence,
        )
        .expect("finite statistics");

        assert_eq!(
            statistics.count,
            3
        );

        assert_eq!(
            statistics.peak_magnitude,
            3.0
        );

        assert_eq!(
            statistics.energy,
            14.0
        );
    }

    #[test]
    fn complex_statistics_are_supported() {
        let sequence = SampleSequence::from_complex(
            vec![
                ComplexSample::new(3.0, 4.0)
                    .expect("finite sample"),
            ],
        )
        .expect("valid sequence");

        let statistics = SampleStatistics::calculate(
            &sequence,
        )
        .expect("finite statistics");

        assert_eq!(
            statistics.count,
            1
        );

        assert_eq!(
            statistics.peak_magnitude,
            5.0
        );

        assert_eq!(
            statistics.energy,
            25.0
        );
    }

    #[test]
    fn sample_iteration_is_in_semantic_order() {
        let sequence = SampleSequence::from_real(
            vec![10.0, 20.0, 30.0],
        )
        .expect("valid sequence");

        let collected: Vec<(SampleIndex, SampleValue)> =
            sequence.iter().collect();

        assert_eq!(
            collected.len(),
            3
        );

        assert_eq!(
            collected[0].0,
            SampleIndex::new(0)
        );

        assert_eq!(
            collected[1].0,
            SampleIndex::new(1)
        );

        assert_eq!(
            collected[2].0,
            SampleIndex::new(2)
        );
    }

    #[test]
    fn sequence_append_requires_same_format() {
        let mut real = SampleSequence::from_real(
            vec![1.0],
        )
        .expect("valid sequence");

        let complex = SampleSequence::from_complex(
            vec![
                ComplexSample::new(1.0, 0.0)
                    .expect("finite sample"),
            ],
        )
        .expect("valid sequence");

        let result = real.append_sequence(
            &complex,
        );

        assert!(matches!(
            result,
            Err(SampleError::FormatMismatch { .. })
        ));
    }

    #[test]
    fn sample_storage_capacity_is_not_semantic() {
        let sequence = SampleSequence::from_real(
            vec![1.0, 2.0],
        )
        .expect("valid sequence");

        assert!(
            sequence.storage_capacity()
                >= sequence.len_usize()
        );
    }
}