//! Zamani Quantum Noise (ZQN) — Measurement Calibration
//!
//! Path:
//!
//!     src/quantum/zqn/calibration/measurement.rs
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module defines backend-independent calibration semantics for quantum
//! measurement operations.
//!
//! A measurement calibration answers:
//
//! > "Given a measurement resource and measurement configuration, what
//! > calibrated information describes its preparation-to-observation mapping,
//! > uncertainty, validity, provenance, and operational applicability?"
//!
//! This module is intentionally independent of:
//!
//! - hardware vendors;
//! - hardware SDKs;
//! - QPU transport;
//! - credentials;
//! - source-language syntax;
//! - the canonical quantum IR implementation;
//! - simulator implementations;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - benchmarking methodology;
//! - concrete serialization formats.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! ```text
//!                    quantum::ir
//!                         │
//!                         │ canonical operation/resource identity
//!                         ▼
//!               calibration::parameter
//!                         │
//!                         │ calibrated values
//!                         ▼
//!             calibration::measurement
//!                         │
//!              ┌──────────┼──────────┐
//!              │          │          │
//!              ▼          ▼          ▼
//!           snapshot     noise      hardware
//!              │          │          │
//!              └──────────┼──────────┘
//!                         ▼
//!                    simulation
//!                         │
//!                         ▼
//!                        QEC
//! ```
//!
//! The parameter layer owns individual calibrated values. This module owns
//! measurement-specific interpretation and relationships between those
//! parameters.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - measurement-calibration identity;
//! - measurement semantic description;
//! - measurement resource scope;
//! - measurement parameter bindings;
//! - outcome-domain descriptions;
//! - calibrated transition references;
//! - measurement validity metadata;
//! - measurement calibration revision;
//! - measurement calibration status;
//! - deterministic canonical ordering;
//! - local measurement-calibration validation;
//! - resource-policy validation;
//! - applicability checks;
//! - immutable measurement-calibration values.
//!
//! ============================================================================
//! DOES NOT OWN
//! ============================================================================
//!
//! This file does NOT own:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - quantum operation identity;
//! - calibration parameter values;
//! - probability mathematics;
//! - quantum-channel mathematics;
//! - statistical estimation;
//! - tomography algorithms;
//! - calibration experiments;
//! - calibration snapshot registries;
//! - hardware discovery;
//! - hardware execution;
//! - readout device drivers;
//! - noise simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking;
//! - serialization formats;
//! - global mutable state.
//!
//! Canonical quantum resource identities remain owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! Individual calibrated parameter values remain owned by:
//!
//!     crate::quantum::zqn::calibration::parameter
//!
//! ============================================================================
//! DESIGN PRINCIPLE
//! ============================================================================
//!
//! Measurement calibration is deliberately more general than binary qubit
//! readout.
//!
//! It can represent:
//!
//! - binary measurements;
//! - multi-outcome measurements;
//! - qudit measurements;
//! - parity measurements;
//! - joint measurements;
//! - generalized measurements;
//! - POVM-related calibration data;
//! - basis-dependent measurements;
//! - state-dependent readout;
//! - correlated readout;
//! - asymmetric assignment errors;
//! - measurement backaction metadata;
//! - threshold-based measurements;
//! - continuous-valued detector responses;
//! - future measurement technologies.
//!
//! No assumption is made that:
//!
//!     outcomes == 2
//!
//! or:
//!
//!     arity == 1
//!
//! or:
//!
//!     measurement == computational-basis qubit readout
//!
//! ============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! ============================================================================
//!
//! There is no semantic machine-size ceiling in this module.
//!
//! In particular, this file does NOT define:
//!
//!     MAX_QUBITS
//!     MAX_MEASUREMENTS
//!     MAX_OUTCOMES
//!     MAX_PARAMETERS
//!     MAX_TRANSITIONS
//!     MAX_ARITY
//!
//! A measurement calibration may therefore describe an arbitrary number of
//! resources and outcomes, subject only to the selected host representation
//! and explicit caller-controlled resource policies.
//!
//! `None` in a resource limit means that this module does not impose a limit.
//!
//! This is deliberately different from claiming that physical hardware,
//! memory, storage, or execution time are infinite.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! This module is deterministic.
//!
//! It:
//!
//! - does not generate random numbers;
//! - does not read a clock;
//! - does not access process IDs;
//! - does not access thread IDs;
//! - does not inspect memory addresses;
//! - does not use global mutable state;
//! - does not perform implicit I/O;
//! - does not depend on hash-map iteration order.
//!
//! Stochastic measurement behavior belongs to the simulation/noise execution
//! layer and must use an explicit reproducibility context.
//!
//! ============================================================================
//! IMMUTABILITY
//! ============================================================================
//!
//! `MeasurementCalibration` is an immutable value object after construction.
//!
//! Methods that conceptually modify calibration state return a new value.
//!
//! This guarantees that a calibration object already used by execution cannot
//! silently change underneath that execution.
//!
//! ============================================================================
//! NUMERICAL SAFETY
//! ============================================================================
//!
//! Numerical quantities are accepted only when finite.
//!
//! This module rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - negative probabilities;
//! - probabilities greater than one;
//! - invalid probability sums;
//! - malformed transition definitions.
//!
//! This module does not silently repair invalid numerical data.
//!
//! ============================================================================
//! ERROR CONTRACT
//! ============================================================================
//!
//! ZQN-wide errors are represented by:
//!
//!     crate::quantum::zqn::core::errors::ZqnError
//!     crate::quantum::zqn::core::errors::ZqnResult
//!
//! This file does not introduce a competing system-wide error hierarchy.
//!
//! ============================================================================
//! SERIALIZATION CONTRACT
//! ============================================================================
//!
//! This module does not define a wire format.
//!
//! Serialization belongs to:
//!
//!     crate::quantum::zqn::io
//!
//! A serializer MUST preserve:
//!
//! - calibration identity;
//! - revision;
//! - measurement identity;
//! - measurement scope;
//! - outcome definitions;
//! - parameter bindings;
//! - transition definitions;
//! - validity;
//! - status;
//! - provenance;
//! - approximation/quality semantics.
//!
//! Rust struct layout is not a serialization contract.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! Measurement calibration is data.
//!
//! It must never contain:
//!
//! - credentials;
//! - API keys;
//! - executable code;
//! - network handles;
//! - file handles;
//! - provider SDK objects;
//! - implicit filesystem paths with execution semantics.
//!
//! Structured identifiers are opaque references and grant no authority.
//!
//! ============================================================================
//! THREAD SAFETY
//! ============================================================================
//!
//! The types in this module contain ordinary owned values and references by
//! value. They do not contain global mutable state or synchronization
//! primitives.
//!
//! `MeasurementCalibration` is therefore suitable for concurrent use through
//! ordinary ownership/borrowing rules and can be shared across threads when
//! its contained canonical IDs satisfy the repository's thread-safety
//! contracts.
//!
//! ============================================================================
//! RUST COMPATIBILITY
//! ============================================================================
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::calibration::parameter::CalibrationScope;
use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::{CalibrationId, NoiseParameterId};

