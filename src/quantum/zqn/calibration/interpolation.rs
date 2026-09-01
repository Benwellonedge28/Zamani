//! Zamani Quantum Noise (ZQN) — Calibration Interpolation.
//!
//! Path:
//!
//!     src/quantum/zqn/calibration/interpolation.rs
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module provides deterministic interpolation of already-observed
//! calibration values across an explicit calibration-time coordinate.
//!
//! Interpolation answers:
//!
//! > "Given validated calibration observations at explicit times, what value
//! > does the selected deterministic interpolation policy produce at another
//! > explicit time?"
//!
//! This module owns:
//!
//! - interpolation samples;
//! - deterministic sample ordering;
//! - duplicate-time validation;
//! - interpolation policies;
//! - extrapolation policies;
//! - scalar/vector/complex/matrix numerical interpolation;
//! - interpolation validation;
//! - explicit resource limits;
//! - deterministic evaluation;
//! - interpolation result provenance within the interpolation object.
//!
//! This module does NOT own:
//!
//! - calibration parameter identity semantics;
//! - calibration snapshot lifetime;
//! - calibration registries;
//! - hardware discovery;
//! - hardware credentials;
//! - calibration experiments;
//! - statistical estimation;
//! - drift laws;
//! - noise channels;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking methodology;
//! - vendor APIs;
//! - serialization formats;
//! - global mutable state;
//! - wall-clock access;
//! - random-number generation.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                         ZQN calibration
//!                              │
//!                 ┌────────────┼────────────┐
//!                 │            │            │
//!                 ▼            ▼            ▼
//!             parameter     snapshot       drift
//!                 │            │            │
//!                 └────────────┼────────────┘
//!                              ▼
//!                       interpolation
//!                              │
//!                              ▼
//!                     derived calibration
//!                              │
//!                 ┌────────────┼────────────┐
//!                 ▼            ▼            ▼
//!              noise        routing     scheduling
//! ```
//!
//! `snapshot.rs` owns immutable calibration snapshots.
//!
//! `parameter.rs` owns the calibrated value representation.
//!
//! `drift.rs` owns analytic temporal evolution/drift laws.
//!
//! This file owns interpolation of discrete observations.
//!
//! ============================================================================
//! IMPORTANT SEMANTIC DISTINCTION
//! ============================================================================
//!
//! Drift and interpolation are intentionally different.
//!
//! Drift:
//!
//!     parameter(t) = deterministic law(t)
//!
//! Interpolation:
//!
//!     observed samples → value between/around samples
//!
//! A drift model MUST NOT be implemented by hiding sampled interpolation inside
//! a drift law.
//!
//! Likewise, interpolation MUST NOT silently invent a physical drift law.
//!
//! ============================================================================
//! TIME REPRESENTATION
//! ============================================================================
//!
//! The interpolation engine uses signed `i128` nanoseconds.
//!
//! This is deliberate.
//!
//! `CalibrationTime` in `calibration/snapshot.rs` is an encapsulated domain
//! type. Its internal representation is not owned by this module. The snapshot
//! layer should expose an explicit conversion to the canonical interpolation
//! coordinate when integrating the two modules.
//!
//! The interpolation API therefore does not access private `CalibrationTime`
//! fields and does not depend on their memory layout.
//!
//! The coordinate is:
//!
//!     timestamp_ns : i128
//!
//! The representation supports:
//!
//! - dates before the Unix epoch;
//! - dates after the Unix epoch;
//! - sub-second calibration changes;
//! - very long-running systems;
//! - deterministic arithmetic.
//!
//! The module never reads the current time.
//!
//! ============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! ============================================================================
//!
//! There is NO semantic machine-size limit here.
//!
//! In particular this module does not define:
//!
//!     MAX_QUBITS
//!     MAX_CALIBRATION_SAMPLES
//!     MAX_PARAMETERS
//!     MAX_DEVICES
//!     MAX_VECTOR_SIZE
//!
//! The number of samples and the dimensionality of a value are data.
//!
//! Explicit `InterpolationLimits` exist only as caller-controlled resource
//! protection against accidental or hostile allocations/computation.
//!
//! A caller may configure those limits according to:
//!
//! - embedded hardware;
//! - desktop systems;
//! - servers;
//! - distributed execution;
//! - fuzzing;
//! - hostile-input boundaries.
//!
//! `None` means that this layer imposes no corresponding limit.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Evaluation is deterministic.
//!
//! Given identical:
//!
//! - samples;
//! - interpolation method;
//! - extrapolation policy;
//! - target timestamp;
//! - limits;
//!
//! the result is identical.
//!
//! This module:
//!
//! - does not read a clock;
//! - does not generate random numbers;
//! - does not use a global RNG;
//! - does not use global mutable state;
//! - does not perform I/O;
//! - does not depend on hash-map iteration order;
//! - does not use thread identity;
//! - does not use process identity.
//!
//! ============================================================================
//! NUMERICAL SAFETY
//! ============================================================================
//!
//! All floating-point calibration values must be finite.
//!
//! NaN and ±infinity are rejected.
//!
//! Arithmetic overflow that produces a non-finite result is rejected.
//!
//! The module never silently performs:
//!
//!     NaN → 0
//!     ∞   → max
//!     negative probability → abs()
//!
//! Interpolation is not probability normalization.
//!
//! A probability parameter that requires [0,1] validation remains the
//! responsibility of the parameter/channel validation layer.
//!
//! ============================================================================
//! APPROXIMATION CONTRACT
//! ============================================================================
//!
//! Interpolation is inherently an inferred value unless the target timestamp
//! exactly matches an observed sample.
//!
//! The result therefore records:
//!
//! - whether the result was exact;
//! - whether it was interpolated;
//! - whether it was extrapolated;
//! - which method was used;
//! - the source sample interval.
//!
//! No approximation is silently presented as an exact observation.
//!
//! ============================================================================
//! SERIALIZATION
//! ============================================================================
//!
//! This module intentionally does NOT derive Serialize/Deserialize.
//!
//! Wire-format ownership belongs to:
//!
//!     crate::quantum::zqn::io
//!
//! The serialization layer must preserve:
//!
//! - parameter identity;
//! - sample timestamps;
//! - sample values;
//! - interpolation method;
//! - extrapolation policy;
//! - revision/identity metadata supplied by the caller;
//! - deterministic sample order.
//!
//! Rust memory layout is not a serialization contract.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! Calibration observations may be untrusted.
//!
//! Validation therefore protects against:
//!
//! - non-finite floating-point values;
//! - duplicate timestamps;
//! - timestamp arithmetic overflow;
//! - excessive sample counts;
//! - excessive vector/matrix dimensions;
//! - matrix shape overflow;
//! - excessive polynomial/cubic computation;
//! - non-finite interpolation results.
//!
//! No sample contains executable code or provider credentials.
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

use crate::quantum::zqn::calibration::parameter::CalibrationValue;
use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::NoiseParameterId;

// ============================================================================
// VERSION
// ============================================================================

/// Semantic revision of this interpolation representation.
pub const CALIBRATION_INTERPOLATION_SCHEMA_VERSION: u16 = 1;

/// Number of nanoseconds in one second.
///
/// This is a unit conversion constant, not a machine-size limit.
const NANOS_PER_SECOND: i128 = 1_000_000_000;

// ============================================================================
// INTERPOLATION METHOD
// ============================================================================

