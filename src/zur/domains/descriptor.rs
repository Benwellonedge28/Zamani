
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Descriptors
//!
//! Path: `src/zur/domains/descriptor.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## PURPOSE
//!
//! Defines immutable, validated, extensible descriptors for computational
//! domains that participate in ZUR execution-provider integration.
//!
//! A descriptor describes the runtime-facing execution contracts associated
//! with a computational domain.
//!
//! It supports classical, quantum, HDL, AI, tensor, hybrid, distributed,
//! accelerator, photonic, and future computational domains without
//! maintaining a closed inventory of domains.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Domain descriptor metadata.
//! - Runtime-facing domain contract declarations.
//! - Supported provider-interface references.
//! - External capability-evidence references.
//! - External resource-requirement references.
//! - Execution-model references.
//! - Interoperability declarations.
//! - Descriptor construction and structural validation.
//! - Duplicate declaration detection.
//!
//! This module does NOT own:
//!
//! - Canonical domain identity.
//! - Source-language domain semantics.
//! - Capability semantics.
//! - Capability evidence generation or authentication.
//! - Resource semantics.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Provider identity.
//! - Provider registration.
//! - Provider discovery.
//! - Provider authentication.
//! - Backend registration.
//! - Compiler planning.
//! - Feasibility evaluation.
//! - Hardware selection.
//! - Scheduling.
//! - Routing.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI or tensor IR.
//! - ZUIR.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//! - Physical device interaction.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! ## ARCHITECTURAL INVARIANTS
//!
//! Domain identity != provider identity.
//! Domain identity != provider interface.
//! Provider interface != capability.
//! Capability != capability evidence.
//! Capability evidence != capability availability.
//! Resource requirement != resource capacity.
//! Resource capacity != resource availability.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Execution submission != successful completion.
//!
//! A descriptor is descriptive metadata.
//!
//! Registration does not establish provider trust or availability.
//!
//! ## OPEN-WORLD EXTENSIBILITY
//!
//! Computational domains are represented by the canonical AST Domain type.
//!
//! No closed enum of computational domains is introduced.
//!
//! Execution-model identifiers are extensible qualified identifiers.
//!
//! New domains and execution models can be introduced without modifying
//! this module.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Domains.
//! - Provider interfaces.
//! - Capability evidence references.
//! - Resource requirement references.
//! - Execution models.
//! - Interoperability declarations.
//! - Identifier length.
//! - Namespace depth.
//!
//! Collections grow according to actual allocation capacity and available
//! system resources.
//!
//! No finite machine promises literal infinite storage or computation.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No global mutable state.
//! No filesystem access.
//! No network access.
//! No hardware discovery.
//! No implicit resource allocation.
//! No fabricated execution success.
//!
//! ## DETERMINISM
//!
//! Ordered collections are used for descriptor declarations.
//!
//! Equivalent descriptor contents produce deterministic iteration order.
//!
//! Descriptor construction does not depend on registration order,
//! physical device state, process identity, timestamps, or random state.
//!
//! ## GRAMMAR AUTHORITY
//!
//! Relevant specifications:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/spec/resources.md
//! - grammar/spec/interoperability.md
//!
//! Relevant grammar:
//!
//! - grammar/declarations/capabilities.g4
//! - grammar/resources/capabilities.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/hybrid/accelerator-interoperability.g4
//!
//! Grammar describes source-level intent.
//!
//! Semantic analysis validates source meaning.
//!
//! This descriptor receives already-established identities and references.
//!
//! It never parses source grammar or interprets source declarations.
//!
//! ## INTEGRATION
//!
//! Canonical domain identity:
//!     frontend::ast::node::domains::domain::Domain
//!
//! Provider interface:
//!     zur::contracts::provider::ProviderInterfaceId
//!
//! Capability evidence:
//!     zur::contracts::provider::CapabilityEvidenceRef
//!
//! Resource requirements:
//!     zur::contracts::provider::ResourceRequirementRef
//!
//! Domain contract version:
//!     zur::version::ServiceContractVersion
//!
//! Operational errors:
//!     zur::error::ZurError
//!
//! The domain registry consumes DomainDescriptor.
//!
//! The domain provider adapter consumes its declared interface references.
//!
//! Validation and compatibility subsystems resolve external references
//! through their respective canonical authorities.
//!
//! No physical execution is performed by this module.

use core::fmt;

use std::collections::BTreeSet;

use crate::frontend::ast::node::domains::domain::Domain;

