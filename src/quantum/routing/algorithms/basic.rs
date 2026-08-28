//! Zamani Quantum Routing — Basic Deterministic Router
//!
//! This module implements Zamani's production baseline routing algorithm.
//!
//! # Responsibility
//!
//! `BasicRouter` performs conservative, deterministic, topology-aware routing
//! of logical quantum interactions onto physical hardware connectivity.
//!
//! It is intentionally a baseline algorithm rather than a global optimizer.
//! Its primary purposes are:
//!
//! - correctness;
//! - determinism;
//! - predictable behavior;
//! - small implementation surface;
//! - a reliable fallback for more advanced routers;
//! - a reference implementation against which heuristic routers can be tested.
//!
//! # Algorithm
//!
//! For every operation in the routing front layer:
//!
//! 1. determine the physical locations of its logical operands;
//! 2. check whether the operation is currently executable;
//! 3. if executable, emit it;
//! 4. otherwise generate legal adjacent movement candidates;
//! 5. choose the deterministic minimum-cost candidate;
//! 6. apply the movement to the mapping;
//! 7. emit the routing operation;
//! 8. repeat until the operation becomes executable;
//! 9. emit the original logical operation;
//! 10. continue with the next operation.
//!
//! This implementation never modifies the caller's input circuit or mapping.
//!
//! # Important architectural boundary
//!
//! This module does NOT:
//!
//! - parse Zamani source;
//! - parse OpenQASM;
//! - manipulate compiler-specific `IrInstruction` values;
//! - know about a particular quantum vendor;
//! - synthesize pulses;
//! - schedule operations;
//! - perform general gate decomposition;
//! - perform circuit optimization;
//! - perform QEC decoding;
//! - communicate with hardware;
//! - assume that a physical SWAP is a native hardware gate.
//!
//! A [`RoutingOperation::Swap`] represents a routing state permutation. Hardware
//! lowering is a later compiler/backend responsibility.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - routing input;
//! - topology;
//! - initial mapping;
//! - cost model;
//! - configuration;
//!
//! this implementation produces identical routing decisions.
//!
//! Candidate ordering is canonicalized using physical qubit identifiers and
//! stable operation indices. No hash-map iteration order is used to make a
//! routing decision.
//!
//! # Rust compatibility
//!
//! This module targets Rust 1.97.1.
//!
//! It requires no nightly features and contains no `unsafe` code.
//!
//! # Integration
//!
//! This file depends only on the stable routing contracts:
//!
//! ```text
//! types.rs
//! errors.rs
//! topology.rs
//! mapping.rs
//! cost.rs
//! config.rs
//! result.rs
//! path.rs
//! candidates.rs
//! moves/swap.rs
//! ```
//!
//! It does NOT depend on:
//!
//! ```text
//! lookahead.rs
//! sabre.rs
//! noise_aware.rs
//! dynamic.rs
//! router.rs
//! transpiler.rs
//! hardware/
//! compiler/
//! frontend/
//! ```
//!
//! Consequently, those later modules can be implemented without requiring a
//! change to this file.

// =============================================================================
// Imports from the frozen routing contracts
// =============================================================================

use std::cmp::Ordering;
use std::collections::HashSet;

use crate::quantum::routing::candidates::SwapCandidate;
use crate::quantum::routing::config::RoutingConfig;
use crate::quantum::routing::cost::RoutingCost;
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::result::{
    RoutingInput,
    RoutingMetrics,
    RoutingResult,
};
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::{
    LogicalQubitId,
    PhysicalQubitId,
    QuantumOperation,
    RoutingOperation,
};

// =============================================================================
// Public algorithm
// =============================================================================

/// Deterministic greedy routing algorithm.
///
/// `BasicRouter` is intentionally conservative:
///
/// - it never speculates across multiple future gates;
/// - it never mutates the caller's state;
/// - it only performs topology-legal movement;
/// - it uses the configured [`RoutingCost`] ordering;
/// - it has explicit iteration and resource limits.
///
/// It is suitable as:
///
/// - the default correctness baseline;
/// - a deterministic fallback;
/// - a reference implementation for testing SABRE/lookahead;
/// - a simple router for small circuits;
/// - a debugging router.
///
/// It should not be expected to achieve the minimum possible SWAP count for
/// arbitrary circuits.
#[derive(Debug, Clone, Default)]
pub struct BasicRouter;

