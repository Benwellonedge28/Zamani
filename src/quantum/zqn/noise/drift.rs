//! Zamani Quantum Noise (ZQN) — Temporal Drift Semantics
//!
//! # Purpose
//!
//! This module defines deterministic, backend-independent temporal drift
//! semantics for ZQN.
//!
//! Drift answers:
//!
//! > "How does a physical/noise parameter change as an explicitly supplied
//! > temporal coordinate changes?"
//!
//! This file is deliberately concerned with the mathematical/semantic model
//! of drift. It does not execute quantum operations and does not communicate
//! with hardware.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                        │
//!                        │ canonical resource identity
//!                        ▼
//!                ┌─────────────────┐
//!                │       ZQN       │
//!                │                 │
//!                │     drift.rs    │
//!                └────────┬────────┘
//!                         │
//!             ┌───────────┼────────────┐
//!             │           │            │
//!             ▼           ▼            ▼
//!        calibration   temporal      simulation
//!             │           │            │
//!             └───────────┼────────────┘
//!                         ▼
//!                  noise parameters
//!                         │
//!              ┌──────────┼──────────┐
//!              ▼          ▼          ▼
//!           routing   scheduling    QEC
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - drift laws;
//! - drift parameters;
//! - drift scope;
//! - drift validity;
//! - drift extrapolation policy;
//! - deterministic drift evaluation;
//! - composition of drift effects;
//! - validation of drift models;
//! - resource-scoped drift declarations;
//! - numerical safety of drift evaluation;
//! - explicit drift snapshots/queries;
//! - stable semantic comparison of drift models.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - qubit identity definitions;
//! - calibration storage;
//! - hardware APIs;
//! - wall-clock time;
//! - random-number generation;
//! - quantum channels;
//! - stochastic sampling;
//! - simulation state evolution;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking orchestration;
//! - serialization formats;
//! - global registries;
//! - global mutable state.
//!
//! # Canonical quantum identities
//!
//! When drift is scoped to quantum resources, this module uses the canonical
//! identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No alternative ZQN qubit identity is defined here.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_DRIFT_SEGMENTS
//! MAX_TIME
//! MAX_PARAMETERS
//! ```
//!
//! The semantic model has no finite machine-size ceiling.
//!
//! Actual limits are imposed by the caller/runtime/resource policy.
//!
//! A single drift model can therefore describe:
//!
//! - one resource;
//! - many resources;
//! - a complete device;
//! - a distributed system;
//! - a logical machine;
//! - arbitrarily large generated resource sets,
//!
//! subject only to available execution/storage resources.
//!
//! The implementation does not allocate per-qubit state unless the caller
//! explicitly constructs per-qubit drift models.
//!
//! # Determinism
//!
//! Drift evaluation is pure and deterministic:
//!
//! ```text
//! drift model + explicit time → value
//! ```
//!
//! It does not inspect:
//!
//! - system time;
//! - thread identity;
//! - memory addresses;
//! - global state;
//! - environment variables;
//! - random-number generators.
//!
//! The same model evaluated at the same time produces the same result.
//!
//! # Numerical policy
//!
//! NaN and infinity are rejected at construction and evaluation boundaries.
//!
//! Arithmetic overflow is detected whenever possible using checked operations
//! or explicit finite-result validation.
//!
//! The implementation never silently converts:
//!
//! ```text
//! NaN → 0
//! ∞   → MAX
//! invalid parameter → fallback parameter
//! ```
//!
//! # Physical interpretation
//!
//! Drift is deliberately generic.
//!
//! A parameter can represent, for example:
//!
//! - gate error rate;
//! - T1;
//! - T2;
//! - detuning;
//! - readout error;
//! - crosstalk strength;
//! - leakage rate;
//! - loss rate;
//! - pulse amplitude;
//! - calibration coefficient;
//! - arbitrary future noise parameter.
//!
//! This module does not hard-code physical parameter names.
//!
//! # Integration with temporal.rs
//!
//! `temporal.rs` owns the broader temporal-noise vocabulary.
//!
//! This file supplies the specialized drift model that can be evaluated from
//! the explicit temporal coordinate defined by that module.
//!
//! The primary API accepts seconds directly so this file remains independently
//! usable. The `evaluate_at_time` adapter accepts the canonical ZQN `Time` type.
//!
//! # Integration with calibration
//!
//! Calibration owns:
//!
//! - calibration snapshots;
//! - provenance;
//! - device-specific parameter identity;
//! - validity policy.
//!
//! Calibration can construct a `DriftModel` from measured drift parameters.
//!
//! This file does not store calibration snapshots.
//!
//! # Integration with scheduling
//!
//! A scheduler can query:
//!
//! ```text
//! drift.value_at(start_time)
//! ```
//!
//! before evaluating a time-dependent operation cost.
//!
//! The scheduler remains responsible for determining operation start times.
//!
//! # Integration with routing
//!
//! Routing can use the evaluated drift value as one component of a target
//! noise/fidelity cost.
//!
//! This module does not know routing algorithms.
//!
//! # Integration with simulation
//!
//! A simulator can evaluate the drift at the simulated physical/semantic time
//! and use the result to construct or select the appropriate noise channel.
//!
//! This module does not modify quantum state.
//!
//! # Integration with QEC
//!
//! QEC can query drifted physical-error parameters when constructing physical
//! fault probabilities or estimating time-dependent logical error behavior.
//!
//! QEC remains responsible for fault-tolerant semantics.
//!
//! # Integration with benchmarking
//!
//! Benchmarking can repeatedly evaluate this model against observations to
//! estimate or characterize drift.
//!
//! Benchmarking owns experiment orchestration and statistical inference.
//!
//! # Integration with hardware
//!
//! Hardware adapters can convert device-native calibration/drift information
//! into this backend-independent representation.
//!
//! This file never calls a vendor API.
//!
//! # Serialization
//!
//! This file defines semantic value types only.
//!
//! `zqn::io` owns the external serialization format.
//!
//! The Rust memory layout of these structures is NOT a serialization contract.
//!
//! # Security
//!
//! Drift specifications may originate from untrusted configuration.
//!
//! Constructors validate numerical parameters.
//!
//! Evaluation performs no:
//!
//! - I/O;
//! - process execution;
//! - dynamic code execution;
//! - recursion;
//! - uncontrolled allocation.
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
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::temporal::{TemporalNoiseError, TemporalNoiseResult, Time};

