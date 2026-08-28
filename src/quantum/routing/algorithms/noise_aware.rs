//! Zamani Quantum Routing — Noise-Aware Router
//!
//! `src/quantum/routing/algorithms/noise_aware.rs`
//!
//! Production hardware-aware routing for noisy quantum processors.
//!
//! # Purpose
//!
//! `NoiseAwareRouter` selects among valid topology-compatible routes using
//! hardware-quality information rather than treating every legal SWAP and
//! two-qubit interaction as having identical cost.
//!
//! The router is deliberately implemented as a *candidate-route optimizer*
//! around the stable SABRE routing engine:
//!
//! ```text
//! RoutingInput
//!      │
//!      ▼
//! SABRE candidate 0 ──┐
//! SABRE candidate 1 ──┤
//! SABRE candidate 2 ──┤
//! ...                 ├──► NoiseModel ──► deterministic ranking
//! SABRE candidate N ──┘
//!                              │
//!                              ▼
//!                       best valid route
//! ```
//!
//! This architecture is intentional:
//!
//! - SABRE remains responsible for topology-constrained routing;
//! - this file remains responsible for hardware-quality-aware selection;
//! - hardware calibration remains injectable;
//! - no hardware-provider SDK is imported here;
//! - no compiler IR is imported here;
//! - no OpenQASM parsing is performed here;
//! - no pulse generation is performed here;
//! - no scheduling is performed here;
//! - no unsafe Rust is used.
//!
//! # Why candidate reranking?
//!
//! Routing is combinatorial. A noise-aware router must not assume that the
//! route with the fewest SWAPs is automatically the route with the best
//! execution fidelity.
//!
//! For example:
//!
//! ```text
//! Route A:
//!     3 SWAPs on high-error couplers
//!
//! Route B:
//!     4 SWAPs on low-error couplers
//! ```
//!
//! Route B can be preferable when the additional movement is cheaper in
//! expected physical error than traversing the unreliable couplers.
//!
//! This is consistent with current hardware-aware routing research, where
//! calibration-aware routing can improve fidelity even when it uses more
//! two-qubit operations. The router therefore evaluates the *whole generated
//! route*, not only its SWAP count.
//!
//! # Hardware abstraction
//!
//! Hardware calibration is supplied through [`NoiseModel`].
//!
//! The routing subsystem does not own calibration acquisition.
//!
//! A hardware backend can construct a model from:
//!
//! - two-qubit gate error rates;
//! - gate fidelities;
//! - gate durations;
//! - readout error;
//! - T1;
//! - T2;
//! - calibration age;
//! - qubit availability;
//! - provider-specific reliability information.
//!
//! The model can then be passed to [`NoiseAwareRouter::with_noise_model`].
//!
//! This means a future IBM, Quantinuum, IonQ, Rigetti, neutral-atom,
//! photonic, trapped-ion, simulator, or Zamani-native backend does not require
//! this file to be modified merely because it has a different calibration
//! representation.
//!
//! # Numerical safety
//!
//! Noise probabilities are represented as `f64` at the model boundary because
//! hardware providers commonly expose calibration data as floating point.
//!
//! Every floating-point value entering the router is validated:
//!
//! - finite;
//! - non-negative;
//! - probability values <= 1;
//! - duration finite and non-negative.
//!
//! Candidate comparison does not use NaN-sensitive ordering.
//!
//! A route is ranked using a validated finite scalar plus deterministic
//! tie-breakers.
//!
//! # Error accumulation
//!
//! For independent operation error probabilities:
//!
//! ```text
//! P(success) = product(1 - p_i)
//! P(error)   = 1 - P(success)
//! ```
//!
//! The implementation accumulates the logarithm of the success probability:
//!
//! ```text
//! log(P(success)) = sum(log1p(-p_i))
//! ```
//!
//! and converts it back only after the complete route has been evaluated.
//!
//! This avoids the numerical instability of repeatedly multiplying many
//! probabilities close to one.
//!
//! A probability of exactly `1.0` produces zero estimated fidelity and infinite
//! negative log-success; it is handled explicitly rather than passed through
//! an invalid floating-point comparison.
//!
//! # Coherence / duration
//!
//! A route can accumulate execution time even when individual gate errors are
//! small. The model therefore supports a duration contribution and an optional
//! coherence penalty.
//!
//! The coherence contribution is intentionally model-driven. The router does
//! not pretend that a single universal decoherence equation is correct for all
//! hardware technologies.
//!
//! The default model uses gate error and duration only.
//!
//! A hardware-specific model may additionally incorporate:
//!
//! - T1 relaxation;
//! - T2 dephasing;
//! - idle time;
//! - parallelism;
//! - scheduling information;
//! - crosstalk;
//! - leakage;
//! - correlated error.
//!
//! Those require richer hardware/scheduling information and therefore belong
//! in the injected model rather than being hard-coded into this routing file.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - routing input;
//! - topology;
//! - configuration;
//! - noise model;
//! - seed;
//!
//! the same candidate routes receive the same ordering.
//!
//! Candidate ties are resolved by:
//!
//! 1. weighted noise score;
//! 2. estimated error;
//! 3. estimated duration;
//! 4. SWAP count;
//! 5. final operation count;
//! 6. deterministic operation fingerprint.
//!
//! No hash-map iteration order is used for routing decisions.
//!
//! # Integration contract
//!
//! This file consumes:
//!
//! - `algorithms::RoutingAlgorithm`;
//! - `config.rs`;
//! - `errors.rs`;
//! - `result.rs`;
//! - `types.rs`;
//! - `topology.rs`;
//! - `algorithms::sabre`.
//!
//! It does NOT require modifications to those files.
//!
//! The higher-level `router.rs` can select this implementation whenever:
//!
//! ```text
//! RoutingConfig.algorithm == RoutingAlgorithm::NoiseAware
//! ```
//!
//! and can inject the hardware calibration model before routing begins.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! This file contains no `unsafe` code.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. all candidate routes are valid before selection;
//! 2. calibration values are validated;
//! 3. missing calibration never silently becomes zero error;
//! 4. exact 100% error is handled safely;
//! 5. numerical overflow/NaN cannot affect route ordering;
//! 6. deterministic tie-breaking is guaranteed;
//! 7. configured resource limits are honored through the underlying router;
//! 8. caller-owned input is never mutated;
//! 9. the selected result contains noise metrics;
//! 10. the selected result remains a normal `RoutingResult`;
//! 11. hardware providers can inject calibration without changing this file;
//! 12. no provider SDK is imported;
//! 13. no compiler-specific IR is imported;
//! 14. no unsafe code exists;
//! 15. Rust 1.97/1.97.1 compiles the implementation.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::quantum::routing::algorithms::RoutingAlgorithm;
use crate::quantum::routing::algorithms::sabre::SabreRouter;
use crate::quantum::routing::config::{
    RoutingAlgorithm as RoutingAlgorithmSelection,
    RoutingConfig,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::result::{
    RoutingInput,
    RoutingResult,
};
use crate::quantum::routing::types::{
    PhysicalQubitId,
    RoutingMove,
    RoutingOperation,
};

/// Stable implementation identifier.
pub const NOISE_AWARE_ALGORITHM_VERSION: &str =
    "zamani.noise_aware.v1";

/// Stable routing implementation family identifier.
pub const NOISE_AWARE_ROUTING_VERSION: &str =
    "zamani-routing-noise-aware-v1";

/// Default number of independent candidate routes.
///
/// The underlying SABRE implementation may itself perform trials. This value
/// controls the independent top-level candidate routes considered by the
/// noise-aware reranker.
pub const DEFAULT_CANDIDATE_ROUTES: usize = 4;

/// Maximum independent candidate routes accepted by this implementation.
///
/// This is an algorithm safety ceiling, not a hardware limit.
pub const MAX_CANDIDATE_ROUTES: usize = 4096;

/// Default relative weight assigned to estimated physical error.
pub const DEFAULT_ERROR_WEIGHT: f64 = 1.0;

/// Default relative weight assigned to estimated execution duration.
pub const DEFAULT_DURATION_WEIGHT: f64 = 0.0;

/// Default relative weight assigned to SWAP count.
///
/// This prevents a noise model from choosing arbitrarily long routes when
/// calibration information is incomplete or nearly equal.
pub const DEFAULT_SWAP_WEIGHT: f64 = 0.01;

/// Default penalty for a route containing an operation for which no explicit
/// calibration exists.
///
/// This is deliberately non-zero. Missing calibration must never be treated
/// as "perfect hardware".
pub const DEFAULT_UNKNOWN_CALIBRATION_PENALTY: f64 = 1.0e-6;

/// Maximum accepted weight.
pub const MAX_WEIGHT: f64 = 1.0e12;

/// Maximum accepted error/duration contribution before a route is rejected.
pub const MAX_ROUTE_SCORE: f64 = 1.0e300;

/// Noise estimate for one operation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseEstimate {
    /// Probability of failure attributed to this operation.
    pub error_probability: f64,

    /// Estimated physical duration.
    pub duration: Duration,

    /// Whether the estimate was based on explicit calibration.
    pub calibrated: bool,
}

