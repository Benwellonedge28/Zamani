
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI — Capability Integration Boundary
//!
//! File: `src/ai/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the AI-domain capability integration boundary.
//!
//! This module describes AI-related computational capabilities while
//! reusing Zamani's canonical semantic capability infrastructure.
//!
//! It does not implement another capability registry, target inventory,
//! hardware discovery system, resource allocator, or scheduler.
//!
//! # Architectural ownership
//!
//! Canonical authorities:
//!
//! - `frontend::semantic::capabilities::model`
//!   Owns capability identities, versions, properties and requirements.
//!
//! - `frontend::semantic::capabilities::evidence`
//!   Owns scoped capability evidence and evidence collections.
//!
//! - `frontend::semantic::capabilities::compatibility`
//!   Owns compatibility evaluation.
//!
//! - `distributed::capabilities`
//!   Owns distributed capability discovery and provider registration.
//!
//! - `backend::capabilities`
//!   Owns backend target capability descriptions and matching.
//!
//! - `zuir::values::capability`
//!   Owns universal IR capability references.
//!
//! - `grammar/core/capabilities.g4`
//!   Owns universal capability syntax.
//!
//! - `grammar/ai/ai-capabilities.g4`
//!   Owns AI-specific capability attachment syntax.
//!
//! This module provides AI-specific adapters and metadata.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - The AI capability namespace.
//! - AI capability identity helpers.
//! - AI capability classification.
//! - AI capability metadata descriptions.
//! - AI capability metadata validation.
//! - AI-domain access to canonical capability evidence.
//! - AI-specific capability evaluation helpers.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - Universal capability identity.
//! - Source-language AST nodes.
//! - Semantic name resolution.
//! - Generic capability requirements.
//! - Capability evidence storage.
//! - Capability provider registration.
//! - Hardware discovery.
//! - Device identification.
//! - Backend capability inventories.
//! - Resource allocation.
//! - Target selection.
//! - Scheduling.
//! - Distributed placement.
//! - Quantum routing.
//! - Quantum error correction.
//! - ZUIR operations.
//! - Runtime execution.
//! - Security authorization.
//!
//! # POCO-REAF
//!
//! AI capability identities describe computational abilities rather
//! than particular hardware implementations.
//!
//! A capability must remain meaningful across:
//!
//! - Embedded systems.
//! - Microcontrollers.
//! - CPUs.
//! - Multicore systems.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - AI accelerators.
//! - Quantum processors.
//! - Neuromorphic systems.
//! - Heterogeneous systems.
//! - Distributed systems.
//! - Future computational architectures.
//!
//! No universal maximum capability count, model count, accelerator count,
//! device count, tensor rank, memory capacity or worker count is imposed.
//!
//! Literal infinity is not physically achievable. The objective is to
//! avoid artificial language-level ceilings and preserve semantic intent
//! subject to actual implementation resources and execution feasibility.
//!
//! # Dependency direction
//!
//! ```text
//! AI grammar
//!     |
//!     v
//! Canonical frontend AST
//!     |
//!     v
//! Semantic capability model
//!     |
//!     v
//! THIS MODULE
//!     |
//!     +---- AI capability metadata
//!     |
//!     +---- Canonical requirements
//!     |
//!     +---- Canonical evidence
//!     |
//!     v
//! Distributed discovery / backend matching
//!     |
//!     v
//! Resource feasibility
//!     |
//!     v
//! ZUIR / domain IR / execution planning
//! ```
//!
//! This module does not reverse the dependency direction.

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::fmt;

use crate::frontend::semantic::capabilities::model::{
    CapabilityId,
    CapabilityModelError,
    CapabilityPropertyValue,
    CapabilityRequirement,
    CapabilityVersion,
    CapabilityVersionConstraint,
};

use crate::frontend::semantic::capabilities::evidence::{
    CapabilityEvidence,
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    CapabilityEvidenceStatus,
    EvidenceEvaluation,
    EvidenceOrigin,
    EvidenceSubject,
};