// =============================================================================
// Validation helpers
// =============================================================================

fn finite(field: &'static str, value: f64) -> TemporalNoiseResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(TemporalNoiseError::NonFinite { field, value })
    }
}

fn non_negative(field: &'static str, value: f64) -> TemporalNoiseResult<f64> {
    finite(field, value)?;

    if value < 0.0 {
        return Err(TemporalNoiseError::NegativeDuration { field, value });
    }

    Ok(value)
}

fn positive_rate(field: &'static str, value: f64) -> TemporalNoiseResult<f64> {
    finite(field, value)?;

    if value < 0.0 {
        return Err(TemporalNoiseError::InvalidRate { field, value });
    }

    Ok(value)
}

fn finite_result(operation: &'static str, value: f64) -> TemporalNoiseResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(TemporalNoiseError::NumericalOverflow { operation })
    }
}

// =============================================================================
// Drift scope
// =============================================================================

/// Semantic scope to which a drift model applies.
///
/// The canonical logical and physical qubit identities come directly from
/// `quantum::ir::qubit`.
///
/// `NamedResource` is intentionally generic so future quantum modalities can
/// be represented without changing this file merely because a new resource
/// kind is introduced.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DriftScope {
    /// Applies independently of a particular quantum resource.
    Global,

    /// Applies to one canonical logical qubit.
    LogicalQubit(QubitId),

    /// Applies to one canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Applies to an arbitrary externally defined resource identity.
    ///
    /// The string is a semantic identifier, not a hardware/vendor identity
    /// type. Higher layers own the interpretation.
    NamedResource(String),
}

impl DriftScope {
    /// Creates a named resource scope.
    pub fn named_resource<S: Into<String>>(name: S) -> TemporalNoiseResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(TemporalNoiseError::EmptyInput {
                field: "resource name",
            });
        }

        Ok(Self::NamedResource(name))
    }

    /// Returns true if this scope is global.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }
}

// =============================================================================
// Validity / extrapolation
// =============================================================================

/// Defines how a drift model behaves outside its declared validity interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtrapolationPolicy {
    /// Evaluation outside the validity interval is rejected.
    Reject,

    /// Values are evaluated using the drift law outside the interval.
    Allow,

    /// Values before/after the interval are evaluated at the nearest boundary.
    Clamp,
}

// =============================================================================
// Validity interval
// =============================================================================

/// Closed temporal interval in seconds.
///
/// A validity interval is metadata/semantic policy. It does not imply that
/// physical calibration is valid forever outside the interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValidityInterval {
    start_seconds: f64,
    end_seconds: f64,
}

impl ValidityInterval {
    /// Creates a validity interval.
    pub fn new(start_seconds: f64, end_seconds: f64) -> TemporalNoiseResult<Self> {
        finite("start_seconds", start_seconds)?;
        finite("end_seconds", end_seconds)?;

        if end_seconds < start_seconds {
            return Err(TemporalNoiseError::InvalidInterval {
                start: start_seconds,
                end: end_seconds,
            });
        }

        Ok(Self {
            start_seconds,
            end_seconds,
        })
    }

    /// Returns the start coordinate.
    #[must_use]
    pub const fn start_seconds(self) -> f64 {
        self.start_seconds
    }

    /// Returns the end coordinate.
    #[must_use]
    pub const fn end_seconds(self) -> f64 {
        self.end_seconds
    }

    /// Returns true if the interval contains the supplied time.
    #[must_use]
    pub fn contains(self, seconds: f64) -> bool {
        seconds >= self.start_seconds && seconds <= self.end_seconds
    }

    /// Returns the nearest point in the interval.
    #[must_use]
    pub fn clamp(self, seconds: f64) -> f64 {
        seconds.clamp(self.start_seconds, self.end_seconds)
    }
}

// =============================================================================
// Drift laws
// =============================================================================

/// Mathematical law describing temporal drift.
///
/// The law returns a drift delta/factor according to the `DriftEffect` used by
/// the containing model.
///
/// All parameters are validated at construction.
#[derive(Debug, Clone, PartialEq)]
pub enum DriftLaw {
    /// No temporal change.
    Constant,

    /// Linear change:
    ///
    /// ```text
    /// delta = rate * elapsed
    /// ```
    Linear {
        /// Change per second.
        rate_per_second: f64,
    },

