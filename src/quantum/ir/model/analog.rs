//! Zamani Quantum IR — Analog Quantum Computation Model
//!
//! Path:
//!     src/quantum/ir/model/analog.rs
//!
//! # Purpose
//!
//! This module defines the canonical, hardware-independent semantic model for
//! analog quantum computation and analog Hamiltonian evolution.
//!
//! It represents WHAT an analog quantum program means without deciding:
//!
//! - which physical machine executes it;
//! - how logical qubits are mapped to physical resources;
//! - which hardware channels are used;
//! - which calibration is selected;
//! - how a Hamiltonian is synthesized physically;
//! - how a waveform is sampled;
//! - how a schedule is constructed;
//! - how a provider API is called;
//! - how a simulator represents quantum state;
//! - how noise is simulated;
//! - how results are retrieved.
//!
//! Those responsibilities belong to downstream IR, hardware, scheduling,
//! simulator and backend subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum frontend
//!      │
//!      ▼
//! canonical Zamani IR
//!      │
//!      ├───────────────────────────────┐
//!      │                               │
//!      ▼                               ▼
//! gate model                     analog model
//!                                      │
//!                                      ▼
//!                           Hamiltonian / controls
//!                                      │
//!                                      ▼
//!                           target-independent passes
//!                                      │
//!                                      ▼
//!                               target capabilities
//!                                      │
//!                                      ▼
//!                                physical mapping
//!                                      │
//!                                      ▼
//!                                  scheduling
//!                                      │
//!                                      ▼
//!                              hardware lowering
//!                                      │
//!                                      ▼
//!                                   backend
//! ```
//!
//! # Universal-program principle
//!
//! The analog model must represent the same semantic program whether the
//! target ultimately contains:
//!
//! - one quantum resource;
//! - hundreds;
//! - thousands;
//! - millions;
//! - or any larger finite number permitted by the compilation/execution
//!   environment.
//!
//! The number of resources is DATA.
//!
//! It is never encoded as an architectural constant in this module.
//!
//! Therefore this file deliberately does NOT define:
//!
//! ```text
//! MAX_QUBITS
//! MAX_ATOMS
//! MAX_RESOURCES
//! MAX_TERMS
//! MAX_DIMENSION
//! MAX_TIME_SAMPLES
//! ```
//!
//! Resource/security limits belong to `quantum::ir::limits` and/or an
//! explicitly supplied validation policy.
//!
//! # Qubit identity
//!
//! When an analog resource represents a logical qubit, this module uses the
//! canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! It does NOT define another qubit identifier.
//!
//! # Analog semantics
//!
//! An analog program is fundamentally a continuous evolution problem:
//!
//! ```text
//! d|ψ(t)> / dt = -i H(t) |ψ(t)> / ħ
//! ```
//!
//! The IR does not require the hardware to expose this equation literally.
//! Instead, it represents the semantic Hamiltonian and its time-dependent
//! controls.
//!
//! A program may contain:
//!
//! - logical quantum resources;
//! - spatial coordinates;
//! - local controls;
//! - global controls;
//! - pair interactions;
//! - many-body interactions;
//! - symbolic coefficients;
//! - time-dependent coefficients;
//! - initial-state declarations;
//! - evolution windows;
//! - observables;
//! - metadata;
//! - capability requirements;
//! - extensible operator kinds.
//!
//! # Important distinction
//!
//! `AnalogProgram` is NOT a hardware program.
//!
//! It is a semantic analog workload.
//!
//! A backend may lower it into:
//!
//! - neutral-atom analog Hamiltonian instructions;
//! - superconducting analog controls;
//! - trapped-ion analog evolution;
//! - photonic continuous-variable controls;
//! - spin-system controls;
//! - simulator instructions;
//! - future quantum architectures.
//!
//! # Integration contract
//!
//! Upstream dependencies:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::parameter::Parameter
//! ```
//!
//! Downstream consumers:
//!
//! ```text
//! quantum::ir::model::hamiltonian
//! quantum::ir::pulse
//! quantum::ir::timing
//! quantum::ir::resources
//! quantum::ir::validation
//! quantum::ir::serialization
//! quantum::ir::hashing
//! quantum::hardware
//! quantum::optimization
//! quantum::scheduling
//! quantum::simulator
//! quantum::backend
//! ```
//!
//! None of those downstream modules are required to own this file.
//!
//! # Serialization
//!
//! This module intentionally does not define the repository-wide serialization
//! format. Canonical serialization belongs to `quantum::ir::serialization`.
//!
//! The data structures in this file are deterministic and contain no hidden
//! runtime state, so the serialization layer can serialize them without
//! depending on hardware or process state.
//!
//! # Hashing
//!
//! This module does not own canonical hashing.
//!
//! Canonical hashing belongs to the IR hashing layer.
//!
//! # Determinism
//!
//! This module:
//!
//! - never reads the system clock;
//! - never reads randomness;
//! - never accesses a network;
//! - never accesses hardware;
//! - never accesses environment variables;
//! - never depends on hash-map iteration order.
//!
//! Ordered semantic collections use `Vec`.
//!
//! Unordered semantic sets use `BTreeSet`.
//!
//! Metadata uses `BTreeMap`.
//!
//! # Numeric policy
//!
//! Concrete floating-point values must be finite.
//!
//! NaN and positive/negative infinity are rejected.
//!
//! Symbolic values are represented using the canonical `Parameter` type.
//!
//! This permits programs such as:
//!
//! ```text
//! Ω(t) = Ω_max * envelope(t)
//! Δ(t) = Δ_0 + α * t
//! ```
//!
//! without prematurely selecting hardware values.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Safety
//!
//! This module forbids unsafe Rust.
//!
//! # Ownership
//!
//! This module owns the semantic analog-program model.
//!
//! It does NOT own:
//!
//! - generic Hamiltonian algebra beyond what is needed to describe an analog
//!   workload;
//! - hardware topology;
//! - physical allocation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - waveform DAC conversion;
//! - simulator state;
//! - provider APIs.
//!
//! Those remain separate concerns.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic identifier for the analog IR model.
pub const ANALOG_MODEL_ID: &str = "zamani.quantum.ir.model.analog";

/// Semantic version of this model.
pub const ANALOG_MODEL_VERSION: u16 = 1;

// =============================================================================
// Result
// =============================================================================

/// Result type used by this module.
pub type AnalogResult<T> = Result<T, AnalogError>;

// =============================================================================
// Error model
// =============================================================================