impl BasicRouter {
    /// Creates a deterministic basic router.
    pub const fn new() -> Self {
        Self
    }

    /// Routes a complete circuit.
    ///
    /// The input is treated as immutable. A private working mapping is created
    /// and all routing operations are accumulated into a new result.
    ///
    /// No externally visible state is modified if routing fails.
    pub fn route(
        &self,
        input: &RoutingInput<'_>,
    ) -> Result<RoutingResult, RoutingError> {
        validate_input(input)?;

        let mut mapping = input.initial_mapping.clone();
        mapping.validate(input.topology)?;

        let initial_mapping = mapping.clone();

        let mut operations = Vec::with_capacity(input.operations.len());
        let mut metrics = RoutingMetrics::new();

        let max_iterations = effective_iteration_limit(input.config);

        for (operation_index, operation) in input.operations.iter().enumerate() {
            metrics.original_operation_count =
                checked_increment(
                    metrics.original_operation_count,
                    "original operation count",
                )?;

            validate_operation(
                operation,
                input.topology,
                &mapping,
            )?;

            if operation.is_zero_qubit() {
                operations.push(operation.clone().into_routing_operation()?);
                continue;
            }

            if operation.is_single_qubit() {
                operations.push(
                    operation
                        .clone()
                        .into_routing_operation()?,
                );
                continue;
            }

            let mut iterations = 0usize;

            while !is_executable(
                operation,
                input.topology,
                &mapping,
            )? {
                iterations = checked_increment(
                    iterations,
                    "basic-router iteration count",
                )?;

                if iterations > max_iterations {
                    return Err(
                        RoutingError::IterationLimit {
                            operation_index,
                            limit: max_iterations,
                        },
                    );
                }

                if let Some(max_swaps) = input.config.max_swaps {
                    if metrics.inserted_swaps >= max_swaps {
                        return Err(
                            RoutingError::RoutingTimeout {
                                operation_index,
                                reason: "maximum SWAP limit reached"
                                    .to_string(),
                            },
                        );
                    }
                }

                let candidates = generate_candidates(
                    operation,
                    input.topology,
                    &mapping,
                )?;

                let candidate = choose_candidate(
                    &candidates,
                    input,
                    &mapping,
                )?;

                apply_candidate(
                    &candidate,
                    input.topology,
                    &mut mapping,
                )?;

                operations.push(
                    RoutingOperation::Swap {
                        a: candidate.a,
                        b: candidate.b,
                    },
                );

                metrics.inserted_swaps =
                    checked_increment(
                        metrics.inserted_swaps,
                        "inserted SWAP count",
                    )?;

                metrics.routing_operation_count =
                    checked_increment(
                        metrics.routing_operation_count,
                        "routing operation count",
                    )?;
            }

            operations.push(
                operation
                    .clone()
                    .into_routing_operation()?,
            );

            metrics.routed_gate_count =
                checked_increment(
                    metrics.routed_gate_count,
                    "routed gate count",
                )?;
        }

        mapping.validate(input.topology)?;

        metrics.final_operation_count = operations.len();

        metrics.routing_overhead =
            metrics
                .final_operation_count
                .checked_sub(
                    metrics.original_operation_count,
                )
                .ok_or(
                    RoutingError::InternalInvariantViolation {
                        message:
                            "final operation count is smaller than original operation count",
                    },
                )?;

        metrics.algorithm = "basic".to_string();

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

    /// Routes a circuit with an explicitly supplied initial mapping.
    ///
    /// This convenience method does not own or mutate the supplied mapping.
    pub fn route_with_mapping(
        &self,
        operations: &[QuantumOperation],
        topology: &Topology,
        mapping: &QubitMapping,
        config: &RoutingConfig,
    ) -> Result<RoutingResult, RoutingError> {
        let input = RoutingInput::new(
            operations,
            topology,
            mapping,
            config,
        )?;

        self.route(&input)
    }

    /// Returns the name of this algorithm.
    ///
    /// This is used by the algorithm registry and reproducibility metadata.
    pub const fn name(&self) -> &'static str {
        "basic"
    }
}

// =============================================================================
// Input validation
// =============================================================================

