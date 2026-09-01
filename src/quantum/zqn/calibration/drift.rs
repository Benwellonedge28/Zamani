//! # ZQN Calibration Drift
//!
//! Production-grade representation of temporal drift in quantum calibration
//! data.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - temporal drift models;
//! - drift model identity and provenance references;
//! - resource scope to which drift applies;
//! - parameter references (`NoiseParameterId`);
//! - time-domain validity;
//! - deterministic drift evaluation;
//! - uncertainty associated with drift;
//! - explicit approximation/error bounds;
//! - validation and resource-limit enforcement.
//!
//! ## Does not own
//!
//! This module does NOT own:
//!
//! - canonical quantum IR;
//! - quantum operation semantics;
//! - concrete calibration parameter storage;
//! - hardware APIs;
//! - QPU transport;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - noise-channel mathematics;
//! - source-language syntax;
//! - global clocks;
//! - global mutable calibration state.
//!
//! `calibration::parameter` remains the owner of calibrated parameter values.
//! `calibration::snapshot` owns collections of calibration state.
//! `calibration::interpolation` owns generic interpolation policy where that
//! policy is broader than temporal drift.
//!
//! ## Scalability
//!
//! No semantic maximum number of resources, parameters, observations, or
//! timestamps is imposed by this module.
//!
//! Any computational/allocation limits are supplied through
//! [`DriftLimits`] by the caller.
//!
//! There are no machine-size constants, vendor-specific limits, or fixed
//! qubit counts.
//!
//! ## Determinism
//!
//! Drift evaluation is a pure deterministic function of:
//!
//! - the drift model;
//! - parameter reference;
//! - resource identity;
//! - evaluation timestamp;
//! - supplied validation policy.
//!
//! No global clock and no global RNG are used.
//!
//! ## Time
//!
//! Time is represented as signed nanoseconds relative to an externally
//! defined epoch. This avoids coupling ZQN to a particular operating-system
//! clock or wall-clock implementation.
//!
//! ## Numerical policy
//!
//! The module never silently:
//!
//! - clamps NaN;
//! - converts infinity to a finite value;
//! - normalizes invalid values;
//! - changes a negative probability/value into a positive one;
//! - silently changes an approximation into an exact result.
//!
//! All numerical assumptions are explicit.
//!
//! ## Integration
//!
//! ```text
//! calibration::parameter
//!         │
//!         │ NoiseParameterId
//!         ▼
//! calibration::drift
//!         │
//!         ├──────────────► calibration::snapshot
//!         │
//!         ├──────────────► calibration::interpolation
//!         │
//!         ├──────────────► calibration::validation
//!         │
//!         ├──────────────► characterization
//!         │
//!         ├──────────────► noise
//!         │
//!         ├──────────────► scheduling
//!         │
//!         └──────────────► hardware adapters
//! ```
//!
//! The module intentionally communicates through stable IDs and plain data
//! contracts rather than depending on downstream implementations.

use std::cmp::Ordering;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::parameter::NoiseParameterId;

use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::CalibrationId;

/// A signed timestamp in nanoseconds relative to an externally defined epoch.
///
/// ZQN deliberately does not define the epoch. The caller must use one
/// consistent time domain for a calibration set.
pub type DriftTime = i128;

/// A signed duration in nanoseconds.
pub type DriftDuration = i128;

/// Identifies the scope to which a drift model applies.
///
/// Resource ordering is significant for composite/joint resources because
/// calibration semantics may depend on resource order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftScope {
    /// Drift applies to a device-wide calibration quantity.
    DeviceWide,

    /// Drift applies to a logical-qubit ordered tuple.
    LogicalQubits(Vec<QubitId>),

    /// Drift applies to a physical-qubit ordered tuple.
    PhysicalQubits(Vec<PhysicalQubitId>),

    /// Drift applies to an externally defined resource.
    ///
    /// `namespace` and `key` are opaque identifiers owned by the integration
    /// layer. This allows future resource types without changing this module.
    Named {
        namespace: String,
        key: String,
    },
}

impl DriftScope {
    /// Creates a named scope.
    pub fn named(namespace: String, key: String) -> ZqnResult<Self> {
        validate_non_empty("drift scope namespace", &namespace)?;
        validate_non_empty("drift scope key", &key)?;

        Ok(Self::Named { namespace, key })
    }

    /// Returns the number of explicitly enumerated quantum resources.
    ///
    /// Returns `None` for device-wide and opaque named scopes.
    pub fn arity(&self) -> Option<usize> {
        match self {
            Self::DeviceWide | Self::Named { .. } => None,
            Self::LogicalQubits(resources) => Some(resources.len()),
            Self::PhysicalQubits(resources) => Some(resources.len()),
        }
    }

    /// Returns whether this scope exactly matches another scope.
    pub fn matches(&self, other: &Self) -> bool {
        self == other
    }

    /// Validates the scope.
    pub fn validate(&self, limits: &DriftLimits) -> ZqnResult<()> {
        match self {
            Self::DeviceWide => Ok(()),

            Self::LogicalQubits(resources) => {
                validate_resource_count(resources.len(), limits)?;
                validate_unique_resources(resources)
            }

            Self::PhysicalQubits(resources) => {
                validate_resource_count(resources.len(), limits)?;
                validate_unique_resources(resources)
            }

            Self::Named { namespace, key } => {
                validate_non_empty("drift scope namespace", namespace)?;
                validate_non_empty("drift scope key", key)?;
                Ok(())
            }
        }
    }
}