// =============================================================================
// MODULE IDENTITY
// =============================================================================

/// Stable identity of the AI capability integration module.
pub const AI_CAPABILITIES_MODULE_NAME: &str =
    "zamani.ai.capabilities";

/// Version of this module's metadata contract.
///
/// This is not the language version, semantic-model version,
/// or serialized model-artifact version.
pub const AI_CAPABILITIES_SCHEMA_VERSION: u16 = 1;

/// Canonical namespace used for AI-owned capabilities.
///
/// Example:
///
/// `ai::inference`
///
/// The namespace is open-world. New capability names do not require
/// modifying this file.
pub const AI_CAPABILITY_NAMESPACE: &str = "ai";

/// Namespace used for AI capability classification metadata.
const AI_CATEGORY_NAMESPACE: &str = "ai.category";

// =============================================================================
// CANONICAL SEMANTIC RE-EXPORTS
// =============================================================================

/// Canonical semantic capability identity.
///
/// This is a re-export, not a new AI capability identity type.
pub type AiCapabilityId = CapabilityId;

/// Canonical semantic capability requirement.
///
/// This represents a mandatory capability requirement.
/// Preferences, constraints and hints must retain their respective
/// source-level roles during semantic analysis.
pub type AiCapabilityRequirement = CapabilityRequirement;

/// Canonical semantic capability version.
pub type AiCapabilityVersion = CapabilityVersion;

/// Canonical semantic capability version constraint.
pub type AiCapabilityVersionConstraint = CapabilityVersionConstraint;

/// Canonical typed capability property.
pub type AiCapabilityPropertyValue = CapabilityPropertyValue;

/// Canonical capability evidence.
pub type AiCapabilityEvidence = CapabilityEvidence;

/// Canonical capability evidence collection.
pub type AiCapabilityEvidenceSet = CapabilityEvidenceSet;

/// Canonical evidence subject.
pub type AiCapabilityEvidenceSubject = EvidenceSubject;

/// Canonical evidence origin.
pub type AiCapabilityEvidenceOrigin = EvidenceOrigin;

/// Canonical evidence status.
pub type AiCapabilityEvidenceStatus = CapabilityEvidenceStatus;

/// Canonical evidence evaluation.
pub type AiCapabilityEvidenceEvaluation = EvidenceEvaluation;

/// Canonical aggregated evidence result.
pub type AiCapabilityEvidenceResult = CapabilityEvidenceResult;

// =============================================================================
// AI CAPABILITY IDENTITY
// =============================================================================

/// Construct an AI-namespaced capability identity.
///
/// The capability name is validated by the canonical semantic model.
///
/// Names are preserved exactly. No automatic lowercasing, normalization,
/// alias resolution or capability discovery takes place.
///
/// # Examples
///
/// ```
/// use zamani_compiler::ai::capabilities::{
///     ai_capability_id,
///     is_ai_capability,
/// };
///
/// let capability = ai_capability_id("inference")
///     .expect("valid capability");
///
/// assert_eq!(capability.namespace(), "ai");
/// assert_eq!(capability.name(), "inference");
/// assert!(is_ai_capability(&capability));
/// ```
///
/// # Errors
///
/// Returns the canonical semantic-model validation error.
pub fn ai_capability_id(
    name: impl Into<String>,
) -> Result<AiCapabilityId, CapabilityModelError> {
    CapabilityId::new(AI_CAPABILITY_NAMESPACE, name)
}

/// Return whether a canonical capability identity belongs to
/// the AI namespace.
///
/// This checks identity only.
///
/// It does not establish that:
///
/// - The capability is registered.
/// - An implementation exists.
/// - A target supports it.
/// - Resources are available.
/// - Execution is authorized.
///
/// Capabilities belonging to other domains may still be required
/// by an AI computation.
#[must_use]
pub fn is_ai_capability(
    capability: &CapabilityId,
) -> bool {
    capability.namespace() == AI_CAPABILITY_NAMESPACE
}

