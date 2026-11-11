
#![forbid(unsafe_code)]

//! # Zamani Standard Library — Classical Computing Facade
//!
//! Path: `src/stdlib/domains/classical.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Expose the canonical Zamani classical-computing subsystem through
//! the standard-library domain API.
//!
//! This module is a compatibility and public-API facade. It does not
//! implement a second classical-computing subsystem.
//!
//! ## Canonical authority
//!
//! The implementation authority is:
//!
//!     crate::classical
//!
//! Its composition root is:
//!
//!     src/classical/mod.rs
//!
//! The canonical subsystem owns classical algorithms, collections,
//! configuration, diagnostics, errors, identities, numerical
//! functionality, concurrency, memory contracts, optimization,
//! validation, lowering, interoperability, and ZCIR integration.
//!
//! This facade does not redefine those concepts.
//!
//! ## Ownership boundaries
//!
//! This module does not own:
//!
//! - Source grammar, tokens, parsing, or AST construction.
//! - Semantic types, effects, or source-language validation.
//! - Canonical classical or universal IR.
//! - Quantum or HDL representations.
//! - Tensor storage or collection implementations.
//! - Numerical algorithms or mathematical semantics.
//! - Hardware discovery or physical resource allocation.
//! - Backend selection or code generation.
//! - Scheduling, placement, or runtime execution.
//! - Distributed coordination or transport.
//!
//! Those responsibilities remain with their established repository
//! authorities.
//!
//! ## POCO-REAF and scalability
//!
//! This facade introduces no universal limits on:
//!
//! - Collection lengths or tensor ranks.
//! - Vector or matrix dimensions.
//! - Operation or function counts.
//! - Processor, thread, or accelerator counts.
//! - Memory or storage capacity.
//! - Distributed nodes or execution participants.
//! - Register widths or hardware characteristics.
//!
//! Actual operations remain subject to available resources,
//! implementation support, representational limits, target
//! capabilities, and explicitly configured operational budgets.
//!
//! No finite implementation promises literal infinite computation.
//!
//! ## Integration contract
//!
//! The dependency direction is:
//!
//!     Zamani grammar
//!          |
//!          v
//!     Frontend and semantic authority
//!          |
//!          v
//!     Canonical classical subsystem
//!          |
//!          v
//!     Standard-library classical facade
//!          |
//!          v
//!     Standard-library consumers
//!
//! The facade re-exports canonical module paths. It does not copy
//! implementations or introduce wrapper types that could diverge
//! from their canonical counterparts.
//!
//! ## Compatibility
//!
//! Existing imports through `crate::classical` remain valid.
//!
//! Standard-library consumers may additionally use
//! `crate::stdlib::domains::classical`.
//!
//! Both paths refer to the same underlying module implementations.
//!
//! ## Lifecycle
//!
//! This facade has no independent initialization or shutdown work.
//! It must not be added to `initialize_stdlib()` or
//! `shutdown_stdlib()`.
//!
//! Any lifecycle requirements belong to the canonical subsystem
//! or to its actual runtime/provider dependencies.
//!
//! ## Completion requirements
//!
//! This file is ready to freeze when:
//!
//! - The crate root exposes `crate::classical`.
//! - The canonical classical module tree compiles.
//! - The standard-library `domains` module is registered.
//! - All re-exported modules resolve.
//! - Existing classical consumers remain compatible.
//! - Safe Rust and the repository's scalability requirements hold.
//! - Formatting, compilation, tests, and lint checks pass.

// =============================================================================
// Canonical subsystem access
// =============================================================================

/// The canonical classical-computing implementation.
///
/// Use this path when a consumer needs access to the complete canonical
/// classical subsystem.
///
/// No implementation is copied or wrapped here.
pub use crate::classical as canonical;

// =============================================================================
// Public classical API
// =============================================================================

/// Stable classical API entry points.
pub use crate::classical::api;

/// Classical artifact compatibility contracts.
pub use crate::classical::compatibility;

/// Classical configuration and operational-budget contracts.
pub use crate::classical::config;

/// Structured classical diagnostics.
pub use crate::classical::diagnostics;

/// Classical error taxonomy and result types.
pub use crate::classical::error;

/// Classical identity interfaces.
pub use crate::classical::ids;

/// Classical provenance interfaces.
pub use crate::classical::provenance;

/// Classical source-span compatibility interfaces.
pub use crate::classical::span;

// =============================================================================
// Classical computational facilities
// =============================================================================

/// Classical algorithms.
pub use crate::classical::algorithms;

/// Classical collections and data structures.
pub use crate::classical::collections;

/// Classical concurrency facilities.
pub use crate::classical::concurrency;

/// Classical control-flow facilities.
pub use crate::classical::control_flow;

/// Classical execution contracts and planning interfaces.
pub use crate::classical::execution;

/// Classical expression facilities.
pub use crate::classical::expressions;

/// Classical memory contracts and adapters.
pub use crate::classical::memory;

/// Classical numerical facilities.
pub use crate::classical::numeric;

/// Classical optimization facilities.
pub use crate::classical::optimization;

/// Classical type adapters.
pub use crate::classical::types;

/// Classical value operations and adapters.
pub use crate::classical::values;

// =============================================================================
// Integration boundaries
// =============================================================================

/// Classical interoperability interfaces.
pub use crate::classical::interoperability;

/// Classical lowering interfaces.
pub use crate::classical::lowering;

/// Classical validation interfaces.
pub use crate::classical::validation;

/// Classical-facing integration with canonical universal IR.
pub use crate::classical::zcir;

// =============================================================================
// Compile-time integration test
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{api, canonical, config};

    /// Verifies that the facade exposes the canonical configuration
    /// and API without defining competing implementations.
    #[test]
    fn facade_uses_canonical_classical_api() {
        let validate_configuration:
            fn(
                &config::ClassicalConfig,
            ) -> canonical::error::ClassicalResult<()> =
            api::validate_configuration;

        // Taking the function as a typed value verifies the public
        // facade paths and the canonical function signature.
        let _ = validate_configuration;
    }
}
