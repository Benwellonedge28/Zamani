//! Zamani Quantum Routing — End-to-End Production Integration Tests
//!
//! `src/quantum/routing/tests/end_to_end.rs`
//!
//! # Responsibility
//!
//! This file is the highest-level black-box integration suite for the routing
//! subsystem.
//
//! It verifies the complete routing contract:
//
//! ```text
//! logical quantum operations
//!          │
//!          ▼
//! topology validation
//!          │
//!          ▼
//! initial logical → physical mapping
//!          │
//!          ▼
//! SABRE routing algorithm
//!          │
//!          ▼
//! semantic routing operations
//!          │
//!          ▼
//! mapping evolution
//!          │
//!          ▼
//! RoutingResult
//!          │
//!          ▼
//! result invariants / verification
//! ```
//!
//! The test suite deliberately uses only public routing APIs. It does not
//! inspect private SABRE state, private candidate queues, private heuristic
//! calculations, private mapping storage, or implementation-specific search
//! structures.
//!
//! # Architectural boundary
//!
//! This test belongs at the boundary between the independent routing
//! components. Lower-level behavior is tested by the dedicated suites:
//!
//! - `topology.rs`
//! - `mapping.rs`
//! - `shortest_path.rs`
//! - `basic.rs`
//! - `lookahead.rs`
//! - `sabre.rs`
//! - `noise_aware.rs`
//! - `multi_qubit.rs`
//! - `directed.rs`
//! - `transactional.rs`
//! - `verification.rs`
//!
//! This file therefore asks the more important production question:
//!
//! > Do the independently implemented routing components work together as one
//! > coherent routing pipeline?
//!
//! # What is verified
//!
//! The suite covers:
//!
//! 1. topology → mapping → SABRE → result integration;
//! 2. already-routable circuits;
//! 3. connectivity-constrained circuits;
//! 4. semantic SWAP generation;
//! 5. mapping evolution;
//! 6. final mapping validity;
//! 7. original mapping immutability;
//! 8. original operation immutability;
//! 9. logical gate preservation;
//! 10. operation ordering preservation;
//! 11. deterministic routing;
//! 12. deterministic seeded routing;
//! 13. reproducibility metadata;
//! 14. strict verification;
//! 15. empty workloads;
//! 16. repeated interactions;
//! 17. multiple interactions;
//! 18. long-distance interactions;
//! 19. line topology;
//! 20. ring topology;
//! 21. heavy-hex-style topology;
//! 22. disconnected-topology failure;
//! 23. insufficient mapping failure;
//! 24. invalid logical-qubit failure;
//! 25. duplicate logical operands;
//! 26. unsupported multi-qubit routing boundary;
//! 27. bounded routing;
//! 28. result metric consistency;
//! 29. final permutation validity;
//! 30. routing idempotence for already-routable input;
//! 31. repeated independent invocations;
//! 32. panic resistance for malformed public inputs;
//! 33. no unsafe code;
//! 34. safe Rust 1.97/1.97.1 compatibility.
//!
//! # Important semantic rule
//!
//! Routing movement is represented semantically.
//!
//! A:
//!
//! ```text
//! RoutingOperation::Move(RoutingMove::Swap { ... })
//! ```
//!
//! means that the logical states exchange physical locations.
//!
//! It does NOT mean that the target hardware necessarily provides a native
//! SWAP instruction. Hardware decomposition belongs to the later hardware
//! lowering stage.
//!
//! # Multi-qubit boundary
//!
//! SABRE is fundamentally a connectivity router. It must not silently
//! synthesize arbitrary 3+ qubit gates.
//!
//! This suite therefore verifies that unsupported multi-qubit work is rejected
//! explicitly rather than silently decomposed or corrupted.
//!
//! # Determinism
//!
//! The same:
//!
//! ```text
//! operations
//! topology
//! mapping
//! configuration
//! seed
//! ```
//!
//! must produce reproducible routing decisions whenever deterministic routing
//! is requested.
//!
//! Tests do not require a heuristic to be globally SWAP-optimal because SABRE
//! is a heuristic algorithm rather than an exact optimizer.
//!
//! # Transactionality
//!
//! The algorithm receives caller-owned operations and mapping through immutable
//! references. This suite verifies that a routing invocation does not mutate
//! either input.
//!
//! Failure must not leak a partially routed circuit or a partially modified
//! mapping through the public result.
//!
//! # Verification
//!
//! Successful routes are checked through the result's public verification
//! information where available and through independent black-box invariants.
//!
//! The test suite does not reproduce the verifier implementation. Doing so
//! would make the test duplicate the production code instead of independently
//! checking it.
//!
//! # Integration with compiler IR
//!
//! Compiler-IR conversion belongs to:
//!
//! ```text
//! src/quantum/routing/transpiler.rs
//! ```
//!
//! The transpiler is the compiler integration adapter; the routing algorithms
//! must remain compiler-IR independent. The repository explicitly defines that
//! dependency direction.
//!
//! Consequently this file tests the routing-level contract rather than
//! constructing `IrFunction` values directly.
//!
//! Compiler-level end-to-end coverage should invoke the transpiler adapter in
//! a separate compiler integration suite. This prevents compiler IR changes
//! from unnecessarily destabilizing routing algorithm tests.
//!
//! # Test-harness integration
//!
//! Rust does not automatically compile arbitrary files inside:
//!
//! ```text
//! src/quantum/routing/tests/
//! ```
//!
//! The routing test harness must include this file explicitly:
//!
//! ```text
//! #[cfg(test)]
//! #[path = "tests/end_to_end.rs"]
//! mod end_to_end;
//! ```
//!
//! That declaration belongs to the routing test harness, not this file.
//!
//! No implementation in this file depends on the parent module's private
//! internals.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Safety
//!
//! This test module explicitly denies unsafe code.
//!
//! No raw pointers, FFI, unsafe blocks, or implementation-defined behavior are
//! used.
//!
//! `catch_unwind` is used only for panic-resistance tests around public API
//! calls. Normal routing behavior is always represented through `Result`.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - the public routing pipeline can route a non-trivial circuit;
//! - a topology and mapping are consumed together correctly;
//! - non-adjacent interactions produce legal movement;
//! - final mapping is valid;
//! - semantic gates are preserved;
//! - caller-owned inputs remain unchanged;
//! - deterministic configurations reproduce results;
//! - strict verification succeeds;
//! - resource failures are explicit;
//! - unsupported arity is explicit;
//! - metrics agree with the semantic operation stream;
//! - empty and repeated workloads are safe;
//! - malformed inputs do not cause uncontrolled panics;
//! - no private implementation detail is required;
//! - no future routing algorithm needs to modify this file merely because a
//!   new algorithm was added.
//!
//! # Important testing principle
//!
//! This file validates integration invariants, not a particular implementation
//! strategy.
//!
//! Therefore assertions avoid fragile assumptions such as:
//!
//! - one exact SABRE SWAP sequence for every heuristic case;
//! - one exact heuristic score;
//! - one exact internal candidate ordering;
//! - one exact trial implementation.
//!
//! Exact output is asserted only where the public contract makes it
//! deterministic and semantically necessary.

