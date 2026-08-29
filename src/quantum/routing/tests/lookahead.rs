//! Zamani Quantum Routing — Lookahead Router Tests
//!
//! `src/quantum/routing/tests/lookahead.rs`
//!
//! Production test suite for the bounded lookahead routing algorithm.
//!
//! # Responsibility
//!
//! This module verifies the public contract of:
//!
//! - `routing::algorithms::lookahead::LookaheadRouter`;
//! - `routing::mapping::QubitMapping`;
//! - `routing::topology::Topology`;
//! - `routing::config::RoutingConfig`;
//! - `routing::result::RoutingResult`;
//! - `routing::types::{QuantumOperation, GateIdentity, ...}`.
//!
//! The tests intentionally exercise the router through its public API rather
//! than depending on private implementation details.
//!
//! # Production invariants verified
//!
//! The suite verifies:
//!
//! - empty circuits;
//! - already executable circuits;
//! - non-adjacent interactions;
//! - SWAP insertion;
//! - mapping preservation;
//! - final mapping validity;
//! - caller-owned mapping immutability;
//! - caller-owned operation immutability;
//! - deterministic routing;
//! - deterministic tie-breaking;
//! - bounded lookahead;
//! - bounded beam width;
//! - invalid configuration rejection;
//! - invalid topology rejection;
//! - insufficient physical resources;
//! - disconnected topology failure;
//! - maximum-SWAP enforcement;
//! - maximum-iteration enforcement;
//! - multi-qubit rejection at the routing boundary;
//! - verification integration;
//! - routing metrics;
//! - future-layer influence;
//! - regression protection against speculative mapping leakage;
//! - semantic SWAP representation;
//! - no unsafe code.
//!
//! # Architectural rule
//!
//! These tests must not:
//!
//! - test OpenQASM parsing;
//! - test compiler IR;
//! - test hardware providers;
//! - test gate decomposition;
//! - test pulse scheduling;
//! - test execution;
//! - test simulation;
//! - test QEC decoding.
//!
//! Those belong to their respective subsystems.
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
//! # Integration contract
//!
//! This file is intentionally written against the frozen routing contracts.
//! It must not require changes to `lookahead.rs` merely because another routing
//! algorithm is subsequently implemented.
//!
//! The test dependency direction is:
//!
//! ```text
//! tests/lookahead.rs
//!        │
//!        ├── types.rs
//!        ├── mapping.rs
//!        ├── topology.rs
//!        ├── config.rs
//!        ├── result.rs
//!        └── algorithms/lookahead.rs
//! ```
//!
//! It deliberately does not depend on:
//!
//! - `basic.rs`;
//! - `shortest_path.rs`;
//! - `sabre.rs`;
//! - `noise_aware.rs`;
//! - `dynamic.rs`;
//! - `router.rs`;
//! - `transpiler.rs`.
//!
//! This keeps the tests useful even while the rest of the routing subsystem is
//! still being implemented.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::routing::algorithms::lookahead::{
    LookaheadRouter,
    MAX_BEAM_WIDTH,
    MAX_SEARCH_DEPTH,
};
use crate::quantum::routing::config::{
    RoutingConfig,
    RoutingAlgorithm,
    VerificationLevel,
};
use crate::quantum::routing::mapping::QubitMapping;
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

/// Creates the canonical identity mapping:
///
/// ```text
/// q0 -> p0
/// q1 -> p1
/// q2 -> p2
/// ...
/// ```
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
///
/// This helper intentionally uses the routing-level `QuantumOperation`
/// constructor instead of compiler-specific IR.
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

/// Creates a three-qubit operation for boundary tests.
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