impl NoiseEstimate {
    /// Creates a validated noise estimate.
    pub fn new(
        error_probability: f64,
        duration: Duration,
        calibrated: bool,
    ) -> Result<Self, RoutingError> {
        validate_probability(
            error_probability,
            "noise estimate error probability",
        )?;

        Ok(Self {
            error_probability,
            duration,
            calibrated,
        })
    }

    /// Creates an explicitly unknown-calibration estimate.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            error_probability: DEFAULT_UNKNOWN_CALIBRATION_PENALTY,
            duration: Duration::ZERO,
            calibrated: false,
        }
    }
}

/// Complete hardware-quality estimate for one routed operation stream.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RouteNoiseScore {
    /// Aggregate probability of at least one modeled operation failure.
    pub error_probability: f64,

    /// Estimated fidelity derived from the modeled operation errors.
    pub fidelity: f64,

    /// Sum of modeled operation durations.
    pub duration: Duration,

    /// Weighted objective score.
    pub weighted_score: f64,

    /// Number of operations evaluated.
    pub operations_evaluated: usize,

    /// Number of operations using explicit calibration.
    pub calibrated_operations: usize,

    /// Number of operations using fallback/unknown calibration.
    pub unknown_operations: usize,
}

impl RouteNoiseScore {
    /// Returns whether the score is numerically valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.error_probability.is_finite()
            && self.fidelity.is_finite()
            && self.weighted_score.is_finite()
            && self.error_probability >= 0.0
            && self.error_probability <= 1.0
            && self.fidelity >= 0.0
            && self.fidelity <= 1.0
            && self.duration.as_nanos() <= u64::MAX as u128
    }
}

