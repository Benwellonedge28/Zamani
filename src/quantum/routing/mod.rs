//! Zamani Quantum Routing
//!
//! `src/quantum/routing/mod.rs`
//!
//! Production namespace and integration boundary for Zamani's quantum
//! logical-to-physical routing subsystem.
//!
//! # Purpose
//!
//! This module is the authoritative Rust module boundary for:
//!
//! - routing contracts and identifiers;
//! - topology representation;
//! - logical/physical qubit mapping;
//! - routing configuration;
//! - routing objectives and cost models;
//! - path finding;
//! - routing candidate generation;
//! - initial layout;
//! - routing algorithms;
//! - semantic movement operations;
//! - routing orchestration;
//! - result and metrics reporting;
//! - verification;
//! - compiler/IR transpilation;
//! - routing caches;
//! - distributed routing;
//! - parallel routing;
//! - routing plugin integration.
//!
//! This file owns namespace composition and stable public re-exports.
//!
//! It does NOT implement routing algorithms itself.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                              ▼
//!                   logical quantum program
//!                              │
//!                              ▼
//!                    quantum::routing
//!                              │
//!       ┌──────────────────────┼──────────────────────┐
//!       │                      │                      │
//!       ▼                      ▼                      ▼
//!     layout               topology                mapping
//!       │                      │                      │
//!       └──────────────────────┼──────────────────────┘
//!                              ▼
//!                         algorithms
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!       shortest           lookahead             SABRE
//!          │                   │                   │
//!          ├───────────────────┼───────────────────┤
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!        basic           noise-aware           dynamic
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                            moves
//!                              │
//!                              ▼
//!                            router
//!                              │
//!               ┌──────────────┼──────────────┐
//!               │              │              │
//!               ▼              ▼              ▼
//!          verification      cache        parallel
//!               │                             │
//!               └──────────────┬──────────────┘
//!                              ▼
//!                         RoutingResult
//!                              │
//!                              ▼
//!                    routing::transpiler
//!                              │
//!                              ▼
//!                   hardware lowering
//! ```
//!
//! # Dependency direction
//!
//! The routing subsystem follows this dependency direction:
//!
//! ```text
//! types
//!   │
//!   ├──────────────┬──────────────┬──────────────┐
//!   ▼              ▼              ▼              ▼
//! errors       topology        mapping        config
//!   │              │              │              │
//!   └──────────────┴──────────────┴──────────────┘
//!                          │
//!                          ▼
//!                         cost
//!                          │
//!                          ▼
//!                        result
//!                          │
//!             ┌────────────┼────────────┐
//!             ▼            ▼            ▼
//!           path       candidates     layout
//!             │            │            │
//!             └────────────┼────────────┘
//!                          ▼
//!                      algorithms
//!                          │
//!                          ▼
//!                         moves
//!                          │
//!              ┌───────────┼───────────┐
//!              ▼           ▼           ▼
//!             cache     parallel   distributed
//!              │           │           │
//!              └───────────┼───────────┘
//!                          ▼
//!                         router
//!                          │
//!                    ┌─────┴─────┐
//!                    ▼           ▼
//!              verification  transpiler
//! ```
//!
//! The important invariant is that concrete algorithms do not depend upward
//! on `router` or `transpiler`.
//!
//! # Ownership boundaries
//!
//! ## `types`
//!
//! Stable routing vocabulary, identifiers, operation descriptions, movement
//! descriptions, and routing-level value types.
//!
//! ## `errors`
//!
//! Canonical routing error contract.
//!
//! ## `topology`
//!
//! Physical connectivity, directed gate legality, physical resources and
//! hardware-independent topology semantics.
//!
//! ## `mapping`
//!
//! Authoritative logical-to-physical and physical-to-logical placement.
//!
//! ## `cost`
//!
//! Routing objectives and cost evaluation.
//!
//! ## `config`
//!
//! Routing policy, limits, algorithm selection, layout selection, verification
//! policy and deterministic/reproducible execution configuration.
//!
//! ## `result`
//!
//! Routing result, metrics, verification summaries and reproducibility data.
//!
//! ## `path`
//!
//! Graph traversal, shortest paths and path-related primitives.
//!
//! ## `candidates`
//!
//! Candidate movement generation and ranking.
//!
//! ## `layout`
//!
//! Initial logical-to-physical placement.
//!
//! ## `algorithms`
//!
//! Behavioral routing-algorithm contract and built-in algorithms.
//!
//! ## `moves`
//!
//! Semantic movement primitives such as SWAP, bridge and permutation.
//!
//! ## `cache`
//!
//! Reusable routing/path/search cache infrastructure.
//!
//! ## `parallel`
//!
//! Parallel independent routing trials and candidate evaluation infrastructure.
//!
//! ## `distributed`
//!
//! Routing abstractions for modular and distributed quantum systems.
//!
//! ## `plugins`
//!
//! Explicit extension/plugin contracts without a global mutable algorithm
//! registry.
//!
//! ## `router`
//!
//! High-level orchestration, algorithm selection, limits, transaction boundary
//! and public routing engine.
//!
//! ## `verification`
//!
//! Structural, mapping, executability and semantic routing verification.
//!
//! ## `transpiler`
//!
//! Compiler/IR integration adapter.
//!
//! # Canonical Quantum IR boundary
//!
//! Routing is deliberately independent from the implementation details of
//! `quantum::ir`.
//!
//! The compiler/IR adapter is responsible for crossing the boundary:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! routing::transpiler
//!      │
//!      ▼
//! routing contracts
//!      │
//!      ├── topology
//!      ├── mapping
//!      ├── layout
//!      ├── algorithms
//!      ├── moves
//!      └── verification
//! ```
//!
//! Concrete routing algorithms must not import OpenQASM parsing or compiler
//! parser internals.
//!
//! # Hardware boundary
//!
//! Routing is provider-neutral.
//!
//! Hardware/provider implementations supply routing-compatible information,
//! including where available:
//!
//! - physical qubits;
//! - connectivity;
//! - directed gate support;
//! - gate durations;
//! - error rates;
//! - fidelity;
//! - availability;
//! - calibration metadata.
//!
//! Routing does not authenticate with, communicate with, or execute against a
//! provider.
//!
//! # Routing versus synthesis
//!
//! Routing represents physical movement and placement constraints.
//!
//! It does not silently synthesize arbitrary unsupported gates.
//!
//! The intended boundary is:
//!
//! ```text
//! Quantum operation
//!       │
//!       ├── target natively supports operation
//!       │             │
//!       │             ▼
//!       │          routing
//!       │
//!       └── target does not support operation
//!                     │
//!                     ▼
//!             synthesis/decomposition
//! ```
//!
//! A routing movement such as `SWAP` is a semantic movement request. It does
//! not imply that the hardware has a native SWAP instruction.
//!
//! # Directed connectivity
//!
//! Physical adjacency and gate executability are different concepts:
//!
//! ```text
//! adjacent(p0, p1)
//!       !=
//! supports_gate(gate, p0, p1)
//! ```
//!
//! Consequently, the routing namespace exposes topology and routing contracts
//! rather than assuming that every physical edge supports every two-qubit gate
//! in both directions.
//!
//! # Multi-qubit operations
//!
//! Routing must not invent decompositions for arbitrary three-or-more-qubit
//! operations.
//!
//! Native multi-qubit operations may be routed when the target explicitly
//! supports them. Unsupported operations must cross the synthesis/decomposition
//! boundary instead of being silently rewritten by routing.
//!
//! # Determinism
//!
//! Production routing must be reproducible when configured for deterministic
//! execution.
//!
//! Deterministic behavior is governed by `config` and implemented by the
//! selected algorithm.
//!
//! This root module does not own random-number generation.
//!
//! # Transactionality
//!
//! Routing is a transaction:
//!
//! ```text
//! caller state
//!      │
//!      ▼
//! immutable routing input
//!      │
//!      ▼
//! routing algorithm
//!      │
//!   ┌──┴───────┐
//!   ▼          ▼
//! success    failure
//!   │          │
//!   ▼          ▼
//! result      error
//!   │          │
//!   ▼          ▼
//! commit    caller unchanged
//! ```
//!
//! Concrete algorithms must not mutate caller-owned state through hidden
//! global state.
//!
//! # Verification
//!
//! Routing results must be independently verifiable.
//!
//! Verification may establish:
//!
//! - valid logical operands;
//! - valid physical operands;
//! - valid topology usage;
//! - valid directed gate usage;
//! - valid mapping;
//! - valid mapping evolution;
//! - valid movement operations;
//! - preservation of logical operations;
//! - preservation of measurement semantics;
//! - valid final mapping;
//! - consistency between routing operations and reported results.
//!
//! # Performance architecture
//!
//! Routing can operate on large circuits and large hardware graphs.
//!
//! The namespace therefore explicitly accommodates:
//!
//! - path caching;
//! - candidate pruning;
//! - parallel independent trials;
//! - multiple deterministic seeded trials;
//! - distributed/modular topology;
//! - reusable search state.
//!
//! These mechanisms remain optional implementation layers and do not change
//! the canonical routing API.
//!
//! # Distributed quantum computing
//!
//! `distributed` is a routing extension point for systems where physical
//! resources are organized into modules/nodes connected by a quantum network.
//!
//! Ordinary local routing remains independent of distributed routing.
//!
//! Future mechanisms such as teleportation, entanglement-assisted movement or
//! network-mediated interactions must be represented explicitly rather than
//! changing the semantics of ordinary `SwapMove`.
//!
//! # Plugin architecture
//!
//! `plugins` provides extension contracts for custom routing algorithms and
//! related strategies.
//!
//! Plugins must not require global mutable registries.
//!
//! A caller-owned router/registry instance is the preferred ownership model.
//!
//! # Safety
//!
//! This namespace is safe Rust.
//!
//! No `unsafe` implementation is permitted.
//!
//! The lint declarations below make accidental unsafe code a compilation
//! failure for this module and its contents where the lint propagates.
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
//! - no unsafe.
//!
//! # Existing repository integration
//!
//! The current repository already contains the following production routing
//! components:
//!
//! ```text
//! src/quantum/routing/
//! ├── algorithms/
//! │   ├── basic.rs
//! │   ├── dynamic.rs
//! │   ├── lookahead.rs
//! │   ├── mod.rs
//! │   ├── noise_aware.rs
//! │   ├── sabre.rs
//! │   └── shortest_path.rs
//! ├── banches/
//! │   ├── routing.rs
//! │   ├── sabre.rs
//! │   └── topology.rs
//! ├── cache.rs
//! ├── candidates.rs
//! ├── config.rs
//! ├── cost.rs
//! ├── distributed.rs
//! ├── errors.rs
//! ├── layout.rs
//! ├── mapping.rs
//! ├── moves/
//! │   ├── bridge.rs
//! │   ├── mod.rs<space>
//! │   ├── permutation.rs
//! │   ├── router.rs
//! │   └── swap.rs
//! ├── parallel.rs
//! ├── path.rs
//! ├── plugins.rs
//! ├── result.rs
//! ├── router.rs
//! ├── tests/
//! ├── topology.rs
//! ├── transpiler.rs
//! └── types.rs
//! ```
//!
//! The `banches` directory is deliberately NOT exposed as part of the
//! production routing namespace. Its name and contents indicate branch/
//! experimental material rather than the canonical implementation.
//!
//! The movement namespace has a repository filename anomaly: the existing
//! module file is `moves/mod.rs ` with a trailing space. The explicit path
//! declaration below accommodates that exact repository state without silently
//! depending on `moves/mod.rs`, which does not currently exist.
//!
//! # Public API policy
//!
//! This root module follows three rules:
//!
//! 1. Every production child module that exists in the canonical routing tree
//!    is declared exactly once.
//!
//! 2. No type, trait, enum, error, configuration object, or algorithm is
//!    duplicated here.
//!
//! 3. Stable high-level contracts are re-exported only when doing so does not
//!    create competing names or couple the root to implementation details.
//!
//! This keeps `mod.rs` a namespace boundary instead of turning it into a
//! second implementation of routing.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! - every canonical production routing module is declared;
//! - the malformed movement-module filename is explicitly handled;
//! - experimental `banches` code is not accidentally exposed;
//! - the existing algorithm namespace is authoritative;
//! - routing infrastructure such as cache, parallel, distributed and plugins
//!   is reachable;
//! - the existing test suite is compiled under `cfg(test)`;
//! - no duplicate routing contracts are defined here;
//! - no future algorithm requires changing this file merely because the
//!   algorithm is added inside `algorithms/` and exported by its own
//!   `algorithms/mod.rs`.
//!
//! New concrete algorithms therefore belong under `algorithms/` and are
//! exported by `algorithms/mod.rs`; they do not require changes to this root
//! module.
//!
//! # Integration summary
//!
//! ```text
//! quantum::routing
//! │
//! ├── contracts
//! │   ├── types
//! │   ├── errors
//! │   ├── config
//! │   └── result
//! │
//! ├── physical model
//! │   ├── topology
//! │   └── mapping
//! │
//! ├── planning
//! │   ├── path
//! │   ├── candidates
//! │   └── layout
//! │
//! ├── algorithms
//! │   └── algorithms/mod.rs
//! │
//! ├── movement
//! │   └── moves/mod.rs<space>
//! │
//! ├── infrastructure
//! │   ├── cache
//! │   ├── parallel
//! │   ├── distributed
//! │   └── plugins
//! │
//! ├── orchestration
//! │   └── router
//! │
//! ├── correctness
//! │   └── verification
//! │
//! └── compiler boundary
//!     └── transpiler
//! ```
//!
//! # Non-responsibilities
//!
//! The following remain outside this namespace:
//!
//! - OpenQASM parsing;
//! - source-language parsing;
//! - canonical Quantum IR ownership;
//! - gate synthesis;
//! - gate decomposition;
//! - pulse generation;
//! - hardware execution;
//! - scheduling;
//! - quantum simulation;
//! - QEC decoding;
//! - benchmark execution.
//!
//! These boundaries are consistent with the wider `quantum` subsystem.
//!
//! # Versioning
//!
//! The routing root intentionally does not define an independent semantic
//! version number. Version identifiers belong to the concrete contracts that
//! report them, especially `router`, algorithms and result metadata.
//!
//! # No global state
//!
//! This namespace does not create:
//!
//! - global algorithm registries;
//! - global routing caches;
//! - global topology state;
//! - global random-number generators;
//! - global mutable configuration.
//!
//! All such state must be explicitly owned by the caller or the appropriate
//! routing object.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Foundational contracts
// =============================================================================