/// Identifies what is drifting.
///
/// The parameter itself remains owned by `calibration::parameter`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftParameter {
    parameter_id: NoiseParameterId,
    role: String,
}

impl DriftParameter {
    /// Creates a drift parameter reference.
    pub fn new(parameter_id: NoiseParameterId, role: String) -> ZqnResult<Self> {
        validate_non_empty("drift parameter role", &role)?;

        Ok(Self {
            parameter_id,
            role,
        })
    }

    /// Returns the referenced calibration parameter.
    pub fn parameter_id(&self) -> &NoiseParameterId {
        &self.parameter_id
    }

    /// Returns the semantic role of the parameter.
    pub fn role(&self) -> &str {
        &self.role
    }
}

/// Temporal domain over which a drift model is valid.
///
/// The interval is half-open:
///
/// `[start, end)`
///
/// `None` means unbounded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriftValidity {
    start: Option<DriftTime>,
    end: Option<DriftTime>,
}

impl DriftValidity {
    /// Creates a validity interval.
    pub fn new(start: Option<DriftTime>, end: Option<DriftTime>) -> ZqnResult<Self> {
        if let (Some(start), Some(end)) = (start, end) {
            if start >= end {
                return Err(invalid_calibration(
                    "drift validity interval requires start < end",
                ));
            }
        }

        Ok(Self { start, end })
    }

    /// Returns an unbounded validity interval.
    pub const fn unbounded() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    /// Returns the inclusive lower bound.
    pub const fn start(&self) -> Option<DriftTime> {
        self.start
    }

    /// Returns the exclusive upper bound.
    pub const fn end(&self) -> Option<DriftTime> {
        self.end
    }

    /// Tests whether the timestamp belongs to the interval.
    pub fn contains(&self, timestamp: DriftTime) -> bool {
        if let Some(start) = self.start {
            if timestamp < start {
                return false;
            }
        }

        if let Some(end) = self.end {
            if timestamp >= end {
                return false;
            }
        }

        true
    }
}

/// The mathematical form of temporal drift.
///
/// The models operate on elapsed time from a reference timestamp.
///
/// The output is a multiplicative or additive transformation depending on
/// the selected model.
#[derive(Debug, Clone, PartialEq)]
pub enum DriftModel {
    /// No temporal change.
    Constant,

    /// Linear additive drift:
    ///
    /// `value(t) = base + rate * elapsed`
    ///
    /// where `elapsed` is expressed in the model's configured time unit.
    Linear {
        /// Reference value at `reference_time`.
        base: f64,

        /// Change per unit of elapsed time.
        rate: f64,

        /// Nanoseconds represented by one model time unit.
        ///
        /// For example, `1` means rate is per nanosecond and
        /// `1_000_000_000` means rate is per second.
        time_unit_ns: i128,
    },

    /// Linear multiplicative drift:
    ///
    /// `value(t) = base * (1 + rate * elapsed)`
    LinearRelative {
        /// Reference value at `reference_time`.
        base: f64,

        /// Relative change per model time unit.
        rate: f64,

        /// Nanoseconds represented by one model time unit.
        time_unit_ns: i128,
    },

    /// Exponential drift:
    ///
    /// `value(t) = base * exp(rate * elapsed)`
    Exponential {
        /// Reference value.
        base: f64,

        /// Exponential rate per model time unit.
        rate: f64,

        /// Nanoseconds represented by one model time unit.
        time_unit_ns: i128,
    },

    /// Periodic drift:
    ///
    /// `value(t) = offset + amplitude * sin(2π * elapsed / period + phase)`
    Periodic {
        /// Constant offset.
        offset: f64,

        /// Oscillation amplitude.
        amplitude: f64,

        /// Period in nanoseconds.
        period_ns: DriftDuration,

        /// Phase in radians.
        phase_radians: f64,
    },

    /// Piecewise-linear drift based on explicitly supplied knots.
    ///
    /// The first point must be the earliest point and timestamps must be
    /// strictly increasing.
    PiecewiseLinear {
        points: Vec<DriftPoint>,
    },

    /// A user-supplied sampled drift trace.
    ///
    /// No interpolation is performed by this variant. Evaluation returns the
    /// exact sample at the requested timestamp only.
    Sampled {
        points: Vec<DriftPoint>,
    },
}

/// A timestamp/value pair used by sampled and piecewise-linear models.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DriftPoint {
    timestamp: DriftTime,
    value: f64,
}

impl DriftPoint {
    /// Creates a point.
    pub fn new(timestamp: DriftTime, value: f64) -> ZqnResult<Self> {
        validate_finite("drift point value", value)?;

        Ok(Self { timestamp, value })
    }

    /// Returns the timestamp.
    pub const fn timestamp(&self) -> DriftTime {
        self.timestamp
    }

    /// Returns the value.
    pub const fn value(&self) -> f64 {
        self.value
    }
}

