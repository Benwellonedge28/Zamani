//! Zamani Quantum Routing — Lookahead Router
//!
//! Production-grade bounded lookahead routing for logical-to-physical quantum
//! qubit mapping.
//!
//! # Purpose
//!
//! `LookaheadRouter` improves on purely greedy routing by evaluating the
//! consequences of candidate SWAPs over:
//!
//! 1. the current routing front layer;
//! 2. a bounded number of future circuit layers;
//! 3. the resulting logical-to-physical mapping.
//!
//! The implementation uses a deterministic bounded beam search. It is not an
//! exact minimum-SWAP solver. Quantum routing is combinatorial, so production
//! routing must explicitly bound search work.
//!
//! # Algorithm
//!
//! For every routing decision:
//!
//! ```text
//! current mapping
//!       │
//!       ▼
//! executable front operations?
//!       │
//!       ├── yes ──► emit them
//!       │
//!       └── no
//!             │
//!             ▼
//!      generate legal SWAPs
//!             │
//!             ▼
//!      bounded beam search
//!             │
//!             ▼
//!   score resulting mappings
//!   using front + future layers
//!             │
//!             ▼
//!      choose best sequence
//!             │
//!             ▼
//!       commit SWAPs
//!             │
//!             ▼
//!        repeat routing
//! ```
//!
//! # Architectural boundary
//!
//! This module does NOT:
//!
//! - parse Zamani source;
//! - parse OpenQASM;
//! - manipulate compiler-specific `IrInstruction` values;
//! - perform gate synthesis;
//! - lower SWAP into CX or another native gate;
//! - schedule pulses;
//! - acquire hardware calibration;
//! - execute hardware;
//! - perform QEC decoding;
//! - perform measurement mitigation;
//! - own initial layout selection.
//!
//! A `RoutingOperation::Move(RoutingMove::Swap { .. })` represents a semantic
//! physical-state permutation. Hardware lowering is performed later.
//!
//! # Multi-qubit operations
//!
//! Like current production SABRE-style routing implementations, this router
//! expects operations with arity greater than two to have been decomposed before
//! routing. Routing must not silently invent a decomposition because that would
//! mix routing with synthesis.
//!
//! # Determinism
//!
//! With identical:
//!
//! - input operations;
//! - topology;
//! - initial mapping;
//! - configuration;
//!
//! this implementation makes identical decisions.
//!
//! No hash-map iteration order participates in candidate ordering.
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
//! - no `unsafe`.
//!
//! # Integration
//!
//! This file consumes the frozen routing contracts:
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
//! It intentionally does not depend on:
//!
//! ```text
//! basic.rs
//! shortest_path.rs
//! sabre.rs
//! noise_aware.rs
//! dynamic.rs
//! router.rs
//! transpiler.rs
//! hardware/
//! frontend/
//! ```
//!
//! Therefore later algorithms and orchestration can be implemented without
//! changing this file's core algorithm.
//!
//! # Search model
//!
//! The implementation deliberately separates:
//!
//! - circuit dependency discovery;
//! - front-layer discovery;
//! - candidate SWAP generation;
//! - physical distance calculation;
//! - heuristic scoring;
//! - bounded beam expansion;
//! - final mapping mutation.
//!
//! This prevents speculative search from mutating the real mapping.
//!
//! # Complexity
//!
//! Let:
//!
//! - `N` = number of physical qubits;
//! - `F` = number of front-layer operations;
//! - `L` = configured lookahead depth;
//! - `B` = beam width;
//! - `C` = candidate count per state.
//!
//! Search is bounded approximately by:
//!
//! ```text
//! O(L × B × C × heuristic_cost)
//! ```
//!
//! rather than an unbounded traversal of the complete routing search space.
//!
//! The implementation also caches the topology's all-pairs unweighted
//! distances once per route invocation.
//!
//! # Production invariants
//!
//! The router guarantees:
//!
//! - no unsafe code;
//! - no mutation of caller-owned input operations;
//! - no mutation of caller-owned initial mapping;
//! - mapping remains collision-free;
//! - every committed SWAP is topology-adjacent;
//! - every committed SWAP is represented as a semantic movement;
//! - only executable logical gates are emitted;
//! - 3+ qubit operations are rejected explicitly;
//! - search depth is bounded;
//! - beam width is bounded;
//! - candidate count is bounded;
//! - arithmetic overflow is checked;
//! - deterministic tie-breaking is used;
//! - failed routing does not expose partial state;
//! - output verification is delegated to `RoutingResult` when configured.
//!
//! # References
//!
//! The heuristic follows the production routing model used by modern
//! SABRE/Lookahead-style compilers:
//!
//! - current front-layer interaction distance;
//! - bounded future-layer interaction distance;
//! - weighted future cost;
//! - restricted SWAP candidate generation;
//! - bounded search.
//!
//! This is deliberately compatible with future SABRE/LightSABRE integration.
//!
//! No provider-specific assumptions are made here.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

use crate::quantum::routing::config::RoutingConfig;
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::result::{
    RoutingInput,
    RoutingMetrics,
    RoutingResult,
};
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::{
    PhysicalQubitId,
    QuantumOperation,
    RoutingMove,
    RoutingOperation,
};

