
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Target Management
//!
//! Path: src/toolchain/targets/mod.rs
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the public composition boundary for target-related
//! toolchain operations.
//!
//! This module coordinates the existing target-profile, target-matrix,
//! compatibility, selection, and validation implementations.
//!
//! It does not implement a second target-management system.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Target identities and computing domains are open-world.
//!
//! The toolchain must accommodate new:
//!
//! - Processor architectures.
//! - Operating systems and execution environments.
//! - Backend implementations.
//! - Hardware providers.
//! - Quantum processors and simulators.
//! - HDL synthesis and simulation targets.
//! - AI and tensor accelerators.
//! - Distributed execution environments.
//! - Embedded and specialized computing environments.
//! - Future computational domains.
//!
//! No universal hardware or computational capacity ceilings
//! are introduced by this module.
//!
//! Actual operations remain subject to available resources,
//! implementation capabilities, semantic requirements,
//! constraints, and explicitly configured operational budgets.
//!
//! Literal infinite physical resources or execution are not promised.
//!
//! # Architectural authorities
//!
//! ## Compiler target identity and intent
//!
//! crate::compiler::configuration::target
//!
//! Owns compiler-facing target identifiers, descriptors,
//! target roles, and target intent.
//!
//! ## Backend target identity and descriptions
//!
//! crate::backend::target
//!
//! Owns canonical backend target descriptions and artifact metadata.
//!
//! ## Capabilities and capability evidence
//!
//! crate::backend::capabilities
//!
//! Owns capability declarations, capability profiles,
//! evidence, and capability matching contracts.
//!
//! ## Workload requirements
//!
//! crate::backend::requirements
//!
//! Owns backend-facing requirements, preferences,
//! constraints, hints, and resource requirements.
//!
//! ## Compatibility
//!
//! crate::backend::compatibility
//!
//! Owns authoritative compatibility evaluation.
//!
//! ## Backend validation
//!
//! crate::backend::validation
//!
//! Owns backend-specific target and artifact validation.
//!
//! ## Target discovery and feasibility
//!
//! crate::compiler::planning
//!
//! Owns target discovery, capability matching,
//! feasibility assessment, and resource planning.
//!
//! ## Generic configuration profiles
//!
//! crate::toolchain::configuration::profiles
//!
//! Owns profile identity, registration, inheritance,
//! resolution, and configuration composition.
//!
//! ## Toolchain configuration
//!
//! crate::toolchain::configuration
//!
//! Owns toolchain configuration and operational policies.
//!
//! ## Toolchain identities
//!
//! crate::toolchain::ids
//!
//! Owns logical toolchain identities.
//!
//! ## Toolchain errors
//!
//! crate::toolchain::errors
//!
//! Owns structured toolchain errors and result types.
//!
//! # Module ownership
//!
//! This composition module owns:
//!
//! - Public target subsystem composition.
//! - Deliberate public API exports.
//! - Target subsystem API identification.
//! - Architectural integration documentation.
//! - The distinction between compiler and backend target identities.
//!
//! It does not own:
//!
//! - Target registration.
//! - Target discovery.
//! - Hardware discovery.
//! - Backend discovery.
//! - Provider registration.
//! - Capability declarations.
//! - Capability matching.
//! - Requirement evaluation.
//! - Constraint solving.
//! - Preference scoring.
//! - Feasibility determination.
//! - Physical resource accounting.
//! - Physical resource allocation.
//! - Device selection.
//! - Compiler target lowering.
//! - Code generation.
//! - Runtime execution.
//! - Deployment execution.
//!
//! # Fundamental distinctions
//!
//! Target intent is not target selection.
//!
//! Target selection is not target realization.
//!
//! A target identifier is not a target descriptor.
//!
//! A target descriptor is not a capability declaration.
//!
//! A capability declaration is not current availability evidence.
//!
//! Compatibility is not physical feasibility.
//!
//! Feasibility is not resource allocation.
//!
//! Successful validation is not successful compilation.
//!
//! Successful compilation is not successful execution.
//!
//! Unknown information must never be interpreted as support.
//!
//! # Requirements, capabilities and policies
//!
//! The target subsystem preserves the following distinctions:
//!
//! - Requirement: mandatory semantic or operational need.
//! - Capability: a property a provider declares or demonstrates.
//! - Preference: a non-mandatory selection preference.
//! - Constraint: a condition that an acceptable realization must satisfy.
//! - Hint: non-binding optimization guidance.
//! - Feasibility: an authoritative assessment of whether a requested
//!   realization can be established using available evidence.
//!
//! These concepts must not be collapsed into one target flag.
//!
//! # Multi-target compilation
//!
//! One logical build may produce multiple target-specific realizations.
//!
//! The target matrix preserves:
//!
//! - Logical build identity.
//! - Source identity.
//! - Dependency identity.
//! - Canonical target identities.
//! - Individual target assessments.
//! - Assessment provenance.
//! - Unresolved and infeasible conditions.
//!
//! Each realization must retain its own evidence.
//!
//! A target matrix must not report universal compatibility merely
//! because multiple targets were requested.
//!
//! # Determinism
//!
//! The composition module introduces no implicit target ordering,
//! hardware probing, random selection, or environment-dependent behavior.
//!
//! Deterministic ordering and selection are delegated to the
//! existing subsystem implementations.
//!
//! # Security
//!
//! This module performs no filesystem, network, environment,
//! process, credential, hardware, or runtime operations.
//!
//! Authorization, provider trust, evidence authentication,
//! and resource protection remain responsibilities of their
//! authoritative subsystems.
//!
//! # Grammar integration
//!
//! Source-level target intent:
//!
//! grammar/compile/target.g4
//!
//! Source-level target-selection policy:
//!
//! grammar/compile/target-selection.g4
//!
//! Source-level reusable profiles:
//!
//! grammar/compile/profiles.g4
//!
//! Compilation grammar composition:
//!
//! grammar/compile/compile.g4
//!
//! These grammars define source-language structures.
//!
//! This module does not parse grammar declarations or introduce
//! a second source-language target model.
//!
//! # Integration contract
//!
//! 1. Frontend parsing produces structured target intent and
//!    selection policy.
//!
//! 2. Semantic analysis validates source-level meaning.
//!
//! 3. Compiler configuration owns compiler-facing target identities.
//!
//! 4. Backend integration resolves explicit mappings to canonical
//!    backend target descriptors.
//!
//! 5. Backend capability providers supply authoritative evidence.
//!
//! 6. Backend compatibility evaluates workload requirements.
//!
//! 7. Compiler planning evaluates feasibility and resource requirements.
//!
//! 8. Target selection orders only acceptable, assessed candidates.
//!
//! 9. Target validation verifies the requested backend preconditions.
//!
//! 10. Target matrices preserve independent target assessments.
//!
//! 11. Build orchestration consumes the resulting validated plans.
//!
//! 12. Backend and runtime authorities perform realization and execution.
//!
//! No stage may silently promote unresolved evidence into support.
//!
//! # API evolution
//!
//! This module exposes a stable composition API version.
//!
//! Individual subsystem API versions remain independently identifiable.
//!
//! A change to one child module must not silently redefine
//! another subsystem's ownership or semantics.

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of the target-management subsystem.
pub const TARGETS_MODULE_NAME: &str =
    "zamani.toolchain.targets";