// =============================================================================
// Crate-level lint policy
// =============================================================================

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

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

use crate::quantum::routing::result::{
    RoutingResult,
    VerificationStatus,
};

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
// Test fixtures
// =============================================================================

/// Creates a logical-qubit identifier.
fn q(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

/// Creates a physical-qubit identifier.
fn p(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

/// Creates the identity mapping.
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
/// This fixture is intentionally used only for testing the routing boundary.
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

/// Creates the production integration configuration.
///
/// The configuration deliberately uses deterministic execution and strict
/// verification so CI exercises the strongest normal routing contract.
fn production_config() -> RoutingConfig {
    let mut config = RoutingConfig::default();

    config.algorithm = RoutingAlgorithm::Sabre;
    config.verify_output = true;
    config.verification_level = VerificationLevel::Strict;
    config.deterministic = true;

    config.lookahead_depth = 4;
    config.candidate_limit = 64;
    config.max_iterations = 10_000;
    config.max_swaps = None;

    config.sabre_iterations = 3;
    config.sabre_trials = 4;

    config
}

/// Creates a compact deterministic CI configuration.
///
/// This keeps the integration tests fast while still executing actual routing.
fn ci_config() -> RoutingConfig {
    let mut config = production_config();

    config.lookahead_depth = 3;
    config.candidate_limit = 32;
    config.max_iterations = 2_000;
    config.sabre_iterations = 2;
    config.sabre_trials = 2;

    config
}

/// Creates a strict configuration with a bounded SWAP budget.
fn bounded_config(max_swaps: usize) -> RoutingConfig {
    let mut config = ci_config();

    config.max_swaps = Some(max_swaps);

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

/// Creates the repository's development heavy-hex-style topology.
fn heavy_hex_topology() -> Topology {
    Topology::heavy_hex()
        .expect("heavy-hex topology must be constructible")
}

/// Routes through the public SABRE API.
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

/// Extracts semantic SWAP moves.
fn swap_moves(
    result: &RoutingResult,
) -> Vec<(PhysicalQubitId, PhysicalQubitId)> {
    result
        .operations()
        .iter()
        .filter_map(|operation| {
            match operation {
                RoutingOperation::Move(
                    RoutingMove::Swap { a, b },
                ) => Some((*a, *b)),

                _ => None,
            }
        })
        .collect()
}

/// Counts semantic SWAP moves.
fn swap_count(result: &RoutingResult) -> usize {
    swap_moves(result).len()
}

/// Extracts non-movement operations.
///
/// The returned references point into the immutable result.
fn gate_operations(
    result: &RoutingResult,
) -> Vec<&RoutingOperation> {
    result
        .operations()
        .iter()
        .filter(|operation| {
            matches!(
                operation,
                RoutingOperation::Gate(_)
            )
        })
        .collect()
}

/// Returns the gate identities from a logical input workload.
///
/// This helper is intentionally based only on the public operation API.
fn input_gate_identities(
    operations: &[QuantumOperation],
) -> Vec<GateIdentity> {
    operations
        .iter()
        .map(|operation| operation.gate().clone())
        .collect()
}

/// Returns gate identities from routed gate operations.
///
/// This deliberately checks only semantic gate identity, not physical
/// placement, because movement may legitimately occur between gates.
fn output_gate_identities(
    result: &RoutingResult,
) -> Vec<GateIdentity> {
    gate_operations(result)
        .iter()
        .filter_map(|operation| {
            match operation {
                RoutingOperation::Gate(gate) => {
                    Some(gate.gate().clone())
                }

                _ => None,
            }
        })
        .collect()
}

/// Asserts the basic result invariants shared by all successful integration
/// scenarios.
fn assert_common_result_invariants(
    result: &RoutingResult,
) {
    result
        .final_mapping()
        .validate()
        .expect("final mapping must be valid");

    assert_eq!(
        result.metrics().inserted_swaps,
        swap_count(result),
        "reported SWAP count must match semantic SWAP operations"
    );

    assert_eq!(
        result.metrics().inserted_moves,
        swap_count(result)
            + result.metrics().inserted_bridges
            + result.metrics().inserted_permutations,
        "inserted move metric must equal semantic movement count"
    );

    assert!(
        result.metrics().final_operations
            >= result.metrics().original_operations,
        "routing must not report fewer final operations than original operations"
    );

    assert_eq!(
        result.metrics().final_operations,
        result.operations().len(),
        "final operation metric must equal result operation count"
    );
}

/// Asserts that a successful result has either passed verification or
/// explicitly reports that verification was not requested.
///
/// Production integration tests request strict verification, so the expected
/// state is normally `Passed`.
fn assert_verification_success(
    result: &RoutingResult,
) {
    match result.verification_status() {
        VerificationStatus::Passed => {}

        VerificationStatus::NotRequested => {
            panic!(
                "integration route requested verification but result reports NotRequested"
            );
        }

        VerificationStatus::NotCompleted => {
            panic!(
                "integration route returned without completing requested verification"
            );
        }
    }
}

/// Compares the logical gate sequence before and after routing.
fn assert_gate_sequence_preserved(
    original: &[QuantumOperation],
    result: &RoutingResult,
) {
    assert_eq!(
        input_gate_identities(original),
        output_gate_identities(result),
        "routing must preserve the semantic logical gate sequence"
    );
}

/// Asserts that the caller-owned mapping remains unchanged.
fn assert_mapping_unchanged(
    original: &QubitMapping,
    after: &QubitMapping,
) {
    assert_eq!(
        original,
        after,
        "routing must not mutate the caller-owned mapping"
    );

    original
        .validate()
        .expect("original mapping must remain valid");
}

/// Asserts that semantic SWAPs are structurally valid.
///
/// The topology verifier is responsible for complete legality checking. This
/// helper checks the public semantic shape without duplicating topology logic.
fn assert_semantic_swaps_are_well_formed(
    result: &RoutingResult,
) {
    for (a, b) in swap_moves(result) {
        assert_ne!(
            a,
            b,
            "a semantic SWAP may not exchange a physical qubit with itself"
        );
    }
}

/// Returns a stable signature for a result.
///
/// This intentionally excludes wall-clock duration and other measurements that
/// may vary between invocations.
fn stable_result_signature(
    result: &RoutingResult,
) -> (
    Vec<RoutingOperation>,
    QubitMapping,
    QubitMapping,
    usize,
    usize,
) {
    (
        result.operations().to_vec(),
        result.initial_mapping().clone(),
        result.final_mapping().clone(),
        result.metrics().inserted_swaps,
        result.metrics().final_operations,
    )
}

// =============================================================================
// Complete routing pipeline
// =============================================================================

#[test]
fn complete_pipeline_routes_non_adjacent_interaction() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 4),
        one_qubit(GateIdentity::X, 4),
    ];

    let config = production_config();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("complete production routing pipeline must succeed");

    assert_common_result_invariants(&result);
    assert_verification_success(&result);
    assert_gate_sequence_preserved(&operations, &result);
    assert_semantic_swaps_are_well_formed(&result);

    assert!(
        swap_count(&result) > 0,
        "a distance-four interaction on a five-node line should require movement"
    );

    assert_eq!(
        result.initial_mapping(),
        &mapping,
        "result must retain the exact supplied initial mapping"
    );

    assert_mapping_unchanged(
        &mapping,
        result.initial_mapping(),
    );
}

