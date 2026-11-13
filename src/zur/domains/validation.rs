
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Validation
//!
//! Path: src/zur/domains/validation.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provides production-grade structural validation for computational-domain
//! descriptors and their relationships with ZUR execution providers.
//!
//! Supports open-world computational domains without requiring modifications
//! when new computational technologies are introduced.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Domain validation orchestration.
//! - Domain descriptor-set validation.
//! - Duplicate domain detection.
//! - Provider/domain structural consistency.
//! - Exact provider-interface matching.
//! - Capability-evidence reference resolution through an injected authority.
//! - Structured domain validation reports.
//!
//! This module does NOT own:
//!
//! - Domain identity.
//! - Domain semantics.
//! - Provider identity.
//! - Provider registration.
//! - Provider discovery.
//! - Provider lifecycle.
//! - Capability definitions.
//! - Capability evidence authentication.
//! - Capability availability discovery.
//! - Resource requirements.
//! - Resource capacity.
//! - Resource allocation.
//! - Resource reservation.
//! - Version compatibility decisions.
//! - Execution feasibility.
//! - Hardware selection.
//! - Backend registration.
//! - Compiler optimization.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Scheduling.
//! - Routing.
//! - Physical execution.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! EXISTING AUTHORITIES
//!
//! Domain identity:
//!     frontend::ast::node::domains::domain::Domain
//!
//! Domain descriptors:
//!     zur::domains::descriptor::DomainDescriptor
//!
//! Provider identity:
//!     stdlib::provider::ProviderDescriptor
//!
//! Provider contracts:
//!     zur::contracts::provider::ProviderContract
//!
//! Provider structural validation:
//!     zur::contracts::provider::validate_provider_contract
//!
//! Capability definitions and status:
//!     stdlib::capability
//!
//! Capability evidence references:
//!     zur::contracts::provider::CapabilityEvidenceRef
//!
//! Contract versions:
//!     zur::version
//!
//! Operational errors:
//!     zur::error
//!
//! GRAMMAR BOUNDARY
//!
//! Grammar describes source-level intent.
//!
//! Semantic analysis establishes source-language meaning.
//!
//! ZUR validation consumes already-established domain descriptors,
//! provider contracts and capability evidence.
//!
//! This module never parses source code or interprets grammar productions.
//!
//! Relevant specifications:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/spec/resources.md
//! - grammar/spec/compatibility.md
//!
//! SAFETY
//!
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No implicit provider selection.
//! No hardware discovery.
//! No fabricated capability availability.
//! No fabricated resource reservations.
//! No fabricated execution success.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Computational domains.
//! - Domain descriptors.
//! - Providers.
//! - Provider interfaces.
//! - Capability evidence references.
//! - Resource requirement references.
//! - Execution models.
//! - Interoperability declarations.
//!
//! Collections grow subject to available resources.
//!
//! No finite implementation promises literal infinite computation.
//!
//! INTEGRATION
//!
//! Upstream:
//!
//! - frontend AST domain identity;
//! - semantic domain resolution;
//! - zur/domains/descriptor.rs;
//! - zur/contracts/provider.rs;
//! - stdlib/capability.rs;
//! - zur/version.rs;
//! - zur/error.rs.
//!
//! Downstream:
//!
//! - zur/domains/registry.rs;
//! - zur/domains/compatibility.rs;
//! - zur/domains/composition.rs;
//! - zur/domains/conformance.rs;
//! - zur/execution/dispatch.rs;
//! - zur/execution/executor.rs.
//!
//! Registration, discovery, compatibility and execution remain separate.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::ast::node::domains::domain::Domain;

use crate::stdlib::capability::{
    CapabilityId,
    CapabilityState,
    CapabilityStatus,
};

use crate::stdlib::provider::{
    ProviderDescriptor,
    ProviderId,
};

use crate::zur::contracts::provider::{
    validate_provider_contract,
    CapabilityEvidenceRef,
    ProviderContract,
    ProviderInterfaceId,
};

