//! Zamani Quantum Optimization — Randomized Search Pass
//!
//! This module provides a backend-independent stochastic optimization engine
//! over the canonical `crate::quantum::ir::QuantumCircuit`. It deliberately
//! does not define another circuit representation, does not execute circuits,
//! and does not communicate with hardware.
//!
//! # Architecture
//!
//! ```text
//! canonical QuantumCircuit
//!          │
//!          ▼
//! randomized candidate generator ──► candidate QuantumCircuit
//!          │                           │
//!          │                           ▼
//!          │                     structural validation
//!          │                           │
//!          │                           ▼
//!          └────────────────────► objective evaluator
//!                                      │
//!                                      ▼
//!                              stochastic acceptance
//!                                      │
//!                                      ▼
//!                              best QuantumCircuit
//! ```
//!
//! The generator and evaluator are dependency-injected through stable traits
//! so this file can be completed and frozen without later edits when new
//! rewrite systems, cost models, target profiles, synthesis engines, or
//! verification implementations are added.
//!
//! # Semantic contract
//!
//! `RandomizedCandidateGenerator` is a trusted compiler component. A candidate
//! it returns must be semantically equivalent to its input under the optimizer
//! equivalence policy. This pass performs structural validation and preserves
//! the circuit identity, IR version, logical namespaces, IR limits, and
//! canonical metadata. Independent semantic verification remains owned by
//! `verification/*` and should be enabled for verified compilation.
//!
//! # Determinism
//!
//! - `Determinism::Deterministic`: randomness is derived only from the stable
//!   optimizer context seed mechanism.
//! - `Determinism::Seeded`: the configured seed is mixed with pass identity and
//!   invocation state by `OptimizationContext`.
//! - `Determinism::Nondeterministic`: an explicit `RandomEntropySource` is
//!   mandatory. No ambient process randomness is silently introduced.
//!
//! The internal PRNG is SplitMix64. It is a deterministic search PRNG, not a
//! cryptographic random-number generator. Cryptographic entropy is outside the
//! optimizer's responsibility.
//!
//! # Scaling
//!
//! The pass has no artificial circuit-size ceiling. Work is bounded by both its
//! local configuration and the global `OptimizationLimits` carried by the
//! `OptimizationContext`. Candidate generation, iterations, accepted rewrites,
//! cancellation, and IR validation are all bounded or checked. Very large
//! workloads therefore scale with available resources rather than with a
//! hidden fixed circuit-size constant.
//!
//! # Search strategies
//!
//! - `BestOfCandidates`: independent stochastic candidates are compared with
//!   the best known circuit.
//! - `RandomWalk`: candidates are generated from the incumbent and only
//!   improving moves are accepted.
//! - `SimulatedAnnealing`: improving moves are always accepted and bounded
//!   probabilistic uphill moves are permitted according to the configured
//!   temperature schedule.
//!
//! # Integration contract
//!
//! `stochastic/mod.rs` should expose this module with `pub mod randomized;` and
//! optionally re-export `RandomizedPass`, `RandomizedConfig`,
//! `SearchStrategy`, and the three service traits. `registry.rs` registers the
//! pass; `planner.rs` selects it for aggressive/stochastic profiles;
//! `pipeline.rs` invokes it; `cost.rs` implementations are adapted through
//! `RandomizedCandidateEvaluator`; rewrite/synthesis implementations are
//! adapted through `RandomizedCandidateGenerator`; and `verification/*`
//! independently verifies accepted output when required.
//!
//! The pass does not require changes to this file when those modules are added.
//!
//! # Rust and safety
//!
//! Target: Rust 1.97 / 1.97.1, Rust 2021, stable features only.
//! No `unsafe` code is permitted.

#![forbid(unsafe_code)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
    PassIdentifier,
};
use super::super::limits::OptimizationResource;
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassComplexity,
    PassDeterminism,
    PassExecutionResult,
    PassKind,
    PassMetadata,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Stable public identifiers
// =============================================================================

/// Stable optimization-pass identifier.
///
/// This identifier is part of optimization provenance and must not be changed
/// casually once released.
pub const PASS_ID: &str = "stochastic.randomized";

/// Human-readable optimization-pass name.
pub const PASS_NAME: &str = "Randomized Quantum Circuit Optimization";

// =============================================================================
// Candidate generation contract
// =============================================================================