/// Errors produced while constructing or validating analog IR.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalogError {
    /// A required field is empty.
    EmptyField {
        /// Field name.
        field: &'static str,
    },

    /// A string field is empty.
    EmptyString {
        /// Field name.
        field: &'static str,
    },

    /// A floating-point value is not finite.
    NonFiniteValue {
        /// Field name.
        field: &'static str,

        /// Supplied value.
        value: f64,
    },

    /// A floating-point value violates an explicit semantic range.
    ValueOutOfRange {
        /// Field name.
        field: &'static str,

        /// Supplied value.
        value: f64,

        /// Optional lower bound.
        minimum: Option<f64>,

        /// Optional upper bound.
        maximum: Option<f64>,
    },

    /// An index refers to a nonexistent analog resource.
    ResourceOutOfBounds {
        /// Referenced resource.
        resource: usize,

        /// Number of declared resources.
        resource_count: usize,
    },

    /// A pair interaction references the same resource twice.
    SelfInteraction {
        /// Resource identifier.
        resource: usize,
    },

    /// A resource ID occurs more than once.
    DuplicateResource {
        /// Duplicate resource ID.
        id: QubitId,
    },

    /// A spatial coordinate vector is empty.
    EmptyPosition,

    /// Spatial dimensions differ where they must be identical.
    DimensionMismatch {
        /// First dimension.
        expected: usize,

        /// Actual dimension.
        actual: usize,
    },

    /// Two resources occupy the same physical coordinate.
    CoincidentResources {
        /// First resource.
        first: QubitId,

        /// Second resource.
        second: QubitId,
    },

    /// A time sample has an invalid time.
    InvalidTime {
        /// Time value.
        time: f64,
    },

    /// A time sequence is not monotonically increasing.
    NonMonotonicTime {
        /// Previous time.
        previous: f64,

        /// Current time.
        current: f64,
    },

    /// An evolution duration is invalid.
    InvalidDuration {
        /// Duration.
        duration: f64,
    },

    /// An evolution window starts before zero.
    NegativeStartTime {
        /// Start time.
        start: f64,
    },

    /// An evolution window ends before it starts.
    InvalidEvolutionWindow {
        /// Start time.
        start: f64,

        /// End time.
        end: f64,
    },

    /// A control sequence has an inconsistent number of values.
    ControlValueMismatch {
        /// Expected value count.
        expected: usize,

        /// Actual value count.
        actual: usize,
    },

    /// A local control has an invalid target mask.
    InvalidTargetSet,

    /// A Hamiltonian term has no target resources.
    EmptyOperatorTargets,

    /// An operator contains duplicate target resources.
    DuplicateOperatorTarget {
        /// Duplicated resource.
        resource: QubitId,
    },

    /// An observable contains no terms.
    EmptyObservable,

    /// An observable has a duplicate name.
    DuplicateObservable {
        /// Name.
        name: String,
    },

    /// A capability identifier is empty.
    EmptyCapability,

    /// A metadata key is empty.
    EmptyMetadataKey,

    /// Metadata key occurs more than once.
    DuplicateMetadataKey {
        /// Key.
        key: String,
    },

    /// A parameter is invalid.
    InvalidParameter {
        /// Human-readable reason.
        reason: String,
    },
}

impl fmt::Display for AnalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "analog field `{field}` cannot be empty")
            }

            Self::EmptyString { field } => {
                write!(formatter, "analog field `{field}` cannot be empty")
            }

            Self::NonFiniteValue { field, value } => {
                write!(
                    formatter,
                    "analog field `{field}` contains non-finite value {value}"
                )
            }

            Self::ValueOutOfRange {
                field,
                value,
                minimum,
                maximum,
            } => {
                write!(
                    formatter,
                    "analog field `{field}` contains value {value} outside the permitted range"
                )?;

                if let Some(minimum) = minimum {
                    write!(formatter, " minimum={minimum}")?;
                }

                if let Some(maximum) = maximum {
                    write!(formatter, " maximum={maximum}")?;
                }

                Ok(())
            }

            Self::ResourceOutOfBounds {
                resource,
                resource_count,
            } => write!(
                formatter,
                "analog resource index {resource} is outside resource count {resource_count}"
            ),

            Self::SelfInteraction { resource } => {
                write!(
                    formatter,
                    "analog interaction cannot target resource {resource} twice"
                )
            }

            Self::DuplicateResource { id } => {
                write!(formatter, "duplicate analog resource `{id}`")
            }

            Self::EmptyPosition => {
                write!(formatter, "analog position must contain at least one coordinate")
            }

            Self::DimensionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "analog spatial dimension mismatch: expected {expected}, got {actual}"
                )
            }

            Self::CoincidentResources { first, second } => {
                write!(
                    formatter,
                    "analog resources `{first}` and `{second}` have coincident positions"
                )
            }

            Self::InvalidTime { time } => {
                write!(formatter, "invalid analog time {time}")
            }

            Self::NonMonotonicTime { previous, current } => {
                write!(
                    formatter,
                    "analog time sequence is not monotonically increasing: {previous} -> {current}"
                )
            }

            Self::InvalidDuration { duration } => {
                write!(formatter, "invalid analog duration {duration}")
            }

            Self::NegativeStartTime { start } => {
                write!(formatter, "analog evolution starts before zero: {start}")
            }

            Self::InvalidEvolutionWindow { start, end } => {
                write!(
                    formatter,
                    "invalid analog evolution window: start={start}, end={end}"
                )
            }

            Self::ControlValueMismatch { expected, actual } => {
                write!(
                    formatter,
                    "analog control value count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::InvalidTargetSet => {
                write!(formatter, "analog control contains an invalid target set")
            }

            Self::EmptyOperatorTargets => {
                write!(formatter, "analog operator must target at least one resource")
            }

            Self::DuplicateOperatorTarget { resource } => {
                write!(
                    formatter,
                    "analog operator contains duplicate target resource `{resource}`"
                )
            }

            Self::EmptyObservable => {
                write!(formatter, "analog observable must contain at least one term")
            }

            Self::DuplicateObservable { name } => {
                write!(formatter, "duplicate analog observable `{name}`")
            }

            Self::EmptyCapability => {
                write!(formatter, "analog capability identifier cannot be empty")
            }

            Self::EmptyMetadataKey => {
                write!(formatter, "analog metadata key cannot be empty")
            }

            Self::DuplicateMetadataKey { key } => {
                write!(formatter, "duplicate analog metadata key `{key}`")
            }

            Self::InvalidParameter { reason } => {
                write!(formatter, "invalid analog parameter: {reason}")
            }
        }
    }
}

impl Error for AnalogError {}

// =============================================================================
// Scalar utilities
// =============================================================================

fn finite(value: f64, field: &'static str) -> AnalogResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(AnalogError::NonFiniteValue { field, value })
    }
}

fn non_negative(value: f64, field: &'static str) -> AnalogResult<f64> {
    finite(value, field)?;

    if value < 0.0 {
        return Err(AnalogError::ValueOutOfRange {
            field,
            value,
            minimum: Some(0.0),
            maximum: None,
        });
    }

    Ok(value)
}

fn validate_parameter(parameter: &Parameter) -> AnalogResult<()> {
    parameter
        .validate()
        .map_err(|error| AnalogError::InvalidParameter {
            reason: error.to_string(),
        })
}

// =============================================================================
// Spatial position
// =============================================================================

/// Hardware-independent spatial coordinate.
///
/// Coordinates are semantic coordinates supplied by the analog program.
/// Their physical interpretation is determined by the target architecture.
///
/// For example, a target may interpret coordinates as metres, while another
/// target may use another canonical physical unit after an explicit lowering
/// step.
///
/// The IR itself does not silently convert units.
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    coordinates: Vec<f64>,
}