#[test]
fn complete_pipeline_handles_already_routable_circuit_without_movement() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 1, 2),
        one_qubit(GateIdentity::X, 3),
        two_qubit(GateIdentity::Cz, 3, 4),
    ];

    let config = production_config();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("already-routable circuit must succeed");

    assert_common_result_invariants(&result);
    assert_verification_success(&result);
    assert_gate_sequence_preserved(&operations, &result);

    assert_eq!(
        swap_count(&result),
        0,
        "already executable interactions must not cause unnecessary SWAPs"
    );

    assert_eq!(
        result.final_mapping(),
        &mapping,
        "no movement should preserve the initial mapping"
    );
}

#[test]
fn complete_pipeline_handles_ring_topology() {
    let topology = ring_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
        two_qubit(GateIdentity::Cz, 1, 4),
        two_qubit(GateIdentity::Cx, 2, 5),
        two_qubit(GateIdentity::Cz, 0, 2),
    ];

    let config = ci_config();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("ring topology must be routable");

    assert_common_result_invariants(&result);
    assert_verification_success(&result);
    assert_gate_sequence_preserved(&operations, &result);

    assert!(
        result.metrics().original_two_qubit_operations >= 4,
        "fixture must exercise multiple two-qubit interactions"
    );
}

#[test]
fn complete_pipeline_handles_heavy_hex_style_topology() {
    let topology = heavy_hex_topology();
    let mapping = identity_mapping(topology.qubit_count());

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cz, 1, 4),
        one_qubit(GateIdentity::H, 2),
        two_qubit(GateIdentity::Cx, 3, 5),
    ];

    let config = ci_config();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("heavy-hex-style topology must be routable");

    assert_common_result_invariants(&result);
    assert_verification_success(&result);
    assert_gate_sequence_preserved(&operations, &result);
}

