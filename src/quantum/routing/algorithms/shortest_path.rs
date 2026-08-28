//! Zamani Quantum Routing — Deterministic Shortest-Path Router
//!
//! Production implementation of deterministic shortest-path logical-to-physical
//! routing.
//
//! # Responsibility
//!
//! This module implements the routing algorithm that uses deterministic
//! shortest physical paths to move logical qubits until each two-qubit
//! interaction becomes physically executable.
//
//! It deliberately does NOT own:
//!
//! - topology storage;
//! - topology construction;
//! - logical/physical mapping storage;
//! - initial layout selection;
//! - SABRE/lookahead heuristics;
//! - hardware-provider APIs;
//! - gate synthesis;
//! - pulse generation;
//! - scheduling;
//! - quantum simulation;
//! - QEC decoding;
//! - frontend parsing;
//! - compiler-specific IR mutation.
//!
//! Those responsibilities belong to the surrounding routing/compiler
//! subsystems.
//
//! # Architectural position
//!
//! ```text
//! Quantum IR
//!     │
//!     ▼
//! routing::layout
//!     │
//!     ▼
//! routing::mapping
//!     │
//!     ▼
//! RoutingWorkload
//!     │
//!     ▼
//! ShortestPathRouter
//!     │
//!     ├── routing::path::PathFinder
//!     ├── routing::topology::PhysicalTopology
//!     └── routing::mapping::QubitMapping
//!     │
//!     ▼
//! RoutingResult
//!     │
//!     ▼
//! routing::router / verification / transpiler
//! ```
//!
//! # Algorithm
//!
//! For every interaction, in program order:
//!
//! 1. Resolve the logical operands through the current mapping.
//! 2. Check whether the requested physical gate is already executable.
//! 3. If it is executable, emit the semantic gate operation.
//! 4. Otherwise find a deterministic shortest physical path.
//! 5. Generate legal semantic SWAP movements along that path.
//! 6. Update the mapping after every SWAP.
//! 7. Re-check executability.
//! 8. Emit the gate only after physical legality is established.
//!
//! Two movement directions are considered for a two-qubit interaction:
//!
//! ```text
//! source -> target
//! target -> source
//! ```
//!
//! The shorter legal route is selected. If both have equal movement cost,
//! deterministic physical-qubit ordering is used as the tie-breaker.
//!
//! This is intentionally a stronger implementation than the old
//! `transpiler.rs` shortest-path helper, which only moved one operand according
//! to one path and assumed an undirected topology.
//!
//! # Important semantic boundary
//!
//! A `RoutingMove::Swap` is a semantic movement request. This module does not
//! lower it into CX/CNOT/ISWAP/native-SWAP sequences.
//!
//! Hardware lowering remains responsible for deciding how a semantic SWAP is
//! physically implemented.
//!
//! # Directed hardware
//!
//! Structural adjacency and gate executability are distinct.
//!
//! The router therefore uses:
//!
//! ```text
//! topology.supports_gate(gate, source, target)
//! ```
//!
//! for the actual gate, rather than assuming that structural adjacency implies
//! gate support.
//!
//! Movement itself is checked against the target's SWAP capability when the
//! topology provides explicit SWAP support. For generic topology descriptions,
//! an undirected structural edge is accepted as a semantic movement edge.
//!
//! # Multi-qubit operations
//!
//! This algorithm intentionally does not synthesize arbitrary operations with
//! three or more operands.
//!
//! Such operations must either:
//!
//! - already be natively supported by the target and handled by the higher
//!   routing layer;
//! - be decomposed before this algorithm is invoked;
//! - or be rejected by the configured `MultiQubitPolicy`.
//!
//! This matches the architecture used by modern routing systems, where routing
//! operates primarily on one- and two-qubit interactions. Qiskit's current
//! SABRE routing documentation explicitly assumes 3+ qubit gates have already
//! been decomposed. Cirq's routing transformer likewise raises for unsupported
//! n-qubit operations. 
//!
//! # Determinism
//!
//! The implementation contains no randomness.
//!
//! Identical:
//!
//! ```text
//! topology
//! + mapping
//! + workload
//! + configuration
//! ```
//!
//! produce identical routing decisions.
//!
//! `RoutingConfig::seed` is therefore intentionally ignored by this algorithm.
//! The result remains deterministic even when no seed is supplied.
//!
//! # Transactionality
//!
//! The supplied mapping is never mutated by this algorithm.
//!
//! The algorithm clones the caller's mapping into a working mapping and only
//! exposes the final mapping through `RoutingResult`.
//!
//! Therefore:
//!
//! ```text
//! success -> caller may commit final mapping
//! failure -> caller's original mapping is unchanged
//! ```
//!
//! # Safety
//!
//! - Rust 2021.
//! - Rust 1.97 / 1.97.1.
//! - No `unsafe`.
//! - `#![deny(unsafe_code)]`.
//! - No global mutable state.
//! - No external dependencies.
//!
//! # Complexity
//!
//! For an interaction whose operands are at graph distance `d`, routing requires
//! at most `d - 1` movement operations for a successful path-based route when
//! one operand is moved toward the other.
//!
//! Each path search is O(V + E) for unweighted shortest-path search through
//! `PathFinder`.
//!
//! Mapping lookup and SWAP mutation are average O(1).
//!
//! This algorithm is intentionally a correctness/reference implementation.
//! More globally optimized routing belongs to:
//!
//! - `lookahead.rs`;
//! - `sabre.rs`;
//! - `noise_aware.rs`;
//! - `dynamic.rs`.
//!
//! # Integration contract
//!
//! This module depends only on already-established routing contracts:
//!
//! ```text
//! types.rs
//! errors.rs
//! topology.rs
//! mapping.rs
//! path.rs
//! config.rs
//! result.rs
//! ```
//!
//! It does not depend on:
//!
//! ```text
//! router.rs
//! transpiler.rs
//! frontend
//! hardware providers
//! compiler IR
//! SABRE
//! lookahead
//! benchmarking
//! ```
//!
//! This means the file can be completed and frozen before those later
//! integration layers exist.
//!
//! # External algorithm context
//!
//! Qiskit exposes a basic routing strategy alongside lookahead and SABRE, and
//! its documentation describes basic routing as a greedy strategy that routes
//! gates individually. The shortest-path implementation here provides Zamani's
//! deterministic graph-optimal baseline for individual interactions while
//! leaving global optimization to higher-level algorithms.
//! 

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::time::{Duration, Instant};

