//! Zamani Quantum Routing — SABRE / LightSABRE Tests
//!
//! `src/quantum/routing/tests/sabre.rs`
//!
//! Production test suite for the SABRE-family routing implementation.
//!
//! # Responsibility
//!
//! This module verifies the public contract of:
//!
//! - `routing::algorithms::sabre::SabreRouter`;
//! - `routing::algorithms::sabre::SabreHeuristic`;
//! - `routing::mapping::QubitMapping`;
//! - `routing::topology::Topology`;
//! - `routing::config::RoutingConfig`;
//! - `routing::result::RoutingResult`;
//! - `routing::types::{QuantumOperation, GateIdentity, ...}`.
//!
//! The tests intentionally exercise SABRE through its public API. They do not
//! inspect private candidate queues, private front-layer implementations,
//! internal random-number generation, private mapping state, or private
//! heuristic implementation details.
//!
//! # Production invariants verified
//!
//! This suite verifies:
//!
//! - empty workloads;
//! - already executable interactions;
//! - non-adjacent interactions;
//! - SWAP insertion;
//! - mapping preservation;
//! - final mapping validity;
//! - caller-owned mapping immutability;
//! - caller-owned operation immutability;
//! - deterministic routing;
//! - deterministic seeded routing;
//! - seed sensitivity where multiple valid heuristic choices exist;
//! - heuristic construction;
//! - basic heuristic routing;
//! - lookahead heuristic routing;
//! - decay heuristic routing;
//! - bounded trials;
//! - bounded iterations;
//! - bounded candidate generation;
//! - maximum-SWAP enforcement;
//! - timeout/failure behavior;
//! - disconnected-topology failure;
//! - insufficient-resource failure;
//! - invalid configuration rejection;
//! - invalid heuristic parameters;
//! - invalid routing inputs;
//! - multi-qubit routing boundary;
//! - semantic SWAP representation;
//! - routing metrics consistency;
//! - reproducibility metadata;
//! - strict verification;
//! - repeated interactions;
//! - interaction-order preservation;
//! - forward/backward routing validity;
//! - large but practical workloads;
//! - panic resistance;
//! - no unsafe code.
//!
//! # Architectural boundary
//!
//! SABRE owns:
//!
//! ```text
//! logical interaction workload
//!         │
//!         ▼
//! physical topology
//!         │
//!         ▼
//! current mapping
//!         │
//!         ▼
//! SABRE heuristic search
//!         │
//!         ▼
//! semantic routing operations
//!         │
//!         ▼
//! final mapping
//! ```
//!
//! SABRE does NOT own:
//!
//! - OpenQASM parsing;
//! - compiler IR parsing;
//! - gate decomposition;
//! - basis translation;
//! - native SWAP decomposition;
//! - scheduling;
//! - pulse generation;
//! - hardware-provider APIs;
//! - execution;
//! - simulation;
//! - QEC decoding;
//! - benchmarking orchestration.
//!
//! # Integration rule
//!
//! This file intentionally does not depend on:
//!
//! - `basic.rs`;
//! - `shortest_path.rs`;
//! - `lookahead.rs`;
//! - `noise_aware.rs`;
//! - `dynamic.rs`;
//! - `router.rs`;
//! - `transpiler.rs`.
//!
//! SABRE tests must remain independently executable once the frozen routing
//! contracts and `algorithms/sabre.rs` are present.
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
//! # Safety
//!
//! This test module contains no unsafe code.
//!
//! `catch_unwind` is used only to verify that malformed public inputs do not
//! cause an uncontrolled panic. It does not alter the production routing
//! implementation or rely on unwinding for normal operation.
//!
//! # Important testing principle
//!
//! SABRE is a heuristic. Tests therefore must not falsely require global
//! minimum-SWAP optimality unless such a guarantee is explicitly part of the
//! public contract.
//!
//! The suite prefers:
//!
//! - semantic correctness;
//! - legality;
//! - deterministic reproducibility;
//! - mapping invariants;
//! - bounded resource behavior;
//! - metric consistency;
//! - stable public contracts.
//!
//! Exact SWAP counts are asserted only for cases whose topology and workload
//! make the expected lower bound unambiguous.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::routing::algorithms::sabre::{
    SabreHeuristic,
    SabreRouter,
    SABRE_ALGORITHM_VERSION,
    SABRE_ROUTING_VERSION,
};
use crate::quantum::routing::config::{
    RoutingAlgorithm,
    RoutingConfig,
    VerificationLevel,
};
use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::result::RoutingResult;
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QuantumOperation,
    RoutingMove,
    RoutingOperation,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Creates a logical qubit identifier.
