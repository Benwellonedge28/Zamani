//! Zamani Quantum IR — Pulse Capture Semantics
//!
//! Canonical, hardware-independent representation of quantum signal
//! acquisition/capture.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This module answers:
//!
//!     "What quantum/control signal does the program request to acquire?"
//!
//! It does NOT answer:
//!
//!     "Which ADC, digitizer, detector, amplifier, readout chain, physical
//!      port, sample clock, discriminator, or vendor API implements it?"
//!
//! Hardware realization belongs to downstream target/hardware/backend layers.
//!
//! The canonical dependency direction is:
//!
//!     Zamani source
//!          |
//!          v
//!       frontend
//!          |
//!          v
//!     canonical Zamani IR
//!          |
//!          v
//!     pulse::capture
//!          |
//!     +----+-------------------+
//!     |                        |
//!     v                        v
//!   scheduling             validation
//!     |
//!     v
//!   mapping
//!     |
//!     v
//!   hardware
//!     |
//!     v
//!   backend
//!     |
//!     v
//!   execution
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - semantic capture intent;
//! - logical capture targets;
//! - capture frame reference;
//! - optional abstract acquisition channel reference;
//! - optional filter/kernel waveform reference;
//! - optional capture duration;
//! - capture output semantics;
//! - capture data shape;
//! - capture post-processing intent;
//! - capture metadata;
//! - capture validation policy;
//! - deterministic structural validation;
//! - capture-local errors.
//!
//! This file does NOT own:
//!
//! - physical ADCs;
//! - physical detectors;
//! - readout electronics;
//! - amplifiers;
//! - cables;
//! - physical ports;
//! - hardware sample rates;
//! - physical qubit allocation;
//! - routing;
//! - scheduling algorithms;
//! - calibration data;
//! - discriminator implementation;
//! - DSP implementation;
//! - backend execution;
//! - provider SDKs;
//! - authentication;
//! - simulator state;
//! - source-language parsing.
//!
//! ============================================================================
//! UNIVERSAL CAPTURE PRINCIPLE
//! ============================================================================
//!
//! A capture is an acquisition request, not a device instruction.
//!
//! A Zamani program may therefore express:
//!
//!     capture(frame, duration) -> raw waveform
//!     capture(frame, filter)   -> complex value
//!     capture(frame, filter)   -> bit
//!     capture(frame, duration) -> count
//!     capture(frame)           -> no direct result
//!
//! without knowing which physical acquisition system will implement it.
//!
//! This follows the OpenPulse model, where the capture command requires a
//! frame while duration/filter and result representation may be supplied by
//! an implementation-specific capture definition.
//!
//! ============================================================================
//! SCALABILITY CONTRACT
//! ============================================================================
//!
//! There is NO architectural maximum for:
//!
//! - number of capture operations;
//! - number of qubits;
//! - number of targets;
//! - number of capture channels;
//! - number of frames;
//! - number of capture results;
//! - number of samples;
//! - number of metadata entries;
//! - number of repetitions;
//! - number of programs.
//!
//! Any finite bound is an explicit validation/resource policy.
//!
//! The semantic model itself does not contain a fixed machine-size limit.
//!
//! This means the same capture representation can describe:
//!
//!     one qubit
//!     many qubits
//!     a large register
//!     a multiplexed readout
//!     a global acquisition
//!     a distributed acquisition resource
//!
//! subject only to the resources and capabilities available to the target.
//!
//! ============================================================================
//! LOGICAL-QUBIT CONTRACT
//! ============================================================================
//!
//! Explicit quantum targets use:
//!
//!     quantum::ir::qubit::QubitId
//!
//! A physical qubit must NOT be substituted for `QubitId`.
//!
//! Logical-to-physical placement belongs to mapping/routing.
//!
//! A capture may additionally reference an abstract `ResourceId` when the
//! acquisition target is not naturally represented as a logical qubit.
//!
//! ============================================================================
//! OPENQASM / OPENPULSE ALIGNMENT
//! ============================================================================
//!
//! OpenPulse models capture around a frame and permits implementations to
//! define additional parameters such as duration and filter/kernel.
//!
//! Capture may produce:
//!
//! - raw waveform data;
//! - integrated/filtered complex data;
//! - discriminated bits;
//! - counts;
//! - scalar numerical results;
//! - opaque implementation-defined results;
//! - no directly returned result.
//!
//! This module therefore deliberately does NOT define:
//!
//!     "capture always returns bit"
//!
//! or:
//!
//!     "capture always returns samples"
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//! - deterministic collections where ordering is observable
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! `identity.rs`
//!     Supplies `FrameId`, `ChannelId`, `WaveformId`, and `ResourceId`.
//!
//! `qubit.rs`
//!     Supplies canonical `QubitId`.
//!
//! `pulse/pulse.rs`
//!     Supplies `PulseDuration` for semantic capture duration.
//!
//! `frame.rs`
//!     Owns the meaning of the referenced frame.
//!
//! `channel.rs`
//!     Owns abstract acquisition/control-channel semantics.
//!
//! `waveform.rs`
//!     Owns waveform definitions used as capture filters/kernels.
//!
//! `operation.rs`
//!     May wrap this semantic capture specification in a universal IR
//!     operation and provide the canonical `OperationId`.
//!
//! `measurement.rs`
//!     May consume capture results when a capture is part of a measurement
//!     definition.
//!
//! `classical.rs`
//!     Owns classical result values after acquisition.
//!
//! `resource.rs` / `capability.rs`
//!     Determine whether a target can satisfy the capture request.
//!
//! `mapping.rs`
//!     Resolves logical qubits to physical resources.
//!
//! `schedule.rs` / `timing.rs`
//!     Determine actual execution timing.
//!
//! `hardware/`
//!     Determines physical acquisition implementation.
//!
//! `serialization.rs`
//!     Serializes the capture structure through the canonical IR format.
//!
//! `hash.rs`
//!     Includes semantic capture fields in canonical content identity.
//!
//! `provenance.rs`
//!     Tracks transformations of capture operations.
//!
//! ============================================================================
//! IMPORTANT FILE-COMPLETION GUARANTEE
//! ============================================================================
//!
//! This module contains its complete semantic contract:
//!
//! - target model;
//! - frame/channel/filter references;
//! - duration;
//! - output model;
//! - result shape;
//! - processing semantics;
//! - metadata;
//! - validation policy;
//! - checked construction;
//! - deterministic validation;
//! - accessors;
//! - tests;
//! - integration documentation.
//!
//! Adding later hardware, backend, routing, scheduling, or frontend modules
//! must not require changing this semantic model merely because those modules
//! are introduced.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::core::identity::{
    ChannelId,
    FrameId,
    ResourceId,
    WaveformId,
};
use super::super::quantum::qubit::QubitId;
use super::pulse::PulseDuration;

