//! Zamani Quantum Routing
//!
//! Authoritative module boundary for hardware-aware quantum routing,
//! logical-to-physical mapping, layout, movement generation, routing
//! algorithms, verification, and compiler integration.
//!
//! # Responsibility
//!
//! This module is intentionally a namespace/composition boundary.
//!
//! It owns the public organization of:
//!
//! - routing types;
//! - routing errors;
//! - physical topology;
//! - logical/physical mapping;
//! - routing costs;
//! - routing configuration;
//! - routing results;
//! - graph/path-finding primitives;
//! - routing candidates;
//! - initial layout;
//! - routing algorithms;
//! - routing movement primitives;
//! - routing orchestration;
//! - routing verification;
//! - compiler/IR transpilation integration.
//!
//! It does NOT implement routing itself.
//!
//! The concrete responsibilities remain in their owning modules.
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
//!                    logical quantum circuit
//!                              │
//!                              ▼
//!                     quantum::routing
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!           layout          topology         mapping
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                         algorithms
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!          shortest         lookahead          SABRE
//!             │                │                │
//!             ├────────────────┼────────────────┤
//!             │                │                │
//!             ▼                ▼                ▼
//!           basic          noise-aware       dynamic
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                            moves
//!                              │
//!                              ▼
//!                           router
//!                              │
//!                              ▼
//!                        verification
//!                              │
//!                              ▼
//!                           result
//!                              │
//!                              ▼
//!                    hardware lowering
//! ```
//!
//! # Stable dependency direction
//!
//! The intended dependency direction is:
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
//!                        cost
//!                          │
//!                          ▼
//!                       result
//!                          │
//!          ┌───────────────┼────────────────┐
//!          ▼               ▼                ▼
//!        path          candidates         layout
//!          │               │                │
//!          └───────────────┼────────────────┘
//!                          ▼
//!                     algorithms
//!                          │
//!                          ▼
//!                         moves
//!                          │
//!                          ▼
//!                        router
//!                          │
//!                 ┌────────┴────────┐
//!                 ▼                 ▼
//!          verification       transpiler
//! ```
//!
//! This structure is deliberately layered so later implementation work does
//! not require changing this module merely because another routing algorithm,
//! cost model, hardware target, or compiler integration is added.
//!
//! # Canonical responsibilities
//!
//! ## `types`
//!
//! Owns stable routing vocabulary and identifiers.
//!
//! ## `errors`
//!
//! Owns the canonical routing error contract.
//!
//! ## `topology`
//!
//! Owns physical connectivity and hardware-independent topology semantics.
//!
//! ## `mapping`
//!
//! Owns authoritative logical-to-physical and physical-to-logical placement.
//!
//! ## `cost`
//!
//! Owns routing objective evaluation and cost models.
//!
//! ## `config`
//!
//! Owns routing policy and configuration.
//!
//! ## `result`
//!
//! Owns the immutable routing result and routing metrics.
//!
//! ## `path`
//!
//! Owns graph/path-finding primitives.
//!
//! ## `candidates`
//!
//! Owns routing candidate generation.
//!
//! ## `layout`
//!
//! Owns initial logical-to-physical layout selection.
//!
//! ## `algorithms`
//!
//! Owns the behavioral routing-algorithm contract and concrete algorithms.
//!
//! ## `moves`
//!
//! Owns semantic movement primitives such as SWAP, bridge, and permutation.
//!
//! ## `router`
//!
//! Owns routing orchestration, algorithm selection, limits, and the public
//! routing engine.
//!
//! ## `verification`
//!
//! Owns post-routing correctness verification.
//!
//! ## `transpiler`
//!
//! Owns integration between routing and the compiler/quantum representation.
//!
//! # Important separation
//!
//! Routing must not become a second implementation of:
//!
//! - OpenQASM parsing;
//! - frontend parsing;
//! - Quantum IR;
//! - gate synthesis;
//! - gate decomposition;
//! - scheduling;
//! - pulse generation;
//! - hardware execution;
//! - simulation;
//! - QEC decoding;
//! - benchmark execution.
//!
//! Those responsibilities remain outside this namespace.
//!
//! # Hardware independence
//!
//! Routing consumes hardware topology/capability information through the
//! routing topology and related contracts.
//!
//! It must not directly depend on a particular provider.
//!
//! The same routing engine must therefore be usable with:
//!
//! - simulators;
//! - superconducting devices;
//! - trapped-ion devices;
//! - neutral-atom devices;
//! - photonic targets;
//! - modular quantum processors;
//! - distributed quantum systems;
//! - future Zamani-native hardware.
//!
//! # Directed connectivity
//!
//! Physical adjacency and gate executability are separate concepts.
//!
//! ```text
//! adjacent(p0, p1)
//!       !=
//! supports_gate(gate, p0, p1)
//! ```
//!
//! Consequently, the routing namespace must expose topology and algorithm
//! modules rather than assuming that an undirected graph is sufficient for
//! every operation.
//!
//! # Multi-qubit operations
//!
//! Routing must not silently synthesize arbitrary multi-qubit operations.
//!
//! The intended boundary is:
//!
//! ```text
//! Quantum operation
//!       │
//!       ├── natively supported by target
//!       │             │
//!       │             ▼
//!       │          routing
//!       │
//!       └── unsupported
//!                     │
//!                     ▼
//!              synthesis/decomposition
//! ```
//!
//! This keeps routing independent of gate-synthesis policy.
//!
//! # Movement semantics
//!
//! Routing movement is semantic rather than necessarily a final hardware
//! instruction.
//!
//! For example, a SWAP movement means that logical states exchange physical
//! locations. A later hardware-lowering stage determines whether that movement
//! becomes:
//!
//! - a native SWAP;
//! - a three-CX decomposition;
//! - another equivalent decomposition;
//! - a calibrated provider-specific operation.
//!
//! # Determinism
//!
//! Routing algorithms must honor the deterministic configuration supplied by
//! the routing configuration layer.
//!
//! The root module intentionally does not implement randomness or seed
//! management. Those concerns belong to `config`, `router`, and concrete
//! algorithms.
//!
//! # Transactionality
//!
//! Routing is designed as a transaction:
//!
//! ```text
//! immutable input
//!       │
//!       ▼
//! routing algorithm
//!       │
//!   ┌───┴────┐
//!   ▼        ▼
//! success   failure
//!   │        │
//!   ▼        ▼
//! result    error
//! ```
//!
//! Caller-owned state must not be partially modified by an unsuccessful
//! routing operation.
//!
//! # Verification
//!
//! Routing results are intended to pass through the dedicated verification
//! subsystem.
//!
//! Verification is responsible for checking the invariants that cannot be
//! established merely by module composition, including:
//!
//! - valid physical operands;
//! - valid mapping;
//! - legal topology usage;
//! - legal gate direction;
//! - mapping evolution;
//! - preservation of logical operation semantics;
//! - valid routing operations;
//! - valid final mapping.
//!
//! # Integration with canonical Quantum IR
//!
//! Routing is deliberately kept independent from a particular Quantum IR
//! implementation in its foundational contracts.
//!
//! The `transpiler` module is the integration boundary between the routing
//! subsystem and the compiler/quantum representation.
//!
//! Therefore the dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! routing::transpiler
//!      │
//!      ▼
//! routing::router
//!      │
//!      ▼
//! routing algorithms
//! ```
//!
//! Routing algorithms themselves must not import compiler parser internals.
//!
//! # Integration with hardware
//!
//! Hardware-specific topology, calibration, capabilities, and execution
//! information remain owned by the hardware subsystem.
//!
//! Routing consumes the relevant routing-compatible representation rather than
//! importing provider SDKs.
//!
//! # Integration with optimization
//!
//! Routing is a compiler pass and must remain composable with optimization.
//!
//! A typical production pipeline is:
//!
//! ```text
//! frontend
//!   │
//!   ▼
//! Quantum IR
//!   │
//!   ▼
//! logical optimization
//!   │
//!   ▼
//! layout
//!   │
//!   ▼
//! routing
//!   │
//!   ▼
//! gate synthesis / lowering
//!   │
//!   ▼
//! low-level optimization
//!   │
//!   ▼
//! scheduling
//!   │
//!   ▼
//! hardware
//! ```
//!
//! The actual pipeline may be configured differently by the compiler.
//!
//! # Integration with benchmarking
//!
//! Routing results expose metrics through `result` so the benchmarking
//! subsystem can measure routing overhead without reimplementing routing.
//!
//! Typical measurements include:
//!
//! - inserted movement count;
//! - routed depth;
//! - routing duration;
//! - candidate/search statistics;
//! - hardware-quality estimates;
//! - selected algorithm;
//! - selected layout;
//! - reproducibility information.
//!
//! # Integration with QEC
//!
//! Routing must not own QEC semantics.
//!
//! The foundational routing contracts are nevertheless designed so future
//! QEC-aware routing can distinguish resource roles such as data, ancilla,
//! syndrome, reserved, or unavailable resources where the underlying routing
//! types support them.
//!
//! # Integration with distributed quantum computing
//!
//! The routing namespace deliberately uses abstract physical locations rather
//! than assuming that all physical qubits belong to a single monolithic chip.
//!
//! Future distributed routing can therefore build additional topology/movement
//! abstractions without changing the meaning of ordinary local routing.
//!
//! # Public API policy
//!
//! This module should re-export only APIs that are already stable and actually
//! defined by the child modules.
//!
//! It must NOT:
//!
//! - invent duplicate types;
//! - define duplicate algorithm traits;
//! - duplicate configuration enums;
//! - duplicate routing errors;
//! - duplicate result structures;
//! - create global registries;
//! - introduce provider-specific aliases.
//!
//! Keeping the re-export surface conservative prevents `mod.rs` from becoming
//! an accidental second API implementation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Routing must remain safe Rust.
//!
//! No unsafe implementation is permitted anywhere in this namespace.
//!
//! The root explicitly denies unsafe code so accidental introduction of unsafe
//! code becomes a compilation error.
//!
//! # Module layout
//!
//! ```text
//! src/quantum/routing/
//! ├── mod.rs
//! ├── types.rs
//! ├── errors.rs
//! ├── topology.rs
//! ├── mapping.rs
//! ├── cost.rs
//! ├── config.rs
//! ├── result.rs
//! ├── path.rs
//! ├── candidates.rs
//! ├── layout.rs
//! ├── algorithms/
//! │   ├── mod.rs
//! │   ├── basic.rs
//! │   ├── shortest_path.rs
//! │   ├── lookahead.rs
//! │   ├── sabre.rs
//! │   ├── noise_aware.rs
//! │   └── dynamic.rs
//! ├── moves/
//! │   ├── mod.rs
//! │   ├── swap.rs
//! │   ├── bridge.rs
//! │   └── permutation.rs
//! ├── router.rs
//! ├── verification.rs
//! └── transpiler.rs
//! ```
//!
//! The module declarations below are the authoritative Rust namespace for
//! this directory.
//!
//! # File-completion contract
//!
//! This file is complete when every routing child is declared exactly once
//! and no child implementation needs to modify this namespace merely because
//! another routing component is implemented.
//!
//! Concrete implementation changes belong in their respective files.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Foundational contracts
// =============================================================================

