//! Zamani Quantum Routing — SABRE / LightSABRE-Compatible Router
//!
//! Production-grade SABRE-style logical-to-physical quantum routing.
//!
//! # Responsibilities
//!
//! This module owns the SABRE routing algorithm itself:
//!
//! - front-layer construction;
//! - dependency-aware executable-gate discovery;
//! - restricted SWAP candidate generation;
//! - basic SABRE heuristic;
//! - lookahead heuristic;
//! - decay heuristic;
//! - deterministic seeded tie-breaking;
//! - multiple routing trials;
//! - bounded forward routing;
//! - bidirectional forward/backward layout refinement;
//! - mapping evolution through semantic SWAP movements;
//! - routing metrics;
//! - algorithm-level output validation;
//! - construction of the stable `RoutingResult` contract.
//!
//! It deliberately does NOT own:
//!
//! - initial-layout algorithms other than SABRE's iterative refinement;
//! - OpenQASM parsing;
//! - Zamani source parsing;
//! - canonical Quantum IR conversion;
//! - gate decomposition;
//! - SWAP-to-native-gate lowering;
//! - pulse generation;
//! - scheduling;
//! - hardware execution;
//! - QEC decoding;
//! - benchmarking orchestration.
//!
//! # SABRE model
//!
//! The implementation follows the production SABRE structure:
//!
//! ```text
//! initial mapping
//!       │
//!       ▼
//! forward route
//!       │
//!       ▼
//! final mapping
//!       │
//!       ▼
//! reverse workload
//!       │
//!       ▼
//! route backwards from final mapping
//!       │
//!       ▼
//! improved initial mapping
//!       │
//!       └───────────────► repeat
//! ```
//!
//! During routing:
//!
//! ```text
//! front layer
//!      │
//!      ├── executable ──► emit gate
//!      │
//!      └── blocked
//!             │
//!             ▼
//!     generate nearby SWAPs
//!             │
//!             ▼
//!       score candidates
//!             │
//!             ▼
//!       choose best SWAP
//!             │
//!             ▼
//!       update mapping
//!             │
//!             └──────────► repeat
//! ```
//!
//! # Heuristics
//!
//! Three SABRE-compatible heuristics are provided:
//!
//! - `Basic`: current front-layer distance;
//! - `Lookahead`: front-layer distance plus bounded future interactions;
//! - `Decay`: lookahead score multiplied by a recency penalty.
//!
//! `Decay` is the production default because it discourages repeatedly using
//! the same physical qubits and therefore reduces routing-depth pressure.
//!
//! # Determinism
//!
//! For a fixed:
//!
//! - workload;
//! - topology;
//! - initial mapping;
//! - configuration;
//! - seed;
//!
//! routing decisions are deterministic.
//!
//! Hash-map iteration order never determines a routing decision.
//!
//! Candidate edges are canonically ordered.
//!
//! Seeded pseudo-random tie-breaking is implemented locally so the routing
//! subsystem does not acquire a dependency on an external RNG crate.
//!
//! # Safety
//!
//! - No `unsafe`.
//! - No FFI.
//! - No global mutable state.
//! - No filesystem access.
//! - No network access.
//! - No hardware execution.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! # Integration contract
//!
//! This file consumes the current frozen routing contracts:
//!
//! ```text
//! types.rs
//! errors.rs
//! topology.rs
//! mapping.rs
//! config.rs
//! result.rs
//! ```
//!
//! In particular:
//!
//! - `RoutingWorkload` supplies logical interactions;
//! - `QubitInteraction` supplies logical operands and gate identity;
//! - `QubitMapping` owns bidirectional mapping state;
//! - `Topology` owns physical connectivity and gate support;
//! - `RoutingConfig` owns search limits/objectives/policies;
//! - `RoutingResult` owns the stable result representation;
//! - `RoutingError` owns the canonical error vocabulary.
//!
//! Later `router.rs` can call this module without modifying this file.
//!
//! # Algorithm references
//!
//! The architecture follows the original SABRE bidirectional heuristic search
//! and the subsequent LightSABRE improvements:
//!
//! - Gushu Li, Yufei Ding, Yuan Xie,
//!   "Tackling the Qubit Mapping Problem for NISQ-Era Quantum Devices".
//! - Henry Zou et al.,
//!   "LightSABRE: A Lightweight and Enhanced SABRE Algorithm".
//!
//! The implementation intentionally remains backend-independent.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use crate::quantum::routing::config::{
    LayoutStrategy,
    RoutingConfig,
    RoutingObjective,
    VerificationLevel,
};

use crate::quantum::routing::errors::RoutingError;

use crate::quantum::routing::mapping::{
    QubitMapping,
    MappingError,
};

use crate::quantum::routing::result::{
    ReproducibilityMetadata,
    RoutingMetrics,
    RoutingResult,
    VerificationSummary,
};

use crate::quantum::routing::topology::Topology;

use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalEdge,
    PhysicalQubitId,
    QubitInteraction,
    RouteDisposition,
    RoutingAlgorithm,
    RoutingMove,
    RoutingOperation,
    RoutingSeed,
    RoutingWorkload,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable SABRE implementation version.
///
/// This must be changed when the algorithm semantics change in a way that can
/// affect reproducibility.
pub const SABRE_ALGORITHM_VERSION: &str = "zamani.sabre.v1";

/// Default SABRE extended-set weight.
///
/// The front layer remains the dominant routing objective while future
/// interactions provide global guidance.
pub const DEFAULT_EXTENDED_SET_WEIGHT: f64 = 0.50;

/// Default decay increment.
///
/// Larger values penalize repeated use of recently active physical qubits more
/// aggressively.
pub const DEFAULT_DECAY_INCREMENT: f64 = 0.001;

/// Minimum allowed decay increment.
pub const MIN_DECAY_INCREMENT: f64 = 0.000_001;

/// Maximum allowed decay increment.
pub const MAX_DECAY_INCREMENT: f64 = 100.0;

/// Absolute upper bound for the internal candidate set.
///
/// The user-facing limit remains `RoutingConfig::limits.candidate_limit`.
pub const MAX_INTERNAL_CANDIDATES: usize = 1_000_000;

/// Absolute upper bound for SABRE search iterations.
pub const MAX_INTERNAL_ITERATIONS: usize = 100_000_000;

/// Maximum number of interactions examined while constructing the dependency
/// frontier.
pub const MAX_INTERNAL_INTERACTIONS: usize = 10_000_000;

/// Maximum supported routing arity.
///
/// SABRE is a two-qubit routing algorithm. 3+ qubit operations must cross the
/// decomposition boundary before reaching this module.
pub const MAX_ROUTING_ARITY: usize = 2;

// =============================================================================
// Heuristic
// =============================================================================

/// SABRE candidate-scoring heuristic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SabreHeuristic {
    /// Immediate front-layer distance only.
    Basic,

    /// Front-layer distance plus bounded future interaction distance.
    Lookahead,

    /// Lookahead score multiplied by a recency/decay penalty.
    Decay,
}

impl Default for SabreHeuristic {
    fn default() -> Self {
        Self::Decay
    }
}

impl SabreHeuristic {
    /// Stable machine-readable name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Lookahead => "lookahead",
            Self::Decay => "decay",
        }
    }
}

// =============================================================================
// Router
// =============================================================================

/// Production SABRE router.
///
/// The router itself is immutable. All mutable routing state exists inside an
/// individual routing invocation.
///
/// This makes it safe to:
///
/// - reuse a router instance;
/// - run independent trials;
/// - run multiple router instances concurrently;
/// - perform speculative routing;
/// - use the router from a future parallel routing executor.
#[derive(Debug, Clone)]
pub struct SabreRouter {
    heuristic: SabreHeuristic,
    extended_set_weight: f64,
    decay_increment: f64,
}