/// Stable routing identifiers, vocabulary and routing-level value types.
pub mod types;

/// Canonical routing errors and diagnostics.
pub mod errors;

/// Physical connectivity and gate-aware topology representation.
pub mod topology;

/// Authoritative logical-to-physical mapping.
pub mod mapping;

/// Routing objectives and cost models.
pub mod cost;

/// Routing configuration and policy.
pub mod config;

/// Successful routing results, metrics and reproducibility metadata.
pub mod result;

// =============================================================================
// Planning and search primitives
// =============================================================================

/// Graph/path-finding primitives.
pub mod path;

/// Routing candidate generation.
pub mod candidates;

/// Initial logical-to-physical layout selection.
pub mod layout;

// =============================================================================
// Routing algorithms
// =============================================================================

/// Built-in routing algorithms and the common routing-algorithm contract.
///
/// This child module is authoritative for concrete algorithm declarations.
/// New algorithms added inside `algorithms/` should be wired through
/// `algorithms/mod.rs`, not through this root module.
pub mod algorithms;

// =============================================================================
// Movement layer
// =============================================================================
//
// IMPORTANT:
// The current repository contains `moves/mod.rs ` with a trailing space.
// Rust's conventional `pub mod moves;` would look for `moves/mod.rs` and fail.
// The explicit path below integrates the repository's current filename exactly.
//
// Once the repository filename itself is normalized from:
//
//     moves/mod.rs<space>
//
// to:
//
//     moves/mod.rs
//
// this declaration can be simplified to:
//
//     pub mod moves;
//
// No routing API redesign is required by that filename normalization.