impl Position {
    /// Creates a spatial position.
    pub fn new(coordinates: Vec<f64>) -> AnalogResult<Self> {
        if coordinates.is_empty() {
            return Err(AnalogError::EmptyPosition);
        }

        for value in &coordinates {
            finite(*value, "position.coordinate")?;
        }

        Ok(Self { coordinates })
    }

    /// Creates a one-dimensional position.
    pub fn one_dimensional(value: f64) -> AnalogResult<Self> {
        Self::new(vec![value])
    }

    /// Creates a two-dimensional position.
    pub fn two_dimensional(x: f64, y: f64) -> AnalogResult<Self> {
        Self::new(vec![x, y])
    }

    /// Creates a three-dimensional position.
    pub fn three_dimensional(x: f64, y: f64, z: f64) -> AnalogResult<Self> {
        Self::new(vec![x, y, z])
    }

    /// Returns the coordinate dimensionality.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.coordinates.len()
    }

    /// Returns the coordinates.
    #[must_use]
    pub fn coordinates(&self) -> &[f64] {
        &self.coordinates
    }

    /// Returns a coordinate by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<f64> {
        self.coordinates.get(index).copied()
    }

    /// Calculates squared Euclidean distance.
    ///
    /// The caller must ensure dimensional compatibility.
    pub fn squared_distance(&self, other: &Self) -> AnalogResult<f64> {
        if self.dimension() != other.dimension() {
            return Err(AnalogError::DimensionMismatch {
                expected: self.dimension(),
                actual: other.dimension(),
            });
        }

        let mut distance = 0.0;

        for (left, right) in self.coordinates.iter().zip(&other.coordinates) {
            let delta = left - right;
            distance += delta * delta;
        }

        finite(distance, "position.squared_distance")
    }
}

// =============================================================================
// Analog resource
// =============================================================================

/// A logical quantum resource participating in analog evolution.
///
/// `QubitId` is the canonical Zamani logical identity.
///
/// This structure intentionally does not contain a physical-qubit identifier.
/// Physical mapping is a downstream responsibility.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogResource {
    id: QubitId,
    position: Option<Position>,
    label: Option<String>,
}

impl AnalogResource {
    /// Creates an analog resource without a spatial position.
    #[must_use]
    pub const fn new(id: QubitId) -> Self {
        Self {
            id,
            position: None,
            label: None,
        }
    }

    /// Creates an analog resource with a spatial position.
    pub fn with_position(id: QubitId, position: Position) -> Self {
        Self {
            id,
            position: Some(position),
            label: None,
        }
    }

    /// Returns the canonical logical-qubit ID.
    #[must_use]
    pub const fn id(&self) -> QubitId {
        self.id
    }

    /// Returns the spatial position.
    #[must_use]
    pub fn position(&self) -> Option<&Position> {
        self.position.as_ref()
    }

    /// Sets the semantic spatial position.
    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }

    /// Sets an optional semantic label.
    ///
    /// The label has no execution semantics.
    pub fn set_label<S: Into<String>>(&mut self, label: S) -> AnalogResult<()> {
        let label = label.into();

        if label.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "resource.label",
            });
        }

        self.label = Some(label);

        Ok(())
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

// =============================================================================
// Time samples
// =============================================================================

/// One time/value sample in a time-dependent analog control.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSample {
    time: f64,
    value: Parameter,
}

impl TimeSample {
    /// Creates a time sample.
    pub fn new(time: f64, value: Parameter) -> AnalogResult<Self> {
        non_negative(time, "time_sample.time")?;
        validate_parameter(&value)?;

        Ok(Self { time, value })
    }

    /// Returns the sample time.
    #[must_use]
    pub const fn time(&self) -> f64 {
        self.time
    }

    /// Returns the sample value.
    #[must_use]
    pub fn value(&self) -> &Parameter {
        &self.value
    }
}

// =============================================================================
// Control interpolation
// =============================================================================

/// Semantic interpolation policy for a control profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interpolation {
    /// Hold the previous value until the next sample.
    PiecewiseConstant,

    /// Linearly interpolate between adjacent samples.
    PiecewiseLinear,

    /// Interpret the samples as target-independent interpolation points whose
    /// exact interpolation is resolved downstream.
    TargetDefined,
}

// =============================================================================
// Control profile
// =============================================================================

/// A time-dependent scalar analog control.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlProfile {
    samples: Vec<TimeSample>,
    interpolation: Interpolation,
}

impl ControlProfile {
    /// Creates a control profile.
    ///
    /// Samples must be in non-decreasing semantic time order.
    pub fn new(
        samples: Vec<TimeSample>,
        interpolation: Interpolation,
    ) -> AnalogResult<Self> {
        if samples.is_empty() {
            return Err(AnalogError::EmptyField {
                field: "control_profile.samples",
            });
        }

        let mut previous = None;

        for sample in &samples {
            if let Some(previous_time) = previous {
                if sample.time() < previous_time {
                    return Err(AnalogError::NonMonotonicTime {
                        previous: previous_time,
                        current: sample.time(),
                    });
                }
            }

            previous = Some(sample.time());
        }

        Ok(Self {
            samples,
            interpolation,
        })
    }

    /// Returns the interpolation mode.
    #[must_use]
    pub const fn interpolation(&self) -> Interpolation {
        self.interpolation
    }

    /// Returns the ordered samples.
    #[must_use]
    pub fn samples(&self) -> &[TimeSample] {
        &self.samples
    }

    /// Returns the final sample time.
    #[must_use]
    pub fn end_time(&self) -> Option<f64> {
        self.samples.last().map(TimeSample::time)
    }

    /// Returns whether the profile contains symbolic values.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.samples.iter().any(|sample| sample.value().is_symbolic())
    }
}

// =============================================================================
// Target selection
// =============================================================================

/// Semantic target set for an analog control.
///
/// This avoids encoding a fixed number of target resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetSet {
    /// All resources in the enclosing analog program.
    All,

    /// Explicit logical resources.
    Explicit(Vec<QubitId>),
}

impl TargetSet {
    /// Creates an explicit target set.
    pub fn explicit(targets: Vec<QubitId>) -> AnalogResult<Self> {
        if targets.is_empty() {
            return Err(AnalogError::InvalidTargetSet);
        }

        let mut seen = BTreeSet::new();

        for target in &targets {
            if !seen.insert(*target) {
                return Err(AnalogError::DuplicateOperatorTarget {
                    resource: *target,
                });
            }
        }

        Ok(Self::Explicit(targets))
    }

    /// Returns whether this targets every resource.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::All)
    }

    /// Returns explicit targets when available.
    #[must_use]
    pub fn explicit_targets(&self) -> Option<&[QubitId]> {
        match self {
            Self::All => None,
            Self::Explicit(targets) => Some(targets),
        }
    }
}

// =============================================================================
// Analog control
// =============================================================================