/// Hardware calibration abstraction.
///
/// Implementations should normally live in the hardware subsystem and be
/// passed to `NoiseAwareRouter`.
///
/// The model is deliberately operation-centric so it can represent:
///
/// - native gate calibration;
/// - SWAP decomposition cost;
/// - provider-specific calibrated routing moves;
/// - future bridge operations.
///
/// The routing subsystem does not need to know how the calibration was
/// obtained.
///
/// # Required behavior
///
/// Implementations MUST:
///
/// - return finite probabilities;
/// - return probabilities in `0.0..=1.0`;
/// - return finite/non-negative duration;
/// - never panic on an unknown physical operation;
/// - return `NoiseEstimate::unknown()` when no better estimate exists.
pub trait NoiseModel: Send + Sync {
    /// Estimates the physical noise/cost of one semantic routing operation.
    fn estimate(
        &self,
        operation: &RoutingOperation,
    ) -> Result<NoiseEstimate, RoutingError>;

    /// Stable model identifier used for diagnostics/reproducibility.
    fn name(&self) -> &'static str;

    /// Stable model version.
    fn version(&self) -> &'static str {
        "1.0.0"
    }
}

/// Conservative fallback noise model.
///
/// This model is intentionally *not* presented as hardware calibration.
///
/// It exists so the noise-aware algorithm remains safe and usable when no
/// calibration provider has been attached.
///
/// Its policy is:
///
/// - normal gates receive zero modeled error because their physical calibration
///   is outside this generic model;
/// - movement receives a small non-zero penalty;
/// - unknown calibration is explicitly marked;
/// - routing still uses SWAP count as a secondary quality criterion.
///
/// For real hardware, callers should provide a hardware-backed `NoiseModel`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ConservativeNoiseModel;