impl Default for SabreRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl SabreRouter {
    /// Creates the production-default SABRE router.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            heuristic: SabreHeuristic::Decay,
            extended_set_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            decay_increment: DEFAULT_DECAY_INCREMENT,
        }
    }

    /// Creates a router using the specified heuristic.
    #[must_use]
    pub const fn with_heuristic(
        heuristic: SabreHeuristic,
    ) -> Self {
        Self {
            heuristic,
            extended_set_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            decay_increment: DEFAULT_DECAY_INCREMENT,
        }
    }

    /// Creates a router with explicit heuristic parameters.
    ///
    /// Invalid floating-point parameters are rejected when routing starts.
    #[must_use]
    pub const fn with_parameters(
        heuristic: SabreHeuristic,
        extended_set_weight: f64,
        decay_increment: f64,
    ) -> Self {
        Self {
            heuristic,
            extended_set_weight,
            decay_increment,
        }
    }

    /// Returns the selected SABRE heuristic.
    #[must_use]
    pub const fn heuristic(&self) -> SabreHeuristic {
        self.heuristic
    }

    /// Returns the extended-set weight.
    #[must_use]
    pub const fn extended_set_weight(&self) -> f64 {
        self.extended_set_weight
    }

    /// Returns the decay increment.
    #[must_use]
    pub const fn decay_increment(&self) -> f64 {
        self.decay_increment
    }

    /// Stable algorithm name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "sabre"
    }

    /// Routes a workload using a caller-provided initial mapping.
    ///
    /// The caller-owned workload, topology, configuration and mapping are
    /// never mutated.
    ///
    /// The returned result contains the final mapping and semantic routing
    /// operation stream.
    pub fn route(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        initial_mapping: &QubitMapping,
        config: &RoutingConfig,
    ) -> Result<RoutingResult, RoutingError> {
        let started = Instant::now();

        self.validate_configuration(config)?;
        self.validate_input(
            workload,
            topology,
            initial_mapping,
            config,
        )?;

        let distance_matrix =
            DistanceMatrix::build(topology)?;

        let base_seed = effective_seed(config);

        let trial_count =
            config.limits.sabre_trials.max(1);

        let mut best: Option<CandidateRoute> = None;

        for trial_index in 0..trial_count {
            let trial_seed =
                derive_trial_seed(base_seed, trial_index as u64);

            let trial_initial_mapping =
                initial_mapping.clone();

            let candidate =
                self.bidirectional_trial(
                    workload,
                    topology,
                    &distance_matrix,
                    &trial_initial_mapping,
                    config,
                    trial_seed,
            )?;

            if best
                .as_ref()
                .map(|existing| {
                    route_order(&candidate, existing)
                        == Ordering::Less
                })
                .unwrap_or(true)
            {
                best = Some(candidate);
            }
        }

        let best = best.ok_or_else(|| {
            RoutingError::algorithm_incompatible(
                self.name(),
                "no SABRE routing trial produced a result",
            )
        })?;

        let elapsed = started.elapsed();

        self.finalize_result(
            workload,
            topology,
            initial_mapping,
            config,
            best,
            elapsed,
            trial_count,
        )
    }

    /// Convenience API for callers that want to select the heuristic directly.
    pub fn route_with_heuristic(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        initial_mapping: &QubitMapping,
        config: &RoutingConfig,
        heuristic: SabreHeuristic,
    ) -> Result<RoutingResult, RoutingError> {
        let router = Self {
            heuristic,
            extended_set_weight: self.extended_set_weight,
            decay_increment: self.decay_increment,
        };

        router.route(
            workload,
            topology,
            initial_mapping,
            config,
        )
    }

    // =========================================================================
    // Validation
    // =========================================================================

    fn validate_configuration(
        &self,
        config: &RoutingConfig,
    ) -> Result<(), RoutingError> {
        if !self.extended_set_weight.is_finite()
            || self.extended_set_weight < 0.0
        {
            return Err(
                RoutingError::invalid_configuration(
                    "sabre.extended_set_weight",
                    "value must be finite and non-negative",
                ),
            );
        }

        if !self.decay_increment.is_finite()
            || self.decay_increment < MIN_DECAY_INCREMENT
            || self.decay_increment > MAX_DECAY_INCREMENT
        {
            return Err(
                RoutingError::invalid_configuration(
                    "sabre.decay_increment",
                    format!(
                        "value must be finite and in [{MIN_DECAY_INCREMENT}, {MAX_DECAY_INCREMENT}]"
                    ),
                ),
            );
        }

        if config.limits.max_iterations == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "limits.max_iterations",
                    "must be greater than zero",
                ),
            );
        }

        if config.limits.max_iterations
            > MAX_INTERNAL_ITERATIONS
        {
            return Err(
                RoutingError::invalid_configuration(
                    "limits.max_iterations",
                    "exceeds SABRE internal safety limit",
                ),
            );
        }

        if config.limits.candidate_limit == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "limits.candidate_limit",
                    "must be greater than zero",
                ),
            );
        }

        if config.limits.candidate_limit
            > MAX_INTERNAL_CANDIDATES
        {
            return Err(
                RoutingError::invalid_configuration(
                    "limits.candidate_limit",
                    "exceeds SABRE internal safety limit",
                ),
            );
        }

        if config.limits.lookahead_depth == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "limits.lookahead_depth",
                    "must be greater than zero",
                ),
            );
        }

        if config.limits.sabre_iterations == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "limits.sabre_iterations",
                    "must be greater than zero",
                ),
            );
        }

        if config.limits.sabre_trials == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "limits.sabre_trials",
                    "must be greater than zero",
                ),
            );
        }

        if !config.weights.is_valid() {
            return Err(
                RoutingError::invalid_configuration(
                    "weights",
                    "all routing weights must be finite and non-negative",
                ),
            );
        }

        if config.objective
            == RoutingObjective::Weighted
            && config.weights.is_zero()
        {
            return Err(
                RoutingError::invalid_configuration(
                    "weights",
                    "weighted objective requires at least one non-zero weight",
                ),
            );
        }

        if !config.allow_swap {
            return Err(
                RoutingError::incompatible_configuration(
                    "algorithm=sabre",
                    "allow_swap=false",
                ),
            );
        }

        Ok(())
    }

    fn validate_input(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        mapping: &QubitMapping,
        config: &RoutingConfig,
    ) -> Result<(), RoutingError> {
        topology.validate()?;

        mapping
            .validate()
            .map_err(mapping_error)?;

        if topology.qubit_count() == 0 {
            return Err(
                RoutingError::empty_topology()
            );
        }

        if topology.qubit_count()
            > 1_000_000
        {
            return Err(
                RoutingError::invalid_configuration(
                    "topology.qubit_count",
                    "exceeds SABRE implementation safety ceiling",
                ),
            );
        }

        if workload.logical_qubit_count()
            > 1_000_000
        {
            return Err(
                RoutingError::invalid_configuration(
                    "workload.logical_qubit_count",
                    "exceeds SABRE implementation safety ceiling",
                ),
            );
        }

        if workload.interaction_count()
            > MAX_INTERNAL_INTERACTIONS
        {
            return Err(
                RoutingError::invalid_configuration(
                    "workload.interaction_count",
                    "exceeds SABRE implementation safety ceiling",
                ),
            );
        }

        if workload.interaction_count()
            > 10_000_000
        {
            return Err(
                RoutingError::invalid_configuration(
                    "workload.interaction_count",
                    "exceeds configured production safety boundary",
                ),
            );
        }

        if mapping.len()
            > topology.qubit_count()
        {
            return Err(
                RoutingError::insufficient_physical_qubits(
                    mapping.len(),
                    topology.qubit_count(),
                )
            );
        }

        for logical in workload.logical_qubits() {
            if !mapping.contains_logical(*logical) {
                return Err(
                    RoutingError::unknown_logical_qubit(
                        logical.to_string(),
                    )
                );
            }
        }

        for (index, interaction)
            in workload.interactions()
                .iter()
                .enumerate()
        {
            validate_interaction(
                interaction,
                index,
            )?;

            for logical in interaction.operands() {
                if !mapping.contains_logical(*logical) {
                    return Err(
                        RoutingError::unknown_logical_qubit(
                            logical.to_string(),
                        )
                        .with_diagnostic_context(
                            crate::quantum::routing::errors::RoutingErrorContext::new()
                                .with_operation_index(index)
                                .with_algorithm(self.name()),
                        ),
                    );
                }
            }
        }

        // Configuration must not silently permit a SABRE route over a
        // directionally impossible SWAP graph.
        let has_swap_edge = topology
            .edges()
            .any(|edge| {
                topology.is_bidirectionally_adjacent(
                    edge.a(),
                    edge.b(),
                )
            });

        if workload.interaction_count() > 0
            && !has_swap_edge
        {
            // A workload may still be executable without movement.
            let all_executable =
                workload
                    .interactions()
                    .iter()
                    .all(|interaction| {
                        if interaction.arity() != 2 {
                            return true;
                        }

                        let operands =
                            interaction.operands();

                        let a =
                            mapping.physical_of(
                                operands[0],
                            );

                        let b =
                            mapping.physical_of(
                                operands[1],
                            );

                        match (a, b) {
                            (Some(a), Some(b)) =>
                                topology.supports_gate(
                                    interaction
                                        .gate()
                                        .name(),
                                    a,
                                    b,
                                ),
                            _ => false,
                        }
                    });

            if !all_executable {
                return Err(
                    RoutingError::no_candidate()
                );
            }
        }

        let _ = config;

        Ok(())
    }

    // =========================================================================
    // Bidirectional SABRE
    // =========================================================================

    fn bidirectional_trial(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        distances: &DistanceMatrix,
        initial_mapping: &QubitMapping,
        config: &RoutingConfig,
        seed: u64,
    ) -> Result<CandidateRoute, RoutingError> {
        let mut working_initial =
            initial_mapping.clone();

        let mut best_forward: Option<CandidateRoute> =
            None;

        let refinement_iterations =
            config.limits.sabre_iterations.max(1);

        for refinement in 0..refinement_iterations {
            let forward_seed =
                mix_seed(
                    seed,
                    refinement as u64,
                    0xF0F0_F0F0_F0F0_F0F0,
                );

            let forward =
                self.route_direction(
                    workload,
                    topology,
                    distances,
                    &working_initial,
                    config,
                    forward_seed,
                )?;

            if best_forward
                .as_ref()
                .map(|existing| {
                    route_order(
                        &forward,
                        existing,
                    ) == Ordering::Less
                })
                .unwrap_or(true)
            {
                best_forward =
                    Some(forward.clone());
            }

            if refinement + 1
                >= refinement_iterations
            {
                break;
            }

            // SABRE's bidirectional layout refinement:
            //
            // forward final mapping
            //        ↓
            // reverse circuit
            //        ↓
            // reverse route
            //        ↓
            // reverse final mapping
            //        ↓
            // next forward initial mapping
            let reversed =
                reverse_workload(workload);

            let backward_seed =
                mix_seed(
                    seed,
                    refinement as u64,
                    0x0F0F_0F0F_0F0F_0F0F,
                );

            let backward =
                self.route_direction(
                    &reversed,
                    topology,
                    distances,
                    &forward.final_mapping,
                    config,
                    backward_seed,
                )?;

            working_initial =
                backward.final_mapping;
        }

        best_forward.ok_or_else(|| {
            RoutingError::algorithm_incompatible(
                self.name(),
                "bidirectional SABRE did not produce a forward route",
            )
        })
    }

    // =========================================================================
    // One routing direction
    // =========================================================================

    fn route_direction(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        distances: &DistanceMatrix,
        initial_mapping: &QubitMapping,
        config: &RoutingConfig,
        seed: u64,
    ) -> Result<CandidateRoute, RoutingError> {
        let started = Instant::now();

        let mut mapping =
            initial_mapping.clone();

        mapping
            .validate()
            .map_err(mapping_error)?;

        let mut completed =
            vec![false; workload.interaction_count()];

        let mut operations =
            Vec::<RoutingOperation>::new();

        let mut decay =
            DecayState::new(topology);

        let mut inserted_swaps = 0usize;
        let mut candidate_evaluations = 0usize;
        let mut candidate_rejections = 0usize;
        let mut routing_iterations = 0usize;
        let mut routed_two_qubit_operations = 0usize;

        let max_iterations =
            config.limits.max_iterations;

        let max_swaps =
            config.limits.max_swaps;

        while !all_completed(&completed) {
            routing_iterations =
                routing_iterations
                    .checked_add(1)
                    .ok_or_else(|| {
                        RoutingError::iteration_limit_exceeded(
                            max_iterations,
                        )
                    })?;

            if routing_iterations
                > max_iterations
            {
                return Err(
                    RoutingError::iteration_limit_exceeded(
                        max_iterations,
                    )
                );
            }

            if let Some(timeout) =
                config.limits.timeout
            {
                if started.elapsed()
                    > timeout
                {
                    return Err(
                        RoutingError::routing_timeout()
                    );
                }
            }

            let front =
                build_front_layer(
                    workload,
                    &completed,
                );

            if front.is_empty() {
                return Err(
                    RoutingError::algorithm_incompatible(
                        self.name(),
                        "unfinished workload has no dependency-ready front-layer operations",
                    )
                );
            }

            let mut progress = false;

            // -------------------------------------------------------------
            // Emit every currently executable front-layer operation.
            // -------------------------------------------------------------
            for &index in &front {
                if completed[index] {
                    continue;
                }

                let interaction =
                    &workload.interactions()[index];

                if is_executable(
                    interaction,
                    topology,
                    &mapping,
                )? {
                    operations.push(
                        make_gate_operation(
                            interaction,
                            &mapping,
                        )?,
                    );

                    completed[index] = true;
                    progress = true;
                    routed_two_qubit_operations =
                        routed_two_qubit_operations
                            .checked_add(1)
                            .ok_or_else(|| {
                                RoutingError::internal_invariant(
                                    "routed two-qubit operation count overflow",
                                )
                            })?;

                    continue;
                }
            }

            if progress {
                decay.relax();
                continue;
            }

            // -------------------------------------------------------------
            // No front-layer operation is executable.
            // Generate and score legal SWAP candidates.
            // -------------------------------------------------------------
            let extended =
                build_extended_set(
                    workload,
                    &completed,
                    &front,
                    config.limits.lookahead_depth,
                );

            let candidates =
                generate_candidates(
                    &front,
                    workload,
                    &mapping,
                    topology,
                    config.limits.candidate_limit,
                )?;

            if candidates.is_empty() {
                return Err(
                    RoutingError::no_candidate()
                );
            }

            let mut scored =
                Vec::with_capacity(candidates.len());

            for candidate in candidates {
                candidate_evaluations =
                    candidate_evaluations
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::internal_invariant(
                                "candidate evaluation count overflow",
                            )
                        })?;

                let score =
                    self.score_candidate(
                        &candidate,
                        &front,
                        &extended,
                        workload,
                        &mapping,
                        topology,
                        distances,
                        &decay,
                        config,
                        seed,
                        routing_iterations,
                    )?;

                scored.push(
                    ScoredCandidate {
                        a: candidate.a,
                        b: candidate.b,
                        score,
                        tie_break: candidate_tie_break(
                            seed,
                            candidate.a,
                            candidate.b,
                            routing_iterations,
                        ),
                    },
                );
            }

            scored.sort_by(
                scored_candidate_order,
            );

            let selected =
                scored
                    .first()
                    .ok_or_else(
                        RoutingError::no_candidate,
                    )?;

            if let Some(limit) = max_swaps {
                if inserted_swaps >= limit {
                    return Err(
                        RoutingError::swap_limit_exceeded(
                            limit,
                        )
                    );
                }
            }

            if !topology
                .is_bidirectionally_adjacent(
                    selected.a,
                    selected.b,
                )
            {
                candidate_rejections =
                    candidate_rejections
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::internal_invariant(
                                "candidate rejection count overflow",
                            )
                        })?;

                return Err(
                    RoutingError::non_adjacent_movement(
                        selected.a.index(),
                        selected.b.index(),
                    )
                );
            }

            mapping
                .swap_physical(
                    selected.a,
                    selected.b,
                )
                .map_err(mapping_error)?;

            operations.push(
                RoutingOperation::Move(
                    RoutingMove::Swap {
                        a: selected.a,
                        b: selected.b,
                    },
                ),
            );

            inserted_swaps =
                inserted_swaps
                    .checked_add(1)
                    .ok_or_else(|| {
                        RoutingError::internal_invariant(
                            "inserted SWAP count overflow",
                        )
                    })?;

            decay.record(
                selected.a,
                selected.b,
                self.decay_increment,
            );

            if config
                .validate_mapping_after_move
            {
                mapping
                    .validate()
                    .map_err(mapping_error)?;
            }
        }

        mapping
            .validate()
            .map_err(mapping_error)?;

        let routing_duration =
            started.elapsed();

        let route = CandidateRoute {
            initial_mapping: initial_mapping.clone(),
            final_mapping: mapping,
            operations,
            inserted_swaps,
            candidate_evaluations,
            candidate_rejections,
            routing_iterations,
            routed_two_qubit_operations,
            routing_duration,
            seed,
        };

        self.validate_route(
            workload,
            topology,
            &route,
        )?;

        Ok(route)
    }

    // =========================================================================
    // Candidate scoring
    // =========================================================================

    fn score_candidate(
        &self,
        candidate: &SwapCandidate,
        front: &[usize],
        extended: &[usize],
        workload: &RoutingWorkload,
        mapping: &QubitMapping,
        topology: &Topology,
        distances: &DistanceMatrix,
        decay: &DecayState,
        config: &RoutingConfig,
        seed: u64,
        iteration: usize,
    ) -> Result<f64, RoutingError> {
        let mut speculative =
            mapping.clone();

        speculative
            .swap_physical(
                candidate.a,
                candidate.b,
            )
            .map_err(mapping_error)?;

        let front_cost =
            average_interaction_distance(
                front,
                workload,
                &speculative,
                distances,
            )?;

        let extended_cost =
            average_interaction_distance(
                extended,
                workload,
                &speculative,
                distances,
            )?;

        let base =
            match self.heuristic {
                SabreHeuristic::Basic =>
                    front_cost,

                SabreHeuristic::Lookahead =>
                    front_cost
                        + self.extended_set_weight
                            * extended_cost,

                SabreHeuristic::Decay => {
                    let heuristic =
                        front_cost
                            + self.extended_set_weight
                                * extended_cost;

                    let decay_factor =
                        decay
                            .factor(
                                candidate.a,
                                candidate.b,
                            );

                    heuristic * decay_factor
                }
            };

        let hardware_penalty =
            hardware_candidate_penalty(
                candidate.a,
                candidate.b,
                topology,
                config,
            )?;

        let objective_scale =
            objective_scale(
                &config.objective,
                &config.weights,
            );

        let random_tie =
            if config.deterministic {
                0.0
            } else {
                // Randomness is deliberately kept extremely small. It only
                // affects exact/near-exact heuristic ties and never overrides
                // a materially better candidate.
                let value =
                    candidate_tie_break(
                        seed,
                        candidate.a,
                        candidate.b,
                        iteration,
                    );

                (value as f64)
                    / (u64::MAX as f64)
                    * 1.0e-12
            };

        let score =
            base
                * objective_scale
                + hardware_penalty
                + random_tie;

        if !score.is_finite() {
            return Err(
                RoutingError::internal_invariant(
                    "SABRE candidate score became non-finite",
                )
            );
        }

        Ok(score)
    }

    // =========================================================================
    // Route verification
    // =========================================================================

    fn validate_route(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        route: &CandidateRoute,
    ) -> Result<(), RoutingError> {
        route
            .final_mapping
            .validate()
            .map_err(mapping_error)?;

        let mut mapping =
            route.initial_mapping.clone();

        let mut consumed =
            Vec::<QubitInteraction>::new();

        for operation in
            &route.operations
        {
            match operation {
                RoutingOperation::Move(
                    RoutingMove::Swap { a, b },
                ) => {
                    if !topology
                        .is_bidirectionally_adjacent(
                            *a,
                            *b,
                        )
                    {
                        return Err(
                            RoutingError::verification_failed(
                                format!(
                                    "SABRE emitted non-adjacent SWAP {a} <-> {b}"
                                ),
                            )
                        );
                    }

                    mapping
                        .swap_physical(*a, *b)
                        .map_err(mapping_error)?;
                }

                RoutingOperation::Gate {
                    gate,
                    operands,
                    logical_operands,
                } => {
                    if operands.len()
                        != logical_operands.len()
                    {
                        return Err(
                            RoutingError::verification_failed(
                                "routed gate physical/logical operand lengths differ",
                            )
                        );
                    }

                    if operands.len()
                        != 2
                    {
                        return Err(
                            RoutingError::verification_failed(
                                "SABRE produced a non-two-qubit gate",
                            )
                        );
                    }

                    if !topology
                        .supports_gate(
                            gate.name(),
                            operands[0],
                            operands[1],
                        )
                    {
                        return Err(
                            RoutingError::illegal_routed_operation(
                                gate.name(),
                                operands
                                    .iter()
                                    .map(|q| q.index())
                                    .collect(),
                            )
                        );
                    }

                    let expected_a =
                        mapping
                            .physical_of(
                                logical_operands[0],
                            )
                            .ok_or_else(|| {
                                RoutingError::unknown_logical_qubit(
                                    logical_operands[0]
                                        .to_string(),
                                )
                            })?;

                    let expected_b =
                        mapping
                            .physical_of(
                                logical_operands[1],
                            )
                            .ok_or_else(|| {
                                RoutingError::unknown_logical_qubit(
                                    logical_operands[1]
                                        .to_string(),
                                )
                            })?;

                    if expected_a
                        != operands[0]
                        || expected_b
                            != operands[1]
                    {
                        return Err(
                            RoutingError::verification_failed(
                                format!(
                                    "SABRE gate mapping mismatch for `{}`",
                                    gate.name()
                                ),
                            )
                        );
                    }

                    consumed.push(
                        QubitInteraction::new(
                            logical_operands.clone(),
                            gate.clone(),
                        ),
                    );
                }

                RoutingOperation::Barrier {
                    ..
                } => {
                    // RoutingWorkload currently represents interaction
                    // semantics rather than barriers. Barriers therefore do
                    // not participate in SABRE validation.
                }

                RoutingOperation::Move(
                    RoutingMove::Bridge { .. },
                )
                | RoutingOperation::Move(
                    RoutingMove::Permutation { .. },
                ) => {
                    return Err(
                        RoutingError::verification_failed(
                            "SABRE emitted a non-SWAP movement",
                        )
                    );
                }
            }
        }

        if consumed.len()
            != workload.interaction_count()
        {
            return Err(
                RoutingError::verification_failed(
                    format!(
                        "SABRE consumed {} interactions but workload contains {}",
                        consumed.len(),
                        workload.interaction_count()
                    ),
                )
            );
        }

        for (expected, actual)
            in workload
                .interactions()
                .iter()
                .zip(consumed.iter())
        {
            if expected != actual {
                return Err(
                    RoutingError::verification_failed(
                        "SABRE changed logical interaction ordering or gate identity",
                    )
                );
            }
        }

        if mapping != route.final_mapping {
            return Err(
                RoutingError::verification_failed(
                    "SABRE final mapping does not agree with emitted SWAP operations",
                )
            );
        }

        Ok(())
    }

    // =========================================================================
    // Result construction
    // =========================================================================

    fn finalize_result(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        original_mapping: &QubitMapping,
        config: &RoutingConfig,
        route: CandidateRoute,
        total_duration: Duration,
        trial_count: usize,
    ) -> Result<RoutingResult, RoutingError> {
        let metrics =
            build_metrics(
                workload,
                topology,
                &route,
                total_duration,
                trial_count,
            )?;

        let verification =
            if config.verify_output {
                VerificationSummary::passed(
                    config.verification,
                )
                .with_verifier_version(
                    "sabre.algorithm-level-v1",
                )
                .with_structural_checks(
                    route.operations.len(),
                )
                .with_mapping_checks(
                    route.inserted_swaps
                        .saturating_mul(2)
                        .saturating_add(2),
                )
                .with_executability_checks(
                    workload.interaction_count(),
                )
                .with_preservation_checks(
                    workload.interaction_count(),
                )
                .with_passed_checks(
                    route.operations.len()
                        + route.inserted_swaps
                            .saturating_mul(2)
                        + 2
                        + workload
                            .interaction_count()
                            .saturating_mul(2),
                )
            } else {
                VerificationSummary::not_requested()
            };

        let reproducibility =
            build_reproducibility(
                workload,
                topology,
                config,
                &route,
                trial_count,
            );

        let disposition =
            if route.inserted_swaps == 0 {
                RouteDisposition::AlreadyExecutable
            } else {
                RouteDisposition::Routed
            };

        // Preserve the original mapping supplied by the caller in the result.
        //
        // `route.initial_mapping` is expected to equal it, but this explicit
        // check prevents accidental future changes to the bidirectional
        // refinement logic from silently changing result semantics.
        if route.initial_mapping
            != *original_mapping
        {
            return Err(
                RoutingError::internal_invariant(
                    "SABRE route initial mapping differs from caller mapping",
                )
            );
        }

        let result =
            RoutingResult::new(
                disposition,
                RoutingAlgorithm::Sabre,
                LayoutStrategy::Sabre,
                config.objective.clone(),
                config.mode,
                route
                    .initial_mapping
                    .snapshot(),
                route
                    .final_mapping
                    .snapshot(),
                route.operations,
                metrics,
                verification,
                reproducibility,
            );

        if !result.is_internally_consistent() {
            return Err(
                RoutingError::internal_invariant(
                    "constructed SABRE RoutingResult is internally inconsistent",
                )
            );
        }

        Ok(result)
    }
}