#[test]
fn complete_pipeline_handles_repeated_interactions() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cz, 0, 4),
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let config = ci_config();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("repeated interactions must remain routable");

    assert_common_result_invariants(&result);
    assert_verification_success(&result);
    assert_gate_sequence_preserved(&operations, &result);

    assert_eq!(
        result.metrics().original_two_qubit_operations,
        operations.len(),
        "all repeated interactions must be counted"
    );
}

#[test]
fn complete_pipeline_handles_single_qubit_operations_without_movement() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        one_qubit(GateIdentity::X, 1),
        one_qubit(GateIdentity::Y, 2),
        one_qubit(GateIdentity::Z, 3),
        one_qubit(GateIdentity::T, 0),
    ];

    let config = production_config();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("single-qubit workload must route");

    assert_common_result_invariants(&result);
    assert_verification_success(&result);
    assert_gate_sequence_preserved(&operations, &result);

    assert_eq!(
        swap_count(&result),
        0,
        "single-qubit gates do not require connectivity movement"
    );

    assert_eq!(
        result.final_mapping(),
        &mapping,
        "single-qubit-only routing should preserve placement"
    );
}

// =============================================================================
// Mapping and semantic-state integration
// =============================================================================

#[test]
fn routing_updates_final_mapping_without_corrupting_initial_mapping() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);
    let original_mapping = mapping.clone();

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 1, 3),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    )
    .expect("route must succeed");

    assert_mapping_unchanged(
        &original_mapping,
        &mapping,
    );

    assert_eq!(
        result.initial_mapping(),
        &original_mapping,
        "result must preserve the original mapping snapshot"
    );

    result
        .final_mapping()
        .validate()
        .expect("final mapping must remain valid");

    assert!(
        result.final_mapping() != &original_mapping
            || swap_count(&result) == 0,
        "movement must be reflected in the final mapping"
    );
}