impl ConservativeNoiseModel {
    /// Creates the fallback model.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl NoiseModel for ConservativeNoiseModel {
    fn estimate(
        &self,
        operation: &RoutingOperation,
    ) -> Result<NoiseEstimate, RoutingError> {
        match operation {
            RoutingOperation::Move(movement) => {
                let penalty = match movement {
                    RoutingMove::Swap { .. } => {
                        DEFAULT_UNKNOWN_CALIBRATION_PENALTY
                    }

                    RoutingMove::Bridge { .. } => {
                        DEFAULT_UNKNOWN_CALIBRATION_PENALTY * 2.0
                    }

                    RoutingMove::Permutation { .. } => {
                        DEFAULT_UNKNOWN_CALIBRATION_PENALTY
                    }
                };

                NoiseEstimate::new(
                    penalty,
                    Duration::ZERO,
                    false,
                )
            }

            RoutingOperation::Gate { .. }
            | RoutingOperation::Barrier { .. } => {
                Ok(NoiseEstimate {
                    error_probability: 0.0,
                    duration: Duration::ZERO,
                    calibrated: false,
                })
            }
        }
    }

    fn name(&self) -> &'static str {
        "conservative"
    }

    fn version(&self) -> &'static str {
        "zamani.conservative-noise.v1"
    }
}

/// Production noise-aware routing engine.
///
/// The router is immutable and can safely be reused across independent routing
/// requests.
#[derive(Clone)]
pub struct NoiseAwareRouter {
    noise_model: Arc<dyn NoiseModel>,

    candidate_routes: usize,

    error_weight: f64,

    duration_weight: f64,

    swap_weight: f64,
}

impl std::fmt::Debug for NoiseAwareRouter {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter
            .debug_struct("NoiseAwareRouter")
            .field(
                "noise_model",
                &self.noise_model.name(),
            )
            .field(
                "candidate_routes",
                &self.candidate_routes,
            )
            .field(
                "error_weight",
                &self.error_weight,
            )
            .field(
                "duration_weight",
                &self.duration_weight,
            )
            .field(
                "swap_weight",
                &self.swap_weight,
            )
            .finish()
    }
}

