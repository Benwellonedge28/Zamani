
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Universal Runtime — Reliability
//!
//! Path: `src/zur/reliability/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # PURPOSE
//!
//! This module is the public composition root for ZUR reliability.
//!
//! It integrates the independently implemented reliability components
//! responsible for:
//!
//! - Operational fault classification and attribution.
//! - Checkpoint creation, restoration and deletion.
//! - Recovery coordination through canonical recovery authorities.
//! - Future reliability services through explicitly declared modules.
//!
//! Reliability provides operational coordination across:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware execution.
//! - AI and tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Accelerators.
//! - Embedded computing.
//! - Networking.
//! - Cryptographic computation.
//! - Future computational domains.
//!
//! # OWNERSHIP
//!
//! This composition root owns:
//!
//! - Reliability module composition.
//! - Public reliability namespace organization.
//! - Reliability integration documentation.
//! - Stable reliability architecture identifiers.
//! - Namespace-level architectural invariants.
//!
//! It does NOT own:
//!
//! - Source-language failure semantics.
//! - Grammar interpretation.
//! - Semantic analysis.
//! - Capability definitions.
//! - Resource feasibility.
//! - Resource reservation.
//! - Physical resource allocation.
//! - Provider discovery.
//! - Hardware diagnostics.
//! - Quantum fault models.
//! - Quantum error correction.
//! - Checkpoint serialization.
//! - Checkpoint storage.
//! - Distributed membership.
//! - Distributed transport.
//! - Recovery planning.
//! - Recovery journals.
//! - Retry policy.
//! - Execution lifecycle.
//! - Provider-specific recovery.
//!
//! Those responsibilities remain with their existing authoritative
//! repository subsystems.
//!
//! # CANONICAL INTEGRATION AUTHORITIES
//!
//! ## ZUR
//!
//! - `crate::zur::error`
//! - `crate::zur::ids`
//! - `crate::zur::execution`
//! - `crate::zur::contracts`
//! - `crate::zur::core`
//!
//! ## Distributed computing
//!
//! - `crate::distributed::error`
//! - `crate::distributed::ids`
//! - `crate::distributed::resilience::checkpoint`
//! - `crate::distributed::resilience::recovery`
//! - `crate::distributed::resilience`
//!
//! ## Other computational domains
//!
//! - `crate::quantum::resilience`
//! - `crate::hybrid`
//! - `crate::backend`
//! - `crate::compiler`
//!
//! These subsystems retain their own semantic and operational
//! responsibilities.
//!
//! # RELIABILITY MODULES
//!
//! `faults`:
//!     Structured operational fault classification and attribution.
//!
//! `checkpoint`:
//!     ZUR integration with canonical checkpoint authorities.
//!
//! `recovery`:
//!     ZUR integration with canonical recovery authorities.
//!
//! Additional reliability components must be introduced only when
//! their implementations and ownership contracts exist.
//!
//! # FAILURE AND RECOVERY INVARIANTS
//!
//! 1. An observed fault is not automatically a permanent failure.
//! 2. An unknown execution outcome remains unknown until reconciled.
//! 3. A cancellation request does not prove execution has stopped.
//! 4. A checkpoint is not proof of successful recovery.
//! 5. A recovery request is not proof of successful recovery.
//! 6. A submitted recovery action is not proof of completion.
//! 7. Recovery success requires authoritative verification.
//! 8. Failed cleanup must remain observable.
//! 9. Retry must respect idempotency and side-effect guarantees.
//! 10. Unknown outcomes must be reconciled before potentially
//!     duplicating execution.
//! 11. Security authorization must not be bypassed by recovery.
//! 12. Domain-specific resilience must remain with its domain owner.
//!
//! # SCALABILITY
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Fault records.
//! - Failure models.
//! - Checkpoints.
//! - Recovery incidents.
//! - Recovery actions.
//! - Execution sessions.
//! - Providers.
//! - Devices.
//! - Computational domains.
//! - Distributed participants.
//! - Resource quantities.
//!
//! Capacity is determined by available memory, storage, execution
//! resources, provider capabilities and explicitly configured
//! operational budgets.
//!
//! No finite machine provides literal infinite computation.
//!
//! # POCO-REAF
//!
//! Reliability must preserve:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Recovery coordination must not depend on a fixed:
//!
//! - Processor architecture.
//! - Device inventory.
//! - Quantum processor.
//! - Hardware vendor.
//! - Distributed topology.
//! - Number of participants.
//! - Number of computational domains.
//!
//! New computational domains must be able to integrate through
//! established reliability and provider contracts.
//!
//! # GRAMMAR BOUNDARY
//!
//! Source-level reliability intent belongs to:
//!
//! - `grammar/execution/recovery.g4`
//! - `grammar/execution/resilience.g4`
//!
//! Normative execution semantics belong to:
//!
//! - `grammar/specification/execution-model.md`
//!
//! The grammar describes what reliability behavior a program requires.
//!
//! Semantic analysis validates that intent.
//!
//! Compiler and resilience planning determine valid realizations.
//!
//! Runtime reliability coordinates actual operational behavior.
//!
//! Domain-specific subsystems implement their respective fault
//! detection, correction and state reconstruction.
//!
//! This module MUST NOT parse grammar files or independently
//! reinterpret source-language recovery declarations.
//!
//! # SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No raw pointers.
//! No global mutable reliability state.
//! No hidden runtime initialization.
//! No fabricated execution success.
//! No fabricated recovery success.
//! No implicit resource allocation.
//! No unconditional retry.
//! No swallowed recovery failures.
//!
//! All reliability dependencies must be supplied through their
//! existing explicit integration interfaces.

// ============================================================================
// MODULE COMPOSITION
// ============================================================================