/// Deterministic interpolation method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterpolationMethod {
    /// Selects the observation whose timestamp is closest to the target.
    ///
    /// If two observations are equally distant, the earlier observation wins.
    Nearest,

    /// Uses the two bracketing samples.
    ///
    /// For scalar values:
    ///
    /// `y = y0 + α(y1 - y0)`
    ///
    /// where:
    ///
    /// `α = (t - t0) / (t1 - t0)`
    ///
    /// For vectors/matrices/complex values the operation is performed
    /// component-wise.
    Linear,

    /// Holds the most recent observation at or before the target.
    ///
    /// This is appropriate for stepwise calibration semantics where a
    /// calibration remains active until replaced.
    Previous,

    /// Holds the first observation at or after the target.
    ///
    /// This is useful for forward-effective calibration schedules.
    Next,

    /// Piecewise cubic Hermite interpolation.
    ///
    /// This method is deterministic and local: each interior interval uses
    /// the interval endpoints and their estimated derivatives.
    ///
    /// Endpoint derivatives use one-sided differences. Interior derivatives
    /// use centered differences.
    ///
    /// It is not a statistical spline and does not claim physical smoothness.
    CubicHermite,
}

impl Default for InterpolationMethod {
    fn default() -> Self {
        Self::Linear
    }
}

impl fmt::Display for InterpolationMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nearest => formatter.write_str("nearest"),
            Self::Linear => formatter.write_str("linear"),
            Self::Previous => formatter.write_str("previous"),
            Self::Next => formatter.write_str("next"),
            Self::CubicHermite => formatter.write_str("cubic_hermite"),
        }
    }
}

// ============================================================================
// EXTRAPOLATION POLICY
// ============================================================================

/// Policy for targets outside the observed sample interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExtrapolationPolicy {
    /// Reject targets outside the sample interval.
    Reject,

    /// Use the nearest endpoint.
    ///
    /// This is explicit clamping, not mathematical extrapolation.
    Clamp,

    /// Extend the first/last local slope.
    Linear,

    /// Extend the cubic Hermite endpoint tangent.
    CubicHermite,
}

impl Default for ExtrapolationPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

impl fmt::Display for ExtrapolationPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject => formatter.write_str("reject"),
            Self::Clamp => formatter.write_str("clamp"),
            Self::Linear => formatter.write_str("linear"),
            Self::CubicHermite => formatter.write_str("cubic_hermite"),
        }
    }
}

// ============================================================================
// INTERPOLATION SAMPLE
// ============================================================================

/// One observed calibration value at an explicit timestamp.
///
/// The timestamp is represented as signed Unix nanoseconds.
///
/// A sample is immutable after construction.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationSample {
    timestamp_ns: i128,
    value: CalibrationValue,
}

impl CalibrationSample {
    /// Creates a calibration sample.
    ///
    /// All numerical values inside `value` must be finite.
    pub fn new(timestamp_ns: i128, value: CalibrationValue) -> ZqnResult<Self> {
        validate_calibration_value(&value)?;

        Ok(Self {
            timestamp_ns,
            value,
        })
    }

    /// Returns the timestamp in Unix nanoseconds.
    #[must_use]
    pub const fn timestamp_ns(&self) -> i128 {
        self.timestamp_ns
    }

    /// Returns the sampled calibration value.
    #[must_use]
    pub fn value(&self) -> &CalibrationValue {
        &self.value
    }
}

// ============================================================================
// INTERPOLATION LIMITS
// ============================================================================

/// Caller-controlled resource policy for interpolation.
///
/// `None` means no limit is imposed by this module.
///
/// These limits are operational safeguards, not semantic restrictions on
/// quantum-machine size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterpolationLimits {
    /// Maximum number of samples accepted.
    pub max_samples: Option<u64>,

    /// Maximum number of scalar elements in one calibration value.
    pub max_value_elements: Option<u64>,

    /// Maximum number of arithmetic components evaluated by one request.
    pub max_evaluation_elements: Option<u64>,
}

impl InterpolationLimits {
    /// Creates an unlimited interpolation policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_samples: None,
            max_value_elements: None,
            max_evaluation_elements: None,
        }
    }

    /// Validates one value against the configured limits.
    pub fn validate_value(&self, value: &CalibrationValue) -> ZqnResult<()> {
        let elements = value.element_count() as u128;

        if let Some(limit) = self.max_value_elements {
            if elements > u128::from(limit) {
                return Err(resource_limit(
                    "calibration value exceeds interpolation value-element limit",
                ));
            }
        }

        Ok(())
    }

    /// Validates a sample count.
    pub fn validate_sample_count(&self, count: usize) -> ZqnResult<()> {
        if let Some(limit) = self.max_samples {
            if (count as u128) > u128::from(limit) {
                return Err(resource_limit(
                    "calibration sample count exceeds interpolation limit",
                ));
            }
        }

        Ok(())
    }

    fn validate_evaluation_elements(&self, elements: usize) -> ZqnResult<()> {
        if let Some(limit) = self.max_evaluation_elements {
            if (elements as u128) > u128::from(limit) {
                return Err(resource_limit(
                    "interpolation evaluation exceeds configured element limit",
                ));
            }
        }

        Ok(())
    }
}

impl Default for InterpolationLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

// ============================================================================
// INTERPOLATION RESULT KIND
// ============================================================================

/// Describes how the returned value relates to observed calibration data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterpolationResultKind {
    /// Target exactly matched an observed timestamp.
    Exact,

    /// Target lies strictly between observed timestamps.
    Interpolated,

    /// Target lies outside the observed interval and the configured policy
    /// allowed evaluation.
    Extrapolated,

    /// Target lies outside the observed interval and endpoint clamping was
    /// explicitly requested.
    Clamped,
}

impl fmt::Display for InterpolationResultKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => formatter.write_str("exact"),
            Self::Interpolated => formatter.write_str("interpolated"),
            Self::Extrapolated => formatter.write_str("extrapolated"),
            Self::Clamped => formatter.write_str("clamped"),
        }
    }
}

// ============================================================================
// INTERPOLATION RESULT
// ============================================================================

/// Result of one interpolation evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolationResult {
    parameter_id: NoiseParameterId,
    target_timestamp_ns: i128,
    value: CalibrationValue,
    kind: InterpolationResultKind,
    method: InterpolationMethod,
    extrapolation: ExtrapolationPolicy,
    lower_timestamp_ns: Option<i128>,
    upper_timestamp_ns: Option<i128>,
}

impl InterpolationResult {
    /// Returns the calibration parameter being evaluated.
    #[must_use]
    pub const fn parameter_id(&self) -> NoiseParameterId {
        self.parameter_id
    }

    /// Returns the requested target timestamp.
    #[must_use]
    pub const fn target_timestamp_ns(&self) -> i128 {
        self.target_timestamp_ns
    }

    /// Returns the resulting calibration value.
    #[must_use]
    pub fn value(&self) -> &CalibrationValue {
        &self.value
    }

    /// Returns the result classification.
    #[must_use]
    pub const fn kind(&self) -> InterpolationResultKind {
        self.kind
    }

    /// Returns the interpolation method.
    #[must_use]
    pub const fn method(&self) -> InterpolationMethod {
        self.method
    }

    /// Returns the extrapolation policy.
    #[must_use]
    pub const fn extrapolation(&self) -> ExtrapolationPolicy {
        self.extrapolation
    }

    /// Returns the lower source timestamp when applicable.
    #[must_use]
    pub const fn lower_timestamp_ns(&self) -> Option<i128> {
        self.lower_timestamp_ns
    }

    /// Returns the upper source timestamp when applicable.
    #[must_use]
    pub const fn upper_timestamp_ns(&self) -> Option<i128> {
        self.upper_timestamp_ns
    }
}