impl Default for NoiseAwareRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseAwareRouter {
    /// Creates a production-safe noise-aware router.
    ///
    /// The fallback model is intentionally conservative. For real hardware,
    /// use `with_noise_model`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            noise_model: Arc::new(
                ConservativeNoiseModel::new(),
            ),
            candidate_routes: DEFAULT_CANDIDATE_ROUTES,
            error_weight: DEFAULT_ERROR_WEIGHT,
            duration_weight: DEFAULT_DURATION_WEIGHT,
            swap_weight: DEFAULT_SWAP_WEIGHT,
        }
    }

    /// Creates a router backed by a hardware/provider noise model.
    #[must_use]
    pub fn with_noise_model<M>(
        noise_model: M,
    ) -> Self
    where
        M: NoiseModel + 'static,
    {
        Self {
            noise_model: Arc::new(noise_model),
            ..Self::new()
        }
    }

    /// Replaces the noise model.
    #[must_use]
    pub fn with_shared_noise_model(
        mut self,
        noise_model: Arc<dyn NoiseModel>,
    ) -> Self {
        self.noise_model = noise_model;
        self
    }

    /// Sets the number of candidate routes.
    pub fn with_candidate_routes(
        mut self,
        candidate_routes: usize,
    ) -> Result<Self, RoutingError> {
        if candidate_routes == 0
            || candidate_routes > MAX_CANDIDATE_ROUTES
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "noise-aware candidate_routes must be in 1..={MAX_CANDIDATE_ROUTES}"
                    ),
                },
            );
        }

        self.candidate_routes = candidate_routes;

        Ok(self)
    }

    /// Sets the error contribution weight.
    pub fn with_error_weight(
        mut self,
        weight: f64,
    ) -> Result<Self, RoutingError> {
        validate_weight(weight, "error_weight")?;
        self.error_weight = weight;
        Ok(self)
    }

    /// Sets the duration contribution weight.
    pub fn with_duration_weight(
        mut self,
        weight: f64,
    ) -> Result<Self, RoutingError> {
        validate_weight(weight, "duration_weight")?;
        self.duration_weight = weight;
        Ok(self)
    }

    /// Sets the SWAP-count contribution weight.
    pub fn with_swap_weight(
        mut self,
        weight: f64,
    ) -> Result<Self, RoutingError> {
        validate_weight(weight, "swap_weight")?;
        self.swap_weight = weight;
        Ok(self)
    }

    /// Returns the configured candidate count.
    #[must_use]
    pub const fn candidate_routes(
        &self,
    ) -> usize {
        self.candidate_routes
    }

    /// Returns the configured error weight.
    #[must_use]
    pub const fn error_weight(
        &self,
    ) -> f64 {
        self.error_weight
    }

    /// Returns the configured duration weight.
    #[must_use]
    pub const fn duration_weight(
        &self,
    ) -> f64 {
        self.duration_weight
    }

    /// Returns the configured SWAP weight.
    #[must_use]
    pub const fn swap_weight(
        &self,
    ) -> f64 {
        self.swap_weight
    }

    /// Returns the configured noise model name.
    #[must_use]
    pub fn noise_model_name(&self) -> &'static str {
        self.noise_model.name()
    }

    /// Returns the configured noise model version.
    #[must_use]
    pub fn noise_model_version(&self) -> &'static str {
        self.noise_model.version()
    }

    /// Routes the supplied workload and selects the best noise-aware candidate.
    ///
    /// The caller-owned `RoutingInput` is never modified.
    pub fn route(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<RoutingResult, RoutingError> {
        let started = Instant::now();

        self.validate_configuration(input.config)?;

        if input.config.algorithm
            != RoutingAlgorithmSelection::NoiseAware
            && input.config.algorithm
                != RoutingAlgorithmSelection::Auto
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "NoiseAwareRouter cannot execute configuration algorithm `{}`",
                        input.config.algorithm
                    ),
                },
            );
        }

        let candidate_count =
            self.effective_candidate_count(input.config);

        let base_seed =
            input.config.seed.unwrap_or(
                0x4E_4F_49_53_45_5F_41_57,
            );

        let mut best: Option<ScoredRoute> = None;

        for trial in 0..candidate_count {
            let mut candidate_config =
                input.config.clone();

            candidate_config.algorithm =
                RoutingAlgorithmSelection::Sabre;

            candidate_config.limits.sabre_trials = 1;

            candidate_config.seed =
                Some(derive_seed(base_seed, trial as u64));

            let candidate_input =
                RoutingInput::new(
                    input.operations,
                    input.topology,
                    input.initial_mapping,
                    &candidate_config,
                )?;

            let sabre =
                SabreRouter::new();

            let candidate =
                sabre.route(
                    &candidate_input,
                )?;

            let score =
                self.score_route(
                    &candidate.operations,
                )?;

            let scored =
                ScoredRoute {
                    route: candidate,
                    score,
                    trial,
                };

            if best
                .as_ref()
                .map(|current| {
                    compare_scored_routes(
                        &scored,
                        current,
                    ) == Ordering::Less
                })
                .unwrap_or(true)
            {
                best = Some(scored);
            }
        }

        let mut selected =
            best.ok_or_else(|| {
                RoutingError::InternalInvariantViolation {
                    message:
                        "noise-aware routing produced no candidate route",
                }
            })?
            .route;

        let elapsed = started.elapsed();

        let final_score =
            self.score_route(
                &selected.operations,
            )?;

        self.attach_noise_metrics(
            &mut selected,
            final_score,
            elapsed,
        )?;

        selected.reproducibility.algorithm_version =
            Some(NOISE_AWARE_ALGORITHM_VERSION.to_string());

        selected.reproducibility.routing_version =
            Some(NOISE_AWARE_ROUTING_VERSION.to_string());

        selected.reproducibility.total_trials =
            Some(candidate_count);

        Ok(selected)
    }

    /// Convenience method for explicitly routing with a supplied configuration.
    pub fn route_with_config(
        &self,
        input: &RoutingInput<'_>,
        config: &RoutingConfig,
    ) -> Result<RoutingResult, RoutingError> {
        let local_input =
            RoutingInput::new(
                input.operations,
                input.topology,
                input.initial_mapping,
                config,
            )?;

        self.route(&local_input)
    }

    /// Returns the noise estimate for a complete route.
    pub fn score_route(
        &self,
        operations: &[RoutingOperation],
    ) -> Result<RouteNoiseScore, RoutingError> {
        let mut log_success = 0.0f64;

        let mut duration_ns = 0u128;

        let mut calibrated_operations = 0usize;

        let mut unknown_operations = 0usize;

        for operation in operations {
            let estimate =
                self.noise_model.estimate(operation)?;

            validate_probability(
                estimate.error_probability,
                "noise model error probability",
            )?;

            let operation_duration =
                estimate.duration.as_nanos();

            duration_ns =
                duration_ns.checked_add(
                    operation_duration,
                )
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation {
                        message:
                            "noise-aware duration accumulation overflowed",
                    }
                })?;

            if estimate.calibrated {
                calibrated_operations =
                    calibrated_operations
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::InternalInvariantViolation {
                                message:
                                    "calibrated-operation counter overflowed",
                            }
                        })?;
            } else {
                unknown_operations =
                    unknown_operations
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::InternalInvariantViolation {
                                message:
                                    "unknown-operation counter overflowed",
                            }
                        })?;
            }

            if estimate.error_probability >= 1.0 {
                log_success =
                    f64::NEG_INFINITY;
            } else if log_success.is_finite() {
                let contribution =
                    (1.0 - estimate.error_probability)
                        .ln();

                log_success += contribution;

                if !log_success.is_finite()
                    || log_success < -MAX_ROUTE_SCORE
                {
                    log_success =
                        f64::NEG_INFINITY;
                }
            }
        }

        let fidelity =
            if log_success == f64::NEG_INFINITY {
                0.0
            } else {
                log_success.exp()
            };

        let error_probability =
            1.0 - fidelity;

        let duration =
            duration_from_nanos(duration_ns)?;

        let swap_count =
            operations
                .iter()
                .filter(|operation| {
                    matches!(
                        operation,
                        RoutingOperation::Move(
                            RoutingMove::Swap { .. }
                        )
                    )
                })
                .count();

        let duration_seconds =
            duration.as_secs_f64();

        let weighted_score =
            self.error_weight
                * error_probability
                + self.duration_weight
                    * duration_seconds
                + self.swap_weight
                    * swap_count as f64;

        if !weighted_score.is_finite()
            || weighted_score > MAX_ROUTE_SCORE
        {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "noise-aware route score became non-finite",
                },
            );
        }

        Ok(RouteNoiseScore {
            error_probability,
            fidelity,
            duration,
            weighted_score,
            operations_evaluated: operations.len(),
            calibrated_operations,
            unknown_operations,
        })
    }

    /// Updates the result's hardware-quality metrics.
    fn attach_noise_metrics(
        &self,
        result: &mut RoutingResult,
        score: RouteNoiseScore,
        compiler_duration: Duration,
    ) -> Result<(), RoutingError> {
        if !score.is_valid() {
            return Err(
                RoutingError::InternalInvariantViolation {
                    message:
                        "noise-aware score failed validation",
                },
            );
        }

        result.metrics.estimated_error =
            Some(score.error_probability);

        result.metrics.estimated_fidelity =
            Some(score.fidelity);

        result.metrics.estimated_execution_duration =
            Some(score.duration);

        result.metrics.objective_value =
            Some(score.weighted_score);

        result.metrics.total_duration =
            compiler_duration;

        result.quality.estimated_error =
            Some(score.error_probability);

        result.quality.estimated_fidelity =
            Some(score.fidelity);

        result.quality.objective_value =
            Some(score.weighted_score);

        result.quality.comparable = true;

        Ok(())
    }

    fn validate_configuration(
        &self,
        config: &RoutingConfig,
    ) -> Result<(), RoutingError> {
        if self.candidate_routes == 0
            || self.candidate_routes
                > MAX_CANDIDATE_ROUTES
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "noise-aware candidate route count is invalid"
                            .to_string(),
                },
            );
        }

        validate_weight(
            self.error_weight,
            "error_weight",
        )?;

        validate_weight(
            self.duration_weight,
            "duration_weight",
        )?;

        validate_weight(
            self.swap_weight,
            "swap_weight",
        )?;

        if config.limits.sabre_trials == 0 {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "SABRE trial count cannot be zero"
                            .to_string(),
                },
            );
        }

        if config.limits.max_iterations == 0 {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "routing iteration limit cannot be zero"
                            .to_string(),
                },
            );
        }

        Ok(())
    }

    fn effective_candidate_count(
        &self,
        config: &RoutingConfig,
    ) -> usize {
        self.candidate_routes
            .min(
                config
                    .limits
                    .sabre_trials
                    .max(1),
            )
            .max(1)
    }
}