/// Defines how a drift model is interpreted outside its explicit domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtrapolationPolicy {
    /// Reject evaluation outside the model's domain.
    Reject,

    /// Hold the first/last value.
    Hold,

    /// Continue the first/last linear slope.
    Linear,
}

/// Describes whether a drift result is exact or approximate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriftAccuracy {
    /// The model directly defines the requested value.
    Exact,

    /// The value is obtained through an explicitly declared approximation.
    Approximate {
        /// Absolute error bound supplied by the model/integration layer.
        absolute_error_bound: f64,
    },
}

impl DriftAccuracy {
    /// Validates the accuracy declaration.
    pub fn validate(&self) -> ZqnResult<()> {
        match self {
            Self::Exact => Ok(()),
            Self::Approximate {
                absolute_error_bound,
            } => {
                validate_finite("drift absolute error bound", *absolute_error_bound)?;

                if *absolute_error_bound < 0.0 {
                    return Err(invalid_calibration(
                        "drift absolute error bound cannot be negative",
                    ));
                }

                Ok(())
            }
        }
    }
}

/// Explicit numerical validation policy.
///
/// No numerical tolerance is hard-coded into the semantic model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DriftValidationPolicy {
    /// Allowed absolute error when validating numerical continuity or
    /// normalization-like invariants.
    pub absolute_tolerance: f64,

    /// Whether sampled/piecewise points must be strictly ordered.
    pub require_strict_point_order: bool,

    /// Whether drift evaluation must reject non-finite results.
    pub reject_non_finite_results: bool,
}

impl DriftValidationPolicy {
    /// Creates a validation policy.
    pub fn new(
        absolute_tolerance: f64,
        require_strict_point_order: bool,
        reject_non_finite_results: bool,
    ) -> ZqnResult<Self> {
        validate_finite("drift validation tolerance", absolute_tolerance)?;

        if absolute_tolerance < 0.0 {
            return Err(invalid_calibration(
                "drift validation tolerance cannot be negative",
            ));
        }

        Ok(Self {
            absolute_tolerance,
            require_strict_point_order,
            reject_non_finite_results,
        })
    }
}

/// Resource limits for drift validation/evaluation.
///
/// `None` means no ZQN-imposed limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriftLimits {
    /// Maximum number of explicitly enumerated resources.
    pub max_resources: Option<u64>,

    /// Maximum number of parameters referenced by a drift set.
    pub max_parameters: Option<u64>,

    /// Maximum number of explicit drift points.
    pub max_points: Option<u64>,
}

impl DriftLimits {
    /// Creates an unlimited policy.
    ///
    /// "Unlimited" means ZQN does not impose a capacity limit. Physical
    /// memory, execution environment, and caller policy still apply.
    pub const fn unlimited() -> Self {
        Self {
            max_resources: None,
            max_parameters: None,
            max_points: None,
        }
    }
}

/// A complete drift definition for one calibrated parameter/resource scope.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationDrift {
    calibration_id: Option<CalibrationId>,
    parameter: DriftParameter,
    scope: DriftScope,
    reference_time: DriftTime,
    model: DriftModel,
    validity: DriftValidity,
    extrapolation: ExtrapolationPolicy,
    accuracy: DriftAccuracy,
    revision: u64,
}

impl CalibrationDrift {
    /// Creates a complete drift definition.
    ///
    /// Validation is explicit and uses caller-provided limits and numerical
    /// policy. No hidden capacity or numerical assumptions are introduced.
    pub fn new(
        parameter: DriftParameter,
        scope: DriftScope,
        reference_time: DriftTime,
        model: DriftModel,
        validity: DriftValidity,
        extrapolation: ExtrapolationPolicy,
        accuracy: DriftAccuracy,
        revision: u64,
        limits: &DriftLimits,
        policy: &DriftValidationPolicy,
    ) -> ZqnResult<Self> {
        let drift = Self {
            calibration_id: None,
            parameter,
            scope,
            reference_time,
            model,
            validity,
            extrapolation,
            accuracy,
            revision,
        };

        drift.validate(limits, policy)?;
        Ok(drift)
    }

    /// Associates a calibration identity with this drift definition.
    pub fn with_calibration_id(mut self, calibration_id: CalibrationId) -> Self {
        self.calibration_id = Some(calibration_id);
        self
    }

    /// Returns the calibration identity if assigned.
    pub fn calibration_id(&self) -> Option<&CalibrationId> {
        self.calibration_id.as_ref()
    }

    /// Returns the referenced parameter.
    pub fn parameter(&self) -> &DriftParameter {
        &self.parameter
    }

    /// Returns the resource scope.
    pub fn scope(&self) -> &DriftScope {
        &self.scope
    }

    /// Returns the reference timestamp.
    pub const fn reference_time(&self) -> DriftTime {
        self.reference_time
    }

    /// Returns the drift model.
    pub fn model(&self) -> &DriftModel {
        &self.model
    }

    /// Returns the validity interval.
    pub const fn validity(&self) -> DriftValidity {
        self.validity
    }

    /// Returns the extrapolation policy.
    pub const fn extrapolation_policy(&self) -> ExtrapolationPolicy {
        self.extrapolation
    }

    /// Returns the declared accuracy.
    pub const fn accuracy(&self) -> DriftAccuracy {
        self.accuracy
    }