// ============================================================================
// INTERPOLATOR
// ============================================================================

/// Immutable deterministic calibration interpolator.
///
/// Samples are normalized into strictly increasing timestamp order during
/// construction.
///
/// The interpolator owns no mutable calibration registry and therefore can be
/// safely shared by concurrent readers when the contained `CalibrationValue`
/// types are used in thread-safe contexts.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationInterpolator {
    parameter_id: NoiseParameterId,
    samples: Vec<CalibrationSample>,
    method: InterpolationMethod,
    extrapolation: ExtrapolationPolicy,
}

impl CalibrationInterpolator {
    /// Creates an interpolator from calibration observations.
    ///
    /// Sample timestamps must be unique.
    ///
    /// Input ordering is not semantically significant; the constructor creates
    /// deterministic timestamp order.
    pub fn new(
        parameter_id: NoiseParameterId,
        mut samples: Vec<CalibrationSample>,
        method: InterpolationMethod,
        extrapolation: ExtrapolationPolicy,
        limits: &InterpolationLimits,
    ) -> ZqnResult<Self> {
        limits.validate_sample_count(samples.len())?;

        if samples.is_empty() {
            return Err(invalid_calibration(
                "interpolation requires at least one calibration sample",
            ));
        }

        for sample in &samples {
            limits.validate_value(sample.value())?;
            validate_calibration_value(sample.value())?;
        }

        samples.sort_by_key(CalibrationSample::timestamp_ns);

        for window in samples.windows(2) {
            if window[0].timestamp_ns() == window[1].timestamp_ns() {
                return Err(invalid_calibration(
                    "interpolation samples contain duplicate timestamps",
                ));
            }

            ensure_compatible_values(window[0].value(), window[1].value())?;
        }

        if samples.len() == 1
            && matches!(
                method,
                InterpolationMethod::Linear | InterpolationMethod::CubicHermite
            )
            && matches!(
                extrapolation,
                ExtrapolationPolicy::Linear
                    | ExtrapolationPolicy::CubicHermite
            )
        {
            // A single point cannot define a slope.
            return Err(invalid_calibration(
                "the selected interpolation/extrapolation policy requires at least two distinct calibration samples",
            ));
        }

        Ok(Self {
            parameter_id,
            samples,
            method,
            extrapolation,
        })
    }

    /// Returns the calibration parameter identity.
    #[must_use]
    pub const fn parameter_id(&self) -> NoiseParameterId {
        self.parameter_id
    }

    /// Returns the configured interpolation method.
    #[must_use]
    pub const fn method(&self) -> InterpolationMethod {
        self.method
    }

    /// Returns the configured extrapolation policy.
    #[must_use]
    pub const fn extrapolation(&self) -> ExtrapolationPolicy {
        self.extrapolation
    }

    /// Returns the number of observations.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Returns the immutable sample slice in canonical timestamp order.
    #[must_use]
    pub fn samples(&self) -> &[CalibrationSample] {
        &self.samples
    }

    /// Returns the first observation.
    #[must_use]
    pub fn first_sample(&self) -> &CalibrationSample {
        &self.samples[0]
    }

    /// Returns the last observation.
    #[must_use]
    pub fn last_sample(&self) -> &CalibrationSample {
        &self.samples[self.samples.len() - 1]
    }

    /// Evaluates the calibration at an explicit timestamp.
    pub fn evaluate(
        &self,
        target_timestamp_ns: i128,
        limits: &InterpolationLimits,
    ) -> ZqnResult<InterpolationResult> {
        if self.samples.is_empty() {
            return Err(invalid_calibration(
                "interpolator contains no calibration samples",
            ));
        }

        for sample in &self.samples {
            limits.validate_value(sample.value())?;
        }

        let first = self.first_sample();
        let last = self.last_sample();

        if target_timestamp_ns == first.timestamp_ns() {
            return Ok(InterpolationResult {
                parameter_id: self.parameter_id,
                target_timestamp_ns,
                value: first.value().clone(),
                kind: InterpolationResultKind::Exact,
                method: self.method,
                extrapolation: self.extrapolation,
                lower_timestamp_ns: Some(first.timestamp_ns()),
                upper_timestamp_ns: Some(first.timestamp_ns()),
            });
        }

        if target_timestamp_ns == last.timestamp_ns() {
            return Ok(InterpolationResult {
                parameter_id: self.parameter_id,
                target_timestamp_ns,
                value: last.value().clone(),
                kind: InterpolationResultKind::Exact,
                method: self.method,
                extrapolation: self.extrapolation,
                lower_timestamp_ns: Some(last.timestamp_ns()),
                upper_timestamp_ns: Some(last.timestamp_ns()),
            });
        }

        if target_timestamp_ns < first.timestamp_ns() {
            return self.evaluate_before_first(target_timestamp_ns, limits);
        }

        if target_timestamp_ns > last.timestamp_ns() {
            return self.evaluate_after_last(target_timestamp_ns, limits);
        }

        let upper_index = self
            .samples
            .binary_search_by_key(
                &target_timestamp_ns,
                CalibrationSample::timestamp_ns,
            )
            .unwrap_or_else(|index| index);

        if upper_index == 0 || upper_index >= self.samples.len() {
            return Err(numerical_failure(
                "failed to locate interpolation interval",
            ));
        }

        let lower = &self.samples[upper_index - 1];
        let upper = &self.samples[upper_index];

        let value = self.interpolate_between(lower, upper, target_timestamp_ns, limits)?;

        Ok(InterpolationResult {
            parameter_id: self.parameter_id,
            target_timestamp_ns,
            value,
            kind: InterpolationResultKind::Interpolated,
            method: self.method,
            extrapolation: self.extrapolation,
            lower_timestamp_ns: Some(lower.timestamp_ns()),
            upper_timestamp_ns: Some(upper.timestamp_ns()),
        })
    }

    fn evaluate_before_first(
        &self,
        target_timestamp_ns: i128,
        limits: &InterpolationLimits,
    ) -> ZqnResult<InterpolationResult> {
        match self.extrapolation {
            ExtrapolationPolicy::Reject => Err(invalid_calibration(
                "target timestamp is before the first calibration sample and extrapolation is disabled",
            )),

            ExtrapolationPolicy::Clamp => Ok(InterpolationResult {
                parameter_id: self.parameter_id,
                target_timestamp_ns,
                value: self.first_sample().value().clone(),
                kind: InterpolationResultKind::Clamped,
                method: self.method,
                extrapolation: self.extrapolation,
                lower_timestamp_ns: Some(self.first_sample().timestamp_ns()),
                upper_timestamp_ns: None,
            }),

            ExtrapolationPolicy::Linear => {
                if self.samples.len() < 2 {
                    return Err(invalid_calibration(
                        "linear extrapolation requires at least two calibration samples",
                    ));
                }

                let lower = &self.samples[0];
                let upper = &self.samples[1];

                let value = linear_extrapolate(
                    lower,
                    upper,
                    target_timestamp_ns,
                    limits,
                )?;

                Ok(InterpolationResult {
                    parameter_id: self.parameter_id,
                    target_timestamp_ns,
                    value,
                    kind: InterpolationResultKind::Extrapolated,
                    method: self.method,
                    extrapolation: self.extrapolation,
                    lower_timestamp_ns: Some(lower.timestamp_ns()),
                    upper_timestamp_ns: Some(upper.timestamp_ns()),
                })
            }

            ExtrapolationPolicy::CubicHermite => {
                if self.samples.len() < 2 {
                    return Err(invalid_calibration(
                        "cubic-Hermite extrapolation requires at least two calibration samples",
                    ));
                }

                let value = self.cubic_hermite_extrapolate(
                    target_timestamp_ns,
                    true,
                    limits,
                )?;

                Ok(InterpolationResult {
                    parameter_id: self.parameter_id,
                    target_timestamp_ns,
                    value,
                    kind: InterpolationResultKind::Extrapolated,
                    method: self.method,
                    extrapolation: self.extrapolation,
                    lower_timestamp_ns: Some(self.samples[0].timestamp_ns()),
                    upper_timestamp_ns: Some(self.samples[1].timestamp_ns()),
                })
            }
        }
    }