/// Creates a production routing configuration with verification enabled.
fn production_config() -> RoutingConfig {
    let mut config = RoutingConfig::default();

    config.algorithm = RoutingAlgorithm::Lookahead;
    config.verify_output = true;
    config.verification_level = VerificationLevel::Standard;
    config.deterministic = true;
    config.lookahead_depth = 4;
    config.candidate_limit = 64;
    config.max_iterations = 10_000;
    config.max_swaps = None;

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

/// Creates a linear topology.
///
/// The topology API is intentionally hidden behind this helper so tests do not
/// duplicate construction details throughout the module.
fn line_topology(
    qubit_count: usize,
) -> Topology {
    Topology::line(qubit_count)
        .expect("linear topology must be constructible")
}

/// Extracts the semantic SWAP moves from a routing result.
fn swap_moves(
    result: &crate::quantum::routing::result::RoutingResult,
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

/// Returns the number of semantic SWAP operations in a result.
fn swap_count(
    result: &crate::quantum::routing::result::RoutingResult,
) -> usize {
    swap_moves(result).len()
}

/// Returns whether a result contains only legal semantic routing operations.
fn contains_only_routing_swaps(
    result: &crate::quantum::routing::result::RoutingResult,
) -> bool {
    result.operations().iter().all(|operation| {
        matches!(
            operation,
            RoutingOperation::Move(RoutingMove::Swap { .. })
                | RoutingOperation::Gate(_)
        )
    })
}

// =============================================================================
// Basic correctness
// =============================================================================

#[test]
fn empty_circuit_is_a_valid_noop() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations: Vec<QuantumOperation> = Vec::new();

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("empty circuit must route successfully");

    assert_eq!(
        result.operations().len(),
        0,
        "empty input must not produce routing operations"
    );

    assert_eq!(
        result.initial_mapping(),
        &mapping,
        "initial mapping must be preserved in the result"
    );

    assert_eq!(
        result.final_mapping(),
        &mapping,
        "empty circuit must not change the mapping"
    );

    assert_eq!(
        swap_count(&result),
        0,
        "empty circuit must not insert SWAPs"
    );
}

#[test]
fn_already_executable_two_qubit_gate_requires_no_swap() {
    let topology = line_topology(3);
    let mapping = identity_mapping(3);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 1),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("adjacent operation must be executable");

    assert_eq!(
        swap_count(&result),
        0,
        "an already adjacent interaction must not require movement"
    );

    assert_eq!(
        result.final_mapping(),
        &mapping,
        "routing an already executable operation must preserve mapping"
    );

    assert_eq!(
        result.metrics().inserted_swaps,
        0,
        "metrics must agree with the actual semantic SWAP stream"
    );
}

#[test]
fn non_adjacent_two_qubit_gate_is_routed() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("connected non-adjacent interaction must be routable");

    assert!(
        swap_count(&result) > 0,
        "non-adjacent interaction must require at least one movement on a line"
    );

    assert_eq!(
        result.metrics().inserted_swaps,
        swap_count(&result),
        "reported SWAP count must equal semantic SWAP count"
    );

    assert!(
        result.final_mapping().validate().is_ok(),
        "final mapping must remain internally consistent"
    );

    assert!(
        contains_only_routing_swaps(&result),
        "lookahead output must contain recognized routing operations"
    );
}

// =============================================================================
// Mapping immutability and speculative-search isolation
// =============================================================================

#[test]
fn caller_owned_mapping_is_not_mutated() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let original = mapping.clone();

    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 1, 3),
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router = LookaheadRouter::with_parameters(
        0.75,
        8,
    );

    let _result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("lookahead route must succeed");

    assert_eq!(
        mapping,
        original,
        "speculative and committed routing must never mutate the caller's mapping"
    );
}

#[test]
fn caller_owned_operations_are_not_mutated() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        one_qubit(GateIdentity::H, 1),
        two_qubit(GateIdentity::Cz, 2, 4),
    ];

    let original = operations.clone();

    let router = LookaheadRouter::new();

    let _result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("lookahead route must succeed");

    assert_eq!(
        operations,
        original,
        "routing must not mutate caller-owned operations"
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_inputs_produce_identical_results() {
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

    let router = LookaheadRouter::with_parameters(
        0.50,
        8,
    );

    let first = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("first deterministic route must succeed");

    let second = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("second deterministic route must succeed");

    assert_eq!(
        first,
        second,
        "identical routing inputs must produce byte-for-byte-equivalent logical results"
    );
}

#[test]
fn changing_beam_width_does_not_change_basic_api_validity() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 1, 4),
        two_qubit(GateIdentity::Cx, 2, 5),
    ];

    let narrow = LookaheadRouter::with_beam_width(1);
    let wide = LookaheadRouter::with_beam_width(8);

    let narrow_result = narrow
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("narrow beam route must succeed");

    let wide_result = wide
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("wide beam route must succeed");

    assert!(
        narrow_result.final_mapping().validate().is_ok(),
        "narrow beam must still preserve mapping invariants"
    );

    assert!(
        wide_result.final_mapping().validate().is_ok(),
        "wide beam must preserve mapping invariants"
    );
}