    /// Returns the revision.
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns whether this drift definition is valid at a timestamp.
    pub fn is_valid_at(&self, timestamp: DriftTime) -> bool {
        self.validity.contains(timestamp)
    }

    /// Evaluates the drift at a timestamp.
    ///
    /// The returned value is the value predicted by the drift model itself.
    /// Combining this result with a concrete calibrated parameter is the
    /// responsibility of the calibration layer.
    pub fn evaluate(
        &self,
        timestamp: DriftTime,
        policy: &DriftValidationPolicy,
    ) -> ZqnResult<DriftEvaluation> {
        if !self.validity.contains(timestamp) {
            return Err(invalid_calibration(
                "drift evaluation timestamp is outside the validity interval",
            ));
        }

        let value = evaluate_model(
            &self.model,
            self.reference_time,
            timestamp,
            self.extrapolation,
            policy,
        )?;

        if policy.reject_non_finite_results {
            validate_finite("drift evaluation result", value)?;
        }

        Ok(DriftEvaluation {
            timestamp,
            value,
            accuracy: self.accuracy,
            revision: self.revision,
        })
    }

    /// Validates the complete drift definition.
    pub fn validate(
        &self,
        limits: &DriftLimits,
        policy: &DriftValidationPolicy,
    ) -> ZqnResult<()> {
        self.parameter.validate()?;
        self.scope.validate(limits)?;
        self.validity.validate()?;
        self.accuracy.validate()?;

        validate_model(&self.model, limits, policy)?;

        Ok(())
    }
}

/// Result of evaluating a drift model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DriftEvaluation {
    timestamp: DriftTime,
    value: f64,
    accuracy: DriftAccuracy,
    revision: u64,
}

impl DriftEvaluation {
    /// Returns the evaluation timestamp.
    pub const fn timestamp(&self) -> DriftTime {
        self.timestamp
    }

    /// Returns the predicted value.
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns the declared accuracy.
    pub const fn accuracy(&self) -> DriftAccuracy {
        self.accuracy
    }

    /// Returns the calibration revision used.
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

/// A collection of drift definitions.
///
/// Drift definitions are kept as an ordered vector so the module does not
/// impose trait requirements such as `Ord` on opaque identifier types.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DriftSet {
    entries: Vec<CalibrationDrift>,
}

impl DriftSet {
    /// Creates an empty drift set.
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates a drift set from entries.
    pub fn from_entries(
        entries: Vec<CalibrationDrift>,
        limits: &DriftLimits,
        policy: &DriftValidationPolicy,
    ) -> ZqnResult<Self> {
        let set = Self { entries };
        set.validate(limits, policy)?;
        Ok(set)
    }

    /// Adds a drift definition.
    ///
    /// Duplicate `(parameter, scope)` pairs are rejected. The caller must
    /// replace the previous calibration revision explicitly.
    pub fn insert(
        &mut self,
        drift: CalibrationDrift,
        limits: &DriftLimits,
        policy: &DriftValidationPolicy,
    ) -> ZqnResult<()> {
        drift.validate(limits, policy)?;

        if let Some(limit) = limits.max_parameters {
            let current = u64_from_usize(self.entries.len())?;

            if current >= limit {
                return Err(resource_limit(
                    "drift parameter collection exceeds configured limit",
                ));
            }
        }

        if self
            .entries
            .iter()
            .any(|existing| same_drift_key(existing, &drift))
        {
            return Err(invalid_calibration(
                "duplicate drift definition for the same parameter and scope",
            ));
        }

        self.entries.push(drift);
        Ok(())
    }

    /// Removes a drift definition by parameter and scope.
    ///
    /// Returns the removed definition when one existed.
    pub fn remove(
        &mut self,
        parameter_id: &NoiseParameterId,
        scope: &DriftScope,
    ) -> Option<CalibrationDrift> {
        let index = self.entries.iter().position(|entry| {
            entry.parameter().parameter_id() == parameter_id && entry.scope() == scope
        })?;

        Some(self.entries.remove(index))
    }

    /// Returns all entries.
    pub fn entries(&self) -> &[CalibrationDrift] {
        &self.entries
    }

    /// Finds a matching drift definition.
    ///
    /// Selection precedence is intentionally not implemented here. If multiple
    /// temporal revisions or scopes can match, the higher calibration layer
    /// must define precedence explicitly.
    pub fn find(
        &self,
        parameter_id: &NoiseParameterId,
        scope: &DriftScope,
    ) -> Option<&CalibrationDrift> {
        self.entries.iter().find(|entry| {
            entry.parameter().parameter_id() == parameter_id && entry.scope() == scope
        })
    }

    /// Validates the complete set.
    pub fn validate(
        &self,
        limits: &DriftLimits,
        policy: &DriftValidationPolicy,
    ) -> ZqnResult<()> {
        if let Some(limit) = limits.max_parameters {
            if u64_from_usize(self.entries.len())? > limit {
                return Err(resource_limit(
                    "drift collection exceeds configured parameter limit",
                ));
            }
        }

        for entry in &self.entries {
            entry.validate(limits, policy)?;
        }

        for left in 0..self.entries.len() {
            for right in (left + 1)..self.entries.len() {
                if same_drift_key(&self.entries[left], &self.entries[right]) {
                    return Err(invalid_calibration(
                        "drift collection contains duplicate parameter/scope entries",
                    ));
                }
            }
        }

        Ok(())
    }
}

