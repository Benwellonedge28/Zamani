
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Hardware Capability Architecture
//!
//! Path: `src/zhal/capabilities/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! This module is the composition root for ZHAL's physical hardware
//! capability architecture.
//!
//! It connects hardware capability evidence, hardware-facing
//! requirements, availability assessments and compatibility reports
//! through a common, extensible public API.
//!
//! It provides the capability integration boundary between:
//!
//! - The Zamani frontend and semantic architecture.
//! - The standard-library capability authority.
//! - Classical hardware.
//! - Quantum hardware.
//! - HDL and programmable hardware.
//! - AI and computational accelerators.
//! - Embedded hardware.
//! - Distributed hardware.
//! - Future computational technologies.
//! - The Zamani Universal Runtime.
//!
//! This module composes existing implementations.
//! It does not implement a second capability subsystem.
//!
//! # 2. ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Capability subsystem composition.
//! - Public capability module registration.
//! - Stable capability API exports.
//! - Capability API documentation.
//! - Capability subsystem schema identification.
//! - Capability subsystem architectural contracts.
//!
//! This module does NOT own:
//!
//! - Capability identity.
//! - Capability definitions.
//! - Source-language capability declarations.
//! - Source-language semantic requirements.
//! - Hardware discovery.
//! - Hardware provider registration.
//! - Hardware device discovery.
//! - Physical resource allocation.
//! - Hardware scheduling.
//! - Backend selection.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Hybrid execution.
//! - Physical hardware access.
//! - Cryptographic verification.
//! - Runtime feasibility decisions.
//!
//! # 3. CANONICAL AUTHORITIES
//!
//! Standard capability identity and descriptions:
//!
//!     crate::stdlib::capability
//!
//! Standard capability evidence and availability:
//!
//!     crate::stdlib::capability
//!
//! Physical hardware capability evidence:
//!
//!     crate::zhal::capabilities::evidence
//!
//! Hardware-facing requirements:
//!
//!     crate::zhal::capabilities::requirements
//!
//! Hardware availability assessment:
//!
//!     crate::zhal::capabilities::availability
//!
//! Hardware compatibility assessment:
//!
//!     crate::zhal::capabilities::compatibility
//!
//! Provider identity:
//!
//!     crate::stdlib::provider
//!
//! Hardware device identity:
//!
//!     crate::zhal::identity
//!
//! Hardware errors:
//!
//!     crate::zhal::errors
//!
//! Source-language semantic requirements:
//!
//!     crate::frontend::semantic
//!
//! Runtime resource management:
//!
//!     crate::zur
//!
//! Domain-specific capability authorities:
//!
//!     crate::classical
//!     crate::quantum
//!     crate::hdl
//!     crate::ai
//!     crate::distributed
//!     crate::hybrid
//!
//! # 4. MODULE RESPONSIBILITIES
//!
//! ## evidence
//!
//! Owns provenance-preserving physical hardware capability observations.
//!
//! Records:
//!
//! - Canonical capability identity.
//! - Capability status.
//! - Evidence origin.
//! - Provider attribution.
//! - Device attribution.
//! - Observation timestamp.
//! - Evidence validity.
//! - Verification classification.
//! - Optional confidence assessment.
//! - Evidence descriptions.
//!
//! Evidence does not independently authenticate providers or guarantee
//! that a device remains operational.
//!
//! ## requirements
//!
//! Owns the hardware-facing representation of capability requirements.
//!
//! It preserves the canonical capability identity and requirement
//! provenance without becoming a second semantic model.
//!
//! Source-level semantic requirements must be translated into this
//! representation through explicit integration adapters.
//!
//! ## availability
//!
//! Owns deterministic assessments of supplied hardware capability
//! evidence.
//!
//! It distinguishes:
//!
//! - Available.
//! - Unavailable.
//! - Conditional.
//! - Unknown.
//! - Stale.
//! - Temporarily inaccessible.
//! - Not yet observed.
//!
//! Unknown or stale evidence must never silently establish availability.
//!
//! ## compatibility
//!
//! Owns structured comparisons between hardware-facing requirements
//! and supplied hardware capability evidence.
//!
//! It preserves:
//!
//! - Requirement identity.
//! - Evidence provenance.
//! - Device and provider scope.
//! - Availability.
//! - Verification requirements.
//! - Compatibility findings.
//! - Indeterminate outcomes.
//!
//! Compatibility does not reserve resources, select hardware or
//! guarantee successful execution.
//!
//! # 5. FUNDAMENTAL ARCHITECTURAL DISTINCTIONS
//!
//! Requirement:
//! A mandatory condition that a valid realization must satisfy.
//!
//! Capability:
//! An ability that an execution environment can provide.
//!
//! Preference:
//! A desirable but non-mandatory property.
//!
//! Constraint:
//! A mandatory restriction on a realization.
//!
//! Hint:
//! Non-binding implementation guidance.
//!
//! Feasibility:
//! An evidence-based assessment of whether mandatory conditions
//! can currently be satisfied.
//!
//! These concepts must not be silently converted into one another.
//!
//! Preferences and hints must not override requirements or constraints.
//!
//! # 6. DOMAIN INDEPENDENCE
//!
//! Capability identifiers are owned by the standard library and
//! remain extensible.
//!
//! This composition root does not enumerate:
//!
//! - CPU architectures.
//! - GPU architectures.
//! - QPU architectures.
//! - FPGA families.
//! - ASIC implementations.
//! - Accelerator vendors.
//! - Embedded platforms.
//! - Distributed computing technologies.
//! - Neuromorphic architectures.
//! - Photonic architectures.
//! - Future computational substrates.
//!
//! New capabilities must be introduced through the existing
//! extensible capability identity and provider contracts.
//!
//! Adding a hardware provider must not require modifying this file.
//!
//! # 7. SCALABILITY AND POCO-REAF
//!
//! This module imposes no arbitrary universal limits on:
//!
//! - Capability count.
//! - Provider count.
//! - Device count.
//! - Evidence count.
//! - Requirement count.
//! - Computational domains.
//! - Hardware categories.
//! - Namespace depth.
//! - Hardware resource quantities.
//! - Hardware topology.
//! - Observation history.
//!
//! No fixed MAX_* hardware limits are introduced.
//!
//! Actual collections grow according to available memory,
//! platform representation limits and explicitly configured
//! operational budgets.
//!
//! This architecture does not promise literal infinite storage
//! or computation.
//!
//! It avoids artificial universal ceilings that would prevent
//! compatible programs from scaling when resources become available.
//!
//! # 8. GRAMMAR BOUNDARY
//!
//! The grammar defines source-level hardware and capability intent.
//!
//! The frontend parses and validates that intent.
//!
//! The semantic layer owns canonical semantic requirements.
//!
//! ZHAL describes physical hardware capability observations.
//!
//! The compiler and runtime consume these descriptions through
//! explicit integration contracts.
//!
//! Relevant grammar authorities include:
//!
//!     grammar/effects/hardware.g4
//!     grammar/types/capability.g4
//!     grammar/types/hardware.g4
//!     grammar/hdl/hardware-software-integration.g4
//!     grammar/resources/
//!     grammar/core/capabilities.g4
//!
//! This module does not:
//!
//! - Parse grammar productions.
//! - Create AST nodes.
//! - Resolve source-level names.
//! - Interpret source expressions.
//! - Perform semantic analysis.
//! - Introduce new source-language syntax.
//! - Select physical hardware.
//!
//! Grammar changes must not be required merely to introduce a new
//! physical hardware provider or capability implementation.
//!
//! # 9. DEPENDENCY DIRECTION
//!
//! Intended integration:
//!
//!     Zamani source
//!          |
//!          v
//!     Frontend and grammar
//!          |
//!          v
//!     Semantic requirements
//!          |
//!          v
//!     Compiler and planning
//!          |
//!          v
//!     ZUIR and domain IRs
//!          |
//!          v
//!     ZUR and resource orchestration
//!          |
//!          v
//!     ZHAL
//!          |
//!          +--> Capability evidence
//!          +--> Hardware requirements
//!          +--> Availability assessment
//!          +--> Compatibility assessment
//!          |
//!          v
//!     Hardware providers
//!          |
//!          +--> Classical
//!          +--> Quantum
//!          +--> HDL
//!          +--> Accelerators
//!          +--> Embedded
//!          +--> Distributed
//!          +--> Future hardware
//!
//! Domain-specific authorities retain their own execution semantics.
//!
//! # 10. SECURITY
//!
//! Capability evidence is not automatically trusted.
//!
//! A provider-reported capability is not equivalent to an
//! independently verified capability.
//!
//! Evidence verification classifications must originate from
//! the appropriate security or attestation authority.
//!
//! This module does not perform authentication, authorization,
//! signature verification or hardware attestation.
//!
//! # 11. DETERMINISM
//!
//! This composition root performs no:
//!
//! - Hardware discovery.
//! - Clock access.
//! - Network operations.
//! - Filesystem operations.
//! - Provider initialization.
//! - Global mutable-state initialization.
//! - Resource allocation.
//!
//! Evaluation functions in the child modules receive their
//! observations and evaluation times explicitly.
//!
//! This keeps capability assessments independently testable.
//!
//! # 12. PUBLIC API POLICY
//!
//! All child modules have stable named paths.
//!
//! Commonly used public types and functions are explicitly re-exported.
//!
//! Wildcard re-exports are prohibited.
//!
//! The standard-library CapabilityId is re-exported as the same
//! canonical type, not wrapped or redefined.
//!
//! # 13. INTEGRATION CONTRACT
//!
//! Hardware providers supply capability evidence.
//!
//! Device models reference capability snapshots.
//!
//! Hardware requirements preserve semantic provenance.
//!
//! Availability assessments consume requirements and evidence.
//!
//! Compatibility assessments consume requirements and evidence.
//!
//! ZUR remains responsible for resource allocation and orchestration.
//!
//! Domain adapters preserve quantum, HDL, classical and accelerator
//! semantics.
//!
//! # 14. SAFETY
//!
//! Unsafe Rust is forbidden.
//!
//! This module introduces no unsafe implementation.
//!
//! # 15. FREEZE CRITERIA
//!
//! This module is complete when:
//!
//! - All child modules exist.
//! - All child modules compile.
//! - All public re-exports resolve.
//! - Canonical capability identity is reused.
//! - No competing semantic authority is introduced.
//! - Grammar ownership remains unchanged.
//! - No arbitrary hardware ceilings are introduced.
//! - Provider integration remains open-world.
//! - No unsafe Rust is introduced.
//! - All subsystem tests pass.
//! - The complete crate passes formatting, checking,
//!   testing and Clippy validation.

