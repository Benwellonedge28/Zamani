//! Zamani Quantum Routing — SABRE / LightSABRE
//!
//! Production logical-to-physical quantum routing using a SABRE-family
//! bidirectional heuristic search.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - SABRE forward routing;
//! - SABRE backward routing;
//! - bidirectional layout refinement;
//! - front-layer construction;
//! - extended-set construction;
//! - legal SWAP candidate generation;
//! - basic/lookahead/decay heuristics;
//! - deterministic seeded tie breaking;
//! - bounded routing trials;
//! - mapping evolution;
//! - routing-time limits;
//! - SWAP limits;
//! - route-level invariant validation;
//! - reproducibility fingerprints;
//! - construction of the stable `RoutingResult`.
//!
//! It does NOT own:
//!
//! - Quantum IR parsing;
//! - OpenQASM parsing;
//! - gate decomposition;
//! - basis translation;
//! - native SWAP decomposition;
//! - scheduling;
//! - pulse generation;
//! - hardware execution;
//! - calibration acquisition;
//! - QEC decoding;
//! - benchmarking orchestration.
//!
//! # Architectural position
//!
//! ```text
//! Canonical Quantum IR
//!        │
//!        ▼
//! RoutingWorkload
//!        │
//!        ▼
//! ┌──────────────────────┐
//! │ SABRE                │
//! │                      │
//! │ front layer          │
//! │ extended set         │
//! │ candidate SWAPs      │
//! │ heuristic evaluation │
//! │ mapping evolution    │
//! │ forward/backward     │
//! └──────────┬───────────┘
//!            │
//!            ▼
//!      RoutingResult
//!            │
//!            ▼
//! decomposition / scheduling / hardware
//! ```
//!
//! # Important semantic rule
//!
//! SABRE operates on two-qubit interactions. Single-qubit operations do not
//! require qubit routing and therefore belong outside the SABRE workload.
//! Multi-qubit operations must be decomposed or explicitly declared native
//! before entering this algorithm.
//!
//! This module consequently rejects arity != 2 rather than silently changing
//! the semantics of an operation.
//!
//! # SABRE
//!
//! SABRE is a heuristic rather than an exact optimizer. Minimum-SWAP routing
//! is computationally hard, so production routing must provide bounded,
//! reproducible heuristic search rather than pretending to guarantee global
//! optimality.
//!
//! The implementation follows the established SABRE pattern:
//!
//! ```text
//! initial mapping
//!       │
//!       ▼
//! forward routing
//!       │
//!       ▼
//! final mapping
//!       │
//!       ▼
//! reverse workload
//!       │
//!       ▼
//! backward routing
//!       │
//!       ▼
//! improved initial mapping
//!       │
//!       └───────────────► repeat
//! ```
//!
//! # Determinism
//!
//! When `RoutingConfig::deterministic` is true, all routing decisions depend
//! only on:
//!
//! - workload;
//! - topology;
//! - initial mapping;
//! - configuration;
//! - explicit seed.
//!
//! No pointer addresses, wall-clock values, hash-map iteration order, or
//! process-global state are used to make routing decisions.
//!
//! When no seed is supplied, deterministic mode uses a fixed algorithm seed.
//! Non-deterministic mode derives a seed from the operating-system-independent
//! monotonic clock. The latter is only used to vary heuristic trials; it is
//! never used for security.
//!
//! # Safety
//!
//! - Rust 1.97 / 1.97.1.
//! - Rust 2021.
//! - No `unsafe`.
//! - No FFI.
//! - No raw-pointer tricks.
//! - No global mutable state.
//! - No filesystem access.
//! - No network access.
//!
//! # Integration contract
//!
//! This implementation consumes the existing frozen routing contracts:
//!
//! - `types.rs`
//! - `errors.rs`
//! - `topology.rs`
//! - `mapping.rs`
//! - `config.rs`
//! - `result.rs`
//!
//! Later modules such as `router.rs` can call [`SabreRouter::route`] without
//! modifying this implementation.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::{Duration, Instant};

use crate::quantum::routing::config::{
    LayoutStrategy,
    RoutingConfig,
    RoutingObjective,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::{
    MappingError,
    QubitMapping,
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
/// Change this whenever routing semantics change in a reproducibility-relevant
/// way.
pub const SABRE_ALGORITHM_VERSION: &str = "zamani.sabre.v2";

/// Stable routing subsystem version.
pub const SABRE_ROUTING_VERSION: &str = "zamani-routing-v1";

/// Default extended-set contribution.
pub const DEFAULT_EXTENDED_SET_WEIGHT: f64 = 0.50;

/// Default decay increment.
pub const DEFAULT_DECAY_INCREMENT: f64 = 0.001;

/// Minimum accepted decay increment.
pub const MIN_DECAY_INCREMENT: f64 = 0.000_001;

/// Maximum accepted decay increment.
pub const MAX_DECAY_INCREMENT: f64 = 100.0;

/// Maximum number of candidates that this implementation will inspect in one
/// routing decision even if a larger value is requested by configuration.
pub const MAX_INTERNAL_CANDIDATES: usize = 1_000_000;

/// Maximum number of routing iterations permitted by this implementation.
pub const MAX_INTERNAL_ITERATIONS: usize = 100_000_000;

/// Maximum number of workload interactions accepted by this implementation.
pub const MAX_INTERNAL_INTERACTIONS: usize = 10_000_000;

/// Maximum number of physical qubits accepted by this implementation.
pub const MAX_INTERNAL_QUBITS: usize = 1_000_000;

/// SABRE supports two-qubit routing interactions.
pub const MAX_ROUTING_ARITY: usize = 2;

/// Fixed seed used only when deterministic routing has no explicit seed.
pub const DEFAULT_DETERMINISTIC_SEED: u64 = 0x5A_B4_E2_02_60_00_00_01;

// =============================================================================
// Heuristic
// =============================================================================

/// Candidate scoring strategy used by SABRE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SabreHeuristic {
    /// Score the current front layer.
    Basic,

    /// Score the current front layer and bounded future interactions.
    Lookahead,

    /// Lookahead plus recency/decay.
    Decay,
}

impl Default for SabreHeuristic {
    fn default() -> Self {
        Self::Decay
    }
}

impl SabreHeuristic {
    /// Stable machine-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Lookahead => "lookahead",
            Self::Decay => "decay",
        }
    }
}

// =============================================================================
// Public router
// =============================================================================

