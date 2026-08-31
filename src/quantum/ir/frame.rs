//! Zamani Quantum IR — Hardware-Independent Frame Semantics
//!
//! Canonical semantic representation of quantum control frames.
//!
//! # Architectural role
//!
//! A frame represents the logical reference used to interpret phase and
//! frequency for quantum-control operations. Frames are particularly
//! important for pulse-level quantum programming.
//!
//! A frame can describe:
//!
//! - a carrier/reference frequency;
//! - a phase reference;
//! - a phase accumulation convention;
//! - frequency shifts;
//! - phase shifts;
//! - frame resets;
//! - symbolic/runtime-controlled frequency and phase values;
//! - logical or physical qubit association;
//! - deterministic frame identity.
//!
//! The frame IR is intentionally hardware-independent.
//!
//! This module does NOT own:
//!
//! - physical DACs;
//! - ADCs;
//! - microwave generators;
//! - laser hardware;
//! - physical channel allocation;
//! - hardware calibration;
//! - hardware topology;
//! - pulse scheduling;
//! - pulse compilation;
//! - waveform generation;
//! - device-specific frame implementations;
//! - QPU communication;
//! - backend execution.
//!
//! Those responsibilities belong to downstream hardware, pulse, scheduling,
//! and backend subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once and can be compiled for a machine
//! ranging from a single qubit to an arbitrarily large finite quantum system,
//! subject only to available resources and explicit compilation/security
//! policies.
//!
//! This module therefore contains no architectural maximum for:
//!
//! - number of frames;
//! - number of qubits;
//! - number of physical qubits;
//! - number of frame changes;
//! - machine size.
//!
//! `usize` is used for collection/index identities because it is the native
//! Rust indexing type. It is not a quantum-computer-size limit.
//!
//! # Relationship with qubits
//!
//! The canonical qubit namespace is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A frame may be associated with either logical or physical qubit identity.
//!
//! This association does NOT perform logical-to-physical mapping.
//!
//! Routing remains responsible for deciding where a logical qubit is placed.
//! Hardware remains responsible for determining whether a physical qubit and
//! its control resources actually exist.
//!
//! # Relationship with parameters
//!
//! Frame frequency and phase can be represented by the canonical IR
//! [`crate::quantum::ir::parameter::Parameter`] type.
//!
//! This permits:
//!
//! ```text
//! frequency = 5.0e9
//! frequency = symbolic_frequency
//! phase      = pi / 2
//! phase      = runtime_phase
//! ```
//!
//! The frame module gives those parameters explicit semantic units:
//!
//! - frequency values are expressed in hertz;
//! - phase values are expressed in radians.
//!
//! # Important unit rule
//!
//! Rust's `f64` does not carry physical units. Therefore this module never
//! exposes a bare `f64` as a public frequency or phase field.
//!
//! Instead:
//!
//! ```text
//! FrameFrequency
//! FramePhase
//! ```
//!
//! provide the semantic boundary.
//!
//! # Frame semantics
//!
//! A frame has two conceptually distinct components:
//!
//! ```text
//! Frame definition
//!     ├── reference frequency
//!     └── reference phase
//!
//! Frame changes
//!     ├── frequency shift
//!     ├── phase shift
//!     └── reset
//! ```
//!
//! A frame change modifies the control reference. It does not itself represent
//! a pulse waveform or a scheduled hardware event.
//!
//! # Virtual-Z / phase semantics
//!
//! A phase change can be represented without emitting a physical waveform.
//! Downstream hardware-aware compilation may lower this into a virtual-Z
//! operation, frame update, phase accumulator update, or another target-native
//! instruction.
//!
//! The IR must preserve the semantic phase change without prematurely choosing
//! a hardware implementation.
//!
//! # Determinism
//!
//! All identifiers and collections provided by this module have deterministic
//! equality, ordering, and hashing semantics.
//!
//! No global frame registry is used.
//!
//! # Safety
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! It requires no nightly features.
//! It requires no `unsafe` code.
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `pulse.rs` may reference [`FrameId`], [`FrameRef`], and [`FrameChange`].
//!
//! `channel.rs` may associate hardware-independent channel references with
//! frame definitions without changing the frame semantics.
//!
//! `schedule.rs` may schedule frame-change operations without changing this
//! module.
//!
//! `hardware/` may map an IR frame to actual device frequencies, phase
//! accumulators, channels, clocks, or control electronics.
//!
//! `routing/` may determine the physical qubit corresponding to a logical
//! qubit. This module never performs that mapping.
//!
//! `optimization/` may transform frame changes while preserving their
//! semantic effect.
//!
//! `serialization.rs` may serialize the public structures directly.
//!
//! `hash.rs` may use the deterministic structural representation.
//!
//! `validation.rs` may invoke the validation methods provided here.
//!
//! `mod.rs` should later declare:
//!
//! ```text
//! pub mod frame;
//! ```
//!
//! and may re-export the stable frame API. This file itself does not depend on
//! `mod.rs` re-exports and therefore does not require a later rewrite merely
//! because the public prelude changes.

#![forbid(unsafe_code)]

use std::fmt;
use std::str::FromStr;

use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

// =============================================================================
// Policy constants
// =============================================================================

/// Default maximum UTF-8 byte length for a frame name.
///
/// This is an input/resource policy, not a quantum-machine limitation.
pub const DEFAULT_MAX_FRAME_NAME_BYTES: usize = 256;

/// Default maximum UTF-8 byte length for a frame namespace.
///
/// This is an input/resource policy, not a quantum-machine limitation.
pub const DEFAULT_MAX_FRAME_NAMESPACE_BYTES: usize = 256;