/// Generates complete candidate circuits for stochastic search.
///
/// The generator is a trusted optimizer component: every candidate it returns
/// must preserve the quantum/classical semantics of the supplied circuit under
/// the optimizer's configured equivalence policy.
///
/// The generator may internally use:
///
/// - local rewrites;
/// - commutation;
/// - algebraic transformations;
/// - synthesis;
/// - target-aware transformations;
/// - randomized templates;
/// - domain-specific heuristics.
///
/// It must not:
///
/// - execute a QPU;
/// - perform backend I/O;
/// - mutate the supplied circuit;
/// - change the circuit's identity or namespace;
/// - return an invalid `QuantumCircuit`.
///
/// The pass still validates every returned candidate defensively.
pub trait RandomizedCandidateGenerator: Send + Sync {
    /// Produces one candidate.
    ///
    /// Returning `Ok(None)` means that this particular stochastic attempt has
    /// no candidate and is not an error.
    fn generate(
        &self,
        current: &QuantumCircuit,
        seed: u64,
        iteration: u64,
        candidate: u64,
    ) -> Result<Option<QuantumCircuit>, String>;
}

// =============================================================================
// Candidate evaluation contract
// =============================================================================

/// Scores a candidate for the active optimization objective.
///
/// Lower scores are always better.
///
/// The evaluator is deliberately independent from `cost.rs`. A future cost
/// model can implement this trait without requiring this file to be modified.
///
/// Examples of evaluator objectives include:
///
/// - total gate count;
/// - weighted gate cost;
/// - two-qubit gate count;
/// - depth;
/// - T-count;
/// - T-depth;
/// - estimated error;
/// - estimated execution duration;
/// - weighted multi-objective cost.
pub trait RandomizedCandidateEvaluator: Send + Sync {
    /// Evaluates one circuit.
    ///
    /// Implementations must return a finite `f64`.
    ///
    /// `NaN`, positive infinity, and negative infinity are rejected by the
    /// randomized pass.
    fn evaluate(&self, circuit: &QuantumCircuit) -> Result<f64, String>;
}

// =============================================================================
// Explicit nondeterministic entropy contract
// =============================================================================

/// Supplies entropy only when `Determinism::Nondeterministic` is explicitly
/// selected.
///
/// This abstraction deliberately avoids silently introducing process-global
/// randomness into the compiler.
///
/// A production application may implement this using an operating-system
/// entropy provider, a compiler-provided entropy service, or another explicitly
/// approved source.
///
/// It is never consulted in deterministic or seeded modes.
pub trait RandomEntropySource: Send + Sync {
    /// Returns one 64-bit entropy value.
    fn next_u64(&self) -> Result<u64, String>;
}

// =============================================================================
// Search strategy
// =============================================================================

/// Strategy used by the randomized optimizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchStrategy {
    /// Generate independent candidates from the current best circuit.
    ///
    /// Only strict objective improvements are accepted.
    BestOfCandidates,

    /// Generate candidates from the current incumbent.
    ///
    /// Only strict improvements are accepted.
    RandomWalk,

    /// Generate candidates from the current incumbent and permit bounded
    /// uphill moves according to simulated-annealing acceptance probability.
    SimulatedAnnealing,
}

impl Default for SearchStrategy {
    fn default() -> Self {
        Self::BestOfCandidates
    }
}

impl SearchStrategy {
    /// Returns a stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BestOfCandidates => "best_of_candidates",
            Self::RandomWalk => "random_walk",
            Self::SimulatedAnnealing => "simulated_annealing",
        }
    }
}

impl fmt::Display for SearchStrategy {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for one randomized-search pass.
///
/// These are local upper bounds. They can never increase the global
/// `OptimizationLimits` supplied through `OptimizationContext`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RandomizedConfig {
    /// Maximum number of stochastic iterations requested by this pass.
    max_iterations: u64,

    /// Maximum candidate attempts per iteration.
    candidates_per_iteration: u64,

    /// Search strategy.
    strategy: SearchStrategy,

    /// Initial simulated-annealing temperature.
    initial_temperature: f64,

    /// Multiplicative temperature cooling factor.
    ///
    /// Must be strictly between zero and one.
    cooling_rate: f64,

    /// Lower bound on simulated-annealing temperature.
    minimum_temperature: f64,
}

impl RandomizedConfig {
    /// Creates the production default configuration.
    pub fn new() -> Result<Self, RandomizedConfigError> {
        Self::default().validate()
    }