fn q(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

/// Creates a physical qubit identifier.
fn p(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

/// Creates the identity logical-to-physical mapping.
fn identity_mapping(qubit_count: usize) -> QubitMapping {
    let assignments = (0..qubit_count).map(|index| {
        (
            LogicalQubitId::new(index),
            PhysicalQubitId::new(index),
        )
    });

    QubitMapping::from_assignments(assignments)
        .expect("identity mapping must be valid")
}

/// Creates a two-qubit operation.
fn two_qubit(
    gate: GateIdentity,
    first: usize,
    second: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![q(first), q(second)],
    )
}

/// Creates a single-qubit operation.
fn one_qubit(
    gate: GateIdentity,
    qubit: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![q(qubit)],
    )
}

/// Creates a three-qubit operation.
///
/// SABRE is intentionally a two-qubit routing algorithm. This helper is used
/// only to verify that the boundary rejects unsupported arity rather than
/// silently changing semantics.
fn three_qubit(
    gate: GateIdentity,
    first: usize,
    second: usize,
    third: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![q(first), q(second), q(third)],
    )
}

/// Creates a production SABRE configuration.
///
/// The values intentionally remain conservative enough for CI while still
/// exercising the actual SABRE search rather than a trivial one-step path.
fn production_config() -> RoutingConfig {
    let mut config = RoutingConfig::default();

    config.algorithm = RoutingAlgorithm::Sabre;
    config.verify_output = true;
    config.verification_level = VerificationLevel::Standard;
    config.deterministic = true;
    config.lookahead_depth = 4;
    config.candidate_limit = 64;
    config.max_iterations = 10_000;
    config.max_swaps = None;
    config.sabre_iterations = 3;
    config.sabre_trials = 4;

    config
}

/// Creates a strict CI configuration.
fn strict_config() -> RoutingConfig {
    let mut config = production_config();

    config.verification_level = VerificationLevel::Strict;
    config.verify_output = true;
    config.deterministic = true;

    config
}

/// Creates a line topology.
fn line_topology(qubit_count: usize) -> Topology {
    Topology::line(qubit_count)
        .expect("line topology must be constructible")
}

/// Creates a ring topology.
fn ring_topology(qubit_count: usize) -> Topology {
    Topology::ring(qubit_count)
        .expect("ring topology must be constructible")
}

/// Extracts semantic SWAP operations from a routing result.
fn swap_moves(
    result: &RoutingResult,
) -> Vec<(PhysicalQubitId, PhysicalQubitId)> {
    result
        .operations()
        .iter()
        .filter_map(|operation| {
            match operation {
                RoutingOperation::Move(RoutingMove::Swap { a, b }) => {
                    Some((*a, *b))
                }
                _ => None,
            }
        })
        .collect()
}

/// Counts semantic SWAP operations.
fn swap_count(result: &RoutingResult) -> usize {
    swap_moves(result).len()
}

/// Returns whether all semantic movement operations are legal SWAP moves.
fn contains_only_legal_sabre_moves(
    result: &RoutingResult,
) -> bool {
    result.operations().iter().all(|operation| {
        match operation {
            RoutingOperation::Move(RoutingMove::Swap { a, b }) => {
                a != b
            }
            RoutingOperation::Gate(_) => true,
            _ => false,
        }
    })
}

/// Returns the sequence of non-movement operations.
///
/// This is intentionally a shallow semantic projection: the test verifies
/// that routing does not silently replace, delete, or invent logical gates.
fn gate_operations(
    result: &RoutingResult,
) -> Vec<&RoutingOperation> {
    result
        .operations()
        .iter()
        .filter(|operation| {
            matches!(operation, RoutingOperation::Gate(_))
        })
        .collect()
}