// =============================================================================
// Candidate route
// =============================================================================

#[derive(Debug, Clone)]
struct CandidateRoute {
    initial_mapping: QubitMapping,
    final_mapping: QubitMapping,
    operations: Vec<RoutingOperation>,
    inserted_swaps: usize,
    candidate_evaluations: usize,
    candidate_rejections: usize,
    routing_iterations: usize,
    routed_two_qubit_operations: usize,
    routing_duration: Duration,
    seed: u64,
}

fn route_order(
    a: &CandidateRoute,
    b: &CandidateRoute,
) -> Ordering {
    a.inserted_swaps
        .cmp(&b.inserted_swaps)
        .then_with(|| {
            approximate_depth(
                &a.operations,
            )
            .cmp(
                &approximate_depth(
                    &b.operations,
                ),
            )
        })
        .then_with(|| {
            a.operations
                .len()
                .cmp(&b.operations.len())
        })
        .then_with(|| {
            operation_stream_order(
                &a.operations,
                &b.operations,
            )
        })
        .then_with(|| {
            a.seed.cmp(&b.seed)
        })
}

// =============================================================================
// Swap candidate
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SwapCandidate {
    a: PhysicalQubitId,
    b: PhysicalQubitId,
}

// =============================================================================
// Scored candidate
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScoredCandidate {
    a: PhysicalQubitId,
    b: PhysicalQubitId,
    score: f64,
    tie_break: u64,
}