// ============================================================================
// Schema
// ============================================================================

/// Stable semantic schema identifier for capture operations.
pub const CAPTURE_SCHEMA_ID: &str = "zamani.quantum.ir.pulse.capture";

/// Semantic schema major version.
pub const CAPTURE_SCHEMA_MAJOR: u16 = 1;

/// Semantic schema minor version.
pub const CAPTURE_SCHEMA_MINOR: u16 = 0;

/// Semantic schema patch version.
pub const CAPTURE_SCHEMA_PATCH: u16 = 0;

// ============================================================================
// Result
// ============================================================================

/// Result type used by capture construction and validation.
pub type CaptureResult<T> = Result<T, CaptureError>;

// ============================================================================
// Capture target
// ============================================================================

/// Semantic target of a capture operation.
///
/// A capture target is intentionally independent of physical hardware.
///
/// `Qubit` and `Qubits` identify logical qubits through the canonical
/// `quantum::ir::qubit::QubitId`.
///
/// `Resource` permits acquisition resources that are not naturally expressed
/// as logical qubits.
///
/// `Global` represents an intentionally target-independent/global acquisition
/// whose concrete scope is resolved by a downstream target compiler.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CaptureTarget {
    /// Capture associated with one logical qubit.
    Qubit(QubitId),

    /// Capture associated with multiple logical qubits.
    ///
    /// The constructor canonicalizes this collection by sorting and removing
    /// duplicates.
    Qubits(Vec<QubitId>),

    /// Capture associated with an abstract IR resource.
    Resource(ResourceId),

    /// Capture whose concrete target scope is resolved downstream.
    Global,
}

impl CaptureTarget {
    /// Creates a single logical-qubit capture target.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates a global capture target.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Creates a resource capture target.
    #[must_use]
    pub const fn resource(resource: ResourceId) -> Self {
        Self::Resource(resource)
    }

    /// Creates a deterministic multi-qubit capture target.
    ///
    /// Duplicate qubit identifiers are removed.
    #[must_use]
    pub fn qubits<I>(qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut values: Vec<QubitId> = qubits.into_iter().collect();

        values.sort();
        values.dedup();

        Self::Qubits(values)
    }

    /// Returns whether this is a single logical-qubit target.
    #[must_use]
    pub const fn is_single_qubit(&self) -> bool {
        matches!(self, Self::Qubit(_))
    }

    /// Returns whether this is a global target.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns whether this target contains an explicit resource.
    #[must_use]
    pub const fn is_resource(&self) -> bool {
        matches!(self, Self::Resource(_))
    }

    /// Returns the number of explicitly represented logical qubits.
    ///
    /// Global and resource targets return zero because their concrete scope is
    /// not represented as a logical-qubit collection.
    #[must_use]
    pub fn explicit_qubit_count(&self) -> usize {
        match self {
            Self::Qubit(_) => 1,
            Self::Qubits(qubits) => qubits.len(),
            Self::Resource(_) | Self::Global => 0,
        }
    }

    /// Returns the explicitly represented logical qubits.
    ///
    /// Global and resource targets return an empty slice.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        match self {
            Self::Qubit(qubit) => std::slice::from_ref(qubit),
            Self::Qubits(qubits) => qubits.as_slice(),
            Self::Resource(_) | Self::Global => &[],
        }
    }

    /// Validates the target's canonical structure.
    pub fn validate(&self) -> CaptureResult<()> {
        match self {
            Self::Qubit(_) => Ok(()),

            Self::Qubits(qubits) => {
                if qubits.is_empty() {
                    return Err(CaptureError::EmptyTargetSet);
                }

                if qubits.windows(2).any(|pair| pair[0] >= pair[1]) {
                    return Err(CaptureError::NonCanonicalTargetSet);
                }

                Ok(())
            }

            Self::Resource(_) | Self::Global => Ok(()),
        }
    }
}