/// Default maximum number of frame changes validated in one local collection.
///
/// This is deliberately a policy value rather than an architectural maximum.
///
/// Larger programs must use an explicit caller-supplied policy.
pub const DEFAULT_MAX_FRAME_CHANGES: usize = 1_048_576;

// =============================================================================
// Frame identifier
// =============================================================================

/// Stable canonical identifier for a frame.
///
/// The identifier has no hardware meaning by itself.
///
/// `FrameId(0)` does not mean physical channel 0, qubit 0, DAC 0, or any
/// other hardware resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FrameId(usize);

impl FrameId {
    /// Creates a frame identifier from a namespace index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the underlying namespace index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }

    /// Returns the next identifier if integer overflow does not occur.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for FrameId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<FrameId> for usize {
    fn from(value: FrameId) -> Self {
        value.index()
    }
}

impl fmt::Display for FrameId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "frame{}", self.0)
    }
}

// =============================================================================
// Frame namespace
// =============================================================================

/// Optional namespace used to make frame names deterministic and portable.
///
/// A namespace is semantic metadata. It does not identify a hardware device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FrameNamespace(String);

impl FrameNamespace {
    /// Creates a validated namespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, FrameError> {
        let value = value.into();

        validate_text(
            &value,
            DEFAULT_MAX_FRAME_NAMESPACE_BYTES,
            "frame namespace",
        )?;

        if value.is_empty() {
            return Err(FrameError::EmptyNamespace);
        }

        Ok(Self(value))
    }

    /// Returns the namespace string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for FrameNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FrameNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Frame name
// =============================================================================

/// Human-readable deterministic frame name.
///
/// Names are metadata and must never be interpreted as physical device
/// identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FrameName(String);

impl FrameName {
    /// Creates a validated frame name.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, FrameError> {
        let value = value.into();

        validate_text(&value, DEFAULT_MAX_FRAME_NAME_BYTES, "frame name")?;

        if value.is_empty() {
            return Err(FrameError::EmptyName);
        }

        Ok(Self(value))
    }

    /// Returns the frame name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for FrameName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FrameName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for FrameName {
    type Err = FrameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// =============================================================================
// Frame target
// =============================================================================

/// Semantic target associated with a frame.
///
/// A target may be:
///
/// - a logical qubit;
/// - a physical qubit;
/// - a named abstract target.
///
/// The named form exists for technologies where the frame belongs to a
/// control resource that is not naturally represented by a single qubit.
///
/// The named form remains hardware-independent. It is resolved by downstream
/// channel/hardware compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FrameTarget {
    /// Logical-qubit frame.
    Logical(QubitId),

    /// Physical-qubit frame.
    ///
    /// This is a reference vocabulary only. It does not prove that the
    /// physical qubit exists.
    Physical(PhysicalQubitId),

    /// Named abstract frame target.
    Named(FrameName),
}

impl FrameTarget {
    /// Creates a logical frame target.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }

    /// Creates a physical frame target.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::Physical(qubit)
    }

    /// Creates a named frame target.
    pub fn named<S: Into<String>>(name: S) -> Result<Self, FrameError> {
        Ok(Self::Named(FrameName::new(name)?))
    }

    /// Converts a canonical qubit reference into a frame target.
    #[must_use]
    pub const fn from_qubit_ref(reference: QubitRef) -> Self {
        match reference {
            QubitRef::Logical(qubit) => Self::Logical(qubit),
            QubitRef::Physical(qubit) => Self::Physical(qubit),
        }
    }

    /// Returns the logical qubit when this target is logical.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        match self {
            Self::Logical(qubit) => Some(*qubit),
            Self::Physical(_) | Self::Named(_) => None,
        }
    }

    /// Returns the physical qubit when this target is physical.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::Physical(qubit) => Some(*qubit),
            Self::Logical(_) | Self::Named(_) => None,
        }
    }

    /// Returns the named target when applicable.
    #[must_use]
    pub fn named_target(&self) -> Option<&str> {
        match self {
            Self::Named(name) => Some(name.as_str()),
            Self::Logical(_) | Self::Physical(_) => None,
        }
    }

    /// Returns true when this is a logical target.
    #[must_use]
    pub const fn is_logical(&self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns true when this is a physical target.
    #[must_use]
    pub const fn is_physical(&self) -> bool {
        matches!(self, Self::Physical(_))
    }

    /// Returns true when this is a named target.
    #[must_use]
    pub const fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }
}

impl From<QubitId> for FrameTarget {
    fn from(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }
}

impl From<PhysicalQubitId> for FrameTarget {
    fn from(qubit: PhysicalQubitId) -> Self {
        Self::Physical(qubit)
    }
}

impl From<QubitRef> for FrameTarget {
    fn from(reference: QubitRef) -> Self {
        Self::from_qubit_ref(reference)
    }
}

impl fmt::Display for FrameTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical(qubit) => write!(formatter, "{qubit}"),
            Self::Physical(qubit) => write!(formatter, "{qubit}"),
            Self::Named(name) => formatter.write_str(name.as_str()),
        }
    }
}

// =============================================================================
// Frame frequency
// =============================================================================

/// Semantic frequency value for a frame.
///
/// The unit is hertz.
///
/// The value may be:
///
/// - a finite concrete value;
/// - a symbolic parameter;
/// - a deterministic arithmetic expression.
///
/// Negative frequencies are permitted because a frame frequency is a
/// mathematical signed reference frequency. Hardware compatibility validation
/// may impose target-specific restrictions later.
///
/// Hardware-specific oscillator ranges do NOT belong here.
#[derive(Debug, Clone, PartialEq)]
pub struct FrameFrequency(Parameter);