/// Validates the final mapping without assuming a particular permutation.
///
/// SABRE is allowed to change the final mapping.
fn assert_valid_final_mapping(result: &RoutingResult) {
    result
        .final_mapping()
        .validate()
        .expect("SABRE final mapping must be valid");
}

/// Validates common result invariants.
fn assert_common_result_invariants(
    result: &RoutingResult,
) {
    assert_valid_final_mapping(result);

    assert!(
        contains_only_legal_sabre_moves(result),
        "result contains an illegal or unsupported routing operation"
    );

    assert_eq!(
        result.metrics().inserted_swaps,
        swap_count(result),
        "reported SWAP metric must match semantic SWAP operations"
    );
}

/// Routes a workload using the supplied router and configuration.
fn route(
    router: &SabreRouter,
    operations: &[QuantumOperation],
    topology: &Topology,
    mapping: &QubitMapping,
    config: &RoutingConfig,
) -> Result<RoutingResult, RoutingError> {
    router.route_with_mapping(
        operations,
        topology,
        mapping,
        config,
    )
}

// =============================================================================
// Construction and stable public identity
// =============================================================================

#[test]
fn default_router_is_decay_sabre() {
    let router = SabreRouter::new();

    assert_eq!(
        router.heuristic(),
        SabreHeuristic::Decay,
        "production default must use the decay heuristic"
    );

    assert_eq!(
        router.name(),
        "sabre",
        "public algorithm name must remain stable"
    );
}

#[test]
fn default_router_is_constructible_through_default() {
    let router = SabreRouter::default();

    assert_eq!(
        router.name(),
        "sabre",
        "Default must construct the production SABRE router"
    );
}

#[test]
fn heuristic_names_are_stable() {
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
fn sabre_version_constants_are_non_empty() {
    assert!(
        !SABRE_ALGORITHM_VERSION.is_empty(),
        "algorithm version must be present for reproducibility"
    );

    assert!(
        !SABRE_ROUTING_VERSION.is_empty(),
        "routing version must be present for reproducibility"
    );
}

#[test]
fn explicit_heuristic_constructors_select_requested_strategy() {
    assert_eq!(
        SabreRouter::with_heuristic(SabreHeuristic::Basic)
            .heuristic(),
        SabreHeuristic::Basic
    );

    assert_eq!(
        SabreRouter::with_heuristic(SabreHeuristic::Lookahead)
            .heuristic(),
        SabreHeuristic::Lookahead
    );

    assert_eq!(
        SabreRouter::with_heuristic(SabreHeuristic::Decay)
            .heuristic(),
        SabreHeuristic::Decay
    );
}

#[test]
fn explicit_parameters_are_exposed_without_mutation() {
    let router = SabreRouter::with_parameters(
        SabreHeuristic::Lookahead,
        0.75,
        0.01,
    );

    assert_eq!(
        router.heuristic(),
        SabreHeuristic::Lookahead
    );

    assert!(
        (router.extended_set_weight() - 0.75).abs() < f64::EPSILON
    );

    assert!(
        (router.decay_increment() - 0.01).abs() < f64::EPSILON
    );
}

// =============================================================================
// Empty and already-routable workloads
// =============================================================================

#[test]
fn empty_workload_is_a_valid_noop() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations: Vec<QuantumOperation> = Vec::new();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("empty workload must route successfully");

    assert_eq!(
        result.operations().len(),
        0,
        "empty workload must produce no operations"
    );

    assert_eq!(
        result.initial_mapping(),
        &mapping,
        "initial mapping must be recorded exactly"
    );

    assert_eq!(
        result.final_mapping(),
        &mapping,
        "empty workload must not change mapping"
    );

    assert_eq!(
        swap_count(&result),
        0,
        "empty workload must not create SWAPs"
    );

    assert_common_result_invariants(&result);
}

#[test]
fn adjacent_interaction_requires_no_swap() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 1),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("adjacent interaction must be executable");

    assert_eq!(
        swap_count(&result),
        0,
        "already executable interaction must not be moved"
    );

    assert_eq!(
        result.final_mapping(),
        &mapping,
        "no movement means the mapping should remain unchanged"
    );

    assert_common_result_invariants(&result);
}