/// Construct an AI capability requirement.
///
/// The resulting requirement uses the canonical semantic representation.
///
/// # Errors
///
/// Returns an error when the capability identity is invalid.
pub fn ai_capability_requirement(
    name: impl Into<String>,
) -> Result<AiCapabilityRequirement, CapabilityModelError> {
    Ok(CapabilityRequirement::new(
        ai_capability_id(name)?,
    ))
}

/// Construct an AI capability requirement with an explicit
/// version constraint.
pub fn ai_capability_requirement_with_version(
    name: impl Into<String>,
    version: CapabilityVersionConstraint,
) -> Result<AiCapabilityRequirement, CapabilityModelError> {
    Ok(CapabilityRequirement::with_version(
        ai_capability_id(name)?,
        version,
    ))
}

// =============================================================================
// OPEN-WORLD CAPABILITY CLASSIFICATION
// =============================================================================

/// An extensible AI capability category.
///
/// Categories are strings rather than a closed Rust enumeration.
///
/// This permits future computational paradigms to introduce categories
/// without modifying the universal compiler.
///
/// Examples:
///
/// - neural
/// - symbolic
/// - probabilistic
/// - differentiable
/// - reasoning
/// - learning
/// - inference
/// - training
/// - distributed
/// - quantum_hybrid
/// - neuromorphic
/// - future_computing
///
/// A category is descriptive metadata, not a hardware capability.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct AiCapabilityCategory(String);

impl AiCapabilityCategory {
    /// Construct an AI capability category.
    ///
    /// Validation uses the canonical capability-name rules.
    ///
    /// No predefined category enumeration is required.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, CapabilityModelError> {
        let value = value.into();

        // Reuse canonical identifier validation rather than
        // introducing another identifier grammar.
        CapabilityId::new(
            AI_CATEGORY_NAMESPACE,
            value.clone(),
        )?;

        Ok(Self(value))
    }

    /// Return the exact category name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Revalidate values obtained through deserialization.
    pub fn validate(&self) -> Result<(), CapabilityModelError> {
        CapabilityId::new(
            AI_CATEGORY_NAMESPACE,
            self.0.clone(),
        )?;

        Ok(())
    }
}

impl fmt::Display for AiCapabilityCategory {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// AI CAPABILITY DESCRIPTORS
// =============================================================================

/// Descriptive metadata for an AI computational capability.
///
/// This structure describes an AI-domain capability definition.
///
/// It does not assert that a particular target supports the capability.
///
/// Support claims belong to canonical capability evidence.
///
/// Metadata is extensible through typed properties. Property schemas
/// and their domain-specific interpretation remain the responsibility
/// of the relevant semantic extension.
///
/// # Examples
///
/// An AI extension may describe:
///
/// - A neural computation capability.
/// - A symbolic reasoning capability.
/// - A probabilistic inference capability.
/// - A distributed training capability.
/// - A future AI computational capability.
///
/// The capability identity remains canonical and open-world.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AiCapabilityDescriptor {
    capability: CapabilityId,
    category: AiCapabilityCategory,
    description: String,
    attributes: BTreeMap<String, CapabilityPropertyValue>,
}

impl AiCapabilityDescriptor {
    /// Construct an AI capability descriptor.
    ///
    /// An AI descriptor may refer to an AI-owned capability or to an
    /// external-domain capability used by AI computation.
    ///
    /// For example, an AI workload may depend on:
    ///
    /// - An AI inference capability.
    /// - A distributed collective capability.
    /// - A quantum computation capability.
    /// - A tensor computation capability.
    ///
    /// No target support is implied by constructing this descriptor.
    #[must_use]
    pub fn new(
        capability: CapabilityId,
        category: AiCapabilityCategory,
        description: impl Into<String>,
    ) -> Self {
        Self {
            capability,
            category,
            description: description.into(),
            attributes: BTreeMap::new(),
        }
    }