/// Validates every invariant required by the basic router.
///
/// This validation intentionally happens before any routing mutation so that
/// invalid input cannot result in a partially constructed output.
fn validate_input(
    input: &RoutingInput<'_>,
) -> Result<(), RoutingError> {
    input.topology.validate()?;

    input.initial_mapping.validate(input.topology)?;

    if input.topology.qubit_count() == 0 {
        return Err(RoutingError::EmptyTopology);
    }

    if input.operations.len() > input.config.max_operations {
        return Err(RoutingError::InvalidConfiguration {
            message: format!(
                "circuit contains {} operations but the configured routing limit is {}",
                input.operations.len(),
                input.config.max_operations
            ),
        });
    }

    if input.initial_mapping.len()
        > input.topology.qubit_count()
    {
        return Err(RoutingError::InsufficientQubits {
            required: input.initial_mapping.len(),
            available: input.topology.qubit_count(),
        });
    }

    for (index, operation) in input.operations.iter().enumerate() {
        validate_operation(
            operation,
            input.topology,
            input.initial_mapping,
        )
        .map_err(|error| {
            error.with_operation_index(index)
        })?;
    }

    Ok(())
}

/// Validates one logical operation against the mapping and topology.
///
/// This does not require the operation to currently be executable. A
/// non-adjacent two-qubit operation is precisely the case that routing is
/// supposed to repair.
fn validate_operation(
    operation: &QuantumOperation,
    topology: &Topology,
    mapping: &QubitMapping,
) -> Result<(), RoutingError> {
    if operation.is_zero_qubit()
        || operation.is_single_qubit()
    {
        for logical in operation.logical_operands() {
            if !mapping.contains_logical(logical) {
                return Err(
                    RoutingError::InvalidLogicalQubit(
                        *logical,
                    ),
                );
            }
        }

        return Ok(());
    }

    let arity = operation.arity();

    if arity != 2 {
        return Err(
            RoutingError::UnsupportedArity {
                gate: operation.name().to_string(),
                arity,
                maximum: 2,
            },
        );
    }

    let operands = operation.logical_operands();

    if operands.len() != 2 {
        return Err(
            RoutingError::InternalInvariantViolation {
                message:
                    "two-qubit operation reported an operand count different from two",
            },
        );
    }

    let logical_a = operands[0];
    let logical_b = operands[1];

    if logical_a == logical_b {
        return Err(
            RoutingError::InvalidQuantumOperation {
                operation: operation.name().to_string(),
                reason:
                    "a two-qubit operation cannot use the same logical qubit twice"
                        .to_string(),
            },
        );
    }

    if !mapping.contains_logical(logical_a) {
        return Err(
            RoutingError::InvalidLogicalQubit(
                *logical_a,
            ),
        );
    }

    if !mapping.contains_logical(logical_b) {
        return Err(
            RoutingError::InvalidLogicalQubit(
                *logical_b,
            ),
        );
    }

    let physical_a = mapping
        .physical_of(*logical_a)
        .ok_or(
            RoutingError::InvalidLogicalQubit(
                *logical_a,
            ),
        )?;

    let physical_b = mapping
        .physical_of(*logical_b)
        .ok_or(
            RoutingError::InvalidLogicalQubit(
                *logical_b,
            ),
        )?;

    if !topology.contains(physical_a) {
        return Err(
            RoutingError::InvalidPhysicalQubit(
                physical_a,
            ),
        );
    }

    if !topology.contains(physical_b) {
        return Err(
            RoutingError::InvalidPhysicalQubit(
                physical_b,
            ),
        );
    }

    Ok(())
}

// =============================================================================
// Executability
// =============================================================================

/// Determines whether an operation can execute under the current mapping.
///
/// Importantly, physical adjacency is not sufficient. Gate-specific topology
/// legality is checked as well, allowing the same topology to distinguish
/// between operations such as directional CX and symmetric CZ.
fn is_executable(
    operation: &QuantumOperation,
    topology: &Topology,
    mapping: &QubitMapping,
) -> Result<bool, RoutingError> {
    if operation.arity() <= 1 {
        return Ok(true);
    }

    if operation.arity() != 2 {
        return Err(
            RoutingError::UnsupportedArity {
                gate: operation.name().to_string(),
                arity: operation.arity(),
                maximum: 2,
            },
        );
    }

    let operands = operation.logical_operands();

    let logical_a = operands
        .first()
        .ok_or(
            RoutingError::InternalInvariantViolation {
                message:
                    "two-qubit operation has no first operand",
            },
        )?;

    let logical_b = operands
        .get(1)
        .ok_or(
            RoutingError::InternalInvariantViolation {
                message:
                    "two-qubit operation has no second operand",
            },
        )?;

    let physical_a = mapping
        .physical_of(**logical_a)
        .ok_or(
            RoutingError::InvalidLogicalQubit(
                **logical_a,
            ),
        )?;

    let physical_b = mapping
        .physical_of(**logical_b)
        .ok_or(
            RoutingError::InvalidLogicalQubit(
                **logical_b,
            ),
        )?;

    Ok(topology.supports_gate(
        operation.name(),
        physical_a,
        physical_b,
    ))
}