// ============================================================================
// Capture duration
// ============================================================================

/// Semantic duration specification for acquisition.
///
/// `Inferred` means the duration is determined by another semantic component
/// or by a downstream target-specific definition.
///
/// The IR intentionally does not require every capture to expose an explicit
/// duration because OpenPulse permits capture duration to be inferred by the
/// capture implementation.
///
/// `Explicit` remains exact and hardware-independent. Conversion to a
/// hardware-specific sample clock (`dt`) occurs downstream.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CaptureDuration {
    /// Duration is inferred from another semantic object or target definition.
    Inferred,

    /// Explicit semantic acquisition duration.
    Explicit(PulseDuration),
}

impl CaptureDuration {
    /// Creates an inferred duration.
    #[must_use]
    pub const fn inferred() -> Self {
        Self::Inferred
    }

    /// Creates an explicit duration.
    #[must_use]
    pub const fn explicit(duration: PulseDuration) -> Self {
        Self::Explicit(duration)
    }

    /// Returns whether the duration is explicit.
    #[must_use]
    pub const fn is_explicit(self) -> bool {
        matches!(self, Self::Explicit(_))
    }

    /// Returns the explicit duration if available.
    #[must_use]
    pub const fn as_explicit(self) -> Option<PulseDuration> {
        match self {
            Self::Inferred => None,
            Self::Explicit(duration) => Some(duration),
        }
    }
}

// ============================================================================
// Capture output kind
// ============================================================================

/// Semantic kind of data produced by a capture.
///
/// This describes the meaning of the requested result, not its physical memory
/// representation.
///
/// The backend may lower these results to a device-specific data format.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CaptureOutputKind {
    /// No directly returned result.
    ///
    /// The backend may store the acquired data in an implementation-defined
    /// buffer or side channel.
    None,

    /// Raw acquired waveform/signal samples.
    RawWaveform,

    /// Filtered/integrated complex signal.
    Complex,

    /// A classified/discriminated boolean or bit result.
    Bit,

    /// A non-negative detection/count result.
    Count,

    /// A signed integer result.
    Integer,

    /// A floating-point scalar result.
    Real,

    /// A backend-independent opaque result whose concrete schema is declared
    /// elsewhere through an extension/dialect.
    Opaque,
}

impl CaptureOutputKind {
    /// Returns whether this kind represents a directly produced value.
    #[must_use]
    pub const fn produces_value(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns whether this kind is raw acquisition data.
    #[must_use]
    pub const fn is_raw(self) -> bool {
        matches!(self, Self::RawWaveform)
    }

    /// Returns whether this kind is a classified bit.
    #[must_use]
    pub const fn is_bit(self) -> bool {
        matches!(self, Self::Bit)
    }

    /// Returns whether this kind is complex-valued.
    #[must_use]
    pub const fn is_complex(self) -> bool {
        matches!(self, Self::Complex)
    }
}

// ============================================================================
// Capture shape
// ============================================================================

/// Shape of a capture result.
///
/// This is separate from [`CaptureOutputKind`] because the same semantic type
/// can be scalar or a sequence depending on the acquisition request.
///
/// For example:
///
///     Complex + Scalar
///
/// may represent one integrated IQ result, while:
///
///     Complex + Sequence
///
/// may represent a sequence of IQ samples.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CaptureShape {
    /// One semantic result.
    Scalar,

    /// A sequence of semantic results.
    ///
    /// The sequence length is determined by the acquisition configuration or
    /// downstream target.
    Sequence,

    /// A multidimensional/structured result whose exact dimensions are
    /// described by the target or an extension.
    Structured,
}

impl CaptureShape {
    /// Returns whether this is scalar.
    #[must_use]
    pub const fn is_scalar(self) -> bool {
        matches!(self, Self::Scalar)
    }

    /// Returns whether this is a sequence.
    #[must_use]
    pub const fn is_sequence(self) -> bool {
        matches!(self, Self::Sequence)
    }
}

// ============================================================================
// Capture filter
// ============================================================================

/// Optional semantic filter/kernel applied to acquired data.
///
/// A waveform is only referenced here. The waveform definition itself remains
/// owned by `waveform.rs`.
///
/// This corresponds to the OpenPulse notion of a capture filter/kernel used to
/// distill acquired IQ data into a result.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct CaptureFilter {
    waveform: WaveformId,
}

impl CaptureFilter {
    /// Creates a waveform-backed capture filter.
    #[must_use]
    pub const fn new(waveform: WaveformId) -> Self {
        Self { waveform }
    }

    /// Returns the referenced waveform.
    #[must_use]
    pub const fn waveform(self) -> WaveformId {
        self.waveform
    }
}

// ============================================================================
// Capture resources
// ============================================================================