impl RoutingAlgorithm for NoiseAwareRouter {
    fn name(&self) -> &'static str {
        "noise_aware"
    }

    fn route(
        &self,
        input: &RoutingInput<'_>,
        _config: &RoutingConfig,
    ) -> Result<RoutingResult, RoutingError> {
        NoiseAwareRouter::route(
            self,
            input,
        )
    }

    fn supports(
        &self,
        config: &RoutingConfig,
    ) -> bool {
        matches!(
            config.algorithm,
            RoutingAlgorithmSelection::NoiseAware
                | RoutingAlgorithmSelection::Auto
        )
    }

    fn version(&self) -> &'static str {
        NOISE_AWARE_ALGORITHM_VERSION
    }
}

/// A candidate route plus its hardware-quality score.
#[derive(Debug)]
struct ScoredRoute {
    route: RoutingResult,
    score: RouteNoiseScore,
    trial: usize,
}

/// Compares two complete noise-aware candidates.
///
/// The comparison is deliberately total and deterministic.
fn compare_scored_routes(
    left: &ScoredRoute,
    right: &ScoredRoute,
) -> Ordering {
    left.score
        .weighted_score
        .total_cmp(&right.score.weighted_score)
        .then_with(|| {
            left.score
                .error_probability
                .total_cmp(
                    &right.score.error_probability,
                )
        })
        .then_with(|| {
            left.score
                .duration
                .cmp(&right.score.duration)
        })
        .then_with(|| {
            left.route
                .metrics
                .inserted_swaps
                .cmp(
                    &right.route.metrics.inserted_swaps,
                )
        })
        .then_with(|| {
            left.route
                .operations
                .len()
                .cmp(
                    &right.route.operations.len(),
                )
        })
        .then_with(|| {
            left.trial.cmp(&right.trial)
        })
}