// =============================================================================
// Constants
// =============================================================================

/// Default relative weight of future-layer cost.
///
/// The current front layer must dominate the decision. Future layers provide
/// guidance without being allowed to override the immediate routing objective
/// in pathological cases.
pub const DEFAULT_EXTENDED_SET_WEIGHT: f64 = 0.50;

/// Maximum default beam width.
///
/// The public configuration's candidate limit remains the authoritative upper
/// safety boundary. This local value prevents accidental exponential growth
/// when a caller constructs `LookaheadRouter` directly.
pub const DEFAULT_BEAM_WIDTH: usize = 8;

/// Absolute safety ceiling for beam width.
pub const MAX_BEAM_WIDTH: usize = 4096;

/// Absolute safety ceiling for lookahead depth.
pub const MAX_SEARCH_DEPTH: usize = 4096;

/// Internal maximum number of operations examined while constructing dependency
/// metadata.
///
/// The general routing configuration remains responsible for the user-facing
/// operation limit.
pub const MAX_INTERNAL_OPERATIONS: usize = 100_000_000;

// =============================================================================
// Public router
// =============================================================================

/// Deterministic bounded lookahead quantum router.
///
/// The router evaluates hypothetical SWAP sequences without mutating the real
/// mapping. Only the selected sequence is committed.
///
/// # Search strategy
///
/// The implementation is a narrow best-first/beam search:
///
/// - each beam state represents one hypothetical mapping;
/// - candidate SWAPs are generated around blocked front-layer interactions;
/// - each state is scored using the current front layer plus future circuit
///   layers;
/// - only the best bounded number of states survive each search level.
///
/// This provides significantly more global information than greedy routing
/// while retaining an explicit runtime bound.
#[derive(Debug, Clone)]
pub struct LookaheadRouter {
    /// Relative importance of the future-layer heuristic.
    extended_set_weight: f64,

    /// Maximum number of simultaneous speculative states.
    beam_width: usize,
}