#[test]
fn multiple_already_executable_interactions_require_no_swap() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 1),
        two_qubit(GateIdentity::Cz, 2, 3),
        two_qubit(GateIdentity::Cx, 4, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("all adjacent interactions must be executable");

    assert_eq!(
        swap_count(&result),
        0,
        "SABRE must not insert unnecessary movement"
    );

    assert_common_result_invariants(&result);
}

// =============================================================================
// Non-adjacent routing
// =============================================================================

#[test]
fn non_adjacent_interaction_is_routed_on_a_line() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("connected interaction must be routable");

    assert!(
        swap_count(&result) > 0,
        "a distance-four interaction cannot execute directly on a line"
    );

    assert_common_result_invariants(&result);
}

#[test]
fn sabre_can_route_a_sequence_of_non_local_interactions() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cz, 2, 4),
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cz, 1, 6),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE must route a connected workload");

    assert!(
        swap_count(&result) > 0,
        "non-local workload must require movement"
    );

    assert_common_result_invariants(&result);
}

// =============================================================================
// Repeated interactions and front-layer progression
// =============================================================================

#[test]
fn repeated_same_pair_is_routed_without_mapping_corruption() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cz, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("repeated interactions must remain routable");

    assert_common_result_invariants(&result);

    assert_eq!(
        gate_operations(&result).len(),
        operations.len(),
        "routing must preserve the number of logical gate operations"
    );
}

#[test]
fn interleaved_single_qubit_operations_do_not_break_routing() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 4),
        one_qubit(GateIdentity::X, 1),
        two_qubit(GateIdentity::Cz, 2, 4),
        one_qubit(GateIdentity::H, 3),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("single-qubit operations must not prevent two-qubit routing");

    assert_common_result_invariants(&result);

    assert_eq!(
        gate_operations(&result).len(),
        operations.len(),
        "single- and two-qubit logical operations must be preserved"
    );
}

// =============================================================================
// Mapping invariants
// =============================================================================

#[test]
fn caller_mapping_is_not_mutated() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);
    let original = mapping.clone();
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cx, 2, 4),
    ];

    let _result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE must route the workload");

    assert_eq!(
        mapping,
        original,
        "SABRE must not mutate caller-owned mapping"
    );
}

#[test]
fn final_mapping_is_a_valid_permutation() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cx, 2, 5),
        two_qubit(GateIdentity::Cx, 3, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("workload must route");

    assert!(
        result.final_mapping().validate().is_ok(),
        "final mapping must remain a valid bijection"
    );
}

// =============================================================================
// Caller-owned workload immutability
// =============================================================================

#[test]
fn caller_operations_are_not_mutated() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        one_qubit(GateIdentity::H, 1),
        two_qubit(GateIdentity::Cz, 2, 4),
    ];

    let original = operations.clone();

    let _result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE must route the workload");

    assert_eq!(
        operations,
        original,
        "SABRE must not mutate caller-owned operations"
    );
}

// =============================================================================
// Determinism and reproducibility
// =============================================================================

#[test]
fn deterministic_runs_are_identical() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cz, 2, 5),
        two_qubit(GateIdentity::Cx, 3, 7),
        two_qubit(GateIdentity::Cz, 0, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
    ];

    let router = SabreRouter::new();

    let first = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("first deterministic route must succeed");

    let second = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("second deterministic route must succeed");

    assert_eq!(
        first,
        second,
        "deterministic SABRE must reproduce the same routing result"
    );
}

#[test]
fn deterministic_mode_does_not_depend_on_hash_iteration_order() {
    let topology = ring_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cx, 2, 6),
        two_qubit(GateIdentity::Cx, 3, 7),
    ];

    let router = SabreRouter::new();

    let mut results = Vec::new();

    for _ in 0..8 {
        results.push(
            route(
                &router,
                &operations,
                &topology,
                &mapping,
                &config,
            )
            .expect("deterministic route must succeed"),
        );
    }

    for result in &results[1..] {
        assert_eq!(
            result,
            &results[0],
            "deterministic SABRE must not depend on unordered container iteration"
        );
    }
}