impl FrameFrequency {
    /// Creates a concrete frequency in hertz.
    ///
    /// The value must be finite.
    pub fn hz(value: f64) -> Result<Self, FrameError> {
        if !value.is_finite() {
            return Err(FrameError::NonFiniteFrequency);
        }

        let parameter =
            Parameter::constant(value).map_err(FrameError::Parameter)?;

        Ok(Self(parameter))
    }

    /// Creates a frequency from a canonical parameter.
    pub fn parameter(parameter: Parameter) -> Result<Self, FrameError> {
        parameter
            .validate()
            .map_err(FrameError::Parameter)?;

        Ok(Self(parameter))
    }

    /// Returns the underlying canonical parameter.
    #[must_use]
    pub fn parameter_ref(&self) -> &Parameter {
        &self.0
    }

    /// Consumes the value and returns its canonical parameter.
    #[must_use]
    pub fn into_parameter(self) -> Parameter {
        self.0
    }

    /// Returns a concrete frequency when this value is non-symbolic.
    #[must_use]
    pub fn as_hz(&self) -> Option<f64> {
        self.0.as_constant()
    }

    /// Returns true when the frequency contains a symbolic value.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.0.is_symbolic()
    }

    /// Validates the frequency.
    pub fn validate(&self) -> Result<(), FrameError> {
        self.0.validate().map_err(FrameError::Parameter)
    }
}

impl fmt::Display for FrameFrequency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_hz() {
            Some(value) => write!(formatter, "{value} Hz"),
            None => write!(formatter, "symbolic-frequency"),
        }
    }
}

// =============================================================================
// Frame phase
// =============================================================================

/// Semantic phase value for a frame.
///
/// The unit is radians.
///
/// The value may be:
///
/// - a finite concrete value;
/// - a symbolic parameter;
/// - a deterministic arithmetic expression.
///
/// Phase is intentionally not normalized into `[0, 2π)`.
///
/// Keeping the original mathematical value is important for deterministic
/// compiler transformations, symbolic manipulation, and exact provenance.
/// Normalization is a target/optimization concern.
#[derive(Debug, Clone, PartialEq)]
pub struct FramePhase(Parameter);

impl FramePhase {
    /// Creates a concrete phase in radians.
    ///
    /// The value must be finite.
    pub fn radians(value: f64) -> Result<Self, FrameError> {
        if !value.is_finite() {
            return Err(FrameError::NonFinitePhase);
        }

        let parameter =
            Parameter::constant(value).map_err(FrameError::Parameter)?;

        Ok(Self(parameter))
    }

    /// Creates a phase from a canonical parameter.
    pub fn parameter(parameter: Parameter) -> Result<Self, FrameError> {
        parameter
            .validate()
            .map_err(FrameError::Parameter)?;

        Ok(Self(parameter))
    }

    /// Returns the underlying canonical parameter.
    #[must_use]
    pub fn parameter_ref(&self) -> &Parameter {
        &self.0
    }

    /// Consumes the value and returns its canonical parameter.
    #[must_use]
    pub fn into_parameter(self) -> Parameter {
        self.0
    }

    /// Returns a concrete phase when this value is non-symbolic.
    #[must_use]
    pub fn as_radians(&self) -> Option<f64> {
        self.0.as_constant()
    }

    /// Returns true when the phase contains a symbolic value.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.0.is_symbolic()
    }

    /// Validates the phase.
    pub fn validate(&self) -> Result<(), FrameError> {
        self.0.validate().map_err(FrameError::Parameter)
    }
}

impl fmt::Display for FramePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_radians() {
            Some(value) => write!(formatter, "{value} rad"),
            None => write!(formatter, "symbolic-phase"),
        }
    }
}

// =============================================================================
// Frame definition
// =============================================================================

/// Immutable semantic definition of a quantum-control frame.
///
/// A frame provides a reference frequency and phase for control operations.
///
/// The frame itself is not a pulse and is not scheduled.
///
/// A backend may later lower this definition to:
///
/// - an oscillator;
/// - a phase accumulator;
/// - a virtual frame;
/// - an optical reference;
/// - another technology-specific control representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    id: FrameId,
    namespace: Option<FrameNamespace>,
    name: Option<FrameName>,
    target: Option<FrameTarget>,
    frequency: Option<FrameFrequency>,
    phase: Option<FramePhase>,
}

impl Frame {
    /// Creates an empty semantic frame definition.
    ///
    /// A frame may initially omit frequency or phase when those values are
    /// supplied later by a compiler stage or inherited from another semantic
    /// construct.
    #[must_use]
    pub const fn new(id: FrameId) -> Self {
        Self {
            id,
            namespace: None,
            name: None,
            target: None,
            frequency: None,
            phase: None,
        }
    }

    /// Returns the frame identifier.
    #[must_use]
    pub const fn id(&self) -> FrameId {
        self.id
    }

    /// Sets a namespace.
    pub fn with_namespace(
        mut self,
        namespace: FrameNamespace,
    ) -> Self {
        self.namespace = Some(namespace);
        self
    }

    /// Sets a frame name.
    pub fn with_name(mut self, name: FrameName) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets a frame target.
    pub fn with_target<T: Into<FrameTarget>>(
        mut self,
        target: T,
    ) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Sets the reference frequency.
    pub fn with_frequency(
        mut self,
        frequency: FrameFrequency,
    ) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Sets the reference phase.
    pub fn with_phase(
        mut self,
        phase: FramePhase,
    ) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> Option<&FrameNamespace> {
        self.namespace.as_ref()
    }