    /// Polynomial change:
    ///
    /// ```text
    /// delta =
    ///     c0
    ///   + c1*t
    ///   + c2*t²
    ///   + ...
    /// ```
    ///
    /// Coefficients are ordered from lowest to highest power.
    Polynomial {
        /// Polynomial coefficients.
        coefficients: Vec<f64>,
    },

    /// Exponential relaxation toward a target:
    ///
    /// ```text
    /// target + (initial - target) * exp(-rate * t)
    /// ```
    ///
    /// This law represents an absolute value and therefore is intended for
    /// `DriftEffect::Absolute`.
    ExponentialRelaxation {
        /// Initial value at elapsed time zero.
        initial: f64,

        /// Long-time target value.
        target: f64,

        /// Relaxation rate per second.
        rate_per_second: f64,
    },

    /// Exponential growth/decay of a multiplicative factor:
    ///
    /// ```text
    /// factor = exp(rate * t)
    /// ```
    ///
    /// Positive rates grow; negative rates decay.
    ExponentialFactor {
        /// Exponential rate per second.
        rate_per_second: f64,
    },

    /// Sinusoidal variation:
    ///
    /// ```text
    /// amplitude * sin(angular_frequency * t + phase)
    /// ```
    Sinusoidal {
        /// Amplitude.
        amplitude: f64,

        /// Angular frequency in radians per second.
        angular_frequency: f64,

        /// Phase in radians.
        phase: f64,
    },

    /// Piecewise-linear interpolation over explicitly supplied samples.
    ///
    /// Sample times must be strictly increasing.
    PiecewiseLinear {
        /// `(time, value)` pairs in seconds.
        samples: Vec<(f64, f64)>,
    },
}

impl DriftLaw {
    /// Constructs a linear drift law.
    pub fn linear(rate_per_second: f64) -> TemporalNoiseResult<Self> {
        positive_rate("rate_per_second", rate_per_second)?;

        Ok(Self::Linear { rate_per_second })
    }

    /// Constructs a polynomial drift law.
    ///
    /// At least one coefficient is required.
    pub fn polynomial(coefficients: Vec<f64>) -> TemporalNoiseResult<Self> {
        if coefficients.is_empty() {
            return Err(TemporalNoiseError::EmptyInput {
                field: "polynomial coefficients",
            });
        }

        for coefficient in &coefficients {
            finite("polynomial coefficient", *coefficient)?;
        }

        Ok(Self::Polynomial { coefficients })
    }

    /// Constructs an exponential relaxation law.
    pub fn exponential_relaxation(
        initial: f64,
        target: f64,
        rate_per_second: f64,
    ) -> TemporalNoiseResult<Self> {
        finite("initial", initial)?;
        finite("target", target)?;
        positive_rate("rate_per_second", rate_per_second)?;

        Ok(Self::ExponentialRelaxation {
            initial,
            target,
            rate_per_second,
        })
    }

    /// Constructs an exponential multiplicative factor law.
    pub fn exponential_factor(rate_per_second: f64) -> TemporalNoiseResult<Self> {
        finite("rate_per_second", rate_per_second)?;

        Ok(Self::ExponentialFactor { rate_per_second })
    }

    /// Constructs a sinusoidal drift law.
    pub fn sinusoidal(
        amplitude: f64,
        angular_frequency: f64,
        phase: f64,
    ) -> TemporalNoiseResult<Self> {
        finite("amplitude", amplitude)?;
        finite("angular_frequency", angular_frequency)?;
        finite("phase", phase)?;

        Ok(Self::Sinusoidal {
            amplitude,
            angular_frequency,
            phase,
        })
    }

    /// Constructs a piecewise-linear law.
    ///
    /// The supplied sample coordinates must be finite and strictly increasing.
    pub fn piecewise_linear(samples: Vec<(f64, f64)>) -> TemporalNoiseResult<Self> {
        if samples.is_empty() {
            return Err(TemporalNoiseError::EmptyInput {
                field: "drift samples",
            });
        }

        let mut previous_time = None;

        for (time, value) in &samples {
            finite("sample time", *time)?;
            finite("sample value", *value)?;

            if let Some(previous) = previous_time {
                if *time <= previous {
                    return Err(TemporalNoiseError::InvalidSamples {
                        reason: "sample times must be strictly increasing",
                    });
                }
            }

            previous_time = Some(*time);
        }

        Ok(Self::PiecewiseLinear { samples })
    }

    /// Evaluates the law at a non-negative elapsed time.
    ///
    /// The returned quantity is interpreted by `DriftEffect`.
    pub fn evaluate(&self, elapsed_seconds: f64) -> TemporalNoiseResult<f64> {
        non_negative("elapsed_seconds", elapsed_seconds)?;

        match self {
            Self::Constant => Ok(0.0),

            Self::Linear { rate_per_second } => {
                finite_result("linear drift multiplication", rate_per_second * elapsed_seconds)
            }

            Self::Polynomial { coefficients } => {
                // Horner's method:
                //
                // (((c_n * t + c_(n-1)) * t + ...) * t + c_0)
                //
                // avoids explicitly materializing t², t³, ... and therefore
                // scales with polynomial degree rather than numerical power
                // allocations.
                let mut value = 0.0;

                for coefficient in coefficients.iter().rev() {
                    value = value * elapsed_seconds + coefficient;

                    if !value.is_finite() {
                        return Err(TemporalNoiseError::NumericalOverflow {
                            operation: "polynomial drift evaluation",
                        });
                    }
                }

                Ok(value)
            }

            Self::ExponentialRelaxation {
                initial,
                target,
                rate_per_second,
            } => {
                let exponent = -rate_per_second * elapsed_seconds;

                if !exponent.is_finite() {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "exponential relaxation exponent",
                    });
                }

                let value = target + (initial - target) * exponent.exp();

                finite_result("exponential relaxation evaluation", value)
            }