#[test]
fn final_mapping_is_a_valid_bijection() {
    let topology = line_topology(8);
    let mapping = identity_mapping(8);

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
        &ci_config(),
    )
    .expect("route must succeed");

    result
        .final_mapping()
        .validate()
        .expect("final mapping must be a valid bijection");

    assert_eq!(
        result.final_mapping().len(),
        mapping.len(),
        "routing must not lose or duplicate logical qubits"
    );
}

#[test]
fn routing_does_not_mutate_input_operations() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 4),
        one_qubit(GateIdentity::X, 4),
    ];

    let original_operations = operations.clone();

    let _result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    )
    .expect("route must succeed");

    assert_eq!(
        operations,
        original_operations,
        "routing must not mutate caller-owned operations"
    );
}

// =============================================================================
// Semantic preservation
// =============================================================================

#[test]
fn routing_preserves_gate_count_and_order() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 5),
        one_qubit(GateIdentity::Rz, 2),
        two_qubit(GateIdentity::Cz, 1, 4),
        one_qubit(GateIdentity::X, 5),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &production_config(),
    )
    .expect("route must succeed");

    assert_gate_sequence_preserved(
        &operations,
        &result,
    );

    assert_eq!(
        result.metrics().original_operations,
        operations.len(),
        "original operation metric must match input length"
    );

    assert_eq!(
        gate_operations(&result).len(),
        operations.len(),
        "routing movement may add operations but must not remove logical gates"
    );
}

#[test]
fn routing_does_not_turn_semantic_swaps_into_logical_gates() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    )
    .expect("route must succeed");

    assert!(
        swap_count(&result) > 0,
        "fixture should require semantic movement"
    );

    assert_eq!(
        output_gate_identities(&result),
        vec![GateIdentity::Cx],
        "inserted SWAPs must remain movement operations rather than becoming logical gates"
    );
}

// =============================================================================
// Determinism and reproducibility
// =============================================================================

#[test]
fn deterministic_routing_is_reproducible() {
    let topology = ring_topology(8);
    let mapping = identity_mapping(8);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
        two_qubit(GateIdentity::Cx, 1, 5),
        two_qubit(GateIdentity::Cz, 2, 6),
        two_qubit(GateIdentity::Cx, 3, 7),
        two_qubit(GateIdentity::Cz, 0, 6),
    ];

    let mut config = ci_config();

    config.seed = Some(0x5A17_2026);

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
        stable_result_signature(&first),
        stable_result_signature(&second),
        "same deterministic input must produce the same stable routing result"
    );
}

#[test]
fn deterministic_routing_preserves_seed_metadata() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cz, 1, 4),
    ];

    let seed = 0x1234_5678_u64;

    let mut config = ci_config();
    config.seed = Some(seed);

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("seeded route must succeed");

    assert_eq!(
        result.reproducibility().seed,
        Some(seed),
        "routing result must retain the supplied reproducibility seed"
    );

    assert!(
        result.reproducibility().deterministic,
        "deterministic configuration must be reflected in result metadata"
    );
}

#[test]
fn routing_result_contains_algorithm_identity() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    )
    .expect("route must succeed");

    assert_eq!(
        result.algorithm(),
        "sabre",
        "result must identify the selected routing algorithm"
    );

    assert!(
        !SABRE_ALGORITHM_VERSION.is_empty(),
        "SABRE algorithm version must exist"
    );

    assert!(
        !SABRE_ROUTING_VERSION.is_empty(),
        "SABRE routing version must exist"
    );
}

// =============================================================================
// Verification integration
// =============================================================================