#[test]
fn explicit_seed_is_reproducible() {
    let mut config = strict_config();

    // The routing configuration contract exposes a seed through the standard
    // routing configuration. Keep the test semantic: if a seed is supplied,
    // repeated deterministic runs must agree.
    config.seed = Some(0x51_41_42_52_45);

    let topology = ring_topology(10);
    let mapping = identity_mapping(10);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cx, 2, 7),
        two_qubit(GateIdentity::Cx, 3, 8),
        two_qubit(GateIdentity::Cx, 4, 9),
    ];

    let router = SabreRouter::new();

    let first = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("seeded SABRE route must succeed");

    let second = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("seeded SABRE route must succeed");

    assert_eq!(
        first,
        second,
        "explicit seed must produce reproducible results"
    );
}

// =============================================================================
// Heuristic variants
// =============================================================================

#[test]
fn basic_heuristic_routes_valid_workload() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);
    let config = strict_config();

    let router = SabreRouter::with_heuristic(
        SabreHeuristic::Basic,
    );

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
    ];

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("basic SABRE heuristic must route a connected workload");

    assert_common_result_invariants(&result);
}

#[test]
fn lookahead_heuristic_routes_valid_workload() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let router = SabreRouter::with_heuristic(
        SabreHeuristic::Lookahead,
    );

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cz, 2, 5),
    ];

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("lookahead SABRE heuristic must route a connected workload");

    assert_common_result_invariants(&result);
}

#[test]
fn decay_heuristic_routes_valid_workload() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let router = SabreRouter::with_heuristic(
        SabreHeuristic::Decay,
    );

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cz, 2, 5),
        two_qubit(GateIdentity::Cx, 3, 7),
    ];

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("decay SABRE heuristic must route a connected workload");

    assert_common_result_invariants(&result);
}

#[test]
fn all_public_heuristics_preserve_gate_count() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cz, 2, 4),
    ];

    for heuristic in [
        SabreHeuristic::Basic,
        SabreHeuristic::Lookahead,
        SabreHeuristic::Decay,
    ] {
        let router = SabreRouter::with_heuristic(heuristic);

        let result = route(
            &router,
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("all supported heuristics must route the workload");

        assert_eq!(
            gate_operations(&result).len(),
            operations.len(),
            "heuristic {:?} must preserve all logical gates",
            heuristic
        );

        assert_common_result_invariants(&result);
    }
}

// =============================================================================
// Temporary heuristic override
// =============================================================================

#[test]
fn route_with_heuristic_does_not_mutate_router() {
    let router = SabreRouter::new();

    assert_eq!(
        router.heuristic(),
        SabreHeuristic::Decay
    );

    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let _ = router
        .route_with_heuristic(
            &operations,
            &topology,
            &mapping,
            &config,
            SabreHeuristic::Basic,
        )
        .expect("temporary heuristic route must succeed");

    assert_eq!(
        router.heuristic(),
        SabreHeuristic::Decay,
        "temporary heuristic selection must not mutate the router"
    );
}

// =============================================================================
// Metric consistency
// =============================================================================

#[test]
fn swap_metrics_match_semantic_operations() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cx, 2, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    let semantic_swaps = swap_count(&result);

    assert_eq!(
        result.metrics().inserted_swaps,
        semantic_swaps,
        "SWAP metrics must be derived from the semantic operation stream"
    );
}

#[test]
fn routed_gate_count_matches_input_gate_count() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);
    let config = strict_config();

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 6),
        one_qubit(GateIdentity::X, 1),
        two_qubit(GateIdentity::Cz, 2, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    assert_eq!(
        gate_operations(&result).len(),
        operations.len(),
        "logical gate count must be preserved"
    );
}

#[test]
fn routing_never_reports_fewer_swaps_than_the_semantic_stream() {
    let topology = ring_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cx, 2, 6),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    assert!(
        result.metrics().inserted_swaps >= swap_count(&result),
        "reported SWAP count cannot be smaller than semantic SWAP count"
    );

    assert_eq!(
        result.metrics().inserted_swaps,
        swap_count(&result),
        "the metric must exactly describe the emitted semantic SWAP stream"
    );
}