/// Public composition API version.
///
/// This version identifies the target-management composition boundary.
/// It is independent of the individual child-module API versions,
/// compiler version, language version, grammar version, and backend version.
pub const TARGETS_API_VERSION: u16 = 1;

/// Return the target-management composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    TARGETS_API_VERSION
}

/// Return the stable architectural module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    TARGETS_MODULE_NAME
}

// ============================================================================
// EXISTING TARGET SUBSYSTEMS
// ============================================================================

/// Target-profile configuration and integration.
///
/// Integrates target-specific settings with the existing generic
/// configuration profile and inheritance system.
///
/// Does not register targets or determine feasibility.
pub mod profiles;

/// Multi-target build coordination.
///
/// Preserves logical build identity, source identity,
/// dependency identity, target-specific assessments,
/// and assessment provenance.
///
/// Does not calculate feasibility or allocate resources.
pub mod matrix;

/// Backend compatibility integration.
///
/// Delegates authoritative compatibility evaluation
/// to the existing backend compatibility subsystem.
///
/// Does not implement a competing compatibility engine.
pub mod compatibility;

/// Deterministic target selection.
///
/// Orders explicitly supplied and assessed candidates
/// using the configured selection policy.
///
/// Does not discover targets, establish compatibility,
/// or allocate hardware.
pub mod selection;