// ============================================================================
// VERSION
// ============================================================================

/// Semantic revision of the measurement-calibration representation.
///
/// This is a representation/schema revision, not a machine-size limit.
pub const MEASUREMENT_CALIBRATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// CALIBRATION STATUS
// ============================================================================

/// Operational status of a measurement calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementCalibrationStatus {
    /// Calibration has passed validation and may be used.
    Valid,

    /// Calibration exists but has not completed validation.
    Unvalidated,

    /// Calibration is outside its declared validity period or otherwise known
    /// to be stale.
    Stale,

    /// Calibration is known to be invalid.
    Invalid,

    /// Calibration is intentionally disabled.
    Disabled,

    /// Calibration has been replaced by another revision.
    Superseded,
}

impl Default for MeasurementCalibrationStatus {
    fn default() -> Self {
        Self::Unvalidated
    }
}

impl fmt::Display for MeasurementCalibrationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => formatter.write_str("valid"),
            Self::Unvalidated => formatter.write_str("unvalidated"),
            Self::Stale => formatter.write_str("stale"),
            Self::Invalid => formatter.write_str("invalid"),
            Self::Disabled => formatter.write_str("disabled"),
            Self::Superseded => formatter.write_str("superseded"),
        }
    }
}

// ============================================================================
// REVISION
// ============================================================================

/// Semantic revision of a measurement calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementCalibrationRevision {
    major: u32,
    minor: u32,
    patch: u32,
}

impl MeasurementCalibrationRevision {
    /// Creates an explicit semantic revision.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major revision.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor revision.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch revision.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl Default for MeasurementCalibrationRevision {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for MeasurementCalibrationRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// VALIDITY
// ============================================================================

/// Explicit validity interval for a measurement calibration.
///
/// Timestamps are represented as signed nanoseconds relative to the canonical
/// time origin chosen by the owning runtime.
///
/// The interval is half-open:
///
///     [not_before_ns, not_after_ns)
///
/// `None` means unbounded on that side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementCalibrationValidity {
    not_before_ns: Option<i128>,
    not_after_ns: Option<i128>,
}