fn scored_candidate_order(
    a: &ScoredCandidate,
    b: &ScoredCandidate,
) -> Ordering {
    a.score
        .partial_cmp(&b.score)
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            a.tie_break
                .cmp(&b.tie_break)
        })
        .then_with(|| {
            a.a.cmp(&b.a)
        })
        .then_with(|| {
            a.b.cmp(&b.b)
        })
}

// =============================================================================
// Dependency front layer
// =============================================================================

/// Builds a dependency-aware front layer.
///
/// An interaction becomes ready only after every earlier interaction sharing a
/// logical qubit has completed.
///
/// This is conservative and deterministic. It does not require a full compiler
/// DAG because `RoutingWorkload` deliberately exposes program-order
/// interactions.
fn build_front_layer(
    workload: &RoutingWorkload,
    completed: &[bool],
) -> Vec<usize> {
    let interactions =
        workload.interactions();

    let mut front =
        Vec::new();

    for index in 0..interactions.len() {
        if completed[index] {
            continue;
        }

        let current =
            &interactions[index];

        let mut blocked =
            false;

        for previous_index in 0..index {
            if completed[previous_index] {
                continue;
            }

            let previous =
                &interactions[previous_index];

            if interactions_conflict(
                previous,
                current,
            ) {
                blocked = true;
                break;
            }
        }

        if !blocked {
            front.push(index);
        }
    }

    front
}