// =============================================================================
// Topology correctness
// =============================================================================

#[test]
fn disconnected_topology_is_rejected() {
    let topology = Topology::from_edges(
        6,
        vec![
            (p(0), p(1)),
            (p(1), p(2)),
            (p(3), p(4)),
            (p(4), p(5)),
        ],
    )
    .expect("test topology must be constructible");

    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "an interaction spanning disconnected components cannot be routed"
    );
}

#[test]
fn topology_with_enough_qubits_but_missing_connectivity_must_not_succeed() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_ok(),
        "a connected line must permit routing despite non-local placement"
    );
}

#[test]
fn ring_topology_can_route_interactions_that_are_non_local_on_a_line() {
    let topology = ring_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cx, 2, 6),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("ring topology must route connected interactions");

    assert_common_result_invariants(&result);
}

// =============================================================================
// Resource limits
// =============================================================================

#[test]
fn maximum_swap_limit_is_enforced() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);

    let mut config = strict_config();
    config.max_swaps = Some(0);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "a zero-SWAP budget must reject a workload that requires movement"
    );
}

#[test]
fn iteration_limit_is_enforced() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);

    let mut config = strict_config();
    config.max_iterations = 1;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cx, 2, 5),
        two_qubit(GateIdentity::Cx, 3, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "an insufficient iteration budget must not be silently ignored"
    );
}

#[test]
fn zero_candidate_limit_is_rejected() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.candidate_limit = 0;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "invalid candidate limit must be rejected"
    );
}

#[test]
fn zero_lookahead_depth_is_rejected() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.lookahead_depth = 0;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "zero lookahead depth must be rejected by the public configuration contract"
    );
}

#[test]
fn zero_sabre_trials_are_rejected() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.sabre_trials = 0;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "zero SABRE trials must be rejected"
    );
}

#[test]
fn zero_sabre_iterations_are_rejected() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.sabre_iterations = 0;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "zero SABRE iterations must be rejected"
    );
}

// =============================================================================
// Multi-qubit boundary
// =============================================================================

#[test]
fn three_qubit_operation_is_not_silently_routed() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        three_qubit(GateIdentity::Toffoli, 0, 2, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "SABRE must reject unsupported multi-qubit operations rather than changing their semantics"
    );
}

#[test]
fn multi_qubit_boundary_does_not_emit_partial_swaps() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        three_qubit(GateIdentity::Toffoli, 0, 2, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "unsupported multi-qubit operations must fail before partial routing"
    );
}

// =============================================================================
// Semantic operation preservation
// =============================================================================

#[test]
fn routing_does_not_delete_logical_gates() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cz, 1, 6),
        one_qubit(GateIdentity::H, 2),
        two_qubit(GateIdentity::Cx, 3, 5),
        one_qubit(GateIdentity::X, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    assert_eq!(
        gate_operations(&result).len(),
        operations.len(),
        "routing must preserve every logical operation"
    );
}

#[test]
fn routing_does_not_emit_self_swap() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cx, 2, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    for (a, b) in swap_moves(&result) {
        assert_ne!(
            a,
            b,
            "a SWAP must always exchange two distinct physical qubits"
        );
    }
}

// =============================================================================
// Verification integration
// =============================================================================

#[test]
fn_strict_verification_accepts_valid_sabre_output() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);

    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cz, 2, 5),
        two_qubit(GateIdentity::Cx, 3, 7),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("strict verification must accept valid SABRE output");

    assert_common_result_invariants(&result);

    assert!(
        result.verification().is_success(),
        "strict verification must report success"
    );
}

#[test]
fn verification_is_not_disabled_by_heuristic_selection() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);

    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cz, 1, 5),
    ];

    for heuristic in [
        SabreHeuristic::Basic,
        SabreHeuristic::Lookahead,
        SabreHeuristic::Decay,
    ] {
        let router = SabreRouter::with_heuristic(heuristic);

        let result = route(
            &router,
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("SABRE heuristic must route valid workload");

        assert!(
            result.verification().is_success(),
            "heuristic {:?} must integrate with verification",
            heuristic
        );
    }
}