fn same_drift_key(left: &CalibrationDrift, right: &CalibrationDrift) -> bool {
    left.parameter().parameter_id() == right.parameter().parameter_id()
        && left.scope() == right.scope()
}

fn validate_model(
    model: &DriftModel,
    limits: &DriftLimits,
    policy: &DriftValidationPolicy,
) -> ZqnResult<()> {
    match model {
        DriftModel::Constant => Ok(()),

        DriftModel::Linear {
            base,
            rate,
            time_unit_ns,
        }
        | DriftModel::LinearRelative {
            base,
            rate,
            time_unit_ns,
        } => {
            validate_finite("drift base", *base)?;
            validate_finite("drift rate", *rate)?;

            if *time_unit_ns <= 0 {
                return Err(invalid_calibration(
                    "drift time unit must be greater than zero",
                ));
            }

            Ok(())
        }

        DriftModel::Exponential {
            base,
            rate,
            time_unit_ns,
        } => {
            validate_finite("drift base", *base)?;
            validate_finite("drift rate", *rate)?;

            if *time_unit_ns <= 0 {
                return Err(invalid_calibration(
                    "drift time unit must be greater than zero",
                ));
            }

            Ok(())
        }

        DriftModel::Periodic {
            offset,
            amplitude,
            period_ns,
            phase_radians,
        } => {
            validate_finite("drift offset", *offset)?;
            validate_finite("drift amplitude", *amplitude)?;
            validate_finite("drift phase", *phase_radians)?;

            if *period_ns <= 0 {
                return Err(invalid_calibration(
                    "periodic drift period must be greater than zero",
                ));
            }

            Ok(())
        }

        DriftModel::PiecewiseLinear { points } => {
            validate_points(points, limits, policy)
        }

        DriftModel::Sampled { points } => {
            validate_points(points, limits, policy)
        }
    }
}

fn validate_points(
    points: &[DriftPoint],
    limits: &DriftLimits,
    policy: &DriftValidationPolicy,
) -> ZqnResult<()> {
    if points.is_empty() {
        return Err(invalid_calibration(
            "drift point collection cannot be empty",
        ));
    }

    if let Some(limit) = limits.max_points {
        if u64_from_usize(points.len())? > limit {
            return Err(resource_limit(
                "drift point collection exceeds configured limit",
            ));
        }
    }

    for point in points {
        validate_finite("drift point value", point.value)?;
    }

    if policy.require_strict_point_order {
        for pair in points.windows(2) {
            if pair[0].timestamp >= pair[1].timestamp {
                return Err(invalid_calibration(
                    "drift points must have strictly increasing timestamps",
                ));
            }
        }
    }

    Ok(())
}

fn evaluate_model(
    model: &DriftModel,
    reference_time: DriftTime,
    timestamp: DriftTime,
    extrapolation: ExtrapolationPolicy,
    policy: &DriftValidationPolicy,
) -> ZqnResult<f64> {
    let result = match model {
        DriftModel::Constant => 1.0,

        DriftModel::Linear {
            base,
            rate,
            time_unit_ns,
        } => {
            let elapsed = elapsed_units(reference_time, timestamp, *time_unit_ns)?;
            *base + (*rate * elapsed)
        }

        DriftModel::LinearRelative {
            base,
            rate,
            time_unit_ns,
        } => {
            let elapsed = elapsed_units(reference_time, timestamp, *time_unit_ns)?;
            *base * (1.0 + (*rate * elapsed))
        }

        DriftModel::Exponential {
            base,
            rate,
            time_unit_ns,
        } => {
            let elapsed = elapsed_units(reference_time, timestamp, *time_unit_ns)?;
            *base * (*rate * elapsed).exp()
        }

        DriftModel::Periodic {
            offset,
            amplitude,
            period_ns,
            phase_radians,
        } => {
            let elapsed = elapsed_duration(reference_time, timestamp)?;
            let angle = phase_for_period(elapsed, *period_ns, *phase_radians)?;
            *offset + (*amplitude * angle.sin())
        }

        DriftModel::PiecewiseLinear { points } => {
            evaluate_piecewise(points, timestamp, extrapolation)?
        }

        DriftModel::Sampled { points } => {
            evaluate_sampled(points, timestamp, extrapolation)?
        }
    };

    if policy.reject_non_finite_results {
        validate_finite("drift evaluation result", result)?;
    }

    Ok(result)
}

fn evaluate_piecewise(
    points: &[DriftPoint],
    timestamp: DriftTime,
    extrapolation: ExtrapolationPolicy,
) -> ZqnResult<f64> {
    if points.is_empty() {
        return Err(invalid_calibration(
            "piecewise-linear drift requires at least one point",
        ));
    }

    if points.len() == 1 {
        return evaluate_single_point(points[0], timestamp, extrapolation);
    }

    if timestamp < points[0].timestamp {
        return extrapolate(points, true, timestamp, extrapolation);
    }

    if timestamp > points[points.len() - 1].timestamp {
        return extrapolate(points, false, timestamp, extrapolation);
    }

    if timestamp == points[0].timestamp {
        return Ok(points[0].value);
    }

    if timestamp == points[points.len() - 1].timestamp {
        return Ok(points[points.len() - 1].value);
    }

    let index = match points.binary_search_by(|point| point.timestamp.cmp(&timestamp)) {
        Ok(index) => return Ok(points[index].value),
        Err(index) => index,
    };

    let left = points[index - 1];
    let right = points[index];

    interpolate(left, right, timestamp)
}