/// Semantic physical movement primitives.
#[path = "moves/mod.rs "]
pub mod moves;

// =============================================================================
// Routing infrastructure
// =============================================================================

/// Routing/path/search caching infrastructure.
pub mod cache;

/// Parallel routing trials and parallel search infrastructure.
pub mod parallel;

/// Distributed and modular quantum routing support.
pub mod distributed;

/// Explicit extension/plugin contracts for routing.
pub mod plugins;

// =============================================================================
// Routing orchestration and correctness
// =============================================================================

/// High-level routing engine and algorithm orchestration.
pub mod router;

/// Independent routing-result verification.
pub mod verification;

// =============================================================================
// Compiler integration
// =============================================================================

/// Compiler/IR integration adapter.
///
/// This is intentionally the only routing child that is permitted to depend
/// directly on compiler IR implementation details.
pub mod transpiler;

// =============================================================================
// Stable high-level re-exports
// =============================================================================
//
// Re-export only names whose ownership is already established by the child
// modules. Do not glob-import every child module here: routing contains several
// intentionally overlapping concepts such as configuration identifiers and
// behavioral algorithm traits.
//
// The explicit exports below provide a concise production API while preserving
// the detailed subsystem paths for advanced callers.

/// Canonical routing configuration.
pub use config::RoutingConfig;