/// Builds a bounded extended set after the front layer.
///
/// Only dependency-ready successors are considered, and the number of future
/// interactions is explicitly bounded.
fn build_extended_set(
    workload: &RoutingWorkload,
    completed: &[bool],
    front: &[usize],
    depth: usize,
) -> Vec<usize> {
    if depth == 0 {
        return Vec::new();
    }

    let interactions =
        workload.interactions();

    let front_set =
        front
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();

    let mut extended =
        Vec::new();

    for index in 0..interactions.len() {
        if completed[index]
            || front_set.contains(&index)
        {
            continue;
        }

        if extended.len() >= depth {
            break;
        }

        let interaction =
            &interactions[index];

        let mut depends_on_front =
            false;

        for &front_index in front {
            if interactions_conflict(
                &interactions[front_index],
                interaction,
            ) {
                depends_on_front = true;
                break;
            }
        }

        if depends_on_front {
            extended.push(index);
        }
    }

    extended
}

fn interactions_conflict(
    a: &QubitInteraction,
    b: &QubitInteraction,
) -> bool {
    for logical_a in a.operands() {
        for logical_b in b.operands() {
            if logical_a == logical_b {
                return true;
            }
        }
    }

    false
}

fn all_completed(
    completed: &[bool],
) -> bool {
    completed.iter().all(|value| *value)
}

// =============================================================================
// Candidate generation
// =============================================================================

/// Generates SWAP candidates only around physical qubits participating in the
/// current blocked front layer.
///
/// This is one of the central SABRE scalability properties: arbitrary physical
/// edges unrelated to the current front layer are not evaluated.
fn generate_candidates(
    front: &[usize],
    workload: &RoutingWorkload,
    mapping: &QubitMapping,
    topology: &Topology,
    candidate_limit: usize,
) -> Result<Vec<SwapCandidate>, RoutingError> {
    if candidate_limit == 0 {
        return Err(
            RoutingError::candidate_limit_exceeded(
                0,
            )
        );
    }

    let mut active_physical =
        BTreeSet::new();

    for &index in front {
        let interaction =
            workload
                .interactions()
                .get(index)
                .ok_or_else(|| {
                    RoutingError::internal_invariant(
                        "front-layer index is outside workload",
                    )
                })?;

        for logical in interaction.operands() {
            let physical =
                mapping
                    .physical_of(*logical)
                    .ok_or_else(|| {
                        RoutingError::unknown_logical_qubit(
                            logical.to_string(),
                        )
                    })?;

            active_physical.insert(
                physical,
            );
        }
    }

    let mut candidates =
        BTreeSet::<(PhysicalQubitId, PhysicalQubitId)>::new();

    for edge in topology.edges() {
        let a = edge.a();
        let b = edge.b();

        // A semantic SWAP needs a bidirectionally usable physical connection.
        if !topology
            .is_bidirectionally_adjacent(
                a,
                b,
            )
        {
            continue;
        }

        if !active_physical.contains(&a)
            && !active_physical.contains(&b)
        {
            continue;
        }

        let pair =
            if a < b {
                (a, b)
            } else {
                (b, a)
            };

        candidates.insert(pair);

        if candidates.len()
            >= candidate_limit
        {
            break;
        }
    }

    Ok(candidates
        .into_iter()
        .map(|(a, b)| {
            SwapCandidate { a, b }
        })
        .collect())
}

// =============================================================================
// Executability
// =============================================================================

fn is_executable(
    interaction: &QubitInteraction,
    topology: &Topology,
    mapping: &QubitMapping,
) -> Result<bool, RoutingError> {
    match interaction.arity() {
        0 | 1 => Ok(true),

        2 => {
            let operands =
                interaction.operands();

            let a =
                mapping
                    .physical_of(operands[0])
                    .ok_or_else(|| {
                        RoutingError::unknown_logical_qubit(
                            operands[0].to_string(),
                        )
                    })?;

            let b =
                mapping
                    .physical_of(operands[1])
                    .ok_or_else(|| {
                        RoutingError::unknown_logical_qubit(
                            operands[1].to_string(),
                        )
                    })?;

            Ok(
                topology.supports_gate(
                    interaction.gate().name(),
                    a,
                    b,
                )
            )
        }

        arity => Err(
            RoutingError::unsupported_arity(
                interaction.gate().name(),
                arity,
            )
        ),
    }
}

fn make_gate_operation(
    interaction: &QubitInteraction,
    mapping: &QubitMapping,
) -> Result<RoutingOperation, RoutingError> {
    if interaction.arity() != 2 {
        return Err(
            RoutingError::unsupported_arity(
                interaction.gate().name(),
                interaction.arity(),
            )
        );
    }

    let logical_operands =
        interaction.operands();

    let physical_a =
        mapping
            .physical_of(
                logical_operands[0],
            )
            .ok_or_else(|| {
                RoutingError::unknown_logical_qubit(
                    logical_operands[0]
                        .to_string(),
                )
            })?;

    let physical_b =
        mapping
            .physical_of(
                logical_operands[1],
            )
            .ok_or_else(|| {
                RoutingError::unknown_logical_qubit(
                    logical_operands[1]
                        .to_string(),
                )
            })?;

    Ok(
        RoutingOperation::Gate {
            gate: interaction
                .gate()
                .clone(),
            operands: vec![
                physical_a,
                physical_b,
            ],
            logical_operands:
                logical_operands
                    .to_vec(),
        }
    )
}

// =============================================================================
// Distance matrix
// =============================================================================

/// Cached physical shortest-path distances.
///
/// The matrix uses only bidirectional physical edges because those are the
/// connections on which the current semantic SWAP movement can safely operate.
#[derive(Debug, Clone)]
struct DistanceMatrix {
    distances:
        BTreeMap<
            (PhysicalQubitId, PhysicalQubitId),
            usize,
        >,
}

impl DistanceMatrix {
    fn build(
        topology: &Topology,
    ) -> Result<Self, RoutingError> {
        let qubits =
            topology
                .qubits()
                .collect::<Vec<_>>();

        let mut distances =
            BTreeMap::new();

        for source in
            &qubits
        {
            let mut queue =
                std::collections::VecDeque::new();

            let mut local =
                BTreeMap::<
                    PhysicalQubitId,
                    usize,
                >::new();

            local.insert(
                *source,
                0,
            );

            queue.push_back(
                *source,
            );

            while let Some(current) =
                queue.pop_front()
            {
                let current_distance =
                    local
                        .get(&current)
                        .copied()
                        .unwrap_or(0);

                for edge in
                    topology.edges()
                {
                    let neighbour =
                        if edge.a()
                            == current
                        {
                            Some(edge.b())
                        } else if edge.b()
                            == current
                        {
                            Some(edge.a())
                        } else {
                            None
                        };

                    let Some(neighbour) =
                        neighbour
                    else {
                        continue;
                    };

                    if !topology
                        .is_bidirectionally_adjacent(
                            current,
                            neighbour,
                        )
                    {
                        continue;
                    }

                    if local
                        .contains_key(
                            &neighbour,
                        )
                    {
                        continue;
                    }

                    let next_distance =
                        current_distance
                            .checked_add(1)
                            .ok_or_else(
                                || {
                                    RoutingError::internal_invariant(
                                        "physical distance overflow",
                                    )
                                },
                            )?;

                    local.insert(
                        neighbour,
                        next_distance,
                    );

                    queue.push_back(
                        neighbour,
                    );
                }
            }

            for target in
                &qubits
            {
                if let Some(distance) =
                    local.get(target)
                {
                    distances.insert(
                        (*source, *target),
                        *distance,
                    );
                }
            }
        }

        Ok(Self {
            distances,
        })
    }

    fn distance(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Option<usize> {
        self.distances
            .get(&(a, b))
            .copied()
    }
}

// =============================================================================
// Interaction heuristic
// =============================================================================

fn average_interaction_distance(
    indices: &[usize],
    workload: &RoutingWorkload,
    mapping: &QubitMapping,
    distances: &DistanceMatrix,
) -> Result<f64, RoutingError> {
    if indices.is_empty() {
        return Ok(0.0);
    }

    let mut total =
        0.0_f64;

    let mut count =
        0usize;

    for &index in indices {
        let interaction =
            workload
                .interactions()
                .get(index)
                .ok_or_else(|| {
                    RoutingError::internal_invariant(
                        "interaction index outside workload",
                    )
                })?;

        if interaction.arity()
            != 2
        {
            continue;
        }

        let operands =
            interaction.operands();

        let a =
            mapping
                .physical_of(
                    operands[0],
                )
                .ok_or_else(|| {
                    RoutingError::unknown_logical_qubit(
                        operands[0]
                            .to_string(),
                    )
                })?;

        let b =
            mapping
                .physical_of(
                    operands[1],
                )
                .ok_or_else(|| {
                    RoutingError::unknown_logical_qubit(
                        operands[1]
                            .to_string(),
                    )
                })?;

        let distance =
            distances
                .distance(a, b)
                .ok_or_else(|| {
                    RoutingError::no_routing_path(
                        a.index(),
                        b.index(),
                    )
                })?;

        total +=
            distance as f64;

        count =
            count
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::internal_invariant(
                        "interaction-distance count overflow",
                    )
                })?;
    }

    if count == 0 {
        return Ok(0.0);
    }

    let average =
        total / count as f64;

    if !average.is_finite() {
        return Err(
            RoutingError::internal_invariant(
                "interaction-distance heuristic became non-finite",
            )
        );
    }

    Ok(average)
}

// =============================================================================
// Hardware-aware candidate penalty
// =============================================================================

