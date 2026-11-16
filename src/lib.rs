
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani — Universal Computing Language
//!
//! Canonical library-crate composition root.
//!
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Mission
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
//!
//! Zamani provides a unified programming-language and computing architecture
//! for classical, quantum, HDL, hybrid, distributed, AI, tensor, embedded,
//! accelerator and future computational domains.
//!
//! ## Responsibilities
//!
//! This file owns only:
//!
//! - Crate-wide safety and lint policy.
//! - The installed package version accessor.
//! - The public top-level module tree.
//!
//! It does not implement compiler stages, semantic analysis, runtime
//! orchestration, hardware discovery, resource allocation, or execution.
//! Those responsibilities belong to the modules declared below.
//!
//! ## Architecture
//!
//! - `frontend` owns source processing and the canonical frontend boundary.
//! - `compiler` coordinates compilation and target-independent planning.
//! - `zuir` owns the universal intermediate-representation boundary.
//! - `classical`, `quantum`, `hdl`, `ai`, and `hybrid` own their
//!   respective computational domains.
//! - `backend` owns target-specific artifact generation.
//! - `zhal` owns the common physical hardware integration boundary.
//! - `zur` owns runtime orchestration and execution infrastructure.
//! - `distributed` owns distributed-computation infrastructure.
//! - `stdlib` owns shared language and platform contracts.
//! - `zamani_project_config` owns project configuration.
//!
//! The crate root exposes these authorities; it must not create competing
//! implementations or duplicate their public types.
//!
//! ## POCO-REAF and scalability
//!
//! Programs express computational meaning independently of incidental
//! properties of a particular machine. Compatible targets may provide
//! different realizations while preserving the program's specified semantics.
//!
//! The library must not introduce arbitrary universal ceilings on processor
//! counts, devices, accelerators, qubits, memory, storage, tensor rank,
//! register width, distributed participants, or computational domains.
//! Actual execution remains subject to physical resources, implementation
//! capabilities, semantic requirements, operational budgets, and policy.
//! Literal infinite computation or storage is not promised.
//!
//! ## Safety
//!
//! Unsafe Rust is prohibited throughout the library crate. External inputs,
//! provider data, and serialized representations must be validated by their
//! owning subsystem at the appropriate boundary.

// ============================================================================
// PACKAGE IDENTITY
// ============================================================================

/// Installed Cargo package version.
///
/// This is distinct from the Zamani language, grammar, compiler API, IR
/// schema, provider API, and hardware firmware versions.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the installed Cargo package version.
#[must_use]
pub const fn version() -> &'static str {
    VERSION
}

// ============================================================================
// LANGUAGE FRONTEND
// ============================================================================

/// Canonical native frontend: source processing, AST and frontend contracts.
pub mod frontend;

// ============================================================================
// COMPILATION AND INTERMEDIATE REPRESENTATION
// ============================================================================

/// Compiler composition and compilation services.
pub mod compiler;

/// Universal intermediate representation.
pub mod zuir;

// ============================================================================
// COMPUTATIONAL DOMAINS
// ============================================================================

/// Classical computation.
pub mod classical;

/// Quantum computation and quantum hardware integration.
pub mod quantum;

/// Hardware-description and hardware-realization domain.
pub mod hdl;

/// Artificial intelligence and machine-learning computation.
pub mod ai;

/// Hybrid computation across supported computational domains.
pub mod hybrid;

/// Distributed computation.
pub mod distributed;

// ============================================================================
// TARGET AND PHYSICAL HARDWARE INTEGRATION
// ============================================================================

/// Target-specific backend implementations and artifact generation.
pub mod backend;

/// Universal physical hardware abstraction and provider integration.
pub mod zhal;

// ============================================================================
// RUNTIME AND SHARED CONTRACTS
// ============================================================================

/// Universal runtime and execution infrastructure.
pub mod zur;

/// Shared standard-library contracts and services.
pub mod stdlib;

// ============================================================================
// PROJECT CONFIGURATION
// ============================================================================

/// Zamani project and manifest configuration.
pub mod zamani_project_config;

// ============================================================================
// CRATE-ROOT CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{version, VERSION};

    #[test]
    fn package_version_is_available() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn version_accessor_matches_package_version() {
        assert_eq!(version(), VERSION);
    }
}