/// Abstract semantic resources associated with acquisition.
///
/// None of these references imply physical allocation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct CaptureResources {
    frame: FrameId,
    channel: Option<ChannelId>,
}

impl CaptureResources {
    /// Creates a capture resource set using only the mandatory frame.
    #[must_use]
    pub const fn new(frame: FrameId) -> Self {
        Self {
            frame,
            channel: None,
        }
    }

    /// Creates a capture resource set with an abstract acquisition channel.
    #[must_use]
    pub const fn with_channel(
        frame: FrameId,
        channel: ChannelId,
    ) -> Self {
        Self {
            frame,
            channel: Some(channel),
        }
    }

    /// Returns the frame reference.
    #[must_use]
    pub const fn frame(self) -> FrameId {
        self.frame
    }

    /// Returns the optional channel reference.
    #[must_use]
    pub const fn channel(self) -> Option<ChannelId> {
        self.channel
    }
}

// ============================================================================
// Capture processing
// ============================================================================

/// Semantic post-acquisition processing intent.
///
/// These are intentionally high-level requests. The actual DSP/discrimination
/// implementation belongs to the backend or a separate classical-processing
/// layer.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum CaptureProcessing {
    /// Return acquired data without semantic processing.
    Raw,

    /// Apply the referenced capture filter/kernel.
    Filter,

    /// Request integration of acquired data.
    Integrate,

    /// Request classification/discrimination.
    Discriminate,

    /// Request counting/detection semantics.
    Count,

    /// Processing is defined by an extension or target dialect.
    Extension,
}

impl CaptureProcessing {
    /// Returns whether this processing mode expects a filter/kernel.
    #[must_use]
    pub const fn requires_filter(self) -> bool {
        matches!(self, Self::Filter | Self::Integrate | Self::Discriminate)
    }
}

// ============================================================================
// Metadata
// ============================================================================

/// Capture metadata.
///
/// Metadata is descriptive and deterministic. It is not an authentication or
/// authorization mechanism.
///
/// Keys and values are bounded by [`CaptureValidationPolicy`] when validated.
pub type CaptureMetadata = BTreeMap<String, String>;

// ============================================================================
// Validation policy
// ============================================================================

/// Explicit capture validation/resource policy.
///
/// These limits are NOT architectural limits of Zamani Quantum IR.
///
/// A caller handling trusted, already validated IR may choose a permissive
/// policy. A compiler service handling untrusted input should provide explicit
/// finite bounds before materializing large structures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureValidationPolicy {
    /// Maximum metadata key length in UTF-8 bytes.
    pub max_metadata_key_bytes: usize,

    /// Maximum metadata value length in UTF-8 bytes.
    pub max_metadata_value_bytes: usize,

    /// Maximum metadata entry count.
    pub max_metadata_fields: usize,

    /// Maximum explicitly materialized logical-qubit targets.
    pub max_explicit_targets: usize,
}

impl CaptureValidationPolicy {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        max_metadata_key_bytes: usize,
        max_metadata_value_bytes: usize,
        max_metadata_fields: usize,
        max_explicit_targets: usize,
    ) -> Self {
        Self {
            max_metadata_key_bytes,
            max_metadata_value_bytes,
            max_metadata_fields,
            max_explicit_targets,
        }
    }

    /// Creates the default defensive policy.
    #[must_use]
    pub const fn default_policy() -> Self {
        Self {
            max_metadata_key_bytes: 256,
            max_metadata_value_bytes: 4096,
            max_metadata_fields: 4096,
            max_explicit_targets: 4096,
        }
    }

    /// Creates an intentionally permissive policy.
    ///
    /// This does not create an infinite-memory guarantee. Rust containers
    /// remain bounded by the process address space and available resources.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_metadata_key_bytes: usize::MAX,
            max_metadata_value_bytes: usize::MAX,
            max_metadata_fields: usize::MAX,
            max_explicit_targets: usize::MAX,
        }
    }

    /// Validates the policy itself.
    pub const fn validate(self) -> CaptureResult<()> {
        if self.max_metadata_key_bytes == 0 {
            return Err(CaptureError::InvalidPolicy(
                CapturePolicyError::ZeroMetadataKeyLimit,
            ));
        }

        if self.max_metadata_value_bytes == 0 {
            return Err(CaptureError::InvalidPolicy(
                CapturePolicyError::ZeroMetadataValueLimit,
            ));
        }

        Ok(())
    }
}

impl Default for CaptureValidationPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}

// ============================================================================
// Capture specification
// ============================================================================

/// Complete hardware-independent capture specification.
///
/// This is the principal type exported by this module.
///
/// It describes acquisition intent without deciding physical implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureSpec {
    target: CaptureTarget,
    resources: CaptureResources,
    duration: CaptureDuration,
    filter: Option<CaptureFilter>,
    output: CaptureOutputKind,
    shape: CaptureShape,
    processing: CaptureProcessing,
    metadata: CaptureMetadata,
}