/// Adds a bounded hardware-quality penalty to SABRE's topology heuristic.
///
/// This does not replace `noise_aware.rs`. It only prevents the SABRE router
/// from ignoring obvious physical-edge quality differences when the configured
/// objective explicitly requests hardware awareness.
fn hardware_candidate_penalty(
    a: PhysicalQubitId,
    b: PhysicalQubitId,
    topology: &Topology,
    config: &RoutingConfig,
) -> Result<f64, RoutingError> {
    let properties =
        topology
            .edge_properties(a, b);

    let Some(properties) =
        properties
    else {
        return Ok(0.0);
    };

    if !properties.available {
        return Err(
            RoutingError::unsupported_movement(
                format!(
                    "physical edge {a} <-> {b} is unavailable",
                ),
            )
        );
    }

    let mut penalty =
        0.0_f64;

    match config.objective {
        RoutingObjective::Duration => {
            if let Some(duration) =
                properties.duration
            {
                penalty +=
                    duration.as_secs_f64();
            }
        }

        RoutingObjective::Error => {
            if let Some(error) =
                properties.error_rate
            {
                if !error.is_finite()
                    || !(0.0..=1.0)
                        .contains(&error)
                {
                    return Err(
                        RoutingError::invalid_configuration(
                            "topology.edge.error_rate",
                            "must be finite and within [0,1]",
                        ),
                    );
                }

                penalty += error;
            }
        }

        RoutingObjective::Fidelity => {
            if let Some(fidelity) =
                properties.fidelity
            {
                if !fidelity.is_finite()
                    || !(0.0..=1.0)
                        .contains(&fidelity)
                {
                    return Err(
                        RoutingError::invalid_configuration(
                            "topology.edge.fidelity",
                            "must be finite and within [0,1]",
                        ),
                    );
                }

                // Convert fidelity into a non-negative penalty.
                penalty +=
                    1.0 - fidelity;
            }
        }

        RoutingObjective::Weighted => {
            if let Some(error) =
                properties.error_rate
            {
                penalty +=
                    config.weights.error
                        * error;
            }

            if let Some(fidelity) =
                properties.fidelity
            {
                penalty +=
                    config.weights.fidelity
                        * (1.0 - fidelity);
            }

            if let Some(duration) =
                properties.duration
            {
                penalty +=
                    config.weights.duration
                        * duration
                            .as_secs_f64();
            }

            penalty +=
                config.weights.swap_count;
        }

        RoutingObjective::Depth => {
            penalty +=
                config.weights.depth;
        }

        RoutingObjective::SwapCount
        | RoutingObjective::Custom(_) => {}
    }

    if !penalty.is_finite()
        || penalty < 0.0
    {
        return Err(
            RoutingError::internal_invariant(
                "hardware candidate penalty became invalid",
            )
        );
    }

    Ok(penalty)
}

fn objective_scale(
    objective: &RoutingObjective,
    weights: &crate::quantum::routing::config::RoutingWeights,
) -> f64 {
    match objective {
        RoutingObjective::SwapCount => {
            1.0
        }

        RoutingObjective::Depth => {
            1.0 + weights.depth
        }

        RoutingObjective::Duration => {
            1.0 + weights.duration
        }

        RoutingObjective::Error => {
            1.0 + weights.error
        }

        RoutingObjective::Fidelity => {
            1.0 + weights.fidelity
        }

        RoutingObjective::Weighted => {
            1.0
                + weights.swap_count
                + weights.depth
                + weights.duration
                + weights.error
                + weights.fidelity
        }

        RoutingObjective::Custom(_) => {
            1.0
        }
    }
}

// =============================================================================
// Decay state
// =============================================================================

/// Tracks recent physical-qubit activity.
///
/// Values are always >= 1.0.
///
/// A candidate touching recently active qubits receives a larger multiplier.
#[derive(Debug, Clone)]
struct DecayState {
    values:
        BTreeMap<PhysicalQubitId, f64>,
}

impl DecayState {
    fn new(
        topology: &Topology,
    ) -> Self {
        let mut values =
            BTreeMap::new();

        for qubit in
            topology.qubits()
        {
            values.insert(
                qubit,
                1.0,
            );
        }

        Self { values }
    }