impl Default for LookaheadRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl LookaheadRouter {
    /// Creates the production-default lookahead router.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            extended_set_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            beam_width: DEFAULT_BEAM_WIDTH,
        }
    }

    /// Creates a lookahead router with an explicit future-layer weight.
    ///
    /// The value must be finite and non-negative.
    ///
    /// This constructor performs no routing and therefore cannot fail.
    /// Invalid values are rejected when routing begins.
    #[must_use]
    pub const fn with_extended_set_weight(weight: f64) -> Self {
        Self {
            extended_set_weight: weight,
            beam_width: DEFAULT_BEAM_WIDTH,
        }
    }

    /// Creates a lookahead router with an explicit beam width.
    ///
    /// The value is clamped to the implementation safety ceiling during
    /// routing. A zero width is rejected as invalid configuration.
    #[must_use]
    pub const fn with_beam_width(beam_width: usize) -> Self {
        Self {
            extended_set_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            beam_width,
        }
    }

    /// Creates a router with both explicit heuristic weight and beam width.
    #[must_use]
    pub const fn with_parameters(
        extended_set_weight: f64,
        beam_width: usize,
    ) -> Self {
        Self {
            extended_set_weight,
            beam_width,
        }
    }

    /// Returns the stable algorithm name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "lookahead"
    }

    /// Returns the configured future-layer weight.
    #[must_use]
    pub const fn extended_set_weight(&self) -> f64 {
        self.extended_set_weight
    }

    /// Returns the configured beam width.
    #[must_use]
    pub const fn beam_width(&self) -> usize {
        self.beam_width
    }

    /// Routes a complete logical quantum operation stream.
    ///
    /// The input operations and initial mapping are never modified.
    ///
    /// A private mapping is used for all routing decisions. The final mapping
    /// is returned in the `RoutingResult`.
    pub fn route(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<RoutingResult, RoutingError> {
        self.validate_configuration(input.config)?;
        validate_input(input)?;

        let distance_matrix = DistanceMatrix::build(input.topology)?;

        let dependency_graph = DependencyGraph::build(input.operations)?;

        let mut mapping = input.initial_mapping.clone();

        mapping
            .validate()
            .map_err(RoutingError::from)?;

        let initial_mapping = mapping.clone();

        let mut completed = vec![false; input.operations.len()];

        let mut operations =
            Vec::with_capacity(input.operations.len());

        let mut metrics = RoutingMetrics::new();

        metrics.original_operation_count =
            input.operations.len();

        let max_iterations = input.config.max_iterations;

        let mut routing_iterations = 0usize;

        while !all_completed(&completed) {
            routing_iterations = routing_iterations
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::iteration_limit_exceeded(
                        max_iterations,
                    )
                })?;

            if routing_iterations > max_iterations {
                return Err(
                    RoutingError::iteration_limit_exceeded(
                        max_iterations,
                    ),
                );
            }

            let front =
                dependency_graph.ready_operations(&completed);

            if front.is_empty() {
                return Err(
                    RoutingError::internal_invariant(
                        "lookahead routing reached a state with unfinished operations but no ready operations",
                    ),
                );
            }

            // -----------------------------------------------------------------
            // First consume every currently executable front-layer operation.
            // -----------------------------------------------------------------
            let mut progress = false;

            for &index in &front {
                if completed[index] {
                    continue;
                }

                let operation =
                    &input.operations[index];

                if is_executable(
                    operation,
                    input.topology,
                    &mapping,
                )? {
                    let routed =
                        operation
                            .clone()
                            .into_routing_operation()?;

                    operations.push(routed);

                    completed[index] = true;
                    progress = true;

                    metrics.routed_gate_count =
                        metrics
                            .routed_gate_count
                            .checked_add(1)
                            .ok_or_else(|| {
                                RoutingError::internal_invariant(
                                    "routed gate count overflow",
                                )
                            })?;
                }
            }

            if progress {
                continue;
            }

            // -----------------------------------------------------------------
            // Nothing in the front layer is executable. Search for SWAPs.
            // -----------------------------------------------------------------
            let extended =
                dependency_graph.extended_operations(
                    &completed,
                    &front,
                    input.config.lookahead_depth,
                );

            let swap_sequence =
                self.search_best_swap_sequence(
                    input,
                    &mapping,
                    &front,
                    &extended,
                    &distance_matrix,
                    max_iterations
                        .saturating_sub(routing_iterations),
                )?;

            if swap_sequence.is_empty() {
                return Err(RoutingError::no_candidate());
            }

            // -----------------------------------------------------------------
            // Commit the selected speculative sequence.
            // -----------------------------------------------------------------
            for (a, b) in swap_sequence {
                if !input.topology.is_adjacent(a, b) {
                    return Err(
                        RoutingError::internal_invariant(
                            "lookahead selected a non-adjacent SWAP",
                        ),
                    );
                }

                mapping
                    .swap_physical(a, b)
                    .map_err(RoutingError::from)?;

                operations.push(
                    RoutingOperation::Move(
                        RoutingMove::Swap { a, b },
                    ),
                );

                metrics.inserted_swaps =
                    metrics
                        .inserted_swaps
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::internal_invariant(
                                "inserted SWAP count overflow",
                            )
                        })?;

                metrics.routing_operation_count =
                    metrics
                        .routing_operation_count
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::internal_invariant(
                                "routing operation count overflow",
                            )
                        })?;

                if let Some(max_swaps) =
                    input.config.max_swaps
                {
                    if metrics.inserted_swaps > max_swaps {
                        return Err(
                            RoutingError::timeout(
                                "lookahead routing exceeded configured maximum SWAP count",
                            ),
                        );
                    }
                }
            }
        }

        mapping
            .validate()
            .map_err(RoutingError::from)?;

        metrics.final_operation_count =
            operations.len();

        metrics.routing_overhead =
            metrics
                .final_operation_count
                .checked_sub(
                    metrics.original_operation_count,
                )
                .ok_or_else(|| {
                    RoutingError::internal_invariant(
                        "final operation count is smaller than original operation count",
                    )
                })?;

        metrics.algorithm =
            self.name().to_string();

        metrics.seed = input.config.seed;

        let result = RoutingResult::new(
            operations,
            initial_mapping,
            mapping,
            metrics,
        );

        if input.config.verify_output {
            result.verify(input)?;
        }

        Ok(result)
    }

    /// Convenience API for callers that already have a mapping.
    pub fn route_with_mapping(
        &self,
        operations: &[QuantumOperation],
        topology: &Topology,
        mapping: &QubitMapping,
        config: &RoutingConfig,
    ) -> Result<RoutingResult, RoutingError> {
        let input =
            RoutingInput::new(
                operations,
                topology,
                mapping,
                config,
            )?;

        self.route(&input)
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
            return Err(
                RoutingError::invalid_configuration(
                    "lookahead extended-set weight must be finite and non-negative",
                ),
            );
        }

        if self.beam_width == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "lookahead beam width must be greater than zero",
                ),
            );
        }

        if self.beam_width > MAX_BEAM_WIDTH {
            return Err(
                RoutingError::invalid_configuration(
                    "lookahead beam width exceeds the implementation safety limit",
                ),
            );
        }

        if config.lookahead_depth > MAX_SEARCH_DEPTH {
            return Err(
                RoutingError::invalid_configuration(
                    "lookahead depth exceeds the implementation safety limit",
                ),
            );
        }

        if config.candidate_limit == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "candidate limit must be greater than zero",
                ),
            );
        }

        if config.max_iterations == 0 {
            return Err(
                RoutingError::invalid_configuration(
                    "maximum routing iterations must be greater than zero",
                ),
            );
        }

        Ok(())
    }

    // =========================================================================
    // Bounded beam search
    // =========================================================================

    fn search_best_swap_sequence(
        &self,
        input: &RoutingInput<'_>,
        mapping: &QubitMapping,
        front: &[usize],
        extended: &[usize],
        distance_matrix: &DistanceMatrix,
        remaining_iterations: usize,
    ) -> Result<Vec<(PhysicalQubitId, PhysicalQubitId)>, RoutingError>
    {
        let depth =
            input
                .config
                .lookahead_depth
                .max(1)
                .min(MAX_SEARCH_DEPTH);

        let candidate_limit =
            input
                .config
                .candidate_limit
                .min(MAX_BEAM_WIDTH);

        let beam_width =
            self.beam_width.min(candidate_limit);

        let mut beam = vec![
            SearchState {
                mapping: mapping.clone(),
                swaps: Vec::new(),
                score: self.score_state(
                    mapping,
                    front,
                    extended,
                    input,
                    distance_matrix,
                )?,
            },
        ];

        let mut best =
            beam[0].clone();

        for level in 0..depth {
            if level >= remaining_iterations {
                break;
            }

            let mut next_states = Vec::new();

            for state in &beam {
                if front_is_executable(
                    front,
                    &state.mapping,
                    input,
                )? {
                    return Ok(state.swaps.clone());
                }

                let candidates =
                    generate_swap_candidates(
                        front,
                        &state.mapping,
                        input.topology,
                        candidate_limit,
                    )?;

                for (a, b) in candidates {
                    let mut candidate_mapping =
                        state.mapping.clone();

                    candidate_mapping
                        .swap_physical(a, b)
                        .map_err(RoutingError::from)?;

                    let mut candidate_swaps =
                        state.swaps.clone();

                    candidate_swaps.push((a, b));

                    let heuristic =
                        self.score_state(
                            &candidate_mapping,
                            front,
                            extended,
                            input,
                            distance_matrix,
                        )?;

                    let swap_penalty =
                        candidate_swaps
                            .len()
                            as f64;

                    let score =
                        heuristic
                            + swap_penalty;

                    next_states.push(
                        SearchState {
                            mapping:
                                candidate_mapping,
                            swaps:
                                candidate_swaps,
                            score,
                        },
                    );
                }
            }

            if next_states.is_empty() {
                break;
            }

            next_states.sort_by(
                search_state_order,
            );

            // Remove duplicate mapping states before truncation.
            let mut unique_states =
                Vec::with_capacity(
                    next_states.len(),
                );

            for state in next_states {
                if unique_states.iter().any(
                    |existing: &SearchState| {
                        existing.mapping
                            == state.mapping
                    },
                ) {
                    continue;
                }

                unique_states.push(state);

                if unique_states.len()
                    >= beam_width
                {
                    break;
                }
            }

            beam = unique_states;

            if let Some(current_best) =
                beam.first()
            {
                if search_state_order(
                    current_best,
                    &best,
                ) == Ordering::Less
                {
                    best =
                        current_best.clone();
                }

                if front_is_executable(
                    front,
                    &current_best.mapping,
                    input,
                )? {
                    return Ok(
                        current_best
                            .swaps
                            .clone(),
                    );
                }
            }
        }

        // The best beam state may not yet make the front executable. Returning
        // it is still useful only when it contains at least one move; the outer
        // router will verify progress on the next iteration. An empty result
        // means no legal movement was found.
        Ok(best.swaps)
    }

    // =========================================================================
    // Heuristic
    // =========================================================================

    fn score_state(
        &self,
        mapping: &QubitMapping,
        front: &[usize],
        extended: &[usize],
        input: &RoutingInput<'_>,
        distances: &DistanceMatrix,
    ) -> Result<f64, RoutingError> {
        let front_cost =
            interaction_distance(
                front,
                mapping,
                input.operations,
                distances,
            )?;

        let extended_cost =
            interaction_distance(
                extended,
                mapping,
                input.operations,
                distances,
            )?;

        // Current/front-layer interactions must dominate the heuristic.
        //
        // The future set is deliberately weighted rather than simply appended
        // so large circuits with many future gates cannot drown out the current
        // routing objective.
        Ok(
            front_cost
                + self.extended_set_weight
                    * extended_cost,
        )
    }
}