impl CaptureSpec {
    /// Creates a minimal capture specification.
    ///
    /// The frame is mandatory because the frame provides the semantic temporal
    /// context for capture.
    ///
    /// The output defaults to `None`, matching the possibility of acquisition
    /// into an implementation-defined buffer.
    #[must_use]
    pub fn new(
        target: CaptureTarget,
        frame: FrameId,
    ) -> Self {
        Self {
            target,
            resources: CaptureResources::new(frame),
            duration: CaptureDuration::Inferred,
            filter: None,
            output: CaptureOutputKind::None,
            shape: CaptureShape::Scalar,
            processing: CaptureProcessing::Raw,
            metadata: BTreeMap::new(),
        }
    }

    /// Sets an explicit acquisition channel.
    #[must_use]
    pub const fn with_channel(
        mut self,
        channel: ChannelId,
    ) -> Self {
        self.resources = CaptureResources::with_channel(
            self.resources.frame(),
            channel,
        );

        self
    }

    /// Sets the capture duration.
    #[must_use]
    pub const fn with_duration(
        mut self,
        duration: PulseDuration,
    ) -> Self {
        self.duration = CaptureDuration::Explicit(duration);
        self
    }

    /// Marks the duration as target/definition inferred.
    #[must_use]
    pub const fn with_inferred_duration(
        mut self,
    ) -> Self {
        self.duration = CaptureDuration::Inferred;
        self
    }

    /// Sets a waveform filter/kernel.
    #[must_use]
    pub const fn with_filter(
        mut self,
        filter: CaptureFilter,
    ) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Sets a waveform filter/kernel by identifier.
    #[must_use]
    pub const fn with_filter_waveform(
        mut self,
        waveform: WaveformId,
    ) -> Self {
        self.filter = Some(CaptureFilter::new(waveform));
        self
    }

    /// Sets the semantic output type.
    #[must_use]
    pub const fn with_output(
        mut self,
        output: CaptureOutputKind,
    ) -> Self {
        self.output = output;
        self
    }

    /// Sets the semantic result shape.
    #[must_use]
    pub const fn with_shape(
        mut self,
        shape: CaptureShape,
    ) -> Self {
        self.shape = shape;
        self
    }

    /// Sets semantic processing intent.
    #[must_use]
    pub const fn with_processing(
        mut self,
        processing: CaptureProcessing,
    ) -> Self {
        self.processing = processing;
        self
    }

    /// Adds or replaces metadata.
    ///
    /// Metadata remains subject to the validation policy.
    #[must_use]
    pub fn with_metadata<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns the target.
    #[must_use]
    pub const fn target(&self) -> &CaptureTarget {
        &self.target
    }

    /// Returns the frame reference.
    #[must_use]
    pub const fn frame(&self) -> FrameId {
        self.resources.frame()
    }

    /// Returns the optional channel reference.
    #[must_use]
    pub const fn channel(&self) -> Option<ChannelId> {
        self.resources.channel()
    }

    /// Returns the duration specification.
    #[must_use]
    pub const fn duration(&self) -> CaptureDuration {
        self.duration
    }

    /// Returns the optional filter.
    #[must_use]
    pub const fn filter(&self) -> Option<CaptureFilter> {
        self.filter
    }

    /// Returns the output kind.
    #[must_use]
    pub const fn output(&self) -> CaptureOutputKind {
        self.output
    }

    /// Returns the result shape.
    #[must_use]
    pub const fn shape(&self) -> CaptureShape {
        self.shape
    }

    /// Returns the processing intent.
    #[must_use]
    pub const fn processing(&self) -> CaptureProcessing {
        self.processing
    }

    /// Returns immutable metadata.
    #[must_use]
    pub fn metadata(&self) -> &CaptureMetadata {
        &self.metadata
    }

    /// Returns the number of explicitly materialized logical-qubit targets.
    #[must_use]
    pub fn explicit_target_count(&self) -> usize {
        self.target.explicit_qubit_count()
    }

    /// Validates using the default defensive policy.
    pub fn validate(&self) -> CaptureResult<()> {
        self.validate_with_policy(
            CaptureValidationPolicy::default(),
        )
    }