    /// Construct a descriptor for an AI-owned capability.
    ///
    /// # Errors
    ///
    /// Returns an error if the capability name or category is invalid.
    pub fn for_ai(
        name: impl Into<String>,
        category: AiCapabilityCategory,
        description: impl Into<String>,
    ) -> Result<Self, CapabilityModelError> {
        Ok(Self::new(
            ai_capability_id(name)?,
            category,
            description,
        ))
    }

    /// Return the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Return the category.
    #[must_use]
    pub fn category(&self) -> &AiCapabilityCategory {
        &self.category
    }

    /// Return the human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return typed extension attributes in deterministic key order.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<String, CapabilityPropertyValue> {
        &self.attributes
    }

    /// Insert an attribute without silently overwriting an existing value.
    ///
    /// Attribute names and values are validated through the canonical
    /// semantic capability requirement model.
    ///
    /// # Errors
    ///
    /// Returns a canonical validation error if the key or value is invalid.
    pub fn add_attribute(
        &mut self,
        name: impl Into<String>,
        value: CapabilityPropertyValue,
    ) -> Result<(), CapabilityModelError> {
        let name = name.into();

        // The canonical requirement model validates both the property
        // identifier and its typed value.
        let mut requirement =
            CapabilityRequirement::new(self.capability.clone());

        requirement.add_property(
            name.clone(),
            value.clone(),
        )?;

        // Duplicate insertion is rejected explicitly.
        if self.attributes.contains_key(&name) {
            return Err(CapabilityModelError::DuplicateProperty {
                name,
            });
        }

        self.attributes.insert(name, value);

        Ok(())
    }

    /// Explicitly replace an existing attribute or insert a new one.
    ///
    /// Replacement is intentional and visible to the caller.
    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: CapabilityPropertyValue,
    ) -> Result<Option<CapabilityPropertyValue>, CapabilityModelError> {
        let name = name.into();

        let mut requirement =
            CapabilityRequirement::new(self.capability.clone());

        requirement.set_property(
            name.clone(),
            value.clone(),
        )?;

        Ok(self.attributes.insert(name, value))
    }

    /// Validate all descriptor invariants.
    ///
    /// This must also be called after deserializing untrusted metadata.
    pub fn validate(&self) -> Result<(), CapabilityModelError> {
        self.category.validate()?;

        for (name, value) in &self.attributes {
            let mut requirement =
                CapabilityRequirement::new(self.capability.clone());

            requirement.add_property(
                name.clone(),
                value.clone(),
            )?;
        }

        Ok(())
    }

    /// Return whether this descriptor describes an AI-owned capability.
    #[must_use]
    pub fn is_ai_owned(&self) -> bool {
        is_ai_capability(&self.capability)
    }
}

// =============================================================================
// CANONICAL EVIDENCE INTEGRATION
// =============================================================================

/// Evaluate one AI-related requirement against canonical evidence.
///
/// This function delegates directly to the canonical semantic evidence
/// collection.
///
/// It does not implement a competing evidence evaluation algorithm.
///
/// Results distinguish:
///
/// - No evidence.
/// - Satisfied.
/// - Not satisfied.
/// - Indeterminate.
/// - Conflicting evidence.
///
/// Missing or incomplete evidence must never be treated as success.
#[must_use]
pub fn evaluate_ai_capability(
    evidence: &CapabilityEvidenceSet,
    subject: &EvidenceSubject,
    requirement: &CapabilityRequirement,
) -> CapabilityEvidenceResult {
    evidence.evaluate(
        subject,
        requirement,
    )
}

/// Return AI-owned evidence records for an explicitly supplied scope.
///
/// Evidence for other domains is not discarded from the original
/// collection. This function merely provides a filtered iterator.
pub fn ai_capability_evidence<'a>(
    evidence: &'a CapabilityEvidenceSet,
    subject: &'a EvidenceSubject,
) -> impl Iterator<Item = &'a CapabilityEvidence> + 'a {
    evidence.iter().filter(move |record| {
        record.subject() == subject
            && is_ai_capability(record.capability())
    })
}