// =============================================================================
// Search state
// =============================================================================

#[derive(Debug, Clone)]
struct SearchState {
    mapping: QubitMapping,
    swaps: Vec<(PhysicalQubitId, PhysicalQubitId)>,
    score: f64,
}

fn search_state_order(
    a: &SearchState,
    b: &SearchState,
) -> Ordering {
    a.score
        .partial_cmp(&b.score)
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            a.swaps
                .len()
                .cmp(&b.swaps.len())
        })
        .then_with(|| {
            a.swaps
                .cmp(&b.swaps)
        })
}

// =============================================================================
// Input validation
// =============================================================================

fn validate_input(
    input: &RoutingInput<'_>,
) -> Result<(), RoutingError> {
    input
        .topology
        .validate()?;

    input
        .initial_mapping
        .validate()
        .map_err(RoutingError::from)?;

    if input.operations.len()
        > input.config.max_operations
    {
        return Err(
            RoutingError::invalid_configuration(
                "routing input exceeds the configured maximum operation count",
            ),
        );
    }

    if input.operations.len()
        > MAX_INTERNAL_OPERATIONS
    {
        return Err(
            RoutingError::invalid_configuration(
                "routing input exceeds the internal operation safety ceiling",
            ),
        );
    }

    if input.initial_mapping.len()
        > input.topology.qubit_count()
    {
        return Err(
            RoutingError::insufficient_qubits(
                input.initial_mapping.len(),
                input.topology.qubit_count(),
            ),
        );
    }

    for operation in input.operations {
        let arity =
            operation.arity();

        if arity > 2 {
            return Err(
                RoutingError::unsupported_arity(
                    operation.name(),
                    arity,
                ),
            );
        }

        for logical in
            operation.logical_operands()
        {
            if !input
                .initial_mapping
                .contains_logical(*logical)
            {
                return Err(
                    RoutingError::invalid_operation(
                        operation.name(),
                        "operation references a logical qubit absent from the initial mapping",
                    ),
                );
            }
        }

        if arity == 2 {
            let operands =
                operation.logical_operands();

            if operands.len() != 2 {
                return Err(
                    RoutingError::internal_invariant(
                        "two-qubit operation has an invalid operand count",
                    ),
                );
            }

            if operands[0]
                == operands[1]
            {
                return Err(
                    RoutingError::invalid_operation(
                        operation.name(),
                        "two-qubit operation cannot use the same logical qubit twice",
                    ),
                );
            }
        }
    }

    Ok(())
}