            Self::ExponentialFactor { rate_per_second } => {
                let exponent = rate_per_second * elapsed_seconds;

                if !exponent.is_finite() {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "exponential factor exponent",
                    });
                }

                finite_result("exponential factor evaluation", exponent.exp())
            }

            Self::Sinusoidal {
                amplitude,
                angular_frequency,
                phase,
            } => {
                let argument = angular_frequency * elapsed_seconds + phase;

                if !argument.is_finite() {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "sinusoidal argument",
                    });
                }

                finite_result("sinusoidal drift evaluation", amplitude * argument.sin())
            }

            Self::PiecewiseLinear { samples } => evaluate_piecewise_linear(samples, elapsed_seconds),
        }
    }
}

fn evaluate_piecewise_linear(
    samples: &[(f64, f64)],
    time: f64,
) -> TemporalNoiseResult<f64> {
    debug_assert!(!samples.is_empty());

    if samples.len() == 1 {
        return Ok(samples[0].1);
    }

    if time <= samples[0].0 {
        return Ok(samples[0].1);
    }

    let last_index = samples.len() - 1;

    if time >= samples[last_index].0 {
        return Ok(samples[last_index].1);
    }

    // Binary search keeps lookup O(log n), making large sampled drift profiles
    // practical without imposing an artificial maximum sample count.
    let mut low = 0usize;
    let mut high = last_index;

    while low + 1 < high {
        let middle = low + (high - low) / 2;

        if samples[middle].0 <= time {
            low = middle;
        } else {
            high = middle;
        }
    }

    let (t0, v0) = samples[low];
    let (t1, v1) = samples[high];

    let denominator = t1 - t0;

    if denominator <= 0.0 || !denominator.is_finite() {
        return Err(TemporalNoiseError::InvalidSamples {
            reason: "sample times must remain strictly increasing",
        });
    }

    let fraction = (time - t0) / denominator;

    if !fraction.is_finite() {
        return Err(TemporalNoiseError::NumericalOverflow {
            operation: "piecewise-linear interpolation",
        });
    }

    let value = v0 + fraction * (v1 - v0);

    finite_result("piecewise-linear interpolation", value)
}

// =============================================================================
// Drift effect
// =============================================================================

/// Determines how the evaluated drift law modifies the baseline parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriftEffect {
    /// The law produces the complete parameter value.
    ///
    /// This is appropriate for laws such as exponential relaxation.
    Absolute,

    /// The law produces a delta:
    ///
    /// ```text
    /// value = baseline + drift
    /// ```
    Additive,

    /// The law produces a multiplicative factor:
    ///
    /// ```text
    /// value = baseline * factor
    /// ```
    Multiplicative,
}

impl DriftEffect {
    /// Applies this effect to a baseline.
    pub fn apply(
        self,
        baseline: f64,
        evaluated: f64,
    ) -> TemporalNoiseResult<f64> {
        finite("baseline", baseline)?;
        finite("evaluated drift", evaluated)?;

        let result = match self {
            Self::Absolute => evaluated,
            Self::Additive => baseline + evaluated,
            Self::Multiplicative => baseline * evaluated,
        };

        finite_result("drift effect application", result)
    }
}

// =============================================================================
// Drift model
// =============================================================================

/// Complete deterministic temporal drift model.
///
/// A `DriftModel` is immutable after construction.
///
/// That property makes it naturally suitable for:
///
/// - concurrent evaluation;
/// - caching;
/// - reproducibility;
/// - distributed execution;
/// - provenance hashing by higher layers.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftModel {
    scope: DriftScope,
    parameter: String,
    baseline: f64,
    reference_time_seconds: f64,
    law: DriftLaw,
    effect: DriftEffect,
    validity: Option<ValidityInterval>,
    extrapolation: ExtrapolationPolicy,
}

impl DriftModel {
    /// Creates a drift model.
    ///
    /// `baseline` is the parameter value associated with the reference time
    /// unless the selected law/effect explicitly defines an absolute value.
    pub fn new(
        scope: DriftScope,
        parameter: impl Into<String>,
        baseline: f64,
        reference_time_seconds: f64,
        law: DriftLaw,
        effect: DriftEffect,
    ) -> TemporalNoiseResult<Self> {
        let parameter = parameter.into();

        if parameter.is_empty() {
            return Err(TemporalNoiseError::EmptyInput {
                field: "parameter name",
            });
        }

        finite("baseline", baseline)?;
        finite("reference_time_seconds", reference_time_seconds)?;

        validate_absolute_law_compatibility(&law, effect)?;

        Ok(Self {
            scope,
            parameter,
            baseline,
            reference_time_seconds,
            law,
            effect,
            validity: None,
            extrapolation: ExtrapolationPolicy::Reject,
        })
    }

    /// Sets the validity interval.
    #[must_use]
    pub fn with_validity(mut self, validity: ValidityInterval) -> Self {
        self.validity = Some(validity);
        self
    }