    /// Validates using an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: CaptureValidationPolicy,
    ) -> CaptureResult<()> {
        policy.validate()?;

        self.target.validate()?;

        if self.explicit_target_count()
            > policy.max_explicit_targets
        {
            return Err(CaptureError::TargetLimitExceeded {
                actual: self.explicit_target_count(),
                maximum: policy.max_explicit_targets,
            });
        }

        if let CaptureDuration::Explicit(duration) = self.duration {
            // Zero-duration capture is structurally meaningful only for an
            // implementation that explicitly supports it. The canonical
            // capture IR rejects it because it otherwise creates an ambiguous
            // acquisition request.
            if duration.is_zero() {
                return Err(CaptureError::ZeroDuration);
            }
        }

        if self.processing.requires_filter()
            && self.filter.is_none()
        {
            return Err(CaptureError::MissingFilter);
        }

        if self.output == CaptureOutputKind::None
            && self.shape != CaptureShape::Scalar
        {
            return Err(CaptureError::InvalidOutputShape);
        }

        if self.processing == CaptureProcessing::Raw
            && self.output != CaptureOutputKind::RawWaveform
            && self.output != CaptureOutputKind::None
        {
            return Err(CaptureError::ProcessingOutputMismatch);
        }

        if self.processing == CaptureProcessing::Discriminate
            && self.output != CaptureOutputKind::Bit
        {
            return Err(CaptureError::ProcessingOutputMismatch);
        }

        if self.processing == CaptureProcessing::Count
            && self.output != CaptureOutputKind::Count
        {
            return Err(CaptureError::ProcessingOutputMismatch);
        }

        if self.processing == CaptureProcessing::Filter
            && self.filter.is_none()
        {
            return Err(CaptureError::MissingFilter);
        }

        for (key, value) in &self.metadata {
            if key.as_bytes().len()
                > policy.max_metadata_key_bytes
            {
                return Err(CaptureError::MetadataKeyTooLarge {
                    actual: key.as_bytes().len(),
                    maximum: policy.max_metadata_key_bytes,
                });
            }

            if value.as_bytes().len()
                > policy.max_metadata_value_bytes
            {
                return Err(CaptureError::MetadataValueTooLarge {
                    actual: value.as_bytes().len(),
                    maximum: policy.max_metadata_value_bytes,
                });
            }
        }

        if self.metadata.len()
            > policy.max_metadata_fields
        {
            return Err(CaptureError::MetadataLimitExceeded {
                actual: self.metadata.len(),
                maximum: policy.max_metadata_fields,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Capture operation wrapper
// ============================================================================

/// Canonical capture operation.
///
/// `CaptureOperation` provides a small semantic wrapper around
/// [`CaptureSpec`] so downstream universal operation infrastructure can carry
/// a complete capture request without redefining its semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureOperation {
    specification: CaptureSpec,
}

impl CaptureOperation {
    /// Creates a capture operation.
    ///
    /// The specification is validated before being accepted.
    pub fn new(
        specification: CaptureSpec,
    ) -> CaptureResult<Self> {
        specification.validate()?;

        Ok(Self { specification })
    }

    /// Creates a capture operation under an explicit policy.
    pub fn new_with_policy(
        specification: CaptureSpec,
        policy: CaptureValidationPolicy,
    ) -> CaptureResult<Self> {
        specification.validate_with_policy(policy)?;

        Ok(Self { specification })
    }

    /// Returns the capture specification.
    #[must_use]
    pub const fn specification(&self) -> &CaptureSpec {
        &self.specification
    }

    /// Consumes the operation and returns its specification.
    #[must_use]
    pub fn into_specification(self) -> CaptureSpec {
        self.specification
    }

    /// Validates the operation.
    pub fn validate(&self) -> CaptureResult<()> {
        self.specification.validate()
    }

    /// Validates under an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: CaptureValidationPolicy,
    ) -> CaptureResult<()> {
        self.specification.validate_with_policy(policy)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Capture semantic errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureError {
    /// No logical targets were supplied to an explicit target set.
    EmptyTargetSet,

    /// An explicit target set is not sorted and unique.
    NonCanonicalTargetSet,

    /// An explicit target count exceeds the caller's policy.
    TargetLimitExceeded {
        /// Number of explicitly supplied targets.
        actual: usize,

        /// Maximum permitted by policy.
        maximum: usize,
    },

    /// Explicit duration is zero.
    ZeroDuration,

    /// Processing requires a waveform filter but none was supplied.
    MissingFilter,

    /// Output kind and result shape are semantically incompatible.
    InvalidOutputShape,

    /// Processing and output kinds are inconsistent.
    ProcessingOutputMismatch,

    /// Metadata key is larger than the policy permits.
    MetadataKeyTooLarge {
        /// Actual key size in UTF-8 bytes.
        actual: usize,

        /// Maximum permitted key size.
        maximum: usize,
    },

    /// Metadata value is larger than the policy permits.
    MetadataValueTooLarge {
        /// Actual value size in UTF-8 bytes.
        actual: usize,

        /// Maximum permitted value size.
        maximum: usize,
    },

    /// Metadata field count exceeds policy.
    MetadataLimitExceeded {
        /// Actual field count.
        actual: usize,

        /// Maximum permitted field count.
        maximum: usize,
    },

    /// Validation policy is invalid.
    InvalidPolicy(CapturePolicyError),
}

/// Errors produced by invalid capture policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapturePolicyError {
    /// Metadata key limit cannot be zero.
    ZeroMetadataKeyLimit,

    /// Metadata value limit cannot be zero.
    ZeroMetadataValueLimit,
}

impl fmt::Display for CapturePolicyError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroMetadataKeyLimit => {
                formatter.write_str(
                    "capture metadata key limit cannot be zero",
                )
            }

            Self::ZeroMetadataValueLimit => {
                formatter.write_str(
                    "capture metadata value limit cannot be zero",
                )
            }
        }
    }
}