    /// Returns the raw production defaults.
    #[must_use]
    pub const fn default_values() -> Self {
        Self {
            max_iterations: 1,
            candidates_per_iteration: 8,
            strategy: SearchStrategy::BestOfCandidates,
            initial_temperature: 1.0,
            cooling_rate: 0.95,
            minimum_temperature: 1.0e-12,
        }
    }

    /// Sets the maximum number of iterations.
    #[must_use]
    pub const fn with_max_iterations(
        mut self,
        value: u64,
    ) -> Self {
        self.max_iterations = value;
        self
    }

    /// Sets the maximum number of candidate attempts per iteration.
    #[must_use]
    pub const fn with_candidates_per_iteration(
        mut self,
        value: u64,
    ) -> Self {
        self.candidates_per_iteration = value;
        self
    }

    /// Sets the search strategy.
    #[must_use]
    pub const fn with_strategy(
        mut self,
        value: SearchStrategy,
    ) -> Self {
        self.strategy = value;
        self
    }

    /// Sets the initial simulated-annealing temperature.
    #[must_use]
    pub const fn with_initial_temperature(
        mut self,
        value: f64,
    ) -> Self {
        self.initial_temperature = value;
        self
    }

    /// Sets the cooling rate.
    #[must_use]
    pub const fn with_cooling_rate(
        mut self,
        value: f64,
    ) -> Self {
        self.cooling_rate = value;
        self
    }

    /// Sets the minimum simulated-annealing temperature.
    #[must_use]
    pub const fn with_minimum_temperature(
        mut self,
        value: f64,
    ) -> Self {
        self.minimum_temperature = value;
        self
    }

    /// Returns the local iteration limit.
    #[must_use]
    pub const fn max_iterations(&self) -> u64 {
        self.max_iterations
    }

    /// Returns the local candidate limit.
    #[must_use]
    pub const fn candidates_per_iteration(&self) -> u64 {
        self.candidates_per_iteration
    }

    /// Returns the selected strategy.
    #[must_use]
    pub const fn strategy(&self) -> SearchStrategy {
        self.strategy
    }

    /// Returns the initial temperature.
    #[must_use]
    pub const fn initial_temperature(&self) -> f64 {
        self.initial_temperature
    }

    /// Returns the cooling rate.
    #[must_use]
    pub const fn cooling_rate(&self) -> f64 {
        self.cooling_rate
    }

    /// Returns the minimum temperature.
    #[must_use]
    pub const fn minimum_temperature(&self) -> f64 {
        self.minimum_temperature
    }

    /// Validates the configuration.
    pub fn validate(
        self,
    ) -> Result<Self, RandomizedConfigError> {
        if self.max_iterations == 0 {
            return Err(RandomizedConfigError::Zero {
                field: "max_iterations",
            });
        }

        if self.candidates_per_iteration == 0 {
            return Err(RandomizedConfigError::Zero {
                field: "candidates_per_iteration",
            });
        }

        if !self.initial_temperature.is_finite()
            || self.initial_temperature <= 0.0
        {
            return Err(RandomizedConfigError::InvalidFloat {
                field: "initial_temperature",
            });
        }

        if !self.cooling_rate.is_finite()
            || self.cooling_rate <= 0.0
            || self.cooling_rate >= 1.0
        {
            return Err(RandomizedConfigError::InvalidFloat {
                field: "cooling_rate",
            });
        }

        if !self.minimum_temperature.is_finite()
            || self.minimum_temperature <= 0.0
        {
            return Err(RandomizedConfigError::InvalidFloat {
                field: "minimum_temperature",
            });
        }

        if self.minimum_temperature > self.initial_temperature {
            return Err(RandomizedConfigError::InvalidRange {
                field: "minimum_temperature",
            });
        }

        Ok(self)
    }
}

impl Default for RandomizedConfig {
    fn default() -> Self {
        Self::default_values()
    }
}

// =============================================================================
// Configuration errors
// =============================================================================

/// Errors produced while constructing randomized-pass configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomizedConfigError {
    /// A required integer was zero.
    Zero {
        /// Invalid field.
        field: &'static str,
    },

    /// A floating-point value was non-finite or outside its valid range.
    InvalidFloat {
        /// Invalid field.
        field: &'static str,
    },

    /// Two numeric fields had an invalid ordering.
    InvalidRange {
        /// Invalid field.
        field: &'static str,
    },
}