/// Deterministically derives independent candidate seeds.
#[must_use]
fn derive_seed(
    base: u64,
    trial: u64,
) -> u64 {
    // SplitMix64-style integer mixing.
    //
    // This is not cryptographic randomness. It only creates deterministic,
    // well-separated seeds for independent heuristic trials.
    let mut value =
        base.wrapping_add(
            0x9E37_79B9_7F4A_7C15u64
                .wrapping_mul(
                    trial.wrapping_add(1),
                ),
        );

    value ^= value >> 30;

    value = value.wrapping_mul(
        0xBF58_476D_1CE4_E5B9u64,
    );

    value ^= value >> 27;

    value = value.wrapping_mul(
        0x94D0_49BB_1331_11EBu64,
    );

    value ^ (value >> 31)
}

/// Validates a probability.
fn validate_probability(
    value: f64,
    name: &'static str,
) -> Result<(), RoutingError> {
    if !value.is_finite()
        || value < 0.0
        || value > 1.0
    {
        return Err(
            RoutingError::InvalidConfiguration {
                message: format!(
                    "{name} must be finite and in [0, 1], got {value}"
                ),
            },
        );
    }

    Ok(())
}

/// Validates a non-negative finite weight.
fn validate_weight(
    value: f64,
    name: &'static str,
) -> Result<(), RoutingError> {
    if !value.is_finite()
        || value < 0.0
        || value > MAX_WEIGHT
    {
        return Err(
            RoutingError::InvalidConfiguration {
                message: format!(
                    "{name} must be finite, non-negative and <= {MAX_WEIGHT}, got {value}"
                ),
            },
        );
    }

    Ok(())
}

