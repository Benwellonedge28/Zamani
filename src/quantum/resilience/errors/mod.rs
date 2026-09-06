//! Zamani Quantum Resilience — Error Module Boundary
//!
//! Path:
//! `src/quantum/resilience/errors/mod.rs`
//!
//! # Purpose
//!
//! This module is the public module boundary for the foundational error
//! contract of `quantum::resilience`.
//!
//! The actual error taxonomy is implemented in [`error`]. This file is
//! intentionally small: it declares the module, re-exports its stable public
//! contract, and contains no resilience business logic.
//!
//! # Architectural ownership
//!
//! ```text
//! quantum::resilience::errors
//!          |
//!          +--> error.rs
//!                |
//!                +--> ResilienceError
//!                +--> ResilienceErrorCode
//!                +--> ResilienceErrorCategory
//!                +--> ResilienceResult<T>
//!                +--> Retryability
//!                +--> Recoverability
//!                +--> Severity
//!                +--> structured diagnostic/context types
//! ```
//!
//! The error implementation is the single source of truth.
//!
//! This module MUST NOT introduce:
//!
//! - another error-code enum;
//! - another resilience result type;
//! - another retryability enum;
//! - another severity enum;
//! - another recoverability enum;
//! - hardware-specific error types;
//! - provider-specific error types;
//! - QEC-specific error types;
//! - routing errors;
//! - scheduling errors;
//! - optimization errors;
//! - fault models;
//! - recovery policies;
//! - detection logic;
//! - planning logic;
//! - telemetry logic.
//!
//! Those concerns belong to their owning subsystems.
//!
//! # Dependency direction
//!
//! ```text
//!                         quantum::resilience
//!                                  |
//!                  +---------------+---------------+
//!                  |               |               |
//!                  v               v               v
//!              detection       planning        recovery
//!                  |               |               |
//!                  +---------------+---------------+
//!                                  |
//!                                  v
//!                    quantum::resilience::errors
//!                                  |
//!                                  v
//!                             error.rs
//! ```
//!
//! The direction is intentionally one-way:
//!
//! ```text
//! resilience subsystem --> errors
//! ```
//!
//! Never:
//!
//! ```text
//! errors --> resilience subsystem
//! ```
//!
//! This makes the error contract independently implementable and prevents
//! circular module dependencies.
//!
//! # Canonical quantum identities
//!
//! Qubit identity is owned by the canonical Quantum IR.
//!
//! The implementation in `error.rs` uses:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module MUST NOT define a resilience-specific `QubitId`.
//!
//! Consequently, logical and physical quantum resources cannot accidentally
//! be represented by the same resilience-local identity type.
//!
//! # Result contract
//!
//! Fallible resilience operations should normally expose:
//!
//! ```text
//! quantum::resilience::errors::ResilienceResult<T>
//! ```
//!
//! which is defined by `error.rs` as:
//!
//! ```text
//! Result<T, ResilienceError>
//! ```
//!
//! This module deliberately does not redefine that alias.
//!
//! # Public API stability
//!
//! Consumers should import errors through this module boundary:
//!
//! ```text
//! use crate::quantum::resilience::errors::ResilienceError;
//! use crate::quantum::resilience::errors::ResilienceResult;
//! ```
//!
//! Rather than reaching through implementation details:
//!
//! ```text
//! quantum::resilience::errors::error::...
//! ```
//!
//! The latter path remains an implementation path and should not become the
//! preferred public API.
//!
//! # Scalability
//!
//! There are no machine-size constants in this module.
//!
//! This boundary therefore imposes no architectural limit on:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of QPUs;
//! - number of backends;
//! - number of recovery attempts;
//! - number of incidents;
//! - number of distributed resources.
//!
//! Resource and execution limits are supplied by the appropriate policy,
//! capability, resource, runtime, and hardware contracts.
//!
//! The module itself performs no allocation proportional to machine size.
//!
//! # Safety
//!
//! Resilience is required to compile without unsafe Rust.
//!
//! The error implementation enforces the safety boundary itself. This module
//! additionally declares the same module-level invariant so that future
//! additions to this module cannot silently introduce unsafe code.
//!
//! ```text
//! no unsafe code
//! ```
//!
//! # Rust compatibility
//!
//! Required toolchain:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration contract
//!
//! The parent resilience module should expose this module with:
//!
//! ```text
//! pub mod errors;
//! ```
//!
//! Other resilience modules may then consume the stable error boundary:
//!
//! ```text
//! use super::errors::{ResilienceError, ResilienceResult};
//! ```
//!
//! or, where appropriate:
//!
//! ```text
//! use crate::quantum::resilience::errors::{ResilienceError, ResilienceResult};
//! ```
//!
//! No changes to this file should be required when a new detector, planner,
//! recovery strategy, mitigation strategy, QEC integration, backend, or
//! hardware provider is added, provided those components use the existing
//! error contract.
//!
//! # Important integration rule
//!
//! `error.rs` is the authoritative location for the error vocabulary.
//!
//! If a future error condition is needed, it belongs in `error.rs`, not in a
//! second error enum introduced here.
//!
//! Likewise, `codes.rs` and `classification.rs` must not be introduced merely
//! to duplicate types already owned by `error.rs`. The repository's resilience
//! design explicitly requires one canonical error-code vocabulary.
//!
//! # Public re-exports
//!
//! Keep these re-exports explicit rather than using a wildcard import. An
//! explicit list prevents accidental expansion of the public API when
//! implementation details are added to `error.rs`.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Module declaration
// =============================================================================

/// Canonical provider-neutral resilience error contract.
///
/// This is the single source of truth for:
/////
/// - stable error codes;
/// - error categories;
/// - severity;
/// - retryability;
/// - recoverability;
/// - structured diagnostics;
/// - resource context;
/// - canonical logical/physical qubit identity;
/// - source-error preservation;
/// - resilience result semantics.
pub mod error;

// =============================================================================
// Stable public API
// =============================================================================

// Re-export the complete stable error contract from this module boundary.
//
// Do not use `pub use error::*;`.
// Explicit re-exports prevent private implementation details from becoming
// part of the public API accidentally.

pub use error::{
    ResilienceError,
    ResilienceErrorCategory,
    ResilienceErrorCode,
    ResilienceResult,
    Retryability,
};

// =============================================================================
// Conditional compatibility aliases
// =============================================================================
//
// The canonical types above intentionally remain the source of truth.
//
// Do not introduce aliases for types whose names are not present in
// `error.rs`. In particular, this module must not invent:
//
// - ResilienceSeverity;
// - ResilienceRecoverability;
// - ErrorCode;
// - ErrorCategory;
// - RecoveryResult;
// - ResilienceResultError.
//
// If the authoritative `error.rs` later exposes additional public semantic
// types, they may be explicitly re-exported here after their API has been
// finalized.
//
// =============================================================================
// Module-level tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_error_types_are_reachable_through_module_boundary() {
        fn assert_error_type<T>()
        where
            T: std::error::Error + Send + Sync + 'static,
        {
        }

        assert_error_type::<ResilienceError>();
    }

    #[test]
    fn canonical_result_alias_is_available() {
        fn accepts_result(_: ResilienceResult<()>) {}

        accepts_result(Ok(()));
    }

    #[test]
    fn canonical_error_code_is_available() {
        let _ = ResilienceErrorCode::InvalidArgument;
    }

    #[test]
    fn canonical_error_category_is_available() {
        let _ = ResilienceErrorCategory::Validation;
    }

    #[test]
    fn canonical_retryability_is_available() {
        let _ = Retryability::Never;
    }
}