    /// Returns the frame name.
    #[must_use]
    pub fn name(&self) -> Option<&FrameName> {
        self.name.as_ref()
    }

    /// Returns the target.
    #[must_use]
    pub fn target(&self) -> Option<&FrameTarget> {
        self.target.as_ref()
    }

    /// Returns the reference frequency.
    #[must_use]
    pub fn frequency(&self) -> Option<&FrameFrequency> {
        self.frequency.as_ref()
    }

    /// Returns the reference phase.
    #[must_use]
    pub fn phase(&self) -> Option<&FramePhase> {
        self.phase.as_ref()
    }

    /// Returns true when a reference frequency is defined.
    #[must_use]
    pub fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }

    /// Returns true when a reference phase is defined.
    #[must_use]
    pub fn has_phase(&self) -> bool {
        self.phase.is_some()
    }

    /// Validates the frame definition.
    ///
    /// This checks only IR-level structural and value validity.
    ///
    /// It does NOT check:
    ///
    /// - whether the target hardware exists;
    /// - whether a frequency is supported;
    /// - whether a channel exists;
    /// - whether a calibration exists;
    /// - whether a target qubit is routable.
    pub fn validate(&self) -> Result<(), FrameError> {
        if let Some(namespace) = &self.namespace {
            validate_text(
                namespace.as_str(),
                DEFAULT_MAX_FRAME_NAMESPACE_BYTES,
                "frame namespace",
            )?;
        }

        if let Some(name) = &self.name {
            validate_text(
                name.as_str(),
                DEFAULT_MAX_FRAME_NAME_BYTES,
                "frame name",
            )?;
        }

        if let Some(frequency) = &self.frequency {
            frequency.validate()?;
        }

        if let Some(phase) = &self.phase {
            phase.validate()?;
        }

        Ok(())
    }

    /// Creates a frame with a concrete frequency.
    pub fn with_frequency_hz(
        mut self,
        frequency_hz: f64,
    ) -> Result<Self, FrameError> {
        self.frequency =
            Some(FrameFrequency::hz(frequency_hz)?);

        Ok(self)
    }

    /// Creates a frame with a concrete phase.
    pub fn with_phase_radians(
        mut self,
        phase_radians: f64,
    ) -> Result<Self, FrameError> {
        self.phase =
            Some(FramePhase::radians(phase_radians)?);

        Ok(self)
    }
}

// =============================================================================
// Frame reference
// =============================================================================

/// Reference to an already-declared frame.
///
/// This is intentionally a lightweight opaque reference rather than an
/// embedded frame definition.
///
/// It allows operations to refer to frames without copying complete frame
/// definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FrameRef(FrameId);

impl FrameRef {
    /// Creates a frame reference.
    #[must_use]
    pub const fn new(id: FrameId) -> Self {
        Self(id)
    }

    /// Returns the referenced frame identifier.
    #[must_use]
    pub const fn id(self) -> FrameId {
        self.0
    }
}

impl From<FrameId> for FrameRef {
    fn from(id: FrameId) -> Self {
        Self::new(id)
    }
}

impl From<FrameRef> for FrameId {
    fn from(reference: FrameRef) -> Self {
        reference.id()
    }
}

impl fmt::Display for FrameRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Frame change
// =============================================================================

/// Semantic frame-change operation.
///
/// A frame change modifies the control reference.
///
/// It is deliberately not a scheduled operation. Scheduling is owned by the
/// scheduling subsystem.
///
/// A frame change can:
///
/// - shift phase;
/// - shift frequency;
/// - reset the frame;
/// - perform more than one of these atomically as one semantic operation.
///
/// Hardware-specific lowering determines how the change is implemented.
#[derive(Debug, Clone, PartialEq)]
pub struct FrameChange {
    frame: FrameRef,
    phase_shift: Option<FramePhase>,
    frequency_shift: Option<FrameFrequency>,
    action: FrameChangeAction,
}

impl FrameChange {
    /// Creates a phase-shift operation.
    pub fn phase_shift(
        frame: FrameRef,
        phase: FramePhase,
    ) -> Self {
        Self {
            frame,
            phase_shift: Some(phase),
            frequency_shift: None,
            action: FrameChangeAction::Update,
        }
    }

    /// Creates a frequency-shift operation.
    pub fn frequency_shift(
        frame: FrameRef,
        frequency: FrameFrequency,
    ) -> Self {
        Self {
            frame,
            phase_shift: None,
            frequency_shift: Some(frequency),
            action: FrameChangeAction::Update,
        }
    }

    /// Creates a combined frame update.
    pub fn update(
        frame: FrameRef,
        phase_shift: Option<FramePhase>,
        frequency_shift: Option<FrameFrequency>,
    ) -> Result<Self, FrameError> {
        if phase_shift.is_none() && frequency_shift.is_none() {
            return Err(FrameError::EmptyFrameUpdate);
        }

        Ok(Self {
            frame,
            phase_shift,
            frequency_shift,
            action: FrameChangeAction::Update,
        })
    }

    /// Creates a reset operation.
    #[must_use]
    pub const fn reset(frame: FrameRef) -> Self {
        Self {
            frame,
            phase_shift: None,
            frequency_shift: None,
            action: FrameChangeAction::Reset,
        }
    }

    /// Returns the referenced frame.
    #[must_use]
    pub const fn frame(&self) -> FrameRef {
        self.frame
    }

    /// Returns the phase shift.
    #[must_use]
    pub fn phase_shift_value(&self) -> Option<&FramePhase> {
        self.phase_shift.as_ref()
    }