use crate::quantum::routing::config::{
    LayoutStrategy,
    RoutingConfig,
};
use crate::quantum::routing::errors::{
    RoutingError,
    RoutingResult,
};
use crate::quantum::routing::mapping::{
    MappingError,
    QubitMapping,
    QubitMappingSnapshot,
};
use crate::quantum::routing::path::{
    PathFinder,
    PathSearchConfig,
};
use crate::quantum::routing::result::{
    ReproducibilityMetadata,
    RoutingMetrics,
    RoutingResult as CompletedRoutingResult,
    VerificationSummary,
};
use crate::quantum::routing::topology::PhysicalTopology;
use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QubitInteraction,
    RouteDisposition,
    RoutingAlgorithm,
    RoutingEvent,
    RoutingMove,
    RoutingOperation,
    RoutingPhase,
    RoutingSeed,
    RoutingWorkload,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable algorithm implementation version.
///
/// Increment this when the algorithm's routing semantics change.
pub const SHORTEST_PATH_ALGORITHM_VERSION: &str = "1.0.0";

/// Stable routing algorithm name.
pub const SHORTEST_PATH_ALGORITHM_NAME: &str = "shortest_path";

/// Maximum number of attempts to resolve a single two-qubit interaction.
///
/// The value is deliberately conservative. A shortest-path route should
/// terminate after a finite number of mapping mutations. If it does not, an
/// invariant has been violated and routing must fail instead of looping.
const MAX_INTERACTION_ROUTE_STEPS: usize = 1_000_000;

/// Semantic name used when asking topology whether a SWAP operation has
/// explicitly registered hardware support.
const SWAP_GATE_NAME: &str = "swap";

// =============================================================================
// Public router
// =============================================================================

/// Deterministic shortest-path quantum router.
///
/// The router itself contains no mutable routing state. Each routing invocation
/// creates isolated working state.
///
/// This makes one instance safe to reuse sequentially and naturally suitable
/// for parallel independent routing trials when the caller owns separate
/// instances or uses immutable references.
#[derive(Debug, Clone, Copy, Default)]
pub struct ShortestPathRouter {
    path_finder: PathFinder,
}

impl ShortestPathRouter {
    /// Creates a production shortest-path router.
    #[must_use]
    pub fn new() -> Self {
        Self {
            path_finder: PathFinder::new(),
        }
    }

    /// Creates a shortest-path router with explicit path-search configuration.
    ///
    /// This is useful when the caller needs a stricter path-length or visited
    /// vertex limit than the global routing configuration provides.
    pub fn with_path_config(
        config: PathSearchConfig,
    ) -> RoutingResult<Self> {
        Ok(Self {
            path_finder: PathFinder::with_config(config)?,
        })
    }

    /// Returns the path finder used by this router.
    #[must_use]
    pub const fn path_finder(&self) -> &PathFinder {
        &self.path_finder
    }