// =============================================================================
// Candidate generation
// =============================================================================

/// Generates all currently legal SWAP candidates relevant to the blocked
/// two-qubit interaction.
///
/// The basic algorithm deliberately limits candidates to edges incident to
/// either operand. A SWAP elsewhere cannot change the distance between the two
/// operands and therefore cannot help route the currently blocked operation.
fn generate_candidates(
    operation: &QuantumOperation,
    topology: &Topology,
    mapping: &QubitMapping,
) -> Result<Vec<SwapCandidate>, RoutingError> {
    if operation.arity() != 2 {
        return Err(
            RoutingError::UnsupportedArity {
                gate: operation.name().to_string(),
                arity: operation.arity(),
                maximum: 2,
            },
        );
    }

    let operands = operation.logical_operands();

    let logical_a = operands
        .first()
        .ok_or(
            RoutingError::InternalInvariantViolation {
                message:
                    "blocked two-qubit operation has no first operand",
            },
        )?;

    let logical_b = operands
        .get(1)
        .ok_or(
            RoutingError::InternalInvariantViolation {
                message:
                    "blocked two-qubit operation has no second operand",
            },
        )?;

    let physical_a = mapping
        .physical_of(**logical_a)
        .ok_or(
            RoutingError::InvalidLogicalQubit(
                **logical_a,
            ),
        )?;

    let physical_b = mapping
        .physical_of(**logical_b)
        .ok_or(
            RoutingError::InvalidLogicalQubit(
                **logical_b,
            ),
        )?;

    let mut candidates = Vec::new();
    let mut seen = HashSet::new();

    add_incident_candidates(
        physical_a,
        physical_b,
        topology,
        mapping,
        &mut candidates,
        &mut seen,
    )?;

    add_incident_candidates(
        physical_b,
        physical_a,
        topology,
        mapping,
        &mut candidates,
        &mut seen,
    )?;

    if candidates.is_empty() {
        return Err(
            RoutingError::NoCandidateSwap {
                operation: operation.name().to_string(),
            },
        );
    }

    candidates.sort_by(compare_candidates);

    Ok(candidates)
}

/// Adds legal SWAPs incident to one physical operand.
fn add_incident_candidates(
    operand: PhysicalQubitId,
    other_operand: PhysicalQubitId,
    topology: &Topology,
    mapping: &QubitMapping,
    output: &mut Vec<SwapCandidate>,
    seen: &mut HashSet<(PhysicalQubitId, PhysicalQubitId)>,
) -> Result<(), RoutingError> {
    let mut neighbours = topology
        .neighbors(operand)
        .collect::<Vec<_>>();

    neighbours.sort_unstable();

    for neighbour in neighbours {
        if neighbour == other_operand {
            continue;
        }

        let edge = canonical_edge(operand, neighbour);

        if !seen.insert(edge) {
            continue;
        }

        if !topology.supports_swap(
            edge.0,
            edge.1,
        ) {
            continue;
        }

        let occupied_by_operand =
            mapping.logical_of(operand);

        let occupied_by_neighbour =
            mapping.logical_of(neighbour);

        // A physical location with no logical qubit is a valid routing target.
        // Therefore the candidate does not require both endpoints to be
        // occupied.
        let distance_before = topology
            .distance(
                operand,
                other_operand,
            )
            .ok_or(
                RoutingError::NoRoutingPath {
                    from: operand,
                    to: other_operand,
                },
            )?;

        let distance_after = projected_distance_after_swap(
            operand,
            neighbour,
            other_operand,
            topology,
        )?;

        let improvement =
            distance_before.saturating_sub(distance_after);

        output.push(
            SwapCandidate::new(
                edge.0,
                edge.1,
                occupied_by_operand,
                occupied_by_neighbour,
                distance_before,
                distance_after,
                improvement,
            ),
        );
    }

    Ok(())
}