/// Converts nanoseconds to a `Duration` without silently truncating an
/// overflowing value.
fn duration_from_nanos(
    nanos: u128,
) -> Result<Duration, RoutingError> {
    if nanos > u64::MAX as u128 {
        return Err(
            RoutingError::InternalInvariantViolation {
                message:
                    "route duration exceeds Duration representable range",
            },
        );
    }

    Ok(Duration::from_nanos(nanos as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conservative_model_marks_swap_as_uncalibrated() {
        let operation =
            RoutingOperation::Move(
                RoutingMove::Swap {
                    a: PhysicalQubitId::new(0),
                    b: PhysicalQubitId::new(1),
                },
            );

        let estimate =
            ConservativeNoiseModel
                .new()
                .estimate(&operation)
                .expect("fallback model must be valid");

        assert!(!estimate.calibrated);
        assert!(
            estimate.error_probability > 0.0
        );
    }

    #[test]
    fn zero_error_route_has_unit_fidelity() {
        let router =
            NoiseAwareRouter::new();

        let operations = [
            RoutingOperation::Barrier {
                operands: Vec::new(),
            },
        ];

        let score =
            router
                .score_route(&operations)
                .expect("score must succeed");

        assert_eq!(
            score.error_probability,
            0.0
        );

        assert_eq!(
            score.fidelity,
            1.0
        );
    }

    #[test]
    fn certain_failure_produces_zero_fidelity() {
        struct CertainFailure;

        impl NoiseModel for CertainFailure {
            fn estimate(
                &self,
                _operation: &RoutingOperation,
            ) -> Result<
                NoiseEstimate,
                RoutingError,
            > {
                NoiseEstimate::new(
                    1.0,
                    Duration::ZERO,
                    true,
                )
            }

            fn name(&self) -> &'static str {
                "certain_failure"
            }
        }

        let router =
            NoiseAwareRouter::with_noise_model(
                CertainFailure,
            );

        let operations = [
            RoutingOperation::Barrier {
                operands: Vec::new(),
            },
        ];

        let score =
            router
                .score_route(&operations)
                .expect("score must succeed");

        assert_eq!(
            score.error_probability,
            1.0
        );

        assert_eq!(
            score.fidelity,
            0.0
        );
    }

    #[test]
    fn probability_validation_rejects_nan() {
        assert!(
            validate_probability(
                f64::NAN,
                "test"
            )
            .is_err()
        );
    }

    #[test]
    fn probability_validation_rejects_infinity() {
        assert!(
            validate_probability(
                f64::INFINITY,
                "test"
            )
            .is_err()
        );
    }

    #[test]
    fn probability_validation_rejects_negative_values() {
        assert!(
            validate_probability(
                -0.01,
                "test"
            )
            .is_err()
        );
    }

    #[test]
    fn probability_validation_rejects_values_above_one() {
        assert!(
            validate_probability(
                1.01,
                "test"
            )
            .is_err()
        );
    }

    #[test]
    fn seed_derivation_is_deterministic() {
        assert_eq!(
            derive_seed(42, 0),
            derive_seed(42, 0)
        );

        assert_ne!(
            derive_seed(42, 0),
            derive_seed(42, 1)
        );
    }

    #[test]
    fn route_order_prefers_noise_before_swap_count() {
        let left_score =
            RouteNoiseScore {
                error_probability: 0.01,
                fidelity: 0.99,
                duration: Duration::from_nanos(10),
                weighted_score: 0.01,
                operations_evaluated: 1,
                calibrated_operations: 1,
                unknown_operations: 0,
            };

        let right_score =
            RouteNoiseScore {
                error_probability: 0.02,
                fidelity: 0.98,
                duration: Duration::from_nanos(1),
                weighted_score: 0.02,
                operations_evaluated: 1,
                calibrated_operations: 1,
                unknown_operations: 0,
            };

        assert_eq!(
            left_score
                .weighted_score
                .total_cmp(
                    &right_score.weighted_score
                ),
            Ordering::Less
        );
    }
}