/// Stable routing vocabulary and strongly typed routing identifiers.
pub mod types;

/// Canonical routing errors and diagnostics.
pub mod errors;

/// Physical topology and connectivity representation.
pub mod topology;

/// Authoritative logical-to-physical mapping.
pub mod mapping;

/// Routing cost models and objective evaluation.
pub mod cost;

/// Routing policy and configuration.
pub mod config;

/// Immutable routing result and metrics.
pub mod result;

// =============================================================================
// Routing primitives
// =============================================================================

/// Graph and path-finding primitives.
pub mod path;

/// Routing candidate generation.
pub mod candidates;

/// Initial logical-to-physical layout selection.
pub mod layout;

// =============================================================================
// Routing algorithms
// =============================================================================

/// Concrete routing algorithms and the common routing-algorithm contract.
pub mod algorithms;

// =============================================================================
// Routing movement primitives
// =============================================================================

/// Semantic movement primitives used by routing algorithms.
///
/// The repository currently contains the movement namespace as a dedicated
/// module. Keep this declaration conventional; if the physical filename has
/// a trailing-space artifact in the repository, that filename should be
/// normalized separately rather than encoding filesystem corruption into the
/// production Rust module contract.
pub mod moves;

// =============================================================================
// Routing orchestration
// =============================================================================

/// Public routing engine and algorithm orchestration.
pub mod router;

/// Routing-result verification.
pub mod verification;

/// Compiler/IR integration adapter.
pub mod transpiler;