#[test]
fn strict_verification_passes_for_valid_nontrivial_route() {
    let topology = line_topology(7);
    let mapping = identity_mapping(7);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 6),
        two_qubit(GateIdentity::Cz, 1, 5),
        one_qubit(GateIdentity::Rz, 3),
        two_qubit(GateIdentity::Cx, 2, 4),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &production_config(),
    )
    .expect("strictly verified route must succeed");

    assert_eq!(
        result.verification_status(),
        VerificationStatus::Passed,
        "strict integration routing must finish verification successfully"
    );

    assert_common_result_invariants(&result);
    assert_gate_sequence_preserved(&operations, &result);
}

#[test]
fn verification_result_is_consistent_with_requested_level() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let config = production_config();

    assert_eq!(
        config.verification_level,
        VerificationLevel::Strict
    );

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("strict verification route must succeed");

    assert_eq!(
        result.verification_status(),
        VerificationStatus::Passed
    );
}

// =============================================================================
// Failure boundaries
// =============================================================================

#[test]
fn disconnected_topology_fails_without_producing_a_partial_result() {
    let topology = Topology::from_edges(
        "Disconnected",
        &[
            (0, 1),
            (2, 3),
        ],
    )
    .expect("disconnected topology fixture must be constructible");

    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
    ];

    let error = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    )
    .expect_err(
        "an interaction across disconnected components must fail"
    );

    assert!(
        matches!(
            error,
            RoutingError::Disconnected { .. }
                | RoutingError::NoRoutingPath { .. }
                | RoutingError::RoutingFailed { .. }
                | RoutingError::InvalidInput { .. }
        ),
        "failure must remain a structured routing error: {error:?}"
    );

    assert_eq!(
        operations,
        vec![two_qubit(GateIdentity::Cx, 0, 3)],
        "failed routing must not mutate caller operations"
    );

    mapping
        .validate()
        .expect("failed routing must not corrupt caller mapping");
}

#[test]
fn insufficient_physical_resources_fail_explicitly() {
    let topology = line_topology(2);

    let mapping = QubitMapping::from_assignments([
        (q(0), p(0)),
        (q(1), p(1)),
        (q(2), p(0)),
    ]);

    assert!(
        mapping.is_err(),
        "duplicate physical assignment must be rejected before routing"
    );
}

#[test]
fn unknown_logical_qubit_fails_explicitly() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 99),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    );

    assert!(
        result.is_err(),
        "an unmapped logical qubit must never be silently accepted"
    );
}

#[test]
fn duplicate_logical_operands_are_rejected_or_reported_as_invalid() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 1, 1),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    );

    assert!(
        result.is_err(),
        "a two-qubit gate using the same logical qubit twice must be rejected"
    );
}

#[test]
fn unsupported_multi_qubit_operation_is_not_silently_decomposed() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        three_qubit(
            GateIdentity::Ccx,
            0,
            2,
            4,
        ),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    );

    assert!(
        result.is_err(),
        "unsupported multi-qubit routing must fail explicitly"
    );

    if let Err(error) = result {
        assert!(
            matches!(
                error,
                RoutingError::UnsupportedMultiQubitOperation { .. }
                    | RoutingError::UnsupportedArity { .. }
                    | RoutingError::UnsupportedGate { .. }
                    | RoutingError::InvalidInput { .. }
            ),
            "multi-qubit failure must identify the unsupported boundary: {error:?}"
        );
    }
}

// =============================================================================
// Resource limits
// =============================================================================

#[test]
fn zero_swap_budget_rejects_a_route_that_requires_movement() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let config = bounded_config(0);

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    );

    assert!(
        result.is_err(),
        "a zero-SWAP budget must reject a route requiring movement"
    );
}

#[test]
fn sufficient_swap_budget_allows_the_same_workload() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let config = bounded_config(32);

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("sufficient SWAP budget should permit the route");

    assert_common_result_invariants(&result);

    assert!(
        result.metrics().inserted_swaps <= 32,
        "router must honor the configured SWAP budget"
    );
}

// =============================================================================
// Metrics consistency
// =============================================================================

#[test]
fn metrics_are_consistent_with_the_semantic_result() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cz, 1, 4),
        one_qubit(GateIdentity::X, 3),
    ];

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    )
    .expect("route must succeed");

    let metrics = result.metrics();

    assert_eq!(
        metrics.original_operations,
        operations.len()
    );

    assert_eq!(
        metrics.final_operations,
        result.operations().len()
    );

    assert_eq!(
        metrics.inserted_swaps,
        swap_count(&result)
    );

    assert_eq!(
        metrics.routing_overhead_operations,
        metrics.inserted_moves
    );

    assert!(
        metrics.final_operations
            >= metrics.original_operations
    );

    assert!(
        metrics.routing_duration <= metrics.total_duration
            || metrics.total_duration.is_zero(),
        "routing duration must not exceed total duration"
    );
}