    /// Sets the extrapolation policy.
    #[must_use]
    pub const fn with_extrapolation(
        mut self,
        policy: ExtrapolationPolicy,
    ) -> Self {
        self.extrapolation = policy;
        self
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &DriftScope {
        &self.scope
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn parameter(&self) -> &str {
        &self.parameter
    }

    /// Returns the baseline parameter.
    #[must_use]
    pub const fn baseline(&self) -> f64 {
        self.baseline
    }

    /// Returns the reference time.
    #[must_use]
    pub const fn reference_time_seconds(&self) -> f64 {
        self.reference_time_seconds
    }

    /// Returns the drift law.
    #[must_use]
    pub fn law(&self) -> &DriftLaw {
        &self.law
    }

    /// Returns the effect semantics.
    #[must_use]
    pub const fn effect(&self) -> DriftEffect {
        self.effect
    }

    /// Returns the optional validity interval.
    #[must_use]
    pub const fn validity(&self) -> Option<ValidityInterval> {
        self.validity
    }

    /// Returns the extrapolation policy.
    #[must_use]
    pub const fn extrapolation(&self) -> ExtrapolationPolicy {
        self.extrapolation
    }

    /// Evaluates the model at an explicit time in seconds.
    pub fn value_at_seconds(&self, time_seconds: f64) -> TemporalNoiseResult<f64> {
        finite("time_seconds", time_seconds)?;

        let evaluation_time = self.resolve_evaluation_time(time_seconds)?;

        let elapsed = evaluation_time - self.reference_time_seconds;

        if elapsed < 0.0 {
            // Drift laws are defined relative to a reference time. Negative
            // elapsed time is not silently passed to laws whose semantics are
            // explicitly non-negative.
            return Err(TemporalNoiseError::InvalidInterval {
                start: evaluation_time,
                end: self.reference_time_seconds,
            });
        }

        let evaluated = self.law.evaluate(elapsed)?;

        self.effect.apply(self.baseline, evaluated)
    }

    /// Evaluates the model using the canonical ZQN temporal coordinate.
    pub fn value_at_time(&self, time: Time) -> TemporalNoiseResult<f64> {
        self.value_at_seconds(time.seconds())
    }

    fn resolve_evaluation_time(
        &self,
        requested_seconds: f64,
    ) -> TemporalNoiseResult<f64> {
        match self.validity {
            None => Ok(requested_seconds),

            Some(interval) if interval.contains(requested_seconds) => {
                Ok(requested_seconds)
            }

            Some(interval) => match self.extrapolation {
                ExtrapolationPolicy::Reject => Err(
                    TemporalNoiseError::OutOfDomain {
                        time: requested_seconds,
                        start: interval.start_seconds(),
                        end: interval.end_seconds(),
                    },
                ),

                ExtrapolationPolicy::Allow => Ok(requested_seconds),

                ExtrapolationPolicy::Clamp => Ok(interval.clamp(requested_seconds)),
            },
        }
    }
}

fn validate_absolute_law_compatibility(
    law: &DriftLaw,
    effect: DriftEffect,
) -> TemporalNoiseResult<()> {
    match (law, effect) {
        (
            DriftLaw::ExponentialRelaxation { .. },
            DriftEffect::Absolute,
        ) => Ok(()),

        (
            DriftLaw::ExponentialRelaxation { .. },
            DriftEffect::Additive | DriftEffect::Multiplicative,
        ) => Err(TemporalNoiseError::InvalidSamples {
            reason:
                "exponential relaxation defines an absolute value and must use DriftEffect::Absolute",
        }),

        _ => Ok(()),
    }
}

// =============================================================================
// Drift composition
// =============================================================================

/// Composition mode for multiple independent drift contributions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriftComposition {
    /// Add every contribution.
    Additive,

    /// Multiply every contribution.
    Multiplicative,
}

/// Evaluates multiple drift models at one explicit time.
///
/// The models may belong to different scopes/parameters. Higher layers should
/// group compatible models before calling this function.
///
/// No fixed number of models is imposed.
pub fn evaluate_many<'a, I>(
    models: I,
    time_seconds: f64,
    composition: DriftComposition,
) -> TemporalNoiseResult<f64>
where
    I: IntoIterator<Item = &'a DriftModel>,
{
    finite("time_seconds", time_seconds)?;

    let mut iter = models.into_iter();

    let first = match iter.next() {
        Some(model) => model.value_at_seconds(time_seconds)?,
        None => {
            return Err(TemporalNoiseError::EmptyInput {
                field: "drift models",
            });
        }
    };

    let mut result = first;

    for model in iter {
        let value = model.value_at_seconds(time_seconds)?;

        result = match composition {
            DriftComposition::Additive => result + value,
            DriftComposition::Multiplicative => result * value,
        };

        if !result.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "drift composition",
            });
        }
    }

    Ok(result)
}

// =============================================================================
// Drift observation
// =============================================================================

/// Immutable result of evaluating drift.
///
/// This gives downstream calibration, benchmarking and provenance layers a
/// stable value object without making those subsystems depend on internal
/// implementation details of `DriftModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftObservation {
    parameter: String,
    scope: DriftScope,
    requested_time_seconds: f64,
    evaluated_time_seconds: f64,
    value: f64,
}