    /// Returns the frequency shift.
    #[must_use]
    pub fn frequency_shift_value(
        &self,
    ) -> Option<&FrameFrequency> {
        self.frequency_shift.as_ref()
    }

    /// Returns the frame-change action.
    #[must_use]
    pub const fn action(&self) -> FrameChangeAction {
        self.action
    }

    /// Returns whether this operation resets the frame.
    #[must_use]
    pub const fn is_reset(&self) -> bool {
        matches!(self.action, FrameChangeAction::Reset)
    }

    /// Returns whether this operation updates the frame.
    #[must_use]
    pub const fn is_update(&self) -> bool {
        matches!(self.action, FrameChangeAction::Update)
    }

    /// Validates the frame change.
    pub fn validate(&self) -> Result<(), FrameError> {
        match self.action {
            FrameChangeAction::Reset => {
                if self.phase_shift.is_some()
                    || self.frequency_shift.is_some()
                {
                    return Err(
                        FrameError::ResetWithUpdateValues,
                    );
                }

                Ok(())
            }

            FrameChangeAction::Update => {
                if self.phase_shift.is_none()
                    && self.frequency_shift.is_none()
                {
                    return Err(FrameError::EmptyFrameUpdate);
                }

                if let Some(phase) = &self.phase_shift {
                    phase.validate()?;
                }

                if let Some(frequency) = &self.frequency_shift {
                    frequency.validate()?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Frame change action
// =============================================================================

/// Semantic action performed by a [`FrameChange`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FrameChangeAction {
    /// Update one or more frame references.
    Update,

    /// Restore the frame's defined reference state.
    Reset,
}

// =============================================================================
// Frame collection
// =============================================================================

/// Deterministic collection of frame definitions.
///
/// The collection uses a vector because `FrameId` is a stable namespace
/// index. It performs explicit checked indexing and never uses unchecked
/// access.
///
/// The collection does not impose an architectural maximum number of frames.
/// Callers may enforce a resource policy before construction or insertion.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameRegistry {
    frames: Vec<Frame>,
}

impl FrameRegistry {
    /// Creates an empty frame registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }

    /// Creates a registry with caller-specified capacity.
    ///
    /// Allocation failure is represented by Rust's standard allocator
    /// behavior; callers dealing with hostile/untrusted resource sizes should
    /// enforce their resource policy before calling this method.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            frames: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of registered frames.
    #[must_use]
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Returns whether the registry contains no frames.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Returns the next frame identifier.
    ///
    /// Returns `None` only if the registry length cannot be incremented
    /// without overflowing `usize`.
    #[must_use]
    pub fn next_id(&self) -> Option<FrameId> {
        Some(FrameId::new(self.frames.len()))
    }

    /// Inserts a frame using its existing identifier.
    ///
    /// The identifier must equal the next sequential registry identifier.
    ///
    /// This prevents accidental sparse allocation and makes registry
    /// serialization deterministic.
    pub fn insert(
        &mut self,
        frame: Frame,
    ) -> Result<FrameRef, FrameError> {
        frame.validate()?;

        let expected = FrameId::new(self.frames.len());

        if frame.id() != expected {
            return Err(FrameError::NonSequentialFrameId {
                expected,
                actual: frame.id(),
            });
        }

        self.frames.push(frame);

        Ok(FrameRef::new(expected))
    }

    /// Creates and inserts a new frame.
    pub fn define(
        &mut self,
        namespace: Option<FrameNamespace>,
        name: Option<FrameName>,
        target: Option<FrameTarget>,
        frequency: Option<FrameFrequency>,
        phase: Option<FramePhase>,
    ) -> Result<FrameRef, FrameError> {
        let id = self
            .next_id()
            .ok_or(FrameError::IdentifierOverflow)?;

        let frame = Frame {
            id,
            namespace,
            name,
            target,
            frequency,
            phase,
        };

        self.insert(frame)
    }

    /// Returns a frame by identifier.
    #[must_use]
    pub fn get(&self, id: FrameId) -> Option<&Frame> {
        self.frames.get(id.index())
    }

    /// Returns a mutable frame by identifier.
    ///
    /// Mutation is intentionally exposed only through a validated frame
    /// object. Callers should revalidate the frame after mutation.
    #[must_use]
    pub fn get_mut(&mut self, id: FrameId) -> Option<&mut Frame> {
        self.frames.get_mut(id.index())
    }

    /// Returns an iterator over frames in deterministic identifier order.
    pub fn iter(&self) -> impl Iterator<Item = &Frame> {
        self.frames.iter()
    }

    /// Validates every frame in the registry.
    pub fn validate(&self) -> Result<(), FrameError> {
        for frame in &self.frames {
            frame.validate()?;
        }

        Ok(())
    }

    /// Returns a checked slice of all frames.
    #[must_use]
    pub fn as_slice(&self) -> &[Frame] {
        &self.frames
    }
}

// =============================================================================
// Frame validation policy
// =============================================================================

/// Explicit policy for validating frame collections.
///
/// The policy is deliberately independent from the representation so that a
/// deployment can choose a more restrictive or more permissive resource
/// budget without changing frame semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameValidationPolicy {
    /// Maximum frame-name byte length.
    pub max_name_bytes: usize,

    /// Maximum namespace byte length.
    pub max_namespace_bytes: usize,

    /// Maximum frame changes validated by one batch operation.
    pub max_frame_changes: usize,
}

impl Default for FrameValidationPolicy {
    fn default() -> Self {
        Self {
            max_name_bytes: DEFAULT_MAX_FRAME_NAME_BYTES,
            max_namespace_bytes: DEFAULT_MAX_FRAME_NAMESPACE_BYTES,
            max_frame_changes: DEFAULT_MAX_FRAME_CHANGES,
        }
    }
}

impl FrameValidationPolicy {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        max_name_bytes: usize,
        max_namespace_bytes: usize,
        max_frame_changes: usize,
    ) -> Self {
        Self {
            max_name_bytes,
            max_namespace_bytes,
            max_frame_changes,
        }
    }

    /// Creates a policy that does not impose a frame-change count limit.
    #[must_use]
    pub const fn unlimited_frame_changes(
        max_name_bytes: usize,
        max_namespace_bytes: usize,
    ) -> Self {
        Self {
            max_name_bytes,
            max_namespace_bytes,
            max_frame_changes: usize::MAX,
        }
    }

    /// Validates the policy.
    pub const fn validate(self) -> Result<(), FramePolicyError> {
        if self.max_name_bytes == 0 {
            return Err(FramePolicyError::ZeroNameLimit);
        }

        if self.max_namespace_bytes == 0 {
            return Err(FramePolicyError::ZeroNamespaceLimit);
        }

        Ok(())
    }
}

/// Errors in a frame validation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramePolicyError {
    /// Frame-name policy cannot be zero.
    ZeroNameLimit,