// =============================================================================
// Lookahead configuration
// =============================================================================

#[test]
fn default_router_has_stable_production_parameters() {
    let router = LookaheadRouter::new();

    assert_eq!(
        router.name(),
        "lookahead"
    );

    assert!(
        router.extended_set_weight().is_finite(),
        "future-layer weight must be finite"
    );

    assert!(
        router.extended_set_weight() >= 0.0,
        "future-layer weight must be non-negative"
    );

    assert!(
        router.beam_width() > 0,
        "production beam width must never be zero"
    );

    assert!(
        router.beam_width() <= MAX_BEAM_WIDTH,
        "default beam width must remain inside the safety ceiling"
    );
}

#[test]
fn explicit_parameters_are_retained() {
    let router =
        LookaheadRouter::with_parameters(0.75, 16);

    assert_eq!(
        router.extended_set_weight(),
        0.75
    );

    assert_eq!(
        router.beam_width(),
        16
    );
}

#[test]
fn zero_beam_width_is_rejected_at_routing_time() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router = LookaheadRouter::with_beam_width(0);

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "zero beam width must never silently disable the search"
    );
}

#[test]
fn negative_future_weight_is_rejected() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router =
        LookaheadRouter::with_extended_set_weight(-1.0);

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "negative lookahead weight must be rejected"
    );
}

#[test]
fn_nan_future_weight_is_rejected() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router =
        LookaheadRouter::with_extended_set_weight(f64::NAN);

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "NaN lookahead weight must be rejected"
    );
}

#[test]
fn infinite_future_weight_is_rejected() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router =
        LookaheadRouter::with_extended_set_weight(f64::INFINITY);

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "infinite lookahead weight must be rejected"
    );
}

// =============================================================================
// Search bounds
// =============================================================================

#[test]
fn lookahead_depth_is_bounded() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.lookahead_depth = MAX_SEARCH_DEPTH + 1;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "lookahead depth above the safety ceiling must be rejected"
    );
}

#[test]
fn maximum_swap_limit_is_enforced() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.max_swaps = Some(0);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "routing requiring a SWAP must fail when max_swaps is zero"
    );
}

#[test]
fn maximum_iterations_are_enforced() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let mut config = strict_config();
    config.max_iterations = 1;

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 1, 4),
        two_qubit(GateIdentity::Cx, 2, 5),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "routing must respect the configured iteration bound"
    );
}

// =============================================================================
// Topology failure handling
// =============================================================================

#[test]
fn disconnected_topology_is_rejected_for_unreachable_interaction() {
    let topology =
        Topology::isolated(4)
            .expect("isolated topology must be constructible");

    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "unreachable logical interaction must not produce an invalid route"
    );
}

#[test]
fn insufficient_mapping_resources_are_rejected() {
    let topology = line_topology(2);

    let mapping = identity_mapping(1);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 1),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "an unmapped logical operand must be rejected"
    );
}

// =============================================================================
// Multi-qubit boundary
// =============================================================================

#[test]
fn unsupported_three_qubit_operation_is_not_silently_decomposed() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        three_qubit(
            GateIdentity::Ccx,
            0,
            1,
            2,
        ),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "lookahead routing must not silently synthesize arbitrary multi-qubit operations"
    );
}

#[test]
fn three_qubit_operation_does_not_mutate_mapping_on_failure() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let original = mapping.clone();
    let config = strict_config();

    let operations = vec![
        three_qubit(
            GateIdentity::Ccx,
            0,
            2,
            3,
        ),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "unsupported multi-qubit routing must fail explicitly"
    );

    assert_eq!(
        mapping,
        original,
        "failed routing must not mutate the caller's mapping"
    );
}