    fn evaluate_after_last(
        &self,
        target_timestamp_ns: i128,
        limits: &InterpolationLimits,
    ) -> ZqnResult<InterpolationResult> {
        match self.extrapolation {
            ExtrapolationPolicy::Reject => Err(invalid_calibration(
                "target timestamp is after the last calibration sample and extrapolation is disabled",
            )),

            ExtrapolationPolicy::Clamp => Ok(InterpolationResult {
                parameter_id: self.parameter_id,
                target_timestamp_ns,
                value: self.last_sample().value().clone(),
                kind: InterpolationResultKind::Clamped,
                method: self.method,
                extrapolation: self.extrapolation,
                lower_timestamp_ns: None,
                upper_timestamp_ns: Some(self.last_sample().timestamp_ns()),
            }),

            ExtrapolationPolicy::Linear => {
                if self.samples.len() < 2 {
                    return Err(invalid_calibration(
                        "linear extrapolation requires at least two calibration samples",
                    ));
                }

                let lower = &self.samples[self.samples.len() - 2];
                let upper = &self.samples[self.samples.len() - 1];

                let value = linear_extrapolate(
                    lower,
                    upper,
                    target_timestamp_ns,
                    limits,
                )?;

                Ok(InterpolationResult {
                    parameter_id: self.parameter_id,
                    target_timestamp_ns,
                    value,
                    kind: InterpolationResultKind::Extrapolated,
                    method: self.method,
                    extrapolation: self.extrapolation,
                    lower_timestamp_ns: Some(lower.timestamp_ns()),
                    upper_timestamp_ns: Some(upper.timestamp_ns()),
                })
            }

            ExtrapolationPolicy::CubicHermite => {
                if self.samples.len() < 2 {
                    return Err(invalid_calibration(
                        "cubic-Hermite extrapolation requires at least two calibration samples",
                    ));
                }

                let value = self.cubic_hermite_extrapolate(
                    target_timestamp_ns,
                    false,
                    limits,
                )?;

                let len = self.samples.len();

                Ok(InterpolationResult {
                    parameter_id: self.parameter_id,
                    target_timestamp_ns,
                    value,
                    kind: InterpolationResultKind::Extrapolated,
                    method: self.method,
                    extrapolation: self.extrapolation,
                    lower_timestamp_ns: Some(self.samples[len - 2].timestamp_ns()),
                    upper_timestamp_ns: Some(self.samples[len - 1].timestamp_ns()),
                })
            }
        }
    }

    fn interpolate_between(
        &self,
        lower: &CalibrationSample,
        upper: &CalibrationSample,
        target_timestamp_ns: i128,
        limits: &InterpolationLimits,
    ) -> ZqnResult<CalibrationValue> {
        match self.method {
            InterpolationMethod::Nearest => {
                let lower_distance = absolute_timestamp_distance(
                    target_timestamp_ns,
                    lower.timestamp_ns(),
                )?;

                let upper_distance = absolute_timestamp_distance(
                    upper.timestamp_ns(),
                    target_timestamp_ns,
                )?;

                if lower_distance <= upper_distance {
                    Ok(lower.value().clone())
                } else {
                    Ok(upper.value().clone())
                }
            }

            InterpolationMethod::Previous => Ok(lower.value().clone()),

            InterpolationMethod::Next => Ok(upper.value().clone()),

            InterpolationMethod::Linear => {
                let alpha = interpolation_fraction(
                    lower.timestamp_ns(),
                    upper.timestamp_ns(),
                    target_timestamp_ns,
                )?;

                interpolate_values(
                    lower.value(),
                    upper.value(),
                    alpha,
                    limits,
                )
            }

            InterpolationMethod::CubicHermite => {
                self.cubic_hermite_between(
                    lower,
                    upper,
                    target_timestamp_ns,
                    limits,
                )
            }
        }
    }

    fn cubic_hermite_between(
        &self,
        lower: &CalibrationSample,
        upper: &CalibrationSample,
        target_timestamp_ns: i128,
        limits: &InterpolationLimits,
    ) -> ZqnResult<CalibrationValue> {
        let lower_index = self
            .samples
            .binary_search_by_key(
                &lower.timestamp_ns(),
                CalibrationSample::timestamp_ns,
            )
            .map_err(|_| numerical_failure("lower interpolation sample is not present"))?;

        let upper_index = self
            .samples
            .binary_search_by_key(
                &upper.timestamp_ns(),
                CalibrationSample::timestamp_ns,
            )
            .map_err(|_| numerical_failure("upper interpolation sample is not present"))?;

        let left_derivative = self.derivative_at(lower_index)?;
        let right_derivative = self.derivative_at(upper_index)?;

        let dt = timestamp_delta_seconds(
            lower.timestamp_ns(),
            upper.timestamp_ns(),
        )?;

        let u = interpolation_fraction(
            lower.timestamp_ns(),
            upper.timestamp_ns(),
            target_timestamp_ns,
        )?;

        let h00 = 2.0 * u * u * u - 3.0 * u * u + 1.0;
        let h10 = u * u * u - 2.0 * u * u + u;
        let h01 = -2.0 * u * u * u + 3.0 * u * u;
        let h11 = u * u * u - u * u;

        hermite_values(
            lower.value(),
            upper.value(),
            &left_derivative,
            &right_derivative,
            h00,
            h10 * dt,
            h01,
            h11 * dt,
            limits,
        )
    }

    fn cubic_hermite_extrapolate(
        &self,
        target_timestamp_ns: i128,
        before: bool,
        limits: &InterpolationLimits,
    ) -> ZqnResult<CalibrationValue> {
        if self.samples.len() < 2 {
            return Err(invalid_calibration(
                "cubic-Hermite extrapolation requires at least two samples",
            ));
        }

        if before {
            let left = &self.samples[0];
            let right = &self.samples[1];

            let derivative = self.derivative_at(0)?;
            let dt = timestamp_delta_seconds(
                left.timestamp_ns(),
                right.timestamp_ns(),
            )?;

            let u = interpolation_fraction(
                left.timestamp_ns(),
                right.timestamp_ns(),
                target_timestamp_ns,
            )?;

            let h00 = 2.0 * u * u * u - 3.0 * u * u + 1.0;
            let h10 = u * u * u - 2.0 * u * u + u;
            let h01 = -2.0 * u * u * u + 3.0 * u * u;
            let h11 = u * u * u - u * u;

            return hermite_values(
                left.value(),
                right.value(),
                &derivative,
                &self.derivative_at(1)?,
                h00,
                h10 * dt,
                h01,
                h11 * dt,
                limits,
            );
        }

        let len = self.samples.len();
        let left = &self.samples[len - 2];
        let right = &self.samples[len - 1];

        let dt = timestamp_delta_seconds(
            left.timestamp_ns(),
            right.timestamp_ns(),
        )?;

        let u = interpolation_fraction(
            left.timestamp_ns(),
            right.timestamp_ns(),
            target_timestamp_ns,
        )?;

        let h00 = 2.0 * u * u * u - 3.0 * u * u + 1.0;
        let h10 = u * u * u - 2.0 * u * u + u;
        let h01 = -2.0 * u * u * u + 3.0 * u * u;
        let h11 = u * u * u - u * u;

        hermite_values(
            left.value(),
            right.value(),
            &self.derivative_at(len - 2)?,
            &self.derivative_at(len - 1)?,
            h00,
            h10 * dt,
            h01,
            h11 * dt,
            limits,
        )
    }