use crate::zur::domains::descriptor::DomainDescriptor;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::version::ServiceContractVersion;

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.domains.validation";

// ============================================================================
// CAPABILITY EVIDENCE RESOLUTION
// ============================================================================

/// Resolves an externally owned capability-evidence reference.
///
/// The implementation must obtain the capability state from the canonical
/// capability authority responsible for the referenced evidence.
///
/// This interface does not authenticate evidence itself.
///
/// Authentication, provenance verification and freshness remain the
/// responsibility of the evidence authority.
///
/// Implementations must not fabricate an evidence record when a reference
/// cannot be resolved.
///
/// This trait deliberately accepts an external resolver instead of
/// introducing another capability registry or evidence database.
pub trait CapabilityEvidenceResolver: Send + Sync {

    /// Resolves a canonical capability-evidence reference.
    ///
    /// Returns:
    ///
    /// - `Some(state)` when the reference has been resolved.
    /// - `None` when the authority cannot resolve the reference.
    ///
    /// A returned state may have any canonical availability status:
    ///
    /// - Available.
    /// - Unavailable.
    /// - Conditional.
    /// - Unknown.
    ///
    /// The caller must preserve that distinction.
    fn resolve<'a>(
        &'a self,
        reference: &CapabilityEvidenceRef,
    ) -> Option<&'a CapabilityState>;
}

/// Implements evidence resolution for an explicitly supplied evidence map.
///
/// The map is supplied by the caller and is not global state.
///
/// Its keys are canonical evidence references.
///
/// Its values are canonical capability states.
///
/// The map does not independently establish evidence authenticity.
impl CapabilityEvidenceResolver
    for BTreeMap<CapabilityEvidenceRef, CapabilityState>
{
    fn resolve<'a>(
        &'a self,
        reference: &CapabilityEvidenceRef,
    ) -> Option<&'a CapabilityState> {

        self.get(reference)
    }
}

// ============================================================================
// EVIDENCE VALIDATION RESULTS
// ============================================================================

/// The result of resolving one capability-evidence reference.
///
/// This describes the information returned by the supplied evidence
/// authority.
///
/// It does not establish that the evidence is authenticated, fresh,
/// authorized or sufficient to execute a particular computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityEvidenceObservation {

    reference: CapabilityEvidenceRef,

    capability: CapabilityId,

    status: CapabilityStatus,

    descriptive_evidence_count: usize,
}

impl CapabilityEvidenceObservation {

    /// Creates an observation from resolved canonical capability state.
    fn from_resolved(
        reference: CapabilityEvidenceRef,
        state: &CapabilityState,
    ) -> Self {

        Self {
            reference,
            capability: state.id().clone(),
            status: state.status(),
            descriptive_evidence_count: state.evidence().len(),
        }
    }

    /// Returns the original canonical evidence reference.
    #[must_use]
    pub fn reference(&self) -> &CapabilityEvidenceRef {
        &self.reference
    }

    /// Returns the capability identity reported by the authority.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the reported capability availability state.
    #[must_use]
    pub const fn status(&self) -> CapabilityStatus {
        self.status
    }

    /// Returns the number of descriptive evidence records supplied
    /// by the capability authority.
    #[must_use]
    pub const fn descriptive_evidence_count(&self) -> usize {
        self.descriptive_evidence_count
    }

    /// Returns whether the authority reports unconditional availability.
    ///
    /// This does not establish that a particular operation is feasible.
    #[must_use]
    pub const fn establishes_availability(&self) -> bool {
        matches!(self.status, CapabilityStatus::Available)
    }
}

// ============================================================================
// VALIDATED DOMAIN INFORMATION
// ============================================================================

/// Structurally validated domain information.
///
/// This structure preserves canonical domain identity and contract version.
///
/// It does not perform version compatibility negotiation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedDomain {

    domain: Domain,

    contract_version: ServiceContractVersion,

    implemented_interfaces: Vec<ProviderInterfaceId>,
}