// ============================================================================
// MODULE REGISTRATION
// ============================================================================

/// Physical hardware capability evidence.
///
/// Owns provider-attributed and device-attributed hardware
/// capability observations, verification classifications,
/// timestamps, validity and provenance.
pub mod evidence;

/// Hardware-facing capability requirements.
///
/// Owns the representation and validation of hardware capability
/// requirements without duplicating source-language semantics.
pub mod requirements;

/// Hardware capability availability assessment.
///
/// Owns evidence-based availability classifications and
/// requirement availability reports.
pub mod availability;

/// Hardware capability compatibility assessment.
///
/// Owns structured compatibility findings between hardware-facing
/// requirements and physical capability evidence.
pub mod compatibility;

// ============================================================================
// CANONICAL CAPABILITY IDENTITY
// ============================================================================

/// Reuses the standard-library capability identity.
///
/// This is a re-export of the existing canonical type.
/// It does not introduce another capability identity authority.
pub use crate::stdlib::capability::CapabilityId;

// ============================================================================
// EVIDENCE PUBLIC API
// ============================================================================

/// Commonly used hardware capability evidence types.
///
/// The complete implementation remains available through
/// [`evidence`].
pub use evidence::{
    EvidenceConfidence,
    EvidenceFreshness,
    EvidenceOrigin,
    EvidenceTimestamp,
    EvidenceVerification,
    HardwareCapabilityEvidence,
    HardwareCapabilityEvidenceSet,
};

