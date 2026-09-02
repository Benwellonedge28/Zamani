//! Zamani Quantum Noise (ZQN) — Propagation / Error Accumulation.
//!
//! Path:
//!     src/quantum/zqn/propagation/accumulation.rs
//!
//! # Purpose
//!
//! This module provides backend-independent mathematical machinery for
//! accumulating quantities that arise while a quantum computation evolves
//! through operations, time intervals, resources, communication links,
//! calibration conditions, or other physical stages.
//!
//! The central design rule is:
//!
//! > Accumulation is a composition problem, not automatically an addition
//! > problem.
//!
//! Some quantities accumulate additively:
//!
//!     a_total = a_1 + a_2 + ... + a_n
//!
//! Some independent uncertainty contributions accumulate in quadrature:
//!
//!     u_total = sqrt(u_1² + u_2² + ... + u_n²)
//!
//! Some survival probabilities compose multiplicatively:
//!
//!     p_total = p_1 * p_2 * ... * p_n
//!
//! Some error probabilities require a domain-specific composition law.
//!
//! Consequently this module never assumes that every quantum error or noise
//! quantity may simply be summed.
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - deterministic accumulation of scalar quantities;
//! - additive accumulation;
//! - multiplicative accumulation;
//! - independent root-sum-square accumulation;
//! - weighted accumulation;
//! - bounded/worst-case accumulation;
//! - first-order sensitivity accumulation;
//! - streaming accumulation;
//! - deterministic ordering;
//! - accumulation diagnostics;
//! - explicit numerical tolerances;
//! - explicit resource policies;
//! - overflow-safe dimension/work accounting;
//! - composition through caller-supplied functions;
//! - immutable accumulation results;
//! - validation of numerical inputs.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - quantum states;
//! - density matrices;
//! - quantum channels;
//! - Kraus operators;
//! - Choi matrices;
//! - noise-model semantics;
//! - probability distributions;
//! - calibration storage;
//! - characterization protocols;
//! - uncertainty semantics;
//! - fidelity definitions;
//! - error-budget policy;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - simulator execution;
//! - canonical Quantum IR;
//! - canonical qubit identity;
//! - serialization wire formats;
//! - random-number generation.
//!
//! # Architectural position
//!
//! ```text
//!                 quantum::ir
//!                     │
//!                     ▼
//!          physical/noise quantities
//!                     │
//!                     ▼
//!             local sensitivities
//!                     │
//!                     ▼
//!        propagation::accumulation
//!             │       │       │
//!             │       │       └────► error_budget
//!             │       └────────────► uncertainty
//!             └────────────────────► fidelity
//!
//! Downstream consumers may additionally include:
//!
//!     routing
//!     scheduling
//!     QEC analysis
//!     calibration
//!     benchmarking
//! ```
//!
//! # Fundamental semantic distinction
//!
//! This module distinguishes:
//!
//! - quantity values;
//! - uncertainty magnitudes;
//! - sensitivities;
//! - worst-case bounds;
//! - survival/reliability factors;
//! - composition rules.
//!
//! A scalar value alone does not identify which of these meanings applies.
//!
//! Therefore callers must select an explicit `AccumulationRule`.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic maximum on:
//!
//! - number of accumulation terms;
//! - number of operations;
//! - number of resources;
//! - number of qubits;
//! - number of parameters;
//! - circuit depth;
//! - execution duration;
//! - machine size.
//!
//! Dimensions are derived exclusively from caller-provided data.
//!
//! The implementation supports streaming accumulation so callers do not need
//! to materialize all terms simultaneously.
//!
//! "Infinity" means that ZQN imposes no artificial machine-size ceiling. It
//! does not mean that finite CPU, memory, numerical precision, or execution
//! time cease to exist.
//!
//! # Resource safety
//!
//! All potentially expensive work is governed by explicit caller-provided
//! limits.
//!
//! No hidden global limit exists.
//!
//! `None` means that this module imposes no limit for that resource.
//!
//! Resource limits are policy, not quantum-machine semantics.
//!
//! # Determinism
//!
//! This module:
//!
//! - uses no RNG;
//! - uses no global mutable state;
//! - does not read the system clock;
//! - does not depend on thread identity;
//! - does not use unordered collections;
//! - processes streaming terms in caller-defined order;
//! - performs deterministic arithmetic.
//!
//! Given identical ordered inputs and identical numerical policy, results are
//! deterministic.
//!
//! Parallel implementations may use this module as a semantic reference, but
//! callers must use a deterministic reduction strategy if bit-for-bit
//! reproducibility is required.
//!
//! # Numerical safety
//!
//! The module rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - invalid tolerances;
//! - invalid weights;
//! - invalid multiplicative factors;
//! - invalid limits;
//! - arithmetic size overflow;
//! - invalid accumulated values.
//!
//! It never silently converts:
//!
//! - NaN to zero;
//! - infinity to a finite number;
//! - negative values to absolute values;
//! - invalid probabilities to valid probabilities;
//! - overflow to saturation.
//!
//! # Important physical interpretation
//!
//! A sum such as:
//!
//!     e_total = e_1 + e_2 + ... + e_n
//!
//! is generally only an approximation or bound for a physical error quantity.
//!
//! This module therefore does not call an additive result an "exact physical
//! error" unless the caller's composition law establishes that semantics.
//!
//! # Sensitivity integration
//!
//! If local sensitivity is represented by:
//!
//!     s_i = ∂y_i / ∂x
//!
//! then first-order additive sensitivity can be accumulated as:
//!
//!     s_total = Σ_i s_i
//!
//! For a sequence of transformations, however, the chain rule may instead
//! require:
//!
//!     dy_n/dx = Σ_i (∂y_n/∂y_i)(∂y_i/∂x)
//!
//! This module therefore provides scalar accumulation primitives but does not
//! pretend to replace a general Jacobian/automatic-differentiation system.
//!
//! # Uncertainty integration
//!
//! Independent standard uncertainties may be accumulated using root-sum-square:
//!
//!     u = sqrt(Σ_i u_i²)
//!
//! This is appropriate only under the caller's independence assumptions.
//!
//! Correlated uncertainty requires covariance-aware propagation and belongs to
//! `propagation::uncertainty`.
//!
//! # Error-budget integration
//!
//! `error_budget.rs` may consume accumulated values and compare them with
//! budgets.
//!
//! This module does not decide:
//!
//! - acceptable error;
//! - allocation policy;
//! - mitigation priority.
//!
//! # Fidelity integration
//!
//! Fidelity metrics may use accumulation primitives, but fidelity semantics
//! remain owned by `fidelity.rs`.
//!
//! # Noise integration
//!
//! Noise models can provide local contributions to an accumulation stream.
//! The noise model remains responsible for explaining what the contribution
//! physically means.
//!
//! # Calibration integration
//!
//! Calibration can provide parameter values, uncertainties, or drift samples.
//! Calibration remains responsible for their physical interpretation and
//! validity.
//!
//! # Routing integration
//!
//! Routing can accumulate costs over candidate paths or mappings. This module
//! provides the mathematical accumulation operation but does not select a
//! route.
//!
//! # Scheduling integration
//!
//! Scheduling can accumulate duration-dependent effects over a schedule.
//! This module does not own scheduling decisions.
//!
//! # QEC integration
//!
//! QEC may use accumulation for physical-error or sensitivity summaries.
//! Syndrome generation, decoding, correction, and logical-fault semantics remain
//! outside this module.
//!
//! # Quantum-resource identity
//!
//! Accumulation is deliberately resource-agnostic.
//!
//! It therefore does not define another `QubitId` or `PhysicalQubitId`.
//!
//! When a higher-level integration layer associates an accumulation with a
//! quantum resource, it must use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! rather than creating ZQN-specific duplicate identities.
//!
//! # Serialization
//!
//! This file defines semantic data structures only.
//!
//! It does not define a wire format.
//!
//! Versioned serialization belongs under:
//!
//!     crate::quantum::zqn::io
//!
//! # Security
//!
//! This module is designed to operate safely on untrusted numerical input.
//!
//! It:
//!
//! - validates all floating-point inputs;
//! - checks integer-size arithmetic;
//! - permits explicit resource limits;
//! - does not invoke external processes;
//! - does not use unsafe code;
//! - does not require recursion;
//! - does not allocate per input term when using streaming accumulation.
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
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. accumulation semantics are explicit;
//! 2. additive accumulation is supported;
//! 3. multiplicative accumulation is supported;
//! 4. root-sum-square accumulation is supported;
//! 5. worst-case accumulation is supported;
//! 6. weighted accumulation is supported;
//! 7. caller-defined composition is supported;
//! 8. streaming operation is supported;
//! 9. no artificial machine-size limit exists;
//! 10. optional resource limits are caller-controlled;
//! 11. invalid floating-point values are rejected;
//! 12. integer size calculations cannot overflow silently;
//! 13. deterministic behavior is explicit;
//! 14. no global RNG/state exists;
//! 15. no duplicate quantum-resource identity exists;
//! 16. serialization ownership remains outside this file;
//! 17. downstream modules can consume the public contract without modifying
//!     this implementation.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