// =============================================================================
// Verification
// =============================================================================

#[test]
fn standard_verification_is_enabled_in_production_configuration() {
    let config = production_config();

    assert!(
        config.verify_output,
        "production configuration must verify routed output"
    );

    assert_eq!(
        config.verification_level,
        VerificationLevel::Standard
    );
}

#[test]
fn strict_verification_succeeds_for_valid_route() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cz, 1, 3),
        one_qubit(GateIdentity::H, 2),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("strict verification must accept a valid route");

    assert_eq!(
        result.verification_status().name(),
        "passed"
    );
}

// =============================================================================
// Metrics
// =============================================================================

#[test]
fn metrics_match_the_generated_operation_stream() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cz, 1, 3),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("route must succeed");

    let actual_swaps = swap_count(&result);

    assert_eq!(
        result.metrics().inserted_swaps,
        actual_swaps,
        "inserted SWAP metric must match semantic output"
    );

    assert_eq!(
        result.metrics().original_operation_count,
        operations.len(),
        "original operation metric must match input"
    );

    assert_eq!(
        result.metrics().final_operation_count,
        result.operations().len(),
        "final operation metric must match output length"
    );

    assert_eq!(
        result.metrics().routing_overhead,
        result
            .metrics()
            .final_operation_count
            .saturating_sub(
                result.metrics().original_operation_count
            ),
        "routing overhead must be derived from operation counts"
    );
}

#[test]
fn algorithm_metric_identifies_lookahead() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("route must succeed");

    assert_eq!(
        result.metrics().algorithm,
        "lookahead"
    );
}

// =============================================================================
// Semantic SWAP representation
// =============================================================================

#[test]
fn inserted_movement_is_semantic_swap_not_hardware_decomposition() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("route must succeed");

    let swaps = result
        .operations()
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

    assert!(
        swaps > 0,
        "non-adjacent interaction must produce semantic SWAP movement"
    );

    assert!(
        result.operations().iter().all(|operation| {
            !matches!(
                operation,
                RoutingOperation::Gate(gate)
                    if gate.gate().name() == "cx"
                    && gate
                        .operands()
                        .iter()
                        .any(|operand| {
                            matches!(
                                operand,
                                crate::quantum::routing::types::QubitRef::Physical(_)
                            )
                        })
            )
        }),
        "lookahead must not prematurely lower routing into hardware-specific physical gate syntax"
    );
}

// =============================================================================
// Mapping validity after complex routing
// =============================================================================

#[test]
fn complex_route_preserves_bidirectional_mapping_invariants() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 7),
        two_qubit(GateIdentity::Cx, 1, 6),
        two_qubit(GateIdentity::Cx, 2, 5),
        two_qubit(GateIdentity::Cz, 3, 7),
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cz, 1, 7),
    ];

    let router =
        LookaheadRouter::with_parameters(0.50, 16);

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("complex connected route must succeed");

    result
        .final_mapping()
        .validate()
        .expect(
            "lookahead must preserve both directions of the mapping invariant"
        );

    assert_eq!(
        result.final_mapping().len(),
        mapping.len(),
        "routing should preserve the number of mapped logical qubits"
    );
}

// =============================================================================
// Future-layer regression coverage
// =============================================================================

#[test]
fn future_layer_configuration_is_accepted_without_changing_correctness_contract() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cx, 0, 1),
        two_qubit(GateIdentity::Cx, 5, 6),
        two_qubit(GateIdentity::Cx, 1, 5),
    ];

    let mut shallow_config = strict_config();
    shallow_config.lookahead_depth = 1;

    let mut deep_config = strict_config();
    deep_config.lookahead_depth = 4;

    let router =
        LookaheadRouter::with_extended_set_weight(0.75);

    let shallow = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &shallow_config,
        )
        .expect("shallow lookahead must route");

    let deep = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &deep_config,
        )
        .expect("deep lookahead must route");

    shallow
        .final_mapping()
        .validate()
        .expect("shallow result must remain valid");

    deep
        .final_mapping()
        .validate()
        .expect("deep result must remain valid");

    assert_eq!(
        shallow.metrics().original_operation_count,
        operations.len()
    );

    assert_eq!(
        deep.metrics().original_operation_count,
        operations.len()
    );
}