impl ValidatedDomain {

    /// Returns the canonical domain identity.
    #[must_use]
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Returns the declared domain contract version.
    #[must_use]
    pub fn contract_version(&self) -> &ServiceContractVersion {
        &self.contract_version
    }

    /// Returns provider interfaces that exactly match this domain's
    /// declared interfaces.
    #[must_use]
    pub fn implemented_interfaces(&self) -> &[ProviderInterfaceId] {
        &self.implemented_interfaces
    }
}

// ============================================================================
// DOMAIN VALIDATION REPORT
// ============================================================================

/// Complete structural validation report for a domain-provider integration.
///
/// The report is produced only after the supplied descriptors and provider
/// contract pass structural validation.
///
/// It is not an execution-feasibility report.
///
/// It does not establish provider availability, resource availability,
/// reservation, authorization or successful execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainValidationReport {

    provider_id: ProviderId,

    provider_contract_version: ServiceContractVersion,

    domains: Vec<ValidatedDomain>,

    capability_evidence: Option<Vec<CapabilityEvidenceObservation>>,
}

impl DomainValidationReport {

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the provider contract version.
    #[must_use]
    pub fn provider_contract_version(&self) -> &ServiceContractVersion {
        &self.provider_contract_version
    }

    /// Returns all structurally validated domains.
    #[must_use]
    pub fn domains(&self) -> &[ValidatedDomain] {
        &self.domains
    }

    /// Returns the number of validated domains.
    #[must_use]
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }

    /// Returns the total number of matched provider interfaces.
    #[must_use]
    pub fn implemented_interface_count(&self) -> usize {
        self.domains
            .iter()
            .map(|domain| domain.implemented_interfaces.len())
            .sum()
    }

    /// Returns whether capability evidence resolution was requested.
    #[must_use]
    pub const fn capability_evidence_was_resolved(&self) -> bool {
        self.capability_evidence.is_some()
    }

    /// Returns resolved capability-evidence observations.
    ///
    /// `None` means no external resolver was supplied.
    ///
    /// `Some(empty)` means resolution was requested but no references
    /// required resolution.
    #[must_use]
    pub fn capability_evidence(
        &self,
    ) -> Option<&[CapabilityEvidenceObservation]> {

        self.capability_evidence.as_deref()
    }

    /// Returns whether the structural validation report contains domains.
    ///
    /// This is not a statement about execution feasibility.
    #[must_use]
    pub fn has_domains(&self) -> bool {
        !self.domains.is_empty()
    }
}

// ============================================================================
// DOMAIN DESCRIPTOR VALIDATION
// ============================================================================

/// Validates one canonical domain descriptor.
///
/// Delegates structural validation to DomainDescriptor.
///
/// No second descriptor validation implementation is introduced.
///
/// # Errors
///
/// Returns the canonical structured validation error if the descriptor
/// violates its structural invariants.
pub fn validate_domain_descriptor(
    descriptor: &DomainDescriptor,
) -> ZurResult<()> {

    descriptor.validate().map_err(|error| {

        error
            .with_component(COMPONENT)
            .with_operation("validate_domain_descriptor")
    })
}

// ============================================================================
// DOMAIN SET VALIDATION
// ============================================================================