    fn derivative_at(&self, index: usize) -> ZqnResult<CalibrationValue> {
        if index >= self.samples.len() {
            return Err(numerical_failure(
                "derivative index is outside interpolation sample range",
            ));
        }

        if self.samples.len() < 2 {
            return Err(invalid_calibration(
                "derivative estimation requires at least two samples",
            ));
        }

        if index == 0 {
            return value_difference_rate(
                self.samples[0].value(),
                self.samples[1].value(),
                self.samples[0].timestamp_ns(),
                self.samples[1].timestamp_ns(),
            );
        }

        if index == self.samples.len() - 1 {
            let last = self.samples.len() - 1;

            return value_difference_rate(
                self.samples[last - 1].value(),
                self.samples[last].value(),
                self.samples[last - 1].timestamp_ns(),
                self.samples[last].timestamp_ns(),
            );
        }

        value_difference_rate(
            self.samples[index - 1].value(),
            self.samples[index + 1].value(),
            self.samples[index - 1].timestamp_ns(),
            self.samples[index + 1].timestamp_ns(),
        )
    }
}

// ============================================================================
// NUMERICAL VALUE OPERATIONS
// ============================================================================

fn interpolate_values(
    lower: &CalibrationValue,
    upper: &CalibrationValue,
    alpha: f64,
    limits: &InterpolationLimits,
) -> ZqnResult<CalibrationValue> {
    if !alpha.is_finite() {
        return Err(numerical_failure(
            "interpolation fraction is not finite",
        ));
    }

    ensure_compatible_values(lower, upper)?;

    match (lower, upper) {
        (CalibrationValue::Scalar(a), CalibrationValue::Scalar(b)) => {
            limits.validate_evaluation_elements(1)?;
            Ok(CalibrationValue::Scalar(interpolate_scalar(
                *a, *b, alpha,
            )?))
        }

        (CalibrationValue::Integer(a), CalibrationValue::Integer(b)) => {
            limits.validate_evaluation_elements(1)?;

            let a = *a as f64;
            let b = *b as f64;

            let value = interpolate_scalar(a, b, alpha)?;

            if !value.is_finite() {
                return Err(numerical_failure(
                    "integer interpolation produced a non-finite result",
                ));
            }

            Ok(CalibrationValue::Scalar(value))
        }

        (
            CalibrationValue::Complex {
                real: ar,
                imaginary: ai,
            },
            CalibrationValue::Complex {
                real: br,
                imaginary: bi,
            },
        ) => {
            limits.validate_evaluation_elements(2)?;

            Ok(CalibrationValue::Complex {
                real: interpolate_scalar(*ar, *br, alpha)?,
                imaginary: interpolate_scalar(*ai, *bi, alpha)?,
            })
        }

        (CalibrationValue::Vector(a), CalibrationValue::Vector(b)) => {
            if a.len() != b.len() {
                return Err(invalid_calibration(
                    "cannot interpolate vectors with different lengths",
                ));
            }

            limits.validate_evaluation_elements(a.len())?;

            let mut values = Vec::with_capacity(a.len());

            for (left, right) in a.iter().zip(b.iter()) {
                values.push(interpolate_scalar(*left, *right, alpha)?);
            }

            CalibrationValue::vector(values)
                .map_err(|_| invalid_calibration("invalid interpolated vector"))
        }

        (
            CalibrationValue::ComplexVector(a),
            CalibrationValue::ComplexVector(b),
        ) => {
            if a.len() != b.len() {
                return Err(invalid_calibration(
                    "cannot interpolate complex vectors with different lengths",
                ));
            }

            limits.validate_evaluation_elements(a.len())?;

            let mut values = Vec::with_capacity(a.len());

            for (left, right) in a.iter().zip(b.iter()) {
                values.push(interpolate_scalar(*left, *right, alpha)?);
            }

            CalibrationValue::complex_vector(values)
                .map_err(|_| invalid_calibration("invalid interpolated complex vector"))
        }

        (
            CalibrationValue::Matrix {
                rows: ar,
                columns: ac,
                values: av,
            },
            CalibrationValue::Matrix {
                rows: br,
                columns: bc,
                values: bv,
            },
        ) => {
            if ar != br || ac != bc {
                return Err(invalid_calibration(
                    "cannot interpolate matrices with different shapes",
                ));
            }

            limits.validate_evaluation_elements(av.len())?;

            let mut values = Vec::with_capacity(av.len());

            for (left, right) in av.iter().zip(bv.iter()) {
                values.push(interpolate_scalar(*left, *right, alpha)?);
            }

            CalibrationValue::matrix(*ar, *ac, values)
                .map_err(|_| invalid_calibration("invalid interpolated matrix"))
        }

        _ => Err(invalid_calibration(
            "the selected calibration values are not numerically interpolatable",
        )),
    }
}

