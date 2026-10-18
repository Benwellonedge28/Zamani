
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Capability Integration Boundary
//!
//! File: `src/hdl/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the HDL subsystem's public integration boundary for capabilities
//! while preserving the canonical capability semantics owned by
//! `crate::frontend::semantic::capabilities`.
//!
//! This module connects hardware-description requirements and target-facing
//! capability information to Zamani's shared semantic capability model.
//! It does not implement another capability registry, requirement model,
//! evidence system, or compatibility engine.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/ast/node/capabilities/capability.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/frontend/semantic/capabilities/requirements.rs`
//! - `src/frontend/semantic/capabilities/evidence.rs`
//! - `src/frontend/semantic/capabilities/compatibility.rs`
//!
//! The normative specifications define meaning. The canonical frontend AST
//! owns source structure. The shared semantic capability subsystem owns
//! capability identity, requirements, evidence, and compatibility.
//!
//! ## OWNS
//!
//! - The HDL-facing capability API.
//! - The canonical namespace used by the HDL-specific convenience constructor.
//! - Explicit aliases for using shared capability types in HDL APIs.
//! - Tests for this module's integration contract.
//!
//! ## DOES NOT OWN
//!
//! - A second capability identity or property-value implementation.
//! - A competing capability requirement or version model.
//! - Target inventories or authenticated hardware discovery.
//! - Resource accounting, resource allocation, or target selection.
//! - Authorization, trust verification, or security policy.
//! - Physical placement, routing, scheduling, or synthesis.
//! - Source parsing, AST construction, or semantic name resolution.
//! - HDLIR, another intermediate representation, or runtime execution.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is outside
//! this module's implementation scope.
//!
//! ## CAPABILITY IDENTITY
//!
//! Capability identities are open-world and namespace-qualified.
//!
//! The helper `hdl_capability_id` uses `zamani.hdl` as a convenient namespace
//! for HDL-specific capabilities. It does not restrict the HDL subsystem to
//! that namespace: imported, shared, vendor-extension, and future-domain
//! capabilities continue to use the canonical `CapabilityId` directly.
//!
//! This module intentionally defines no closed enumeration of capabilities.
//! Adding a new capability name must not require editing this file.
//!
//! ## REQUIREMENTS, EVIDENCE, AND FEASIBILITY
//!
//! A capability identity names an ability. A capability requirement states
//! what a program needs. Capability evidence records a scoped claim about
//! support. Compatibility evaluates requirements against that evidence.
//!
//! These are distinct concepts:
//!
//! - A declared capability does not prove target support.
//! - Missing evidence does not prove support or non-support.
//! - Unknown or incomplete evidence must not become a success result.
//! - Conflicting support and non-support evidence must remain observable.
//! - Capability support does not establish resource sufficiency.
//! - A target incompatibility does not, by itself, invalidate source syntax.
//! - This module does not authenticate evidence or allocate hardware.
//!
//! Consumers must use the shared semantic compatibility API and the
//! downstream target-feasibility layer rather than reimplementing these rules.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! There is no fixed maximum number of capability names, namespaces,
//! capability requirements, evidence records, target profiles, or hardware
//! resources defined here.
//!
//! Names and properties use the canonical semantic model. Evidence
//! collections and compatibility operations use the existing shared
//! implementation.
//!
//! Actual execution remains finite and bounded by available memory,
//! processing time, and explicitly configured operational budgets. No
//! literal-infinity guarantee is implied.
//!
//! Operational-budget exhaustion must not be represented as proof that a
//! capability requirement is satisfied.
//!
//! ## DETERMINISM
//!
//! Capability identity and ordering follow the canonical semantic model.
//! Evidence storage and compatibility evaluation follow the contracts of
//! `evidence.rs` and `compatibility.rs`.
//!
//! This module introduces no global mutable state, random identifiers,
//! timestamps, or hardware-dependent identity.
//!
//! ## INTEGRATION
//!
//! 1. The existing AST adapter preserves source capability identity and
//!    provenance when constructing semantic requirements.
//! 2. The semantic pipeline uses `CapabilityRequirement` and the canonical
//!    requirement collection.
//! 3. A target description or provider supplies explicitly scoped evidence.
//! 4. The shared evidence and compatibility services evaluate that evidence.
//! 5. HDL target-feasibility analysis consumes those results together with
//!    resource and constraint analysis.
//! 6. Realization reports retain missing, conflicting, unsupported, and
//!    indeterminate results without silently changing source meaning.
//!
//! This module must not make the parser or source-level semantic validator
//! depend on physical target discovery.
//!
//! ## FREEZE CRITERIA
//!
//! - The frontend semantic capability module is exposed and compiles.
//! - The HDL module is correctly registered.
//! - Every public alias resolves to its canonical shared type.
//! - Open-world and namespace behavior passes the tests below.
//! - The shared evidence and compatibility tests pass.
//! - No competing capability model is introduced.
//! - The workspace passes the required formatting, compilation, test, and
//!   Clippy checks on the supported toolchain.