/// Stable semantic identifier for this module.
pub const ACCUMULATION_SCHEMA_ID: &str =
    "zamani.quantum.zqn.propagation.accumulation";

/// Semantic version of this module's public contract.
pub const ACCUMULATION_SCHEMA_VERSION: u32 = 1;

/// Default numerical tolerance.
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;

/// Default maximum number of terms when a caller explicitly chooses the
/// default resource policy.
///
/// This value is intentionally `None` rather than a machine-size limit.
pub const DEFAULT_MAX_TERMS: Option<u64> = None;

/// Errors produced by accumulation operations.
#[derive(Clone, Debug, PartialEq)]
pub enum AccumulationError {
    /// A floating-point value was not finite.
    NonFinite {
        /// Semantic field containing the invalid value.
        field: &'static str,
        /// Invalid value.
        value: f64,
    },

    /// A weight was negative or otherwise invalid.
    InvalidWeight {
        /// Supplied weight.
        value: f64,
    },

    /// A multiplicative factor was invalid.
    InvalidFactor {
        /// Supplied factor.
        value: f64,
    },

    /// A tolerance was invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// A resource limit was invalid.
    InvalidLimit {
        /// Resource name.
        resource: &'static str,
    },

    /// An explicit resource limit was exceeded.
    ResourceLimitExceeded {
        /// Resource category.
        resource: &'static str,
        /// Number requested.
        requested: u128,
        /// Maximum allowed.
        maximum: u128,
    },