/// A semantic analog control field.
///
/// A control field can be global or explicitly localized.
///
/// Examples include:
//!
//! - drive amplitude;
//! - detuning;
//! - phase;
//! - field strength;
//! - coupling coefficient;
//! - externally controlled parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogControl {
    name: String,
    targets: TargetSet,
    profile: ControlProfile,
    units: Option<String>,
}

impl AnalogControl {
    /// Creates a control field.
    pub fn new<S: Into<String>>(
        name: S,
        targets: TargetSet,
        profile: ControlProfile,
    ) -> AnalogResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "analog_control.name",
            });
        }

        Ok(Self {
            name,
            targets,
            profile,
            units: None,
        })
    }

    /// Sets semantic unit metadata.
    ///
    /// Unit interpretation is explicit metadata. No hardware conversion is
    /// performed by this module.
    pub fn with_units<S: Into<String>>(mut self, units: S) -> AnalogResult<Self> {
        let units = units.into();

        if units.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "analog_control.units",
            });
        }

        self.units = Some(units);

        Ok(self)
    }

    /// Returns the control name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the target set.
    #[must_use]
    pub fn targets(&self) -> &TargetSet {
        &self.targets
    }

    /// Returns the time profile.
    #[must_use]
    pub fn profile(&self) -> &ControlProfile {
        &self.profile
    }

    /// Returns optional unit metadata.
    #[must_use]
    pub fn units(&self) -> Option<&str> {
        self.units.as_deref()
    }
}

// =============================================================================
// Operator kinds
// =============================================================================

/// Standard semantic operator families useful to analog quantum computation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardOperator {
    /// Identity operator.
    Identity,

    /// Pauli-X operator.
    PauliX,

    /// Pauli-Y operator.
    PauliY,

    /// Pauli-Z operator.
    PauliZ,

    /// Number/projector operator.
    Number,

    /// Raising operator.
    Raising,

    /// Lowering operator.
    Lowering,

    /// Generic spin operator.
    Spin,

    /// Generic bosonic creation operator.
    Creation,

    /// Generic bosonic annihilation operator.
    Annihilation,
}

/// Extensible analog operator kind.
///
/// Standard operators cover common cases. `Custom` prevents the canonical IR
/// from becoming permanently closed around today's operator vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperatorKind {
    /// Standard semantic operator.
    Standard(StandardOperator),

    /// Namespaced extension.
    ///
    /// The namespace and name are semantic identifiers, not provider names.
    Custom {
        /// Extension namespace.
        namespace: String,

        /// Operator name.
        name: String,
    },
}

impl OperatorKind {
    /// Creates a custom operator kind.
    pub fn custom<N: Into<String>, S: Into<String>>(
        namespace: N,
        name: S,
    ) -> AnalogResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "operator.namespace",
            });
        }

        if name.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "operator.name",
            });
        }

        Ok(Self::Custom { namespace, name })
    }
}

// =============================================================================
// Hamiltonian term
// =============================================================================

/// One term in an analog Hamiltonian.
///
/// Semantically:
///
/// ```text
/// coefficient(t) * operator(targets)
/// ```
///
/// The coefficient can be constant or symbolic. Time-dependent behaviour can
/// be represented using an associated control profile rather than forcing
/// numerical evaluation during IR construction.
#[derive(Debug, Clone, PartialEq)]
pub struct HamiltonianTerm {
    operator: OperatorKind,
    targets: Vec<QubitId>,
    coefficient: Parameter,
    control: Option<String>,
    metadata: BTreeMap<String, String>,
}

impl HamiltonianTerm {
    /// Creates a Hamiltonian term.
    pub fn new(
        operator: OperatorKind,
        targets: Vec<QubitId>,
        coefficient: Parameter,
    ) -> AnalogResult<Self> {
        if targets.is_empty() {
            return Err(AnalogError::EmptyOperatorTargets);
        }

        let mut seen = BTreeSet::new();

        for target in &targets {
            if !seen.insert(*target) {
                return Err(AnalogError::DuplicateOperatorTarget {
                    resource: *target,
                });
            }
        }

        validate_parameter(&coefficient)?;

        Ok(Self {
            operator,
            targets,
            coefficient,
            control: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Associates this term with a named time-dependent control.
    pub fn with_control<S: Into<String>>(mut self, control: S) -> AnalogResult<Self> {
        let control = control.into();

        if control.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "hamiltonian_term.control",
            });
        }

        self.control = Some(control);

        Ok(self)
    }

    /// Adds semantic metadata.
    pub fn insert_metadata<S: Into<String>>(
        &mut self,
        key: S,
        value: S,
    ) -> AnalogResult<()> {
        let key = key.into();

        if key.is_empty() {
            return Err(AnalogError::EmptyMetadataKey);
        }

        if self.metadata.contains_key(&key) {
            return Err(AnalogError::DuplicateMetadataKey { key });
        }

        self.metadata.insert(key, value.into());

        Ok(())
    }

    /// Returns the operator.
    #[must_use]
    pub fn operator(&self) -> &OperatorKind {
        &self.operator
    }

    /// Returns target resources.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &Parameter {
        &self.coefficient
    }

    /// Returns the optional control name.
    #[must_use]
    pub fn control(&self) -> Option<&str> {
        self.control.as_deref()
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

// =============================================================================
// Analog Hamiltonian
// =============================================================================

/// Semantic Hamiltonian used by an analog program.
///
/// This is the analog-model representation of the Hamiltonian, not a
/// simulator matrix and not a hardware-native instruction sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogHamiltonian {
    terms: Vec<HamiltonianTerm>,
    time_dependent: bool,
}

impl AnalogHamiltonian {
    /// Creates an empty Hamiltonian.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            terms: Vec::new(),
            time_dependent: false,
        }
    }

    /// Adds a Hamiltonian term.
    pub fn push_term(&mut self, term: HamiltonianTerm) {
        if term.control().is_some() {
            self.time_dependent = true;
        }

        self.terms.push(term);
    }

    /// Creates a Hamiltonian from terms.
    pub fn from_terms(terms: Vec<HamiltonianTerm>) -> Self {
        let time_dependent = terms.iter().any(|term| term.control().is_some());

        Self {
            terms,
            time_dependent,
        }
    }

    /// Returns Hamiltonian terms.
    #[must_use]
    pub fn terms(&self) -> &[HamiltonianTerm] {
        &self.terms
    }

    /// Returns whether the Hamiltonian is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Returns the number of terms.
    #[must_use]
    pub fn term_count(&self) -> usize {
        self.terms.len()
    }

    /// Returns whether at least one term is time-dependent.
    #[must_use]
    pub const fn is_time_dependent(&self) -> bool {
        self.time_dependent
    }

    /// Validates the Hamiltonian against the declared resource IDs.
    pub fn validate(&self, resources: &[AnalogResource]) -> AnalogResult<()> {
        if self.terms.is_empty() {
            return Err(AnalogError::EmptyField {
                field: "hamiltonian.terms",
            });
        }

        let resource_ids: BTreeSet<QubitId> =
            resources.iter().map(AnalogResource::id).collect();

        for term in &self.terms {
            validate_parameter(term.coefficient())?;

            for target in term.targets() {
                if !resource_ids.contains(target) {
                    return Err(AnalogError::ResourceOutOfBounds {
                        resource: target.index(),
                        resource_count: resources.len(),
                    });
                }
            }
        }

        Ok(())
    }
}