// ============================================================================
// REQUIREMENTS PUBLIC API
// ============================================================================

/// Commonly used hardware capability requirement types.
///
/// The complete implementation remains available through
/// [`requirements`].
pub use requirements::{
    HardwareCapabilityRequirement,
    HardwareCapabilityRequirementSet,
    RequirementError,
    RequirementField,
    RequirementOrigin,
};

// ============================================================================
// AVAILABILITY PUBLIC API
// ============================================================================

/// Commonly used hardware capability availability types.
///
/// The complete implementation remains available through
/// [`availability`].
pub use availability::{
    AccessObservation,
    AvailabilityAssessment,
    AvailabilityState,
    EvidenceCounts,
    RequirementAvailability,
    RequirementAvailabilityReport,
    assess_capability,
    assess_requirement,
    assess_requirements,
};

// ============================================================================
// COMPATIBILITY PUBLIC API
// ============================================================================

/// Commonly used hardware capability compatibility types.
///
/// The complete implementation remains available through
/// [`compatibility`].
pub use compatibility::{
    CompatibilityFinding,
    CompatibilityFindingKind,
    CompatibilityPolicy,
    CompatibilityReport,
    CompatibilityScope,
    CompatibilityStatus,
    VerificationCounts,
    evaluate_compatibility,
    evaluate_requirement,
};

