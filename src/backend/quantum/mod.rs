
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Quantum backend integration boundary for Zamani.
//!
//! Path: `src/backend/quantum/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//!
//! # Purpose
//!
//! Compose quantum-specific backend functionality while reusing the
//! repository's existing compatibility contracts and canonical quantum IR.
//!
//! This module is intentionally a composition boundary. It does not create
//! another IR, compatibility evaluator, capability registry, resource model,
//! target selector, hardware allocator, or quantum execution runtime.
//!
//! # Ownership
//!
//! - `compatibility`: quantum-specific compatibility requests, target
//!   evidence, and reports.
//! - `crate::backend::compatibility`: canonical compatibility evaluation,
//!   shared capability and resource identifiers, evidence, and policy.
//! - `crate::quantum::ir`: canonical hardware-independent quantum IR.
//! - `crate::quantum::optimization`: quantum optimization.
//! - `crate::quantum::routing`: quantum routing.
//! - `crate::quantum::scheduling`: quantum scheduling.
//! - `crate::quantum::error_correction` and `crate::quantum::resilience`:
//!   domain-specific fault tolerance and resilience.
//! - `crate::quantum::hardware`: hardware-specific realization contracts.
//!
//! # Architectural invariants
//!
//! 1. Source semantics and canonical IR are independent of the selected
//!    quantum processor, simulator, vendor, or physical topology.
//! 2. Compatibility is evaluated by the shared backend evaluator.
//! 3. Unknown or partial target evidence is never silently promoted to
//!    full support for a mandatory requirement.
//! 4. Resource capacities are reported through explicit evidence, not
//!    universal hardware limits.
//! 5. Logical qubit identity is not replaced by physical placement.
//! 6. Lowering, decomposition, placement, routing, scheduling, calibration,
//!    pulse generation, and error correction remain with their authoritative
//!    domain subsystems.
//! 7. Unsupported semantics must produce an explicit compatibility failure.
//! 8. No unsafe Rust is permitted.
//!
//! # Integration order
//!
//! Verified semantic facts or canonical quantum IR
//!     -> quantum compatibility request
//!     -> target capability/resource evidence
//!     -> shared compatibility evaluation
//!     -> backend selection and validation
//!     -> target-specific lowering and artifact generation
//!
//! The existence of a compatibility profile does not establish that a
//! complete quantum code-generation or execution path is implemented.

/// Quantum-specific adapter over the canonical backend compatibility model.
pub mod compatibility;

// Re-export the stable quantum-facing compatibility API at
// `crate::backend::quantum::*`. The implementation remains owned by
// `compatibility.rs`; these exports do not create duplicate types.
pub use compatibility::{
    QuantumCompatibilityError,
    QuantumCompatibilityReport,
    QuantumCompatibilityRequest,
    QuantumCompatibilityResult,
    QuantumFeatureRequirement,
    QuantumResourceRequirement,
    QuantumTargetProfile,
};

/// Canonical shared policy and evidence types commonly needed by quantum
/// backend integrations.
///
/// These are re-exports of the shared backend model, not quantum-specific
/// replacements.
pub use super::compatibility::{
    CompatibilityPolicy,
    CompatibilityStatus,
    InventoryCompleteness,
    RequirementStrength,
    ResourceAvailability,
    SupportLevel,
};

/// Convenient imports for quantum backend compatibility consumers.
///
/// This prelude deliberately excludes IR, routing, scheduling, and physical
/// device types. Consumers should import those from their canonical owners.
pub mod prelude {
    pub use super::{
        CompatibilityPolicy,
        CompatibilityStatus,
        InventoryCompleteness,
        QuantumCompatibilityError,
        QuantumCompatibilityReport,
        QuantumCompatibilityRequest,
        QuantumCompatibilityResult,
        QuantumFeatureRequirement,
        QuantumResourceRequirement,
        QuantumTargetProfile,
        RequirementStrength,
        ResourceAvailability,
        SupportLevel,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn quantum_backend_module_exposes_shared_contracts() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(
                QuantumFeatureRequirement::required(
                    "quantum.measurement",
                )
                .expect("valid quantum feature identifier"),
            );

        let target = QuantumTargetProfile::new(
            "test.quantum.target",
        )
        .expect("valid target identifier");

        assert_eq!(request.feature_count(), 1);
        assert_eq!(request.resource_count(), 0);
        assert_eq!(
            target.inventory_completeness(),
            InventoryCompleteness::Partial,
        );
    }

    #[test]
    fn unknown_target_evidence_is_not_full_compatibility() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(
                QuantumFeatureRequirement::required(
                    "quantum.dynamic_control",
                )
                .expect("valid quantum feature identifier"),
            );

        let target = QuantumTargetProfile::new(
            "test.quantum.unknown",
        )
        .expect("valid target identifier");

        let report = request.evaluate(&target);

        // The target has not declared support for the required feature.
        // A partial inventory means support is unknown, not proven absent.
        assert!(!report.is_fully_compatible());
    }
}