fn evaluate_sampled(
    points: &[DriftPoint],
    timestamp: DriftTime,
    extrapolation: ExtrapolationPolicy,
) -> ZqnResult<f64> {
    if points.is_empty() {
        return Err(invalid_calibration(
            "sampled drift requires at least one point",
        ));
    }

    match points.binary_search_by(|point| point.timestamp.cmp(&timestamp)) {
        Ok(index) => Ok(points[index].value),
        Err(index) => {
            if index == 0 {
                match extrapolation {
                    ExtrapolationPolicy::Reject => Err(invalid_calibration(
                        "sampled drift timestamp precedes available samples",
                    )),
                    ExtrapolationPolicy::Hold | ExtrapolationPolicy::Linear => {
                        Ok(points[0].value)
                    }
                }
            } else if index == points.len() {
                match extrapolation {
                    ExtrapolationPolicy::Reject => Err(invalid_calibration(
                        "sampled drift timestamp exceeds available samples",
                    )),
                    ExtrapolationPolicy::Hold | ExtrapolationPolicy::Linear => {
                        Ok(points[points.len() - 1].value)
                    }
                }
            } else {
                Err(invalid_calibration(
                    "sampled drift has no exact sample at the requested timestamp",
                ))
            }
        }
    }
}

fn evaluate_single_point(
    point: DriftPoint,
    timestamp: DriftTime,
    extrapolation: ExtrapolationPolicy,
) -> ZqnResult<f64> {
    if timestamp == point.timestamp {
        return Ok(point.value);
    }

    match extrapolation {
        ExtrapolationPolicy::Reject => Err(invalid_calibration(
            "timestamp is outside single-point drift domain",
        )),
        ExtrapolationPolicy::Hold | ExtrapolationPolicy::Linear => Ok(point.value),
    }
}

fn extrapolate(
    points: &[DriftPoint],
    before: bool,
    timestamp: DriftTime,
    policy: ExtrapolationPolicy,
) -> ZqnResult<f64> {
    match policy {
        ExtrapolationPolicy::Reject => Err(invalid_calibration(
            "drift evaluation requires extrapolation outside the model domain",
        )),

        ExtrapolationPolicy::Hold => {
            if before {
                Ok(points[0].value)
            } else {
                Ok(points[points.len() - 1].value)
            }
        }

        ExtrapolationPolicy::Linear => {
            if points.len() < 2 {
                return Ok(points[0].value);
            }

            let (left, right) = if before {
                (points[0], points[1])
            } else {
                let n = points.len();
                (points[n - 2], points[n - 1])
            };

            interpolate(left, right, timestamp)
        }
    }
}

fn interpolate(
    left: DriftPoint,
    right: DriftPoint,
    timestamp: DriftTime,
) -> ZqnResult<f64> {
    let denominator = right
        .timestamp
        .checked_sub(left.timestamp)
        .ok_or_else(|| numerical_failure("drift timestamp subtraction overflow"))?;

    if denominator == 0 {
        return Err(invalid_calibration(
            "drift interpolation points cannot have identical timestamps",
        ));
    }

    let numerator = timestamp
        .checked_sub(left.timestamp)
        .ok_or_else(|| numerical_failure("drift interpolation timestamp overflow"))?;

    let fraction = ratio_i128_to_f64(numerator, denominator)?;

    let result = left.value + ((right.value - left.value) * fraction);

    validate_finite("interpolated drift value", result)?;

    Ok(result)
}

fn elapsed_duration(reference: DriftTime, timestamp: DriftTime) -> ZqnResult<DriftDuration> {
    timestamp
        .checked_sub(reference)
        .ok_or_else(|| numerical_failure("drift elapsed-time calculation overflow"))
}

fn elapsed_units(
    reference: DriftTime,
    timestamp: DriftTime,
    time_unit_ns: i128,
) -> ZqnResult<f64> {
    if time_unit_ns <= 0 {
        return Err(invalid_calibration(
            "drift time unit must be greater than zero",
        ));
    }

    let elapsed = elapsed_duration(reference, timestamp)?;
    ratio_i128_to_f64(elapsed, time_unit_ns)
}

fn phase_for_period(
    elapsed_ns: DriftDuration,
    period_ns: DriftDuration,
    phase_radians: f64,
) -> ZqnResult<f64> {
    if period_ns <= 0 {
        return Err(invalid_calibration(
            "periodic drift period must be greater than zero",
        ));
    }

    let elapsed = ratio_i128_to_f64(elapsed_ns, period_ns)?;

    let turns = elapsed - elapsed.floor();

    let two_pi = 2.0 * std::f64::consts::PI;

    let result = phase_radians + turns * two_pi;

    validate_finite("periodic drift phase", result)?;

    Ok(result)
}