/// Computes the distance between the routing operands after a hypothetical
/// exchange of the two physical locations.
///
/// Only the operand occupying `operand` or `neighbour` needs to be considered.
/// If the target interaction's second operand is neither endpoint, its
/// physical position remains unchanged.
fn projected_distance_after_swap(
    operand: PhysicalQubitId,
    neighbour: PhysicalQubitId,
    other_operand: PhysicalQubitId,
    topology: &Topology,
) -> Result<usize, RoutingError> {
    let new_operand =
        if other_operand == operand {
            neighbour
        } else if other_operand == neighbour {
            operand
        } else {
            other_operand
        };

    topology
        .distance(new_operand, other_operand)
        .ok_or(
            RoutingError::NoRoutingPath {
                from: new_operand,
                to: other_operand,
            },
        )
}

// =============================================================================
// Candidate selection
// =============================================================================

/// Selects the minimum-cost candidate.
///
/// The baseline router uses a lexicographic objective:
///
/// 1. maximize distance improvement;
/// 2. minimize resulting distance;
/// 3. minimize configured routing cost;
/// 4. canonical physical endpoint order.
///
/// The explicit endpoint tie-break guarantees deterministic behavior even when
/// the cost model considers two candidates equal.
fn choose_candidate(
    candidates: &[SwapCandidate],
    input: &RoutingInput<'_>,
    mapping: &QubitMapping,
) -> Result<SwapCandidate, RoutingError> {
    let mut best: Option<(
        SwapCandidate,
        RoutingCost,
    )> = None;

    for candidate in candidates {
        let cost =
            input.config.cost_model.evaluate_swap(
                candidate,
                input.topology,
                mapping,
            )?;

        match &best {
            None => {
                best = Some((
                    candidate.clone(),
                    cost,
                ));
            }

            Some((
                best_candidate,
                best_cost,
            )) => {
                let ordering =
                    compare_candidate_with_cost(
                        candidate,
                        &cost,
                        best_candidate,
                        best_cost,
                    );

                if ordering == Ordering::Less {
                    best = Some((
                        candidate.clone(),
                        cost,
                    ));
                }
            }
        }
    }

    best.map(|(candidate, _)| candidate)
        .ok_or(
            RoutingError::NoCandidateSwap {
                operation: "basic".to_string(),
            },
        )
}

/// Canonical candidate ordering before cost evaluation.
///
/// This is deliberately stable and independent of hash-map ordering.
fn compare_candidates(
    left: &SwapCandidate,
    right: &SwapCandidate,
) -> Ordering {
    right
        .improvement
        .cmp(&left.improvement)
        .then_with(|| {
            left.distance_after
                .cmp(&right.distance_after)
        })
        .then_with(|| {
            left.a.cmp(&right.a)
        })
        .then_with(|| {
            left.b.cmp(&right.b)
        })
}

/// Cost-aware candidate ordering.
fn compare_candidate_with_cost(
    left: &SwapCandidate,
    left_cost: &RoutingCost,
    right: &SwapCandidate,
    right_cost: &RoutingCost,
) -> Ordering {
    left_cost
        .cmp(right_cost)
        .then_with(|| compare_candidates(left, right))
}

// =============================================================================
// Mapping mutation
// =============================================================================

/// Applies a selected routing SWAP to the working mapping.
///
/// This operation is intentionally performed through `QubitMapping` rather
/// than by manipulating its internal storage. That keeps mapping invariants
/// centralized in `mapping.rs`.
fn apply_candidate(
    candidate: &SwapCandidate,
    topology: &Topology,
    mapping: &mut QubitMapping,
) -> Result<(), RoutingError> {
    if candidate.a == candidate.b {
        return Err(
            RoutingError::InvalidQuantumOperation {
                operation: "SWAP".to_string(),
                reason:
                    "a routing SWAP cannot use the same physical qubit twice"
                        .to_string(),
            },
        );
    }

    if !topology.contains(candidate.a) {
        return Err(
            RoutingError::InvalidPhysicalQubit(
                candidate.a,
            ),
        );
    }

    if !topology.contains(candidate.b) {
        return Err(
            RoutingError::InvalidPhysicalQubit(
                candidate.b,
            ),
        );
    }

    if !topology.is_adjacent(
        candidate.a,
        candidate.b,
    ) {
        return Err(
            RoutingError::InvalidQuantumOperation {
                operation: "SWAP".to_string(),
                reason: format!(
                    "physical qubits {} and {} are not adjacent",
                    candidate.a,
                    candidate.b
                ),
            },
        );
    }

    if !topology.supports_swap(
        candidate.a,
        candidate.b,
    ) {
        return Err(
            RoutingError::UnsupportedMove {
                operation: "SWAP".to_string(),
                a: candidate.a,
                b: candidate.b,
            },
        );
    }

    mapping.swap_physical(
        candidate.a,
        candidate.b,
    )?;

    mapping.validate(topology)?;

    Ok(())
}