    /// Namespace policy cannot be zero.
    ZeroNamespaceLimit,
}

impl fmt::Display for FramePolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroNameLimit => {
                formatter.write_str(
                    "frame name byte limit cannot be zero",
                )
            }

            Self::ZeroNamespaceLimit => {
                formatter.write_str(
                    "frame namespace byte limit cannot be zero",
                )
            }
        }
    }
}

impl std::error::Error for FramePolicyError {}

// =============================================================================
// Batch validation
// =============================================================================

/// Validates a sequence of frame changes with an explicit policy.
///
/// The function is iterative and therefore does not consume call-stack depth
/// proportional to the number of changes.
pub fn validate_frame_changes(
    changes: &[FrameChange],
    policy: FrameValidationPolicy,
) -> Result<(), FrameError> {
    policy
        .validate()
        .map_err(FrameError::InvalidPolicy)?;

    if changes.len() > policy.max_frame_changes {
        return Err(FrameError::FrameChangeLimitExceeded {
            actual: changes.len(),
            maximum: policy.max_frame_changes,
        });
    }

    for change in changes {
        change.validate()?;
    }

    Ok(())
}

// =============================================================================
// Semantic helpers
// =============================================================================

/// Creates a concrete frame frequency in hertz.
///
/// This helper exists so callers do not need to know the internal wrapper
/// representation.
pub fn frequency_hz(value: f64) -> Result<FrameFrequency, FrameError> {
    FrameFrequency::hz(value)
}

/// Creates a concrete frame phase in radians.
pub fn phase_radians(value: f64) -> Result<FramePhase, FrameError> {
    FramePhase::radians(value)
}

/// Creates a logical frame target.
#[must_use]
pub const fn logical_frame_target(qubit: QubitId) -> FrameTarget {
    FrameTarget::Logical(qubit)
}

/// Creates a physical frame target.
#[must_use]
pub const fn physical_frame_target(
    qubit: PhysicalQubitId,
) -> FrameTarget {
    FrameTarget::Physical(qubit)
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the frame IR module.
///
/// These errors are local semantic errors. The future `errors.rs` integration
/// layer may wrap them into the canonical `IrError` taxonomy without changing
/// the frame representation.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameError {
    /// Frame name is empty.
    EmptyName,

    /// Frame namespace is empty.
    EmptyNamespace,

    /// Text exceeds the configured byte limit.
    TextTooLong {
        /// Semantic field name.
        field: &'static str,

        /// Actual UTF-8 byte length.
        actual_bytes: usize,

        /// Maximum permitted bytes.
        maximum_bytes: usize,
    },

    /// Frequency is NaN or infinite.
    NonFiniteFrequency,

    /// Phase is NaN or infinite.
    NonFinitePhase,

    /// Canonical parameter validation failed.
    Parameter(crate::quantum::ir::errors::IrError),

    /// Frame update contains neither a phase nor a frequency change.
    EmptyFrameUpdate,

    /// Reset was combined with update values.
    ResetWithUpdateValues,

    /// Frame identifier does not match the registry's next identifier.
    NonSequentialFrameId {
        /// Identifier expected by the registry.
        expected: FrameId,

        /// Identifier supplied by the caller.
        actual: FrameId,
    },

    /// The frame namespace index overflowed.
    IdentifierOverflow,

    /// Validation policy is invalid.
    InvalidPolicy(FramePolicyError),

    /// Too many frame changes were supplied under the active policy.
    FrameChangeLimitExceeded {
        /// Number of supplied changes.
        actual: usize,

        /// Policy maximum.
        maximum: usize,
    },
}