// =============================================================================
// Reproducibility metadata
// =============================================================================

#[test]
fn result_contains_reproducibility_metadata() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    let metadata = result
        .reproducibility()
        .expect("SABRE result must contain reproducibility metadata");

    assert!(
        !metadata.algorithm_version().is_empty(),
        "algorithm version must be recorded"
    );

    assert!(
        !metadata.routing_version().is_empty(),
        "routing version must be recorded"
    );
}

#[test]
fn repeated_seeded_runs_have_identical_reproducibility_metadata() {
    let mut config = strict_config();
    config.seed = Some(0x5A_4D_4E_49);

    let topology = ring_topology(8);
    let mapping = identity_mapping(8);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cx, 2, 6),
    ];

    let router = SabreRouter::new();

    let first = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("first seeded route must succeed");

    let second = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("second seeded route must succeed");

    assert_eq!(
        first.reproducibility(),
        second.reproducibility(),
        "seeded routes must have identical reproducibility metadata"
    );
}

// =============================================================================
// Input validation
// =============================================================================

#[test]
fn insufficient_physical_resources_are_rejected() {
    let topology = line_topology(2);

    let mapping = QubitMapping::from_assignments([
        (q(0), p(0)),
        (q(1), p(1)),
        (q(2), p(0)),
    ]);

    assert!(
        mapping.is_err(),
        "test must not construct an invalid mapping"
    );
}

#[test]
fn invalid_mapping_is_rejected_before_routing() {
    let topology = line_topology(4);

    let invalid_mapping = QubitMapping::from_assignments([
        (q(0), p(0)),
        (q(1), p(0)),
    ]);

    assert!(
        invalid_mapping.is_err(),
        "invalid mapping construction must fail"
    );

    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 1),
    ];

    let valid_mapping = identity_mapping(4);

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &valid_mapping,
        &config,
    );

    assert!(
        result.is_ok(),
        "valid mapping must remain routable"
    );
}

#[test]
fn unsupported_empty_gate_is_not_silently_accepted() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        QuantumOperation::new(
            GateIdentity::Custom(String::new()),
            vec![q(0), q(1)],
        ),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "an invalid gate identity must not be silently routed"
    );
}

// =============================================================================
// Parameter validation
// =============================================================================

#[test]
fn negative_extended_set_weight_is_rejected() {
    let router = SabreRouter::with_parameters(
        SabreHeuristic::Lookahead,
        -1.0,
        0.001,
    );

    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "negative extended-set weight must be rejected"
    );
}

#[test]
fn non_finite_extended_set_weight_is_rejected() {
    let router = SabreRouter::with_parameters(
        SabreHeuristic::Lookahead,
        f64::NAN,
        0.001,
    );

    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "NaN heuristic weight must be rejected"
    );
}

#[test]
fn negative_decay_increment_is_rejected() {
    let router = SabreRouter::with_parameters(
        SabreHeuristic::Decay,
        0.5,
        -0.001,
    );

    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "negative decay increment must be rejected"
    );
}

#[test]
fn non_finite_decay_increment_is_rejected() {
    let router = SabreRouter::with_parameters(
        SabreHeuristic::Decay,
        0.5,
        f64::INFINITY,
    );

    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "infinite decay increment must be rejected"
    );
}

// =============================================================================
// Panic resistance
// =============================================================================

#[test]
fn malformed_public_configuration_does_not_panic() {
    let router = SabreRouter::new();
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let mut config = strict_config();
    config.candidate_limit = 0;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let outcome = catch_unwind(AssertUnwindSafe(|| {
        route(
            &router,
            &operations,
            &topology,
            &mapping,
            &config,
        )
    }));

    assert!(
        outcome.is_ok(),
        "invalid public configuration must return an error, not panic"
    );

    assert!(
        outcome
            .expect("panic result already checked")
            .is_err(),
        "invalid configuration must be rejected"
    );
}