impl Default for AnalogHamiltonian {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Evolution window
// =============================================================================

/// A semantic continuous-evolution window.
///
/// `start` and `end` are semantic program times. Their physical clock mapping
/// is resolved downstream.
#[derive(Debug, Clone, PartialEq)]
pub struct EvolutionWindow {
    start: f64,
    end: f64,
    hamiltonian: AnalogHamiltonian,
}

impl EvolutionWindow {
    /// Creates an evolution window.
    pub fn new(
        start: f64,
        end: f64,
        hamiltonian: AnalogHamiltonian,
    ) -> AnalogResult<Self> {
        non_negative(start, "evolution.start")?;
        non_negative(end, "evolution.end")?;

        if end < start {
            return Err(AnalogError::InvalidEvolutionWindow { start, end });
        }

        if hamiltonian.is_empty() {
            return Err(AnalogError::EmptyField {
                field: "evolution.hamiltonian",
            });
        }

        Ok(Self {
            start,
            end,
            hamiltonian,
        })
    }

    /// Creates an evolution window beginning at zero.
    pub fn from_duration(
        duration: f64,
        hamiltonian: AnalogHamiltonian,
    ) -> AnalogResult<Self> {
        non_negative(duration, "evolution.duration")?;

        Self::new(0.0, duration, hamiltonian)
    }

    /// Returns the semantic start time.
    #[must_use]
    pub const fn start(&self) -> f64 {
        self.start
    }

    /// Returns the semantic end time.
    #[must_use]
    pub const fn end(&self) -> f64 {
        self.end
    }

    /// Returns the semantic duration.
    #[must_use]
    pub fn duration(&self) -> f64 {
        self.end - self.start
    }

    /// Returns the Hamiltonian.
    #[must_use]
    pub fn hamiltonian(&self) -> &AnalogHamiltonian {
        &self.hamiltonian
    }
}

// =============================================================================
// Initial state
// =============================================================================

/// Initial-state semantics for analog evolution.
#[derive(Debug, Clone, PartialEq)]
pub enum InitialState {
    /// Target-specific default initial state.
    ///
    /// The target must explicitly document what this means.
    Default,

    /// Computational-basis assignment.
    ///
    /// Each entry maps a logical qubit to 0 or 1.
    ComputationalBasis(BTreeMap<QubitId, bool>),

    /// Named semantic state preparation.
    Named(String),