/// Structured operational fault classification and attribution.
///
/// This module does not perform recovery or independently determine
/// whether a failed operation can safely be retried.
pub mod faults;

/// ZUR integration with canonical checkpoint authorities.
///
/// Checkpoint serialization, persistence, integrity and storage
/// remain owned by their existing implementations.
pub mod checkpoint;

/// ZUR integration with canonical recovery authorities.
///
/// Recovery plans, journals, action identities, reconciliation
/// and authoritative recovery verification remain owned by
/// the distributed resilience subsystem.
pub mod recovery;

// ============================================================================
// STABLE ARCHITECTURAL IDENTIFIERS
// ============================================================================

/// Stable identifier for the ZUR reliability namespace.
///
/// This identifies the software architecture, not a hardware target.
pub const MODULE_ID: &str = "zamani.zur.reliability";

/// Stable identifier for the reliability composition boundary.
pub const COMPOSITION_BOUNDARY_ID: &str =
    "zamani.zur.reliability.composition";

/// Reliability architecture's scalability policy.
///
/// This is descriptive metadata, not a numerical capacity limit.
pub const SCALABILITY_POLICY: &str =
    "resource-bounded-by-available-capabilities";

/// Reliability architecture's safety policy.
pub const SAFETY_POLICY: &str = "safe-rust-no-unsafe";

/// Reliability architecture's recovery verification policy.
pub const RECOVERY_VERIFICATION_POLICY: &str =
    "authoritative-verification-required";

/// Reliability architecture's retry policy.
pub const RETRY_POLICY: &str =
    "explicit-idempotency-and-reconciliation-required";

/// Reliability architecture's failure reporting policy.
pub const FAILURE_REPORTING_POLICY: &str =
    "preserve-primary-and-secondary-failures";

// ============================================================================
// NAMESPACE ACCESSORS
// ============================================================================

/// Returns the stable reliability module identifier.
#[must_use]
pub const fn module_id() -> &'static str {
    MODULE_ID
}

/// Returns the stable reliability composition boundary identifier.
#[must_use]
pub const fn composition_boundary_id() -> &'static str {
    COMPOSITION_BOUNDARY_ID
}

/// Returns the reliability scalability policy.
#[must_use]
pub const fn scalability_policy() -> &'static str {
    SCALABILITY_POLICY
}

/// Returns the reliability safety policy.
#[must_use]
pub const fn safety_policy() -> &'static str {
    SAFETY_POLICY
}

/// Returns the recovery verification policy.
#[must_use]
pub const fn recovery_verification_policy() -> &'static str {
    RECOVERY_VERIFICATION_POLICY
}

/// Returns the retry policy.
#[must_use]
pub const fn retry_policy() -> &'static str {
    RETRY_POLICY
}

/// Returns the failure reporting policy.
#[must_use]
pub const fn failure_reporting_policy() -> &'static str {
    FAILURE_REPORTING_POLICY
}

// ============================================================================
// NAMESPACE-ONLY PRELUDE
// ============================================================================

/// Namespace-only prelude for reliability consumers.
///
/// This deliberately exports module namespaces instead of flattening
/// all reliability symbols into one namespace.
///
/// This avoids accidental name collisions and keeps the public API
/// stable when individual reliability implementations evolve.
///
/// Example:
///
/// ```rust
/// use crate::zur::reliability::prelude::{
///     checkpoint,
///     faults,
///     recovery,
/// };
/// ```
///
/// Individual implementations remain accessible through their
/// canonical module paths.
pub mod prelude {
    /// Checkpoint integration namespace.
    pub use super::checkpoint;

    /// Fault classification namespace.
    pub use super::faults;

    /// Recovery integration namespace.
    pub use super::recovery;
}

// ============================================================================
// ARCHITECTURAL INVARIANT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reliability_namespace_has_stable_identity() {
        assert_eq!(
            module_id(),
            "zamani.zur.reliability"
        );

        assert_eq!(
            composition_boundary_id(),
            "zamani.zur.reliability.composition"
        );
    }

    #[test]
    fn reliability_requires_safe_rust() {
        assert_eq!(
            safety_policy(),
            "safe-rust-no-unsafe"
        );
    }

    #[test]
    fn reliability_has_no_fixed_capacity_policy() {
        assert_eq!(
            scalability_policy(),
            "resource-bounded-by-available-capabilities"
        );
    }

    #[test]
    fn recovery_requires_authoritative_verification() {
        assert_eq!(
            recovery_verification_policy(),
            "authoritative-verification-required"
        );
    }

    #[test]
    fn retries_require_idempotency_and_reconciliation() {
        assert_eq!(
            retry_policy(),
            "explicit-idempotency-and-reconciliation-required"
        );
    }

    #[test]
    fn primary_and_secondary_failures_must_be_preserved() {
        assert_eq!(
            failure_reporting_policy(),
            "preserve-primary-and-secondary-failures"
        );
    }

    #[test]
    fn existing_reliability_namespaces_are_available() {
        let _faults = core::any::type_name::<
            faults::FaultCategory
        >();

        let _checkpoint = core::any::type_name::<
            checkpoint::ZurCheckpointAdapter
        >();

        let _recovery = core::any::type_name::<
            recovery::ZurRecoveryCoordinator
        >();
    }

    #[test]
    fn prelude_preserves_independent_module_namespaces() {
        use super::prelude::{
            checkpoint,
            faults,
            recovery,
        };

        let _ = core::any::type_name::<
            faults::FaultCategory
        >();

        let _ = core::any::type_name::<
            checkpoint::ZurCheckpointAdapter
        >();

        let _ = core::any::type_name::<
            recovery::ZurRecoveryCoordinator
        >();
    }
}