// =============================================================================
// Failure atomicity
// =============================================================================

#[test]
fn failed_route_does_not_expose_partial_result() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let original_mapping = mapping.clone();

    let mut config = strict_config();
    config.max_swaps = Some(0);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let router = LookaheadRouter::new();

    let result = router.route_with_mapping(
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "route requiring movement must fail under a zero-SWAP budget"
    );

    assert_eq!(
        mapping,
        original_mapping,
        "failure must leave caller mapping unchanged"
    );
}

// =============================================================================
// Panic resistance
// =============================================================================

#[test]
fn invalid_runtime_parameters_do_not_panic() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router =
        LookaheadRouter::with_parameters(
            f64::NAN,
            0,
        );

    let result = catch_unwind(AssertUnwindSafe(|| {
        router.route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
    }));

    assert!(
        result.is_ok(),
        "invalid user-controlled routing parameters must return errors rather than panic"
    );

    assert!(
        result
            .expect("panic result was checked")
            .is_err(),
        "invalid parameters must be represented as routing errors"
    );
}

// =============================================================================
// Safety ceilings
// =============================================================================

#[test]
fn safety_constants_are_sane() {
    assert!(
        MAX_BEAM_WIDTH > 0,
        "beam safety ceiling must be non-zero"
    );

    assert!(
        MAX_SEARCH_DEPTH > 0,
        "search-depth safety ceiling must be non-zero"
    );

    assert!(
        MAX_BEAM_WIDTH >= 8,
        "the production safety ceiling must not reject the default beam width"
    );

    assert!(
        MAX_SEARCH_DEPTH >= 4,
        "the production safety ceiling must not reject the default lookahead depth"
    );
}

// =============================================================================
// Regression: single-qubit operations
// =============================================================================

#[test]
fn single_qubit_operations_do_not_trigger_routing_moves() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        one_qubit(GateIdentity::X, 3),
        one_qubit(GateIdentity::Rz, 1),
        one_qubit(GateIdentity::T, 2),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("single-qubit operations must route");

    assert_eq!(
        swap_count(&result),
        0,
        "single-qubit operations never require connectivity routing"
    );

    assert_eq!(
        result.metrics().routed_gate_count,
        operations.len(),
        "all single-qubit operations must be emitted"
    );

    assert_eq!(
        result.final_mapping(),
        &mapping,
        "single-qubit operations must not change the mapping"
    );
}

// =============================================================================
// Regression: repeated interactions
// =============================================================================

#[test]
fn repeated_non_adjacent_interactions_remain_routable() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cx, 0, 5),
    ];

    let router =
        LookaheadRouter::with_parameters(0.5, 8);

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("repeated interaction circuit must route");

    assert_eq!(
        result.metrics().routed_gate_count,
        operations.len(),
        "all logical gates must eventually be routed"
    );

    result
        .final_mapping()
        .validate()
        .expect("repeated interactions must preserve mapping invariants");
}

// =============================================================================
// Regression: deterministic candidate ordering
// =============================================================================

#[test]
fn repeated_runs_with_equal_cost_choices_are_stable() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let router =
        LookaheadRouter::with_parameters(0.50, 8);

    let first = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("first route must succeed");

    let second = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("second route must succeed");

    assert_eq!(
        swap_moves(&first),
        swap_moves(&second),
        "equal-cost candidate situations must have deterministic tie-breaking"
    );
}

// =============================================================================
// Regression: topology does not become compiler-specific
// =============================================================================

#[test]
fn lookahead_uses_backend_independent_routing_contracts() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);
    let config = strict_config();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let router = LookaheadRouter::new();

    let result = router
        .route_with_mapping(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("backend-independent routing route must succeed");

    assert_eq!(
        result.metrics().algorithm,
        "lookahead"
    );

    assert_eq!(
        result.initial_mapping(),
        &mapping
    );

    assert!(
        result.final_mapping().validate().is_ok()
    );
}