impl fmt::Display for RandomizedConfigError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Zero { field } => {
                write!(
                    formatter,
                    "{field} must be greater than zero"
                )
            }

            Self::InvalidFloat { field } => {
                write!(
                    formatter,
                    "{field} must be finite and within its valid range"
                )
            }

            Self::InvalidRange { field } => {
                write!(
                    formatter,
                    "{field} must not exceed initial_temperature"
                )
            }
        }
    }
}

impl std::error::Error for RandomizedConfigError {}

// =============================================================================
// Search statistics
// =============================================================================

/// Detailed statistics produced internally by one randomized invocation.
///
/// The generic `OptimizationContext` remains the authoritative accounting
/// boundary for resource consumption. This structure describes stochastic
/// search behavior specifically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RandomizedStatistics {
    /// Number of iterations actually entered.
    pub iterations: u64,

    /// Candidate attempts generated.
    pub candidates_generated: u64,

    /// Candidates passing structural validation.
    pub candidates_valid: u64,

    /// Candidates rejected by the acceptance policy.
    pub candidates_rejected: u64,

    /// Candidates accepted by the search policy.
    pub candidates_accepted: u64,

    /// Candidates rejected because their IR was invalid.
    pub invalid_candidates: u64,

    /// Candidate evaluator failures.
    pub evaluation_failures: u64,

    /// Number of times the global best objective improved.
    pub best_score_updates: u64,
}

// =============================================================================
// Randomized pass
// =============================================================================

/// Production randomized quantum optimization pass.
///
/// The pass is stateless with respect to one optimizer invocation. Mutable
/// invocation state remains in `OptimizationContext`.
#[derive(Clone)]
pub struct RandomizedPass {
    metadata: PassMetadata,
    config: RandomizedConfig,
    generator: Arc<dyn RandomizedCandidateGenerator>,
    evaluator: Arc<dyn RandomizedCandidateEvaluator>,
    entropy: Option<Arc<dyn RandomEntropySource>>,
}

impl fmt::Debug for RandomizedPass {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RandomizedPass")
            .field("metadata", &self.metadata)
            .field("config", &self.config)
            .field("has_generator", &true)
            .field("has_evaluator", &true)
            .field("has_entropy_source", &self.entropy.is_some())
            .finish()
    }
}

impl RandomizedPass {
    /// Creates a randomized pass with production defaults.
    pub fn new(
        generator: Arc<dyn RandomizedCandidateGenerator>,
        evaluator: Arc<dyn RandomizedCandidateEvaluator>,
    ) -> Result<Self, RandomizedConfigError> {
        Self::with_config(
            RandomizedConfig::default(),
            generator,
            evaluator,
        )
    }

    /// Creates a randomized pass with explicit local configuration.
    pub fn with_config(
        config: RandomizedConfig,
        generator: Arc<dyn RandomizedCandidateGenerator>,
        evaluator: Arc<dyn RandomizedCandidateEvaluator>,
    ) -> Result<Self, RandomizedConfigError> {
        let config = config.validate()?;

        let identifier = PassIdentifier::new(PASS_ID)
            .expect(
                "stochastic.randomized has a valid static identifier",
            );

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::Stochastic,
        )
        .expect(
            "randomized optimizer metadata must be valid",
        )
        .with_scope(PassScope::Circuit)
        .with_complexity(PassComplexity::Search)
        .with_determinism(PassDeterminism::Seeded)
        .with_capability(PassCapability::UsesRandomness)
        .with_capability(PassCapability::ReplacesOperations)
        .with_capability(PassCapability::ChangesGateCount)
        .with_capability(PassCapability::ChangesDepth)
        .with_semantic_preservation(true)
        .fixed_point_safe(false);