use crate::zur::contracts::provider::{
    CapabilityEvidenceRef,
    ProviderInterfaceId,
    ResourceRequirementRef,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::version::ServiceContractVersion;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates an open-world qualified identifier.
///
/// Identifier segments are separated by dots.
///
/// Supported characters:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
///
/// Identifiers are case-sensitive.
///
/// No arbitrary maximum identifier length or namespace depth is imposed.
fn validate_qualified_identifier(
    value: &str,
    field: &'static str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(
            ZurError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_component("zur.domains.descriptor"),
        );
    }

    if value.trim() != value {
        return Err(
            ZurError::invalid_input(
                format!(
                    "{field} must not contain surrounding whitespace"
                ),
            )
            .with_component("zur.domains.descriptor"),
        );
    }

    let valid = value.split('.').all(|segment| {
        !segment.is_empty()
            && segment.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
            })
    });

    if !valid {
        return Err(
            ZurError::invalid_input(
                format!(
                    "{field} must contain non-empty \
                     dot-separated identifier segments"
                ),
            )
            .with_component("zur.domains.descriptor"),
        );
    }

    Ok(())
}

/// Validates human-readable descriptive text.
///
/// Unicode text is supported.
/// Control characters and empty descriptions are rejected.
///
/// No arbitrary description-length ceiling is imposed.
fn validate_description(
    value: &str,
    field: &'static str,
) -> ZurResult<()> {
    if value.trim().is_empty() {
        return Err(
            ZurError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_component("zur.domains.descriptor"),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ZurError::invalid_input(
                format!(
                    "{field} must not contain control characters"
                ),
            )
            .with_component("zur.domains.descriptor"),
        );
    }

    Ok(())
}

// ============================================================================
// EXECUTION MODEL IDENTITY
// ============================================================================

/// Stable, open-world identity of an execution model.
///
/// An execution model describes a declared execution contract associated
/// with a computational domain.
///
/// It does not identify a physical machine, device, backend, or vendor.
///
/// Examples:
///
/// - `zamani.classical.native`
/// - `zamani.classical.interpreted`
/// - `zamani.quantum.circuit`
/// - `zamani.hdl.hardware-realization`
/// - `zamani.ai.inference`
/// - `zamani.distributed.execution`
/// - `future.photonics.execution`
///
/// These examples do not constitute a closed inventory.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ExecutionModelId(String);

impl ExecutionModelId {
    /// Constructs a validated execution-model identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_qualified_identifier(
            &value,
            "execution-model identity",
        )?;

        Ok(Self(value))
    }

    /// Returns the complete execution-model identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identity and returns its underlying string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ExecutionModelId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// EXECUTION MODEL REFERENCE
// ============================================================================

/// Reference to an execution model owned by an external authority.
///
/// The referenced execution contract remains owned by its appropriate
/// compiler, domain, interoperability, or execution subsystem.
///
/// This reference does not implement or execute that model.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ExecutionModelRef {
    authority: String,
    model: ExecutionModelId,
}

impl ExecutionModelRef {
    /// Creates a reference to an externally owned execution model.
    pub fn new(
        authority: impl Into<String>,
        model: ExecutionModelId,
    ) -> ZurResult<Self> {
        let authority = authority.into();

        validate_qualified_identifier(
            &authority,
            "execution-model authority",
        )?;

        Ok(Self {
            authority,
            model,
        })
    }

    /// Returns the owning authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the execution-model identity.
    #[must_use]
    pub fn model(&self) -> &ExecutionModelId {
        &self.model
    }
}

// ============================================================================
// INTEROPERABILITY CONTRACT REFERENCE
// ============================================================================

/// Stable reference to an interoperability contract owned elsewhere.
///
/// The referenced contract may describe a typed boundary between
/// computational domains.
///
/// It does not define the foreign ABI, data layout, conversion algorithm,
/// transport, or execution implementation.
///
/// Those remain owned by their canonical subsystems.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct InteroperabilityContractRef {
    authority: String,
    contract: String,
}

impl InteroperabilityContractRef {
    /// Creates an externally owned interoperability-contract reference.
    pub fn new(
        authority: impl Into<String>,
        contract: impl Into<String>,
    ) -> ZurResult<Self> {
        let authority = authority.into();
        let contract = contract.into();

        validate_qualified_identifier(
            &authority,
            "interoperability authority",
        )?;

        validate_qualified_identifier(
            &contract,
            "interoperability contract identity",
        )?;

        Ok(Self {
            authority,
            contract,
        })
    }