/// Validates a collection of domain descriptors.
///
/// The function verifies:
///
/// - Each descriptor is structurally valid.
/// - No canonical domain identity is repeated.
///
/// A BTreeSet is used for deterministic identity lookup.
///
/// This avoids the quadratic pairwise duplicate search used by simpler
/// collection-validation implementations.
///
/// The function does not require that a collection contain a particular
/// domain or a predetermined number of domains.
///
/// An empty collection is structurally valid here. Provider validation
/// separately requires at least one declared domain.
///
/// # Errors
///
/// Returns an error if any descriptor is invalid or a domain identity
/// occurs more than once.
pub fn validate_domain_descriptors(
    descriptors: &[DomainDescriptor],
) -> ZurResult<()> {

    let mut identities = BTreeSet::<&Domain>::new();

    for descriptor in descriptors {

        validate_domain_descriptor(descriptor)?;

        if !identities.insert(descriptor.domain()) {

            return Err(
                ZurError::invalid_input(
                    format!(
                        "computational domain {} is declared more than once",
                        descriptor.domain().qualified_name(),
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("validate_domain_descriptors"),
            );
        }
    }

    Ok(())
}

// ============================================================================
// CAPABILITY EVIDENCE RESOLUTION
// ============================================================================

/// Resolves all supplied canonical capability-evidence references.
///
/// Duplicate references are processed only once.
///
/// The function preserves the canonical availability status without
/// interpreting it as feasibility or resource availability.
///
/// When a resolver is supplied, every referenced record must be resolvable.
///
/// When no resolver is supplied, callers must treat evidence references
/// as unresolved external metadata rather than as verified evidence.
///
/// # Errors
///
/// Returns an error if the supplied authority cannot resolve a reference.
pub fn resolve_capability_evidence(
    references: impl IntoIterator<Item = CapabilityEvidenceRef>,
    resolver: &dyn CapabilityEvidenceResolver,
) -> ZurResult<Vec<CapabilityEvidenceObservation>> {

    let unique_references: BTreeSet<_> =
        references.into_iter().collect();

    let mut observations = Vec::with_capacity(unique_references.len());

    for reference in unique_references {

        let state = resolver.resolve(&reference).ok_or_else(|| {

            ZurError::invalid_data(
                format!(
                    "capability evidence reference {}:{} \
                     could not be resolved",
                    reference.authority(),
                    reference.record(),
                ),
            )
            .with_component(COMPONENT)
            .with_operation("resolve_capability_evidence")
        })?;

        observations.push(
            CapabilityEvidenceObservation::from_resolved(
                reference,
                state,
            ),
        );
    }

    Ok(observations)
}

// ============================================================================
// PROVIDER / DOMAIN VALIDATION
// ============================================================================

/// Validates the complete structural relationship between a provider
/// descriptor, its execution contract and its declared computational domains.
///
/// This function is the principal entry point for domain-provider validation.
///
/// It validates:
///
/// 1. Provider descriptor identity consistency.
/// 2. Provider execution-contract structure.
/// 3. Domain descriptor structure.
/// 4. Duplicate domain identities.
/// 5. Exact provider-interface implementation.
/// 6. Optional capability-evidence reference resolution.
///
/// It deliberately does not:
///
/// - Perform provider discovery.
/// - Register providers.
/// - Initialize providers.
/// - Authenticate providers.
/// - Select providers.
/// - Compare version compatibility policies.
/// - Evaluate execution feasibility.
/// - Discover hardware.
/// - Establish capability availability independently.
/// - Allocate or reserve resources.
/// - Execute computations.
///
/// Version compatibility remains the responsibility of the canonical
/// domain compatibility subsystem.
///
/// Capability evidence authentication remains the responsibility of
/// the evidence authority.
///
/// # Arguments
///
/// * `provider_descriptor` - Canonical standard-library provider descriptor.
/// * `provider_contract` - Canonical ZUR execution-provider contract.
/// * `domains` - Domain descriptors implemented by the provider.
/// * `evidence_resolver` - Optional external capability-evidence authority.
///
/// # Errors
///
/// Returns an error if:
///
/// - Provider descriptors disagree.
/// - The provider contract is structurally invalid.
/// - No domains are supplied.
/// - A domain descriptor is invalid.
/// - Duplicate domain identities exist.
/// - A domain has no matching provider interface.
/// - Supplied evidence references cannot be resolved.
///
/// # Important
///
/// Successful validation does not mean that the provider is available,
/// that a required resource exists, or that execution is feasible.
pub fn validate_domain_provider(
    provider_descriptor: &ProviderDescriptor,
    provider_contract: &ProviderContract,
    domains: &[DomainDescriptor],
    evidence_resolver: Option<&dyn CapabilityEvidenceResolver>,
) -> ZurResult<DomainValidationReport> {

    // ------------------------------------------------------------------------
    // 1. Provider identity consistency
    // ------------------------------------------------------------------------

    if provider_descriptor != provider_contract.descriptor() {

        return Err(
            ZurError::invalid_data(
                "standard-library provider descriptor and \
                 ZUR provider contract descriptor disagree",
            )
            .with_component(COMPONENT)
            .with_operation("validate_domain_provider")
            .with_provider(
                provider_descriptor.id().to_string(),
            ),
        );
    }

    // ------------------------------------------------------------------------
    // 2. Canonical provider-contract validation
    // ------------------------------------------------------------------------

    validate_provider_contract(provider_contract)
        .map_err(|error| {

            error
                .with_component(COMPONENT)
                .with_operation("validate_domain_provider")
                .with_provider(
                    provider_descriptor.id().to_string(),
                )
        })?;

    // ------------------------------------------------------------------------
    // 3. At least one domain is required for a domain provider
    // ------------------------------------------------------------------------

    if domains.is_empty() {

        return Err(
            ZurError::invalid_input(
                "a domain provider must declare at least one domain",
            )
            .with_component(COMPONENT)
            .with_operation("validate_domain_provider")
            .with_provider(
                provider_descriptor.id().to_string(),
            ),
        );
    }

    // ------------------------------------------------------------------------
    // 4. Domain descriptors and duplicate identities
    // ------------------------------------------------------------------------

    let mut domain_identities = BTreeSet::<&Domain>::new();

    let mut validated_domains =
        Vec::with_capacity(domains.len());

    let mut evidence_references =
        BTreeSet::<CapabilityEvidenceRef>::new();

    for domain_descriptor in domains {

        validate_domain_descriptor(domain_descriptor)
            .map_err(|error| {

                error
                    .with_operation("validate_domain_provider")
                    .with_provider(
                        provider_descriptor.id().to_string(),
                    )
            })?;

        let domain = domain_descriptor.domain();

        if !domain_identities.insert(domain) {

            return Err(
                ZurError::invalid_input(
                    format!(
                        "provider {} declares computational domain \
                         {} more than once",
                        provider_descriptor.id(),
                        domain.qualified_name(),
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("validate_domain_provider")
                .with_provider(
                    provider_descriptor.id().to_string(),
                ),
            );
        }

        // --------------------------------------------------------------------
        // 5. Exact provider-interface implementation
        // --------------------------------------------------------------------

        let mut implemented_interfaces = Vec::new();

        for declared_interface in domain_descriptor.interfaces() {

            if let Some(implemented) =
                provider_contract.interface(declared_interface)
            {
                implemented_interfaces.push(
                    implemented.id().clone(),
                );
            }
        }

        if implemented_interfaces.is_empty() {

            return Err(
                ZurError::compatibility(
                    format!(
                        "provider {} does not implement any interface \
                         declared by domain {}",
                        provider_descriptor.id(),
                        domain.qualified_name(),
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("validate_domain_provider")
                .with_provider(
                    provider_descriptor.id().to_string(),
                ),
            );
        }

        // Preserve the canonical domain version.
        //
        // Version compatibility is intentionally not decided here.
        // The compatibility subsystem owns that decision.

        validated_domains.push(
            ValidatedDomain {
                domain: domain.clone(),

                contract_version:
                    domain_descriptor.contract_version().clone(),

                implemented_interfaces,
            },
        );

        // Collect canonical evidence references.
        evidence_references.extend(
            domain_descriptor
                .capability_evidence()
                .iter()
                .cloned(),
        );
    }

    // ------------------------------------------------------------------------
    // 6. Provider-level capability evidence references
    // ------------------------------------------------------------------------

    evidence_references.extend(
        provider_contract
            .capability_evidence()
            .iter()
            .cloned(),
    );

    // ------------------------------------------------------------------------
    // 7. Optional evidence resolution
    // ------------------------------------------------------------------------

    let capability_evidence =
        match evidence_resolver {

            Some(resolver) => {

                Some(
                    resolve_capability_evidence(
                        evidence_references,
                        resolver,
                    )
                    .map_err(|error| {

                        error
                            .with_operation("validate_domain_provider")
                            .with_provider(
                                provider_descriptor.id().to_string(),
                            )
                    })?
                )
            }

            None => None,
        };

    // ------------------------------------------------------------------------
    // 8. Complete structural report
    // ------------------------------------------------------------------------

    Ok(
        DomainValidationReport {

            provider_id:
                provider_descriptor.id().clone(),

            provider_contract_version:
                provider_contract.contract_version().clone(),

            domains:
                validated_domains,

            capability_evidence,
        },
    )
}

// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for ValidatedDomain {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        write!(
            formatter,
            "{}@{}",
            self.domain,
            self.contract_version,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::stdlib::capability::{
        Capability,
        CapabilityEvidence,
        CapabilityId,
        CapabilityKind,
        CapabilityState,
    };

    use crate::zur::contracts::provider::{
        ProviderInterfaceId,
    };

    use crate::zur::version::{
        ServiceContractVersion,
        Version,
    };

    fn domain(
        namespace: &str,
        name: &str,
    ) -> Domain {

        Domain::new(namespace, name)
            .expect("valid test domain")
    }

    fn interface(
        name: &str,
    ) -> ProviderInterfaceId {

        ProviderInterfaceId::new(name)
            .expect("valid test interface")
    }

    fn version() -> ServiceContractVersion {

        ServiceContractVersion::new(
            Version::new(1, 0, 0),
        )
    }

    fn descriptor(
        namespace: &str,
        name: &str,
        interface_name: &str,
    ) -> DomainDescriptor {

        let mut descriptor =
            DomainDescriptor::new(
                domain(namespace, name),
                version(),
                format!("{name} test domain"),
            )
            .expect("valid descriptor");

        descriptor
            .insert_interface(interface(interface_name))
            .expect("valid interface insertion");

        descriptor
    }

    #[test]
    fn validates_individual_domain_descriptors() {

        let descriptor = descriptor(
            "zamani",
            "classical",
            "zamani.classical.execution",
        );

        assert!(
            validate_domain_descriptor(&descriptor).is_ok()
        );
    }

    #[test]
    fn validates_empty_domain_collections_structurally() {

        assert!(
            validate_domain_descriptors(&[]).is_ok()
        );
    }

    #[test]
    fn rejects_duplicate_domain_identities() {

        let descriptors = vec![
            descriptor(
                "zamani",
                "classical",
                "zamani.classical.execution",
            ),

            descriptor(
                "zamani",
                "classical",
                "zamani.classical.other",
            ),
        ];

        assert!(
            validate_domain_descriptors(&descriptors).is_err()
        );
    }

    #[test]
    fn accepts_multiple_distinct_domains() {

        let descriptors = vec![
            descriptor(
                "zamani",
                "classical",
                "zamani.classical.execution",
            ),

            descriptor(
                "zamani",
                "quantum",
                "zamani.quantum.execution",
            ),

            descriptor(
                "future",
                "photonics",
                "future.photonics.execution",
            ),
        ];

        assert!(
            validate_domain_descriptors(&descriptors).is_ok()
        );
    }

    #[test]
    fn preserves_open_world_domain_identity() {

        let descriptor = descriptor(
            "future",
            "new_architecture",
            "future.new_architecture.execution",
        );

        assert_eq!(
            descriptor.domain().qualified_name(),
            "future.new_architecture",
        );
    }

    #[test]
    fn evidence_resolver_preserves_unknown_availability() {

        let reference = CapabilityEvidenceRef::new(
            "frontend.semantic.capabilities",
            "evidence.future",
        )
        .expect("valid evidence reference");

        let capability = Capability::new(
            CapabilityId::new(
                "future.photonic.execution",
            )
            .expect("valid capability"),
            CapabilityKind::new(
                "computation",
            )
            .expect("valid capability kind"),
        );

        let state = CapabilityState::new(
            capability,
            CapabilityStatus::Unknown,
        );

        let mut evidence = BTreeMap::new();

        evidence.insert(
            reference.clone(),
            state,
        );

        let observations =
            resolve_capability_evidence(
                [reference],
                &evidence,
            )
            .expect("evidence resolution");

        assert_eq!(
            observations.len(),
            1,
        );

        assert_eq!(
            observations[0].status(),
            CapabilityStatus::Unknown,
        );

        assert!(
            !observations[0].establishes_availability()
        );
    }

    #[test]
    fn evidence_resolver_preserves_unavailable_status() {

        let reference = CapabilityEvidenceRef::new(
            "frontend.semantic.capabilities",
            "evidence.unavailable",
        )
        .unwrap();

        let capability = Capability::new(
            CapabilityId::new(
                "example.execution",
            )
            .unwrap(),
            CapabilityKind::new(
                "computation",
            )
            .unwrap(),
        );

        let state = CapabilityState::new(
            capability,
            CapabilityStatus::Unavailable,
        );

        let mut evidence = BTreeMap::new();

        evidence.insert(
            reference.clone(),
            state,
        );

        let observations =
            resolve_capability_evidence(
                [reference],
                &evidence,
            )
            .unwrap();

        assert_eq!(
            observations[0].status(),
            CapabilityStatus::Unavailable,
        );
    }

    #[test]
    fn unresolved_evidence_is_not_silently_accepted() {

        let reference = CapabilityEvidenceRef::new(
            "frontend.semantic.capabilities",
            "evidence.missing",
        )
        .unwrap();

        let evidence =
            BTreeMap::<
                CapabilityEvidenceRef,
                CapabilityState,
            >::new();

        assert!(
            resolve_capability_evidence(
                [reference],
                &evidence,
            )
            .is_err()
        );
    }

    #[test]
    fn duplicate_evidence_references_are_resolved_once() {

        let reference = CapabilityEvidenceRef::new(
            "frontend.semantic.capabilities",
            "evidence.shared",
        )
        .unwrap();

        let capability = Capability::new(
            CapabilityId::new(
                "example.execution",
            )
            .unwrap(),
            CapabilityKind::new(
                "computation",
            )
            .unwrap(),
        );

        let state = CapabilityState::new(
            capability,
            CapabilityStatus::Conditional,
        );

        let mut evidence = BTreeMap::new();

        evidence.insert(
            reference.clone(),
            state,
        );

        let observations =
            resolve_capability_evidence(
                [
                    reference.clone(),
                    reference.clone(),
                    reference,
                ],
                &evidence,
            )
            .unwrap();

        assert_eq!(
            observations.len(),
            1,
        );

        assert_eq!(
            observations[0].status(),
            CapabilityStatus::Conditional,
        );
    }

    #[test]
    fn capability_evidence_can_include_descriptive_provenance() {

        let reference = CapabilityEvidenceRef::new(
            "runtime.capabilities",
            "evidence.local",
        )
        .unwrap();

        let capability = Capability::new(
            CapabilityId::new(
                "classical.execution",
            )
            .unwrap(),
            CapabilityKind::new(
                "computation",
            )
            .unwrap(),
        );

        let descriptive_evidence =
            CapabilityEvidence::new(
                "test-provider",
                "Capability information supplied by a test authority",
            )
            .unwrap();

        let state = CapabilityState::new(
            capability,
            CapabilityStatus::Available,
        )
        .with_evidence(descriptive_evidence);

        let mut evidence = BTreeMap::new();

        evidence.insert(
            reference.clone(),
            state,
        );

        let observations =
            resolve_capability_evidence(
                [reference],
                &evidence,
            )
            .unwrap();

        assert_eq!(
            observations[0].descriptive_evidence_count(),
            1,
        );

        assert!(
            observations[0].establishes_availability()
        );
    }
}