        Ok(Self {
            metadata,
            config,
            generator,
            evaluator,
            entropy: None,
        })
    }

    /// Supplies an explicit entropy source for nondeterministic compilation.
    ///
    /// Deterministic and seeded modes never invoke this source.
    #[must_use]
    pub fn with_entropy_source(
        mut self,
        entropy: Arc<dyn RandomEntropySource>,
    ) -> Self {
        self.entropy = Some(entropy);
        self
    }

    /// Returns the immutable local configuration.
    #[must_use]
    pub const fn config(&self) -> &RandomizedConfig {
        &self.config
    }

    /// Returns the stable pass identifier.
    #[must_use]
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Derives the root seed for one pass invocation.
    fn root_seed(
        &self,
        context: &mut OptimizationContext,
    ) -> Result<u64, OptimizationError> {
        if context.is_deterministic() {
            context
                .next_seed()
                .map_err(|error| internal(error.to_string()))
        } else if let Some(source) = &self.entropy {
            source
                .next_u64()
                .map_err(|error| {
                    internal(format!(
                        "random entropy source failed: {error}"
                    ))
                })
        } else {
            Err(OptimizationError::invalid_configuration(
                "optimization.determinism",
                "stochastic.randomized requires an explicit RandomEntropySource in nondeterministic mode",
            ))
        }
    }

    /// Derives a stable candidate seed from the invocation root.
    #[must_use]
    fn derive_seed(
        root: u64,
        iteration: u64,
        candidate: u64,
    ) -> u64 {
        splitmix64(
            root
                ^ splitmix64(iteration)
                ^ splitmix64(
                    candidate.wrapping_add(1),
                ),
        )
    }

    /// Returns whether another candidate can be charged.
    #[must_use]
    fn candidate_budget_available(
        &self,
        context: &OptimizationContext,
    ) -> bool {
        let used = context.count(
            OptimizationResource::RewriteCandidates,
        );

        used < context
            .limits()
            .max_rewrite_candidates()
    }

    /// Returns whether another stochastic iteration can be charged.
    #[must_use]
    fn iteration_budget_available(
        &self,
        context: &OptimizationContext,
    ) -> bool {
        let used = context.count(
            OptimizationResource::Iterations,
        );

        used < context.limits().max_iterations()
    }

    /// Returns whether another accepted stochastic transformation can be
    /// charged as a rewrite.
    #[must_use]
    fn rewrite_budget_available(
        &self,
        context: &OptimizationContext,
    ) -> bool {
        let used = context.count(
            OptimizationResource::Rewrites,
        );

        used < context.limits().max_rewrites()
    }

    /// Validates the immutable circuit envelope a candidate is required to
    /// preserve.
    fn validate_candidate_envelope(
        original: &QuantumCircuit,
        candidate: &QuantumCircuit,
    ) -> Result<(), OptimizationError> {
        if candidate.id() != original.id() {
            return Err(OptimizationError::invalid_input(
                OptimizationStage::Rewrite,
                "randomized candidate changed circuit identity",
            ));
        }

        if candidate.version() != original.version() {
            return Err(OptimizationError::invalid_input(
                OptimizationStage::Rewrite,
                "randomized candidate changed Quantum IR version",
            ));
        }

        if candidate.num_qubits()
            != original.num_qubits()
        {
            return Err(OptimizationError::invalid_input(
                OptimizationStage::Rewrite,
                "randomized candidate changed logical qubit namespace",
            ));
        }

        if candidate.num_classical_bits()
            != original.num_classical_bits()
        {
            return Err(OptimizationError::invalid_input(
                OptimizationStage::Rewrite,
                "randomized candidate changed classical-bit namespace",
            ));
        }

        if candidate.limits() != original.limits() {
            return Err(OptimizationError::invalid_input(
                OptimizationStage::Rewrite,
                "randomized candidate changed the canonical IR resource policy",
            ));
        }

        if candidate.metadata()
            != original.metadata()
        {
            return Err(OptimizationError::invalid_input(
                OptimizationStage::Rewrite,
                "randomized candidate changed canonical circuit metadata",
            ));
        }

        Ok(())
    }
}

impl OptimizationPass for RandomizedPass {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> PassExecutionResult {
        // ---------------------------------------------------------------------
        // Input safety
        // ---------------------------------------------------------------------

