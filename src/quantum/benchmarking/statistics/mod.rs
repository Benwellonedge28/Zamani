//! Zamani Quantum Benchmarking — Statistics
//!
//! Public module boundary for protocol-independent statistical primitives used
//! by Zamani's quantum-benchmarking subsystem.
//!
//! The statistics subsystem consumes numerical observations and produces
//! auditable statistical estimates. It does not generate or execute quantum
//! circuits and does not depend on a particular simulator, hardware provider,
//! Quantum IR, frontend, routing system, scheduler, or runtime.
//!
//! # Architecture
//!
//! ```text
//! benchmark protocol
//!       │
//!       ▼
//! raw observations
//!       │
//!       ▼
//! statistics
//! ┌─────┼──────────┬──────────┬──────────┬──────────┬──────────┐
//! ▼     ▼          ▼          ▼          ▼          ▼          ▼
//! dist confidence bootstrap regression hypothesis outliers aggregation
//!       │
//!       ▼
//! statistical results
//!       │
//!       ▼
//! metrics / analysis / reporting
//! ```
//!
//! # Modules
//!
//! - [`distributions`] — probability/distribution primitives.
//! - [`confidence`] — confidence levels and confidence intervals.
//! - [`bootstrap`] — deterministic, bounded bootstrap/resampling.
//! - [`regression`] — regression and decay-model fitting.
//! - [`hypothesis`] — hypothesis tests and statistical decisions.
//! - [`outliers`] — robust and auditable outlier detection.
//! - [`aggregation`] — aggregation across benchmark observations.
//!
//! # Dependency boundary
//!
//! The statistics subsystem MUST remain below benchmark protocols and above
//! raw numerical observations.
//!
//! It must not depend on:
//!
//! - benchmark protocols;
//! - circuit generators;
//! - execution backends;
//! - hardware implementations;
//! - Quantum IR;
//! - frontend/lowering;
//! - algorithms;
//! - routing;
//! - scheduling;
//! - runtime.
//!
//! Statistics may depend on foundational benchmarking infrastructure such as
//! `core::limits` where required to enforce resource limits.
//!
//! # Production requirements
//!
//! Statistical implementations exposed through this module must:
//!
//! - reject NaN and infinity;
//! - validate probability bounds;
//! - validate sample counts;
//! - prevent integer overflow;
//! - enforce benchmark resource limits where applicable;
//! - avoid unbounded allocations;
//! - avoid hidden global state;
//! - use explicit seeds for randomized procedures;
//! - preserve reproducibility;
//! - return structured errors rather than panic;
//! - expose the statistical method used;
//! - expose confidence assumptions;
//! - never silently discard observations;
//! - distinguish statistical estimates from physical interpretations.
//!
//! # Reproducibility
//!
//! Randomized statistical procedures must use explicit deterministic seeds or
//! caller-supplied RNG state. They must never implicitly seed from wall-clock
//! time or process-global state.
//!
//! The existing bootstrap implementation follows this model and documents its
//! explicit seed, bounded-resampling, and deterministic execution contract.
//!
//! # Scientific semantics
//!
//! A statistical result does not by itself establish a physical interpretation.
//!
//! In particular:
//!
//! - confidence intervals are not hypothesis tests;
//! - bootstrap intervals do not establish independence or stationarity;
//! - RB decay parameters depend on protocol assumptions;
//! - XEB estimates depend on the classical-reference model;
//! - correlations do not establish causal attribution;
//! - outlier detection does not authorize silent data deletion.
//!
//! Those interpretations belong to the appropriate benchmark protocol,
//! analysis, and provenance layers.
//!
//! # Integration with Quantum Volume
//!
//! `volume_estimator.rs` is intentionally a pure Quantum Volume mathematical
//! layer. It does not execute circuits or select hardware.
//!
//! The long-term dependency direction is:
//!
//! ```text
//! protocols::quantum_volume
//!          │
//!          ▼
//! volume_estimator / statistics::confidence
//!          │
//!          ▼
//! core::result
//! ```
//!
//! This module therefore does not import `volume_estimator.rs`.
//!
//! That keeps the statistical foundation reusable by Quantum Volume, RB,
//! XEB, cycle benchmarking, application benchmarks, QEC benchmarks, and
//! future benchmark families.
//!
//! # Public API policy
//!
//! This file is deliberately limited to module declarations.
//!
//! It does NOT wildcard-re-export every symbol from every statistics module.
//! Doing so would create namespace collisions and would make the public API
//! increasingly fragile as the benchmarking system grows.
//!
//! Consumers should use explicit paths such as:
//!
//! ```text
//! quantum::benchmarking::statistics::confidence::ConfidenceInterval
//! quantum::benchmarking::statistics::bootstrap::BootstrapEstimate
//! ```
//!
//! This makes statistical provenance and ownership explicit.
//!
//! # Adding a new statistical family
//!
//! A new family should:
//!
//! 1. receive its own file;
//! 2. remain protocol-independent;
//! 3. define structured errors;
//! 4. validate finite inputs;
//! 5. validate numerical bounds;
//! 6. enforce relevant resource limits;
//! 7. document assumptions;
//! 8. provide deterministic tests;
//! 9. provide reproducibility tests if randomized;
//! 10. be declared in this module.
//!
//! Protocol-specific algorithms do not belong in `mod.rs`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.

// ============================================================================
// Statistical modules
// ============================================================================

/// Probability distributions and distribution-level numerical primitives.
pub mod distributions;

/// Confidence levels and confidence-interval calculations.
pub mod confidence;

/// Deterministic, bounded bootstrap and resampling analysis.
pub mod bootstrap;

/// Regression and statistical model fitting.
pub mod regression;

/// Hypothesis tests and explicit statistical decisions.
pub mod hypothesis;

/// Robust, auditable outlier detection.
pub mod outliers;

/// Statistical aggregation across benchmark observations.
pub mod aggregation;