/// Backend target validation and readiness integration.
///
/// Coordinates backend validation, artifact requirements,
/// compatibility reports, and emitter-registration evidence.
///
/// Does not generate artifacts or claim successful execution.
pub mod validation;

// ============================================================================
// TARGET PROFILE PUBLIC API
// ============================================================================

pub use self::profiles::{
    target_profile_section_key,
    TargetProfileSettings,
    TARGET_PROFILE_SCHEMA_VERSION,
    TARGET_PROFILE_SECTION_NAME,
    TARGET_PROFILES_API_VERSION,
    TARGET_PROFILES_MODULE_NAME,
};

// ============================================================================
// MULTI-TARGET MATRIX PUBLIC API
// ============================================================================

pub use self::matrix::{
    AssessmentAuthority,
    AssessmentEvidence,
    AssessmentReason,
    AssessmentReference,
    TargetMatrix,
    TargetMatrixAssessment,
    TargetMatrixDisposition,
    TargetMatrixEntry,
    TargetMatrixRequest,
    TargetMatrixStatus,
    TARGET_MATRIX_API_VERSION,
    TARGET_MATRIX_MODULE_NAME,
};

// ============================================================================
// COMPATIBILITY PUBLIC API
// ============================================================================

pub use self::compatibility::{
    evaluate_target_compatibility,
    TargetCompatibilityService,
    TARGET_COMPATIBILITY_API_VERSION,
    TARGET_COMPATIBILITY_MODULE_NAME,
};

// ============================================================================
// TARGET SELECTION PUBLIC API
// ============================================================================

pub use self::selection::{
    select_target,
    CandidateDecision,
    CandidateDisposition,
    SelectionOutcome,
    TargetSelectionCandidate,
    TargetSelectionPolicy,
    TargetSelectionService,
    TARGET_SELECTION_API_VERSION,
    TARGET_SELECTION_MODULE_NAME,
};

// ============================================================================
// TARGET VALIDATION PUBLIC API
// ============================================================================

pub use self::validation::{
    validate_target,
    TargetValidationReport,
    TargetValidationRequest,
    TargetValidationService,
    TargetValidationStatus,
    TARGET_VALIDATION_API_VERSION,
    TARGET_VALIDATION_MODULE_NAME,
};

// ============================================================================
// IDENTITY INTEGRATION
// ============================================================================

/// Compiler-facing target identity.
///
/// This is an alias to the canonical compiler TargetId.
/// It does not create a new target identifier type.
pub use crate::compiler::configuration::target::TargetId as CompilerTargetId;

/// Backend-facing target identity.
///
/// This is an alias to the canonical backend TargetId.
/// It does not create a new target identifier type.
pub use crate::backend::target::TargetId as BackendTargetId;

// ============================================================================
// API CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.toolchain.targets"
        );

        assert_eq!(api_version(), TARGETS_API_VERSION);
        assert!(api_version() > 0);
    }

    #[test]
    fn child_subsystem_api_versions_are_available() {
        assert!(TARGET_PROFILES_API_VERSION > 0);
        assert!(TARGET_MATRIX_API_VERSION > 0);
        assert!(TARGET_COMPATIBILITY_API_VERSION > 0);
        assert!(TARGET_SELECTION_API_VERSION > 0);
        assert!(TARGET_VALIDATION_API_VERSION > 0);
    }

    #[test]
    fn child_subsystem_identities_are_distinct() {
        let identities = [
            TARGET_PROFILES_MODULE_NAME,
            TARGET_MATRIX_MODULE_NAME,
            TARGET_COMPATIBILITY_MODULE_NAME,
            TARGET_SELECTION_MODULE_NAME,
            TARGET_VALIDATION_MODULE_NAME,
        ];

        let unique: std::collections::BTreeSet<_> =
            identities.into_iter().collect();

        assert_eq!(unique.len(), identities.len());
    }

    #[test]
    fn target_profile_has_no_implicit_target_selection() {
        let settings = TargetProfileSettings::new();

        assert!(!settings.has_explicit_target_references());
        assert!(settings.is_empty());
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn target_profile_schema_is_explicit() {
        let settings = TargetProfileSettings::new();

        assert_eq!(
            settings.schema_version(),
            TARGET_PROFILE_SCHEMA_VERSION
        );
    }
}