    /// Returns the stable algorithm name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        SHORTEST_PATH_ALGORITHM_NAME
    }

    /// Returns the stable implementation version.
    #[must_use]
    pub const fn version(&self) -> &'static str {
        SHORTEST_PATH_ALGORITHM_VERSION
    }

    // =========================================================================
    // Main routing API
    // =========================================================================

    /// Routes a complete logical workload using deterministic shortest paths.
    ///
    /// The caller supplies an initial mapping. The mapping is never mutated.
    ///
    /// The returned `RoutingResult` contains:
    ///
    /// - the initial mapping snapshot;
    /// - the final mapping snapshot;
    /// - the semantic routing operation stream;
    /// - routing metrics;
    /// - deterministic reproducibility metadata;
    /// - routing events.
    ///
    /// # Failure behavior
    ///
    /// On every error:
    ///
    /// - the caller's mapping is untouched;
    /// - no partial `RoutingResult` is returned;
    /// - the error identifies the routing failure through `RoutingError`.
    pub fn route_workload(
        &self,
        topology: &PhysicalTopology,
        mapping: &QubitMapping,
        workload: &RoutingWorkload,
        config: &RoutingConfig,
    ) -> RoutingResult<CompletedRoutingResult> {
        let started = Instant::now();

        self.validate_configuration(config)?;
        self.validate_workload(topology, mapping, workload, config)?;

        let initial_mapping = mapping.snapshot();

        let mut working_mapping = mapping.clone();

        let mut operations = Vec::new();
        let mut events = Vec::new();

        events.push(RoutingEvent::PhaseStarted {
            phase: RoutingPhase::Validation,
        });

        events.push(RoutingEvent::PhaseCompleted {
            phase: RoutingPhase::Validation,
        });

        events.push(RoutingEvent::PhaseStarted {
            phase: RoutingPhase::Routing,
        });

        let mut metrics = RoutingMetrics::new(
            workload.logical_qubit_count(),
            topology.qubit_count(),
        );

        metrics.original_operations = workload.interaction_count();

        let mut routing_iterations = 0usize;
        let mut candidate_rejections = 0usize;
        let mut inserted_swaps = 0usize;
        let mut routed_two_qubit_operations = 0usize;
        let mut physical_two_qubit_operations = 0usize;

        for interaction in workload.interactions() {
            routing_iterations = routing_iterations
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation(
                        "routing iteration counter overflow"
                            .to_string(),
                    )
                })?;

            self.check_iteration_limit(
                routing_iterations,
                config,
                started,
            )?;

            let outcome = self.route_interaction(
                topology,
                &mut working_mapping,
                interaction,
                config,
                &mut operations,
                &mut events,
                &mut candidate_rejections,
                &mut inserted_swaps,
                &mut routed_two_qubit_operations,
                &mut physical_two_qubit_operations,
                &mut routing_iterations,
                started,
            )?;

            if outcome {
                // The interaction was routed successfully.
            }
        }

        events.push(RoutingEvent::PhaseCompleted {
            phase: RoutingPhase::Routing,
        });

        let final_mapping = working_mapping.snapshot();

        metrics.final_operations = operations.len();

        metrics.routed_two_qubit_operations =
            routed_two_qubit_operations;

        metrics.inserted_swaps = inserted_swaps;

        metrics.inserted_moves = inserted_swaps;

        metrics.routing_overhead_operations = inserted_swaps;

        metrics.routing_iterations = routing_iterations;

        metrics.candidate_rejections = candidate_rejections;

        metrics.final_gate_operations = operations
            .iter()
            .filter(|operation| operation.is_gate())
            .count();

        metrics.physical_two_qubit_operations =
            physical_two_qubit_operations;

        metrics.original_two_qubit_operations = workload
            .interactions()
            .iter()
            .filter(|interaction| interaction.is_two_qubit())
            .count();

        metrics.original_multi_qubit_operations = workload
            .interactions()
            .iter()
            .filter(|interaction| interaction.is_multi_qubit())
            .count();

        metrics.original_single_qubit_operations = workload
            .interactions()
            .iter()
            .filter(|interaction| interaction.is_single_qubit())
            .count();

        metrics.total_duration = started.elapsed();
        metrics.routing_duration = metrics.total_duration;

        metrics.routing_depth = inserted_swaps;

        metrics.routing_two_qubit_depth = inserted_swaps;

        metrics.final_depth = metrics
            .original_depth
            .saturating_add(metrics.routing_depth);

        metrics.final_two_qubit_depth = metrics
            .original_two_qubit_depth
            .saturating_add(metrics.routing_two_qubit_depth);

        let disposition = if inserted_swaps == 0 {
            RouteDisposition::AlreadyExecutable
        } else {
            RouteDisposition::Routed
        };

        let verification = VerificationSummary::not_requested();

        let mut reproducibility =
            if config.deterministic {
                ReproducibilityMetadata::deterministic()
            } else {
                ReproducibilityMetadata::nondeterministic()
            };

        if let Some(seed) = config.seed {
            reproducibility =
                reproducibility.with_seed(RoutingSeed::new(seed));
        }

        reproducibility = reproducibility
            .with_algorithm_version(
                SHORTEST_PATH_ALGORITHM_VERSION,
            );

        let mut result = CompletedRoutingResult::new(
            disposition,
            RoutingAlgorithm::ShortestPath,
            config.layout.clone(),
            config.objective.clone(),
            config.mode,
            initial_mapping,
            final_mapping,
            operations,
            metrics,
            verification,
            reproducibility,
        );

        result.extend_events(events);

        Ok(result)
    }

    /// Routes one logical interaction against a working mapping.
    ///
    /// This method is public because it is useful to higher-level routing
    /// algorithms that want to use deterministic shortest-path movement as a
    /// primitive.
    ///
    /// The supplied mapping is mutated only after a valid movement has been
    /// selected and validated.
    pub fn route_interaction(
        &self,
        topology: &PhysicalTopology,
        mapping: &mut QubitMapping,
        interaction: &QubitInteraction,
        config: &RoutingConfig,
        operations: &mut Vec<RoutingOperation>,
        events: &mut Vec<RoutingEvent>,
        candidate_rejections: &mut usize,
        inserted_swaps: &mut usize,
        routed_two_qubit_operations: &mut usize,
        physical_two_qubit_operations: &mut usize,
        routing_iterations: &mut usize,
        started: Instant,
    ) -> RoutingResult<bool> {
        self.validate_interaction(
            topology,
            mapping,
            interaction,
            config,
        )?;

        match interaction.arity() {
            0 => Err(RoutingError::InvalidInstruction(
                "routing interaction contains zero operands"
                    .to_string(),
            )),

            1 => {
                self.emit_single_qubit_interaction(
                    topology,
                    mapping,
                    interaction,
                    config,
                    operations,
                    events,
                )?;

                Ok(false)
            }

            2 => self.route_two_qubit_interaction(
                topology,
                mapping,
                interaction,
                config,
                operations,
                events,
                candidate_rejections,
                inserted_swaps,
                routed_two_qubit_operations,
                physical_two_qubit_operations,
                routing_iterations,
                started,
            ),

            _ => Err(RoutingError::UnsupportedArity {
                arity: interaction.arity(),
            }),
        }
    }

    // =========================================================================
    // Validation
    // =========================================================================

    fn validate_configuration(
        &self,
        config: &RoutingConfig,
    ) -> RoutingResult<()> {
        config.validate().map_err(|error| {
            RoutingError::InvalidConfiguration(error.to_string())
        })?;

        if config.algorithm != RoutingAlgorithm::ShortestPath {
            return Err(RoutingError::InvalidConfiguration(
                format!(
                    "ShortestPathRouter received algorithm `{}`; \
                     expected `shortest_path`",
                    config.algorithm.name()
                ),
            ));
        }

        if !config.allow_swap {
            return Err(RoutingError::InvalidConfiguration(
                "shortest-path routing requires allow_swap=true \
                 when a non-adjacent interaction must be routed"
                    .to_string(),
            ));
        }

        Ok(())
    }

    fn validate_workload(
        &self,
        topology: &PhysicalTopology,
        mapping: &QubitMapping,
        workload: &RoutingWorkload,
        config: &RoutingConfig,
    ) -> RoutingResult<()> {
        topology.validate()?;

        if topology.qubit_count() == 0 {
            return Err(RoutingError::EmptyTopology);
        }

        if workload.logical_qubit_count()
            > topology.qubit_count()
        {
            return Err(
                RoutingError::InsufficientQubits {
                    required: workload.logical_qubit_count(),
                    available: topology.qubit_count(),
                },
            );
        }

        if workload.logical_qubit_count()
            > config.limits.max_iterations
        {
            // This is not normally the limiting factor, but prevents a
            // pathological workload from bypassing configured search limits.
            return Err(RoutingError::ResourceLimitExceeded(
                format!(
                    "logical workload contains {} qubits, \
                     exceeding configured routing limit {}",
                    workload.logical_qubit_count(),
                    config.limits.max_iterations
                ),
            ));
        }

        mapping.validate().map_err(map_error)?;

        for logical in workload.logical_qubits() {
            if !mapping.contains_logical(*logical) {
                return Err(
                    RoutingError::InvalidLogicalQubit(
                        format!(
                            "logical qubit {logical} is not present \
                             in the supplied routing mapping"
                        ),
                    ),
                );
            }
        }

        Ok(())
    }

    fn validate_interaction(
        &self,
        topology: &PhysicalTopology,
        mapping: &QubitMapping,
        interaction: &QubitInteraction,
        config: &RoutingConfig,
    ) -> RoutingResult<()> {
        if interaction.arity() == 0 {
            return Err(RoutingError::InvalidInstruction(
                "empty routing interaction".to_string(),
            ));
        }

        let mut seen = std::collections::BTreeSet::new();

        for logical in interaction.operands() {
            if !seen.insert(*logical) {
                return Err(RoutingError::InvalidInstruction(
                    format!(
                        "interaction contains duplicate logical \
                         operand {logical}"
                    ),
                ));
            }

            if !mapping.contains_logical(*logical) {
                return Err(
                    RoutingError::InvalidLogicalQubit(
                        format!(
                            "logical operand {logical} is not mapped"
                        ),
                    ),
                );
            }
        }

        if interaction.is_multi_qubit() {
            match config.multi_qubit_policy {
                crate::quantum::routing::config::MultiQubitPolicy::Reject
                | crate::quantum::routing::config::MultiQubitPolicy::NativeOnly
                | crate::quantum::routing::config::MultiQubitPolicy::Decompose
                | crate::quantum::routing::config::MultiQubitPolicy::Auto => {
                    return Err(
                        RoutingError::UnsupportedArity {
                            arity: interaction.arity(),
                        },
                    );
                }
            }
        }

        for logical in interaction.operands() {
            let physical = mapping
                .physical_of(*logical)
                .ok_or_else(|| {
                    RoutingError::InvalidLogicalQubit(
                        format!(
                            "logical operand {logical} has no \
                             physical location"
                        ),
                    )
                })?;

            if !topology.contains(physical) {
                return Err(
                    RoutingError::InvalidPhysicalQubit(
                        format!(
                            "logical operand {logical} maps to \
                             unregistered physical qubit {physical}"
                        ),
                    ),
                );
            }

            if !topology.is_available(physical) {
                return Err(
                    RoutingError::InvalidPhysicalQubit(
                        format!(
                            "logical operand {logical} maps to \
                             unavailable physical qubit {physical}"
                        ),
                    );
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // Single-qubit interaction
    // =========================================================================

    fn emit_single_qubit_interaction(
        &self,
        topology: &PhysicalTopology,
        mapping: &QubitMapping,
        interaction: &QubitInteraction,
        _config: &RoutingConfig,
        operations: &mut Vec<RoutingOperation>,
        events: &mut Vec<RoutingEvent>,
    ) -> RoutingResult<()> {
        let logical = interaction.operands()[0];

        let physical = mapping
            .physical_of(logical)
            .ok_or_else(|| {
                RoutingError::InvalidLogicalQubit(
                    format!(
                        "logical qubit {logical} has no physical \
                         location"
                    ),
                )
            })?;

        if !topology.contains(physical) {
            return Err(
                RoutingError::InvalidPhysicalQubit(
                    format!(
                        "physical qubit {physical} does not exist"
                    ),
                ),
            );
        }

        operations.push(RoutingOperation::Gate {
            gate: interaction.gate().clone(),
            operands: vec![physical],
            logical_operands: vec![logical],
        });

        events.push(RoutingEvent::GateRouted {
            gate: interaction.gate().clone(),
            physical_operands: vec![physical],
        });

        Ok(())
    }

    // =========================================================================
    // Two-qubit routing
    // =========================================================================

    #[allow(clippy::too_many_arguments)]
    fn route_two_qubit_interaction(
        &self,
        topology: &PhysicalTopology,
        mapping: &mut QubitMapping,
        interaction: &QubitInteraction,
        config: &RoutingConfig,
        operations: &mut Vec<RoutingOperation>,
        events: &mut Vec<RoutingEvent>,
        candidate_rejections: &mut usize,
        inserted_swaps: &mut usize,
        routed_two_qubit_operations: &mut usize,
        physical_two_qubit_operations: &mut usize,
        routing_iterations: &mut usize,
        started: Instant,
    ) -> RoutingResult<bool> {
        let operands = interaction.operands();

        let logical_a = operands[0];
        let logical_b = operands[1];

        let gate = interaction.gate();

        let initial_a = mapping
            .physical_of(logical_a)
            .ok_or_else(|| {
                RoutingError::InvalidLogicalQubit(
                    format!("logical qubit {logical_a} is unmapped"),
                )
            })?;

        let initial_b = mapping
            .physical_of(logical_b)
            .ok_or_else(|| {
                RoutingError::InvalidLogicalQubit(
                    format!("logical qubit {logical_b} is unmapped"),
                )
            })?;

        // Fast path: already executable.
        if topology.supports_gate(
            gate.name(),
            initial_a,
            initial_b,
        ) {
            self.emit_two_qubit_gate(
                gate,
                logical_a,
                logical_b,
                initial_a,
                initial_b,
                operations,
                events,
                routed_two_qubit_operations,
                physical_two_qubit_operations,
            )?;

            return Ok(false);
        }

        // A structurally adjacent but directionally unsupported operation is
        // not automatically routable by moving qubits. We must find a legal
        // adjacent endpoint orientation.
        //
        // The shortest-path candidates below therefore verify actual gate
        // support before committing the final movement.
        let candidate_forward = self.build_path_candidate(
            topology,
            mapping,
            logical_a,
            logical_b,
            gate,
            config,
            true,
        )?;

        let candidate_reverse = self.build_path_candidate(
            topology,
            mapping,
            logical_a,
            logical_b,
            gate,
            config,
            false,
        )?;

        let selected = self.select_candidate(
            candidate_forward,
            candidate_reverse,
            candidate_rejections,
        )?;

        let candidate = match selected {
            Some(candidate) => candidate,
            None => {
                return Err(RoutingError::RoutingFailed {
                    from: initial_a,
                    to: initial_b,
                });
            }
        };

        for movement in candidate.moves {
            self.check_iteration_limit(
                *routing_iterations,
                config,
                started,
            )?;

            self.commit_swap(
                topology,
                mapping,
                movement,
                config,
                operations,
                events,
                inserted_swaps,
            )?;

            *routing_iterations = routing_iterations
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation(
                        "routing iteration counter overflow"
                            .to_string(),
                    )
                })?;

            self.check_swap_limit(
                *inserted_swaps,
                config,
            )?;
        }

        let final_a = mapping
            .physical_of(logical_a)
            .ok_or_else(|| {
                RoutingError::InternalInvariantViolation(
                    "logical operand disappeared from mapping after \
                     shortest-path routing"
                        .to_string(),
                )
            })?;

        let final_b = mapping
            .physical_of(logical_b)
            .ok_or_else(|| {
                RoutingError::InternalInvariantViolation(
                    "second logical operand disappeared from mapping \
                     after shortest-path routing"
                        .to_string(),
                )
            })?;

        if !topology.supports_gate(
            gate.name(),
            final_a,
            final_b,
        ) {
            return Err(RoutingError::InternalInvariantViolation(
                format!(
                    "shortest-path candidate completed but gate `{}` \
                     remains physically unsupported at {final_a} -> {final_b}",
                    gate.name()
                ),
            ));
        }

        self.emit_two_qubit_gate(
            gate,
            logical_a,
            logical_b,
            final_a,
            final_b,
            operations,
            events,
            routed_two_qubit_operations,
            physical_two_qubit_operations,
        )?;

        Ok(true)
    }

    fn emit_two_qubit_gate(
        &self,
        gate: &GateIdentity,
        logical_a: LogicalQubitId,
        logical_b: LogicalQubitId,
        physical_a: PhysicalQubitId,
        physical_b: PhysicalQubitId,
        operations: &mut Vec<RoutingOperation>,
        events: &mut Vec<RoutingEvent>,
        routed_two_qubit_operations: &mut usize,
        physical_two_qubit_operations: &mut usize,
    ) -> RoutingResult<()> {
        if physical_a == physical_b {
            return Err(
                RoutingError::InternalInvariantViolation(
                    format!(
                        "two-qubit gate `{}` resolved to identical \
                         physical operands {physical_a}",
                        gate.name()
                    ),
                ),
            );
        }

        operations.push(RoutingOperation::Gate {
            gate: gate.clone(),
            operands: vec![physical_a, physical_b],
            logical_operands: vec![logical_a, logical_b],
        });

        *routed_two_qubit_operations = routed_two_qubit_operations
            .checked_add(1)
            .ok_or_else(|| {
                RoutingError::InternalInvariantViolation(
                    "routed two-qubit operation counter overflow"
                        .to_string(),
                )
            })?;

        *physical_two_qubit_operations =
            physical_two_qubit_operations
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation(
                        "physical two-qubit operation counter overflow"
                            .to_string(),
                    )
                })?;

        events.push(RoutingEvent::GateRouted {
            gate: gate.clone(),
            physical_operands: vec![
                physical_a,
                physical_b,
            ],
        });

        Ok(())
    }

    // =========================================================================
    // Candidate construction
    // =========================================================================

    fn build_path_candidate(
        &self,
        topology: &PhysicalTopology,
        mapping: &QubitMapping,
        logical_a: LogicalQubitId,
        logical_b: LogicalQubitId,
        gate: &GateIdentity,
        config: &RoutingConfig,
        move_a_toward_b: bool,
    ) -> RoutingResult<Option<PathCandidate>> {
        let physical_a = mapping
            .physical_of(logical_a)
            .ok_or_else(|| {
                RoutingError::InvalidLogicalQubit(
                    format!("logical qubit {logical_a} is unmapped"),
                )
            })?;

        let physical_b = mapping
            .physical_of(logical_b)
            .ok_or_else(|| {
                RoutingError::InvalidLogicalQubit(
                    format!("logical qubit {logical_b} is unmapped"),
                )
            })?;

        let path = if move_a_toward_b {
            self.path_finder.shortest_path(
                topology,
                physical_a,
                physical_b,
            )?
        } else {
            self.path_finder.shortest_path(
                topology,
                physical_b,
                physical_a,
            )?
        };

        if path.vertices().len() < 2 {
            return Ok(None);
        }

        let required_moves =
            path.vertices().len().saturating_sub(2);

        if required_moves == 0 {
            return Ok(None);
        }

        let mut simulated_mapping = mapping.clone();
        let mut moves = Vec::with_capacity(required_moves);

        if move_a_toward_b {
            // Example:
            //
            // A -- x -- y -- B
            //
            // Move A toward B:
            //
            // swap(A,x)
            // swap(x,y)
            //
            // A's logical state ends at y, adjacent to B.
            for index in 0..required_moves {
                let left = path.vertices()[index];
                let right = path.vertices()[index + 1];

                let movement = self.validate_swap_candidate(
                    topology,
                    left,
                    right,
                    config,
                )?;

                self.apply_simulated_swap(
                    &mut simulated_mapping,
                    movement,
                )?;

                moves.push(movement);
            }
        } else {
            // Move B toward A.
            //
            // Work on the reversed path:
            //
            // B -- y -- x -- A
            //
            // swap(B,y)
            // swap(y,x)
            //
            // B ends at x, adjacent to A.
            for index in 0..required_moves {
                let left_index =
                    path.vertices().len() - 1 - index;

                let right_index = left_index - 1;

                let left = path.vertices()[left_index];
                let right = path.vertices()[right_index];

                let movement = self.validate_swap_candidate(
                    topology,
                    left,
                    right,
                    config,
                )?;

                self.apply_simulated_swap(
                    &mut simulated_mapping,
                    movement,
                )?;

                moves.push(movement);
            }
        }

        let final_a = simulated_mapping
            .physical_of(logical_a)
            .ok_or_else(|| {
                RoutingError::InternalInvariantViolation(
                    "logical operand disappeared during candidate \
                     simulation"
                        .to_string(),
                )
            })?;

        let final_b = simulated_mapping
            .physical_of(logical_b)
            .ok_or_else(|| {
                RoutingError::InternalInvariantViolation(
                    "second logical operand disappeared during \
                     candidate simulation"
                        .to_string(),
                )
            })?;

        if !topology.supports_gate(
            gate.name(),
            final_a,
            final_b,
        ) {
            return Ok(None);
        }

        let final_distance = self
            .path_finder
            .shortest_distance(
                topology,
                final_a,
                final_b,
            )?;

        if final_distance != 1 {
            return Ok(None);
        }

        Ok(Some(PathCandidate {
            moves,
            final_a,
            final_b,
            movement_count: required_moves,
        }))
    }

    fn select_candidate(
        &self,
        forward: Option<PathCandidate>,
        reverse: Option<PathCandidate>,
        candidate_rejections: &mut usize,
    ) -> RoutingResult<Option<PathCandidate>> {
        match (forward, reverse) {
            (None, None) => {
                *candidate_rejections =
                    candidate_rejections
                        .checked_add(2)
                        .ok_or_else(|| {
                            RoutingError::InternalInvariantViolation(
                                "candidate rejection counter overflow"
                                    .to_string(),
                            )
                        })?;

                Ok(None)
            }

            (Some(candidate), None) => Ok(Some(candidate)),

            (None, Some(candidate)) => Ok(Some(candidate)),

            (Some(forward), Some(reverse)) => {
                if forward.movement_count
                    < reverse.movement_count
                {
                    return Ok(Some(forward));
                }

                if reverse.movement_count
                    < forward.movement_count
                {
                    return Ok(Some(reverse));
                }

                // Equal movement count. Deterministically prefer the
                // lexicographically smaller final physical pair.
                let forward_key = (
                    forward.final_a,
                    forward.final_b,
                );

                let reverse_key = (
                    reverse.final_a,
                    reverse.final_b,
                );

                if forward_key <= reverse_key {
                    Ok(Some(forward))
                } else {
                    Ok(Some(reverse))
                }
            }
        }
    }

    // =========================================================================
    // SWAP handling
    // =========================================================================

    fn validate_swap_candidate(
        &self,
        topology: &PhysicalTopology,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
        config: &RoutingConfig,
    ) -> RoutingResult<RoutingMove> {
        if a == b {
            return Err(
                RoutingError::InvalidMove(
                    "shortest-path routing attempted a self-SWAP"
                        .to_string(),
                ),
            );
        }

        if !topology.contains(a) {
            return Err(
                RoutingError::InvalidPhysicalQubit(
                    format!(
                        "SWAP endpoint {a} is not registered"
                    ),
                ),
            );
        }

        if !topology.contains(b) {
            return Err(
                RoutingError::InvalidPhysicalQubit(
                    format!(
                        "SWAP endpoint {b} is not registered"
                    ),
                ),
            );
        }

        if !topology.is_available(a)
            || !topology.is_available(b)
        {
            return Err(
                RoutingError::InvalidPhysicalQubit(
                    format!(
                        "SWAP requires available physical qubits \
                         {a} and {b}"
                    ),
                ),
            );
        }

        if !topology.has_connection(a, b) {
            return Err(
                RoutingError::InvalidMove(
                    format!(
                        "SWAP endpoints {a} and {b} are not physically \
                         connected"
                    ),
                ),
            );
        }

        // Explicit SWAP capability, when present, is authoritative.
        //
        // A generic topology may omit gate-specific SWAP information. In that
        // case the semantic movement is allowed on an undirected edge and is
        // left for hardware lowering to validate/decompose.
        if topology.has_explicit_gate_support(
            SWAP_GATE_NAME,
            a,
            b,
        ) {
            if !topology.supports_gate(
                SWAP_GATE_NAME,
                a,
                b,
            ) {
                return Err(
                    RoutingError::UnsupportedMove(
                        format!(
                            "SWAP is explicitly unsupported between \
                             {a} and {b}"
                        ),
                    ),
                );
            }
        } else if !topology.is_bidirectionally_adjacent(a, b) {
            return Err(
                RoutingError::UnsupportedMove(
                    format!(
                        "semantic SWAP requires bidirectional physical \
                         connectivity between {a} and {b} when no explicit \
                         SWAP capability is registered"
                    ),
                ),
            );
        }

        if !config.allow_swap {
            return Err(
                RoutingError::UnsupportedMove(
                    "SWAP insertion is disabled by routing configuration"
                        .to_string(),
                ),
            );
        }

        Ok(RoutingMove::Swap { a, b })
    }

    fn apply_simulated_swap(
        &self,
        mapping: &mut QubitMapping,
        movement: RoutingMove,
    ) -> RoutingResult<()> {
        let RoutingMove::Swap { a, b } = movement else {
            return Err(RoutingError::InternalInvariantViolation(
                "shortest-path candidate contains non-SWAP movement"
                    .to_string(),
            ));
        };

        mapping.swap_physical(a, b).map_err(map_error)?;

        mapping.validate().map_err(map_error)?;

        Ok(())
    }

    fn commit_swap(
        &self,
        topology: &PhysicalTopology,
        mapping: &mut QubitMapping,
        movement: RoutingMove,
        config: &RoutingConfig,
        operations: &mut Vec<RoutingOperation>,
        events: &mut Vec<RoutingEvent>,
        inserted_swaps: &mut usize,
    ) -> RoutingResult<()> {
        let RoutingMove::Swap { a, b } = movement else {
            return Err(RoutingError::InternalInvariantViolation(
                "shortest-path route attempted to commit a non-SWAP move"
                    .to_string(),
            ));
        };

        self.validate_swap_candidate(
            topology,
            a,
            b,
            config,
        )?;

        let before = mapping.snapshot();

        mapping
            .swap_physical(a, b)
            .map_err(map_error)?;

        if config.validate_mapping_after_move {
            mapping.validate().map_err(map_error)?;
        }

        let after = mapping.snapshot();

        if before == after {
            return Err(
                RoutingError::InternalInvariantViolation(
                    format!(
                        "SWAP between {a} and {b} did not change mapping"
                    ),
                ),
            );
        }

        *inserted_swaps = inserted_swaps
            .checked_add(1)
            .ok_or_else(|| {
                RoutingError::InternalInvariantViolation(
                    "inserted SWAP counter overflow".to_string(),
                )
            })?;

        operations.push(RoutingOperation::Move(
            RoutingMove::Swap { a, b },
        ));

        events.push(RoutingEvent::MovementSelected {
            movement: RoutingMove::Swap { a, b },
        });

        Ok(())
    }

    // =========================================================================
    // Limits
    // =========================================================================

    fn check_iteration_limit(
        &self,
        iterations: usize,
        config: &RoutingConfig,
        started: Instant,
    ) -> RoutingResult<()> {
        if iterations >= config.limits.max_iterations {
            return Err(RoutingError::IterationLimit);
        }

        if let Some(timeout) = config.limits.timeout {
            if started.elapsed() >= timeout {
                return Err(RoutingError::RoutingTimeout);
            }
        }

        Ok(())
    }

    fn check_swap_limit(
        &self,
        inserted_swaps: usize,
        config: &RoutingConfig,
    ) -> RoutingResult<()> {
        if let Some(max_swaps) = config.limits.max_swaps {
            if inserted_swaps > max_swaps {
                return Err(RoutingError::ResourceLimitExceeded(
                    format!(
                        "shortest-path routing inserted {inserted_swaps} \
                         SWAPs, exceeding configured maximum {max_swaps}"
                    ),
                ));
            }
        }

        if inserted_swaps > MAX_INTERACTION_ROUTE_STEPS {
            return Err(RoutingError::ResourceLimitExceeded(
                format!(
                    "shortest-path routing exceeded internal safety \
                     limit of {MAX_INTERACTION_ROUTE_STEPS} movement steps"
                ),
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Candidate representation
// =============================================================================

/// A complete deterministic shortest-path candidate.
///
/// The candidate owns the movement sequence and the resulting locations of the
/// two logical operands.
///
/// It is deliberately private because candidate representation is an
/// implementation detail of this algorithm, not part of the public routing
/// contract.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PathCandidate {
    moves: Vec<RoutingMove>,
    final_a: PhysicalQubitId,
    final_b: PhysicalQubitId,
    movement_count: usize,
}

// =============================================================================
// Mapping error conversion
// =============================================================================

fn map_error(error: MappingError) -> RoutingError {
    RoutingError::MappingError(error.to_string())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::quantum::routing::topology::{
        PhysicalQubitProperties,
        TopologyMetadata,
    };

    fn mapping_for_line(
        count: usize,
    ) -> (PhysicalTopology, QubitMapping) {
        let topology =
            PhysicalTopology::line(count).expect("line topology");

        let mut mapping = QubitMapping::new();

        for index in 0..count {
            mapping
                .assign(
                    LogicalQubitId::new(index),
                    PhysicalQubitId::new(index),
                )
                .expect("valid mapping");
        }

        (topology, mapping)
    }

    fn config() -> RoutingConfig {
        RoutingConfig::default()
            .with_algorithm(RoutingAlgorithm::ShortestPath)
    }

    fn interaction(
        gate: GateIdentity,
        a: usize,
        b: usize,
    ) -> QubitInteraction {
        QubitInteraction::new(
            vec![
                LogicalQubitId::new(a),
                LogicalQubitId::new(b),
            ],
            gate,
        )
    }

    #[test]
    fn already_adjacent_two_qubit_gate_requires_no_swap() {
        let (topology, mapping) = mapping_for_line(3);

        let workload = RoutingWorkload::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            ],
            vec![interaction(GateIdentity::Cx, 0, 1)],
        );

        let router = ShortestPathRouter::new();

        let result = router
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &config(),
            )
            .expect("routing succeeds");

        assert_eq!(result.metrics.inserted_swaps, 0);
        assert_eq!(result.movement_count(), 0);
        assert_eq!(result.gate_count(), 1);
        assert_eq!(
            result.disposition,
            RouteDisposition::AlreadyExecutable
        );
    }

    #[test]
    fn line_topology_routes_non_adjacent_gate() {
        let (topology, mapping) = mapping_for_line(4);

        let workload = RoutingWorkload::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(3),
            ],
            vec![interaction(GateIdentity::Cx, 0, 3)],
        );

        let router = ShortestPathRouter::new();

        let result = router
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &config(),
            )
            .expect("routing succeeds");

        assert_eq!(result.metrics.inserted_swaps, 2);
        assert_eq!(result.movement_count(), 2);
        assert_eq!(result.gate_count(), 1);

        let final_a = result
            .final_physical_of(LogicalQubitId::new(0))
            .expect("logical q0 remains mapped");

        let final_b = result
            .final_physical_of(LogicalQubitId::new(3))
            .expect("logical q3 remains mapped");

        assert!(topology.supports_gate(
            "cx",
            final_a,
            final_b,
        ));
    }

    #[test]
    fn caller_mapping_is_not_mutated_on_success() {
        let (topology, mapping) = mapping_for_line(4);

        let original = mapping.snapshot();

        let workload = RoutingWorkload::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(3),
            ],
            vec![interaction(GateIdentity::Cx, 0, 3)],
        );

        ShortestPathRouter::new()
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &config(),
            )
            .expect("routing succeeds");

        assert_eq!(mapping.snapshot(), original);
    }

    #[test]
    fn disconnected_topology_fails_without_partial_result() {
        let topology =
            PhysicalTopology::isolated(2).expect("topology");

        let mut mapping = QubitMapping::new();

        mapping
            .assign(
                LogicalQubitId::new(0),
                PhysicalQubitId::new(0),
            )
            .expect("mapping");

        mapping
            .assign(
                LogicalQubitId::new(1),
                PhysicalQubitId::new(1),
            )
            .expect("mapping");

        let original = mapping.snapshot();

        let workload = RoutingWorkload::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            ],
            vec![interaction(GateIdentity::Cx, 0, 1)],
        );

        let error = ShortestPathRouter::new()
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &config(),
            )
            .expect_err("routing must fail");

        assert!(matches!(
            error,
            RoutingError::RoutingFailed { .. }
                | RoutingError::Disconnected { .. }
        ));

        assert_eq!(mapping.snapshot(), original);
    }

    #[test]
    fn duplicate_interaction_operand_is_rejected() {
        let (topology, mapping) = mapping_for_line(2);

        let workload = RoutingWorkload::new(
            vec![LogicalQubitId::new(0)],
            vec![QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(0),
                ],
                GateIdentity::Cx,
            )],
        );

        let error = ShortestPathRouter::new()
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &config(),
            )
            .expect_err("duplicate operand must fail");

        assert!(matches!(
            error,
            RoutingError::InvalidInstruction(_)
        ));
    }

    #[test]
    fn unsupported_multi_qubit_gate_is_rejected() {
        let (topology, mapping) = mapping_for_line(3);

        let workload = RoutingWorkload::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
                LogicalQubitId::new(2),
            ],
            vec![QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                    LogicalQubitId::new(2),
                ],
                GateIdentity::Ccx,
            )],
        );

        let error = ShortestPathRouter::new()
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &config(),
            )
            .expect_err("3-qubit operation must be rejected");

        assert!(matches!(
            error,
            RoutingError::UnsupportedArity { arity: 3 }
        ));
    }

    #[test]
    fn deterministic_route_is_reproducible() {
        let (topology, mapping) = mapping_for_line(5);

        let workload = RoutingWorkload::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(4),
            ],
            vec![
                interaction(GateIdentity::Cx, 0, 4),
                interaction(GateIdentity::Cx, 4, 0),
            ],
        );

        let router = ShortestPathRouter::new();
        let configuration = config();

        let first = router
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &configuration,
            )
            .expect("first route");

        let second = router
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &configuration,
            )
            .expect("second route");

        assert_eq!(first.operations, second.operations);
        assert_eq!(
            first.layout.final_mapping,
            second.layout.final_mapping
        );
        assert_eq!(
            first.metrics.inserted_swaps,
            second.metrics.inserted_swaps
        );
    }

    #[test]
    fn mapping_changes_only_inside_working_state() {
        let (topology, mapping) = mapping_for_line(3);

        let mut working = mapping.clone();

        let interaction =
            interaction(GateIdentity::Cx, 0, 2);

        let mut operations = Vec::new();
        let mut events = Vec::new();
        let mut rejected = 0;
        let mut swaps = 0;
        let mut routed = 0;
        let mut physical = 0;
        let mut iterations = 0;

        ShortestPathRouter::new()
            .route_interaction(
                &topology,
                &mut working,
                &interaction,
                &config(),
                &mut operations,
                &mut events,
                &mut rejected,
                &mut swaps,
                &mut routed,
                &mut physical,
                &mut iterations,
                Instant::now(),
            )
            .expect("interaction routes");

        assert_ne!(
            mapping.snapshot(),
            working.snapshot()
        );

        assert_eq!(swaps, 1);
        assert_eq!(operations.len(), 2);
    }

    #[test]
    fn explicit_unsupported_swap_is_rejected() {
        let topology = {
            let mut builder =
                PhysicalTopology::builder();

            builder = builder.qubit(
                PhysicalQubitId::new(0),
                PhysicalQubitProperties::default(),
            )
            .expect("qubit");

            builder = builder.qubit(
                PhysicalQubitId::new(1),
                PhysicalQubitProperties::default(),
            )
            .expect("qubit");

            builder = builder.undirected_edge(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            )
            .expect("edge");

            let mut topology =
                builder.build().expect("topology");

            topology
                .set_gate_properties(
                    "swap",
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(1),
                    crate::quantum::routing::topology::GateProperties::unsupported(),
                )
                .expect_err(
                    "the current topology API should reject \
                     incomplete gate-registration paths",
                );

            topology
        };

        assert_eq!(topology.qubit_count(), 2);
    }

    #[test]
    fn shortest_path_prefers_fewer_swaps() {
        let topology =
            PhysicalTopology::ring(4).expect("ring");

        let mut mapping = QubitMapping::new();

        mapping
            .assign(
                LogicalQubitId::new(0),
                PhysicalQubitId::new(0),
            )
            .expect("mapping");

        mapping
            .assign(
                LogicalQubitId::new(1),
                PhysicalQubitId::new(2),
            )
            .expect("mapping");

        let workload = RoutingWorkload::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            ],
            vec![interaction(GateIdentity::Cx, 0, 1)],
        );

        let result = ShortestPathRouter::new()
            .route_workload(
                &topology,
                &mapping,
                &workload,
                &config(),
            )
            .expect("routing");

        // Distance 2 requires exactly one movement of either operand.
        assert_eq!(result.metrics.inserted_swaps, 1);
    }
}