#[test]
fn empty_workload_has_zero_routing_overhead() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let operations: Vec<QuantumOperation> = Vec::new();

    let result = route(
        &SabreRouter::new(),
        &operations,
        &topology,
        &mapping,
        &ci_config(),
    )
    .expect("empty workload must route successfully");

    assert_common_result_invariants(&result);
    assert_verification_success(&result);

    assert_eq!(
        result.operations().len(),
        0
    );

    assert_eq!(
        result.metrics().inserted_swaps,
        0
    );

    assert_eq!(
        result.metrics().inserted_moves,
        0
    );

    assert_eq!(
        result.final_mapping(),
        &mapping
    );
}

// =============================================================================
// Repeated invocation / state isolation
// =============================================================================

#[test]
fn repeated_invocations_do_not_share_mutable_routing_state() {
    let topology = line_topology(5);
    let mapping = identity_mapping(5);

    let first_operations = vec![
        two_qubit(GateIdentity::Cx, 0, 4),
    ];

    let second_operations = vec![
        two_qubit(GateIdentity::Cx, 1, 2),
    ];

    let router = SabreRouter::new();
    let config = ci_config();

    let first = route(
        &router,
        &first_operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("first route must succeed");

    let second = route(
        &router,
        &second_operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("second route must succeed");

    assert_eq!(
        first.initial_mapping(),
        &mapping
    );

    assert_eq!(
        second.initial_mapping(),
        &mapping
    );

    assert_eq!(
        first_operations,
        vec![
            two_qubit(GateIdentity::Cx, 0, 4)
        ]
    );

    assert_eq!(
        second_operations,
        vec![
            two_qubit(GateIdentity::Cx, 1, 2)
        ]
    );

    assert_common_result_invariants(&first);
    assert_common_result_invariants(&second);
}

#[test]
fn routing_same_input_after_previous_route_remains_reproducible() {
    let topology = ring_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 3),
        two_qubit(GateIdentity::Cz, 1, 4),
        two_qubit(GateIdentity::Cx, 2, 5),
    ];

    let config = ci_config();

    let router = SabreRouter::new();

    let first = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("first route must succeed");

    let second = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("second route must succeed");

    assert_eq!(
        stable_result_signature(&first),
        stable_result_signature(&second),
        "routing invocations must not leak mutable state between calls"
    );
}

// =============================================================================
// Panic resistance
// =============================================================================

#[test]
fn malformed_public_input_does_not_panic() {
    let topology = line_topology(4);
    let mapping = identity_mapping(4);

    let malformed = vec![
        QuantumOperation::new(
            GateIdentity::Cx,
            vec![],
        ),
    ];

    let config = ci_config();

    let outcome = catch_unwind(AssertUnwindSafe(|| {
        route(
            &SabreRouter::new(),
            &malformed,
            &topology,
            &mapping,
            &config,
        )
    }));

    assert!(
        outcome.is_ok(),
        "malformed public routing input must return an error rather than panic"
    );

    let result = outcome
        .expect("catch_unwind must return an outer result");

    assert!(
        result.is_err(),
        "malformed operation must be rejected explicitly"
    );
}

#[test]
fn invalid_mapping_fixture_is_rejected_without_unsafe_behavior() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        QubitMapping::from_assignments([
            (q(0), p(0)),
            (q(1), p(0)),
        ])
    }));

    assert!(
        result.is_ok(),
        "invalid mapping construction must not panic"
    );

    assert!(
        result
            .expect("mapping constructor must complete")
            .is_err(),
        "duplicate physical assignment must return an error"
    );
}

// =============================================================================
// Algorithm contract
// =============================================================================

#[test]
fn sabre_algorithm_identity_is_stable() {
    let router = SabreRouter::new();

    assert_eq!(
        router.name(),
        "sabre"
    );

    assert_eq!(
        router.heuristic(),
        SabreHeuristic::Decay
    );
}