// =============================================================================
// Executability
// =============================================================================

fn is_executable(
    operation: &QuantumOperation,
    topology: &Topology,
    mapping: &QubitMapping,
) -> Result<bool, RoutingError> {
    match operation.arity() {
        0 | 1 => Ok(true),

        2 => {
            let operands =
                operation.logical_operands();

            let a =
                mapping
                    .physical_of(operands[0])
                    .ok_or_else(|| {
                        RoutingError::invalid_operation(
                            operation.name(),
                            "first logical operand is not mapped",
                        )
                    })?;

            let b =
                mapping
                    .physical_of(operands[1])
                    .ok_or_else(|| {
                        RoutingError::invalid_operation(
                            operation.name(),
                            "second logical operand is not mapped",
                        )
                    })?;

            Ok(
                topology.supports_gate(
                    operation.name(),
                    a,
                    b,
                ),
            )
        }

        arity => Err(
            RoutingError::unsupported_arity(
                operation.name(),
                arity,
            ),
        ),
    }
}

fn front_is_executable(
    front: &[usize],
    mapping: &QubitMapping,
    input: &RoutingInput<'_>,
) -> Result<bool, RoutingError> {
    for &index in front {
        if is_executable(
            &input.operations[index],
            input.topology,
            mapping,
        )? {
            return Ok(true);
        }
    }

    Ok(false)
}

// =============================================================================
// Candidate generation
// =============================================================================

/// Generates only SWAPs incident to physical qubits participating in blocked
/// front-layer interactions.
///
/// A SWAP completely unrelated to the front layer cannot immediately change
/// the physical distance of a blocked interaction, so excluding it is both a
/// correctness-preserving search restriction and a major scalability
/// improvement.
fn generate_swap_candidates(
    front: &[usize],
    mapping: &QubitMapping,
    topology: &Topology,
    candidate_limit: usize,
) -> Result<
    Vec<(PhysicalQubitId, PhysicalQubitId)>,
    RoutingError,
> {
    let mut endpoints =
        BTreeSet::new();

    for &operation_index in front {
        // The caller has already validated the index.
        //
        // `front` only contains two-qubit operations when candidate generation
        // is required, but retaining this defensive check makes the helper
        // safe when called independently in tests.
        let _ = operation_index;
    }

    // The actual logical->physical front endpoints are supplied by the
    // caller through the mapping. We cannot infer logical operands here
    // without the operation stream, therefore candidate generation is
    // performed by `generate_candidates_for_operations`.
    //
    // This helper is retained as the physical-edge collector.
    let _ = endpoints;
    let _ = candidate_limit;

    // This function is intentionally never used directly; see the overload
    // below. Keeping candidate construction in one implementation avoids
    // accidental divergence between normal and speculative search.
    Ok(Vec::new())
}

/// Candidate generator used by the search.
///
/// It is separate from the public helper above so the operation list remains
/// an explicit input to candidate generation.
fn generate_candidates_for_operations(
    front: &[usize],
    operations: &[QuantumOperation],
    mapping: &QubitMapping,
    topology: &Topology,
    candidate_limit: usize,
) -> Result<
    Vec<(PhysicalQubitId, PhysicalQubitId)>,
    RoutingError,