impl fmt::Display for FrameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("frame name cannot be empty")
            }

            Self::EmptyNamespace => {
                formatter.write_str(
                    "frame namespace cannot be empty",
                )
            }

            Self::TextTooLong {
                field,
                actual_bytes,
                maximum_bytes,
            } => write!(
                formatter,
                "{field} is {actual_bytes} bytes but the maximum is {maximum_bytes} bytes"
            ),

            Self::NonFiniteFrequency => {
                formatter.write_str(
                    "frame frequency must be finite",
                )
            }

            Self::NonFinitePhase => {
                formatter.write_str(
                    "frame phase must be finite",
                )
            }

            Self::Parameter(error) => {
                write!(formatter, "invalid frame parameter: {error}")
            }

            Self::EmptyFrameUpdate => {
                formatter.write_str(
                    "frame update must change phase or frequency",
                )
            }

            Self::ResetWithUpdateValues => {
                formatter.write_str(
                    "frame reset cannot contain phase or frequency update values",
                )
            }

            Self::NonSequentialFrameId { expected, actual } => {
                write!(
                    formatter,
                    "frame identifier {actual} is not the next registry identifier {expected}"
                )
            }

            Self::IdentifierOverflow => {
                formatter.write_str(
                    "frame identifier namespace overflowed",
                )
            }

            Self::InvalidPolicy(error) => {
                write!(formatter, "invalid frame validation policy: {error}")
            }

            Self::FrameChangeLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "frame-change count {actual} exceeds validation limit {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for FrameError {}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_text(
    value: &str,
    maximum_bytes: usize,
    field: &'static str,
) -> Result<(), FrameError> {
    if value.len() > maximum_bytes {
        return Err(FrameError::TextTooLong {
            field,
            actual_bytes: value.len(),
            maximum_bytes,
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::parameter::Parameter;
    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
        QubitRef,
    };

    #[test]
    fn frame_id_is_stable_and_checked() {
        let id = FrameId::new(42);

        assert_eq!(id.index(), 42);
        assert_eq!(id.checked_next(), Some(FrameId::new(43)));
        assert_eq!(id.to_string(), "frame42");
    }

    #[test]
    fn frame_id_overflow_is_checked() {
        let id = FrameId::new(usize::MAX);

        assert_eq!(id.checked_next(), None);
    }

    #[test]
    fn logical_qubit_target_uses_canonical_qubit_namespace() {
        let qubit = QubitId::new(17);
        let target = FrameTarget::logical(qubit);

        assert_eq!(target.logical_qubit(), Some(qubit));
        assert!(target.is_logical());
        assert!(!target.is_physical());
    }

    #[test]
    fn physical_qubit_target_is_distinct() {
        let qubit = PhysicalQubitId::new(99);
        let target = FrameTarget::physical(qubit);

        assert_eq!(target.physical_qubit(), Some(qubit));
        assert!(target.is_physical());
        assert!(!target.is_logical());
    }

    #[test]
    fn qubit_ref_converts_without_losing_namespace() {
        let logical = QubitRef::Logical(QubitId::new(3));
        let physical =
            QubitRef::Physical(PhysicalQubitId::new(8));

        assert_eq!(
            FrameTarget::from_qubit_ref(logical)
                .logical_qubit(),
            Some(QubitId::new(3))
        );

        assert_eq!(
            FrameTarget::from_qubit_ref(physical)
                .physical_qubit(),
            Some(PhysicalQubitId::new(8))
        );
    }

    #[test]
    fn concrete_frequency_is_strongly_typed() {
        let frequency =
            FrameFrequency::hz(5.0e9).expect("finite frequency");

        assert_eq!(frequency.as_hz(), Some(5.0e9));
        assert!(!frequency.is_symbolic());
        assert!(frequency.validate().is_ok());
    }

    #[test]
    fn concrete_phase_is_strongly_typed() {
        let phase =
            FramePhase::radians(std::f64::consts::FRAC_PI_2)
                .expect("finite phase");

        assert_eq!(
            phase.as_radians(),
            Some(std::f64::consts::FRAC_PI_2)
        );
        assert!(!phase.is_symbolic());
        assert!(phase.validate().is_ok());
    }

    #[test]
    fn symbolic_frequency_is_supported() {
        let parameter =
            Parameter::symbol("drive_frequency")
                .expect("valid symbol");

        let frequency =
            FrameFrequency::parameter(parameter)
                .expect("valid symbolic frequency");

        assert!(frequency.is_symbolic());
        assert!(frequency.as_hz().is_none());
        assert!(frequency.validate().is_ok());
    }

    #[test]
    fn symbolic_phase_is_supported() {
        let parameter =
            Parameter::symbol("phase")
                .expect("valid symbol");

        let phase =
            FramePhase::parameter(parameter)
                .expect("valid symbolic phase");

        assert!(phase.is_symbolic());
        assert!(phase.as_radians().is_none());
        assert!(phase.validate().is_ok());
    }

    #[test]
    fn non_finite_frequency_is_rejected() {
        assert!(matches!(
            FrameFrequency::hz(f64::NAN),
            Err(FrameError::NonFiniteFrequency)
        ));

        assert!(matches!(
            FrameFrequency::hz(f64::INFINITY),
            Err(FrameError::NonFiniteFrequency)
        ));
    }

    #[test]
    fn non_finite_phase_is_rejected() {
        assert!(matches!(
            FramePhase::radians(f64::NAN),
            Err(FrameError::NonFinitePhase)
        ));

        assert!(matches!(
            FramePhase::radians(f64::NEG_INFINITY),
            Err(FrameError::NonFinitePhase)
        ));
    }

    #[test]
    fn frame_definition_validates() {
        let frame = Frame::new(FrameId::new(0))
            .with_target(QubitId::new(0))
            .with_frequency_hz(5.0e9)
            .expect("valid frequency")
            .with_phase_radians(0.0)
            .expect("valid phase");

        assert!(frame.validate().is_ok());
        assert_eq!(
            frame.target().and_then(FrameTarget::logical_qubit),
            Some(QubitId::new(0))
        );
        assert_eq!(
            frame.frequency()
                .and_then(FrameFrequency::as_hz),
            Some(5.0e9)
        );
        assert_eq!(
            frame.phase()
                .and_then(FramePhase::as_radians),
            Some(0.0)
        );
    }

    #[test]
    fn phase_change_is_valid() {
        let change = FrameChange::phase_shift(
            FrameRef::new(FrameId::new(0)),
            FramePhase::radians(
                std::f64::consts::FRAC_PI_2,
            )
            .expect("valid phase"),
        );

        assert!(change.validate().is_ok());
        assert!(change.is_update());
        assert!(!change.is_reset());
    }

    #[test]
    fn frequency_change_is_valid() {
        let change = FrameChange::frequency_shift(
            FrameRef::new(FrameId::new(0)),
            FrameFrequency::hz(10.0e6)
                .expect("valid frequency"),
        );

        assert!(change.validate().is_ok());
        assert!(change.is_update());
    }

    #[test]
    fn combined_change_requires_at_least_one_value() {
        assert!(matches!(
            FrameChange::update(
                FrameRef::new(FrameId::new(0)),
                None,
                None,
            ),
            Err(FrameError::EmptyFrameUpdate)
        ));
    }

    #[test]
    fn reset_contains_no_update_values() {
        let change =
            FrameChange::reset(FrameRef::new(FrameId::new(7)));

        assert!(change.validate().is_ok());
        assert!(change.is_reset());
        assert!(change.phase_shift_value().is_none());
        assert!(change.frequency_shift_value().is_none());
    }

    #[test]
    fn reset_with_update_values_is_rejected() {
        let change = FrameChange {
            frame: FrameRef::new(FrameId::new(0)),
            phase_shift: Some(
                FramePhase::radians(1.0)
                    .expect("valid phase"),
            ),
            frequency_shift: None,
            action: FrameChangeAction::Reset,
        };

        assert!(matches!(
            change.validate(),
            Err(FrameError::ResetWithUpdateValues)
        ));
    }

    #[test]
    fn registry_assigns_deterministic_ids() {
        let mut registry = FrameRegistry::new();

        let first = registry
            .define(
                None,
                None,
                Some(QubitId::new(0).into()),
                Some(
                    FrameFrequency::hz(5.0e9)
                        .expect("valid frequency"),
                ),
                Some(
                    FramePhase::radians(0.0)
                        .expect("valid phase"),
                ),
            )
            .expect("first frame");

        let second = registry
            .define(
                None,
                None,
                Some(QubitId::new(1).into()),
                None,
                None,
            )
            .expect("second frame");

        assert_eq!(first.id(), FrameId::new(0));
        assert_eq!(second.id(), FrameId::new(1));
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn registry_rejects_non_sequential_ids() {
        let mut registry = FrameRegistry::new();

        let frame = Frame::new(FrameId::new(10));

        assert!(matches!(
            registry.insert(frame),
            Err(FrameError::NonSequentialFrameId { .. })
        ));
    }

    #[test]
    fn registry_lookup_is_checked() {
        let mut registry = FrameRegistry::new();

        registry
            .define(None, None, None, None, None)
            .expect("frame");

        assert!(registry.get(FrameId::new(0)).is_some());
        assert!(registry.get(FrameId::new(1)).is_none());
    }

    #[test]
    fn frame_change_batch_validation_is_iterative() {
        let changes = [
            FrameChange::phase_shift(
                FrameRef::new(FrameId::new(0)),
                FramePhase::radians(0.1)
                    .expect("valid phase"),
            ),
            FrameChange::frequency_shift(
                FrameRef::new(FrameId::new(0)),
                FrameFrequency::hz(1.0e6)
                    .expect("valid frequency"),
            ),
            FrameChange::reset(
                FrameRef::new(FrameId::new(0)),
            ),
        ];

        let policy = FrameValidationPolicy::new(
            DEFAULT_MAX_FRAME_NAME_BYTES,
            DEFAULT_MAX_FRAME_NAMESPACE_BYTES,
            3,
        );

        assert!(
            validate_frame_changes(&changes, policy).is_ok()
        );
    }

    #[test]
    fn frame_change_policy_rejects_excessive_batch() {
        let changes = [
            FrameChange::reset(
                FrameRef::new(FrameId::new(0)),
            ),
            FrameChange::reset(
                FrameRef::new(FrameId::new(1)),
            ),
        ];

        let policy = FrameValidationPolicy::new(
            DEFAULT_MAX_FRAME_NAME_BYTES,
            DEFAULT_MAX_FRAME_NAMESPACE_BYTES,
            1,
        );

        assert!(matches!(
            validate_frame_changes(&changes, policy),
            Err(FrameError::FrameChangeLimitExceeded {
                actual: 2,
                maximum: 1
            })
        ));
    }

    #[test]
    fn empty_name_is_rejected() {
        assert!(matches!(
            FrameName::new(""),
            Err(FrameError::EmptyName)
        ));
    }

    #[test]
    fn empty_namespace_is_rejected() {
        assert!(matches!(
            FrameNamespace::new(""),
            Err(FrameError::EmptyNamespace)
        ));
    }

    #[test]
    fn phase_is_not_implicitly_normalized() {
        let phase =
            FramePhase::radians(10.0 * std::f64::consts::PI)
                .expect("finite phase");

        assert_eq!(
            phase.as_radians(),
            Some(10.0 * std::f64::consts::PI)
        );
    }

    #[test]
    fn named_target_is_hardware_independent() {
        let target =
            FrameTarget::named("control_reference")
                .expect("valid target");

        assert!(target.is_named());
        assert_eq!(
            target.named_target(),
            Some("control_reference")
        );
    }
}