        context
            .check_cancelled()
            .map_err(|error| {
                internal(error.to_string())
            })?;

        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "randomized optimization received invalid Quantum IR: {error}"
                ),
            )
        })?;

        // ---------------------------------------------------------------------
        // Determinism policy
        // ---------------------------------------------------------------------

        if !context.is_deterministic()
            && self.entropy.is_none()
        {
            return Err(
                OptimizationError::invalid_configuration(
                    "optimization.determinism",
                    "stochastic.randomized requires an explicit RandomEntropySource in nondeterministic mode",
                ),
            );
        }

        // ---------------------------------------------------------------------
        // Initial state
        // ---------------------------------------------------------------------

        let operations_before = usize_to_u64(
            circuit.len(),
            "operation count before randomized optimization",
        )?;

        let original = circuit.clone();

        let mut incumbent = original.clone();
        let mut best = original.clone();

        let mut best_score = self
            .evaluator
            .evaluate(&best)
            .map_err(|error| {
                OptimizationError::analysis_failed(
                    None,
                    format!(
                        "randomized objective evaluation failed: {error}"
                    ),
                )
            })?;

        ensure_finite(
            best_score,
            "initial objective score",
        )?;

        let mut incumbent_score = best_score;

        let root_seed = self.root_seed(context)?;

        let mut statistics =
            RandomizedStatistics::default();

        let mut temperature =
            self.config.initial_temperature;

        // ---------------------------------------------------------------------
        // Stochastic search
        // ---------------------------------------------------------------------

        'iterations: for iteration
            in 0..self.config.max_iterations
        {
            context
                .check_cancelled()
                .map_err(|error| {
                    internal(error.to_string())
                })?;

            if !self
                .iteration_budget_available(context)
            {
                break 'iterations;
            }

            context
                .record_iteration()
                .map_err(|error| {
                    internal(error.to_string())
                })?;

            statistics.iterations =
                statistics
                    .iterations
                    .checked_add(1)
                    .ok_or_else(|| {
                        internal(
                            "randomized iteration counter overflow",
                        )
                    })?;

            for candidate_index in
                0..self.config.candidates_per_iteration
            {
                context
                    .check_cancelled()
                    .map_err(|error| {
                        internal(error.to_string())
                    })?;

                if !self
                    .candidate_budget_available(context)
                {
                    break 'iterations;
                }

                context
                    .record_rewrite_candidate()
                    .map_err(|error| {
                        internal(error.to_string())
                    })?;

                statistics.candidates_generated =
                    statistics
                        .candidates_generated
                        .checked_add(1)
                        .ok_or_else(|| {
                            internal(
                                "candidate counter overflow",
                            )
                        })?;

                let seed = Self::derive_seed(
                    root_seed,
                    iteration,
                    candidate_index,
                );

                let base = match self.config.strategy
                {
                    SearchStrategy::BestOfCandidates => {
                        &best
                    }

                    SearchStrategy::RandomWalk
                    | SearchStrategy::SimulatedAnnealing => {
                        &incumbent
                    }
                };

                let candidate =
                    match self.generator.generate(
                        base,
                        seed,
                        iteration,
                        candidate_index,
                    ) {
                        Ok(Some(value)) => value,

                        Ok(None) => {
                            statistics
                                .candidates_rejected =
                                statistics
                                    .candidates_rejected
                                    .checked_add(1)
                                    .ok_or_else(|| {
                                        internal(
                                            "candidate rejection counter overflow",
                                        )
                                    })?;

                            continue;
                        }

                        Err(error) => {
                            return Err(internal(
                                format!(
                                    "randomized candidate generation failed at iteration {iteration}, candidate {candidate_index}: {error}"
                                ),
                            ));
                        }
                    };

                // -------------------------------------------------------------
                // Candidate validation
                // -------------------------------------------------------------

                if candidate.validate().is_err() {
                    statistics.invalid_candidates =
                        statistics
                            .invalid_candidates
                            .checked_add(1)
                            .ok_or_else(|| {
                                internal(
                                    "invalid-candidate counter overflow",
                                )
                            })?;

                    continue;
                }

                Self::validate_candidate_envelope(
                    &original,
                    &candidate,
                )?;

                statistics.candidates_valid =
                    statistics
                        .candidates_valid
                        .checked_add(1)
                        .ok_or_else(|| {
                            internal(
                                "valid-candidate counter overflow",
                            )
                        })?;

                // -------------------------------------------------------------
                // Candidate objective evaluation
                // -------------------------------------------------------------

                let score =
                    match self.evaluator.evaluate(
                        &candidate,
                    ) {
                        Ok(value) => value,

                        Err(_) => {
                            statistics.evaluation_failures =
                                statistics
                                    .evaluation_failures
                                    .checked_add(1)
                                    .ok_or_else(
                                        || {
                                            internal(
                                                "evaluation-failure counter overflow",
                                            )
                                        },
                                    )?;

                            continue;
                        }
                    };

                ensure_finite(
                    score,
                    "candidate objective score",
                )?;

                // -------------------------------------------------------------
                // Acceptance policy
                // -------------------------------------------------------------

                let accepted =
                    match self.config.strategy
                    {
                        SearchStrategy::BestOfCandidates => {
                            score < best_score
                        }

                        SearchStrategy::RandomWalk => {
                            score < incumbent_score
                        }

                        SearchStrategy::SimulatedAnnealing => {
                            if score < incumbent_score {
                                true
                            } else {
                                let delta =
                                    score - incumbent_score;

                                let effective_temperature =
                                    temperature.max(
                                        self.config
                                            .minimum_temperature,
                                    );

                                let probability =
                                    (-delta
                                        / effective_temperature)
                                        .exp();

                                let random =
                                    unit_interval(
                                        Self::derive_seed(
                                            seed,
                                            iteration,
                                            candidate_index,
                                        ),
                                    );

                                random < probability
                            }
                        }
                    };

                if !accepted {
                    statistics
                        .candidates_rejected =
                        statistics
                            .candidates_rejected
                            .checked_add(1)
                            .ok_or_else(|| {
                                internal(
                                    "candidate rejection counter overflow",
                                )
                            })?;

                    continue;
                }

                // -------------------------------------------------------------
                // Accepted transformation budget
                // -------------------------------------------------------------

                if !self
                    .rewrite_budget_available(context)
                {
                    break 'iterations;
                }

                context
                    .record_rewrite()
                    .map_err(|error| {
                        internal(error.to_string())
                    })?;

                statistics.candidates_accepted =
                    statistics
                        .candidates_accepted
                        .checked_add(1)
                        .ok_or_else(|| {
                            internal(
                                "candidate acceptance counter overflow",
                            )
                        })?;

                incumbent = candidate.clone();
                incumbent_score = score;

                if score < best_score {
                    best = candidate;
                    best_score = score;

                    statistics.best_score_updates =
                        statistics
                            .best_score_updates
                            .checked_add(1)
                            .ok_or_else(|| {
                                internal(
                                    "best-score update counter overflow",
                                )
                            })?;
                }
            }

            // -------------------------------------------------------------
            // Temperature schedule
            // -------------------------------------------------------------

            if matches!(
                self.config.strategy,
                SearchStrategy::SimulatedAnnealing
            ) {
                temperature = (
                    temperature
                        * self.config.cooling_rate
                )
                .max(
                    self.config.minimum_temperature,
                );
            }

            // Keep the annealing incumbent aligned with the global best when
            // the global best becomes strictly better.
            if statistics.best_score_updates > 0
                && best_score < incumbent_score
            {
                incumbent = best.clone();
                incumbent_score = best_score;
            }
        }

        // ---------------------------------------------------------------------
        // Final improvement check
        // ---------------------------------------------------------------------

        let original_score =
            self.evaluator.evaluate(circuit)
                .map_err(|error| {
                    OptimizationError::analysis_failed(
                        None,
                        format!(
                            "final objective evaluation failed: {error}"
                        ),
                    )
                })?;

        ensure_finite(
            original_score,
            "final original objective score",
        )?;

        if best_score >= original_score {
            return Ok(
                PassOutcome::no_improvement(
                    operations_before,
                    operations_before,
                )
                .with_iterations(
                    statistics.iterations,
                )
                .with_rewrites(
                    statistics.candidates_accepted,
                )
                .with_message(format!(
                    "randomized search generated {} valid candidates without an improving result",
                    statistics.candidates_valid
                )),
            );
        }

        // ---------------------------------------------------------------------
        // Atomic final commit
        // ---------------------------------------------------------------------

        context
            .check_cancelled()
            .map_err(|error| {
                internal(error.to_string())
            })?;

        *circuit = best;

        circuit.validate().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::OutputValidation,
                format!(
                    "randomized optimization produced invalid Quantum IR: {error}"
                ),
            )
        })?;

        let operations_after = usize_to_u64(
            circuit.len(),
            "operation count after randomized optimization",
        )?;

        let operations_removed =
            operations_before
                .saturating_sub(operations_after);

        let operations_added =
            operations_after
                .saturating_sub(operations_before);

        Ok(
            PassOutcome::changed(
                operations_before,
                operations_after,
            )
            .with_operations_removed(
                operations_removed,
            )
            .with_operations_added(
                operations_added,
            )
            .with_rewrites(
                statistics.candidates_accepted,
            )
            .with_iterations(
                statistics.iterations,
            )
            .with_message(format!(
                "randomized search accepted {} candidates and improved the objective",
                statistics.candidates_accepted
            )),
        )
    }
}