/// Canonical routing error.
pub use errors::RoutingError;

/// Authoritative logical/physical mapping.
pub use mapping::QubitMapping;

/// Canonical routing result.
pub use result::RoutingResult;

/// Physical routing topology.
pub use topology::Topology;

/// Stable logical-qubit identifier.
pub use types::LogicalQubitId;

/// Stable physical-qubit identifier.
pub use types::PhysicalQubitId;

/// Stable routing input contract.
pub use types::RoutingInput;

/// Stable routing operation contract.
pub use types::RoutingOperation;

// =============================================================================
// Compatibility exports
// =============================================================================
//
// The historical compiler adapter remains available through its canonical
// child path:
//
//     quantum::routing::transpiler
//
// We intentionally do not flatten every legacy type from `transpiler.rs` into
// this root. Doing so would make the compatibility layer compete with the
// canonical routing contracts.
//
// Existing users that explicitly require the compatibility adapter should use:
//
//     quantum::routing::transpiler::QuantumTranspiler
//
// or its existing public API.
//
// =============================================================================
// Compile-time architectural checks
// =============================================================================

#[cfg(test)]
mod namespace_tests {
    use super::*;

    #[test]
    fn foundational_routing_types_are_reachable() {
        let _ = std::any::TypeId::of::<LogicalQubitId>();
        let _ = std::any::TypeId::of::<PhysicalQubitId>();
        let _ = std::any::TypeId::of::<QubitMapping>();
        let _ = std::any::TypeId::of::<RoutingConfig>();
        let _ = std::any::TypeId::of::<RoutingResult>();
        let _ = std::any::TypeId::of::<Topology>();
    }

