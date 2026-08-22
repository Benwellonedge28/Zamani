//! Zamani Quantum Algorithms.
//!
//! # Purpose
//!
//! This module is the public module boundary for the backend-independent
//! quantum-algorithm subsystem.
//!
//! The algorithms layer describes logical quantum computation. It does not
//! own quantum hardware, physical topology, routing, transpilation, error
//! correction, or backend execution.
//!
//! # Architectural boundary
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │              quantum::algorithms             │
//! │                                             │
//! │  variational ──┬── VQE                     │
//! │                 └── QAOA                   │
//! │                                             │
//! │  Grover                                   │
//! │  Amplitude Amplification / Estimation      │
//! │  Phase Estimation                         │
//! └──────────────────────┬──────────────────────┘
//!                        │
//!                        ▼
//!                 quantum::ir
//!                        │
//!                        ▼
//!              validation / transpilation
//!                        │
//!                        ▼
//!                   routing
//!                        │
//!                        ▼
//!             error-correction boundary
//!                        │
//!                        ▼
//!                    execution
//!                        │
//!                        ▼
//!                  hardware/QPU
//! ```
//!
//! # Module responsibilities
//!
//! - [`error`] — stable algorithm-wide error vocabulary.
//! - [`types`] — stable shared algorithm data contracts.
//! - [`execution`] — backend-independent execution boundary.
//! - [`objective`] — objective-function abstractions.
//! - [`optimizer`] — optimization contracts and implementations.
//! - [`variational`] — generic variational orchestration.
//! - [`vqe`] — Variational Quantum Eigensolver.
//! - [`qaoa`] — Quantum Approximate Optimization Algorithm.
//! - [`grover`] — Grover search.
//! - [`amplitude`] — amplitude amplification and estimation.
//! - [`phase_estimation`] — Quantum Phase Estimation.
//!
//! # Dependency direction
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! error
//!   │
//!   ▼
//! types
//!   │
//!   ├───────────────┐
//!   ▼               ▼
//! execution      objective
//!   │               │
//!   └───────┬───────┘
//!           ▼
//!       optimizer
//!           │
//!           ▼
//!      variational
//!        │      │
//!        ▼      ▼
//!       VQE    QAOA
//!
//! error + types + execution
//!          │
//!          ├──► Grover
//!          ├──► Amplitude
//!          └──► Phase Estimation
//! ```
//!
//! Algorithm modules must not introduce dependencies in the reverse direction.
//!
//! # IR ownership
//!
//! This module does not define or re-export quantum IR primitives merely for
//! convenience. The canonical IR remains under `crate::quantum::ir`.
//!
//! Algorithms may consume the IR through their explicitly defined integration
//! boundaries, while IR semantics remain owned by the IR subsystem.
//!
//! # Execution ownership
//!
//! Backend execution remains behind [`execution::QuantumExecutor`].
//!
//! Algorithm implementations must not directly depend on:
//!
//! - simulator implementations;
//! - CPU/GPU execution engines;
//! - QPU vendors;
//! - network transports;
//! - device calibration;
//! - credentials;
//! - physical topology.
//!
//! # Error ownership
//!
//! Algorithm modules use [`error::AlgorithmError`] and [`error::Result`].
//!
//! Individual algorithms must not introduce competing top-level error
//! vocabularies when an existing algorithm-wide error variant is sufficient.
//!
//! # Public API policy
//!
//! This module intentionally exposes the concrete submodules instead of using
//! wildcard re-exports such as:
//!
//! ```text
//! pub use amplitude::*;
//! pub use execution::*;
//! pub use types::*;
//! ```
//!
//! Wildcard re-exports are avoided because they create:
//!
//! - accidental public API expansion;
//! - name collisions;
//! - unstable API surface;
//! - unnecessary coupling between otherwise independent algorithm files;
//! - future re-edit requirements whenever another module adds a public item.
//!
//! Consumers may use the explicit stable paths:
//!
//! ```text
//! crate::quantum::algorithms::vqe
//! crate::quantum::algorithms::qaoa
//! crate::quantum::algorithms::grover
//! crate::quantum::algorithms::amplitude
//! crate::quantum::algorithms::phase_estimation
//! ```
//!
//! Shared contracts are additionally exposed through their owning modules:
//!
//! ```text
//! crate::quantum::algorithms::error
//! crate::quantum::algorithms::types
//! crate::quantum::algorithms::execution
//! crate::quantum::algorithms::objective
//! crate::quantum::algorithms::optimizer
//! ```
//!
//! This keeps ownership explicit and prevents `mod.rs` from becoming a second
//! definition layer.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required by this module.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// ============================================================================
// Core contracts
// ============================================================================

/// Stable algorithm-wide error vocabulary and `Result` alias.
pub mod error;

/// Stable shared algorithm data contracts.
pub mod types;

/// Backend-independent quantum execution boundary.
pub mod execution;

/// Objective-function abstractions.
pub mod objective;

/// Optimizer abstractions and implementations.
pub mod optimizer;

// ============================================================================
// Variational algorithm layer
// ============================================================================

/// Generic variational-algorithm orchestration.
pub mod variational;

/// Variational Quantum Eigensolver.
pub mod vqe;

/// Quantum Approximate Optimization Algorithm.
pub mod qaoa;

// ============================================================================
// Non-variational algorithm families
// ============================================================================

/// Grover search.
pub mod grover;

/// Amplitude amplification and amplitude estimation.
pub mod amplitude;

/// Quantum Phase Estimation.
pub mod phase_estimation;

// ============================================================================
// Stable common API
// ============================================================================

/// Common algorithm error/result types.
///
/// These are the only common contracts promoted to the algorithms root.
/// Algorithm-specific types remain under their owning modules to avoid
/// namespace collisions and accidental API expansion.
pub use error::{AlgorithmError, Result};

/// Stable algorithm identity and version contracts.
///
/// These are shared across all algorithm implementations and therefore are
/// safe to expose at the subsystem boundary.
pub use types::{AlgorithmId, AlgorithmMetadata, AlgorithmVersion};