impl fmt::Display for CaptureError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyTargetSet => {
                formatter.write_str(
                    "capture target set cannot be empty",
                )
            }

            Self::NonCanonicalTargetSet => {
                formatter.write_str(
                    "capture target set must be sorted and unique",
                )
            }

            Self::TargetLimitExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "capture target count {actual} exceeds policy maximum {maximum}"
                )
            }

            Self::ZeroDuration => {
                formatter.write_str(
                    "explicit capture duration must be greater than zero",
                )
            }

            Self::MissingFilter => {
                formatter.write_str(
                    "capture processing mode requires a waveform filter",
                )
            }

            Self::InvalidOutputShape => {
                formatter.write_str(
                    "capture output kind and result shape are incompatible",
                )
            }

            Self::ProcessingOutputMismatch => {
                formatter.write_str(
                    "capture processing mode and output kind are incompatible",
                )
            }

            Self::MetadataKeyTooLarge {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "capture metadata key length {actual} exceeds policy maximum {maximum}"
                )
            }

            Self::MetadataValueTooLarge {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "capture metadata value length {actual} exceeds policy maximum {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "capture metadata field count {actual} exceeds policy maximum {maximum}"
                )
            }

            Self::InvalidPolicy(error) => {
                write!(
                    formatter,
                    "invalid capture validation policy: {error}"
                )
            }
        }
    }
}

impl std::error::Error for CaptureError {}

// ============================================================================
// Schema information
// ============================================================================

/// Returns the semantic capture schema identifier.
#[must_use]
pub const fn schema_id() -> &'static str {
    CAPTURE_SCHEMA_ID
}