/// Return evidence records for a particular AI capability.
///
/// The caller supplies a canonical capability identity and scope.
pub fn evidence_for_ai_capability<'a>(
    evidence: &'a CapabilityEvidenceSet,
    subject: &'a EvidenceSubject,
    capability: &'a CapabilityId,
) -> impl Iterator<Item = &'a CapabilityEvidence> + 'a {
    evidence.for_capability(
        subject,
        capability,
    )
    .filter(|record| is_ai_capability(record.capability()))
}

// =============================================================================
// AI CAPABILITY CONTRACT VALIDATION
// =============================================================================

/// Validate an AI capability descriptor and its associated requirement.
///
/// The descriptor and requirement must identify the same capability.
///
/// This prevents metadata from accidentally being attached to an
/// unrelated semantic requirement.
pub fn validate_ai_capability_contract(
    descriptor: &AiCapabilityDescriptor,
    requirement: &CapabilityRequirement,
) -> Result<(), AiCapabilityContractError> {
    descriptor
        .validate()
        .map_err(AiCapabilityContractError::InvalidDescriptor)?;

    if descriptor.capability() != requirement.capability() {
        return Err(AiCapabilityContractError::CapabilityMismatch {
            descriptor: descriptor.capability().qualified_name(),
            requirement: requirement.capability().qualified_name(),
        });
    }

    Ok(())
}

/// Errors from AI-specific capability contract validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AiCapabilityContractError {
    /// Descriptor metadata failed canonical validation.
    InvalidDescriptor(CapabilityModelError),

    /// The descriptor and requirement refer to different capabilities.
    CapabilityMismatch {
        /// Descriptor capability identity.
        descriptor: String,

        /// Requirement capability identity.
        requirement: String,
    },
}

impl fmt::Display for AiCapabilityContractError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDescriptor(error) => {
                write!(
                    formatter,
                    "invalid AI capability descriptor: {error}"
                )
            }

            Self::CapabilityMismatch {
                descriptor,
                requirement,
            } => {
                write!(
                    formatter,
                    "AI capability descriptor `{descriptor}` does not \
                     match requirement `{requirement}`"
                )
            }
        }
    }
}