    /// Extensible state-preparation description.
    Custom {
        /// Namespace.
        namespace: String,

        /// Preparation name.
        name: String,

        /// Parameter values.
        parameters: Vec<Parameter>,
    },
}

impl InitialState {
    /// Creates a named initial state.
    pub fn named<S: Into<String>>(name: S) -> AnalogResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "initial_state.name",
            });
        }

        Ok(Self::Named(name))
    }

    /// Creates a custom initial state.
    pub fn custom<N: Into<String>, S: Into<String>>(
        namespace: N,
        name: S,
        parameters: Vec<Parameter>,
    ) -> AnalogResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "initial_state.namespace",
            });
        }

        if name.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "initial_state.name",
            });
        }

        for parameter in &parameters {
            validate_parameter(parameter)?;
        }

        Ok(Self::Custom {
            namespace,
            name,
            parameters,
        })
    }

    /// Validates qubits referenced by the initial state.
    pub fn validate(&self, resources: &[AnalogResource]) -> AnalogResult<()> {
        let resource_ids: BTreeSet<QubitId> =
            resources.iter().map(AnalogResource::id).collect();

        match self {
            Self::Default | Self::Named(_) => Ok(()),

            Self::ComputationalBasis(assignments) => {
                for qubit in assignments.keys() {
                    if !resource_ids.contains(qubit) {
                        return Err(AnalogError::ResourceOutOfBounds {
                            resource: qubit.index(),
                            resource_count: resources.len(),
                        });
                    }
                }

                Ok(())
            }

            Self::Custom { parameters, .. } => {
                for parameter in parameters {
                    validate_parameter(parameter)?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Observable
// =============================================================================

/// One term in an analog observable.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservableTerm {
    operator: OperatorKind,
    targets: Vec<QubitId>,
    coefficient: Parameter,
}

impl ObservableTerm {
    /// Creates an observable term.
    pub fn new(
        operator: OperatorKind,
        targets: Vec<QubitId>,
        coefficient: Parameter,
    ) -> AnalogResult<Self> {
        if targets.is_empty() {
            return Err(AnalogError::EmptyOperatorTargets);
        }

        let mut seen = BTreeSet::new();

        for target in &targets {
            if !seen.insert(*target) {
                return Err(AnalogError::DuplicateOperatorTarget {
                    resource: *target,
                });
            }
        }

        validate_parameter(&coefficient)?;

        Ok(Self {
            operator,
            targets,
            coefficient,
        })
    }

    /// Returns operator.
    #[must_use]
    pub fn operator(&self) -> &OperatorKind {
        &self.operator
    }

    /// Returns targets.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &Parameter {
        &self.coefficient
    }
}

// =============================================================================
// Observable
// =============================================================================

/// A named observable requested from an analog workload.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogObservable {
    name: String,
    terms: Vec<ObservableTerm>,
}

impl AnalogObservable {
    /// Creates an observable.
    pub fn new<S: Into<String>>(
        name: S,
        terms: Vec<ObservableTerm>,
    ) -> AnalogResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "observable.name",
            });
        }

        if terms.is_empty() {
            return Err(AnalogError::EmptyObservable);
        }

        Ok(Self { name, terms })
    }

    /// Returns the observable name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns observable terms.
    #[must_use]
    pub fn terms(&self) -> &[ObservableTerm] {
        &self.terms
    }

    /// Validates referenced resources.
    pub fn validate(&self, resources: &[AnalogResource]) -> AnalogResult<()> {
        let resource_ids: BTreeSet<QubitId> =
            resources.iter().map(AnalogResource::id).collect();

        for term in &self.terms {
            validate_parameter(term.coefficient())?;

            for target in term.targets() {
                if !resource_ids.contains(target) {
                    return Err(AnalogError::ResourceOutOfBounds {
                        resource: target.index(),
                        resource_count: resources.len(),
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Analog program metadata
// =============================================================================

/// Deterministic semantic metadata attached to an analog program.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnalogMetadata {
    entries: BTreeMap<String, String>,
}

impl AnalogMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts metadata.
    pub fn insert<S: Into<String>>(
        &mut self,
        key: S,
        value: S,
    ) -> AnalogResult<()> {
        let key = key.into();

        if key.is_empty() {
            return Err(AnalogError::EmptyMetadataKey);
        }

        if self.entries.contains_key(&key) {
            return Err(AnalogError::DuplicateMetadataKey { key });
        }

        self.entries.insert(key, value.into());

        Ok(())
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns all metadata.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }
}

// =============================================================================
// Analog program
// =============================================================================

/// Canonical analog quantum program.
///
/// This is the principal public type of this module.
///
/// It represents an analog computation independently of a target machine.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogProgram {
    name: Option<String>,
    resources: Vec<AnalogResource>,
    controls: Vec<AnalogControl>,
    evolution: Vec<EvolutionWindow>,
    initial_state: InitialState,
    observables: Vec<AnalogObservable>,
    capabilities: BTreeSet<String>,
    metadata: AnalogMetadata,
}

impl AnalogProgram {
    /// Creates an empty analog program.
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            resources: Vec::new(),
            controls: Vec::new(),
            evolution: Vec::new(),
            initial_state: InitialState::Default,
            observables: Vec::new(),
            capabilities: BTreeSet::new(),
            metadata: AnalogMetadata::new(),
        }
    }

    /// Creates a named analog program.
    pub fn named<S: Into<String>>(name: S) -> AnalogResult<Self> {
        let mut program = Self::new();
        program.set_name(name)?;
        Ok(program)
    }

    /// Sets the semantic program name.
    pub fn set_name<S: Into<String>>(
        &mut self,
        name: S,
    ) -> AnalogResult<()> {
        let name = name.into();

        if name.is_empty() {
            return Err(AnalogError::EmptyString {
                field: "program.name",
            });
        }

        self.name = Some(name);

        Ok(())
    }

    /// Returns the optional program name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Adds a logical analog resource.
    pub fn add_resource(
        &mut self,
        resource: AnalogResource,
    ) -> AnalogResult<()> {
        if self
            .resources
            .iter()
            .any(|existing| existing.id() == resource.id())
        {
            return Err(AnalogError::DuplicateResource {
                id: resource.id(),
            });
        }

        self.resources.push(resource);

        Ok(())
    }

    /// Adds a resource using a canonical logical qubit ID.
    pub fn add_qubit(&mut self, qubit: QubitId) -> AnalogResult<()> {
        self.add_resource(AnalogResource::new(qubit))
    }

    /// Returns all logical analog resources.
    #[must_use]
    pub fn resources(&self) -> &[AnalogResource] {
        &self.resources
    }

    /// Returns the number of logical analog resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Adds a control field.
    pub fn add_control(&mut self, control: AnalogControl) {
        self.controls.push(control);
    }

    /// Returns analog controls.
    #[must_use]
    pub fn controls(&self) -> &[AnalogControl] {
        &self.controls
    }

    /// Adds an evolution window.
    pub fn add_evolution(
        &mut self,
        evolution: EvolutionWindow,
    ) -> AnalogResult<()> {
        self.evolution.push(evolution);

        Ok(())
    }

    /// Returns evolution windows.
    #[must_use]
    pub fn evolution(&self) -> &[EvolutionWindow] {
        &self.evolution
    }

    /// Sets the initial-state semantics.
    pub fn set_initial_state(
        &mut self,
        initial_state: InitialState,
    ) -> AnalogResult<()> {
        initial_state.validate(&self.resources)?;
        self.initial_state = initial_state;
        Ok(())
    }

    /// Returns the initial state.
    #[must_use]
    pub fn initial_state(&self) -> &InitialState {
        &self.initial_state
    }

    /// Adds an observable.
    pub fn add_observable(
        &mut self,
        observable: AnalogObservable,
    ) -> AnalogResult<()> {
        if self
            .observables
            .iter()
            .any(|existing| existing.name() == observable.name())
        {
            return Err(AnalogError::DuplicateObservable {
                name: observable.name().to_owned(),
            });
        }

        observable.validate(&self.resources)?;
        self.observables.push(observable);

        Ok(())
    }

    /// Returns observables.
    #[must_use]
    pub fn observables(&self) -> &[AnalogObservable] {
        &self.observables
    }

    /// Adds a provider-neutral capability requirement.
    ///
    /// Examples:
    ///
    /// ```text
    /// analog_control
    /// analog_hamiltonian_evolution
    /// spatial_control
    /// time_dependent_control
    /// local_control
    /// pair_interaction
    /// ```
    ///
    /// This file intentionally stores capability identifiers without importing
    /// a hardware implementation.
    pub fn require_capability<S: Into<String>>(
        &mut self,
        capability: S,
    ) -> AnalogResult<()> {
        let capability = capability.into();

        if capability.is_empty() {
            return Err(AnalogError::EmptyCapability);
        }

        self.capabilities.insert(capability);

        Ok(())
    }

    /// Returns required capabilities in deterministic order.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<String> {
        &self.capabilities
    }

    /// Adds metadata.
    pub fn insert_metadata<S: Into<String>>(
        &mut self,
        key: S,
        value: S,
    ) -> AnalogResult<()> {
        self.metadata.insert(key, value)
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &AnalogMetadata {
        &self.metadata
    }

    /// Returns the final semantic evolution time.
    ///
    /// This is not a scheduler result. It is derived from the semantic
    /// evolution windows already present in the program.
    #[must_use]
    pub fn end_time(&self) -> f64 {
        self.evolution
            .iter()
            .map(EvolutionWindow::end)
            .fold(0.0, f64::max)
    }

    /// Returns whether the program contains symbolic parameters.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        let initial_symbolic = match self.initial_state() {
            InitialState::Custom { parameters, .. } => {
                parameters.iter().any(Parameter::is_symbolic)
            }
            _ => false,
        };

        if initial_symbolic {
            return true;
        }

        for control in &self.controls {
            if control.profile().is_symbolic() {
                return true;
            }
        }

        for window in &self.evolution {
            for term in window.hamiltonian().terms() {
                if term.coefficient().is_symbolic() {
                    return true;
                }
            }
        }

        for observable in &self.observables {
            for term in observable.terms() {
                if term.coefficient().is_symbolic() {
                    return true;
                }
            }
        }

        false
    }

    /// Validates the complete semantic program.
    ///
    /// This validates semantic consistency but does not validate a particular
    /// hardware target.
    pub fn validate(&self) -> AnalogResult<()> {
        self.validate_resources()?;
        self.initial_state.validate(&self.resources)?;

        let resource_ids: BTreeSet<QubitId> =
            self.resources.iter().map(AnalogResource::id).collect();

        self.validate_controls(&resource_ids)?;
        self.validate_evolution(&resource_ids)?;
        self.validate_observables(&resource_ids)?;
        self.validate_spatial_consistency()?;

        Ok(())
    }

    fn validate_resources(&self) -> AnalogResult<()> {
        let mut ids = BTreeSet::new();

        for resource in &self.resources {
            if !ids.insert(resource.id()) {
                return Err(AnalogError::DuplicateResource {
                    id: resource.id(),
                });
            }
        }

        Ok(())
    }

    fn validate_controls(
        &self,
        resource_ids: &BTreeSet<QubitId>,
    ) -> AnalogResult<()> {
        for control in &self.controls {
            for sample in control.profile().samples() {
                validate_parameter(sample.value())?;
            }

            if let TargetSet::Explicit(targets) = control.targets() {
                if targets.is_empty() {
                    return Err(AnalogError::InvalidTargetSet);
                }

                for target in targets {
                    if !resource_ids.contains(target) {
                        return Err(AnalogError::ResourceOutOfBounds {
                            resource: target.index(),
                            resource_count: self.resources.len(),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_evolution(
        &self,
        resource_ids: &BTreeSet<QubitId>,
    ) -> AnalogResult<()> {
        for window in &self.evolution {
            if window.hamiltonian().is_empty() {
                return Err(AnalogError::EmptyField {
                    field: "evolution.hamiltonian",
                });
            }

            for term in window.hamiltonian().terms() {
                validate_parameter(term.coefficient())?;

                for target in term.targets() {
                    if !resource_ids.contains(target) {
                        return Err(AnalogError::ResourceOutOfBounds {
                            resource: target.index(),
                            resource_count: self.resources.len(),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_observables(
        &self,
        resource_ids: &BTreeSet<QubitId>,
    ) -> AnalogResult<()> {
        let mut names = BTreeSet::new();

        for observable in &self.observables {
            if !names.insert(observable.name()) {
                return Err(AnalogError::DuplicateObservable {
                    name: observable.name().to_owned(),
                });
            }

            if observable.terms().is_empty() {
                return Err(AnalogError::EmptyObservable);
            }

            for term in observable.terms() {
                validate_parameter(term.coefficient())?;

                for target in term.targets() {
                    if !resource_ids.contains(target) {
                        return Err(AnalogError::ResourceOutOfBounds {
                            resource: target.index(),
                            resource_count: self.resources.len(),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_spatial_consistency(&self) -> AnalogResult<()> {
        let positioned: Vec<&AnalogResource> = self
            .resources
            .iter()
            .filter(|resource| resource.position().is_some())
            .collect();

        if positioned.is_empty() {
            return Ok(());
        }

        let first_dimension = positioned[0]
            .position()
            .map(Position::dimension)
            .unwrap_or(0);

        for resource in &positioned {
            let Some(position) = resource.position() else {
                continue;
            };

            if position.dimension() != first_dimension {
                return Err(AnalogError::DimensionMismatch {
                    expected: first_dimension,
                    actual: position.dimension(),
                });
            }
        }

        for (index, left) in positioned.iter().enumerate() {
            for right in positioned.iter().skip(index + 1) {
                let Some(left_position) = left.position() else {
                    continue;
                };

                let Some(right_position) = right.position() else {
                    continue;
                };

                let distance =
                    left_position.squared_distance(right_position)?;

                if distance == 0.0 {
                    return Err(AnalogError::CoincidentResources {
                        first: left.id(),
                        second: right.id(),
                    });
                }
            }
        }

        Ok(())
    }
}

impl Default for AnalogProgram {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Capability inference
// =============================================================================

/// Provider-neutral capability identifiers inferred from an analog program.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnalogCapabilitySet {
    capabilities: BTreeSet<String>,
}

impl AnalogCapabilitySet {
    /// Creates an empty capability set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            capabilities: BTreeSet::new(),
        }
    }

    /// Adds a capability.
    pub fn insert<S: Into<String>>(
        &mut self,
        capability: S,
    ) -> AnalogResult<()> {
        let capability = capability.into();

        if capability.is_empty() {
            return Err(AnalogError::EmptyCapability);
        }

        self.capabilities.insert(capability);

        Ok(())
    }

    /// Returns whether a capability is required.
    #[must_use]
    pub fn contains(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }

    /// Returns all capabilities.
    #[must_use]
    pub fn as_set(&self) -> &BTreeSet<String> {
        &self.capabilities
    }
}

impl Default for AnalogCapabilitySet {
    fn default() -> Self {
        Self::new()
    }
}

/// Infers provider-neutral capabilities required by an analog program.
///
/// This function does not inspect hardware and therefore remains entirely
/// target-independent.
pub fn infer_capabilities(
    program: &AnalogProgram,
) -> AnalogResult<AnalogCapabilitySet> {
    program.validate()?;

    let mut result = AnalogCapabilitySet::new();

    result.insert("analog_control")?;
    result.insert("analog_hamiltonian_evolution")?;

    if program
        .resources()
        .iter()
        .any(|resource| resource.position().is_some())
    {
        result.insert("spatial_control")?;
    }

    if program.controls().iter().any(|control| {
        control.profile().samples().len() > 1
            || control.profile().is_symbolic()
    }) {
        result.insert("time_dependent_control")?;
    }

    if program.controls().iter().any(|control| {
        matches!(control.targets(), TargetSet::Explicit(_))
    }) {
        result.insert("local_control")?;
    }

    if program.evolution().iter().any(|window| {
        window
            .hamiltonian()
            .terms()
            .iter()
            .any(|term| term.targets().len() >= 2)
    }) {
        result.insert("multi_resource_interaction")?;
    }

    if program.evolution().iter().any(|window| {
        window
            .hamiltonian()
            .terms()
            .iter()
            .any(|term| term.control().is_some())
    }) {
        result.insert("time_dependent_hamiltonian")?;
    }

    Ok(result)
}

// =============================================================================
// Resource analysis
// =============================================================================

/// Target-independent resource information derivable from an analog program.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalogResourceEstimate {
    /// Number of logical resources.
    pub logical_resources: usize,

    /// Number of Hamiltonian terms.
    pub hamiltonian_terms: usize,

    /// Number of control fields.
    pub controls: usize,

    /// Number of evolution windows.
    pub evolution_windows: usize,

    /// Number of observables.
    pub observables: usize,

    /// Semantic end time.
    pub end_time: f64,
}

impl AnalogProgram {
    /// Produces a target-independent structural resource estimate.
    ///
    /// This is an observation of the current IR, not a hardware resource
    /// estimate.
    pub fn structural_estimate(
        &self,
    ) -> AnalogResourceEstimate {
        let hamiltonian_terms = self
            .evolution()
            .iter()
            .map(|window| window.hamiltonian().term_count())
            .sum();

        AnalogResourceEstimate {
            logical_resources: self.resource_count(),
            hamiltonian_terms,
            controls: self.controls().len(),
            evolution_windows: self.evolution().len(),
            observables: self.observables().len(),
            end_time: self.end_time(),
        }
    }
}

// =============================================================================
// Constructors for common analog terms
// =============================================================================

/// Creates a single-resource Pauli-X Hamiltonian term.
pub fn pauli_x(
    qubit: QubitId,
    coefficient: Parameter,
) -> AnalogResult<HamiltonianTerm> {
    HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliX),
        vec![qubit],
        coefficient,
    )
}

/// Creates a single-resource Pauli-Y Hamiltonian term.
pub fn pauli_y(
    qubit: QubitId,
    coefficient: Parameter,
) -> AnalogResult<HamiltonianTerm> {
    HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliY),
        vec![qubit],
        coefficient,
    )
}

/// Creates a single-resource Pauli-Z Hamiltonian term.
pub fn pauli_z(
    qubit: QubitId,
    coefficient: Parameter,
) -> AnalogResult<HamiltonianTerm> {
    HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliZ),
        vec![qubit],
        coefficient,
    )
}

/// Creates a two-resource Pauli-Z interaction term.
pub fn pauli_zz(
    first: QubitId,
    second: QubitId,
    coefficient: Parameter,
) -> AnalogResult<HamiltonianTerm> {
    if first == second {
        return Err(AnalogError::SelfInteraction {
            resource: first.index(),
        });
    }

    HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliZ),
        vec![first, second],
        coefficient,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_rejects_empty_coordinates() {
        let result = Position::new(Vec::new());

        assert!(matches!(
            result,
            Err(AnalogError::EmptyPosition)
        ));
    }

    #[test]
    fn position_rejects_non_finite_values() {
        let result = Position::one_dimensional(f64::NAN);

        assert!(matches!(
            result,
            Err(AnalogError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn resource_uses_canonical_qubit_id() {
        let qubit = QubitId::new(7);
        let resource = AnalogResource::new(qubit);

        assert_eq!(resource.id(), qubit);
    }

    #[test]
    fn target_set_rejects_duplicates() {
        let result = TargetSet::explicit(vec![
            QubitId::new(0),
            QubitId::new(0),
        ]);

        assert!(matches!(
            result,
            Err(AnalogError::DuplicateOperatorTarget { .. })
        ));
    }

    #[test]
    fn control_profile_rejects_reverse_time() {
        let first =
            TimeSample::new(2.0, Parameter::Constant(1.0)).unwrap();

        let second =
            TimeSample::new(1.0, Parameter::Constant(2.0)).unwrap();

        let result = ControlProfile::new(
            vec![first, second],
            Interpolation::PiecewiseLinear,
        );

        assert!(matches!(
            result,
            Err(AnalogError::NonMonotonicTime { .. })
        ));
    }

    #[test]
    fn hamiltonian_term_rejects_duplicate_targets() {
        let result = HamiltonianTerm::new(
            OperatorKind::Standard(StandardOperator::PauliZ),
            vec![QubitId::new(1), QubitId::new(1)],
            Parameter::Constant(1.0),
        );

        assert!(matches!(
            result,
            Err(AnalogError::DuplicateOperatorTarget { .. })
        ));
    }

    #[test]
    fn evolution_requires_hamiltonian() {
        let hamiltonian = AnalogHamiltonian::new();

        let result =
            EvolutionWindow::from_duration(1.0, hamiltonian);

        assert!(matches!(
            result,
            Err(AnalogError::EmptyField { .. })
        ));
    }

    #[test]
    fn program_can_scale_without_architectural_qubit_constant() {
        let mut program = AnalogProgram::new();

        for index in 0..10_000usize {
            program
                .add_qubit(QubitId::new(index))
                .unwrap();
        }

        assert_eq!(program.resource_count(), 10_000);
        assert!(program.validate().is_ok());
    }

    #[test]
    fn program_rejects_unknown_hamiltonian_resource() {
        let mut program = AnalogProgram::new();

        program.add_qubit(QubitId::new(0)).unwrap();

        let term = pauli_z(
            QubitId::new(1),
            Parameter::Constant(1.0),
        )
        .unwrap();

        let hamiltonian =
            AnalogHamiltonian::from_terms(vec![term]);

        let evolution =
            EvolutionWindow::from_duration(1.0, hamiltonian)
                .unwrap();

        program.add_evolution(evolution).unwrap();

        assert!(matches!(
            program.validate(),
            Err(AnalogError::ResourceOutOfBounds { .. })
        ));
    }

    #[test]
    fn program_accepts_symbolic_control() {
        let sample = TimeSample::new(
            0.0,
            Parameter::Symbol("omega".to_owned()),
        )
        .unwrap();

        let profile = ControlProfile::new(
            vec![sample],
            Interpolation::PiecewiseConstant,
        )
        .unwrap();

        let control = AnalogControl::new(
            "drive",
            TargetSet::All,
            profile,
        )
        .unwrap();

        let mut program = AnalogProgram::new();
        program.add_qubit(QubitId::new(0)).unwrap();
        program.add_control(control);

        assert!(program.is_symbolic());
        assert!(program.validate().is_ok());
    }

    #[test]
    fn capabilities_are_inferred_without_hardware_dependency() {
        let mut program = AnalogProgram::new();

        program.add_qubit(QubitId::new(0)).unwrap();
        program.add_qubit(QubitId::new(1)).unwrap();

        let term = pauli_zz(
            QubitId::new(0),
            QubitId::new(1),
            Parameter::Constant(1.0),
        )
        .unwrap();

        let hamiltonian =
            AnalogHamiltonian::from_terms(vec![term]);

        let evolution =
            EvolutionWindow::from_duration(1.0, hamiltonian)
                .unwrap();

        program.add_evolution(evolution).unwrap();

        let capabilities =
            infer_capabilities(&program).unwrap();

        assert!(
            capabilities.contains("analog_control")
        );

        assert!(
            capabilities.contains(
                "analog_hamiltonian_evolution"
            )
        );

        assert!(
            capabilities.contains(
                "multi_resource_interaction"
            )
        );
    }

    #[test]
    fn spatial_resources_are_validated() {
        let mut program = AnalogProgram::new();

        program
            .add_resource(
                AnalogResource::with_position(
                    QubitId::new(0),
                    Position::two_dimensional(0.0, 0.0)
                        .unwrap(),
                ),
            )
            .unwrap();

        program
            .add_resource(
                AnalogResource::with_position(
                    QubitId::new(1),
                    Position::two_dimensional(1.0, 0.0)
                        .unwrap(),
                ),
            )
            .unwrap();

        assert!(program.validate().is_ok());
    }

    #[test]
    fn coincident_spatial_resources_are_rejected() {
        let mut program = AnalogProgram::new();

        program
            .add_resource(
                AnalogResource::with_position(
                    QubitId::new(0),
                    Position::two_dimensional(0.0, 0.0)
                        .unwrap(),
                ),
            )
            .unwrap();

        program
            .add_resource(
                AnalogResource::with_position(
                    QubitId::new(1),
                    Position::two_dimensional(0.0, 0.0)
                        .unwrap(),
                ),
            )
            .unwrap();

        assert!(matches!(
            program.validate(),
            Err(AnalogError::CoincidentResources { .. })
        ));
    }

    #[test]
    fn structural_estimate_is_target_independent() {
        let mut program = AnalogProgram::new();

        program.add_qubit(QubitId::new(0)).unwrap();
        program.add_qubit(QubitId::new(1)).unwrap();

        let term = pauli_zz(
            QubitId::new(0),
            QubitId::new(1),
            Parameter::Constant(1.0),
        )
        .unwrap();

        let hamiltonian =
            AnalogHamiltonian::from_terms(vec![term]);

        program
            .add_evolution(
                EvolutionWindow::from_duration(
                    10.0,
                    hamiltonian,
                )
                .unwrap(),
            )
            .unwrap();

        let estimate = program.structural_estimate();

        assert_eq!(estimate.logical_resources, 2);
        assert_eq!(estimate.hamiltonian_terms, 1);
        assert_eq!(estimate.evolution_windows, 1);
        assert_eq!(estimate.end_time, 10.0);
    }
}