// ============================================================================
// PUBLIC COMPONENT CONTRACT
// ============================================================================

/// Stable identity of the ZHAL capability composition component.
pub const COMPONENT: &str = "zhal.capabilities";

/// Capability subsystem composition schema version.
///
/// This identifies the public composition contract, not the
/// language, package, provider or hardware firmware version.
pub const CAPABILITIES_SCHEMA_VERSION: &str = "1";

/// Returns the stable component identity.
#[must_use]
pub const fn component_id() -> &'static str {
    COMPONENT
}

/// Returns the capability subsystem schema version.
#[must_use]
pub const fn schema_version() -> &'static str {
    CAPABILITIES_SCHEMA_VERSION
}

/// Returns the number of registered capability implementation modules.
///
/// This is a count of architectural modules, not a hardware capability
/// or device limit.
#[must_use]
pub const fn implementation_module_count() -> usize {
    4
}

/// Returns the capability subsystem safety policy.
#[must_use]
pub const fn safety_policy() -> &'static str {
    "unsafe Rust forbidden"
}

/// Returns the capability subsystem extensibility policy.
#[must_use]
pub const fn extensibility_policy() -> &'static str {
    "open-world capability identifiers and provider implementations"
}

// ============================================================================
// PRELUDE
// ============================================================================

/// Common capability types for downstream ZHAL consumers.
///
/// This prelude intentionally exposes only commonly used types.
/// Specialized implementations remain available through their
/// respective child modules.
pub mod prelude {
    pub use super::CapabilityId;

    pub use super::evidence::{
        EvidenceConfidence,
        EvidenceFreshness,
        EvidenceOrigin,
        EvidenceTimestamp,
        EvidenceVerification,
        HardwareCapabilityEvidence,
        HardwareCapabilityEvidenceSet,
    };

    pub use super::requirements::{
        HardwareCapabilityRequirement,
        HardwareCapabilityRequirementSet,
        RequirementError,
        RequirementField,
        RequirementOrigin,
    };

    pub use super::availability::{
        AccessObservation,
        AvailabilityAssessment,
        AvailabilityState,
        EvidenceCounts,
        RequirementAvailability,
        RequirementAvailabilityReport,
        assess_capability,
        assess_requirement,
        assess_requirements,
    };

    pub use super::compatibility::{
        CompatibilityFinding,
        CompatibilityFindingKind,
        CompatibilityPolicy,
        CompatibilityReport,
        CompatibilityScope,
        CompatibilityStatus,
        VerificationCounts,
        evaluate_compatibility,
        evaluate_requirement,
    };
}

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_identity_is_stable() {
        assert_eq!(
            component_id(),
            "zhal.capabilities"
        );
    }

    #[test]
    fn schema_version_is_available() {
        assert_eq!(
            schema_version(),
            "1"
        );
    }

    #[test]
    fn all_capability_modules_are_registered() {
        assert_eq!(
            implementation_module_count(),
            4
        );
    }

    #[test]
    fn safety_policy_forbids_unsafe_rust() {
        assert_eq!(
            safety_policy(),
            "unsafe Rust forbidden"
        );
    }

    #[test]
    fn extensibility_policy_is_open_world() {
        assert!(
            extensibility_policy().contains("open-world")
        );
    }

    #[test]
    fn canonical_capability_identity_is_reusable() {
        let capability = CapabilityId::new(
            "quantum.computation"
        )
        .expect("valid canonical capability identifier");

        assert_eq!(
            capability.as_str(),
            "quantum.computation"
        );
    }

    #[test]
    fn future_capability_namespaces_are_supported() {
        let capability = CapabilityId::new(
            "future.photonic.computation"
        )
        .expect("valid extensible capability identifier");

        assert_eq!(
            capability.as_str(),
            "future.photonic.computation"
        );
    }

    #[test]
    fn capability_module_paths_are_available() {
        let _: fn() -> &'static str = component_id;
        let _: fn() -> &'static str = schema_version;

        let _: Option<HardwareCapabilityEvidence> = None;
        let _: Option<HardwareCapabilityRequirement> = None;
        let _: Option<AvailabilityAssessment> = None;
        let _: Option<CompatibilityReport> = None;
    }
}