    /// Returns the owning interoperability authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the referenced contract identity.
    #[must_use]
    pub fn contract(&self) -> &str {
        &self.contract
    }
}

// ============================================================================
// INTEROPERABILITY DECLARATION
// ============================================================================

/// Declares interoperability with another computational domain.
///
/// The other domain is represented by the canonical AST Domain type.
///
/// The contract reference identifies the externally owned semantic
/// interoperability contract.
///
/// This declaration does not prove that the other domain is registered,
/// available, compatible, or executable.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct InteroperabilityDeclaration {
    domain: Domain,
    contract: InteroperabilityContractRef,
}

impl InteroperabilityDeclaration {
    /// Constructs a domain interoperability declaration.
    ///
    /// Self-interoperability is rejected.
    pub fn new(
        owner: &Domain,
        domain: Domain,
        contract: InteroperabilityContractRef,
    ) -> ZurResult<Self> {
        if !domain.validate().is_ok() {
            return Err(
                ZurError::invalid_input(
                    "interoperability domain identity is invalid",
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        if owner == &domain {
            return Err(
                ZurError::invalid_input(
                    "a domain cannot declare interoperability \
                     with itself through an external domain boundary",
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        Ok(Self {
            domain,
            contract,
        })
    }

    /// Returns the interoperating domain.
    #[must_use]
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Returns the external interoperability contract reference.
    #[must_use]
    pub fn contract(&self) -> &InteroperabilityContractRef {
        &self.contract
    }
}

// ============================================================================
// DOMAIN DESCRIPTOR
// ============================================================================

/// Immutable, validated runtime-facing descriptor for a computational
/// domain.
///
/// The descriptor references existing semantic and provider authorities
/// rather than creating competing domain, capability, or resource models.
///
/// All collections use deterministic ordering.
///
/// A descriptor is constructed through [`DomainDescriptor::new`] and
/// extended using its insertion methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainDescriptor {
    domain: Domain,

    contract_version: ServiceContractVersion,

    purpose: String,

    interfaces: BTreeSet<ProviderInterfaceId>,

    capability_evidence: BTreeSet<CapabilityEvidenceRef>,

    resource_requirements: BTreeSet<ResourceRequirementRef>,

    execution_models: BTreeSet<ExecutionModelRef>,

    interoperability: BTreeSet<InteroperabilityDeclaration>,
}

impl DomainDescriptor {
    /// Creates a domain descriptor.
    ///
    /// The canonical AST domain identity and its contract version are
    /// supplied by the caller.
    ///
    /// The purpose must be non-empty and contain no control characters.
    ///
    /// A descriptor must declare at least one supported provider
    /// interface before it is accepted for registration.
    pub fn new(
        domain: Domain,
        contract_version: ServiceContractVersion,
        purpose: impl Into<String>,
    ) -> ZurResult<Self> {
        domain.validate().map_err(|error| {
            ZurError::invalid_input(
                format!("invalid domain identity: {error}"),
            )
            .with_component("zur.domains.descriptor")
            .with_source(error)
        })?;

        let purpose = purpose.into();

        validate_description(
            &purpose,
            "domain purpose",
        )?;

        Ok(Self {
            domain,
            contract_version,
            purpose,
            interfaces: BTreeSet::new(),
            capability_evidence: BTreeSet::new(),
            resource_requirements: BTreeSet::new(),
            execution_models: BTreeSet::new(),
            interoperability: BTreeSet::new(),
        })
    }

    // ------------------------------------------------------------------------
    // Domain identity and version
    // ------------------------------------------------------------------------

    /// Returns the canonical computational-domain identity.
    #[must_use]
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Returns the domain contract version.
    ///
    /// This is independent of:
    ///
    /// - Zamani language version.
    /// - AST schema version.
    /// - ZUIR version.
    /// - Compiler version.
    /// - Backend version.
    /// - Physical hardware version.
    #[must_use]
    pub fn contract_version(&self) -> &ServiceContractVersion {
        &self.contract_version
    }

    /// Returns the domain's semantic purpose.
    #[must_use]
    pub fn purpose(&self) -> &str {
        &self.purpose
    }

    // ------------------------------------------------------------------------
    // Provider interfaces
    // ------------------------------------------------------------------------

    /// Inserts a supported provider interface reference.
    ///
    /// The interface remains owned by the canonical ZUR provider contract.
    ///
    /// Duplicate declarations are rejected.
    pub fn insert_interface(
        &mut self,
        interface: ProviderInterfaceId,
    ) -> ZurResult<()> {
        if !self.interfaces.insert(interface.clone()) {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "provider interface {interface} \
                         is already declared for domain {}",
                        self.domain.qualified_name(),
                    ),
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        Ok(())
    }

    /// Returns supported provider-interface references.
    #[must_use]
    pub fn interfaces(&self) -> &BTreeSet<ProviderInterfaceId> {
        &self.interfaces
    }

    // ------------------------------------------------------------------------
    // Capability evidence
    // ------------------------------------------------------------------------

    /// Inserts a reference to externally owned capability evidence.
    ///
    /// This does not establish evidence authenticity, freshness,
    /// or capability availability.
    ///
    /// Duplicate references are rejected.
    pub fn insert_capability_evidence(
        &mut self,
        evidence: CapabilityEvidenceRef,
    ) -> ZurResult<()> {
        if !self.capability_evidence.insert(evidence.clone()) {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "capability evidence reference \
                         {}:{} is already declared",
                        evidence.authority(),
                        evidence.record(),
                    ),
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        Ok(())
    }

    /// Returns capability-evidence references.
    #[must_use]
    pub fn capability_evidence(
        &self,
    ) -> &BTreeSet<CapabilityEvidenceRef> {
        &self.capability_evidence
    }

    // ------------------------------------------------------------------------
    // Resource requirements
    // ------------------------------------------------------------------------

    /// Inserts a reference to a canonical resource requirement.
    ///
    /// A requirement is not a capacity declaration, availability claim,
    /// allocation, reservation, or lease.
    ///
    /// Duplicate references are rejected.
    pub fn insert_resource_requirement(
        &mut self,
        requirement: ResourceRequirementRef,
    ) -> ZurResult<()> {
        if !self.resource_requirements.insert(requirement.clone()) {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "resource requirement reference \
                         {}:{} is already declared",
                        requirement.authority(),
                        requirement.requirement(),
                    ),
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        Ok(())
    }

    /// Returns canonical resource-requirement references.
    #[must_use]
    pub fn resource_requirements(
        &self,
    ) -> &BTreeSet<ResourceRequirementRef> {
        &self.resource_requirements
    }

    // ------------------------------------------------------------------------
    // Execution models
    // ------------------------------------------------------------------------

    /// Inserts an externally owned execution-model reference.
    ///
    /// Execution-model identities are open-world.
    ///
    /// This method does not select or execute an execution model.
    ///
    /// Duplicate declarations are rejected.
    pub fn insert_execution_model(
        &mut self,
        model: ExecutionModelRef,
    ) -> ZurResult<()> {
        if !self.execution_models.insert(model.clone()) {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "execution model {}:{} is already declared",
                        model.authority(),
                        model.model(),
                    ),
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        Ok(())
    }

    /// Returns declared execution models.
    #[must_use]
    pub fn execution_models(
        &self,
    ) -> &BTreeSet<ExecutionModelRef> {
        &self.execution_models
    }

    // ------------------------------------------------------------------------
    // Interoperability
    // ------------------------------------------------------------------------

    /// Inserts an interoperability declaration.
    ///
    /// Self-interoperability is prohibited.
    ///
    /// Duplicate declarations are rejected.
    pub fn insert_interoperability(
        &mut self,
        declaration: InteroperabilityDeclaration,
    ) -> ZurResult<()> {
        if declaration.domain() == &self.domain {
            return Err(
                ZurError::invalid_input(
                    "domain interoperability cannot reference \
                     the owning domain itself",
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        if !self.interoperability.insert(declaration.clone()) {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "interoperability declaration for domain {} \
                         and contract {}:{} is already present",
                        declaration.domain().qualified_name(),
                        declaration.contract().authority(),
                        declaration.contract().contract(),
                    ),
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        Ok(())
    }

    /// Returns declared interoperability relationships.
    #[must_use]
    pub fn interoperability(
        &self,
    ) -> &BTreeSet<InteroperabilityDeclaration> {
        &self.interoperability
    }

    // ------------------------------------------------------------------------
    // Structural validation
    // ------------------------------------------------------------------------

    /// Validates the complete descriptor.
    ///
    /// This method performs structural validation only.
    ///
    /// It does not:
    ///
    /// - Resolve providers.
    /// - Discover hardware.
    /// - Authenticate evidence.
    /// - Validate capability availability.
    /// - Allocate resources.
    /// - Evaluate feasibility.
    /// - Select an execution strategy.
    /// - Establish interoperability implementation compatibility.
    ///
    /// Those decisions belong to the appropriate downstream authorities.
    pub fn validate(&self) -> ZurResult<()> {
        self.domain.validate().map_err(|error| {
            ZurError::invalid_input(
                format!("invalid domain identity: {error}"),
            )
            .with_component("zur.domains.descriptor")
            .with_source(error)
        })?;

        validate_description(
            &self.purpose,
            "domain purpose",
        )?;

        if self.interfaces.is_empty() {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "domain {} must declare at least one \
                         provider interface",
                        self.domain.qualified_name(),
                    ),
                )
                .with_component("zur.domains.descriptor"),
            );
        }

        for interface in &self.interfaces {
            if interface.as_str().is_empty() {
                return Err(
                    ZurError::invalid_input(
                        "domain descriptor contains an empty interface identity",
                    )
                    .with_component("zur.domains.descriptor"),
                );
            }
        }

        for model in &self.execution_models {
            validate_qualified_identifier(
                model.model().as_str(),
                "execution-model identity",
            )?;

            validate_qualified_identifier(
                model.authority(),
                "execution-model authority",
            )?;
        }

        for declaration in &self.interoperability {
            if declaration.domain() == &self.domain {
                return Err(
                    ZurError::invalid_input(
                        "domain descriptor contains self-interoperability",
                    )
                    .with_component("zur.domains.descriptor"),
                );
            }
        }

        Ok(())
    }

    /// Returns whether the descriptor declares a particular provider
    /// interface.
    #[must_use]
    pub fn supports_interface(
        &self,
        interface: &ProviderInterfaceId,
    ) -> bool {
        self.interfaces.contains(interface)
    }

    /// Returns whether the descriptor declares a particular execution model.
    #[must_use]
    pub fn declares_execution_model(
        &self,
        model: &ExecutionModelRef,
    ) -> bool {
        self.execution_models.contains(model)
    }

    /// Returns whether interoperability with the supplied domain
    /// is declared.
    ///
    /// This is a declaration lookup, not proof of compatibility.
    #[must_use]
    pub fn declares_interoperability_with(
        &self,
        domain: &Domain,
    ) -> bool {
        self.interoperability
            .iter()
            .any(|entry| entry.domain() == domain)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::version::{
        ServiceContractVersion,
        Version,
    };

    fn version() -> ServiceContractVersion {
        ServiceContractVersion::new(
            Version::new(1, 0, 0),
        )
    }

    fn domain(namespace: &str, name: &str) -> Domain {
        Domain::new(namespace, name)
            .expect("valid domain identity")
    }

    fn interface(name: &str) -> ProviderInterfaceId {
        ProviderInterfaceId::new(name)
            .expect("valid provider interface")
    }

    fn descriptor() -> DomainDescriptor {
        let mut descriptor = DomainDescriptor::new(
            domain("zamani", "classical"),
            version(),
            "General-purpose classical computation",
        )
        .expect("valid descriptor");

        descriptor
            .insert_interface(interface("zamani.classical.execution"))
            .expect("interface insertion");

        descriptor
    }

    #[test]
    fn constructs_valid_domain_descriptor() {
        let descriptor = descriptor();

        assert_eq!(
            descriptor.domain().qualified_name(),
            "zamani.classical",
        );

        assert_eq!(
            descriptor.purpose(),
            "General-purpose classical computation",
        );

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn rejects_empty_purpose() {
        assert!(
            DomainDescriptor::new(
                domain("zamani", "classical"),
                version(),
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_purpose() {
        assert!(
            DomainDescriptor::new(
                domain("zamani", "classical"),
                version(),
                "Classical\ncomputation",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_descriptor_without_interfaces() {
        let descriptor = DomainDescriptor::new(
            domain("zamani", "classical"),
            version(),
            "Classical computation",
        )
        .expect("descriptor construction");

        assert!(descriptor.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_interfaces() {
        let mut descriptor = descriptor();

        let duplicate =
            interface("zamani.classical.execution");

        assert!(
            descriptor
                .insert_interface(duplicate)
                .is_err()
        );
    }

    #[test]
    fn accepts_future_execution_models() {
        let mut descriptor = descriptor();

        let model = ExecutionModelRef::new(
            "future.photonics",
            ExecutionModelId::new(
                "future.photonics.optical-computation",
            )
            .expect("valid model identity"),
        )
        .expect("valid model reference");

        descriptor
            .insert_execution_model(model.clone())
            .expect("model insertion");

        assert!(
            descriptor.declares_execution_model(&model)
        );
    }

    #[test]
    fn rejects_malformed_execution_model_identity() {
        assert!(
            ExecutionModelId::new(
                "future..photonics",
            )
            .is_err()
        );

        assert!(
            ExecutionModelId::new(
                " future.photonics",
            )
            .is_err()
        );
    }

    #[test]
    fn supports_large_execution_model_identifiers() {
        let segment = "a".repeat(16_384);

        let identifier = format!(
            "future.{segment}.execution"
        );

        let model = ExecutionModelId::new(
            identifier.clone(),
        )
        .expect("large valid identifier");

        assert_eq!(
            model.as_str(),
            identifier,
        );
    }

    #[test]
    fn rejects_self_interoperability() {
        let owner = domain("zamani", "classical");

        let contract =
            InteroperabilityContractRef::new(
                "zamani.interoperability",
                "zamani.interoperability.classical",
            )
            .expect("valid contract reference");

        assert!(
            InteroperabilityDeclaration::new(
                &owner,
                owner.clone(),
                contract,
            )
            .is_err()
        );
    }

    #[test]
    fn supports_cross_domain_interoperability() {
        let owner = domain("zamani", "classical");

        let other = domain("zamani", "quantum");

        let contract =
            InteroperabilityContractRef::new(
                "zamani.interoperability",
                "zamani.interoperability.quantum-classical",
            )
            .expect("valid contract reference");

        let declaration =
            InteroperabilityDeclaration::new(
                &owner,
                other.clone(),
                contract,
            )
            .expect("valid interoperability declaration");

        let mut descriptor = descriptor();

        descriptor
            .insert_interoperability(declaration)
            .expect("interoperability insertion");

        assert!(
            descriptor
                .declares_interoperability_with(&other)
        );
    }

    #[test]
    fn rejects_duplicate_interoperability_declarations() {
        let mut descriptor = descriptor();

        let owner = descriptor.domain().clone();

        let contract =
            InteroperabilityContractRef::new(
                "zamani.interoperability",
                "zamani.interoperability.classical-quantum",
            )
            .expect("valid contract");

        let declaration =
            InteroperabilityDeclaration::new(
                &owner,
                domain("zamani", "quantum"),
                contract,
            )
            .expect("valid declaration");

        descriptor
            .insert_interoperability(declaration.clone())
            .expect("first insertion");

        assert!(
            descriptor
                .insert_interoperability(declaration)
                .is_err()
        );
    }

    #[test]
    fn preserves_deterministic_interface_order() {
        let mut descriptor = descriptor();

        descriptor
            .insert_interface(
                interface("vendor.quantum.execution"),
            )
            .expect("interface insertion");

        descriptor
            .insert_interface(
                interface("future.photonics.execution"),
            )
            .expect("interface insertion");

        let identities: Vec<_> = descriptor
            .interfaces()
            .iter()
            .map(|interface| interface.as_str())
            .collect();

        let mut sorted = identities.clone();
        sorted.sort_unstable();

        assert_eq!(identities, sorted);
    }

    #[test]
    fn domain_identity_is_not_a_closed_enum() {
        let identities = [
            domain("zamani", "classical"),
            domain("zamani", "quantum"),
            domain("zamani", "hdl"),
            domain("zamani", "ai"),
            domain("future", "photonics"),
            domain("future", "unknown-computing-model"),
        ];

        for identity in identities {
            let mut descriptor = DomainDescriptor::new(
                identity,
                version(),
                "Extensible computational domain",
            )
            .expect("valid descriptor");

            descriptor
                .insert_interface(
                    interface("future.execution.interface"),
                )
                .expect("interface insertion");

            assert!(descriptor.validate().is_ok());
        }
    }

    #[test]
    fn descriptor_clone_preserves_contents() {
        let original = descriptor();
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn does_not_claim_execution_from_declarations() {
        let descriptor = descriptor();

        // An interface declaration is descriptive metadata.
        // It does not establish provider availability or execution success.

        assert!(
            descriptor.supports_interface(
                &interface("zamani.classical.execution")
            )
        );

        // Execution must be established by the provider and execution
        // lifecycle, not by this descriptor.
    }
}