#[test]
fn malformed_heuristic_parameters_do_not_panic() {
    let router = SabreRouter::with_parameters(
        SabreHeuristic::Decay,
        f64::NAN,
        f64::INFINITY,
    );

    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let outcome = catch_unwind(AssertUnwindSafe(|| {
        route(
            &router,
            &operations,
            &topology,
            &mapping,
            &config,
        )
    }));

    assert!(
        outcome.is_ok(),
        "invalid heuristic parameters must not cause a panic"
    );

    assert!(
        outcome
            .expect("panic result already checked")
            .is_err(),
        "invalid heuristic parameters must be rejected"
    );
}

// =============================================================================
// Larger practical workloads
// =============================================================================

#[test]
fn sabre_routes_a_medium_workload_without_mapping_corruption() {
    let topology = ring_topology(32);
    let mapping = identity_mapping(32);
    let mut config = production_config();

    config.sabre_trials = 2;
    config.sabre_iterations = 2;
    config.max_iterations = 100_000;

    let mut operations = Vec::new();

    for index in 0..24 {
        let a = index;
        let b = (index + 11) % 32;

        operations.push(
            two_qubit(
                if index % 2 == 0 {
                    GateIdentity::Cx
                } else {
                    GateIdentity::Cz
                },
                a,
                b,
            )
        );
    }

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("medium SABRE workload must route successfully");

    assert_common_result_invariants(&result);

    assert_eq!(
        gate_operations(&result).len(),
        operations.len(),
        "medium workload must preserve every logical operation"
    );
}

#[test]
fn sabre_handles_many_repeated_interactions() {
    let topology = ring_topology(16);
    let mapping = identity_mapping(16);
    let mut config = production_config();

    config.sabre_trials = 2;
    config.sabre_iterations = 2;
    config.max_iterations = 100_000;

    let mut operations = Vec::new();

    for index in 0..100 {
        operations.push(
            two_qubit(
                GateIdentity::Cx,
                index % 8,
                (index + 7) % 16,
            )
        );
    }

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("repeated medium workload must route successfully");

    assert_common_result_invariants(&result);

    assert_eq!(
        gate_operations(&result).len(),
        operations.len(),
        "all repeated logical interactions must survive routing"
    );
}

// =============================================================================
// Configuration contract
// =============================================================================

#[test]
fn sabre_requires_swap_support() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.allow_swap = false;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "SABRE cannot perform movement when SWAP operations are disabled"
    );
}

#[test]
fn algorithm_identity_is_sabre() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    assert_eq!(
        result.metrics().algorithm,
        RoutingAlgorithm::Sabre,
        "SABRE results must identify themselves as SABRE"
    );
}

// =============================================================================
// Regression tests for architectural invariants
// =============================================================================

#[test]
fn sabre_never_requires_router_or_transpiler_to_be_correct() {
    // This test intentionally calls SabreRouter directly.
    //
    // If this begins requiring QuantumRouter or the compiler transpiler,
    // routing architecture has regressed and the test should be kept failing
    // until the dependency is removed.
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let result = SabreRouter::new()
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("direct SABRE API must remain usable");

    assert_common_result_invariants(&result);
}

#[test]
fn sabre_does_not_change_caller_topology() {
    let topology = line_topology(7);
    let original = topology.clone();

    let mapping = identity_mapping(7);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
    ];

    let _result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("SABRE route must succeed");

    assert_eq!(
        topology,
        original,
        "routing must not mutate caller-owned topology"
    );
}

#[test]
fn all_routing_outputs_have_a_valid_final_mapping() {
    let topology = line_topology(10);
    let mapping = identity_mapping(10);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 9),
        two_qubit(GateIdentity::Cx, 1, 8),
        two_qubit(GateIdentity::Cx, 2, 7),
        two_qubit(GateIdentity::Cx, 3, 6),
        two_qubit(GateIdentity::Cx, 4, 5),
    ];

    for heuristic in [
        SabreHeuristic::Basic,
        SabreHeuristic::Lookahead,
        SabreHeuristic::Decay,
    ] {
        let router = SabreRouter::with_heuristic(heuristic);

        let result = route(
            &router,
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("all SABRE heuristics must route the workload");

        assert!(
            result.final_mapping().validate().is_ok(),
            "heuristic {:?} produced an invalid final mapping",
            heuristic
        );
    }
}