/// Immutable SABRE router configuration.
///
/// Mutable routing state is created per invocation, which makes this type safe
/// to reuse from independent compilation requests.
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

    /// Creates a SABRE router with the requested heuristic.
    #[must_use]
    pub const fn with_heuristic(heuristic: SabreHeuristic) -> Self {
        Self {
            heuristic,
            extended_set_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            decay_increment: DEFAULT_DECAY_INCREMENT,
        }
    }

    /// Creates a SABRE router with explicit heuristic parameters.
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

    /// Returns the selected heuristic.
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

    /// Returns the stable algorithm name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "sabre"
    }

    /// Routes a two-qubit workload.
    ///
    /// The caller's workload, topology, configuration and mapping are never
    /// mutated.
    ///
    /// The result contains the semantic routing operation stream and the final
    /// logical-to-physical mapping.
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
        )?;

        let distances = DistanceMatrix::build(topology)?;

        let base_seed = effective_seed(config);
        let trials = config.limits.sabre_trials.max(1);

        let mut best: Option<CandidateRoute> = None;

        for trial in 0..trials {
            self.check_timeout(started, config)?;

            let seed = derive_trial_seed(
                base_seed,
                trial as u64,
            );

            let candidate = self.bidirectional_trial(
                workload,
                topology,
                &distances,
                initial_mapping,
                config,
                seed,
                started,
            )?;

            if best
                .as_ref()
                .map(|existing| {
                    route_order(&candidate, existing) == Ordering::Less
                })
                .unwrap_or(true)
            {
                best = Some(candidate);
            }
        }

        let route = best.ok_or_else(|| {
            RoutingError::algorithm_incompatible(
                self.name(),
                "SABRE produced no candidate route",
            )
        })?;

        let elapsed = started.elapsed();

        self.finalize_result(
            workload,
            topology,
            initial_mapping,
            config,
            route,
            elapsed,
            trials,
        )
    }

    /// Routes using a temporary heuristic override.
    pub fn route_with_heuristic(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        initial_mapping: &QubitMapping,
        config: &RoutingConfig,
        heuristic: SabreHeuristic,
    ) -> Result<RoutingResult, RoutingError> {
        Self {
            heuristic,
            extended_set_weight: self.extended_set_weight,
            decay_increment: self.decay_increment,
        }
        .route(
            workload,
            topology,
            initial_mapping,
            config,
        )
    }

    // =========================================================================
    // Configuration validation
    // =========================================================================

    fn validate_configuration(
        &self,
        config: &RoutingConfig,
    ) -> Result<(), RoutingError> {
        if !self.extended_set_weight.is_finite()
            || self.extended_set_weight < 0.0
        {
            return Err(RoutingError::invalid_configuration(
                "sabre.extended_set_weight",
                "must be finite and non-negative",
            ));
        }

        if !self.decay_increment.is_finite()
            || self.decay_increment < MIN_DECAY_INCREMENT
            || self.decay_increment > MAX_DECAY_INCREMENT
        {
            return Err(RoutingError::invalid_configuration(
                "sabre.decay_increment",
                "must be finite and within the supported SABRE range",
            ));
        }

        if config.limits.max_iterations == 0
            || config.limits.max_iterations > MAX_INTERNAL_ITERATIONS
        {
            return Err(RoutingError::invalid_configuration(
                "limits.max_iterations",
                "must be non-zero and within the SABRE safety limit",
            ));
        }

        if config.limits.candidate_limit == 0
            || config.limits.candidate_limit > MAX_INTERNAL_CANDIDATES
        {
            return Err(RoutingError::invalid_configuration(
                "limits.candidate_limit",
                "must be non-zero and within the SABRE safety limit",
            ));
        }

        if config.limits.lookahead_depth == 0 {
            return Err(RoutingError::invalid_configuration(
                "limits.lookahead_depth",
                "must be greater than zero",
            ));
        }

        if config.limits.sabre_iterations == 0 {
            return Err(RoutingError::invalid_configuration(
                "limits.sabre_iterations",
                "must be greater than zero",
            ));
        }

        if config.limits.sabre_trials == 0 {
            return Err(RoutingError::invalid_configuration(
                "limits.sabre_trials",
                "must be greater than zero",
            ));
        }

        if !config.weights.is_valid() {
            return Err(RoutingError::invalid_configuration(
                "weights",
                "all routing weights must be finite and non-negative",
            ));
        }

        if config.objective == RoutingObjective::Weighted
            && config.weights.is_zero()
        {
            return Err(RoutingError::invalid_configuration(
                "weights",
                "weighted objective requires at least one non-zero weight",
            ));
        }

        if !config.allow_swap {
            return Err(RoutingError::incompatible_configuration(
                "algorithm=sabre",
                "allow_swap=false",
            ));
        }

        Ok(())
    }

    // =========================================================================
    // Input validation
    // =========================================================================

    fn validate_input(
        &self,
        workload: &RoutingWorkload,
        topology: &Topology,
        mapping: &QubitMapping,
    ) -> Result<(), RoutingError> {
        topology.validate()?;

        mapping
            .validate()
            .map_err(mapping_error)?;

        let qubit_count = topology.qubit_count();

        if qubit_count == 0 {
            return Err(RoutingError::empty_topology());
        }

        if qubit_count > MAX_INTERNAL_QUBITS {
            return Err(RoutingError::invalid_configuration(
                "topology.qubit_count",
                "exceeds SABRE safety limit",
            ));
        }

        if workload.interaction_count() > MAX_INTERNAL_INTERACTIONS {
            return Err(RoutingError::invalid_configuration(
                "workload.interaction_count",
                "exceeds SABRE safety limit",
            ));
        }

        if mapping.len() > qubit_count {
            return Err(RoutingError::insufficient_physical_qubits(
                mapping.len(),
                qubit_count,
            ));
        }

        for logical in workload.logical_qubits() {
            if !mapping.contains_logical(*logical) {
                return Err(
                    RoutingError::unknown_logical_qubit(
                        logical.to_string(),
                    ),
                );
            }
        }

        for (index, interaction) in
            workload.interactions().iter().enumerate()
        {
            validate_interaction(interaction, index)?;

            for logical in interaction.operands() {
                if !mapping.contains_logical(*logical) {
                    return Err(
                        RoutingError::unknown_logical_qubit(
                            logical.to_string(),
                        )
                        .with_diagnostic_context(
                            crate::quantum::routing::errors::
                                RoutingErrorContext::new()
                                .with_operation_index(index)
                                .with_algorithm(self.name()),
                        ),
                    );
                }
            }
        }

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
        started: Instant,
    ) -> Result<CandidateRoute, RoutingError> {
        let mut working_mapping = initial_mapping.clone();
        let mut best: Option<CandidateRoute> = None;

        let iterations = config.limits.sabre_iterations.max(1);

        for iteration in 0..iterations {
            self.check_timeout(started, config)?;

            let forward_seed = mix_seed(
                seed,
                iteration as u64,
                0xF0F0_F0F0_F0F0_F0F0,
            );

            let forward = self.route_direction(
                workload,
                topology,
                distances,
                &working_mapping,
                config,
                forward_seed,
                started,
            )?;

            if best
                .as_ref()
                .map(|existing| {
                    route_order(&forward, existing)
                        == Ordering::Less
                })
                .unwrap_or(true)
            {
                best = Some(forward.clone());
            }

            if iteration + 1 >= iterations {
                break;
            }

            /*
             * SABRE layout refinement:
             *
             * forward:
             *
             *   M0 -> circuit -> Mf
             *
             * reverse:
             *
             *   Mf -> reverse(circuit) -> Mb
             *
             * Mb becomes the starting mapping of the next forward pass.
             *
             * This is a layout search operation. It is not used as the final
             * executable route.
             */
            let reversed = reverse_workload(workload);

            let backward_seed = mix_seed(
                seed,
                iteration as u64,
                0x0F0F_0F0F_0F0F_0F0F,
            );

            let backward = self.route_direction(
                &reversed,
                topology,
                distances,
                &forward.final_mapping,
                config,
                backward_seed,
                started,
            )?;

            working_mapping = backward.final_mapping;
        }

        best.ok_or_else(|| {
            RoutingError::algorithm_incompatible(
                self.name(),
                "bidirectional SABRE produced no forward route",
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
        started: Instant,
    ) -> Result<CandidateRoute, RoutingError> {
        let mut mapping = initial_mapping.clone();

        mapping
            .validate()
            .map_err(mapping_error)?;

        let count = workload.interaction_count();

        if count == 0 {
            return Ok(CandidateRoute {
                initial_mapping: initial_mapping.clone(),
                final_mapping: mapping,
                operations: Vec::new(),
                inserted_swaps: 0,
                candidate_evaluations: 0,
                candidate_rejections: 0,
                routing_iterations: 0,
                routed_two_qubit_operations: 0,
                routing_duration: Duration::ZERO,
                seed,
            });
        }

        let mut completed = vec![false; count];
        let mut operations = Vec::new();

        let mut decay = DecayState::new(topology);

        let mut inserted_swaps = 0usize;
        let mut candidate_evaluations = 0usize;
        let mut candidate_rejections = 0usize;
        let mut routing_iterations = 0usize;
        let mut routed_two_qubit_operations = 0usize;

        let route_started = Instant::now();

        while !completed.iter().all(|done| *done) {
            self.check_timeout(started, config)?;

            routing_iterations = routing_iterations
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::internal_invariant(
                        "SABRE routing iteration counter overflow",
                    )
                })?;

            if routing_iterations > config.limits.max_iterations {
                return Err(
                    RoutingError::iteration_limit_exceeded(
                        config.limits.max_iterations,
                    ),
                );
            }

            let front = build_front_layer(
                workload,
                &completed,
            );

            if front.is_empty() {
                return Err(
                    RoutingError::algorithm_incompatible(
                        self.name(),
                        "unfinished workload has no dependency-ready front layer",
                    ),
                );
            }

            /*
             * A single routing iteration may make several gates executable.
             * This is important: inserting a SWAP should not be required when
             * another independent front-layer interaction is already legal.
             */
            let mut progress = false;

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
                    routed_two_qubit_operations =
                        routed_two_qubit_operations
                            .checked_add(1)
                            .ok_or_else(|| {
                                RoutingError::internal_invariant(
                                    "routed gate counter overflow",
                                )
                            })?;

                    progress = true;
                }
            }

            if progress {
                decay.relax();
                continue;
            }

            let extended = build_extended_set(
                workload,
                &completed,
                &front,
                config.limits.lookahead_depth,
            );

            let candidates = generate_candidates(
                &front,
                workload,
                &mapping,
                topology,
                config.limits.candidate_limit,
            )?;

            if candidates.is_empty() {
                return Err(RoutingError::no_candidate());
            }

            let mut scored =
                Vec::with_capacity(candidates.len());

            for candidate in candidates {
                candidate_evaluations =
                    candidate_evaluations
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::internal_invariant(
                                "candidate evaluation counter overflow",
                            )
                        })?;

                let score = self.score_candidate(
                    &candidate,
                    &front,
                    &extended,
                    workload,
                    &mapping,
                    topology,
                    distances,
                    &decay,
                    config,
                )?;

                scored.push(ScoredCandidate {
                    a: candidate.a,
                    b: candidate.b,
                    score,
                    tie_break: candidate_tie_break(
                        seed,
                        candidate.a,
                        candidate.b,
                        routing_iterations,
                    ),
                });
            }

            scored.sort_by(scored_candidate_order);

            let selected = scored
                .first()
                .ok_or_else(RoutingError::no_candidate)?;

            if let Some(max_swaps) = config.limits.max_swaps {
                if inserted_swaps >= max_swaps {
                    return Err(
                        RoutingError::swap_limit_exceeded(
                            max_swaps,
                        ),
                    );
                }
            }

            /*
             * The movement must be a genuine bidirectional physical SWAP.
             * A merely directed edge is insufficient because a semantic SWAP
             * exchanges two quantum states.
             */
            if !topology.is_bidirectionally_adjacent(
                selected.a,
                selected.b,
            ) {
                candidate_rejections =
                    candidate_rejections
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::internal_invariant(
                                "candidate rejection counter overflow",
                            )
                        })?;

                return Err(
                    RoutingError::non_adjacent_movement(
                        selected.a.index(),
                        selected.b.index(),
                    ),
                );
            }

            /*
             * Check availability immediately before committing the move.
             * This protects against topology implementations whose resource
             * state can change between candidate generation and commit.
             */
            if !topology.is_available(selected.a)
                || !topology.is_available(selected.b)
            {
                candidate_rejections =
                    candidate_rejections
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::internal_invariant(
                                "candidate rejection counter overflow",
                            )
                        })?;

                return Err(
                    RoutingError::unsupported_movement(
                        format!(
                            "candidate SWAP uses unavailable qubit(s): {} <-> {}",
                            selected.a,
                            selected.b,
                        ),
                    ),
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
                            "inserted SWAP counter overflow",
                        )
                    })?;

            decay.record(
                selected.a,
                selected.b,
                self.decay_increment,
            );

            if config.validate_mapping_after_move {
                mapping
                    .validate()
                    .map_err(mapping_error)?;
            }
        }

        mapping
            .validate()
            .map_err(mapping_error)?;

        let route = CandidateRoute {
            initial_mapping: initial_mapping.clone(),
            final_mapping: mapping,
            operations,
            inserted_swaps,
            candidate_evaluations,
            candidate_rejections,
            routing_iterations,
            routed_two_qubit_operations,
            routing_duration: route_started.elapsed(),
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
    ) -> Result<f64, RoutingError> {
        let mut speculative = mapping.clone();

        speculative
            .swap_physical(
                candidate.a,
                candidate.b,
            )
            .map_err(mapping_error)?;

        let front_cost = average_interaction_distance(
            front,
            workload,
            &speculative,
            distances,
        )?;

        let future_cost = average_interaction_distance(
            extended,
            workload,
            &speculative,
            distances,
        )?;

        let topology_cost =
            candidate_hardware_cost(
                candidate.a,
                candidate.b,
                topology,
                config,
            )?;

        let heuristic_cost = match self.heuristic {
            SabreHeuristic::Basic => front_cost,

            SabreHeuristic::Lookahead => {
                front_cost
                    + self.extended_set_weight
                        * future_cost
            }

            SabreHeuristic::Decay => {
                let base =
                    front_cost
                        + self.extended_set_weight
                            * future_cost;

                base * decay.factor(
                    candidate.a,
                    candidate.b,
                )
            }
        };

        /*
         * Candidate comparison is always finite and deterministic.
         *
         * Hardware cost is additive rather than multiplied into the topology
         * distance. This prevents a huge hardware-duration value from
         * accidentally changing the meaning of the SABRE distance heuristic.
         */
        let objective_cost =
            objective_cost(
                config.objective.clone(),
                &config.weights,
                front_cost,
                future_cost,
                topology_cost,
            );

        let score =
            heuristic_cost + objective_cost;

        if !score.is_finite() {
            return Err(
                RoutingError::internal_invariant(
                    "SABRE candidate score became non-finite",
                ),
            );
        }

        Ok(score)
    }

    // =========================================================================
    // Timeout
    // =========================================================================

    fn check_timeout(
        &self,
        started: Instant,
        config: &RoutingConfig,
    ) -> Result<(), RoutingError> {
        if let Some(timeout) = config.limits.timeout {
            if started.elapsed() > timeout {
                return Err(
                    RoutingError::routing_timeout(),
                );
            }
        }

        Ok(())
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
            .initial_mapping
            .validate()
            .map_err(mapping_error)?;

        route
            .final_mapping
            .validate()
            .map_err(mapping_error)?;

        let mut mapping =
            route.initial_mapping.clone();

        let mut consumed = Vec::<QubitInteraction>::new();

        for operation in &route.operations {
            match operation {
                RoutingOperation::Move(
                    RoutingMove::Swap { a, b },
                ) => {
                    if !topology.is_bidirectionally_adjacent(
                        *a,
                        *b,
                    ) {
                        return Err(
                            RoutingError::verification_failed(
                                format!(
                                    "SABRE emitted illegal SWAP {} <-> {}",
                                    a, b
                                ),
                            ),
                        );
                    }

                    if !topology.is_available(*a)
                        || !topology.is_available(*b)
                    {
                        return Err(
                            RoutingError::verification_failed(
                                format!(
                                    "SABRE emitted SWAP on unavailable qubit(s): {} <-> {}",
                                    a, b
                                ),
                            ),
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
                    if operands.len() != 2
                        || logical_operands.len() != 2
                    {
                        return Err(
                            RoutingError::verification_failed(
                                "SABRE produced a non-two-qubit gate",
                            ),
                        );
                    }

                    if !topology.supports_gate(
                        gate.name(),
                        operands[0],
                        operands[1],
                    ) {
                        return Err(
                            RoutingError::illegal_routed_operation(
                                gate.name(),
                                operands
                                    .iter()
                                    .map(|q| q.index())
                                    .collect(),
                            ),
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

                    if expected_a != operands[0]
                        || expected_b != operands[1]
                    {
                        return Err(
                            RoutingError::verification_failed(
                                format!(
                                    "SABRE physical mapping mismatch for gate `{}`",
                                    gate.name()
                                ),
                            ),
                        );
                    }

                    consumed.push(
                        QubitInteraction::new(
                            logical_operands.clone(),
                            gate.clone(),
                        ),
                    );
                }

                RoutingOperation::Barrier { .. } => {
                    /*
                     * SABRE does not generate barriers. If a future routing
                     * pipeline injects one, it is semantically neutral here.
                     */
                }

                RoutingOperation::Move(
                    RoutingMove::Bridge { .. },
                )
                | RoutingOperation::Move(
                    RoutingMove::Permutation { .. },
                ) => {
                    return Err(
                        RoutingError::verification_failed(
                            "SABRE produced an unsupported movement primitive",
                        ),
                    );
                }
            }
        }

        if consumed != workload.interactions() {
            return Err(
                RoutingError::verification_failed(
                    "SABRE changed logical interaction ordering or gate identity",
                ),
            );
        }

        if mapping != route.final_mapping {
            return Err(
                RoutingError::verification_failed(
                    "SABRE final mapping does not match emitted SWAP sequence",
                ),
            );
        }

        Ok(())
    }

    // =========================================================================
    // Result
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
        if route.initial_mapping != *original_mapping {
            return Err(
                RoutingError::internal_invariant(
                    "SABRE changed the caller's initial mapping",
                ),
            );
        }

        let metrics = build_metrics(
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
                    "sabre.algorithm-level-v2",
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
                        + workload.interaction_count()
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

        let result = RoutingResult::new(
            disposition,
            RoutingAlgorithm::Sabre,
            LayoutStrategy::Sabre,
            config.objective.clone(),
            config.mode,
            route.initial_mapping.snapshot(),
            route.final_mapping.snapshot(),
            route.operations,
            metrics,
            verification,
            reproducibility,
        );

        if !result.is_internally_consistent() {
            return Err(
                RoutingError::internal_invariant(
                    "constructed SABRE RoutingResult is inconsistent",
                ),
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

// =============================================================================
// Candidate representation
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SwapCandidate {
    a: PhysicalQubitId,
    b: PhysicalQubitId,
}

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
        .total_cmp(&b.score)
        .then_with(|| a.tie_break.cmp(&b.tie_break))
        .then_with(|| a.a.cmp(&b.a))
        .then_with(|| a.b.cmp(&b.b))
}

// =============================================================================
// Route comparison
// =============================================================================

fn route_order(
    a: &CandidateRoute,
    b: &CandidateRoute,
) -> Ordering {
    /*
     * Primary SABRE trial-selection objective:
     *
     * 1. inserted SWAPs;
     * 2. final depth;
     * 3. operation count;
     * 4. deterministic operation stream;
     * 5. seed.
     *
     * This mirrors the production principle that SWAP count is the dominant
     * route-quality signal while depth remains a useful tie breaker.
     */
    a.inserted_swaps
        .cmp(&b.inserted_swaps)
        .then_with(|| {
            approximate_depth(&a.operations)
                .cmp(&approximate_depth(&b.operations))
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
        .then_with(|| a.seed.cmp(&b.seed))
}

// =============================================================================
// Dependency front layer
// =============================================================================

fn build_front_layer(
    workload: &RoutingWorkload,
    completed: &[bool],
) -> Vec<usize> {
    let interactions = workload.interactions();
    let mut front = Vec::new();

    for index in 0..interactions.len() {
        if completed[index] {
            continue;
        }

        let current = &interactions[index];

        let mut blocked = false;

        for previous_index in 0..index {
            if completed[previous_index] {
                continue;
            }

            if interactions_conflict(
                &interactions[previous_index],
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

fn build_extended_set(
    workload: &RoutingWorkload,
    completed: &[bool],
    front: &[usize],
    depth: usize,
) -> Vec<usize> {
    if depth == 0 {
        return Vec::new();
    }

    let interactions = workload.interactions();

    let front_set: BTreeSet<usize> =
        front.iter().copied().collect();

    let mut extended = Vec::new();

    /*
     * We deliberately select future interactions in program order.
     *
     * This is deterministic and avoids treating unrelated future gates as
     * though they were part of the immediate SABRE extended set.
     */
    for index in 0..interactions.len() {
        if completed[index]
            || front_set.contains(&index)
        {
            continue;
        }

        if extended.len() >= depth {
            break;
        }

        let candidate = &interactions[index];

        if front.iter().any(|front_index| {
            interactions_conflict(
                &interactions[front_index],
                candidate,
            )
        }) {
            extended.push(index);
        }
    }

    extended
}

fn interactions_conflict(
    a: &QubitInteraction,
    b: &QubitInteraction,
) -> bool {
    a.operands().iter().any(|left| {
        b.operands()
            .iter()
            .any(|right| left == right)
    })
}

// =============================================================================
// Candidate generation
// =============================================================================

fn generate_candidates(
    front: &[usize],
    workload: &RoutingWorkload,
    mapping: &QubitMapping,
    topology: &Topology,
    candidate_limit: usize,
) -> Result<Vec<SwapCandidate>, RoutingError> {
    if candidate_limit == 0 {
        return Err(
            RoutingError::candidate_limit_exceeded(0),
        );
    }

    let mut active_physical =
        BTreeSet::<PhysicalQubitId>::new();

    for &index in front {
        let interaction =
            workload.interactions().get(index)
                .ok_or_else(|| {
                    RoutingError::internal_invariant(
                        "front-layer index outside workload",
                    )
                })?;

        for logical in interaction.operands() {
            let physical =
                mapping.physical_of(*logical)
                    .ok_or_else(|| {
                        RoutingError::unknown_logical_qubit(
                            logical.to_string(),
                        )
                    })?;

            active_physical.insert(physical);
        }
    }

    /*
     * A candidate is useful if at least one endpoint currently carries a
     * qubit participating in the blocked front layer.
     *
     * This is substantially cheaper than evaluating every physical edge.
     */
    let mut candidates =
        BTreeSet::<(
            PhysicalQubitId,
            PhysicalQubitId,
        )>::new();

    for edge in topology.edges() {
        let a = edge.a();
        let b = edge.b();

        if !active_physical.contains(&a)
            && !active_physical.contains(&b)
        {
            continue;
        }

        if !topology.is_bidirectionally_adjacent(
            a,
            b,
        ) {
            continue;
        }

        if !topology.is_available(a)
            || !topology.is_available(b)
        {
            continue;
        }

        let pair =
            if a <= b {
                (a, b)
            } else {
                (b, a)
            };

        candidates.insert(pair);

        if candidates.len() >= candidate_limit {
            break;
        }
    }

    Ok(candidates
        .into_iter()
        .map(|(a, b)| SwapCandidate { a, b })
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
    if interaction.arity() != 2 {
        return Err(
            RoutingError::unsupported_arity(
                interaction.gate().name(),
                interaction.arity(),
            ),
        );
    }

    let operands = interaction.operands();

    let a = mapping
        .physical_of(operands[0])
        .ok_or_else(|| {
            RoutingError::unknown_logical_qubit(
                operands[0].to_string(),
            )
        })?;

    let b = mapping
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
        ),
    )
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
            ),
        );
    }

    let logical = interaction.operands();

    let physical_a = mapping
        .physical_of(logical[0])
        .ok_or_else(|| {
            RoutingError::unknown_logical_qubit(
                logical[0].to_string(),
            )
        })?;

    let physical_b = mapping
        .physical_of(logical[1])
        .ok_or_else(|| {
            RoutingError::unknown_logical_qubit(
                logical[1].to_string(),
            )
        })?;

    Ok(
        RoutingOperation::Gate {
            gate: interaction.gate().clone(),
            operands: vec![physical_a, physical_b],
            logical_operands: logical.to_vec(),
        },
    )
}

// =============================================================================
// Distance matrix
// =============================================================================

/// Cached all-pairs shortest-path distances over the bidirectional SWAP graph.
///
/// The graph is constructed once per routing invocation rather than once per
/// candidate.
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
            topology.qubits().collect::<Vec<_>>();

        let mut distances =
            BTreeMap::new();

        /*
         * BFS is appropriate because every semantic SWAP edge has unit
         * movement cost at the topology-distance level.
         */
        for source in &qubits {
            let mut queue =
                VecDeque::new();

            let mut local =
                BTreeMap::<
                    PhysicalQubitId,
                    usize,
                >::new();

            local.insert(*source, 0);
            queue.push_back(*source);

            while let Some(current) =
                queue.pop_front()
            {
                let current_distance =
                    local[&current];

                for edge in topology.edges() {
                    let neighbour =
                        if edge.a() == current {
                            Some(edge.b())
                        } else if edge.b() == current {
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

                    if !topology.is_available(neighbour) {
                        continue;
                    }

                    if local.contains_key(&neighbour) {
                        continue;
                    }

                    let next =
                        current_distance
                            .checked_add(1)
                            .ok_or_else(|| {
                                RoutingError::internal_invariant(
                                    "physical distance overflow",
                                )
                            })?;

                    local.insert(neighbour, next);
                    queue.push_back(neighbour);
                }
            }

            for target in &qubits {
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

        Ok(Self { distances })
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
// Distance heuristic
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

    let mut total = 0usize;
    let mut count = 0usize;

    for &index in indices {
        let interaction =
            workload.interactions().get(index)
                .ok_or_else(|| {
                    RoutingError::internal_invariant(
                        "interaction index outside workload",
                    )
                })?;

        let operands = interaction.operands();

        let a = mapping
            .physical_of(operands[0])
            .ok_or_else(|| {
                RoutingError::unknown_logical_qubit(
                    operands[0].to_string(),
                )
            })?;

        let b = mapping
            .physical_of(operands[1])
            .ok_or_else(|| {
                RoutingError::unknown_logical_qubit(
                    operands[1].to_string(),
                )
            })?;

        let distance =
            distances.distance(a, b)
                .ok_or_else(|| {
                    RoutingError::no_routing_path(
                        a.index(),
                        b.index(),
                    )
                })?;

        total = total
            .checked_add(distance)
            .ok_or_else(|| {
                RoutingError::internal_invariant(
                    "distance accumulation overflow",
                )
            })?;

        count = count
            .checked_add(1)
            .ok_or_else(|| {
                RoutingError::internal_invariant(
                    "distance interaction count overflow",
                )
            })?;
    }

    if count == 0 {
        return Ok(0.0);
    }

    let value =
        total as f64 / count as f64;

    if !value.is_finite() {
        return Err(
            RoutingError::internal_invariant(
                "distance heuristic became non-finite",
            ),
        );
    }

    Ok(value)
}

// =============================================================================
// Objective / hardware cost
// =============================================================================

fn candidate_hardware_cost(
    a: PhysicalQubitId,
    b: PhysicalQubitId,
    topology: &Topology,
    config: &RoutingConfig,
) -> Result<f64, RoutingError> {
    let properties =
        topology.edge_properties(a, b);

    let Some(properties) = properties
    else {
        return Ok(0.0);
    };

    if !properties.available {
        return Err(
            RoutingError::unsupported_movement(
                format!(
                    "physical edge {} <-> {} is unavailable",
                    a, b,
                ),
            ),
        );
    }

    let error =
        properties.error_rate.unwrap_or(0.0);

    let fidelity =
        properties.fidelity.unwrap_or(1.0);

    let duration =
        properties
            .duration
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0);

    if !error.is_finite()
        || !(0.0..=1.0).contains(&error)
    {
        return Err(
            RoutingError::invalid_configuration(
                "topology.edge.error_rate",
                "must be finite and within [0,1]",
            ),
        );
    }

    if !fidelity.is_finite()
        || !(0.0..=1.0).contains(&fidelity)
    {
        return Err(
            RoutingError::invalid_configuration(
                "topology.edge.fidelity",
                "must be finite and within [0,1]",
            ),
        );
    }

    if !duration.is_finite()
        || duration < 0.0
    {
        return Err(
            RoutingError::invalid_configuration(
                "topology.edge.duration",
                "must be finite and non-negative",
            ),
        );
    }

    let weighted =
        match config.objective {
            RoutingObjective::SwapCount => 0.0,

            RoutingObjective::Depth => {
                config.weights.depth
            }

            RoutingObjective::Duration => {
                duration
            }

            RoutingObjective::Error => {
                error
            }

            RoutingObjective::Fidelity => {
                1.0 - fidelity
            }

            RoutingObjective::Weighted => {
                config.weights.duration * duration
                    + config.weights.error * error
                    + config.weights.fidelity
                        * (1.0 - fidelity)
                    + config.weights.swap_count
            }

            RoutingObjective::Lexicographic => {
                /*
                 * Lexicographic ordering is handled primarily by
                 * route_order(). The local candidate score still receives
                 * a small hardware-aware contribution so equal-SWAP
                 * candidates prefer better physical resources.
                 */
                error
                    + (1.0 - fidelity)
                    + duration
                        * config.weights.duration
            }

            RoutingObjective::Custom(_) => 0.0,
        };

    if !weighted.is_finite()
        || weighted < 0.0
    {
        return Err(
            RoutingError::internal_invariant(
                "hardware candidate cost became invalid",
            ),
        );
    }

    Ok(weighted)
}

fn objective_cost(
    objective: RoutingObjective,
    weights: &crate::quantum::routing::config::RoutingWeights,
    front_cost: f64,
    future_cost: f64,
    hardware_cost: f64,
) -> f64 {
    /*
     * This function deliberately does NOT multiply the SABRE distance by an
     * arbitrary "objective scale". The previous implementation did that,
     * which meant e.g. `RoutingObjective::Error` could still be dominated by
     * topological distance without a well-defined relationship between the
     * units.
     *
     * Here:
     *
     * - topology distance remains the SABRE heuristic;
     * - hardware properties contribute only through the selected objective.
     */
    match objective {
        RoutingObjective::SwapCount => 0.0,

        RoutingObjective::Depth => {
            weights.depth * front_cost
        }

        RoutingObjective::Duration => {
            hardware_cost
        }

        RoutingObjective::Error => {
            hardware_cost
        }

        RoutingObjective::Fidelity => {
            hardware_cost
        }

        RoutingObjective::Weighted => {
            weights.swap_count
                + weights.depth * front_cost
                + weights.duration * hardware_cost
                + weights.error * hardware_cost
                + weights.fidelity * hardware_cost
        }

        RoutingObjective::Lexicographic => {
            /*
             * Candidate-level lexicographic approximation:
             * topology remains primary and hardware quality breaks ties.
             */
            front_cost
                + (future_cost * 1.0e-6)
                + hardware_cost * 1.0e-9
        }

        RoutingObjective::Custom(_) => 0.0,
    }
}

// =============================================================================
// Decay
// =============================================================================

#[derive(Debug, Clone)]
struct DecayState {
    values:
        BTreeMap<PhysicalQubitId, f64>,
}

impl DecayState {
    fn new(topology: &Topology) -> Self {
        let mut values = BTreeMap::new();

        for qubit in topology.qubits() {
            values.insert(qubit, 1.0);
        }

        Self { values }
    }

    fn factor(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> f64 {
        let left =
            self.values.get(&a)
                .copied()
                .unwrap_or(1.0);

        let right =
            self.values.get(&b)
                .copied()
                .unwrap_or(1.0);

        left.max(right)
    }

    fn record(
        &mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
        increment: f64,
    ) {
        let left =
            self.values.get(&a)
                .copied()
                .unwrap_or(1.0);

        let right =
            self.values.get(&b)
                .copied()
                .unwrap_or(1.0);

        self.values.insert(
            a,
            left.max(1.0) + increment,
        );

        self.values.insert(
            b,
            right.max(1.0) + increment,
        );
    }

    fn relax(&mut self) {
        /*
         * SABRE's decay state is intentionally relaxed gradually. The
         * multiplication below is deterministic and keeps values >= 1.
         */
        for value in self.values.values_mut() {
            *value =
                1.0 + (*value - 1.0) * 0.95;
        }
    }
}

// =============================================================================
// Seed handling
// =============================================================================

fn effective_seed(config: &RoutingConfig) -> u64 {
    if let Some(seed) = config.seed {
        return seed;
    }

    if config.deterministic {
        return DEFAULT_DETERMINISTIC_SEED;
    }

    /*
     * No pointer-address entropy.
     *
     * Pointer addresses make reproducibility depend on allocator/process
     * layout and are inappropriate for a production compiler.
     *
     * `Instant` is used only as a cheap non-cryptographic source for
     * trial variation when the caller explicitly requested non-determinism.
     */
    let elapsed =
        Instant::now()
            .elapsed()
            .as_nanos() as u64;

    mix64(elapsed ^ 0xD1B5_4A32_9C77_1201)
}

fn derive_trial_seed(
    seed: u64,
    trial: u64,
) -> u64 {
    mix64(
        seed
            ^ trial.wrapping_mul(
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

/// SplitMix64-style deterministic mixing.
///
/// This is not a cryptographic primitive.
fn mix64(mut value: u64) -> u64 {
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

fn candidate_tie_break(
    seed: u64,
    a: PhysicalQubitId,
    b: PhysicalQubitId,
    iteration: usize,
) -> u64 {
    let left =
        a.index().min(b.index()) as u64;

    let right =
        a.index().max(b.index()) as u64;

    mix64(
        seed
            ^ left.rotate_left(7)
            ^ right.rotate_left(23)
            ^ (iteration as u64)
                .rotate_left(41),
    )
}

// =============================================================================
// Reverse workload
// =============================================================================

fn reverse_workload(
    workload: &RoutingWorkload,
) -> RoutingWorkload {
    let mut interactions =
        workload.interactions().to_vec();

    interactions.reverse();

    RoutingWorkload::new(
        workload.logical_qubits().to_vec(),
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
    let arity = interaction.arity();

    /*
     * SABRE is deliberately strict about arity.
     *
     * A 1-qubit operation does not need routing.
     * A 3+ qubit operation requires decomposition/native support before SABRE.
     */
    if arity != MAX_ROUTING_ARITY {
        if arity > MAX_ROUTING_ARITY {
            return Err(
                RoutingError::requires_decomposition(
                    interaction.gate().name(),
                    arity,
                )
                .with_diagnostic_context(
                    crate::quantum::routing::errors::
                        RoutingErrorContext::new()
                        .with_operation_index(
                            operation_index,
                        )
                        .with_gate(
                            interaction.gate().name(),
                        )
                        .with_algorithm("sabre"),
                ),
            );
        }

        return Err(
            RoutingError::unsupported_arity(
                interaction.gate().name(),
                arity,
            )
            .with_diagnostic_context(
                crate::quantum::routing::errors::
                    RoutingErrorContext::new()
                    .with_operation_index(
                        operation_index,
                    )
                    .with_algorithm("sabre"),
            ),
        );
    }

    if interaction.gate().name().trim().is_empty() {
        return Err(
            RoutingError::unsupported_gate(
                "<empty>",
            ),
        );
    }

    let operands =
        interaction.operands();

    if operands[0] == operands[1] {
        return Err(
            RoutingError::invalid_operand(
                format!(
                    "logical qubit {} occurs twice in interaction {}",
                    operands[0],
                    operation_index,
                ),
            ),
        );
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
        workload.logical_qubit_count();

    let physical_qubits =
        topology.qubit_count();

    let original_operations =
        workload.interaction_count();

    let final_operations =
        route.operations.len();

    let mut original_single = 0usize;
    let mut original_two = 0usize;
    let mut original_multi = 0usize;

    for interaction in workload.interactions() {
        match interaction.arity() {
            1 => {
                original_single =
                    original_single.saturating_add(1);
            }

            2 => {
                original_two =
                    original_two.saturating_add(1);
            }

            _ => {
                original_multi =
                    original_multi.saturating_add(1);
            }
        }
    }

    let final_gate_operations =
        route.operations.iter()
            .filter(|operation| operation.is_gate())
            .count();

    let inserted_moves =
        route.operations.iter()
            .filter(|operation| operation.is_move())
            .count();

    let inserted_bridges =
        route.operations.iter()
            .filter(|operation| {
                matches!(
                    operation,
                    RoutingOperation::Move(
                        RoutingMove::Bridge { .. }
                    )
                )
            })
            .count();

    let inserted_permutations =
        route.operations.iter()
            .filter(|operation| {
                matches!(
                    operation,
                    RoutingOperation::Move(
                        RoutingMove::Permutation { .. }
                    )
                )
            })
            .count();

    let routing_overhead =
        final_operations
            .checked_sub(original_operations)
            .ok_or_else(|| {
                RoutingError::internal_invariant(
                    "final operation count is smaller than original operation count",
                )
            })?;

    let original_depth =
        approximate_workload_depth(workload);

    let final_depth =
        approximate_depth(&route.operations);

    let routing_depth =
        final_depth.saturating_sub(original_depth);

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

    /*
     * These are routing-layer estimates. Exact hardware timing/depth belongs
     * to scheduling once native gate durations are known.
     */
    metrics.original_two_qubit_depth =
        original_two;

    metrics.final_two_qubit_depth =
        route.routed_two_qubit_operations
            .saturating_add(route.inserted_swaps);

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
        route.routed_two_qubit_operations
            .saturating_add(route.inserted_swaps);

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

    let mut depth = 0usize;

    for interaction in workload.interactions() {
        let mut layer = 0usize;

        for logical in interaction.operands() {
            layer = layer.max(
                last_layer
                    .get(logical)
                    .copied()
                    .unwrap_or(0),
            );
        }

        layer =
            layer.saturating_add(1);

        for logical in interaction.operands() {
            last_layer.insert(*logical, layer);
        }

        depth = depth.max(layer);
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

    let mut depth = 0usize;

    for operation in operations {
        let touched =
            operation.physical_qubits();

        if touched.is_empty() {
            continue;
        }

        let mut layer = 0usize;

        for physical in &touched {
            layer = layer.max(
                last_layer
                    .get(physical)
                    .copied()
                    .unwrap_or(0),
            );
        }

        layer =
            layer.saturating_add(1);

        for physical in touched {
            last_layer.insert(
                physical,
                layer,
            );
        }

        depth = depth.max(layer);
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
        stable_workload_hash(workload);

    let topology_hash =
        stable_topology_hash(topology);

    let configuration_hash =
        stable_configuration_hash(config);

    let result_hash =
        stable_operation_hash(
            &route.operations,
        );

    let routing_id =
        mix64(
            input_hash
                ^ topology_hash.rotate_left(13)
                ^ configuration_hash.rotate_left(29)
                ^ result_hash.rotate_left(43),
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
                crate::quantum::routing::types::
                    RoutingId::new(routing_id),
            )
            .with_seed(
                RoutingSeed::new(route.seed),
            )
            .with_routing_version(
                SABRE_ROUTING_VERSION,
            )
            .with_algorithm_version(
                SABRE_ALGORITHM_VERSION,
            )
            .with_configuration_hash(
                format!("{configuration_hash:016x}"),
            )
            .with_input_hash(
                format!("{input_hash:016x}"),
            )
            .with_topology_hash(
                format!("{topology_hash:016x}"),
            )
            .with_result_hash(
                format!("{result_hash:016x}"),
            )
            .with_trial(
                0,
                trial_count,
            );

    metadata
}

fn stable_workload_hash(
    workload: &RoutingWorkload,
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    for logical in workload.logical_qubits() {
        hash = fnv_mix(
            hash,
            logical.index() as u64,
        );
    }

    for interaction in workload.interactions() {
        hash = fnv_bytes(
            hash,
            interaction.gate().name().as_bytes(),
        );

        for logical in interaction.operands() {
            hash = fnv_mix(
                hash,
                logical.index() as u64,
            );
        }

        hash = fnv_mix(
            hash,
            interaction.arity() as u64,
        );
    }

    hash
}

fn stable_topology_hash(
    topology: &Topology,
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    for qubit in topology.qubits() {
        hash = fnv_mix(
            hash,
            qubit.index() as u64,
        );

        hash = fnv_mix(
            hash,
            topology.is_available(qubit) as u64,
        );
    }

    for edge in topology.edges() {
        hash = fnv_mix(
            hash,
            edge.a().index() as u64,
        );

        hash = fnv_mix(
            hash,
            edge.b().index() as u64,
        );

        /*
         * Avoid depending on an enum's discriminant representation in a
         * reproducibility hash.
         */
        hash = fnv_bytes(
            hash,
            format!("{:?}", edge.direction())
                .as_bytes(),
        );
    }

    hash
}

fn stable_configuration_hash(
    config: &RoutingConfig,
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    hash = fnv_bytes(
        hash,
        config.algorithm.name().as_bytes(),
    );

    hash = fnv_bytes(
        hash,
        config.objective.name().as_bytes(),
    );

    hash = fnv_mix(
        hash,
        config.deterministic as u64,
    );

    hash = fnv_mix(
        hash,
        config.allow_swap as u64,
    );

    hash = fnv_mix(
        hash,
        config.limits.max_iterations as u64,
    );

    hash = fnv_mix(
        hash,
        config.limits.candidate_limit as u64,
    );

    hash = fnv_mix(
        hash,
        config.limits.lookahead_depth as u64,
    );

    hash = fnv_mix(
        hash,
        config.limits.sabre_iterations as u64,
    );

    hash = fnv_mix(
        hash,
        config.limits.sabre_trials as u64,
    );

    hash
}

fn stable_operation_hash(
    operations: &[RoutingOperation],
) -> u64 {
    let mut hash =
        0xcbf2_9ce4_8422_2325_u64;

    for operation in operations {
        match operation {
            RoutingOperation::Move(
                RoutingMove::Swap { a, b },
            ) => {
                hash = fnv_mix(hash, 1);
                hash = fnv_mix(
                    hash,
                    a.index() as u64,
                );
                hash = fnv_mix(
                    hash,
                    b.index() as u64,
                );
            }

            RoutingOperation::Gate {
                gate,
                operands,
                logical_operands,
            } => {
                hash = fnv_mix(hash, 2);

                hash = fnv_bytes(
                    hash,
                    gate.name().as_bytes(),
                );

                for physical in operands {
                    hash = fnv_mix(
                        hash,
                        physical.index() as u64,
                    );
                }

                for logical in logical_operands {
                    hash = fnv_mix(
                        hash,
                        logical.index() as u64,
                    );
                }
            }

            RoutingOperation::Barrier {
                operands,
            } => {
                hash = fnv_mix(hash, 3);

                for physical in operands {
                    hash = fnv_mix(
                        hash,
                        physical.index() as u64,
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
                hash = fnv_mix(hash, 4);
                hash = fnv_mix(
                    hash,
                    a.index() as u64,
                );
                hash = fnv_mix(
                    hash,
                    bridge.index() as u64,
                );
                hash = fnv_mix(
                    hash,
                    b.index() as u64,
                );
                hash = fnv_bytes(
                    hash,
                    gate.name().as_bytes(),
                );
            }

            RoutingOperation::Move(
                RoutingMove::Permutation {
                    mapping,
                },
            ) => {
                hash = fnv_mix(hash, 5);

                for (logical, physical) in mapping {
                    hash = fnv_mix(
                        hash,
                        logical.index() as u64,
                    );

                    hash = fnv_mix(
                        hash,
                        physical.index() as u64,
                    );
                }
            }
        }
    }

    hash
}

fn fnv_mix(
    mut hash: u64,
    value: u64,
) -> u64 {
    hash ^= value;
    hash = hash.wrapping_mul(
        0x0000_0100_0000_01B3,
    );
    hash
}

fn fnv_bytes(
    mut hash: u64,
    bytes: &[u8],
) -> u64 {
    for byte in bytes {
        hash = fnv_mix(
            hash,
            *byte as u64,
        );
    }

    hash
}

// =============================================================================
// Deterministic operation ordering
// =============================================================================

fn operation_stream_order(
    a: &[RoutingOperation],
    b: &[RoutingOperation],
) -> Ordering {
    let count = a.len().min(b.len());

    for index in 0..count {
        let left =
            operation_key(&a[index]);

        let right =
            operation_key(&b[index]);

        let ordering =
            left.cmp(&right);

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    a.len().cmp(&b.len())
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

    fn line_mapping(count: usize) -> QubitMapping {
        QubitMapping::from_assignments(
            (0..count).map(|index| {
                (
                    LogicalQubitId::new(index),
                    PhysicalQubitId::new(index),
                )
            }),
        )
        .expect("test mapping must be valid")
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
            vec![logical_a, logical_b],
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
            "sabre",
        );
    }

    #[test]
    fn default_heuristic_is_decay() {
        assert_eq!(
            SabreRouter::new().heuristic(),
            SabreHeuristic::Decay,
        );
    }

    #[test]
    fn heuristic_names_are_stable() {
        assert_eq!(
            SabreHeuristic::Basic.name(),
            "basic",
        );

        assert_eq!(
            SabreHeuristic::Lookahead.name(),
            "lookahead",
        );

        assert_eq!(
            SabreHeuristic::Decay.name(),
            "decay",
        );
    }

    #[test]
    fn deterministic_seed_is_stable() {
        let config =
            RoutingConfig::default();

        assert_eq!(
            effective_seed(&config),
            effective_seed(&config),
        );
    }

    #[test]
    fn explicit_seed_wins() {
        let mut config =
            RoutingConfig::default();

        config.seed = Some(42);

        assert_eq!(
            effective_seed(&config),
            42,
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

        assert_eq!(first, second);
    }

    #[test]
    fn candidate_tie_break_is_endpoint_order_independent() {
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
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(1),
                7,
            );

        assert_eq!(first, second);
    }

    #[test]
    fn reverse_workload_reverses_program_order() {
        let workload =
            line_workload(0, 1);

        let reversed =
            reverse_workload(&workload);

        assert_eq!(
            reversed.logical_qubits(),
            workload.logical_qubits(),
        );

        assert_eq!(
            reversed.interactions().len(),
            workload.interactions().len(),
        );

        assert_eq!(
            reversed.interactions()[0],
            workload.interactions()[0],
        );
    }

    #[test]
    fn decay_state_relaxes_toward_one() {
        let topology =
            Topology::line(2)
                .expect("test topology");

        let mut decay =
            DecayState::new(&topology);

        decay.record(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            1.0,
        );

        let before =
            decay.factor(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            );

        decay.relax();

        let after =
            decay.factor(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            );

        assert!(before > after);
        assert!(after >= 1.0);
    }

    #[test]
    fn front_layer_contains_independent_operations() {
        let q0 =
            LogicalQubitId::new(0);

        let q1 =
            LogicalQubitId::new(1);

        let q2 =
            LogicalQubitId::new(2);

        let q3 =
            LogicalQubitId::new(3);

        let workload =
            RoutingWorkload::new(
                vec![q0, q1, q2, q3],
                vec![
                    QubitInteraction::new(
                        vec![q0, q1],
                        GateIdentity::Cx,
                    ),
                    QubitInteraction::new(
                        vec![q2, q3],
                        GateIdentity::Cx,
                    ),
                ],
            );

        let completed =
            vec![false, false];

        let front =
            build_front_layer(
                &workload,
                &completed,
            );

        assert_eq!(
            front,
            vec![0, 1],
        );
    }

    #[test]
    fn front_layer_respects_dependencies() {
        let q0 =
            LogicalQubitId::new(0);

        let q1 =
            LogicalQubitId::new(1);

        let workload =
            RoutingWorkload::new(
                vec![q0, q1],
                vec![
                    QubitInteraction::new(
                        vec![q0, q1],
                        GateIdentity::Cx,
                    ),
                    QubitInteraction::new(
                        vec![q0, q1],
                        GateIdentity::Cx,
                    ),
                ],
            );

        let completed =
            vec![false, false];

        let front =
            build_front_layer(
                &workload,
                &completed,
            );

        assert_eq!(
            front,
            vec![0],
        );
    }

    #[test]
    fn completed_dependency_is_released() {
        let q0 =
            LogicalQubitId::new(0);

        let q1 =
            LogicalQubitId::new(1);

        let workload =
            RoutingWorkload::new(
                vec![q0, q1],
                vec![
                    QubitInteraction::new(
                        vec![q0, q1],
                        GateIdentity::Cx,
                    ),
                    QubitInteraction::new(
                        vec![q0, q1],
                        GateIdentity::Cx,
                    ),
                ],
            );

        let completed =
            vec![true, false];

        let front =
            build_front_layer(
                &workload,
                &completed,
            );

        assert_eq!(
            front,
            vec![1],
        );
    }

    #[test]
    fn candidate_generation_is_deterministic() {
        let topology =
            Topology::line(4)
                .expect("test topology");

        let mapping =
            line_mapping(4);

        let workload =
            line_workload(0, 3);

        let front =
            vec![0];

        let first =
            generate_candidates(
                &front,
                &workload,
                &mapping,
                &topology,
                64,
            )
            .expect("candidate generation");

        let second =
            generate_candidates(
                &front,
                &workload,
                &mapping,
                &topology,
                64,
            )
            .expect("candidate generation");

        assert_eq!(
            first,
            second,
        );
    }

    #[test]
    fn distance_matrix_has_line_distances() {
        let topology =
            Topology::line(4)
                .expect("test topology");

        let distances =
            DistanceMatrix::build(&topology)
                .expect("distance matrix");

        assert_eq!(
            distances.distance(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            ),
            Some(3),
        );

        assert_eq!(
            distances.distance(
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(3),
            ),
            Some(2),
        );
    }

    #[test]
    fn invalid_arity_is_rejected() {
        let interaction =
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                ],
                GateIdentity::Cx,
            );

        assert!(
            validate_interaction(
                &interaction,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn duplicate_operands_are_rejected() {
        let q0 =
            LogicalQubitId::new(0);

        let interaction =
            QubitInteraction::new(
                vec![q0, q0],
                GateIdentity::Cx,
            );

        assert!(
            validate_interaction(
                &interaction,
                0,
            )
            .is_err()
        );
    }
}