impl Default for RandomizedPass {
    fn default() -> Self {
        panic!(
            "RandomizedPass requires a candidate generator and evaluator; \
             construct it with RandomizedPass::new or RandomizedPass::with_config"
        );
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Converts context/internal failures into the canonical optimizer error.
fn internal(
    message: impl Into<String>,
) -> OptimizationError {
    OptimizationError::internal(
        OptimizationStage::Rewrite,
        message.into(),
    )
}

/// Requires a finite objective value.
fn ensure_finite(
    value: f64,
    name: &'static str,
) -> Result<(), OptimizationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(
            OptimizationError::invalid_configuration(
                name,
                "objective scores must be finite",
            ),
        )
    }
}

/// Converts a platform `usize` count to the optimizer's stable `u64`
/// accounting representation.
fn usize_to_u64(
    value: usize,
    name: &'static str,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        internal(format!(
            "{name} does not fit into u64"
        ))
    })
}

/// Stable SplitMix64 mixer.
///
/// This is deterministic search randomness only. It is not a cryptographic
/// random-number generator.
fn splitmix64(
    mut value: u64,
) -> u64 {
    value = value.wrapping_add(
        0x9E37_79B9_7F4A_7C15,
    );

    let mut z = value;

    z = (z ^ (z >> 30))
        .wrapping_mul(
            0xBF58_476D_1CE4_E5B9,
        );

    z = (z ^ (z >> 27))
        .wrapping_mul(
            0x94D0_49BB_1331_11EB,
        );

    z ^ (z >> 31)
}