fn hermite_values(
    lower: &CalibrationValue,
    upper: &CalibrationValue,
    lower_derivative: &CalibrationValue,
    upper_derivative: &CalibrationValue,
    h00: f64,
    h10: f64,
    h01: f64,
    h11: f64,
    limits: &InterpolationLimits,
) -> ZqnResult<CalibrationValue> {
    if !h00.is_finite()
        || !h10.is_finite()
        || !h01.is_finite()
        || !h11.is_finite()
    {
        return Err(numerical_failure(
            "Hermite basis coefficient is not finite",
        ));
    }

    match (
        lower,
        upper,
        lower_derivative,
        upper_derivative,
    ) {
        (
            CalibrationValue::Scalar(a),
            CalibrationValue::Scalar(b),
            CalibrationValue::Scalar(da),
            CalibrationValue::Scalar(db),
        ) => {
            limits.validate_evaluation_elements(1)?;

            let value = h00 * *a
                + h10 * *da
                + h01 * *b
                + h11 * *db;

            finite_scalar(value)
        }

        (
            CalibrationValue::Complex {
                real: ar,
                imaginary: ai,
            },
            CalibrationValue::Complex {
                real: br,
                imaginary: bi,
            },
            CalibrationValue::Complex {
                real: dar,
                imaginary: dai,
            },
            CalibrationValue::Complex {
                real: dbr,
                imaginary: dbi,
            },
        ) => {
            limits.validate_evaluation_elements(2)?;

            Ok(CalibrationValue::Complex {
                real: finite_f64(
                    h00 * *ar + h10 * *dar + h01 * *br + h11 * *dbr,
                    "Hermite complex real result",
                )?,
                imaginary: finite_f64(
                    h00 * *ai + h10 * *dai + h01 * *bi + h11 * *dbi,
                    "Hermite complex imaginary result",
                )?,
            })
        }

        (
            CalibrationValue::Vector(a),
            CalibrationValue::Vector(b),
            CalibrationValue::Vector(da),
            CalibrationValue::Vector(db),
        ) => {
            if a.len() != b.len()
                || a.len() != da.len()
                || a.len() != db.len()
            {
                return Err(invalid_calibration(
                    "Hermite vector operands have incompatible lengths",
                ));
            }

            limits.validate_evaluation_elements(a.len())?;

            let mut values = Vec::with_capacity(a.len());

            for (((left, right), dleft), dright) in a
                .iter()
                .zip(b.iter())
                .zip(da.iter())
                .zip(db.iter())
            {
                values.push(finite_f64(
                    h00 * *left
                        + h10 * *dleft
                        + h01 * *right
                        + h11 * *dright,
                    "Hermite vector result",
                )?);
            }

            CalibrationValue::vector(values)
                .map_err(|_| invalid_calibration("invalid Hermite vector result"))
        }

        (
            CalibrationValue::ComplexVector(a),
            CalibrationValue::ComplexVector(b),
            CalibrationValue::ComplexVector(da),
            CalibrationValue::ComplexVector(db),
        ) => {
            if a.len() != b.len()
                || a.len() != da.len()
                || a.len() != db.len()
            {
                return Err(invalid_calibration(
                    "Hermite complex-vector operands have incompatible lengths",
                ));
            }

            limits.validate_evaluation_elements(a.len())?;

            let mut values = Vec::with_capacity(a.len());

            for (((left, right), dleft), dright) in a
                .iter()
                .zip(b.iter())
                .zip(da.iter())
                .zip(db.iter())
            {
                values.push(finite_f64(
                    h00 * *left
                        + h10 * *dleft
                        + h01 * *right
                        + h11 * *dright,
                    "Hermite complex-vector result",
                )?);
            }

            CalibrationValue::complex_vector(values)
                .map_err(|_| invalid_calibration("invalid Hermite complex-vector result"))
        }

        (
            CalibrationValue::Matrix {
                rows: ar,
                columns: ac,
                values: av,
            },
            CalibrationValue::Matrix {
                rows: br,
                columns: bc,
                values: bv,
            },
            CalibrationValue::Matrix {
                rows: dar,
                columns: dac,
                values: dav,
            },
            CalibrationValue::Matrix {
                rows: dbr,
                columns: dbc,
                values: dbv,
            },
        ) => {
            if ar != br
                || ac != bc
                || ar != dar
                || ac != dac
                || ar != dbr
                || ac != dbc
            {
                return Err(invalid_calibration(
                    "Hermite matrix operands have incompatible shapes",
                ));
            }

            limits.validate_evaluation_elements(av.len())?;

            let mut values = Vec::with_capacity(av.len());

            for (((left, right), dleft), dright) in av
                .iter()
                .zip(bv.iter())
                .zip(dav.iter())
                .zip(dbv.iter())
            {
                values.push(finite_f64(
                    h00 * *left
                        + h10 * *dleft
                        + h01 * *right
                        + h11 * *dright,
                    "Hermite matrix result",
                )?);
            }

            CalibrationValue::matrix(*ar, *ac, values)
                .map_err(|_| invalid_calibration("invalid Hermite matrix result"))
        }

        _ => Err(invalid_calibration(
            "Hermite interpolation requires numerically compatible calibration values",
        )),
    }
}

fn value_difference_rate(
    lower: &CalibrationValue,
    upper: &CalibrationValue,
    lower_timestamp_ns: i128,
    upper_timestamp_ns: i128,
) -> ZqnResult<CalibrationValue> {
    let dt = timestamp_delta_seconds(
        lower_timestamp_ns,
        upper_timestamp_ns,
    )?;

    if dt == 0.0 {
        return Err(invalid_calibration(
            "cannot estimate a derivative from identical timestamps",
        ));
    }

    divide_value_difference(
        subtract_values(upper, lower)?,
        dt,
    )
}

fn subtract_values(
    upper: &CalibrationValue,
    lower: &CalibrationValue,
) -> ZqnResult<CalibrationValue> {
    ensure_compatible_values(upper, lower)?;

    match (upper, lower) {
        (CalibrationValue::Scalar(a), CalibrationValue::Scalar(b)) => {
            finite_scalar(*a - *b)
        }

        (
            CalibrationValue::Complex {
                real: ar,
                imaginary: ai,
            },
            CalibrationValue::Complex {
                real: br,
                imaginary: bi,
            },
        ) => Ok(CalibrationValue::Complex {
            real: finite_f64(*ar - *br, "complex real difference")?,
            imaginary: finite_f64(
                *ai - *bi,
                "complex imaginary difference",
            )?,
        }),

        (CalibrationValue::Vector(a), CalibrationValue::Vector(b)) => {
            let mut result = Vec::with_capacity(a.len());

            for (left, right) in a.iter().zip(b.iter()) {
                result.push(finite_f64(
                    *left - *right,
                    "vector difference",
                )?);
            }

            CalibrationValue::vector(result)
                .map_err(|_| invalid_calibration("invalid vector difference"))
        }

        (
            CalibrationValue::ComplexVector(a),
            CalibrationValue::ComplexVector(b),
        ) => {
            let mut result = Vec::with_capacity(a.len());

            for (left, right) in a.iter().zip(b.iter()) {
                result.push(finite_f64(
                    *left - *right,
                    "complex-vector difference",
                )?);
            }

            CalibrationValue::complex_vector(result)
                .map_err(|_| invalid_calibration("invalid complex-vector difference"))
        }

        (
            CalibrationValue::Matrix {
                rows: ar,
                columns: ac,
                values: av,
            },
            CalibrationValue::Matrix {
                rows: br,
                columns: bc,
                values: bv,
            },
        ) => {
            if ar != br || ac != bc {
                return Err(invalid_calibration(
                    "cannot subtract matrices with different shapes",
                ));
            }

            let mut result = Vec::with_capacity(av.len());

            for (left, right) in av.iter().zip(bv.iter()) {
                result.push(finite_f64(
                    *left - *right,
                    "matrix difference",
                )?);
            }

            CalibrationValue::matrix(*ar, *ac, result)
                .map_err(|_| invalid_calibration("invalid matrix difference"))
        }

        _ => Err(invalid_calibration(
            "calibration values cannot be subtracted for derivative estimation",
        )),
    }
}

fn divide_value_difference(
    value: CalibrationValue,
    divisor: f64,
) -> ZqnResult<CalibrationValue> {
    if !divisor.is_finite() || divisor == 0.0 {
        return Err(numerical_failure(
            "invalid derivative divisor",
        ));
    }

    match value {
        CalibrationValue::Scalar(value) => {
            finite_scalar(value / divisor)
        }

        CalibrationValue::Complex { real, imaginary } => {
            Ok(CalibrationValue::Complex {
                real: finite_f64(
                    real / divisor,
                    "complex derivative real component",
                )?,
                imaginary: finite_f64(
                    imaginary / divisor,
                    "complex derivative imaginary component",
                )?,
            })
        }

        CalibrationValue::Vector(values) => {
            let mut result = Vec::with_capacity(values.len());

            for value in values {
                result.push(finite_f64(
                    value / divisor,
                    "vector derivative",
                )?);
            }

            CalibrationValue::vector(result)
                .map_err(|_| invalid_calibration("invalid vector derivative"))
        }

        CalibrationValue::ComplexVector(values) => {
            let mut result = Vec::with_capacity(values.len());

            for value in values {
                result.push(finite_f64(
                    value / divisor,
                    "complex-vector derivative",
                )?);
            }

            CalibrationValue::complex_vector(result)
                .map_err(|_| invalid_calibration("invalid complex-vector derivative"))
        }

        CalibrationValue::Matrix {
            rows,
            columns,
            values,
        } => {
            let mut result = Vec::with_capacity(values.len());

            for value in values {
                result.push(finite_f64(
                    value / divisor,
                    "matrix derivative",
                )?);
            }

            CalibrationValue::matrix(rows, columns, result)
                .map_err(|_| invalid_calibration("invalid matrix derivative"))
        }

        _ => Err(invalid_calibration(
            "calibration value has no numerical derivative representation",
        )),
    }
}

