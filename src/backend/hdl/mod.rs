
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! HDL backend integration for Zamani.
//!
//! Path: `src/backend/hdl/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the composition root for HDL-specific backend integration.
//! It exposes the existing HDL compatibility contract to the generic backend
//! without introducing a second HDL semantic model, parser, HDLIR, target
//! registry, or hardware-realization implementation.
//!
//! # Ownership
//!
//! - `crate::backend::compatibility` owns generic capability and resource
//!   compatibility semantics.
//! - `crate::backend::hdl::compatibility` owns HDL-specific compatibility
//!   assessment and its evidence requirements.
//! - `crate::hdl` owns the HDL domain and its public subsystem boundaries.
//! - `crate::hdl::hdlir` owns the hardware-domain intermediate representation.
//! - `crate::compiler` owns compilation orchestration.
//! - Existing HDL target, timing, verification, and realization subsystems
//!   own their respective algorithms and authoritative evidence.
//!
//! This module must not duplicate those responsibilities.
//!
//! # Portability and scalability
//!
//! The backend must preserve declared design semantics and distinguish
//! target incompatibility from incomplete evidence and operational failure.
//! It must not introduce universal ceilings on ports, signals, widths,
//! clock domains, memory, devices, or design size.
//!
//! Target-specific constraints must come from explicit target descriptions
//! and verified evidence. Operational budgets are implementation controls,
//! not language-level hardware limits.
//!
//! Literal infinity is not a realizable execution guarantee: compilation
//! remains subject to available resources, representable values, supported
//! semantics, and explicitly configured budgets.
//!
//! # Integration
//!
//! The active `crate::backend` root must declare `pub mod hdl;`.
//! The existing `src/backend/hdl/compatibility.rs` remains the sole
//! implementation of HDL-specific compatibility assessment.
//!
//! The generic compilation pipeline should:
//!
//! 1. Verify the incoming representation using its authoritative verifier.
//! 2. Construct a compatibility request from verified design requirements.
//! 3. Supply the selected target's declared capabilities and resource evidence.
//! 4. Supply validation evidence only from the responsible validators/tools.
//! 5. Evaluate compatibility before target emission.
//! 6. Preserve the report and reject incompatible or insufficiently evidenced
//!    mandatory requirements.
//! 7. Independently validate generated artifacts and downstream realization.
//!
//! A compatible report does not, by itself, prove that synthesis succeeded,
//! timing closed, a bitstream was generated, or physical hardware works.
//!
//! # Module registration policy
//!
//! Only implemented modules with reviewed ownership and dependencies should
//! be declared here. Do not declare speculative child modules merely because
//! a future directory has been proposed.

/// Stable identifier for HDL backend integration.
pub const HDL_BACKEND_DOMAIN_NAME: &str = "zamani.hdl";

/// Version of this HDL backend composition contract.
///
/// This version is independent of the Zamani language version, HDL grammar,
/// frontend semantic model, HDLIR artifact format, and crate version.
pub const HDL_BACKEND_API_VERSION: u16 = 1;

/// HDL target compatibility assessment and validation-evidence contracts.
///
/// This module reuses the shared backend capability and resource model.
/// It does not implement synthesis, simulation, timing analysis, or
/// physical realization.
pub mod compatibility;

/// Public HDL compatibility API.
///
/// These re-exports are convenience paths to the authoritative definitions
/// in [`compatibility`]. They do not introduce duplicate types.
pub use compatibility::{
    EvidenceStatus,
    HdlCapabilityRequirement,
    HdlCompatibilityError,
    HdlCompatibilityEvaluator,
    HdlCompatibilityIssue,
    HdlCompatibilityReport,
    HdlCompatibilityRequest,
    HdlCompatibilityStatus,
    HdlDesignRequirements,
    HdlEvidence,
    HdlIssueCode,
    ResetKind,
};

#[cfg(test)]
mod tests {
    use super::{
        HDL_BACKEND_API_VERSION,
        HDL_BACKEND_DOMAIN_NAME,
        HdlCompatibilityEvaluator,
        HdlCompatibilityRequest,
        HdlDesignRequirements,
        HdlCompatibilityStatus,
    };
    use crate::backend::compatibility::TargetCapabilityProfile;

    #[test]
    fn backend_domain_identity_is_stable() {
        assert_eq!(HDL_BACKEND_DOMAIN_NAME, "zamani.hdl");
        assert!(!HDL_BACKEND_DOMAIN_NAME.is_empty());
    }

    #[test]
    fn backend_api_version_is_nonzero() {
        assert_ne!(HDL_BACKEND_API_VERSION, 0);
    }

    #[test]
    fn compatibility_api_is_reachable_from_the_composition_root() {
        let target = TargetCapabilityProfile::new("test.hdl.target")
            .expect("test target identifier must be valid");

        let request = HdlCompatibilityRequest::new(
            HdlDesignRequirements::new(),
            target,
        );

        let report = HdlCompatibilityEvaluator.evaluate(&request);

        assert_eq!(report.status, HdlCompatibilityStatus::Compatible);
        assert!(report.is_compatible());
    }
}