impl MeasurementCalibrationValidity {
    /// Creates a validity interval.
    ///
    /// Returns an error if both bounds are present and the end is not strictly
    /// after the beginning.
    pub fn new(
        not_before_ns: Option<i128>,
        not_after_ns: Option<i128>,
    ) -> ZqnResult<Self> {
        if let (Some(start), Some(end)) = (not_before_ns, not_after_ns) {
            if start >= end {
                return Err(ZqnError::invalid_calibration(
                    "measurement calibration validity interval must have \
                     end strictly greater than start",
                ));
            }
        }

        Ok(Self {
            not_before_ns,
            not_after_ns,
        })
    }

    /// Creates an unbounded validity interval.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            not_before_ns: None,
            not_after_ns: None,
        }
    }

    /// Returns the lower validity bound.
    #[must_use]
    pub const fn not_before_ns(&self) -> Option<i128> {
        self.not_before_ns
    }

    /// Returns the upper validity bound.
    #[must_use]
    pub const fn not_after_ns(&self) -> Option<i128> {
        self.not_after_ns
    }

    /// Returns whether the supplied timestamp belongs to this interval.
    #[must_use]
    pub fn contains(&self, timestamp_ns: i128) -> bool {
        if let Some(start) = self.not_before_ns {
            if timestamp_ns < start {
                return false;
            }
        }

        if let Some(end) = self.not_after_ns {
            if timestamp_ns >= end {
                return false;
            }
        }

        true
    }
}

impl Default for MeasurementCalibrationValidity {
    fn default() -> Self {
        Self::unbounded()
    }
}

// ============================================================================
// MEASUREMENT IDENTITY
// ============================================================================

/// Backend-independent identity of the calibrated measurement semantic.
///
/// Names are descriptive identifiers and must not be interpreted as vendor
/// API calls.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementIdentity {
    /// Stable semantic name.
    name: String,

    /// Optional semantic variant, such as a basis or measurement mode.
    ///
    /// The value is intentionally opaque and is not interpreted by this file.
    variant: Option<String>,
}

impl MeasurementIdentity {
    /// Creates a measurement identity.
    pub fn new(name: impl Into<String>) -> ZqnResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement calibration identity name must not be empty",
            ));
        }

        Ok(Self {
            name,
            variant: None,
        })
    }

    /// Creates an identity with an explicit variant.
    pub fn with_variant(
        name: impl Into<String>,
        variant: impl Into<String>,
    ) -> ZqnResult<Self> {
        let name = name.into();
        let variant = variant.into();

        if name.trim().is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement calibration identity name must not be empty",
            ));
        }

        if variant.trim().is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement calibration identity variant must not be empty",
            ));
        }

        Ok(Self {
            name,
            variant: Some(variant),
        })
    }

    /// Returns the semantic measurement name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional semantic variant.
    #[must_use]
    pub fn variant(&self) -> Option<&str> {
        self.variant.as_deref()
    }

    /// Validates the identity.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.name.trim().is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement calibration identity name must not be empty",
            ));
        }

        if let Some(variant) = self.variant.as_deref() {
            if variant.trim().is_empty() {
                return Err(ZqnError::invalid_calibration(
                    "measurement calibration identity variant must not be empty",
                ));
            }
        }

        Ok(())
    }
}

impl fmt::Display for MeasurementIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)?;

        if let Some(variant) = self.variant.as_deref() {
            write!(formatter, "::{}", variant)?;
        }

        Ok(())
    }
}

// ============================================================================
// OUTCOME DOMAIN
// ============================================================================

/// Semantic description of a measurement outcome.
///
/// An outcome is intentionally represented by an opaque stable label rather
/// than an integer index. This permits arbitrary and future measurement
/// alphabets.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementOutcome {
    label: String,
}

impl MeasurementOutcome {
    /// Creates an outcome label.
    pub fn new(label: impl Into<String>) -> ZqnResult<Self> {
        let label = label.into();

        if label.trim().is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement outcome label must not be empty",
            ));
        }

        Ok(Self { label })
    }

    /// Returns the outcome label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }
}

impl fmt::Display for MeasurementOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

/// An explicitly declared measurement outcome domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementOutcomeDomain {
    outcomes: Vec<MeasurementOutcome>,
}