impl std::error::Error for AiCapabilityContractError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidDescriptor(error) => Some(error),
            Self::CapabilityMismatch { .. } => None,
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn category(
        value: &str,
    ) -> AiCapabilityCategory {
        AiCapabilityCategory::new(value)
            .expect("valid category")
    }

    fn subject() -> EvidenceSubject {
        EvidenceSubject::new("test.ai.target")
            .expect("valid evidence subject")
    }

    #[test]
    fn ai_namespace_is_canonical() {
        let id = ai_capability_id("inference")
            .expect("valid capability");

        assert_eq!(id.namespace(), "ai");
        assert_eq!(id.name(), "inference");
        assert!(is_ai_capability(&id));
    }

    #[test]
    fn capability_names_remain_open_world() {
        let id = ai_capability_id(
            "future_computational_capability_v2",
        )
        .expect("future capability is valid");

        assert!(is_ai_capability(&id));
    }

    #[test]
    fn external_domain_capabilities_are_supported() {
        let id = CapabilityId::new(
            "accelerator",
            "tensor_compute",
        )
        .expect("valid external capability");

        assert!(!is_ai_capability(&id));

        let descriptor = AiCapabilityDescriptor::new(
            id.clone(),
            category("accelerated_computation"),
            "Accelerated tensor computation",
        );

        assert_eq!(descriptor.capability(), &id);
        assert!(!descriptor.is_ai_owned());
    }

    #[test]
    fn capability_categories_are_open_world() {
        let first = category("neural");
        let second = category("quantum_hybrid");
        let future = category("future_ai_paradigm");

        assert_eq!(first.as_str(), "neural");
        assert_eq!(second.as_str(), "quantum_hybrid");
        assert_eq!(future.as_str(), "future_ai_paradigm");
    }

    #[test]
    fn empty_categories_are_rejected() {
        assert!(AiCapabilityCategory::new("").is_err());
        assert!(AiCapabilityCategory::new("   ").is_err());
    }

    #[test]
    fn descriptor_preserves_canonical_identity() {
        let descriptor = AiCapabilityDescriptor::for_ai(
            "training",
            category("learning"),
            "AI training computation",
        )
        .expect("valid descriptor");

        assert!(descriptor.is_ai_owned());
        assert_eq!(descriptor.capability().name(), "training");
        assert_eq!(descriptor.category().as_str(), "learning");
    }

    #[test]
    fn descriptor_accepts_typed_attributes() {
        let mut descriptor = AiCapabilityDescriptor::for_ai(
            "inference",
            category("neural"),
            "Neural inference",
        )
        .expect("valid descriptor");

        descriptor
            .add_attribute(
                "deterministic",
                CapabilityPropertyValue::Boolean(true),
            )
            .expect("valid property");

        assert_eq!(
            descriptor.attributes().get("deterministic"),
            Some(&CapabilityPropertyValue::Boolean(true))
        );

        descriptor.validate()
            .expect("descriptor must remain valid");
    }

    #[test]
    fn duplicate_attributes_are_rejected() {
        let mut descriptor = AiCapabilityDescriptor::for_ai(
            "inference",
            category("neural"),
            "Neural inference",
        )
        .expect("valid descriptor");

        descriptor
            .add_attribute(
                "deterministic",
                CapabilityPropertyValue::Boolean(true),
            )
            .expect("first insertion");

        assert!(
            descriptor.add_attribute(
                "deterministic",
                CapabilityPropertyValue::Boolean(false),
            ).is_err()
        );
    }

    #[test]
    fn explicit_attribute_replacement_is_supported() {
        let mut descriptor = AiCapabilityDescriptor::for_ai(
            "inference",
            category("neural"),
            "Neural inference",
        )
        .expect("valid descriptor");

        descriptor
            .add_attribute(
                "deterministic",
                CapabilityPropertyValue::Boolean(true),
            )
            .expect("first insertion");

        let previous = descriptor
            .set_attribute(
                "deterministic",
                CapabilityPropertyValue::Boolean(false),
            )
            .expect("replacement");

        assert_eq!(
            previous,
            Some(CapabilityPropertyValue::Boolean(true))
        );
    }

    #[test]
    fn invalid_attribute_names_are_rejected() {
        let mut descriptor = AiCapabilityDescriptor::for_ai(
            "inference",
            category("neural"),
            "Neural inference",
        )
        .expect("valid descriptor");

        assert!(
            descriptor.add_attribute(
                "",
                CapabilityPropertyValue::Boolean(true),
            ).is_err()
        );
    }

    #[test]
    fn canonical_requirement_construction_works() {
        let requirement =
            ai_capability_requirement("inference")
                .expect("valid requirement");

        assert!(is_ai_capability(requirement.capability()));
    }

    #[test]
    fn versioned_requirements_use_canonical_version_types() {
        let version = CapabilityVersionConstraint::between(
            CapabilityVersion::new(1, 0, 0),
            CapabilityVersion::new(2, 0, 0),
        )
        .expect("valid version interval");

        let requirement =
            ai_capability_requirement_with_version(
                "inference",
                version,
            )
            .expect("valid requirement");

        assert!(
            requirement.accepts_version(
                CapabilityVersion::new(1, 5, 0)
            )
        );
    }

    #[test]
    fn canonical_evidence_satisfies_ai_requirement() {
        let id = ai_capability_id("inference")
            .expect("valid capability");

        let requirement =
            CapabilityRequirement::new(id.clone());

        let subject = subject();

        let origin = EvidenceOrigin::new(
            "verification",
            "test_provider",
            Some("test_report".to_owned()),
        )
        .expect("valid origin");

        let record = CapabilityEvidence::new(
            id,
            subject.clone(),
            CapabilityEvidenceStatus::Supported,
            None,
            BTreeMap::new(),
            origin,
        )
        .expect("valid evidence");

        let mut evidence = CapabilityEvidenceSet::new();

        evidence.insert(record)
            .expect("evidence insertion");

        assert_eq!(
            evaluate_ai_capability(
                &evidence,
                &subject,
                &requirement,
            ),
            CapabilityEvidenceResult::Satisfied
        );
    }

    #[test]
    fn missing_evidence_is_not_success() {
        let requirement =
            ai_capability_requirement("inference")
                .expect("valid requirement");

        let evidence = CapabilityEvidenceSet::new();

        assert_eq!(
            evaluate_ai_capability(
                &evidence,
                &subject(),
                &requirement,
            ),
            CapabilityEvidenceResult::NoEvidence
        );
    }

    #[test]
    fn unsupported_evidence_is_not_success() {
        let id = ai_capability_id("inference")
            .expect("valid capability");

        let requirement =
            CapabilityRequirement::new(id.clone());

        let subject = subject();

        let origin = EvidenceOrigin::new(
            "verification",
            "test_provider",
            None,
        )
        .expect("valid origin");

        let record = CapabilityEvidence::new(
            id,
            subject.clone(),
            CapabilityEvidenceStatus::Unsupported,
            None,
            BTreeMap::new(),
            origin,
        )
        .expect("valid evidence");

        let mut evidence = CapabilityEvidenceSet::new();

        evidence.insert(record)
            .expect("evidence insertion");

        assert_eq!(
            evaluate_ai_capability(
                &evidence,
                &subject,
                &requirement,
            ),
            CapabilityEvidenceResult::NotSatisfied
        );
    }

    #[test]
    fn evidence_filter_preserves_other_domains() {
        let ai_id = ai_capability_id("inference")
            .expect("valid AI capability");

        let external_id = CapabilityId::new(
            "quantum",
            "computation",
        )
        .expect("valid quantum capability");

        let subject = subject();

        let origin = EvidenceOrigin::new(
            "discovery",
            "test_provider",
            None,
        )
        .expect("valid origin");

        let ai_record = CapabilityEvidence::new(
            ai_id,
            subject.clone(),
            CapabilityEvidenceStatus::Supported,
            None,
            BTreeMap::new(),
            origin.clone(),
        )
        .expect("valid AI evidence");

        let external_record = CapabilityEvidence::new(
            external_id,
            subject.clone(),
            CapabilityEvidenceStatus::Supported,
            None,
            BTreeMap::new(),
            origin,
        )
        .expect("valid external evidence");

        let mut evidence = CapabilityEvidenceSet::new();

        evidence.insert(ai_record)
            .expect("AI evidence");

        evidence.insert(external_record)
            .expect("external evidence");

        let filtered: Vec<_> = ai_capability_evidence(
            &evidence,
            &subject,
        )
        .collect();

        assert_eq!(filtered.len(), 1);
        assert!(is_ai_capability(
            filtered[0].capability()
        ));

        // Filtering does not mutate the canonical evidence collection.
        assert_eq!(evidence.len(), 2);
    }

    #[test]
    fn descriptor_requirement_mismatch_is_rejected() {
        let descriptor = AiCapabilityDescriptor::for_ai(
            "inference",
            category("neural"),
            "Neural inference",
        )
        .expect("valid descriptor");

        let requirement =
            ai_capability_requirement("training")
                .expect("valid requirement");

        assert!(
            validate_ai_capability_contract(
                &descriptor,
                &requirement,
            ).is_err()
        );
    }

    #[test]
    fn matching_descriptor_and_requirement_are_accepted() {
        let descriptor = AiCapabilityDescriptor::for_ai(
            "inference",
            category("neural"),
            "Neural inference",
        )
        .expect("valid descriptor");

        let requirement =
            ai_capability_requirement("inference")
                .expect("valid requirement");

        assert!(
            validate_ai_capability_contract(
                &descriptor,
                &requirement,
            ).is_ok()
        );
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(AI_CAPABILITIES_SCHEMA_VERSION, 1);
    }
}