    #[test]
    fn routing_infrastructure_modules_are_reachable() {
        let _ = std::any::TypeId::of::<
            algorithms::RoutingAlgorithmCapabilities,
        >();

        let _ = std::any::TypeId::of::<
            moves::MoveKind,
        >();
    }

    #[test]
    fn routing_namespace_contains_the_expected_production_boundaries() {
        fn assert_module_boundaries_exist() {
            let _ = &types::LogicalQubitId::new;
            let _ = &errors::RoutingError::to_string;
            let _ = &topology::Topology::validate;
            let _ = &mapping::QubitMapping::validate;
            let _ = &config::RoutingConfig::validate;
            let _ = &path::PathFinder::new;
            let _ = &router::Router::new;
            let _ = &verification::RoutingVerifier::new;
        }

        assert_module_boundaries_exist();
    }
}

// =============================================================================
// Existing routing test suite
// =============================================================================
//
// The repository stores routing tests under `src/quantum/routing/tests/`.
// Rust does not automatically compile arbitrary files in such a directory.
// They are therefore explicitly attached to the routing module in test builds.
//
// These are test-only modules and do not become part of release builds.
//
// The test files themselves are intentionally kept independent from this
// namespace implementation. They consume the public routing contracts.

#[cfg(test)]
#[path = "tests/basic.rs"]
mod routing_test_basic;

#[cfg(test)]
#[path = "tests/directed.rs"]
mod routing_test_directed;

#[cfg(test)]
#[path = "tests/end_to_end.rs"]
mod routing_test_end_to_end;

#[cfg(test)]
#[path = "tests/lookahead.rs"]
mod routing_test_lookahead;

#[cfg(test)]
#[path = "tests/mapping.rs"]
mod routing_test_mapping;

#[cfg(test)]
#[path = "tests/multi_qubit.rs"]
mod routing_test_multi_qubit;

#[cfg(test)]
#[path = "tests/noise_aware.rs"]
mod routing_test_noise_aware;

#[cfg(test)]
#[path = "tests/sabre.rs"]
mod routing_test_sabre;

#[cfg(test)]
#[path = "tests/shortest_path.rs"]
mod routing_test_shortest_path;

#[cfg(test)]
#[path = "tests/topology.rs"]
mod routing_test_topology;

#[cfg(test)]
#[path = "tests/transactional.rs"]
mod routing_test_transactional;

#[cfg(test)]
#[path = "tests/verification.rs"]
mod routing_test_verification;