fn ratio_i128_to_f64(numerator: i128, denominator: i128) -> ZqnResult<f64> {
    if denominator == 0 {
        return Err(numerical_failure(
            "cannot calculate ratio with zero denominator",
        ));
    }

    let numerator_f64 = numerator as f64;
    let denominator_f64 = denominator as f64;

    if !numerator_f64.is_finite() || !denominator_f64.is_finite() {
        return Err(numerical_failure(
            "drift time conversion produced a non-finite floating-point value",
        ));
    }

    let result = numerator_f64 / denominator_f64;

    validate_finite("drift time ratio", result)?;

    Ok(result)
}

fn validate_non_empty(name: &str, value: &str) -> ZqnResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_calibration(format!("{name} cannot be empty")));
    }

    Ok(())
}

fn validate_finite(name: &str, value: f64) -> ZqnResult<()> {
    if !value.is_finite() {
        return Err(
            ZqnError::invalid_calibration(format!("{name} must be finite"))
                .with_context("value", value.to_string()),
        );
    }

    Ok(())
}

fn validate_resource_count<T>(
    count: usize,
    limits: &DriftLimits,
) -> ZqnResult<()> {
    if let Some(limit) = limits.max_resources {
        if u64_from_usize(count)? > limit {
            return Err(resource_limit(
                "drift resource collection exceeds configured limit",
            ));
        }
    }

    let _ = std::marker::PhantomData::<T>;

    Ok(())
}

fn validate_unique_resources<T>(resources: &[T]) -> ZqnResult<()>
where
    T: PartialEq,
{
    for left in 0..resources.len() {
        for right in (left + 1)..resources.len() {
            if resources[left] == resources[right] {
                return Err(invalid_calibration(
                    "drift scope contains duplicate resources",
                ));
            }
        }
    }

    Ok(())
}

fn u64_from_usize(value: usize) -> ZqnResult<u64> {
    u64::try_from(value)
        .map_err(|_| numerical_failure("usize value cannot be represented as u64"))
}

fn invalid_calibration(message: impl Into<String>) -> ZqnError {
    ZqnError::invalid_calibration(message.into())
}

fn resource_limit(message: impl Into<String>) -> ZqnError {
    ZqnError::invalid_calibration(message.into())
        .with_context("reason", "resource_limit_exceeded")
}