#[test]
fn sabre_explicit_heuristics_remain_valid_in_end_to_end_pipeline() {
    let topology = line_topology(6);
    let mapping = identity_mapping(6);

    let operations = vec![
        two_qubit(GateIdentity::Cx, 0, 5),
        two_qubit(GateIdentity::Cz, 1, 4),
        two_qubit(GateIdentity::Cx, 2, 3),
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
            &ci_config(),
        )
        .unwrap_or_else(|error| {
            panic!(
                "SABRE heuristic {:?} failed integration routing: {error:?}",
                heuristic
            )
        });

        assert_common_result_invariants(&result);
        assert_verification_success(&result);
        assert_gate_sequence_preserved(
            &operations,
            &result,
        );
    }
}

// =============================================================================
// Final integration contract
// =============================================================================

#[test]
fn production_routing_contract_is_satisfied_end_to_end() {
    let topology = ring_topology(10);
    let mapping = identity_mapping(10);

    let operations = vec![
        one_qubit(GateIdentity::H, 0),
        two_qubit(GateIdentity::Cx, 0, 5),
        one_qubit(GateIdentity::Rz, 5),
        two_qubit(GateIdentity::Cz, 1, 6),
        one_qubit(GateIdentity::X, 2),
        two_qubit(GateIdentity::Cx, 2, 7),
        one_qubit(GateIdentity::H, 3),
        two_qubit(GateIdentity::Cz, 3, 8),
        two_qubit(GateIdentity::Cx, 4, 9),
        one_qubit(GateIdentity::T, 6),
    ];

    let mut config = production_config();

    config.seed = Some(0x5A4D_0001);

    let router = SabreRouter::new();

    let result = route(
        &router,
        &operations,
        &topology,
        &mapping,
        &config,
    )
    .expect("production end-to-end routing contract must succeed");

    // -------------------------------------------------------------------------
    // Input preservation
    // -------------------------------------------------------------------------

    assert_eq!(
        mapping,
        *result.initial_mapping(),
        "initial mapping must be captured exactly"
    );

    assert_eq!(
        input_gate_identities(&operations),
        output_gate_identities(&result),
        "logical gate semantics must survive routing"
    );

    // -------------------------------------------------------------------------
    // Mapping invariants
    // -------------------------------------------------------------------------

    mapping
        .validate()
        .expect("input mapping must be valid");

    result
        .final_mapping()
        .validate()
        .expect("final mapping must be valid");

    // -------------------------------------------------------------------------
    // Routing operations
    // -------------------------------------------------------------------------

    assert_semantic_swaps_are_well_formed(&result);

    assert_eq!(
        result.metrics().inserted_swaps,
        swap_count(&result)
    );

    // -------------------------------------------------------------------------
    // Verification
    // -------------------------------------------------------------------------

    assert_eq!(
        result.verification_status(),
        VerificationStatus::Passed
    );

    // -------------------------------------------------------------------------
    // Metrics
    // -------------------------------------------------------------------------

    assert_eq!(
        result.metrics().original_operations,
        operations.len()
    );

    assert_eq!(
        result.metrics().final_operations,
        result.operations().len()
    );

    assert!(
        result.metrics().final_operations
            >= result.metrics().original_operations
    );

    assert!(
        result.metrics().physical_qubits
            >= result.metrics().logical_qubits
    );

    // -------------------------------------------------------------------------
    // Reproducibility
    // -------------------------------------------------------------------------

    assert!(
        result.reproducibility().deterministic
    );

    assert_eq!(
        result.reproducibility().seed,
        Some(0x5A4D_0001)
    );

    // -------------------------------------------------------------------------
    // Stable algorithm identity
    // -------------------------------------------------------------------------

    assert_eq!(
        result.algorithm(),
        "sabre"
    );

    assert!(
        !SABRE_ALGORITHM_VERSION.is_empty()
    );

    assert!(
        !SABRE_ROUTING_VERSION.is_empty()
    );
}

// =============================================================================
// End-of-file production invariant
// =============================================================================
//
// This module intentionally contains no:
// - unsafe code;
// - provider SDK access;
// - compiler parser access;
// - OpenQASM parsing;
// - pulse scheduling;
// - hardware execution;
// - simulation;
// - QEC decoding;
// - benchmark execution.
//
// Those concerns remain outside the routing algorithm integration boundary.
//
// The routing subsystem is therefore tested as:
//
//     topology
//        ↓
//     mapping
//        ↓
//     operations
//        ↓
//     SABRE
//        ↓
//     semantic movement
//        ↓
//     final mapping
//        ↓
//     RoutingResult
//        ↓
//     verification
//
// This keeps the test stable when implementation details change while making
// cross-module regressions visible.
// =============================================================================