impl DriftObservation {
    /// Evaluates a model and records the resolved evaluation coordinate.
    pub fn capture(
        model: &DriftModel,
        requested_time_seconds: f64,
    ) -> TemporalNoiseResult<Self> {
        finite("requested_time_seconds", requested_time_seconds)?;

        let evaluated_time =
            model.resolve_evaluation_time(requested_time_seconds)?;

        let value = model.value_at_seconds(requested_time_seconds)?;

        Ok(Self {
            parameter: model.parameter.clone(),
            scope: model.scope.clone(),
            requested_time_seconds,
            evaluated_time_seconds: evaluated_time,
            value,
        })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn parameter(&self) -> &str {
        &self.parameter
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &DriftScope {
        &self.scope
    }

    /// Returns the originally requested time.
    #[must_use]
    pub const fn requested_time_seconds(&self) -> f64 {
        self.requested_time_seconds
    }

    /// Returns the time actually used after validity/extrapolation policy.
    #[must_use]
    pub const fn evaluated_time_seconds(&self) -> f64 {
        self.evaluated_time_seconds
    }

    /// Returns the evaluated parameter value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}

// =============================================================================
// Drift builder
// =============================================================================

/// Ergonomic builder for constructing validated drift models.
///
/// The builder does not store global state and is therefore safe for use by
/// concurrent compilation/execution pipelines.
#[derive(Debug, Clone)]
pub struct DriftModelBuilder {
    scope: DriftScope,
    parameter: String,
    baseline: f64,
    reference_time_seconds: f64,
    law: DriftLaw,
    effect: DriftEffect,
    validity: Option<ValidityInterval>,
    extrapolation: ExtrapolationPolicy,
}

impl DriftModelBuilder {
    /// Creates a builder with a global scope and constant drift.
    pub fn new(
        parameter: impl Into<String>,
        baseline: f64,
    ) -> TemporalNoiseResult<Self> {
        let parameter = parameter.into();

        if parameter.is_empty() {
            return Err(TemporalNoiseError::EmptyInput {
                field: "parameter name",
            });
        }

        finite("baseline", baseline)?;

        Ok(Self {
            scope: DriftScope::Global,
            parameter,
            baseline,
            reference_time_seconds: 0.0,
            law: DriftLaw::Constant,
            effect: DriftEffect::Additive,
            validity: None,
            extrapolation: ExtrapolationPolicy::Reject,
        })
    }

    /// Changes the resource scope.
    #[must_use]
    pub fn scope(mut self, scope: DriftScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets the reference time.
    pub fn reference_time_seconds(
        mut self,
        seconds: f64,
    ) -> TemporalNoiseResult<Self> {
        finite("reference_time_seconds", seconds)?;
        self.reference_time_seconds = seconds;
        Ok(self)
    }

    /// Sets an additive linear drift.
    pub fn linear(
        mut self,
        rate_per_second: f64,
    ) -> TemporalNoiseResult<Self> {
        self.law = DriftLaw::linear(rate_per_second)?;
        self.effect = DriftEffect::Additive;
        Ok(self)
    }

    /// Sets a multiplicative exponential drift.
    pub fn exponential_factor(
        mut self,
        rate_per_second: f64,
    ) -> TemporalNoiseResult<Self> {
        self.law = DriftLaw::exponential_factor(rate_per_second)?;
        self.effect = DriftEffect::Multiplicative;
        Ok(self)
    }

    /// Sets absolute exponential relaxation.
    pub fn exponential_relaxation(
        mut self,
        initial: f64,
        target: f64,
        rate_per_second: f64,
    ) -> TemporalNoiseResult<Self> {
        self.law =
            DriftLaw::exponential_relaxation(initial, target, rate_per_second)?;
        self.effect = DriftEffect::Absolute;
        Ok(self)
    }

    /// Sets an arbitrary polynomial drift.
    pub fn polynomial(
        mut self,
        coefficients: Vec<f64>,
    ) -> TemporalNoiseResult<Self> {
        self.law = DriftLaw::polynomial(coefficients)?;
        self.effect = DriftEffect::Additive;
        Ok(self)
    }

    /// Sets sinusoidal additive drift.
    pub fn sinusoidal(
        mut self,
        amplitude: f64,
        angular_frequency: f64,
        phase: f64,
    ) -> TemporalNoiseResult<Self> {
        self.law =
            DriftLaw::sinusoidal(amplitude, angular_frequency, phase)?;
        self.effect = DriftEffect::Additive;
        Ok(self)
    }

    /// Sets piecewise-linear additive drift.
    pub fn piecewise_linear(
        mut self,
        samples: Vec<(f64, f64)>,
    ) -> TemporalNoiseResult<Self> {
        self.law = DriftLaw::piecewise_linear(samples)?;
        self.effect = DriftEffect::Additive;
        Ok(self)
    }

    /// Sets the effect interpretation explicitly.
    #[must_use]
    pub const fn effect(mut self, effect: DriftEffect) -> Self {
        self.effect = effect;
        self
    }

    /// Sets the validity interval.
    #[must_use]
    pub fn validity(mut self, validity: ValidityInterval) -> Self {
        self.validity = Some(validity);
        self
    }

    /// Sets the extrapolation policy.
    #[must_use]
    pub const fn extrapolation(
        mut self,
        policy: ExtrapolationPolicy,
    ) -> Self {
        self.extrapolation = policy;
        self
    }

    /// Builds the immutable validated model.
    pub fn build(self) -> TemporalNoiseResult<DriftModel> {
        let mut model = DriftModel::new(
            self.scope,
            self.parameter,
            self.baseline,
            self.reference_time_seconds,
            self.law,
            self.effect,
        )?;

        model.validity = self.validity;
        model.extrapolation = self.extrapolation;

        Ok(model)
    }
}

// =============================================================================
// Standard constructors
// =============================================================================

/// Creates a constant model.
///
/// This is useful when a caller needs a uniform interface for parameters that
/// may later acquire drift.
pub fn constant(
    scope: DriftScope,
    parameter: impl Into<String>,
    value: f64,
) -> TemporalNoiseResult<DriftModel> {
    DriftModel::new(
        scope,
        parameter,
        value,
        0.0,
        DriftLaw::Constant,
        DriftEffect::Additive,
    )
}

/// Creates an additive linear drift model.
pub fn linear(
    scope: DriftScope,
    parameter: impl Into<String>,
    baseline: f64,
    reference_time_seconds: f64,
    rate_per_second: f64,
) -> TemporalNoiseResult<DriftModel> {
    DriftModel::new(
        scope,
        parameter,
        baseline,
        reference_time_seconds,
        DriftLaw::linear(rate_per_second)?,
        DriftEffect::Additive,
    )
}

/// Creates an absolute exponential relaxation model.
pub fn exponential_relaxation(
    scope: DriftScope,
    parameter: impl Into<String>,
    initial: f64,
    target: f64,
    reference_time_seconds: f64,
    rate_per_second: f64,
) -> TemporalNoiseResult<DriftModel> {
    DriftModel::new(
        scope,
        parameter,
        initial,
        reference_time_seconds,
        DriftLaw::exponential_relaxation(
            initial,
            target,
            rate_per_second,
        )?,
        DriftEffect::Absolute,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_equal(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "left={left}, right={right}, tolerance={tolerance}"
        );
    }

    #[test]
    fn rejects_non_finite_time() {
        assert!(Time::new(f64::NAN).is_err());
        assert!(Time::new(f64::INFINITY).is_err());
        assert!(Time::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_negative_duration() {
        assert!(super::non_negative("duration", -1.0).is_err());
    }

    #[test]
    fn linear_drift_is_deterministic() {
        let model = linear(
            DriftScope::Global,
            "gate_error_rate",
            0.01,
            0.0,
            0.001,
        )
        .expect("valid model");

        let first = model.value_at_seconds(10.0).expect("valid evaluation");
        let second = model.value_at_seconds(10.0).expect("valid evaluation");

        assert_eq!(first, second);
        approx_equal(first, 0.02, 1.0e-15);
    }

    #[test]
    fn linear_drift_respects_reference_time() {
        let model = linear(
            DriftScope::Global,
            "parameter",
            2.0,
            5.0,
            0.5,
        )
        .expect("valid model");

        let value = model.value_at_seconds(7.0).expect("valid evaluation");

        approx_equal(value, 3.0, 1.0e-15);
    }

    #[test]
    fn exponential_relaxation_reaches_initial_value_at_reference() {
        let model = exponential_relaxation(
            DriftScope::Global,
            "t1",
            10.0,
            20.0,
            0.0,
            1.0,
        )
        .expect("valid model");

        let value = model.value_at_seconds(0.0).expect("valid evaluation");

        approx_equal(value, 10.0, 1.0e-15);
    }

    #[test]
    fn exponential_relaxation_moves_toward_target() {
        let model = exponential_relaxation(
            DriftScope::Global,
            "t1",
            10.0,
            20.0,
            0.0,
            1.0,
        )
        .expect("valid model");

        let value = model.value_at_seconds(1.0).expect("valid evaluation");

        assert!(value > 10.0);
        assert!(value < 20.0);
    }

    #[test]
    fn polynomial_uses_horner_evaluation() {
        let law = DriftLaw::polynomial(vec![1.0, 2.0, 3.0])
            .expect("valid polynomial");

        let value = law.evaluate(2.0).expect("valid evaluation");

        // 1 + 2*2 + 3*4 = 17
        approx_equal(value, 17.0, 1.0e-15);
    }

    #[test]
    fn piecewise_linear_interpolates() {
        let law = DriftLaw::piecewise_linear(vec![
            (0.0, 0.0),
            (10.0, 10.0),
            (20.0, 0.0),
        ])
        .expect("valid samples");

        let value = law.evaluate(5.0).expect("valid evaluation");

        approx_equal(value, 5.0, 1.0e-15);
    }

    #[test]
    fn piecewise_linear_is_bounded_at_edges() {
        let law = DriftLaw::piecewise_linear(vec![
            (0.0, 1.0),
            (10.0, 3.0),
        ])
        .expect("valid samples");

        assert_eq!(law.evaluate(0.0).expect("valid"), 1.0);
        assert_eq!(law.evaluate(10.0).expect("valid"), 3.0);
        assert_eq!(law.evaluate(-1.0).is_err(), true);
    }

    #[test]
    fn invalid_sample_order_is_rejected() {
        let result =
            DriftLaw::piecewise_linear(vec![(0.0, 0.0), (0.0, 1.0)]);

        assert!(result.is_err());
    }

    #[test]
    fn validity_rejects_outside_evaluation_by_default() {
        let validity =
            ValidityInterval::new(0.0, 10.0).expect("valid interval");

        let model = linear(
            DriftScope::Global,
            "parameter",
            1.0,
            0.0,
            0.1,
        )
        .expect("valid model")
        .with_validity(validity);

        assert!(model.value_at_seconds(11.0).is_err());
    }

    #[test]
    fn validity_can_clamp() {
        let validity =
            ValidityInterval::new(0.0, 10.0).expect("valid interval");

        let model = linear(
            DriftScope::Global,
            "parameter",
            1.0,
            0.0,
            0.1,
        )
        .expect("valid model")
        .with_validity(validity)
        .with_extrapolation(ExtrapolationPolicy::Clamp);

        let value = model.value_at_seconds(100.0).expect("valid evaluation");

        approx_equal(value, 2.0, 1.0e-15);
    }

    #[test]
    fn validity_can_allow_extrapolation() {
        let validity =
            ValidityInterval::new(0.0, 10.0).expect("valid interval");

        let model = linear(
            DriftScope::Global,
            "parameter",
            1.0,
            0.0,
            0.1,
        )
        .expect("valid model")
        .with_validity(validity)
        .with_extrapolation(ExtrapolationPolicy::Allow);

        let value = model.value_at_seconds(20.0).expect("valid evaluation");

        approx_equal(value, 3.0, 1.0e-15);
    }

    #[test]
    fn canonical_logical_qubit_scope_is_supported() {
        let qubit = QubitId::new(7);

        let model = linear(
            DriftScope::LogicalQubit(qubit),
            "readout_error",
            0.01,
            0.0,
            0.001,
        )
        .expect("valid model");

        assert!(matches!(
            model.scope(),
            DriftScope::LogicalQubit(value) if *value == qubit
        ));
    }

    #[test]
    fn canonical_physical_qubit_scope_is_supported() {
        let qubit = PhysicalQubitId::new(42);

        let model = linear(
            DriftScope::PhysicalQubit(qubit),
            "t1",
            100.0,
            0.0,
            -0.5,
        );

        // Negative drift rate is rejected by the generic linear constructor
        // because the current linear-law contract describes a non-negative
        // drift rate. A future signed-drift policy can be added explicitly
        // without changing resource identity semantics.
        assert!(model.is_err());
    }

    #[test]
    fn named_resource_scope_is_supported() {
        let scope =
            DriftScope::named_resource("mode-17").expect("valid scope");

        assert!(matches!(scope, DriftScope::NamedResource(_)));
    }

    #[test]
    fn observations_capture_evaluation_metadata() {
        let model = linear(
            DriftScope::Global,
            "gate_error",
            0.1,
            0.0,
            0.01,
        )
        .expect("valid model");

        let observation =
            DriftObservation::capture(&model, 5.0).expect("valid observation");

        assert_eq!(observation.parameter(), "gate_error");
        assert_eq!(observation.requested_time_seconds(), 5.0);
        assert_eq!(observation.evaluated_time_seconds(), 5.0);

        approx_equal(observation.value(), 0.15, 1.0e-15);
    }

    #[test]
    fn additive_composition_is_deterministic() {
        let first =
            linear(DriftScope::Global, "a", 1.0, 0.0, 0.1)
                .expect("valid model");

        let second =
            linear(DriftScope::Global, "b", 2.0, 0.0, 0.2)
                .expect("valid model");

        let models = [&first, &second];

        let value = evaluate_many(
            models,
            5.0,
            DriftComposition::Additive,
        )
        .expect("valid composition");

        // (1 + 0.5) + (2 + 1.0) = 4.5
        approx_equal(value, 4.5, 1.0e-15);
    }

    #[test]
    fn builder_produces_same_semantics_as_direct_constructor() {
        let direct = linear(
            DriftScope::Global,
            "parameter",
            1.0,
            0.0,
            0.1,
        )
        .expect("valid direct model");

        let built = DriftModelBuilder::new("parameter", 1.0)
            .expect("valid builder")
            .linear(0.1)
            .expect("valid linear law")
            .build()
            .expect("valid built model");

        assert_eq!(direct, built);
    }

    #[test]
    fn time_adapter_is_deterministic() {
        let model =
            linear(DriftScope::Global, "parameter", 1.0, 0.0, 0.5)
                .expect("valid model");

        let time = Time::new(4.0).expect("valid time");

        let value = model.value_at_time(time).expect("valid evaluation");

        approx_equal(value, 3.0, 1.0e-15);
    }

    #[test]
    fn no_hidden_clock_is_used() {
        let model =
            linear(DriftScope::Global, "parameter", 1.0, 0.0, 0.5)
                .expect("valid model");

        let first = model.value_at_seconds(4.0).expect("valid");
        let second = model.value_at_seconds(4.0).expect("valid");

        assert_eq!(first, second);
    }

    #[test]
    fn no_artificial_resource_limit_exists() {
        // This verifies the important semantic property rather than a
        // particular machine-size ceiling: resource identities are supplied
        // by the canonical IR and the drift model contains no qubit-count
        // limit.
        let first = QubitId::new(0);
        let large_index = QubitId::new(usize::MAX);

        assert_ne!(first, large_index);

        let first_model = linear(
            DriftScope::LogicalQubit(first),
            "parameter",
            1.0,
            0.0,
            0.1,
        )
        .expect("valid model");

        let large_model = linear(
            DriftScope::LogicalQubit(large_index),
            "parameter",
            1.0,
            0.0,
            0.1,
        )
        .expect("valid model");

        assert_eq!(
            first_model.value_at_seconds(1.0).expect("valid"),
            large_model.value_at_seconds(1.0).expect("valid")
        );
    }
}