// =============================================================================
// Utility
// =============================================================================

/// Produces a canonical undirected edge representation.
///
/// This prevents `(a,b)` and `(b,a)` from being considered different
/// candidates when the physical SWAP is symmetric.
fn canonical_edge(
    a: PhysicalQubitId,
    b: PhysicalQubitId,
) -> (
    PhysicalQubitId,
    PhysicalQubitId,
) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Returns the configured iteration limit, with an overflow-safe fallback.
fn effective_iteration_limit(
    config: &RoutingConfig,
) -> usize {
    config
        .max_iterations
        .max(1)
}

/// Overflow-safe increment used for metrics and counters.
fn checked_increment(
    value: usize,
    counter: &'static str,
) -> Result<usize, RoutingError> {
    value.checked_add(1).ok_or(
        RoutingError::InternalInvariantViolation {
            message: counter,
        },
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test helper
    // -------------------------------------------------------------------------

    fn line_topology() -> Topology {
        Topology::line(3)
            .expect("three-qubit line topology must be valid")
    }

    fn default_config() -> RoutingConfig {
        RoutingConfig::default()
    }

    // -------------------------------------------------------------------------
    // Constructor
    // -------------------------------------------------------------------------

    #[test]
    fn constructor_is_const_and_has_stable_name() {
        let router = BasicRouter::new();

        assert_eq!(
            router.name(),
            "basic"
        );
    }

    // -------------------------------------------------------------------------
    // Adjacent interaction
    // -------------------------------------------------------------------------

    #[test]
    fn adjacent_operation_requires_no_swap() {
        let topology = line_topology();

        let mapping =
            QubitMapping::from_pairs(
                [
                    (
                        LogicalQubitId::new(0),
                        PhysicalQubitId::new(0),
                    ),
                    (
                        LogicalQubitId::new(1),
                        PhysicalQubitId::new(1),
                    ),
                ],
                &topology,
            )
            .expect("mapping must be valid");

        let operation =
            QuantumOperation::two_qubit(
                "cx",
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            )
            .expect("operation must be valid");

        let input =
            RoutingInput::new(
                &[operation],
                &topology,
                &mapping,
                &default_config(),
            )
            .expect("input must be valid");

        let result =
            BasicRouter::new()
                .route(&input)
                .expect("adjacent operation must route");

        assert_eq!(
            result.metrics.inserted_swaps,
            0
        );

        assert_eq!(
            result.operations.len(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Non-adjacent interaction
    // -------------------------------------------------------------------------

    #[test]
    fn non_adjacent_operation_inserts_swap() {
        let topology = line_topology();

        let mapping =
            QubitMapping::from_pairs(
                [
                    (
                        LogicalQubitId::new(0),
                        PhysicalQubitId::new(0),
                    ),
                    (
                        LogicalQubitId::new(1),
                        PhysicalQubitId::new(2),
                    ),
                ],
                &topology,
            )
            .expect("mapping must be valid");

        let operation =
            QuantumOperation::two_qubit(
                "cx",
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            )
            .expect("operation must be valid");

        let input =
            RoutingInput::new(
                &[operation],
                &topology,
                &mapping,
                &default_config(),
            )
            .expect("input must be valid");

        let result =
            BasicRouter::new()
                .route(&input)
                .expect("operation must route");

        assert_eq!(
            result.metrics.inserted_swaps,
            1
        );

        assert!(
            result.operations.len() >= 2
        );

        assert!(
            result
                .final_mapping
                .validate(&topology)
                .is_ok()
        );
    }

    // -------------------------------------------------------------------------
    // Determinism
    // -------------------------------------------------------------------------

    #[test]
    fn routing_is_deterministic() {
        let topology = Topology::line(5)
            .expect("topology must be valid");

        let mapping =
            QubitMapping::from_pairs(
                [
                    (
                        LogicalQubitId::new(0),
                        PhysicalQubitId::new(0),
                    ),
                    (
                        LogicalQubitId::new(1),
                        PhysicalQubitId::new(4),
                    ),
                ],
                &topology,
            )
            .expect("mapping must be valid");

        let operation =
            QuantumOperation::two_qubit(
                "cx",
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            )
            .expect("operation must be valid");

        let config = default_config();

        let input =
            RoutingInput::new(
                &[operation],
                &topology,
                &mapping,
                &config,
            )
            .expect("input must be valid");

        let first =
            BasicRouter::new()
                .route(&input)
                .expect("first route must succeed");

        let second =
            BasicRouter::new()
                .route(&input)
                .expect("second route must succeed");

        assert_eq!(
            first.operations,
            second.operations
        );

        assert_eq!(
            first.final_mapping,
            second.final_mapping
        );

        assert_eq!(
            first.metrics.inserted_swaps,
            second.metrics.inserted_swaps
        );
    }

    // -------------------------------------------------------------------------
    // Transactional behavior
    // -------------------------------------------------------------------------

    #[test]
    fn failed_routing_does_not_modify_input_mapping() {
        let topology = line_topology();

        let mapping =
            QubitMapping::from_pairs(
                [
                    (
                        LogicalQubitId::new(0),
                        PhysicalQubitId::new(0),
                    ),
                    (
                        LogicalQubitId::new(1),
                        PhysicalQubitId::new(2),
                    ),
                ],
                &topology,
            )
            .expect("mapping must be valid");

        let original_mapping =
            mapping.clone();

        let operation =
            QuantumOperation::two_qubit(
                "cx",
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            )
            .expect("operation must be valid");

        let mut config =
            default_config();

        config.max_iterations = 0;

        let input =
            RoutingInput::new(
                &[operation],
                &topology,
                &mapping,
                &config,
            )
            .expect("input construction must succeed");

        let result =
            BasicRouter::new().route(&input);

        assert!(
            result.is_err()
        );

        assert_eq!(
            mapping,
            original_mapping
        );
    }

    // -------------------------------------------------------------------------
    // Same logical qubit
    // -------------------------------------------------------------------------

    #[test]
    fn same_qubit_two_operand_operation_is_rejected() {
        let topology = line_topology();

        let mapping =
            QubitMapping::from_pairs(
                [(
                    LogicalQubitId::new(0),
                    PhysicalQubitId::new(0),
                )],
                &topology,
            )
            .expect("mapping must be valid");

        let operation =
            QuantumOperation::two_qubit(
                "cx",
                LogicalQubitId::new(0),
                LogicalQubitId::new(0),
            );

        assert!(
            operation.is_err()
        );

        let _ = mapping;
    }

    // -------------------------------------------------------------------------
    // Unsupported arity
    // -------------------------------------------------------------------------

    #[test]
    fn basic_router_rejects_unlowered_multi_qubit_operations() {
        let topology = line_topology();

        let mapping =
            QubitMapping::from_pairs(
                [
                    (
                        LogicalQubitId::new(0),
                        PhysicalQubitId::new(0),
                    ),
                    (
                        LogicalQubitId::new(1),
                        PhysicalQubitId::new(1),
                    ),
                    (
                        LogicalQubitId::new(2),
                        PhysicalQubitId::new(2),
                    ),
                ],
                &topology,
            )
            .expect("mapping must be valid");

        let operation =
            QuantumOperation::multi_qubit(
                "ccx",
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                    LogicalQubitId::new(2),
                ],
            )
            .expect("operation construction must succeed");

        let config =
            default_config();

        let input =
            RoutingInput::new(
                &[operation],
                &topology,
                &mapping,
                &config,
            )
            .expect("input should be structurally valid");

        let error =
            BasicRouter::new()
                .route(&input)
                .expect_err(
                    "basic router must reject unlowered three-qubit operations",
                );

        assert!(
            matches!(
                error,
                RoutingError::UnsupportedArity { .. }
                    | RoutingError::InvalidQuantumOperation { .. }
            )
        );
    }

    // -------------------------------------------------------------------------
    // Candidate canonicalization
    // -------------------------------------------------------------------------

    #[test]
    fn canonical_edge_is_direction_independent() {
        let a =
            PhysicalQubitId::new(2);

        let b =
            PhysicalQubitId::new(5);

        assert_eq!(
            canonical_edge(a, b),
            canonical_edge(b, a)
        );
    }
}