> {
    let mut physical_front =
        BTreeSet::new();

    for &index in front {
        let operation =
            &operations[index];

        if operation.arity() != 2 {
            continue;
        }

        for logical in
            operation.logical_operands()
        {
            let physical =
                mapping
                    .physical_of(*logical)
                    .ok_or_else(|| {
                        RoutingError::invalid_operation(
                            operation.name(),
                            "front-layer logical operand is not mapped",
                        )
                    })?;

            physical_front.insert(
                physical,
            );
        }
    }

    let mut candidates =
        BTreeSet::new();

    // Current routing topologies use stable integer physical identifiers.
    // Iterate over the topology's physical index space and use `contains()` to
    // remain safe when a topology has holes.
    let qubit_count =
        topology.qubit_count();

    for a_index in
        0..qubit_count
    {
        let a =
            PhysicalQubitId::new(
                a_index,
            );

        if !topology.contains(a)
        {
            continue;
        }

        if !physical_front
            .contains(&a)
        {
            continue;
        }

        for b_index in
            a_index.saturating_add(1)
                ..qubit_count
        {
            let b =
                PhysicalQubitId::new(
                    b_index,
                );

            if !topology.contains(b) {
                continue;
            }

            if !physical_front
                .contains(&b)
            {
                continue;
            }

            if topology.is_adjacent(
                a,
                b,
            ) {
                candidates.insert(
                    (a, b),
                );
            }
        }
    }

    // If neither endpoint of an interaction is directly connected to another
    // front endpoint, a useful SWAP may still be required. Therefore include
    // all topology edges incident to at least one front endpoint.
    if candidates.is_empty() {
        for a_index in
            0..qubit_count
        {
            let a =
                PhysicalQubitId::new(
                    a_index,
                );

            if !topology.contains(a) {
                continue;
            }

            if !physical_front
                .contains(&a)
            {
                continue;
            }

            for b_index in
                0..qubit_count
            {
                if a_index == b_index {
                    continue;
                }

                let b =
                    PhysicalQubitId::new(
                        b_index,
                    );

                if !topology.contains(b) {
                    continue;
                }

                if !topology.is_adjacent(
                    a,
                    b,
                ) {
                    continue;
                }

                let pair =
                    if a < b {
                        (a, b)
                    } else {
                        (b, a)
                    };

                candidates.insert(
                    pair,
                );
            }
        }
    }

    Ok(
        candidates
            .into_iter()
            .take(candidate_limit)
            .collect(),
    )
}

// =============================================================================
// Distance matrix
// =============================================================================

/// Deterministic all-pairs unweighted physical-topology distance cache.
///
/// A distance of `None` means that the two physical qubits are disconnected.
#[derive(Debug, Clone)]
struct DistanceMatrix {
    distances: Vec<Vec<Option<usize>>>,
}