use crate::frontend::semantic::capabilities::CapabilityId;
use crate::frontend::semantic::capabilities::CapabilityModelError;

/// Canonical namespace for capabilities specifically defined by Zamani HDL.
///
/// This is a naming convention, not a list of supported hardware features.
/// Other namespaces remain valid and can be used directly through
/// [`CapabilityId`].
pub const HDL_CAPABILITY_NAMESPACE: &str = "zamani.hdl";

// =============================================================================
// Canonical semantic capability API
// =============================================================================

/// Canonical capability identity shared with semantic analysis.
///
/// This alias introduces no second identity representation.
pub type HdlCapabilityId = CapabilityId;

/// Canonical capability version.
pub type HdlCapabilityVersion =
    crate::frontend::semantic::capabilities::CapabilityVersion;

/// Canonical capability version constraint.
pub type HdlCapabilityVersionConstraint =
    crate::frontend::semantic::capabilities::CapabilityVersionConstraint;

/// Canonical typed capability property value.
pub type HdlCapabilityPropertyValue =
    crate::frontend::semantic::capabilities::CapabilityPropertyValue;

/// Canonical semantic capability requirement.
pub type HdlCapabilityRequirement =
    crate::frontend::semantic::capabilities::CapabilityRequirement;

/// Canonical collection of source-derived capability requirements.
///
/// This retains the source provenance and ordering guarantees of the shared
/// semantic implementation.
pub type HdlCapabilityRequirements =
    crate::frontend::semantic::capabilities::CapabilityRequirements;

/// Canonical evidence record for a scoped capability-support claim.
pub type HdlCapabilityEvidence =
    crate::frontend::semantic::capabilities::CapabilityEvidence;

/// Canonical evidence status.
pub type HdlCapabilityEvidenceStatus =
    crate::frontend::semantic::capabilities::CapabilityEvidenceStatus;

/// Canonical scope identifier for capability evidence.
///
/// A scope can identify an abstract target profile, simulation environment,
/// toolchain configuration, or another explicitly identified environment.
/// It is not necessarily a physical device identifier.
pub type HdlCapabilityProfileId =
    crate::frontend::semantic::capabilities::EvidenceSubject;

/// Canonical evidence origin.
pub type HdlCapabilityEvidenceOrigin =
    crate::frontend::semantic::capabilities::EvidenceOrigin;

/// Canonical evidence collection with deterministic storage and evaluation.
pub type HdlCapabilityEvidenceSet =
    crate::frontend::semantic::capabilities::CapabilityEvidenceSet;

/// Result of evaluating one evidence record against one requirement.
pub type HdlCapabilityEvidenceEvaluation =
    crate::frontend::semantic::capabilities::EvidenceEvaluation;

/// Aggregated result of evaluating requirements against scoped evidence.
pub type HdlCapabilityEvidenceResult =
    crate::frontend::semantic::capabilities::CapabilityEvidenceResult;

/// Canonical compatibility report.
pub type HdlCapabilityCompatibilityReport =
    crate::frontend::semantic::capabilities::CapabilityCompatibilityReport;

/// Canonical compatibility status.
pub type HdlCapabilityCompatibilityStatus =
    crate::frontend::semantic::capabilities::CapabilityCompatibilityStatus;

/// Canonical compatibility summary.
pub type HdlCapabilityCompatibilitySummary =
    crate::frontend::semantic::capabilities::CapabilityCompatibilitySummary;

/// Canonical compatibility record.
pub type HdlCapabilityCompatibilityRecord =
    crate::frontend::semantic::capabilities::CapabilityCompatibilityRecord;

/// Canonical semantic capability validation error.
pub type HdlCapabilityModelError = CapabilityModelError;

/// Canonical evidence validation error.
pub type HdlCapabilityEvidenceError =
    crate::frontend::semantic::capabilities::CapabilityEvidenceError;