    /// Integer arithmetic would overflow.
    SizeOverflow {
        /// Semantic operation.
        context: &'static str,
    },

    /// Floating-point arithmetic produced an invalid result.
    NumericalFailure {
        /// Semantic operation.
        context: &'static str,
    },

    /// A caller supplied an invalid composition result.
    InvalidCompositionResult {
        /// Supplied result.
        value: f64,
    },
}

impl fmt::Display for AccumulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field, value } => {
                write!(f, "non-finite value in `{field}`: {value}")
            }
            Self::InvalidWeight { value } => {
                write!(f, "invalid accumulation weight: {value}")
            }
            Self::InvalidFactor { value } => {
                write!(f, "invalid accumulation factor: {value}")
            }
            Self::InvalidTolerance { value } => {
                write!(f, "invalid accumulation tolerance: {value}")
            }
            Self::InvalidLimit { resource } => {
                write!(f, "invalid resource limit for `{resource}`")
            }
            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                f,
                "resource limit exceeded for `{resource}`: requested {requested}, maximum {maximum}"
            ),
            Self::SizeOverflow { context } => {
                write!(f, "size arithmetic overflow in `{context}`")
            }
            Self::NumericalFailure { context } => {
                write!(f, "numerical failure during `{context}`")
            }
            Self::InvalidCompositionResult { value } => {
                write!(f, "composition function returned invalid value: {value}")
            }
        }
    }
}

impl std::error::Error for AccumulationError {}

/// Result alias used by this module.
pub type AccumulationResult<T> = Result<T, AccumulationError>;

/// Explicit rule describing how values are accumulated.
///
/// The variants intentionally have different semantics. The caller must
/// select the rule appropriate for the quantity being represented.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccumulationRule {
    /// Sum all values.
    Additive,

    /// Multiply all factors.
    Multiplicative,

    /// Accumulate independent magnitude-like contributions in quadrature.
    RootSumSquare,

    /// Sum absolute magnitudes, producing a conservative magnitude bound.
    WorstCase,

    /// Accumulate weighted values.
    WeightedAdditive,

    /// Accumulate weighted squared values and return their square root.
    WeightedRootSumSquare,
}

impl AccumulationRule {
    /// Returns the identity value for the rule.
    pub const fn identity(self) -> f64 {
        match self {
            Self::Additive
            | Self::RootSumSquare
            | Self::WorstCase
            | Self::WeightedAdditive
            | Self::WeightedRootSumSquare => 0.0,
            Self::Multiplicative => 1.0,
        }
    }

    /// Returns whether this rule requires non-negative terms.
    pub const fn requires_non_negative_terms(self) -> bool {
        matches!(
            self,
            Self::RootSumSquare
                | Self::WorstCase
                | Self::WeightedRootSumSquare
        )
    }

    /// Returns whether this rule interprets the input as a multiplicative
    /// factor rather than an additive contribution.
    pub const fn is_multiplicative(self) -> bool {
        matches!(self, Self::Multiplicative)
    }
}

/// Explicit resource policy for accumulation.
///
/// All fields are optional. `None` means that this module imposes no limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccumulationLimits {
    /// Maximum number of terms accepted by an accumulator.
    pub max_terms: Option<u64>,

    /// Maximum number of explicit work units permitted.
    pub max_work_units: Option<u128>,
}

impl Default for AccumulationLimits {
    fn default() -> Self {
        Self {
            max_terms: DEFAULT_MAX_TERMS,
            max_work_units: None,
        }
    }
}

impl AccumulationLimits {
    /// Creates an unlimited resource policy.
    pub const fn unlimited() -> Self {
        Self {
            max_terms: None,
            max_work_units: None,
        }
    }