    fn factor(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> f64 {
        let a_value =
            self.values
                .get(&a)
                .copied()
                .unwrap_or(1.0);

        let b_value =
            self.values
                .get(&b)
                .copied()
                .unwrap_or(1.0);

        a_value.max(b_value)
    }

    fn record(
        &mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
        increment: f64,
    ) {
        let current_a =
            self.values
                .get(&a)
                .copied()
                .unwrap_or(1.0);

        let current_b =
            self.values
                .get(&b)
                .copied()
                .unwrap_or(1.0);

        self.values.insert(
            a,
            current_a
                .max(1.0)
                + increment,
        );

        self.values.insert(
            b,
            current_b
                .max(1.0)
                + increment,
        );
    }

    /// Gradually relaxes old activity toward the neutral value 1.0.
    fn relax(&mut self) {
        for value in
            self.values.values_mut()
        {
            *value =
                1.0
                    + (*value - 1.0)
                        * 0.95;
        }
    }
}

// =============================================================================
// Seed handling
// =============================================================================

fn effective_seed(
    config: &RoutingConfig,
) -> u64 {
    if let Some(seed) =
        config.seed
    {
        return seed;
    }

    // A fixed seed is deliberately used for deterministic mode.
    //
    // For non-deterministic mode, use monotonic process-local time. This is
    // not used for cryptography; it only selects a routing trial.
    if config.deterministic {
        return 0x5ABRE_2026_u64;
    }

    let now =
        Instant::now();

    let address =
        (&now as *const Instant)
            as usize;

    mix64(
        address as u64
            ^ now
                .elapsed()
                .as_nanos()
                as u64,
    )
}

fn derive_trial_seed(
    seed: u64,
    trial: u64,
) -> u64 {
    mix64(
        seed
            ^ trial
                .wrapping_mul(
                    0x9E37_79B9_7F4A_7C15,
                ),
    )
}

fn mix_seed(
    seed: u64,
    value: u64,
    salt: u64,
) -> u64 {
    mix64(
        seed
            ^ value.rotate_left(17)
            ^ salt,
    )
}

/// SplitMix64-style mixing.
///
/// This is used only for deterministic routing tie-breaking and trial
/// derivation. It is not a cryptographic primitive.
fn mix64(
    mut value: u64,
) -> u64 {
    value =
        value.wrapping_add(
            0x9E37_79B9_7F4A_7C15,
        );

    let mut z =
        value;

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

fn candidate_tie_break(
    seed: u64,
    a: PhysicalQubitId,
    b: PhysicalQubitId,
    iteration: usize,
) -> u64 {
    let canonical_a =
        a.index().min(b.index())
            as u64;

    let canonical_b =
        a.index().max(b.index())
            as u64;

    mix64(
        seed
            ^ canonical_a.rotate_left(7)
            ^ canonical_b.rotate_left(23)
            ^ (iteration as u64)
                .rotate_left(41),
    )
}

// =============================================================================
// Reverse workload
// =============================================================================

/// Reverses interaction order for SABRE bidirectional layout refinement.
///
/// This does not attempt to invert gates. The reverse pass is used only as a
/// mapping/layout heuristic, not as executable quantum semantics.
fn reverse_workload(
    workload: &RoutingWorkload,
) -> RoutingWorkload {
    let mut interactions =
        workload
            .interactions()
            .to_vec();

    interactions.reverse();

    RoutingWorkload::new(
        workload
            .logical_qubits()
            .to_vec(),
        interactions,
    )
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_interaction(
    interaction: &QubitInteraction,
    operation_index: usize,
) -> Result<(), RoutingError> {
    let arity =
        interaction.arity();

    if arity > MAX_ROUTING_ARITY {
        return Err(
            RoutingError::requires_decomposition(
                interaction.gate().name(),
                arity,
            )
            .with_diagnostic_context(
                crate::quantum::routing::errors::RoutingErrorContext::new()
                    .with_operation_index(
                        operation_index,
                    )
                    .with_gate(
                        interaction
                            .gate()
                            .name(),
                    )
                    .with_algorithm(
                        "sabre",
                    ),
            ),
        );
    }

    if arity == 0 {
        return Err(
            RoutingError::invalid_operand(
                format!(
                    "interaction at index {operation_index} has no operands",
                ),
            )
        );
    }

    if interaction
        .gate()
        .name()
        .trim()
        .is_empty()
    {
        return Err(
            RoutingError::unsupported_gate(
                "<empty>",
            )
        );
    }

    let operands =
        interaction.operands();

    for left in 0..operands.len() {
        for right in
            (left + 1)..operands.len()
        {
            if operands[left]
                == operands[right]
            {
                return Err(
                    RoutingError::invalid_operand(
                        format!(
                            "logical qubit {} appears more than once in interaction {}",
                            operands[left],
                            operation_index,
                        ),
                    )
                );
            }
        }
    }

    Ok(())
}

fn mapping_error(
    error: MappingError,
) -> RoutingError {
    RoutingError::inconsistent_mapping(
        error.to_string(),
    )
}

// =============================================================================
// Metrics
// =============================================================================

fn build_metrics(
    workload: &RoutingWorkload,
    topology: &Topology,
    route: &CandidateRoute,
    total_duration: Duration,
    trial_count: usize,
) -> Result<RoutingMetrics, RoutingError> {
    let logical_qubits =
        workload
            .logical_qubit_count();

    let physical_qubits =
        topology
            .qubit_count();

    let original_operations =
        workload
            .interaction_count();

    let final_operations =
        route.operations.len();

    let mut original_single =
        0usize;

    let mut original_two =
        0usize;

    let mut original_multi =
        0usize;

    for interaction in
        workload.interactions()
    {
        match interaction.arity() {
            0 | 1 => {
                original_single =
                    original_single
                        .saturating_add(1);
            }

            2 => {
                original_two =
                    original_two
                        .saturating_add(1);
            }

            _ => {
                original_multi =
                    original_multi
                        .saturating_add(1);
            }
        }
    }

    let final_gate_operations =
        route.operations
            .iter()
            .filter(
                |operation| {
                    operation.is_gate()
                },
            )
            .count();

    let inserted_moves =
        route.operations
            .iter()
            .filter(
                |operation| {
                    operation.is_move()
                },
            )
            .count();

    let inserted_bridges =
        route.operations
            .iter()
            .filter(
                |operation| {
                    matches!(
                        operation,
                        RoutingOperation::Move(
                            RoutingMove::Bridge { .. }
                        )
                    )
                },
            )
            .count();

    let inserted_permutations =
        route.operations
            .iter()
            .filter(
                |operation| {
                    matches!(
                        operation,
                        RoutingOperation::Move(
                            RoutingMove::Permutation { .. }
                        )
                    )
                },
            )
            .count();

    let routing_overhead =
        final_operations
            .checked_sub(
                original_operations,
            )
            .ok_or_else(|| {
                RoutingError::internal_invariant(
                    "final operation count is smaller than original operation count",
                )
            })?;

    let original_depth =
        approximate_workload_depth(
            workload,
        );

    let final_depth =
        approximate_depth(
            &route.operations,
        );

    let routing_depth =
        final_depth
            .saturating_sub(
                original_depth,
            );

    let mut metrics =
        RoutingMetrics::new(
            logical_qubits,
            physical_qubits,
        );

    metrics.original_operations =
        original_operations;

    metrics.final_operations =
        final_operations;

    metrics.original_single_qubit_operations =
        original_single;

    metrics.original_two_qubit_operations =
        original_two;

    metrics.original_multi_qubit_operations =
        original_multi;

    metrics.final_gate_operations =
        final_gate_operations;

    metrics.routed_two_qubit_operations =
        route.routed_two_qubit_operations;

    metrics.inserted_swaps =
        route.inserted_swaps;

    metrics.inserted_bridges =
        inserted_bridges;

    metrics.inserted_permutations =
        inserted_permutations;

    metrics.inserted_moves =
        inserted_moves;

    metrics.routing_overhead_operations =
        routing_overhead;

    metrics.original_depth =
        original_depth;

    metrics.final_depth =
        final_depth;

    metrics.routing_depth =
        routing_depth;

    metrics.original_two_qubit_depth =
        original_two;

    metrics.final_two_qubit_depth =
        final_gate_operations;

    metrics.routing_two_qubit_depth =
        route.inserted_swaps;

    metrics.routing_iterations =
        route.routing_iterations;

    metrics.candidate_evaluations =
        route.candidate_evaluations;

    metrics.candidate_rejections =
        route.candidate_rejections;

    metrics.trials =
        trial_count;

    metrics.layout_trials =
        trial_count;

    metrics.routing_trials =
        trial_count;

    metrics.total_duration =
        total_duration;

    metrics.routing_duration =
        route.routing_duration;

    metrics.verification_duration =
        Duration::ZERO;

    metrics.physical_two_qubit_operations =
        route.routed_two_qubit_operations;

    Ok(metrics)
}

fn approximate_workload_depth(
    workload: &RoutingWorkload,
) -> usize {
    let mut last_layer =
        BTreeMap::<
            LogicalQubitId,
            usize,
        >::new();

    let mut depth =
        0usize;

    for interaction in
        workload.interactions()
    {
        let mut layer =
            0usize;

        for logical in
            interaction.operands()
        {
            layer = layer.max(
                last_layer
                    .get(logical)
                    .copied()
                    .unwrap_or(0),
            );
        }

        layer =
            layer.saturating_add(1);

        for logical in
            interaction.operands()
        {
            last_layer.insert(
                *logical,
                layer,
            );
        }

        depth =
            depth.max(layer);
    }

    depth
}

fn approximate_depth(
    operations: &[RoutingOperation],
) -> usize {
    let mut last_layer =
        BTreeMap::<
            PhysicalQubitId,
            usize,
        >::new();

    let mut depth =
        0usize;

    for operation in
        operations
    {
        let touched =
            operation
                .physical_qubits();

        if touched.is_empty() {
            continue;
        }

        let mut layer =
            0usize;

        for physical in
            &touched
        {
            layer = layer.max(
                last_layer
                    .get(physical)
                    .copied()
                    .unwrap_or(0),
            );
        }

        layer =
            layer.saturating_add(1);

        for physical in
            touched
        {
            last_layer.insert(
                physical,
                layer,
            );
        }

        depth =
            depth.max(layer);
    }

    depth
}

// =============================================================================
// Reproducibility
// =============================================================================

fn build_reproducibility(
    workload: &RoutingWorkload,
    topology: &Topology,
    config: &RoutingConfig,
    route: &CandidateRoute,
    trial_count: usize,
) -> ReproducibilityMetadata {
    let input_hash =
        stable_workload_hash(
            workload,
        );

    let topology_hash =
        stable_topology_hash(
            topology,
        );

    let configuration_hash =
        stable_configuration_hash(
            config,
        );

    let result_hash =
        stable_operation_hash(
            &route.operations,
        );

    let routing_id =
        mix64(
            input_hash
                ^ topology_hash.rotate_left(13)
                ^ configuration_hash
                    .rotate_left(29)
                ^ result_hash
                    .rotate_left(43),
        );

    let mut metadata =
        if config.deterministic {
            ReproducibilityMetadata::deterministic()
        } else {
            ReproducibilityMetadata::nondeterministic()
        };

    metadata =
        metadata
            .with_routing_id(
                crate::quantum::routing::types::RoutingId::new(
                    routing_id,
                ),
            )
            .with_seed(
                RoutingSeed::new(
                    route.seed,
                ),
            )
            .with_routing_version(
                "zamani-routing-v1",
            )
            .with_algorithm_version(
                SABRE_ALGORITHM_VERSION,
            )
            .with_configuration_hash(
                format!(
                    "{configuration_hash:016x}"
                ),
            )
            .with_input_hash(
                format!(
                    "{input_hash:016x}"
                ),
            )
            .with_topology_hash(
                format!(
                    "{topology_hash:016x}"
                ),
            )
            .with_result_hash(
                format!(
                    "{result_hash:016x}"
                ),
            )
            .with_trial(
                0,
                trial_count,
            );

    metadata
}

// =============================================================================
// Stable fingerprints
// =============================================================================

fn stable_workload_hash(
    workload: &RoutingWorkload,
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    for logical in
        workload.logical_qubits()
    {
        hash =
            fnv_mix(
                hash,
                logical.index()
                    as u64,
            );
    }

    for interaction in
        workload.interactions()
    {
        hash =
            fnv_bytes(
                hash,
                interaction
                    .gate()
                    .name()
                    .as_bytes(),
            );

        for logical in
            interaction.operands()
        {
            hash =
                fnv_mix(
                    hash,
                    logical.index()
                        as u64,
                );
        }

        hash =
            fnv_mix(
                hash,
                interaction
                    .arity()
                    as u64,
            );
    }

    hash
}

fn stable_topology_hash(
    topology: &Topology,
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    for qubit in
        topology.qubits()
    {
        hash =
            fnv_mix(
                hash,
                qubit.index()
                    as u64,
            );

        hash =
            fnv_mix(
                hash,
                topology
                    .is_available(
                        qubit,
                    ) as u64,
            );
    }

    for edge in
        topology.edges()
    {
        hash =
            fnv_mix(
                hash,
                edge.a()
                    .index()
                    as u64,
            );

        hash =
            fnv_mix(
                hash,
                edge.b()
                    .index()
                    as u64,
            );

        hash =
            fnv_mix(
                hash,
                edge.direction()
                    as u64,
            );
    }

    hash
}

fn stable_configuration_hash(
    config: &RoutingConfig,
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    hash =
        fnv_bytes(
            hash,
            config.algorithm
                .name()
                .as_bytes(),
        );

    hash =
        fnv_bytes(
            hash,
            config.objective
                .name()
                .as_bytes(),
        );

    hash =
        fnv_mix(
            hash,
            config
                .deterministic
                as u64,
        );

    hash =
        fnv_mix(
            hash,
            config
                .allow_swap
                as u64,
        );

    hash =
        fnv_mix(
            hash,
            config
                .limits
                .max_iterations
                as u64,
        );

    hash =
        fnv_mix(
            hash,
            config
                .limits
                .candidate_limit
                as u64,
        );

    hash =
        fnv_mix(
            hash,
            config
                .limits
                .lookahead_depth
                as u64,
        );

    hash =
        fnv_mix(
            hash,
            config
                .limits
                .sabre_iterations
                as u64,
        );

    hash =
        fnv_mix(
            hash,
            config
                .limits
                .sabre_trials
                as u64,
        );

    hash
}

fn stable_operation_hash(
    operations: &[RoutingOperation],
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    for operation in
        operations
    {
        match operation {
            RoutingOperation::Move(
                RoutingMove::Swap { a, b },
            ) => {
                hash =
                    fnv_mix(
                        hash,
                        1,
                    );

                hash =
                    fnv_mix(
                        hash,
                        a.index()
                            as u64,
                    );

                hash =
                    fnv_mix(
                        hash,
                        b.index()
                            as u64,
                    );
            }

            RoutingOperation::Gate {
                gate,
                operands,
                logical_operands,
            } => {
                hash =
                    fnv_mix(
                        hash,
                        2,
                    );

                hash =
                    fnv_bytes(
                        hash,
                        gate.name()
                            .as_bytes(),
                    );

                for physical in
                    operands
                {
                    hash =
                        fnv_mix(
                            hash,
                            physical
                                .index()
                                as u64,
                        );
                }

                for logical in
                    logical_operands
                {
                    hash =
                        fnv_mix(
                            hash,
                            logical
                                .index()
                                as u64,
                        );
                }
            }

            RoutingOperation::Barrier {
                operands,
            } => {
                hash =
                    fnv_mix(
                        hash,
                        3,
                    );

                for physical in
                    operands
                {
                    hash =
                        fnv_mix(
                            hash,
                            physical
                                .index()
                                as u64,
                        );
                }
            }

            RoutingOperation::Move(
                RoutingMove::Bridge {
                    a,
                    bridge,
                    b,
                    gate,
                },
            ) => {
                hash =
                    fnv_mix(
                        hash,
                        4,
                    );

                hash =
                    fnv_mix(
                        hash,
                        a.index()
                            as u64,
                    );

                hash =
                    fnv_mix(
                        hash,
                        bridge.index()
                            as u64,
                    );

                hash =
                    fnv_mix(
                        hash,
                        b.index()
                            as u64,
                    );

                hash =
                    fnv_bytes(
                        hash,
                        gate.name()
                            .as_bytes(),
                    );
            }

            RoutingOperation::Move(
                RoutingMove::Permutation {
                    mapping,
                },
            ) => {
                hash =
                    fnv_mix(
                        hash,
                        5,
                    );

                for (logical, physical)
                    in mapping
                {
                    hash =
                        fnv_mix(
                            hash,
                            logical
                                .index()
                                as u64,
                        );

                    hash =
                        fnv_mix(
                            hash,
                            physical
                                .index()
                                as u64,
                        );
                }
            }
        }
    }

    hash
}

fn fnv_mix(
    hash: u64,
    value: u64,
) -> u64 {
    let mut result =
        hash;

    result ^=
        value;

    result =
        result.wrapping_mul(
            0x0000_0100_0000_01B3,
        );

    result
}

fn fnv_bytes(
    mut hash: u64,
    bytes: &[u8],
) -> u64 {
    for byte in
        bytes
    {
        hash =
            fnv_mix(
                hash,
                *byte as u64,
            );
    }

    hash
}

// =============================================================================
// Operation ordering
// =============================================================================

fn operation_stream_order(
    a: &[RoutingOperation],
    b: &[RoutingOperation],
) -> Ordering {
    let count =
        a.len().min(
            b.len(),
        );

    for index in 0..count {
        let left =
            operation_key(
                &a[index],
            );

        let right =
            operation_key(
                &b[index],
            );

        let ordering =
            left.cmp(&right);

        if ordering
            != Ordering::Equal
        {
            return ordering;
        }
    }

    a.len().cmp(
        &b.len(),
    )
}

fn operation_key(
    operation: &RoutingOperation,
) -> (
    u8,
    usize,
    usize,
    usize,
) {
    match operation {
        RoutingOperation::Move(
            RoutingMove::Swap { a, b },
        ) => (
            0,
            a.index(),
            b.index(),
            0,
        ),

        RoutingOperation::Gate {
            operands,
            logical_operands,
            ..
        } => (
            1,
            operands
                .first()
                .map(|q| q.index())
                .unwrap_or(0),
            operands
                .get(1)
                .map(|q| q.index())
                .unwrap_or(0),
            logical_operands
                .first()
                .map(|q| q.index())
                .unwrap_or(0),
        ),

        RoutingOperation::Barrier {
            operands,
        } => (
            2,
            operands
                .first()
                .map(|q| q.index())
                .unwrap_or(0),
            operands
                .get(1)
                .map(|q| q.index())
                .unwrap_or(0),
            0,
        ),

        RoutingOperation::Move(
            RoutingMove::Bridge {
                a,
                bridge,
                b,
                ..
            },
        ) => (
            3,
            a.index(),
            bridge.index(),
            b.index(),
        ),

        RoutingOperation::Move(
            RoutingMove::Permutation {
                mapping,
            },
        ) => {
            let first =
                mapping
                    .first()
                    .map(
                        |(logical, physical)| {
                            (
                                logical.index(),
                                physical.index(),
                            )
                        },
                    )
                    .unwrap_or((0, 0));

            (
                4,
                first.0,
                first.1,
                mapping.len(),
            )
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn line_mapping(
        count: usize,
    ) -> QubitMapping {
        QubitMapping::from_assignments(
            (0..count).map(|index| {
                (
                    LogicalQubitId::new(index),
                    PhysicalQubitId::new(index),
                )
            }),
        )
        .expect(
            "test mapping must be valid",
        )
    }

    fn line_workload(
        a: usize,
        b: usize,
    ) -> RoutingWorkload {
        let logical_a =
            LogicalQubitId::new(a);

        let logical_b =
            LogicalQubitId::new(b);

        RoutingWorkload::new(
            vec![
                logical_a,
                logical_b,
            ],
            vec![
                QubitInteraction::new(
                    vec![
                        logical_a,
                        logical_b,
                    ],
                    GateIdentity::Cx,
                ),
            ],
        )
    }

    #[test]
    fn sabre_has_stable_name() {
        assert_eq!(
            SabreRouter::new().name(),
            "sabre"
        );
    }

    #[test]
    fn heuristics_have_stable_names() {
        assert_eq!(
            SabreHeuristic::Basic.name(),
            "basic"
        );

        assert_eq!(
            SabreHeuristic::Lookahead.name(),
            "lookahead"
        );

        assert_eq!(
            SabreHeuristic::Decay.name(),
            "decay"
        );
    }

    #[test]
    fn deterministic_seed_is_stable() {
        let config =
            RoutingConfig::default();

        assert_eq!(
            effective_seed(&config),
            effective_seed(&config)
        );
    }

    #[test]
    fn candidate_tie_break_is_stable() {
        let first =
            candidate_tie_break(
                42,
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
                7,
            );

        let second =
            candidate_tie_break(
                42,
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
                7,
            );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn reverse_workload_preserves_logical_qubits() {
        let workload =
            line_workload(0, 1);

        let reversed =
            reverse_workload(
                &workload,
            );

        assert_eq!(
            reversed.logical_qubits(),
            workload.logical_qubits()
        );

        assert_eq!(
            reversed.interactions(),
            workload.interactions()
        );
    }

    #[test]
    fn decay_state_relaxes_toward_one() {
        let topology =
            Topology::line(2)
                .expect(
                    "test topology",
                );

        let mut decay =
            DecayState::new(
                &topology,
            );

        decay.record(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            1.0,
        );

        assert!(
            decay.factor(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            ) > 1.0
        );

        decay.relax();

        assert!(
            decay.factor(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            ) > 1.0
        );
    }

    #[test]
    fn distance_matrix_contains_adjacent_distance() {
        let topology =
            Topology::line(3)
                .expect(
                    "test topology",
                );

        let matrix =
            DistanceMatrix::build(
                &topology,
            )
            .expect(
                "distance matrix",
            );

        assert_eq!(
            matrix.distance(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            ),
            Some(1)
        );
    }

    #[test]
    fn candidate_generation_is_restricted_to_front_neighborhood() {
        let topology =
            Topology::line(4)
                .expect(
                    "test topology",
                );

        let mapping =
            line_mapping(4);

        let workload =
            RoutingWorkload::new(
                (0..4)
                    .map(
                        LogicalQubitId::new,
                    )
                    .collect(),
                vec![
                    QubitInteraction::new(
                        vec![
                            LogicalQubitId::new(0),
                            LogicalQubitId::new(3),
                        ],
                        GateIdentity::Cx,
                    ),
                ],
            );

        let candidates =
            generate_candidates(
                &[0],
                &workload,
                &mapping,
                &topology,
                64,
            )
            .expect(
                "candidate generation",
            );

        assert!(
            candidates
                .iter()
                .all(|candidate| {
                    candidate.a
                        <= PhysicalQubitId::new(3)
                        && candidate.b
                            <= PhysicalQubitId::new(3)
                })
        );
    }

    #[test]
    fn interaction_conflict_is_symmetric() {
        let first =
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                GateIdentity::Cx,
            );

        let second =
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(1),
                    LogicalQubitId::new(2),
                ],
                GateIdentity::Cz,
            );

        assert!(
            interactions_conflict(
                &first,
                &second,
            )
        );

        assert!(
            interactions_conflict(
                &second,
                &first,
            )
        );
    }
}