fn linear_extrapolate(
    lower: &CalibrationSample,
    upper: &CalibrationSample,
    target_timestamp_ns: i128,
    limits: &InterpolationLimits,
) -> ZqnResult<CalibrationValue> {
    let alpha = interpolation_fraction(
        lower.timestamp_ns(),
        upper.timestamp_ns(),
        target_timestamp_ns,
    )?;

    interpolate_values(
        lower.value(),
        upper.value(),
        alpha,
        limits,
    )
}

fn interpolate_scalar(
    lower: f64,
    upper: f64,
    alpha: f64,
) -> ZqnResult<f64> {
    ensure_finite(lower, "lower interpolation value")?;
    ensure_finite(upper, "upper interpolation value")?;

    // This formulation is numerically preferable to:
    //
    //     lower + alpha * (upper - lower)
    //
    // when lower and upper have large similar magnitudes.
    let value = lower * (1.0 - alpha) + upper * alpha;

    finite_f64(value, "interpolated scalar")
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_calibration_value(value: &CalibrationValue) -> ZqnResult<()> {
    if !value.is_finite() {
        return Err(ZqnError::non_finite_value(
            "calibration interpolation received a non-finite calibration value",
        ));
    }

    match value {
        CalibrationValue::Matrix {
            rows,
            columns,
            values,
        } => {
            let expected = rows.checked_mul(*columns).ok_or_else(|| {
                invalid_calibration(
                    "calibration matrix dimensions overflow usize",
                )
            })?;

            if expected != values.len() {
                return Err(invalid_calibration(
                    "calibration matrix shape does not match its value count",
                ));
            }
        }

        CalibrationValue::ComplexVector(values) => {
            if values.len() % 2 != 0 {
                return Err(invalid_calibration(
                    "complex calibration vector must contain an even number of scalar components",
                ));
            }
        }

        CalibrationValue::Text(value) => {
            if value.is_empty() {
                return Err(invalid_calibration(
                    "empty textual calibration values are not interpolatable",
                ));
            }
        }

        CalibrationValue::Structured { schema, .. } => {
            if schema.trim().is_empty() {
                return Err(invalid_calibration(
                    "structured calibration value requires a non-empty schema identifier",
                ));
            }
        }

        CalibrationValue::Scalar(_)
        | CalibrationValue::Integer(_)
        | CalibrationValue::Boolean(_)
        | CalibrationValue::Vector(_)
        | CalibrationValue::Complex { .. } => {}
    }

    Ok(())
}

fn ensure_compatible_values(
    left: &CalibrationValue,
    right: &CalibrationValue,
) -> ZqnResult<()> {
    match (left, right) {
        (CalibrationValue::Scalar(_), CalibrationValue::Scalar(_))
        | (CalibrationValue::Integer(_), CalibrationValue::Integer(_))
        | (CalibrationValue::Boolean(_), CalibrationValue::Boolean(_))
        | (CalibrationValue::Text(_), CalibrationValue::Text(_))
        | (CalibrationValue::Vector(_), CalibrationValue::Vector(_))
        | (
            CalibrationValue::Complex { .. },
            CalibrationValue::Complex { .. },
        )
        | (
            CalibrationValue::ComplexVector(_),
            CalibrationValue::ComplexVector(_),
        )
        | (
            CalibrationValue::Structured { .. },
            CalibrationValue::Structured { .. },
        ) => Ok(()),

        (
            CalibrationValue::Matrix {
                rows: lr,
                columns: lc,
                ..
            },
            CalibrationValue::Matrix {
                rows: rr,
                columns: rc,
                ..
            },
        ) if lr == rr && lc == rc => Ok(()),

        _ => Err(invalid_calibration(
            "calibration samples contain incompatible value representations",
        )),
    }
}

// ============================================================================
// TIME ARITHMETIC
// ============================================================================

fn timestamp_delta_seconds(
    start_ns: i128,
    end_ns: i128,
) -> ZqnResult<f64> {
    let delta = end_ns.checked_sub(start_ns).ok_or_else(|| {
        numerical_failure("calibration timestamp subtraction overflowed")
    })?;

    let seconds = delta / NANOS_PER_SECOND;
    let remainder = delta % NANOS_PER_SECOND;

    let seconds_f64 = seconds as f64;
    let remainder_f64 = remainder as f64 / NANOS_PER_SECOND as f64;

    let result = seconds_f64 + remainder_f64;

    finite_f64(result, "calibration time delta")
}

fn absolute_timestamp_distance(
    left_ns: i128,
    right_ns: i128,
) -> ZqnResult<i128> {
    left_ns
        .checked_sub(right_ns)
        .map(|value| value.abs())
        .ok_or_else(|| {
            numerical_failure(
                "absolute calibration timestamp distance overflowed",
            )
        })
}

fn interpolation_fraction(
    lower_ns: i128,
    upper_ns: i128,
    target_ns: i128,
) -> ZqnResult<f64> {
    let denominator = timestamp_delta_seconds(lower_ns, upper_ns)?;

    if denominator <= 0.0 {
        return Err(invalid_calibration(
            "interpolation sample timestamps must be strictly increasing",
        ));
    }

    let numerator = timestamp_delta_seconds(lower_ns, target_ns)?;

    let alpha = numerator / denominator;

    finite_f64(alpha, "interpolation fraction")
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn invalid_calibration(message: impl Into<String>) -> ZqnError {
    ZqnError::invalid_calibration(message.into())
}

fn numerical_failure(message: impl Into<String>) -> ZqnError {
    ZqnError::numerical_failure(message.into())
}

fn resource_limit(message: impl Into<String>) -> ZqnError {
    ZqnError::resource_limit_exceeded(message.into())
}

fn finite_scalar(value: f64) -> ZqnResult<CalibrationValue> {
    Ok(CalibrationValue::Scalar(finite_f64(
        value,
        "calibration interpolation result",
    )?))
}

fn finite_f64(value: f64, context: &str) -> ZqnResult<f64> {
    if !value.is_finite() {
        return Err(ZqnError::non_finite_value(context));
    }

    Ok(value)
}

fn ensure_finite(value: f64, context: &str) -> ZqnResult<()> {
    if !value.is_finite() {
        return Err(ZqnError::non_finite_value(context));
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn scalar_sample(timestamp_ns: i128, value: f64) -> CalibrationSample {
        CalibrationSample::new(
            timestamp_ns,
            CalibrationValue::scalar(value).expect("finite scalar"),
        )
        .expect("valid calibration sample")
    }

    fn interpolator(
        samples: Vec<CalibrationSample>,
        method: InterpolationMethod,
        extrapolation: ExtrapolationPolicy,
    ) -> CalibrationInterpolator {
        CalibrationInterpolator::new(
            NoiseParameterId::new(0, 1),
            samples,
            method,
            extrapolation,
            &InterpolationLimits::unlimited(),
        )
        .expect("valid interpolator")
    }

    #[test]
    fn samples_are_sorted_deterministically() {
        let interpolator = interpolator(
            vec![
                scalar_sample(2_000_000_000, 20.0),
                scalar_sample(0, 0.0),
                scalar_sample(1_000_000_000, 10.0),
            ],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
        );

        assert_eq!(
            interpolator.samples()[0].timestamp_ns(),
            0
        );

        assert_eq!(
            interpolator.samples()[1].timestamp_ns(),
            1_000_000_000
        );

        assert_eq!(
            interpolator.samples()[2].timestamp_ns(),
            2_000_000_000
        );
    }

    #[test]
    fn linear_interpolation_is_deterministic() {
        let interpolator = interpolator(
            vec![
                scalar_sample(0, 0.0),
                scalar_sample(1_000_000_000, 10.0),
            ],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
        );

        let result = interpolator
            .evaluate(
                500_000_000,
                &InterpolationLimits::unlimited(),
            )
            .expect("interpolation should succeed");

        match result.value() {
            CalibrationValue::Scalar(value) => {
                assert!((*value - 5.0).abs() < 1.0e-12);
            }
            _ => panic!("expected scalar"),
        }

        assert_eq!(
            result.kind(),
            InterpolationResultKind::Interpolated
        );
    }

    #[test]
    fn exact_sample_is_not_reported_as_approximation() {
        let interpolator = interpolator(
            vec![
                scalar_sample(0, 3.0),
                scalar_sample(1_000_000_000, 7.0),
            ],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
        );

        let result = interpolator
            .evaluate(
                1_000_000_000,
                &InterpolationLimits::unlimited(),
            )
            .expect("exact lookup should succeed");

        assert_eq!(
            result.kind(),
            InterpolationResultKind::Exact
        );

        match result.value() {
            CalibrationValue::Scalar(value) => {
                assert_eq!(*value, 7.0);
            }
            _ => panic!("expected scalar"),
        }
    }

    #[test]
    fn previous_is_stepwise() {
        let interpolator = interpolator(
            vec![
                scalar_sample(0, 10.0),
                scalar_sample(1_000_000_000, 20.0),
            ],
            InterpolationMethod::Previous,
            ExtrapolationPolicy::Reject,
        );

        let result = interpolator
            .evaluate(
                750_000_000,
                &InterpolationLimits::unlimited(),
            )
            .expect("previous interpolation should succeed");

        match result.value() {
            CalibrationValue::Scalar(value) => {
                assert_eq!(*value, 10.0);
            }
            _ => panic!("expected scalar"),
        }
    }

    #[test]
    fn next_is_stepwise() {
        let interpolator = interpolator(
            vec![
                scalar_sample(0, 10.0),
                scalar_sample(1_000_000_000, 20.0),
            ],
            InterpolationMethod::Next,
            ExtrapolationPolicy::Reject,
        );

        let result = interpolator
            .evaluate(
                750_000_000,
                &InterpolationLimits::unlimited(),
            )
            .expect("next interpolation should succeed");

        match result.value() {
            CalibrationValue::Scalar(value) => {
                assert_eq!(*value, 20.0);
            }
            _ => panic!("expected scalar"),
        }
    }

    #[test]
    fn duplicate_timestamps_are_rejected() {
        let result = CalibrationInterpolator::new(
            NoiseParameterId::new(0, 1),
            vec![
                scalar_sample(1_000, 1.0),
                scalar_sample(1_000, 2.0),
            ],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
            &InterpolationLimits::unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn extrapolation_can_be_rejected() {
        let interpolator = interpolator(
            vec![
                scalar_sample(1_000_000_000, 10.0),
                scalar_sample(2_000_000_000, 20.0),
            ],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
        );

        assert!(
            interpolator
                .evaluate(
                    0,
                    &InterpolationLimits::unlimited(),
                )
                .is_err()
        );
    }

    #[test]
    fn clamping_is_explicit() {
        let interpolator = interpolator(
            vec![
                scalar_sample(1_000_000_000, 10.0),
                scalar_sample(2_000_000_000, 20.0),
            ],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Clamp,
        );

        let result = interpolator
            .evaluate(
                0,
                &InterpolationLimits::unlimited(),
            )
            .expect("clamping should succeed");

        assert_eq!(
            result.kind(),
            InterpolationResultKind::Clamped
        );

        match result.value() {
            CalibrationValue::Scalar(value) => {
                assert_eq!(*value, 10.0);
            }
            _ => panic!("expected scalar"),
        }
    }

    #[test]
    fn vector_interpolation_is_component_wise() {
        let first = CalibrationSample::new(
            0,
            CalibrationValue::vector(vec![0.0, 10.0])
                .expect("valid vector"),
        )
        .expect("valid sample");

        let second = CalibrationSample::new(
            1_000_000_000,
            CalibrationValue::vector(vec![10.0, 30.0])
                .expect("valid vector"),
        )
        .expect("valid sample");

        let interpolator = interpolator(
            vec![first, second],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
        );

        let result = interpolator
            .evaluate(
                500_000_000,
                &InterpolationLimits::unlimited(),
            )
            .expect("vector interpolation should succeed");

        match result.value() {
            CalibrationValue::Vector(values) => {
                assert!((values[0] - 5.0).abs() < 1.0e-12);
                assert!((values[1] - 20.0).abs() < 1.0e-12);
            }
            _ => panic!("expected vector"),
        }
    }

    #[test]
    fn matrix_interpolation_preserves_shape() {
        let first = CalibrationSample::new(
            0,
            CalibrationValue::matrix(
                2,
                2,
                vec![0.0, 2.0, 4.0, 6.0],
            )
            .expect("valid matrix"),
        )
        .expect("valid sample");

        let second = CalibrationSample::new(
            1_000_000_000,
            CalibrationValue::matrix(
                2,
                2,
                vec![2.0, 4.0, 6.0, 8.0],
            )
            .expect("valid matrix"),
        )
        .expect("valid sample");

        let interpolator = interpolator(
            vec![first, second],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
        );

        let result = interpolator
            .evaluate(
                500_000_000,
                &InterpolationLimits::unlimited(),
            )
            .expect("matrix interpolation should succeed");

        match result.value() {
            CalibrationValue::Matrix {
                rows,
                columns,
                values,
            } => {
                assert_eq!(*rows, 2);
                assert_eq!(*columns, 2);
                assert_eq!(
                    values,
                    &vec![1.0, 3.0, 5.0, 7.0]
                );
            }
            _ => panic!("expected matrix"),
        }
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let result = CalibrationSample::new(
            0,
            CalibrationValue::Scalar(f64::NAN),
        );

        assert!(result.is_err());
    }

    #[test]
    fn resource_limits_are_enforced() {
        let limits = InterpolationLimits {
            max_samples: Some(1),
            max_value_elements: None,
            max_evaluation_elements: None,
        };

        let result = CalibrationInterpolator::new(
            NoiseParameterId::new(0, 1),
            vec![
                scalar_sample(0, 1.0),
                scalar_sample(1, 2.0),
            ],
            InterpolationMethod::Linear,
            ExtrapolationPolicy::Reject,
            &limits,
        );

        assert!(result.is_err());
    }

    #[test]
    fn nearest_tie_prefers_earlier_sample() {
        let interpolator = interpolator(
            vec![
                scalar_sample(0, 10.0),
                scalar_sample(10, 20.0),
            ],
            InterpolationMethod::Nearest,
            ExtrapolationPolicy::Reject,
        );

        let result = interpolator
            .evaluate(
                5,
                &InterpolationLimits::unlimited(),
            )
            .expect("nearest interpolation should succeed");

        match result.value() {
            CalibrationValue::Scalar(value) => {
                assert_eq!(*value, 10.0);
            }
            _ => panic!("expected scalar"),
        }
    }
}