impl MeasurementOutcomeDomain {
    /// Creates an outcome domain.
    ///
    /// At least one outcome is required. Duplicate outcomes are rejected.
    pub fn new(outcomes: Vec<MeasurementOutcome>) -> ZqnResult<Self> {
        if outcomes.is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement outcome domain must contain at least one outcome",
            ));
        }

        let mut canonical = outcomes;

        canonical.sort();
        canonical.dedup();

        Ok(Self {
            outcomes: canonical,
        })
    }

    /// Returns all outcomes in deterministic order.
    #[must_use]
    pub fn outcomes(&self) -> &[MeasurementOutcome] {
        &self.outcomes
    }

    /// Returns the number of outcomes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    /// Returns whether the domain contains no outcomes.
    ///
    /// This can never be true for a successfully constructed domain, but the
    /// method is useful for generic collection APIs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    /// Tests whether an outcome belongs to this domain.
    #[must_use]
    pub fn contains(&self, outcome: &MeasurementOutcome) -> bool {
        self.outcomes.binary_search(outcome).is_ok()
    }

    /// Validates the domain.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.outcomes.is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement outcome domain must contain at least one outcome",
            ));
        }

        for outcome in &self.outcomes {
            if outcome.label.trim().is_empty() {
                return Err(ZqnError::invalid_calibration(
                    "measurement outcome label must not be empty",
                ));
            }
        }

        for pair in self.outcomes.windows(2) {
            if pair[0] >= pair[1] {
                return Err(ZqnError::invalid_calibration(
                    "measurement outcome domain must be strictly canonicalized",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// RESOURCE ARITY
// ============================================================================

/// Measurement resource arity.
///
/// `None` means that the arity is determined by the bound calibration scope or
/// target context rather than explicitly declared here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementArity(Option<usize>);

impl MeasurementArity {
    /// Creates an explicitly declared arity.
    #[must_use]
    pub const fn exact(value: usize) -> Self {
        Self(Some(value))
    }

    /// Creates an unspecified arity.
    #[must_use]
    pub const fn inferred() -> Self {
        Self(None)
    }

    /// Returns the explicit arity, when present.
    #[must_use]
    pub const fn value(self) -> Option<usize> {
        self.0
    }

    /// Validates the arity.
    ///
    /// Zero is rejected when an explicit arity is supplied.
    pub fn validate(self) -> ZqnResult<()> {
        if let Some(value) = self.0 {
            if value == 0 {
                return Err(ZqnError::invalid_calibration(
                    "explicit measurement arity must be greater than zero",
                ));
            }
        }

        Ok(())
    }
}

impl Default for MeasurementArity {
    fn default() -> Self {
        Self::inferred()
    }
}

// ============================================================================
// PARAMETER ROLE
// ============================================================================

/// Semantic role of a calibration parameter within a measurement calibration.
///
/// Roles are opaque strings rather than a fixed enum so new measurement
/// technologies can introduce new calibrated quantities without changing this
/// file.
///
/// Examples include:
///
/// - assignment probability;
/// - threshold;
/// - response coefficient;
/// - gain;
/// - offset;
/// - backaction parameter;
/// - detector efficiency;
/// - state-dependent response.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementParameterRole(String);

impl MeasurementParameterRole {
    /// Creates a parameter role.
    pub fn new(value: impl Into<String>) -> ZqnResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement calibration parameter role must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the role name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MeasurementParameterRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PARAMETER BINDING
// ============================================================================

/// Associates a semantic measurement role with a canonical ZQN calibration
/// parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementParameterBinding {
    role: MeasurementParameterRole,
    parameter_id: NoiseParameterId,
}

impl MeasurementParameterBinding {
    /// Creates a parameter binding.
    #[must_use]
    pub const fn new(
        role: MeasurementParameterRole,
        parameter_id: NoiseParameterId,
    ) -> Self {
        Self {
            role,
            parameter_id,
        }
    }

    /// Returns the semantic role.
    #[must_use]
    pub fn role(&self) -> &MeasurementParameterRole {
        &self.role
    }

    /// Returns the referenced parameter identity.
    #[must_use]
    pub const fn parameter_id(&self) -> NoiseParameterId {
        self.parameter_id
    }
}

// ============================================================================
// TRANSITION
// ============================================================================

/// Calibrated relationship between an actual/prepared outcome and an observed
/// outcome.
///
/// The numerical probability is deliberately stored as a reference to a
/// `CalibrationParameter` rather than duplicated here.
///
/// This gives the parameter layer one authoritative owner for:
///
/// - value;
//! - unit;
//! - uncertainty;
//! - validity;
//! - provenance.
//!
//! A transition therefore contains semantic structure while the parameter
//! subsystem contains the calibrated numerical value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementTransition {
    actual: MeasurementOutcome,
    observed: MeasurementOutcome,
    probability_parameter: NoiseParameterId,
}

impl MeasurementTransition {
    /// Creates a calibrated transition reference.
    #[must_use]
    pub const fn new(
        actual: MeasurementOutcome,
        observed: MeasurementOutcome,
        probability_parameter: NoiseParameterId,
    ) -> Self {
        Self {
            actual,
            observed,
            probability_parameter,
        }
    }

    /// Returns the actual/prepared outcome.
    #[must_use]
    pub fn actual(&self) -> &MeasurementOutcome {
        &self.actual
    }

    /// Returns the observed outcome.
    #[must_use]
    pub fn observed(&self) -> &MeasurementOutcome {
        &self.observed
    }

    /// Returns the calibration parameter containing the probability.
    #[must_use]
    pub const fn probability_parameter(&self) -> NoiseParameterId {
        self.probability_parameter
    }
}

// ============================================================================
// BACKACTION
// ============================================================================

/// Optional reference to calibrated measurement backaction.
///
/// The backaction model itself belongs to the channel/noise subsystem. This
/// structure only identifies the calibrated parameter describing it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementBackaction {
    parameter_id: NoiseParameterId,
}

impl MeasurementBackaction {
    /// Creates a backaction parameter reference.
    #[must_use]
    pub const fn new(parameter_id: NoiseParameterId) -> Self {
        Self { parameter_id }
    }

    /// Returns the referenced parameter.
    #[must_use]
    pub const fn parameter_id(&self) -> NoiseParameterId {
        self.parameter_id
    }
}

// ============================================================================
// MEASUREMENT CALIBRATION LIMITS
// ============================================================================

/// Explicit caller-controlled resource policy for measurement calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementCalibrationLimits {
    /// Maximum number of declared outcomes.
    pub max_outcomes: Option<u64>,

    /// Maximum number of parameter bindings.
    pub max_parameter_bindings: Option<u64>,

    /// Maximum number of calibrated transitions.
    pub max_transitions: Option<u64>,

    /// Maximum resource arity.
    pub max_arity: Option<u64>,
}

impl Default for MeasurementCalibrationLimits {
    fn default() -> Self {
        Self {
            max_outcomes: None,
            max_parameter_bindings: None,
            max_transitions: None,
            max_arity: None,
        }
    }
}

impl MeasurementCalibrationLimits {
    /// Validates the calibration against this explicit resource policy.
    pub fn validate(
        &self,
        calibration: &MeasurementCalibration,
    ) -> ZqnResult<()> {
        if let Some(limit) = self.max_outcomes {
            if calibration.outcome_domain.len() as u128 > u128::from(limit) {
                return Err(
                    ZqnError::resource_limit_exceeded(
                        "measurement calibration outcome count exceeds \
                         configured limit",
                    ),
                );
            }
        }

        if let Some(limit) = self.max_parameter_bindings {
            if calibration.parameter_bindings.len() as u128
                > u128::from(limit)
            {
                return Err(
                    ZqnError::resource_limit_exceeded(
                        "measurement calibration parameter-binding count \
                         exceeds configured limit",
                    ),
                );
            }
        }

        if let Some(limit) = self.max_transitions {
            if calibration.transitions.len() as u128 > u128::from(limit) {
                return Err(
                    ZqnError::resource_limit_exceeded(
                        "measurement calibration transition count exceeds \
                         configured limit",
                    ),
                );
            }
        }

        if let Some(max_arity) = self.max_arity {
            if let Some(arity) = calibration.arity.value() {
                if arity as u128 > u128::from(max_arity) {
                    return Err(
                        ZqnError::resource_limit_exceeded(
                            "measurement calibration arity exceeds configured \
                             limit",
                        ),
                    );
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// MEASUREMENT CALIBRATION
// ============================================================================

/// Immutable backend-independent measurement calibration.
///
/// The type contains semantic references to calibrated parameters rather than
/// duplicating parameter values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementCalibration {
    calibration_id: CalibrationId,
    identity: MeasurementIdentity,
    scope: CalibrationScope,
    arity: MeasurementArity,
    outcome_domain: MeasurementOutcomeDomain,
    parameter_bindings: Vec<MeasurementParameterBinding>,
    transitions: Vec<MeasurementTransition>,
    backaction: Option<MeasurementBackaction>,
    validity: MeasurementCalibrationValidity,
    revision: MeasurementCalibrationRevision,
    status: MeasurementCalibrationStatus,
    provenance: Option<CalibrationId>,
    description: Option<String>,
}

impl MeasurementCalibration {
    /// Creates a measurement calibration.
    ///
    /// The constructor establishes all local structural invariants.
    ///
    /// It does not verify that referenced parameter IDs actually exist. That
    /// lookup belongs to the calibration snapshot/registry layer.
    pub fn new(
        calibration_id: CalibrationId,
        identity: MeasurementIdentity,
        scope: CalibrationScope,
        outcome_domain: MeasurementOutcomeDomain,
    ) -> ZqnResult<Self> {
        identity.validate()?;
        scope
            .validate()
            .map_err(|error| {
                ZqnError::invalid_calibration(error.to_string())
            })?;
        outcome_domain.validate()?;

        Ok(Self {
            calibration_id,
            identity,
            scope,
            arity: MeasurementArity::inferred(),
            outcome_domain,
            parameter_bindings: Vec::new(),
            transitions: Vec::new(),
            backaction: None,
            validity: MeasurementCalibrationValidity::unbounded(),
            revision: MeasurementCalibrationRevision::default(),
            status: MeasurementCalibrationStatus::Unvalidated,
            provenance: None,
            description: None,
        })
    }

    /// Returns the calibration identity.
    #[must_use]
    pub const fn calibration_id(&self) -> CalibrationId {
        self.calibration_id
    }

    /// Returns the measurement semantic identity.
    #[must_use]
    pub fn identity(&self) -> &MeasurementIdentity {
        &self.identity
    }

    /// Returns the measurement scope.
    #[must_use]
    pub fn scope(&self) -> &CalibrationScope {
        &self.scope
    }

    /// Returns the declared measurement arity.
    #[must_use]
    pub const fn arity(&self) -> MeasurementArity {
        self.arity
    }

    /// Returns the outcome domain.
    #[must_use]
    pub fn outcome_domain(&self) -> &MeasurementOutcomeDomain {
        &self.outcome_domain
    }

    /// Returns all parameter bindings in canonical order.
    #[must_use]
    pub fn parameter_bindings(&self) -> &[MeasurementParameterBinding] {
        &self.parameter_bindings
    }

    /// Returns all transition references in canonical order.
    #[must_use]
    pub fn transitions(&self) -> &[MeasurementTransition] {
        &self.transitions
    }

    /// Returns optional measurement backaction information.
    #[must_use]
    pub fn backaction(&self) -> Option<&MeasurementBackaction> {
        self.backaction.as_ref()
    }

    /// Returns the validity interval.
    #[must_use]
    pub const fn validity(&self) -> MeasurementCalibrationValidity {
        self.validity
    }

    /// Returns the semantic revision.
    #[must_use]
    pub const fn revision(&self) -> MeasurementCalibrationRevision {
        self.revision
    }

    /// Returns the lifecycle status.
    #[must_use]
    pub const fn status(&self) -> MeasurementCalibrationStatus {
        self.status
    }

    /// Returns optional calibration lineage.
    #[must_use]
    pub const fn provenance(&self) -> Option<CalibrationId> {
        self.provenance
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns a copy with an explicit arity.
    pub fn with_arity(&self, arity: usize) -> ZqnResult<Self> {
        let arity = MeasurementArity::exact(arity);
        arity.validate()?;

        let mut result = self.clone();
        result.arity = arity;
        result.validate()?;

        Ok(result)
    }

    /// Returns a copy with a validity interval.
    pub fn with_validity(
        &self,
        validity: MeasurementCalibrationValidity,
    ) -> ZqnResult<Self> {
        let mut result = self.clone();
        result.validity = validity;
        result.validate()?;

        Ok(result)
    }

    /// Returns a copy with a semantic revision.
    #[must_use]
    pub fn with_revision(
        &self,
        revision: MeasurementCalibrationRevision,
    ) -> Self {
        let mut result = self.clone();
        result.revision = revision;
        result
    }

    /// Returns a copy with an operational status.
    #[must_use]
    pub fn with_status(
        &self,
        status: MeasurementCalibrationStatus,
    ) -> Self {
        let mut result = self.clone();
        result.status = status;
        result
    }

    /// Returns a copy with calibration lineage.
    #[must_use]
    pub fn with_provenance(
        &self,
        provenance: CalibrationId,
    ) -> Self {
        let mut result = self.clone();
        result.provenance = Some(provenance);
        result
    }

    /// Returns a copy with a human-readable description.
    pub fn with_description(
        &self,
        description: impl Into<String>,
    ) -> ZqnResult<Self> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(ZqnError::invalid_calibration(
                "measurement calibration description must not be empty",
            ));
        }

        let mut result = self.clone();
        result.description = Some(description);
        result.validate()?;

        Ok(result)
    }

    /// Adds a parameter binding.
    ///
    /// Parameter IDs are references only. The referenced parameter must be
    /// resolved by the calibration snapshot/registry layer.
    pub fn with_parameter_binding(
        &self,
        binding: MeasurementParameterBinding,
    ) -> ZqnResult<Self> {
        if self
            .parameter_bindings
            .iter()
            .any(|existing| existing.role == binding.role)
        {
            return Err(ZqnError::invalid_calibration(
                "duplicate measurement calibration parameter role",
            ));
        }

        let mut result = self.clone();
        result.parameter_bindings.push(binding);
        result.parameter_bindings.sort();
        result.validate()?;

        Ok(result)
    }

    /// Adds a calibrated transition.
    ///
    /// The same actual/observed pair may not occur twice.
    pub fn with_transition(
        &self,
        transition: MeasurementTransition,
    ) -> ZqnResult<Self> {
        if self.transitions.iter().any(|existing| {
            existing.actual == transition.actual
                && existing.observed == transition.observed
        }) {
            return Err(ZqnError::invalid_calibration(
                "duplicate measurement transition",
            ));
        }

        let mut result = self.clone();
        result.transitions.push(transition);
        result.transitions.sort();
        result.validate()?;

        Ok(result)
    }

    /// Returns a copy with calibrated measurement backaction.
    pub fn with_backaction(
        &self,
        backaction: MeasurementBackaction,
    ) -> ZqnResult<Self> {
        let mut result = self.clone();
        result.backaction = Some(backaction);
        result.validate()?;

        Ok(result)
    }

    /// Returns whether this calibration applies to the exact supplied scope.
    ///
    /// Scope semantics remain owned by `CalibrationScope`. This method performs
    /// exact semantic equality only; it does not invent wildcard or subset
    /// matching rules.
    #[must_use]
    pub fn applies_to_scope(&self, scope: &CalibrationScope) -> bool {
        &self.scope == scope
    }

    /// Returns whether the calibration applies to the supplied timestamp.
    #[must_use]
    pub fn applies_at(&self, timestamp_ns: i128) -> bool {
        self.validity.contains(timestamp_ns)
            && matches!(
                self.status,
                MeasurementCalibrationStatus::Valid
                    | MeasurementCalibrationStatus::Unvalidated
            )
    }

    /// Returns whether a parameter role is present.
    #[must_use]
    pub fn has_parameter_role(&self, role: &MeasurementParameterRole) -> bool {
        self.parameter_bindings
            .binary_search_by(|binding| binding.role.cmp(role))
            .is_ok()
    }

    /// Finds the parameter ID associated with a role.
    #[must_use]
    pub fn parameter_for_role(
        &self,
        role: &MeasurementParameterRole,
    ) -> Option<NoiseParameterId> {
        self.parameter_bindings
            .binary_search_by(|binding| binding.role.cmp(role))
            .ok()
            .map(|index| self.parameter_bindings[index].parameter_id)
    }

    /// Validates all local semantic invariants.
    ///
    /// This does not resolve parameter references.
    pub fn validate(&self) -> ZqnResult<()> {
        self.identity.validate()?;

        self.scope
            .validate()
            .map_err(|error| {
                ZqnError::invalid_calibration(error.to_string())
            })?;

        self.arity.validate()?;
        self.outcome_domain.validate()?;

        for pair in self.parameter_bindings.windows(2) {
            if pair[0] >= pair[1] {
                return Err(ZqnError::invalid_calibration(
                    "measurement parameter bindings are not in canonical \
                     strict order",
                ));
            }
        }

        for pair in self.transitions.windows(2) {
            if pair[0] >= pair[1] {
                return Err(ZqnError::invalid_calibration(
                    "measurement transitions are not in canonical strict \
                     order",
                ));
            }
        }

        for binding in &self.parameter_bindings {
            if binding.role.as_str().trim().is_empty() {
                return Err(ZqnError::invalid_calibration(
                    "measurement parameter role must not be empty",
                ));
            }
        }

        for transition in &self.transitions {
            if !self.outcome_domain.contains(&transition.actual)
                || !self.outcome_domain.contains(&transition.observed)
            {
                return Err(ZqnError::invalid_calibration(
                    "measurement transition references an outcome outside \
                     the declared outcome domain",
                ));
            }
        }

        if let Some(backaction) = &self.backaction {
            let _ = backaction.parameter_id();
        }

        if let Some(description) = self.description.as_deref() {
            if description.trim().is_empty() {
                return Err(ZqnError::invalid_calibration(
                    "measurement calibration description must not be empty",
                ));
            }
        }

        if matches!(
            self.status,
            MeasurementCalibrationStatus::Valid
                | MeasurementCalibrationStatus::Unvalidated
        ) {
            self.validity.validate()?;
        }

        Ok(())
    }

    /// Validates against explicit resource limits.
    pub fn validate_with_limits(
        &self,
        limits: &MeasurementCalibrationLimits,
    ) -> ZqnResult<()> {
        self.validate()?;
        limits.validate(self)
    }

    /// Returns the number of declared calibrated transitions.
    #[must_use]
    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Returns the number of parameter bindings.
    #[must_use]
    pub fn parameter_binding_count(&self) -> usize {
        self.parameter_bindings.len()
    }

    /// Returns whether the calibration has a complete transition relation for
    /// the declared outcome domain.
    ///
    /// This method checks only structural completeness:
    ///
    ///     outcomes × outcomes
    ///
    /// It does not inspect the referenced parameter values.
    ///
    /// For enormous outcome domains this is intentionally an explicit
    /// potentially-expensive operation. Callers should use resource limits
    /// before invoking it on untrusted data.
    pub fn has_complete_transition_relation(&self) -> ZqnResult<bool> {
        let outcome_count = self.outcome_domain.len();

        let expected = outcome_count.checked_mul(outcome_count).ok_or_else(
            || {
                ZqnError::resource_limit_exceeded(
                    "measurement transition relation size overflows host \
                     representation",
                )
            },
        )?;

        if self.transitions.len() != expected {
            return Ok(false);
        }

        Ok(true)
    }

    /// Returns all canonical logical qubit IDs directly represented by this
    /// calibration scope.
    ///
    /// This is intentionally limited to direct logical-qubit scopes. Composite
    /// scope interpretation remains owned by the scope subsystem.
    #[must_use]
    pub fn logical_qubits(&self) -> Vec<QubitId> {
        match &self.scope {
            CalibrationScope::LogicalQubit(qubit) => vec![*qubit],

            CalibrationScope::LogicalQubits(qubits) => {
                let mut result = qubits.clone();
                result.sort();
                result.dedup();
                result
            }

            _ => Vec::new(),
        }
    }

    /// Returns all canonical physical qubit IDs directly represented by this
    /// calibration scope.
    #[must_use]
    pub fn physical_qubits(&self) -> Vec<PhysicalQubitId> {
        match &self.scope {
            CalibrationScope::PhysicalQubit(qubit) => vec![*qubit],

            CalibrationScope::PhysicalQubits(qubits) => {
                let mut result = qubits.clone();
                result.sort();
                result.dedup();
                result
            }

            _ => Vec::new(),
        }
    }
}

// ============================================================================
// VALIDITY HELPERS
// ============================================================================

impl MeasurementCalibrationValidity {
    /// Validates the interval.
    pub fn validate(&self) -> ZqnResult<()> {
        if let (Some(start), Some(end)) =
            (self.not_before_ns, self.not_after_ns)
        {
            if start >= end {
                return Err(ZqnError::invalid_calibration(
                    "measurement calibration validity interval must have \
                     end strictly greater than start",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity_interval_is_half_open() {
        let validity =
            MeasurementCalibrationValidity::new(Some(10), Some(20))
                .expect("valid interval");

        assert!(!validity.contains(9));
        assert!(validity.contains(10));
        assert!(validity.contains(19));
        assert!(!validity.contains(20));
    }

    #[test]
    fn invalid_validity_interval_is_rejected() {
        let result =
            MeasurementCalibrationValidity::new(Some(20), Some(10));

        assert!(result.is_err());
    }

    #[test]
    fn outcome_domain_is_canonicalized() {
        let a = MeasurementOutcome::new("a").expect("outcome");
        let b = MeasurementOutcome::new("b").expect("outcome");

        let domain =
            MeasurementOutcomeDomain::new(vec![b.clone(), a.clone()])
                .expect("domain");

        assert_eq!(domain.outcomes(), &[a, b]);
    }

    #[test]
    fn duplicate_outcomes_are_collapsed_deterministically() {
        let a = MeasurementOutcome::new("a").expect("outcome");

        let domain =
            MeasurementOutcomeDomain::new(vec![a.clone(), a.clone()])
                .expect("domain");

        assert_eq!(domain.len(), 1);
        assert_eq!(domain.outcomes()[0], a);
    }

    #[test]
    fn empty_outcome_domain_is_rejected() {
        assert!(MeasurementOutcomeDomain::new(Vec::new()).is_err());
    }

    #[test]
    fn measurement_identity_rejects_empty_name() {
        assert!(MeasurementIdentity::new(" ").is_err());
    }

    #[test]
    fn measurement_identity_variant_is_supported() {
        let identity =
            MeasurementIdentity::with_variant("measurement", "custom")
                .expect("identity");

        assert_eq!(identity.name(), "measurement");
        assert_eq!(identity.variant(), Some("custom"));
    }

    #[test]
    fn explicit_zero_arity_is_rejected() {
        assert!(MeasurementArity::exact(0).validate().is_err());
    }

    #[test]
    fn inferred_arity_has_no_semantic_machine_limit() {
        assert_eq!(MeasurementArity::inferred().value(), None);
    }

    #[test]
    fn limits_are_unbounded_by_default() {
        let limits = MeasurementCalibrationLimits::default();

        assert_eq!(limits.max_outcomes, None);
        assert_eq!(limits.max_parameter_bindings, None);
        assert_eq!(limits.max_transitions, None);
        assert_eq!(limits.max_arity, None);
    }
}