/// Converts a `u64` value into a `[0, 1)` floating-point sample.
///
/// The conversion is intentionally simple and deterministic. The result is
/// suitable for acceptance probabilities, not cryptographic use.
fn unit_interval(
    value: u64,
) -> f64 {
    const SCALE: f64 =
        1.0 / 18_446_744_073_709_551_616.0;

    let sample = (value as f64) * SCALE;

    // Floating-point rounding can turn the largest representable integer into
    // exactly 1.0. Clamp defensively so the mathematical contract remains
    // [0, 1).
    if sample >= 1.0 {
        0.999_999_999_999_999_9
    } else {
        sample
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_defaults_are_valid() {
        let config = RandomizedConfig::default();

        assert!(config.validate().is_ok());
        assert_eq!(config.max_iterations(), 1);
        assert_eq!(
            config.candidates_per_iteration(),
            8
        );
        assert_eq!(
            config.strategy(),
            SearchStrategy::BestOfCandidates
        );
    }

    #[test]
    fn invalid_configuration_is_rejected() {
        assert!(
            RandomizedConfig::default()
                .with_max_iterations(0)
                .validate()
                .is_err()
        );

        assert!(
            RandomizedConfig::default()
                .with_candidates_per_iteration(0)
                .validate()
                .is_err()
        );

        assert!(
            RandomizedConfig::default()
                .with_cooling_rate(1.0)
                .validate()
                .is_err()
        );

        assert!(
            RandomizedConfig::default()
                .with_initial_temperature(f64::NAN)
                .validate()
                .is_err()
        );

        assert!(
            RandomizedConfig::default()
                .with_minimum_temperature(2.0)
                .validate()
                .is_err()
        );
    }

    #[test]
    fn seed_derivation_is_stable() {
        let first =
            RandomizedPass::derive_seed(
                7,
                11,
                13,
            );

        let second =
            RandomizedPass::derive_seed(
                7,
                11,
                13,
            );

        let different =
            RandomizedPass::derive_seed(
                7,
                11,
                14,
            );

        assert_eq!(first, second);
        assert_ne!(first, different);
    }

    #[test]
    fn unit_interval_is_bounded() {
        let values = [
            0_u64,
            1_u64,
            u64::MAX / 2,
            u64::MAX,
        ];

        for value in values {
            let sample = unit_interval(value);

            assert!(sample >= 0.0);
            assert!(sample < 1.0);
        }
    }

    #[test]
    fn search_strategy_identifiers_are_stable() {
        assert_eq!(
            SearchStrategy::BestOfCandidates.as_str(),
            "best_of_candidates"
        );

        assert_eq!(
            SearchStrategy::RandomWalk.as_str(),
            "random_walk"
        );

        assert_eq!(
            SearchStrategy::SimulatedAnnealing.as_str(),
            "simulated_annealing"
        );
    }
}