/// Returns the semantic capture schema version tuple.
#[must_use]
pub const fn schema_version() -> (u16, u16, u16) {
    (
        CAPTURE_SCHEMA_MAJOR,
        CAPTURE_SCHEMA_MINOR,
        CAPTURE_SCHEMA_PATCH,
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::core::identity::{
        ChannelId,
        FrameId,
        ResourceId,
        WaveformId,
    };

    use super::super::super::quantum::qubit::QubitId;

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            schema_id(),
            "zamani.quantum.ir.pulse.capture"
        );

        assert_eq!(
            schema_version(),
            (1, 0, 0)
        );
    }

    #[test]
    fn single_qubit_target_uses_canonical_qubit_id() {
        let target =
            CaptureTarget::qubit(QubitId::new(7));

        assert!(target.is_single_qubit());
        assert_eq!(
            target.explicit_qubit_count(),
            1
        );

        assert_eq!(
            target.logical_qubits(),
            &[QubitId::new(7)]
        );

        assert!(target.validate().is_ok());
    }

    #[test]
    fn multi_qubit_target_is_canonicalized() {
        let target = CaptureTarget::qubits([
            QubitId::new(5),
            QubitId::new(2),
            QubitId::new(5),
            QubitId::new(1),
        ]);

        assert_eq!(
            target.logical_qubits(),
            &[
                QubitId::new(1),
                QubitId::new(2),
                QubitId::new(5),
            ]
        );

        assert_eq!(
            target.explicit_qubit_count(),
            3
        );

        assert!(target.validate().is_ok());
    }

    #[test]
    fn empty_multi_qubit_target_is_rejected() {
        let target =
            CaptureTarget::Qubits(Vec::new());

        assert_eq!(
            target.validate(),
            Err(CaptureError::EmptyTargetSet)
        );
    }

    #[test]
    fn non_canonical_target_is_rejected() {
        let target =
            CaptureTarget::Qubits(vec![
                QubitId::new(3),
                QubitId::new(1),
            ]);

        assert_eq!(
            target.validate(),
            Err(CaptureError::NonCanonicalTargetSet)
        );
    }

    #[test]
    fn resource_and_global_targets_are_supported() {
        let resource =
            CaptureTarget::resource(
                ResourceId::new(42),
            );

        assert!(resource.is_resource());
        assert_eq!(
            resource.explicit_qubit_count(),
            0
        );

        let global =
            CaptureTarget::global();

        assert!(global.is_global());
        assert_eq!(
            global.logical_qubits(),
            &[]
        );
    }

    #[test]
    fn duration_can_be_inferred() {
        let duration =
            CaptureDuration::inferred();

        assert!(!duration.is_explicit());
        assert_eq!(
            duration.as_explicit(),
            None
        );
    }

    #[test]
    fn explicit_duration_is_preserved() {
        let duration =
            PulseDuration::from_nanoseconds(20)
                .expect("20ns must be representable");

        let capture_duration =
            CaptureDuration::explicit(duration);

        assert!(capture_duration.is_explicit());
        assert_eq!(
            capture_duration.as_explicit(),
            Some(duration)
        );
    }

    #[test]
    fn minimal_capture_is_valid() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            );

        assert!(capture.validate().is_ok());
    }

    #[test]
    fn explicit_duration_capture_is_valid() {
        let duration =
            PulseDuration::from_nanoseconds(20)
                .expect("20ns must be representable");

        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_duration(duration)
            .with_output(
                CaptureOutputKind::RawWaveform,
            );

        assert!(capture.validate().is_ok());
    }

    #[test]
    fn zero_duration_is_rejected() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_duration(
                PulseDuration::ZERO,
            );

        assert_eq!(
            capture.validate(),
            Err(CaptureError::ZeroDuration)
        );
    }

    #[test]
    fn filter_processing_requires_filter() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_processing(
                CaptureProcessing::Filter,
            )
            .with_output(
                CaptureOutputKind::Complex,
            );

        assert_eq!(
            capture.validate(),
            Err(CaptureError::MissingFilter)
        );
    }

    #[test]
    fn filter_processing_accepts_waveform() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_filter_waveform(
                WaveformId::new(3),
            )
            .with_processing(
                CaptureProcessing::Filter,
            )
            .with_output(
                CaptureOutputKind::Complex,
            );

        assert!(capture.validate().is_ok());
    }

    #[test]
    fn discriminate_requires_bit_output() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_filter_waveform(
                WaveformId::new(3),
            )
            .with_processing(
                CaptureProcessing::Discriminate,
            )
            .with_output(
                CaptureOutputKind::Bit,
            );

        assert!(capture.validate().is_ok());
    }

    #[test]
    fn discriminate_with_complex_output_is_rejected() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_filter_waveform(
                WaveformId::new(3),
            )
            .with_processing(
                CaptureProcessing::Discriminate,
            )
            .with_output(
                CaptureOutputKind::Complex,
            );

        assert_eq!(
            capture.validate(),
            Err(CaptureError::ProcessingOutputMismatch)
        );
    }

    #[test]
    fn count_processing_requires_count_output() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_processing(
                CaptureProcessing::Count,
            )
            .with_output(
                CaptureOutputKind::Count,
            );

        assert!(capture.validate().is_ok());
    }

    #[test]
    fn raw_processing_accepts_raw_output() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_processing(
                CaptureProcessing::Raw,
            )
            .with_output(
                CaptureOutputKind::RawWaveform,
            );

        assert!(capture.validate().is_ok());
    }

    #[test]
    fn no_result_capture_is_valid() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_output(
                CaptureOutputKind::None,
            )
            .with_shape(
                CaptureShape::Scalar,
            );

        assert!(capture.validate().is_ok());
    }

    #[test]
    fn non_scalar_none_output_is_rejected() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_output(
                CaptureOutputKind::None,
            )
            .with_shape(
                CaptureShape::Sequence,
            );

        assert_eq!(
            capture.validate(),
            Err(CaptureError::InvalidOutputShape)
        );
    }

    #[test]
    fn channel_reference_is_preserved() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(2),
            )
            .with_channel(
                ChannelId::new(9),
            );

        assert_eq!(
            capture.channel(),
            Some(ChannelId::new(9))
        );

        assert_eq!(
            capture.frame(),
            FrameId::new(2)
        );
    }

    #[test]
    fn metadata_is_deterministic() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_metadata(
                "experiment",
                "readout",
            )
            .with_metadata(
                "purpose",
                "calibration",
            );

        let metadata = capture.metadata();

        let keys: Vec<&String> =
            metadata.keys().collect();

        assert_eq!(
            keys,
            vec![
                &"experiment".to_string(),
                &"purpose".to_string(),
            ]
        );
    }

    #[test]
    fn metadata_policy_is_enforced() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_metadata(
                "key",
                "value",
            );

        let policy =
            CaptureValidationPolicy::new(
                2,
                4096,
                4096,
                4096,
            );

        assert!(matches!(
            capture.validate_with_policy(policy),
            Err(CaptureError::MetadataKeyTooLarge {
                ..
            })
        ));
    }

    #[test]
    fn target_policy_is_enforced() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubits([
                    QubitId::new(0),
                    QubitId::new(1),
                    QubitId::new(2),
                ]),
                FrameId::new(0),
            );

        let policy =
            CaptureValidationPolicy::new(
                256,
                4096,
                4096,
                2,
            );

        assert!(matches!(
            capture.validate_with_policy(policy),
            Err(CaptureError::TargetLimitExceeded {
                actual: 3,
                maximum: 2,
            })
        ));
    }

    #[test]
    fn operation_validates_before_construction() {
        let invalid =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_processing(
                CaptureProcessing::Discriminate,
            )
            .with_output(
                CaptureOutputKind::Complex,
            );

        assert!(CaptureOperation::new(invalid).is_err());
    }

    #[test]
    fn operation_accepts_valid_capture() {
        let valid =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            )
            .with_processing(
                CaptureProcessing::Raw,
            )
            .with_output(
                CaptureOutputKind::RawWaveform,
            );

        let operation =
            CaptureOperation::new(valid)
                .expect("capture should be valid");

        assert_eq!(
            operation.specification().frame(),
            FrameId::new(0)
        );
    }

    #[test]
    fn unlimited_policy_does_not_change_semantics() {
        let capture =
            CaptureSpec::new(
                CaptureTarget::qubit(
                    QubitId::new(0),
                ),
                FrameId::new(0),
            );

        assert!(
            capture
                .validate_with_policy(
                    CaptureValidationPolicy::unlimited()
                )
                .is_ok()
        );
    }

    #[test]
    fn output_kind_properties_are_consistent() {
        assert!(
            !CaptureOutputKind::None
                .produces_value()
        );

        assert!(
            CaptureOutputKind::RawWaveform
                .produces_value()
        );

        assert!(
            CaptureOutputKind::RawWaveform
                .is_raw()
        );

        assert!(
            CaptureOutputKind::Complex
                .is_complex()
        );

        assert!(
            CaptureOutputKind::Bit
                .is_bit()
        );
    }
}