    /// Validates this policy.
    pub fn validate(&self) -> AccumulationResult<()> {
        if self.max_terms == Some(0) {
            return Err(AccumulationError::InvalidLimit {
                resource: "max_terms",
            });
        }

        if self.max_work_units == Some(0) {
            return Err(AccumulationError::InvalidLimit {
                resource: "max_work_units",
            });
        }

        Ok(())
    }

    fn check_term(&self, current_terms: u64) -> AccumulationResult<()> {
        let next = current_terms
            .checked_add(1)
            .ok_or(AccumulationError::SizeOverflow {
                context: "term count",
            })?;

        if let Some(maximum) = self.max_terms {
            if next > maximum {
                return Err(AccumulationError::ResourceLimitExceeded {
                    resource: "terms",
                    requested: u128::from(next),
                    maximum: u128::from(maximum),
                });
            }
        }

        if let Some(maximum) = self.max_work_units {
            let requested = u128::from(next);

            if requested > maximum {
                return Err(AccumulationError::ResourceLimitExceeded {
                    resource: "work_units",
                    requested,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

/// Immutable description of one contribution.
///
/// The optional `weight` is used by weighted accumulation rules and ignored
/// by unweighted rules.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccumulationTerm {
    /// Contribution value.
    pub value: f64,

    /// Optional non-negative weight.
    pub weight: Option<f64>,
}

impl AccumulationTerm {
    /// Creates an unweighted term.
    pub const fn new(value: f64) -> Self {
        Self {
            value,
            weight: None,
        }
    }

    /// Creates a weighted term.
    pub const fn weighted(value: f64, weight: f64) -> Self {
        Self {
            value,
            weight: Some(weight),
        }
    }

    fn validate(&self, rule: AccumulationRule) -> AccumulationResult<()> {
        validate_finite("term.value", self.value)?;

        if let Some(weight) = self.weight {
            validate_finite("term.weight", weight)?;

            if weight < 0.0 {
                return Err(AccumulationError::InvalidWeight { value: weight });
            }
        }

        if rule.requires_non_negative_terms() && self.value < 0.0 {
            return Err(AccumulationError::NumericalFailure {
                context: "non-negative accumulation term",
            });
        }

        if rule.is_multiplicative() && self.value < 0.0 {
            return Err(AccumulationError::InvalidFactor { value: self.value });
        }

        Ok(())
    }
}

/// Diagnostics describing an accumulation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccumulationDiagnostics {
    /// Number of accepted terms.
    pub term_count: u64,

    /// Selected accumulation rule.
    pub rule: AccumulationRule,

    /// Whether the result represents a conservative magnitude bound.
    pub is_worst_case: bool,

    /// Whether weighted terms were used.
    pub weighted: bool,
}

/// Immutable result of accumulation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccumulationSummary {
    /// Accumulated value.
    pub value: f64,

    /// Diagnostics.
    pub diagnostics: AccumulationDiagnostics,
}

impl AccumulationSummary {
    /// Returns the accumulated value.
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns the number of terms.
    pub const fn term_count(self) -> u64 {
        self.diagnostics.term_count
    }

    /// Returns the accumulation rule.
    pub const fn rule(self) -> AccumulationRule {
        self.diagnostics.rule
    }
}

/// Streaming accumulator.
///
/// This is the preferred API when the number of terms may be very large.
///
/// It stores only the current accumulation state rather than materializing
/// all contributions.
#[derive(Clone, Debug)]
pub struct Accumulator {
    rule: AccumulationRule,
    limits: AccumulationLimits,
    value: f64,
    terms: u64,
    weighted: bool,
}

impl Accumulator {
    /// Creates an accumulator with the supplied rule and resource policy.
    pub fn new(
        rule: AccumulationRule,
        limits: AccumulationLimits,
    ) -> AccumulationResult<Self> {
        limits.validate()?;

        Ok(Self {
            rule,
            limits,
            value: rule.identity(),
            terms: 0,
            weighted: false,
        })
    }

    /// Creates an unlimited accumulator.
    pub fn unlimited(rule: AccumulationRule) -> AccumulationResult<Self> {
        Self::new(rule, AccumulationLimits::unlimited())
    }

    /// Returns the selected rule.
    pub const fn rule(&self) -> AccumulationRule {
        self.rule
    }

    /// Returns the number of accepted terms.
    pub const fn term_count(&self) -> u64 {
        self.terms
    }

    /// Returns the current value.
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Adds one unweighted contribution.
    pub fn add(&mut self, value: f64) -> AccumulationResult<()> {
        self.add_term(AccumulationTerm::new(value))
    }

    /// Adds one weighted contribution.
    pub fn add_weighted(
        &mut self,
        value: f64,
        weight: f64,
    ) -> AccumulationResult<()> {
        self.weighted = true;
        self.add_term(AccumulationTerm::weighted(value, weight))
    }

    /// Adds a contribution according to the selected rule.
    pub fn add_term(
        &mut self,
        term: AccumulationTerm,
    ) -> AccumulationResult<()> {
        term.validate(self.rule)?;
        self.limits.check_term(self.terms)?;

        let next = compose_builtin(self.rule, self.value, term)?;

        validate_finite("accumulator.value", next)?;

        self.value = next;
        self.terms = self
            .terms
            .checked_add(1)
            .ok_or(AccumulationError::SizeOverflow {
                context: "accumulator term count",
            })?;

        Ok(())
    }

    /// Finalizes the accumulation.
    pub fn finish(self) -> AccumulationResult<AccumulationSummary> {
        validate_finite("accumulation.result", self.value)?;

        Ok(AccumulationSummary {
            value: self.value,
            diagnostics: AccumulationDiagnostics {
                term_count: self.terms,
                rule: self.rule,
                is_worst_case: matches!(
                    self.rule,
                    AccumulationRule::WorstCase
                ),
                weighted: self.weighted,
            },
        })
    }
}

/// Accumulates an iterator of terms without materializing it.
pub fn accumulate<I>(
    rule: AccumulationRule,
    terms: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = AccumulationTerm>,
{
    let mut accumulator = Accumulator::new(rule, limits)?;

    for term in terms {
        accumulator.add_term(term)?;
    }

    accumulator.finish()
}

/// Accumulates scalar values without requiring callers to construct terms.
pub fn accumulate_values<I>(
    rule: AccumulationRule,
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
{
    accumulate(
        rule,
        values.into_iter().map(AccumulationTerm::new),
        limits,
    )
}

/// Accumulates weighted values.
///
/// The input tuple is `(value, weight)`.
pub fn accumulate_weighted<I>(
    rule: AccumulationRule,
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    accumulate(
        rule,
        values
            .into_iter()
            .map(|(value, weight)| AccumulationTerm::weighted(value, weight)),
        limits,
    )
}

/// Applies a caller-defined composition law to a stream of values.
///
/// This is the extensibility boundary for physical models whose composition
/// cannot be represented safely by one of the built-in rules.
///
/// The function receives:
///
/// - the current accumulated value;
/// - the next contribution.
///
/// It must return the next accumulated value.
///
/// The callback must be deterministic if deterministic results are required.
pub fn accumulate_with<I, F>(
    initial: f64,
    values: I,
    limits: AccumulationLimits,
    mut compose: F,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
    F: FnMut(f64, f64) -> f64,
{
    limits.validate()?;
    validate_finite("initial", initial)?;

    let mut current = initial;
    let mut count = 0_u64;

    for value in values {
        validate_finite("term", value)?;
        limits.check_term(count)?;

        current = compose(current, value);

        if !current.is_finite() {
            return Err(AccumulationError::InvalidCompositionResult {
                value: current,
            });
        }

        count = count
            .checked_add(1)
            .ok_or(AccumulationError::SizeOverflow {
                context: "custom accumulation term count",
            })?;
    }

    Ok(AccumulationSummary {
        value: current,
        diagnostics: AccumulationDiagnostics {
            term_count: count,
            rule: AccumulationRule::Additive,
            is_worst_case: false,
            weighted: false,
        },
    })
}

/// Performs additive accumulation.
///
/// This is mathematically:
///
///     Σ x_i
pub fn sum<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
{
    accumulate_values(AccumulationRule::Additive, values, limits)
}

/// Performs multiplicative accumulation.
///
/// This is mathematically:
///
///     Π x_i
///
/// It is appropriate for quantities such as survival/reliability factors when
/// the factors have that physical meaning.
pub fn product<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
{
    accumulate_values(AccumulationRule::Multiplicative, values, limits)
}

/// Performs root-sum-square accumulation.
///
/// This is:
///
///     sqrt(Σ x_i²)
///
/// It should only be used when the caller's statistical/physical assumptions
/// justify independent quadratic accumulation.
pub fn root_sum_square<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
{
    accumulate_values(AccumulationRule::RootSumSquare, values, limits)
}

/// Performs conservative worst-case magnitude accumulation.
///
/// This is:
///
///     Σ |x_i|
pub fn worst_case<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
{
    let mut accumulator =
        Accumulator::new(AccumulationRule::WorstCase, limits)?;

    for value in values {
        validate_finite("term", value)?;
        accumulator.add(value.abs())?;
    }

    accumulator.finish()
}

/// Performs weighted additive accumulation.
///
/// The result is:
///
///     Σ w_i x_i
///
/// with non-negative weights.
pub fn weighted_sum<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    accumulate_weighted(
        AccumulationRule::WeightedAdditive,
        values,
        limits,
    )
}

/// Performs weighted root-sum-square accumulation.
///
/// The result is:
///
///     sqrt(Σ w_i x_i²)
///
/// with non-negative weights.
pub fn weighted_root_sum_square<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    accumulate_weighted(
        AccumulationRule::WeightedRootSumSquare,
        values,
        limits,
    )
}

/// Performs first-order additive sensitivity accumulation.
///
/// This helper is intentionally scalar.
///
/// General chain-rule/Jacobian propagation belongs to `sensitivity.rs` and
/// `uncertainty.rs`.
pub fn accumulate_sensitivity<I>(
    sensitivities: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
{
    sum(sensitivities, limits)
}

/// Computes a first-order worst-case output change.
///
/// Given local sensitivities `s_i` and parameter perturbation magnitudes
/// `δx_i`, this computes:
///
///     Σ |s_i δx_i|
///
/// This is a conservative first-order bound and is not an exact nonlinear
/// propagation result.
pub fn first_order_worst_case<I>(
    contributions: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    let mapped = contributions.into_iter().map(|(sensitivity, delta)| {
        validate_finite("sensitivity", sensitivity)?;
        validate_finite("delta", delta)?;

        let contribution = sensitivity * delta;

        if !contribution.is_finite() {
            return Err(AccumulationError::NumericalFailure {
                context: "first-order sensitivity contribution",
            });
        }

        Ok(contribution.abs())
    });

    let mut accumulator =
        Accumulator::new(AccumulationRule::Additive, limits)?;

    for contribution in mapped {
        accumulator.add(contribution?)?;
    }

    accumulator.finish()
}

/// Computes a first-order independent uncertainty estimate.
///
/// Given:
///
///     contribution_i = sensitivity_i * uncertainty_i
///
/// the result is:
///
///     sqrt(Σ contribution_i²)
///
/// This function assumes the supplied contributions are independent. It does
/// not attempt to detect or infer correlation.
pub fn first_order_independent_uncertainty<I>(
    contributions: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    let mapped = contributions.into_iter().map(|(sensitivity, uncertainty)| {
        validate_finite("sensitivity", sensitivity)?;
        validate_finite("uncertainty", uncertainty)?;

        if uncertainty < 0.0 {
            return Err(AccumulationError::NumericalFailure {
                context: "negative uncertainty",
            });
        }

        let contribution = sensitivity * uncertainty;

        if !contribution.is_finite() {
            return Err(AccumulationError::NumericalFailure {
                context: "first-order uncertainty contribution",
            });
        }

        Ok(contribution)
    });

    let mut accumulator =
        Accumulator::new(AccumulationRule::RootSumSquare, limits)?;

    for contribution in mapped {
        accumulator.add(contribution?)?;
    }

    accumulator.finish()
}

/// Computes the maximum absolute value in a stream.
///
/// This is useful when the relevant quantity is a maximum local sensitivity,
/// maximum local error, or maximum local contribution rather than a sum.
pub fn maximum_absolute<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<AccumulationSummary>
where
    I: IntoIterator<Item = f64>,
{
    limits.validate()?;

    let mut maximum = 0.0_f64;
    let mut count = 0_u64;

    for value in values {
        validate_finite("term", value)?;
        limits.check_term(count)?;

        let magnitude = value.abs();

        if magnitude > maximum {
            maximum = magnitude;
        }

        count = count
            .checked_add(1)
            .ok_or(AccumulationError::SizeOverflow {
                context: "maximum accumulation term count",
            })?;
    }

    Ok(AccumulationSummary {
        value: maximum,
        diagnostics: AccumulationDiagnostics {
            term_count: count,
            rule: AccumulationRule::WorstCase,
            is_worst_case: true,
            weighted: false,
        },
    })
}

/// Computes the arithmetic mean without storing all terms.
///
/// Returns an error for an empty input because there is no unique semantic
/// mean for an empty sequence.
pub fn mean<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<f64>
where
    I: IntoIterator<Item = f64>,
{
    limits.validate()?;

    let mut accumulator =
        Accumulator::new(AccumulationRule::Additive, limits)?;

    for value in values {
        accumulator.add(value)?;
    }

    let summary = accumulator.finish()?;

    if summary.term_count == 0 {
        return Err(AccumulationError::NumericalFailure {
            context: "mean of empty sequence",
        });
    }

    let count = summary.term_count as f64;
    let result = summary.value / count;

    validate_finite("mean", result)?;

    Ok(result)
}

/// Computes the mean of a weighted sequence.
///
/// Returns:
///
///     Σ(w_i x_i) / Σw_i
///
/// Zero total weight is rejected.
pub fn weighted_mean<I>(
    values: I,
    limits: AccumulationLimits,
) -> AccumulationResult<f64>
where
    I: IntoIterator<Item = (f64, f64)>,
{
    limits.validate()?;

    let mut weighted_total =
        Accumulator::new(AccumulationRule::WeightedAdditive, limits)?;

    let mut weight_total = 0.0_f64;

    for (value, weight) in values {
        validate_finite("value", value)?;
        validate_finite("weight", weight)?;

        if weight < 0.0 {
            return Err(AccumulationError::InvalidWeight { value: weight });
        }

        weighted_total.add_weighted(value, weight)?;

        weight_total += weight;

        if !weight_total.is_finite() {
            return Err(AccumulationError::NumericalFailure {
                context: "total weight",
            });
        }
    }

    if weight_total == 0.0 {
        return Err(AccumulationError::NumericalFailure {
            context: "weighted mean with zero total weight",
        });
    }

    let result = weighted_total.value() / weight_total;

    validate_finite("weighted mean", result)?;

    Ok(result)
}

/// Validates a floating-point value.
///
/// This function is intentionally strict. Numerical invalidity is never
/// repaired silently.
pub fn validate_finite(
    field: &'static str,
    value: f64,
) -> AccumulationResult<()> {
    if !value.is_finite() {
        return Err(AccumulationError::NonFinite { field, value });
    }

    Ok(())
}

/// Validates a numerical tolerance.
pub fn validate_tolerance(value: f64) -> AccumulationResult<()> {
    validate_finite("tolerance", value)?;

    if value < 0.0 {
        return Err(AccumulationError::InvalidTolerance { value });
    }

    Ok(())
}

/// Returns the square root of a non-negative accumulated squared quantity.
///
/// A small negative value caused by floating-point roundoff is not silently
/// converted to zero here; the caller must supply a valid non-negative value.
pub fn checked_sqrt(value: f64) -> AccumulationResult<f64> {
    validate_finite("square-root input", value)?;

    if value < 0.0 {
        return Err(AccumulationError::NumericalFailure {
            context: "square root of negative value",
        });
    }

    let result = value.sqrt();

    validate_finite("square-root result", result)?;

    Ok(result)
}

fn compose_builtin(
    rule: AccumulationRule,
    current: f64,
    term: AccumulationTerm,
) -> AccumulationResult<f64> {
    match rule {
        AccumulationRule::Additive => {
            let result = current + term.value;

            if !result.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "additive accumulation",
                });
            }

            Ok(result)
        }

        AccumulationRule::Multiplicative => {
            let result = current * term.value;

            if !result.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "multiplicative accumulation",
                });
            }

            Ok(result)
        }