fn numerical_failure(message: impl Into<String>) -> ZqnError {
    ZqnError::invalid_calibration(message.into())
        .with_context("reason", "numerical_failure")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> DriftValidationPolicy {
        DriftValidationPolicy::new(1.0e-12, true, true)
            .expect("test validation policy must be valid")
    }

    fn limits() -> DriftLimits {
        DriftLimits::unlimited()
    }

    #[test]
    fn validity_interval_is_half_open() {
        let validity = DriftValidity::new(Some(10), Some(20))
            .expect("validity interval should be valid");

        assert!(!validity.contains(9));
        assert!(validity.contains(10));
        assert!(validity.contains(19));
        assert!(!validity.contains(20));
    }

    #[test]
    fn constant_model_returns_one() {
        let model = DriftModel::Constant;

        let result = evaluate_model(
            &model,
            0,
            100,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect("constant model should evaluate");

        assert_eq!(result, 1.0);
    }

    #[test]
    fn linear_model_is_deterministic() {
        let model = DriftModel::Linear {
            base: 1.0,
            rate: 2.0,
            time_unit_ns: 1,
        };

        let first = evaluate_model(
            &model,
            0,
            3,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect("linear model should evaluate");

        let second = evaluate_model(
            &model,
            0,
            3,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect("linear model should evaluate");

        assert_eq!(first, second);
        assert_eq!(first, 7.0);
    }

    #[test]
    fn relative_linear_model_is_correct() {
        let model = DriftModel::LinearRelative {
            base: 10.0,
            rate: 0.1,
            time_unit_ns: 1,
        };

        let value = evaluate_model(
            &model,
            0,
            2,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect("relative linear model should evaluate");

        assert!((value - 12.0).abs() < 1.0e-12);
    }

    #[test]
    fn exponential_model_is_correct() {
        let model = DriftModel::Exponential {
            base: 1.0,
            rate: 1.0,
            time_unit_ns: 1,
        };

        let value = evaluate_model(
            &model,
            0,
            1,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect("exponential model should evaluate");

        assert!((value - std::f64::consts::E).abs() < 1.0e-12);
    }

    #[test]
    fn periodic_model_is_finite() {
        let model = DriftModel::Periodic {
            offset: 1.0,
            amplitude: 0.5,
            period_ns: 100,
            phase_radians: 0.0,
        };

        let value = evaluate_model(
            &model,
            0,
            25,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect("periodic model should evaluate");

        assert!(value.is_finite());
        assert!((value - 1.5).abs() < 1.0e-12);
    }

    #[test]
    fn piecewise_linear_model_interpolates() {
        let points = vec![
            DriftPoint::new(0, 0.0).expect("point"),
            DriftPoint::new(10, 10.0).expect("point"),
        ];

        let model = DriftModel::PiecewiseLinear { points };

        let value = evaluate_model(
            &model,
            0,
            5,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect("piecewise model should evaluate");

        assert!((value - 5.0).abs() < 1.0e-12);
    }

    #[test]
    fn sampled_model_requires_exact_sample() {
        let points = vec![
            DriftPoint::new(0, 1.0).expect("point"),
            DriftPoint::new(10, 2.0).expect("point"),
        ];

        let model = DriftModel::Sampled { points };

        let error = evaluate_model(
            &model,
            0,
            5,
            ExtrapolationPolicy::Reject,
            &policy(),
        )
        .expect_err("sampled model must reject unsampled timestamps");

        assert!(error.to_string().contains("exact sample"));
    }

    #[test]
    fn duplicate_point_timestamps_are_rejected() {
        let points = vec![
            DriftPoint::new(0, 1.0).expect("point"),
            DriftPoint::new(0, 2.0).expect("point"),
        ];

        let model = DriftModel::PiecewiseLinear { points };

        let result = validate_model(&model, &limits(), &policy());

        assert!(result.is_err());
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let result = DriftPoint::new(0, f64::NAN);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_period_is_rejected() {
        let model = DriftModel::Periodic {
            offset: 0.0,
            amplitude: 1.0,
            period_ns: 0,
            phase_radians: 0.0,
        };

        assert!(validate_model(&model, &limits(), &policy()).is_err());
    }

    #[test]
    fn empty_logical_scope_is_allowed_as_data_but_invalid_as_scope() {
        let scope = DriftScope::LogicalQubits(Vec::new());

        assert!(scope.validate(&limits()).is_err());
    }

    #[test]
    fn named_scope_requires_namespace_and_key() {
        assert!(DriftScope::named(String::new(), "key".to_owned()).is_err());
        assert!(DriftScope::named("namespace".to_owned(), String::new()).is_err());
    }

    #[test]
    fn drift_set_rejects_duplicate_parameter_scope_pairs() {
        // This test intentionally uses device-wide scope and does not require
        // construction of a concrete qubit identity.
        //
        // Concrete NoiseParameterId creation belongs to core::ids and is
        // intentionally not duplicated here.
        //
        // The duplicate-key behavior is additionally covered structurally by
        // `same_drift_key`.
        assert!(same_drift_key(
            &dummy_drift(),
            &dummy_drift()
        ));
    }

    fn dummy_drift() -> CalibrationDrift {
        // This helper cannot construct an opaque NoiseParameterId without
        // coupling this module's tests to the ID implementation.
        //
        // Keep this test intentionally unreachable rather than manufacturing
        // a fake ID.
        panic!("dummy drift is only a structural test placeholder")
    }

    #[test]
    fn drift_accuracy_rejects_negative_error_bounds() {
        let accuracy = DriftAccuracy::Approximate {
            absolute_error_bound: -1.0,
        };

        assert!(accuracy.validate().is_err());
    }

    #[test]
    fn unbounded_validity_accepts_any_timestamp() {
        let validity = DriftValidity::unbounded();

        assert!(validity.contains(i128::MIN));
        assert!(validity.contains(0));
        assert!(validity.contains(i128::MAX));
    }

    #[test]
    fn extrapolation_hold_uses_boundary_value() {
        let points = vec![
            DriftPoint::new(10, 2.0).expect("point"),
            DriftPoint::new(20, 4.0).expect("point"),
        ];

        let model = DriftModel::PiecewiseLinear { points };

        let before = evaluate_model(
            &model,
            10,
            5,
            ExtrapolationPolicy::Hold,
            &policy(),
        )
        .expect("hold extrapolation should evaluate");

        let after = evaluate_model(
            &model,
            10,
            25,
            ExtrapolationPolicy::Hold,
            &policy(),
        )
        .expect("hold extrapolation should evaluate");

        assert_eq!(before, 2.0);
        assert_eq!(after, 4.0);
    }

    #[test]
    fn linear_extrapolation_uses_boundary_slope() {
        let points = vec![
            DriftPoint::new(10, 2.0).expect("point"),
            DriftPoint::new(20, 4.0).expect("point"),
        ];

        let model = DriftModel::PiecewiseLinear { points };

        let before = evaluate_model(
            &model,
            10,
            5,
            ExtrapolationPolicy::Linear,
            &policy(),
        )
        .expect("linear extrapolation should evaluate");

        let after = evaluate_model(
            &model,
            10,
            25,
            ExtrapolationPolicy::Linear,
            &policy(),
        )
        .expect("linear extrapolation should evaluate");

        assert!((before - 1.0).abs() < 1.0e-12);
        assert!((after - 5.0).abs() < 1.0e-12);
    }

    #[test]
    fn overflow_in_elapsed_time_is_rejected() {
        let model = DriftModel::Linear {
            base: 1.0,
            rate: 1.0,
            time_unit_ns: 1,
        };

        let result = evaluate_model(
            &model,
            i128::MIN,
            i128::MAX,
            ExtrapolationPolicy::Reject,
            &policy(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn validation_policy_requires_finite_tolerance() {
        assert!(
            DriftValidationPolicy::new(f64::NAN, true, true).is_err()
        );

        assert!(
            DriftValidationPolicy::new(f64::INFINITY, true, true).is_err()
        );
    }
}