/// Canonical compatibility evaluation error.
pub type HdlCapabilityCompatibilityError =
    crate::frontend::semantic::capabilities::CapabilityCompatibilityError;

// =============================================================================
// HDL namespace helpers
// =============================================================================

/// Construct an HDL-namespaced capability identity.
///
/// The supplied name is validated by the canonical semantic model. Its
/// spelling is preserved; this function does not lowercase, normalize,
/// resolve aliases, or check whether a provider implements the capability.
///
/// # Examples
///
/// ```ignore
/// use zamani_compiler::hdl::capabilities::hdl_capability_id;
///
/// let capability = hdl_capability_id("sequential_logic")?;
/// assert_eq!(capability.namespace(), "zamani.hdl");
/// assert_eq!(capability.name(), "sequential_logic");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// To identify a capability owned by another domain or extension, construct
/// a canonical [`CapabilityId`] with that capability's namespace instead.
pub fn hdl_capability_id(
    name: impl Into<String>,
) -> Result<HdlCapabilityId, HdlCapabilityModelError> {
    CapabilityId::new(HDL_CAPABILITY_NAMESPACE, name)
}

/// Return whether a canonical capability identity belongs to the HDL
/// namespace.
///
/// This tests namespace identity only. It does not determine whether the
/// capability is known, implemented, available, compatible, or feasible.
#[must_use]
pub fn is_hdl_capability(capability: &CapabilityId) -> bool {
    capability.namespace() == HDL_CAPABILITY_NAMESPACE
}

// =============================================================================
// Public semantic operations
// =============================================================================

/// Evaluate the canonical semantic requirements against evidence for one
/// explicitly selected scope.
///
/// This is a thin forwarding function: the canonical compatibility service
/// retains ownership of evidence validation, aggregation, provenance, and
/// compatibility semantics.
///
/// Missing or incomplete evidence must remain distinguishable from success.
#[must_use]
pub fn evaluate_hdl_capability_compatibility(
    requirements: &HdlCapabilityRequirements,
    evidence: &HdlCapabilityEvidenceSet,
    subject: &HdlCapabilityProfileId,
) -> Result<HdlCapabilityCompatibilityReport, HdlCapabilityCompatibilityError> {
    crate::frontend::semantic::capabilities::evaluate_compatibility(
        requirements,
        evidence,
        subject,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hdl_helper_uses_the_canonical_hdl_namespace() {
        let capability = hdl_capability_id("sequential_logic")
            .expect("a non-empty capability name should be valid");

        assert_eq!(capability.namespace(), HDL_CAPABILITY_NAMESPACE);
        assert_eq!(capability.name(), "sequential_logic");
        assert!(is_hdl_capability(&capability));
    }

    #[test]
    fn capability_names_remain_open_ended() {
        let capability = hdl_capability_id(
            "extension_defined_feature_for_a_future_hardware_domain",
        )
        .expect("an unknown but structurally valid capability name is allowed");

        assert!(is_hdl_capability(&capability));
        assert_eq!(
            capability.name(),
            "extension_defined_feature_for_a_future_hardware_domain"
        );
    }

    #[test]
    fn non_hdl_namespaces_remain_supported() {
        let capability = CapabilityId::new(
            "future.vendor_extension",
            "specialized_execution",
        )
        .expect("a non-empty extension identity should be valid");

        assert!(!is_hdl_capability(&capability));
        assert_eq!(capability.namespace(), "future.vendor_extension");
    }

    #[test]
    fn capability_identity_is_not_provider_evidence() {
        let capability = hdl_capability_id("future_feature")
            .expect("the capability identity should be valid");

        // Creating an identity establishes only its name. It does not create
        // evidence that any target supports it.
        assert_eq!(capability.namespace(), HDL_CAPABILITY_NAMESPACE);
    }

    #[test]
    fn empty_capability_names_are_rejected_by_the_canonical_model() {
        assert!(hdl_capability_id("").is_err());
        assert!(hdl_capability_id("   ").is_err());
    }

    #[test]
    fn namespace_membership_does_not_depend_on_capability_name() {
        let first = hdl_capability_id("feature_a")
            .expect("the first identity should be valid");
        let second = hdl_capability_id("feature_b")
            .expect("the second identity should be valid");

        assert!(is_hdl_capability(&first));
        assert!(is_hdl_capability(&second));
        assert_ne!(first, second);
    }
}