        AccumulationRule::RootSumSquare => {
            let square = term.value * term.value;

            if !square.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "root-sum-square term",
                });
            }

            let result = current.mul_add(current, square);

            if !result.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "root-sum-square accumulation",
                });
            }

            Ok(result.sqrt())
        }

        AccumulationRule::WorstCase => {
            let magnitude = term.value.abs();
            let result = current + magnitude;

            if !result.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "worst-case accumulation",
                });
            }

            Ok(result)
        }

        AccumulationRule::WeightedAdditive => {
            let weight = term.weight.unwrap_or(1.0);

            if weight < 0.0 {
                return Err(AccumulationError::InvalidWeight {
                    value: weight,
                });
            }

            let contribution = weight * term.value;

            if !contribution.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "weighted additive contribution",
                });
            }

            let result = current + contribution;

            if !result.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "weighted additive accumulation",
                });
            }

            Ok(result)
        }

        AccumulationRule::WeightedRootSumSquare => {
            let weight = term.weight.unwrap_or(1.0);

            if weight < 0.0 {
                return Err(AccumulationError::InvalidWeight {
                    value: weight,
                });
            }

            let weighted_square = weight * term.value * term.value;

            if !weighted_square.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "weighted root-sum-square term",
                });
            }

            let squared_current = current * current;

            if !squared_current.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "weighted root-sum-square current value",
                });
            }

            let squared_total = squared_current + weighted_square;

            if !squared_total.is_finite() {
                return Err(AccumulationError::NumericalFailure {
                    context: "weighted root-sum-square accumulation",
                });
            }

            Ok(squared_total.sqrt())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unlimited() -> AccumulationLimits {
        AccumulationLimits::unlimited()
    }

    #[test]
    fn additive_accumulation_is_correct() {
        let result = sum([1.0, 2.0, 3.0], unlimited())
            .expect("valid accumulation");

        assert_eq!(result.value(), 6.0);
        assert_eq!(result.term_count(), 3);
    }

    #[test]
    fn multiplicative_accumulation_is_correct() {
        let result = product([0.5, 0.8, 0.9], unlimited())
            .expect("valid accumulation");

        assert!((result.value() - 0.36).abs() < 1.0e-14);
    }

    #[test]
    fn root_sum_square_is_correct() {
        let result = root_sum_square([3.0, 4.0], unlimited())
            .expect("valid accumulation");

        assert!((result.value() - 5.0).abs() < 1.0e-14);
    }

    #[test]
    fn worst_case_uses_absolute_magnitude() {
        let result = worst_case([-2.0, 3.0, -4.0], unlimited())
            .expect("valid accumulation");

        assert_eq!(result.value(), 9.0);
    }

    #[test]
    fn weighted_sum_is_correct() {
        let result = weighted_sum(
            [(2.0, 0.5), (4.0, 2.0)],
            unlimited(),
        )
        .expect("valid accumulation");

        assert!((result.value() - 9.0).abs() < 1.0e-14);
    }

    #[test]
    fn weighted_mean_is_correct() {
        let result = weighted_mean(
            [(2.0, 1.0), (4.0, 3.0)],
            unlimited(),
        )
        .expect("valid weighted mean");

        assert!((result - 3.5).abs() < 1.0e-14);
    }

    #[test]
    fn mean_is_correct() {
        let result = mean([2.0, 4.0, 6.0], unlimited())
            .expect("valid mean");

        assert!((result - 4.0).abs() < 1.0e-14);
    }

    #[test]
    fn first_order_worst_case_is_correct() {
        let result = first_order_worst_case(
            [(2.0, 0.1), (-3.0, 0.2)],
            unlimited(),
        )
        .expect("valid first-order bound");

        assert!((result.value() - 0.8).abs() < 1.0e-14);
    }

    #[test]
    fn first_order_independent_uncertainty_is_correct() {
        let result = first_order_independent_uncertainty(
            [(2.0, 0.1), (3.0, 0.2)],
            unlimited(),
        )
        .expect("valid uncertainty");

        let expected = (0.04 + 0.36_f64).sqrt();

        assert!((result.value() - expected).abs() < 1.0e-14);
    }

    #[test]
    fn maximum_absolute_is_correct() {
        let result = maximum_absolute(
            [-1.0, 7.0, -3.0],
            unlimited(),
        )
        .expect("valid maximum");

        assert_eq!(result.value(), 7.0);
    }

    #[test]
    fn custom_composition_is_supported() {
        let result = accumulate_with(
            1.0,
            [2.0, 3.0],
            unlimited(),
            |current, next| current * next,
        )
        .expect("valid custom composition");

        assert_eq!(result.value(), 6.0);
    }

    #[test]
    fn streaming_does_not_require_materializing_terms() {
        let values = (1_u64..=10_000).map(|value| value as f64);

        let result = sum(values, unlimited())
            .expect("valid streaming accumulation");

        assert_eq!(result.term_count(), 10_000);
    }

    #[test]
    fn resource_limit_is_enforced() {
        let limits = AccumulationLimits {
            max_terms: Some(2),
            max_work_units: None,
        };

        let result = sum([1.0, 2.0, 3.0], limits);

        assert!(matches!(
            result,
            Err(AccumulationError::ResourceLimitExceeded {
                resource: "terms",
                ..
            })
        ));
    }

    #[test]
    fn nan_is_rejected() {
        let result = sum([1.0, f64::NAN], unlimited());

        assert!(matches!(
            result,
            Err(AccumulationError::NonFinite { .. })
        ));
    }

    #[test]
    fn positive_infinity_is_rejected() {
        let result = sum([1.0, f64::INFINITY], unlimited());

        assert!(matches!(
            result,
            Err(AccumulationError::NonFinite { .. })
        ));
    }

    #[test]
    fn negative_infinity_is_rejected() {
        let result = sum([f64::NEG_INFINITY], unlimited());

        assert!(matches!(
            result,
            Err(AccumulationError::NonFinite { .. })
        ));
    }

    #[test]
    fn zero_weight_mean_is_rejected() {
        let result = weighted_mean(
            [(1.0, 0.0), (2.0, 0.0)],
            unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn negative_weight_is_rejected() {
        let result = weighted_sum(
            [(1.0, -1.0)],
            unlimited(),
        );

        assert!(matches!(
            result,
            Err(AccumulationError::InvalidWeight { .. })
        ));
    }

    #[test]
    fn empty_mean_is_rejected() {
        let result = mean(core::iter::empty::<f64>(), unlimited());

        assert!(result.is_err());
    }

    #[test]
    fn limits_validate() {
        let invalid = AccumulationLimits {
            max_terms: Some(0),
            max_work_units: None,
        };

        assert!(invalid.validate().is_err());
    }

    #[test]
    fn custom_composition_rejects_non_finite_result() {
        let result = accumulate_with(
            1.0,
            [2.0],
            unlimited(),
            |_current, _next| f64::INFINITY,
        );

        assert!(matches!(
            result,
            Err(AccumulationError::InvalidCompositionResult { .. })
        ));
    }

    #[test]
    fn deterministic_order_is_preserved() {
        let first = sum([1.0, 2.0, 3.0], unlimited())
            .expect("valid accumulation");

        let second = sum([1.0, 2.0, 3.0], unlimited())
            .expect("valid accumulation");

        assert_eq!(first, second);
    }
}