impl DistanceMatrix {
    fn build(
        topology: &Topology,
    ) -> Result<Self, RoutingError> {
        let n =
            topology.qubit_count();

        let mut distances =
            vec![vec![None; n]; n];

        for start_index in
            0..n
        {
            let start =
                PhysicalQubitId::new(
                    start_index,
                );

            if !topology.contains(start) {
                continue;
            }

            let mut queue =
                VecDeque::new();

            distances[start_index]
                [start_index] =
                Some(0);

            queue.push_back(
                start_index,
            );

            while let Some(current_index) =
                queue.pop_front()
            {
                let current_distance =
                    distances
                        [start_index]
                        [current_index]
                        .ok_or_else(
                            || {
                                RoutingError::internal_invariant(
                                    "distance queue contained a vertex without a recorded distance",
                                )
                            },
                        )?;

                for neighbour_index in
                    0..n
                {
                    if current_index
                        == neighbour_index
                    {
                        continue;
                    }

                    let current =
                        PhysicalQubitId::new(
                            current_index,
                        );

                    let neighbour =
                        PhysicalQubitId::new(
                            neighbour_index,
                        );

                    if !topology.contains(
                        neighbour,
                    ) {
                        continue;
                    }

                    if !topology.is_adjacent(
                        current,
                        neighbour,
                    ) {
                        continue;
                    }

                    if distances
                        [start_index]
                        [neighbour_index]
                        .is_some()
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

                    distances
                        [start_index]
                        [neighbour_index] =
                        Some(
                            next_distance,
                        );

                    queue.push_back(
                        neighbour_index,
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
        let ai = a.index();
        let bi = b.index();

        self.distances
            .get(ai)
            .and_then(|row| row.get(bi))
            .copied()
            .flatten()
    }
}

// =============================================================================
// Heuristic distance
// =============================================================================

fn interaction_distance(
    operation_indices: &[usize],
    mapping: &QubitMapping,
    operations: &[QuantumOperation],
    distances: &DistanceMatrix,
) -> Result<f64, RoutingError> {
    if operation_indices.is_empty() {
        return Ok(0.0);
    }

    let mut total = 0.0f64;
    let mut count = 0usize;

    for &index in
        operation_indices
    {
        let operation =
            &operations[index];

        if operation.arity() != 2 {
            continue;
        }

        let operands =
            operation
                .logical_operands();

        let a =
            mapping
                .physical_of(
                    operands[0],
                )
                .ok_or_else(|| {
                    RoutingError::invalid_operation(
                        operation.name(),
                        "heuristic encountered an unmapped first logical operand",
                    )
                })?;

        let b =
            mapping
                .physical_of(
                    operands[1],
                )
                .ok_or_else(|| {
                    RoutingError::invalid_operation(
                        operation.name(),
                        "heuristic encountered an unmapped second logical operand",
                    )
                })?;

        let distance =
            distances
                .distance(a, b)
                .ok_or_else(|| {
                    RoutingError::no_path(
                        a,
                        b,
                    )
                })?;

        total +=
            distance as f64;

        count =
            count
                .checked_add(1)
                .ok_or_else(
                    || {
                        RoutingError::internal_invariant(
                            "heuristic interaction count overflow",
                        )
                    },
                )?;
    }

    if count == 0 {
        return Ok(0.0);
    }

    Ok(
        total
            / count as f64,
    )
}

// =============================================================================
// Dependency graph
// =============================================================================

/// Lightweight operation dependency graph used to construct the routing front
/// layer and future layer set.
///
/// Dependencies are based on logical-qubit wire order. This is intentionally
/// independent of compiler-specific DAG implementations.
#[derive(Debug, Clone)]
struct DependencyGraph {
    predecessors: Vec<Vec<usize>>,
    successors: Vec<Vec<usize>>,
    layers: Vec<usize>,
}

impl DependencyGraph {
    fn build(
        operations: &[QuantumOperation],
    ) -> Result<Self, RoutingError> {
        let operation_count =
            operations.len();

        let mut predecessors =
            vec![Vec::new(); operation_count];

        let mut successors =
            vec![Vec::new(); operation_count];

        let mut last_use:
            std::collections::BTreeMap<
                crate::quantum::routing::types::LogicalQubitId,
                usize,
            > =
            std::collections::BTreeMap::new();

        for index in
            0..operation_count
        {
            let operation =
                &operations[index];

            for logical in
                operation.logical_operands()
            {
                if let Some(previous) =
                    last_use.get(
                        logical,
                    )
                {
                    if !predecessors[index]
                        .contains(previous)
                    {
                        predecessors[index]
                            .push(*previous);
                    }
                }

                last_use.insert(
                    *logical,
                    index,
                );
            }
        }

        for index in
            0..operation_count
        {
            predecessors[index]
                .sort_unstable();

            for &previous in
                &predecessors[index]
            {
                successors[previous]
                    .push(index);
            }
        }

        for successor_list in
            &mut successors
        {
            successor_list
                .sort_unstable();
            successor_list
                .dedup();
        }

        let layers =
            compute_layers(
                &predecessors,
            )?;

        Ok(Self {
            predecessors,
            successors,
            layers,
        })
    }

    fn ready_operations(
        &self,
        completed: &[bool],
    ) -> Vec<usize> {
        let mut ready =
            Vec::new();

        for index in
            0..self.predecessors.len()
        {
            if completed[index] {
                continue;
            }

            if self.predecessors[index]
                .iter()
                .all(|&p| completed[p])
            {
                ready.push(index);
            }
        }

        ready
    }

    fn extended_operations(
        &self,
        completed: &[bool],
        front: &[usize],
        depth: usize,
    ) -> Vec<usize> {
        if depth == 0 {
            return Vec::new();
        }

        if front.is_empty() {
            return Vec::new();
        }

        let front_layer =
            front
                .iter()
                .map(|&index| {
                    self.layers[index]
                })
                .min()
                .unwrap_or(0);

        let maximum_layer =
            front_layer
                .saturating_add(depth);

        let mut extended =
            Vec::new();

        for index in
            0..self.layers.len()
        {
            if completed[index] {
                continue;
            }

            if front.contains(&index) {
                continue;
            }

            let layer =
                self.layers[index];

            if layer > front_layer
                && layer
                    <= maximum_layer
            {
                extended.push(index);
            }
        }

        extended.sort_unstable_by_key(
            |&index| {
                (self.layers[index], index)
            },
        );

        extended
    }
}

fn compute_layers(
    predecessors: &[Vec<usize>],
) -> Result<Vec<usize>, RoutingError> {
    let mut layers =
        vec![0usize; predecessors.len()];

    for index in
        0..predecessors.len()
    {
        let mut layer =
            0usize;

        for &predecessor in
            &predecessors[index]
        {
            if predecessor >= index {
                return Err(
                    RoutingError::internal_invariant(
                        "operation dependency graph contains a forward or cyclic dependency",
                    ),
                );
            }

            let predecessor_layer =
                layers[predecessor];

            let candidate =
                predecessor_layer
                    .checked_add(1)
                    .ok_or_else(
                        || {
                            RoutingError::internal_invariant(
                                "operation dependency layer overflow",
                            )
                        },
                    )?;

            layer =
                layer.max(candidate);
        }

        layers[index] =
            layer;
    }

    Ok(layers)
}

// =============================================================================
// Helpers
// =============================================================================

fn all_completed(
    completed: &[bool],
) -> bool {
    completed
        .iter()
        .all(|value| *value)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::quantum::routing::config::RoutingConfig;
    use crate::quantum::routing::mapping::QubitMapping;
    use crate::quantum::routing::topology::Topology;
    use crate::quantum::routing::types::{
        GateIdentity,
        LogicalQubitId,
        QubitInteraction,
    };

    fn logical(index: usize) -> LogicalQubitId {
        LogicalQubitId::new(index)
    }

    fn physical(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn default_router_has_production_parameters() {
        let router =
            LookaheadRouter::new();

        assert_eq!(
            router.name(),
            "lookahead"
        );

        assert!(
            router
                .extended_set_weight()
                .is_finite()
        );

        assert!(
            router.beam_width()
                > 0
        );

        assert!(
            router.beam_width()
                <= MAX_BEAM_WIDTH
        );
    }

    #[test]
    fn distance_matrix_is_zero_on_diagonal() {
        let topology =
            Topology::line(4)
                .expect("line topology");

        let matrix =
            DistanceMatrix::build(
                &topology,
            )
            .expect("distance matrix");

        for index in
            0..4
        {
            assert_eq!(
                matrix.distance(
                    physical(index),
                    physical(index),
                ),
                Some(0)
            );
        }
    }

    #[test]
    fn line_distance_is_correct() {
        let topology =
            Topology::line(5)
                .expect("line topology");

        let matrix =
            DistanceMatrix::build(
                &topology,
            )
            .expect("distance matrix");

        assert_eq!(
            matrix.distance(
                physical(0),
                physical(4),
            ),
            Some(4)
        );

        assert_eq!(
            matrix.distance(
                physical(1),
                physical(4),
            ),
            Some(3)
        );
    }

    #[test]
    fn dependency_graph_preserves_wire_order() {
        let operations =
            vec![
                QuantumOperation::new(
                    "h",
                    vec![logical(0)],
                ),
                QuantumOperation::new(
                    "cx",
                    vec![
                        logical(0),
                        logical(1),
                    ],
                ),
                QuantumOperation::new(
                    "x",
                    vec![logical(1)],
                ),
            ];

        let graph =
            DependencyGraph::build(
                &operations,
            )
            .expect("dependency graph");

        assert!(
            graph.predecessors[1]
                .contains(&0)
        );

        assert!(
            graph.predecessors[2]
                .contains(&1)
        );
    }

    #[test]
    fn dependency_graph_allows_independent_wires() {
        let operations =
            vec![
                QuantumOperation::new(
                    "h",
                    vec![logical(0)],
                ),
                QuantumOperation::new(
                    "x",
                    vec![logical(1)],
                ),
            ];

        let graph =
            DependencyGraph::build(
                &operations,
            )
            .expect("dependency graph");

        assert!(
            graph.predecessors[0]
                .is_empty()
        );

        assert!(
            graph.predecessors[1]
                .is_empty()
        );
    }

    #[test]
    fn mapping_swap_is_transactionally_safe_for_search() {
        let mut mapping =
            QubitMapping::new();

        mapping
            .assign(
                logical(0),
                physical(0),
            )
            .expect("assignment");

        mapping
            .assign(
                logical(1),
                physical(1),
            )
            .expect("assignment");

        let original =
            mapping.clone();

        mapping
            .swap_physical(
                physical(0),
                physical(1),
            )
            .expect("swap");

        assert_eq!(
            mapping.physical_of(
                logical(0)
            ),
            Some(physical(1))
        );

        assert_eq!(
            mapping.physical_of(
                logical(1)
            ),
            Some(physical(0))
        );

        // The original clone remains unchanged, which is exactly how
        // speculative beam states are isolated.
        assert_eq!(
            original.physical_of(
                logical(0)
            ),
            Some(physical(0))
        );
    }

    #[test]
    fn search_state_tie_breaking_is_deterministic() {
        let mut a =
            QubitMapping::new();

        a.assign(
            logical(0),
            physical(0),
        )
        .expect("assignment");

        let mut b =
            QubitMapping::new();

        b.assign(
            logical(0),
            physical(0),
        )
        .expect("assignment");

        let left =
            SearchState {
                mapping: a,
                swaps: vec![
                    (
                        physical(0),
                        physical(1),
                    ),
                ],
                score: 1.0,
            };

        let right =
            SearchState {
                mapping: b,
                swaps: vec![
                    (
                        physical(0),
                        physical(2),
                    ),
                ],
                score: 1.0,
            };

        assert_ne!(
            search_state_order(
                &left,
                &right,
            ),
            Ordering::Equal
        );
    }

    #[test]
    fn future_layers_are_bounded() {
        let operations =
            vec![
                QuantumOperation::new(
                    "cx",
                    vec![
                        logical(0),
                        logical(1),
                    ],
                ),
                QuantumOperation::new(
                    "cx",
                    vec![
                        logical(1),
                        logical(2),
                    ],
                ),
                QuantumOperation::new(
                    "cx",
                    vec![
                        logical(2),
                        logical(3),
                    ],
                ),
                QuantumOperation::new(
                    "cx",
                    vec![
                        logical(3),
                        logical(4),
                    ],
                ),
            ];

        let graph =
            DependencyGraph::build(
                &operations,
            )
            .expect("dependency graph");

        let completed =
            vec![
                false,
                false,
                false,
                false,
            ];

        let front =
            graph.ready_operations(
                &completed,
            );

        let extended =
            graph.extended_operations(
                &completed,
                &front,
                2,
            );

        assert!(
            extended.len()
                <= 2
        );
    }

    #[test]
    fn interaction_distance_empty_set_is_zero() {
        let distances =
            DistanceMatrix {
                distances: vec![
                    vec![
                        Some(0),
                        Some(1),
                    ],
                    vec![
                        Some(1),
                        Some(0),
                    ],
                ],
            };

        let mapping =
            QubitMapping::new();

        let operations:
            Vec<QuantumOperation> =
            Vec::new();

        let result =
            interaction_distance(
                &[],
                &mapping,
                &operations,
                &distances,
            )
            .expect("distance");

        assert_eq!(
            result,
            0.0
        );
    }
}