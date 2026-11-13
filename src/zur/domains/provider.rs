
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Providers
//!
//! Path: src/zur/domains/provider.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Defines the runtime-facing integration contract for providers
//! implementing one or more computational domains.
//!
//! A domain provider connects an existing canonical domain descriptor
//! to an existing ZUR execution-provider implementation.
//!
//! Providers may implement classical, quantum, HDL, AI, tensor,
//! accelerator, hybrid, distributed, photonic, or future computation.
//!
//! These domains are not enumerated here.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - DomainProvider.
//! - Domain-provider contract validation.
//! - Domain-to-provider-interface consistency validation.
//! - Provider identity consistency validation.
//! - Domain-provider structural validation helpers.
//!
//! This module does NOT own:
//!
//! - Domain identity.
//! - Domain semantics.
//! - Domain descriptors.
//! - Provider identity.
//! - Provider descriptors.
//! - Provider lifecycle.
//! - Provider execution operations.
//! - Capability definitions.
//! - Capability evidence generation or authentication.
//! - Resource requirements.
//! - Resource capacity.
//! - Resource allocation.
//! - Resource reservation.
//! - Provider registration.
//! - Provider discovery.
//! - Provider authentication.
//! - Provider authorization.
//! - Feasibility analysis.
//! - Provider selection.
//! - Hardware discovery.
//! - Backend registration.
//! - Code generation.
//! - Compiler optimization.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//! - Scheduling.
//! - Routing.
//! - Device drivers.
//! - HAL implementation.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! EXISTING AUTHORITIES
//!
//! Domain identity and metadata:
//!     zur::domains::descriptor
//!
//! Common provider identity and lifecycle:
//!     stdlib::provider
//!
//! Provider execution contracts:
//!     zur::contracts::provider
//!
//! Operation cancellation:
//!     stdlib::context
//!
//! Operational errors:
//!     zur::error
//!
//! Contract versions:
//!     zur::version
//!
//! Capability semantics:
//!     stdlib::capability
//!
//! Resource semantics:
//!     frontend semantic/resource authorities
//!
//! GRAMMAR BOUNDARY
//!
//! This module consumes validated domain representations.
//!
//! It does not parse or interpret source grammar.
//!
//! Relevant specifications:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/spec/resources.md
//! - grammar/spec/semantics.md
//! - grammar/spec/compatibility.md
//! - grammar/hardware/
//! - grammar/execution/
//!
//! ARCHITECTURAL INVARIANTS
//!
//! Domain identity != provider identity.
//! Domain identity != provider interface.
//! Provider interface != operation implementation.
//! Capability != capability evidence.
//! Capability evidence != capability availability.
//! Requirement != capability.
//! Requirement != resource reservation.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Submission != successful completion.
//! Provider registration != provider initialization.
//! Provider initialization != provider availability.
//!
//! A domain descriptor is descriptive metadata.
//!
//! A provider contract is an implementation declaration.
//!
//! Neither declaration independently proves that an operation
//! is currently executable.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No fabricated success.
//! No implicit provider selection.
//! No implicit resource allocation.
//! No arbitrary hardware ceilings.
//! No arbitrary computational-domain ceilings.
//!
//! SCALABILITY
//!
//! Providers may expose any number of domain descriptors supported
//! by available memory and the implementation's actual resources.
//!
//! This module imposes no fixed maximum on:
//!
//! - computational domains;
//! - domain descriptors;
//! - providers;
//! - provider interfaces;
//! - operations;
//! - resources;
//! - devices;
//! - execution sessions;
//! - domain combinations.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! INTEGRATION
//!
//! Upstream:
//!
//! - frontend AST domain identity;
//! - zur/domains/descriptor.rs;
//! - zur/contracts/provider.rs;
//! - stdlib/provider.rs;
//! - stdlib/context.rs;
//! - zur/error.rs.
//!
//! Downstream:
//!
//! - zur/domains/registry.rs;
//! - zur/domains/validation.rs;
//! - zur/domains/compatibility.rs;
//! - zur/domains/composition.rs;
//! - zur/domains/conformance.rs;
//! - zur/execution/dispatch.rs;
//! - zur/execution/executor.rs.
//!
//! Provider registration and discovery must be implemented by the
//! corresponding domain registry and discovery modules.

use crate::stdlib::provider::ProviderDescriptor;

use crate::zur::contracts::provider::{
    validate_provider_contract,
    ExecutionProvider,
    ProviderContract,
    ProviderInterfaceId,
};

use crate::zur::domains::descriptor::DomainDescriptor;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.domains.provider";

// ============================================================================
// STRUCTURAL VALIDATION
// ============================================================================

/// Validates a domain-provider integration.
///
/// This operation checks structural consistency between:
///
/// - the canonical provider descriptor;
/// - the canonical ZUR execution-provider contract;
/// - one or more canonical domain descriptors.
///
/// It does not perform provider discovery, authentication,
/// capability verification, feasibility analysis, resource
/// allocation, or execution.
///
/// Each domain descriptor must:
///
/// - pass its own structural validation;
/// - have a unique domain identity in the supplied collection;
/// - declare at least one interface implemented by the provider.
///
/// The provider's common identity must agree with the identity
/// in its ZUR execution contract.
///
/// The provider contract itself must also pass its canonical
/// structural validation.
///
/// # Arguments
///
/// * `provider_descriptor` - Canonical standard-library provider
///   descriptor returned by the provider implementation.
/// * `provider_contract` - Canonical ZUR execution contract.
/// * `domains` - Canonical domain descriptors supported by this
///   provider implementation.
///
/// # Errors
///
/// Returns a structured error when:
///
/// - the provider descriptor and execution contract disagree;
/// - the provider contract is structurally invalid;
/// - no domain descriptor is supplied;
/// - a domain descriptor is invalid;
/// - duplicate domain identities are supplied;
/// - a domain has no matching provider interface.
///
/// # Important
///
/// Passing this validation does not establish:
///
/// - provider authenticity;
/// - provider trust;
/// - provider availability;
/// - capability availability;
/// - requirement satisfaction;
/// - resource availability;
/// - feasibility;
/// - resource reservation;
/// - execution success.
pub fn validate_domain_provider_contract(
    provider_descriptor: &ProviderDescriptor,
    provider_contract: &ProviderContract,
    domains: &[DomainDescriptor],
) -> ZurResult<()> {

    if provider_descriptor != provider_contract.descriptor() {
        return Err(
            ZurError::compatibility(
                "standard-library provider descriptor and ZUR \
                 execution-provider descriptor do not agree",
            )
            .with_component(COMPONENT)
            .with_operation("validate_domain_provider_contract")
            .with_provider(
                provider_descriptor.id().to_string(),
            ),
        );
    }

    validate_provider_contract(provider_contract)
        .map_err(|error| {
            error
                .with_component(COMPONENT)
                .with_operation("validate_domain_provider_contract")
                .with_provider(
                    provider_descriptor.id().to_string(),
                )
        })?;

    if domains.is_empty() {
        return Err(
            ZurError::invalid_input(
                "a domain provider must declare at least one domain",
            )
            .with_component(COMPONENT)
            .with_operation("validate_domain_provider_contract")
            .with_provider(
                provider_descriptor.id().to_string(),
            ),
        );
    }

    // The provider contract is the canonical source of implemented
    // provider interfaces.
    //
    // Collect identities once so that validation is not dependent
    // on the provider's interface declaration order.
    let implemented_interfaces =
        provider_contract
            .interfaces()
            .iter()
            .map(|interface| interface.id().clone())
            .collect::<std::collections::BTreeSet<_>>();

    for (index, domain) in domains.iter().enumerate() {

        domain.validate().map_err(|error| {
            error
                .with_component(COMPONENT)
                .with_operation("validate_domain_provider_contract")
                .with_provider(
                    provider_descriptor.id().to_string(),
                )
        })?;

        // A provider may implement multiple domains, but the same
        // domain must not be declared more than once.
        if domains[..index]
            .iter()
            .any(|previous| previous.domain() == domain.domain())
        {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "domain {} is declared more than once \
                         by provider {}",
                        domain.domain().qualified_name(),
                        provider_descriptor.id(),
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("validate_domain_provider_contract")
                .with_provider(
                    provider_descriptor.id().to_string(),
                ),
            );
        }

        // DomainDescriptor owns the list of interfaces associated
        // with its domain.
        //
        // ProviderContract owns the interfaces implemented by
        // this particular provider.
        //
        // A valid provider/domain relationship requires at least
        // one matching interface.
        let has_implemented_interface =
            domain
                .interfaces()
                .iter()
                .any(|interface| {
                    implemented_interfaces.contains(interface)
                });

        if !has_implemented_interface {
            return Err(
                ZurError::compatibility(
                    format!(
                        "provider {} does not implement any \
                         interface declared by domain {}",
                        provider_descriptor.id(),
                        domain.domain().qualified_name(),
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("validate_domain_provider_contract")
                .with_provider(
                    provider_descriptor.id().to_string(),
                ),
            );
        }
    }

    Ok(())
}

// ============================================================================
// DOMAIN PROVIDER CONTRACT
// ============================================================================

/// Runtime-facing computational-domain provider.
///
/// A domain provider implements the existing ZUR execution-provider
/// contract and exposes its canonical domain descriptors.
///
/// This trait deliberately extends [`ExecutionProvider`] instead
/// of defining another provider lifecycle, identity, operation
/// interface, execution result, or cancellation system.
///
/// A provider may implement multiple computational domains.
///
/// Examples include:
///
/// - classical execution and tensor computation;
/// - classical and quantum execution;
/// - HDL realization and hardware simulation;
/// - AI inference and accelerator execution;
/// - distributed and hybrid computation;
/// - photonic and classical computation;
/// - future combinations of computational domains.
///
/// These examples do not define a closed domain inventory.
///
/// Each descriptor remains owned by the canonical
/// `zur::domains::descriptor` module.
///
/// # Implementor requirements
///
/// Implementations must:
///
/// 1. Implement the inherited standard-library provider contract.
/// 2. Implement provider lifecycle operations.
/// 3. Implement the canonical ZUR execution-provider contract.
/// 4. Return stable domain descriptors for their lifetime.
/// 5. Expose only domains for which their declared interfaces
///    actually provide an implementation.
/// 6. Preserve the canonical domain identities.
/// 7. Propagate actual execution failures.
/// 8. Observe cancellation through `OperationContext`.
/// 9. Never report submission as successful completion.
/// 10. Never fabricate capability availability or resource reservation.
///
/// Domain-specific request and response types remain owned by
/// their canonical domain contracts.
///
/// For example, quantum providers continue to use the canonical
/// quantum hardware and execution interfaces.
///
/// Distributed providers continue to use the canonical distributed
/// execution and transport interfaces.
///
/// HDL providers continue to use the canonical HDL and realization
/// interfaces.
///
/// # Thread safety
///
/// The inherited execution contract requires `Send + Sync`.
///
/// Implementations must provide suitable synchronization for
/// mutable internal state.
///
/// No global mutable state is required.
///
/// # Lifecycle
///
/// Registration and lifecycle management remain separate.
///
/// This trait does not automatically initialize or shut down
/// providers.
///
/// The canonical lifecycle manager coordinates initialization,
/// dependencies, admission, readiness and shutdown.
pub trait DomainProvider: ExecutionProvider {

    /// Returns the canonical descriptors of the computational
    /// domains implemented by this provider.
    ///
    /// The returned collection must remain stable for the
    /// lifetime of the provider instance.
    ///
    /// Each descriptor must declare at least one provider
    /// interface implemented by this provider's execution contract.
    ///
    /// A provider may return multiple descriptors.
    ///
    /// The collection is not a capability-availability snapshot.
    ///
    /// Returning a descriptor does not establish:
    ///
    /// - current provider availability;
    /// - physical device availability;
    /// - capability availability;
    /// - resource reservation;
    /// - execution feasibility.
    fn domain_descriptors(&self) -> &[DomainDescriptor];

    /// Validates this provider's domain integration.
    ///
    /// This method uses the canonical validation function.
    ///
    /// It performs structural validation only.
    ///
    /// It does not initialize the provider, discover hardware,
    /// authenticate the provider, reserve resources, or execute
    /// an operation.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the domain descriptors
    /// and canonical provider execution contract disagree.
    fn validate_domain_integration(&self) -> ZurResult<()> {

        validate_domain_provider_contract(
            self.descriptor(),
            self.execution_contract(),
            self.domain_descriptors(),
        )
    }

    /// Returns whether this provider declares the supplied
    /// canonical domain identity.
    ///
    /// This is an exact descriptor lookup.
    ///
    /// It does not prove that the domain is currently available
    /// or that a particular operation is feasible.
    #[must_use]
    fn declares_domain(
        &self,
        domain: &crate::frontend::ast::node::domains::domain::Domain,
    ) -> bool {

        self.domain_descriptors()
            .iter()
            .any(|descriptor| descriptor.domain() == domain)
    }

    /// Returns whether this provider implements a particular
    /// interface for the supplied domain.
    ///
    /// Both the domain descriptor and provider execution contract
    /// must declare the interface.
    ///
    /// This is a structural declaration lookup, not a runtime
    /// availability check.
    #[must_use]
    fn implements_domain_interface(
        &self,
        domain: &crate::frontend::ast::node::domains::domain::Domain,
        interface: &ProviderInterfaceId,
    ) -> bool {

        self.domain_descriptors()
            .iter()
            .filter(|descriptor| descriptor.domain() == domain)
            .any(|descriptor| {
                descriptor.supports_interface(interface)
                    && self
                        .execution_contract()
                        .interfaces()
                        .iter()
                        .any(|implemented| {
                            implemented.id() == interface
                        })
            })
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validates one domain descriptor independently.
///
/// This helper performs structural validation using the canonical
/// descriptor implementation.
///
/// It does not require a provider implementation.
///
/// It is useful when a domain registry validates descriptors
/// before associating them with providers.
///
/// # Errors
///
/// Returns the canonical structured validation error.
pub fn validate_domain_descriptor(
    descriptor: &DomainDescriptor,
) -> ZurResult<()> {

    descriptor.validate().map_err(|error| {
        error
            .with_component(COMPONENT)
            .with_operation("validate_domain_descriptor")
    })
}

/// Checks whether a provider contract implements an interface
/// declared by a domain descriptor.
///
/// This performs exact interface identity matching.
///
/// It does not perform version negotiation, capability evaluation,
/// resource analysis, provider selection or execution.
///
/// Version compatibility and operational compatibility remain
/// the responsibility of their canonical compatibility authorities.
///
/// # Errors
///
/// Returns an error if the domain descriptor is structurally
/// invalid or the provider contract does not implement any
/// declared domain interface.
pub fn validate_domain_interface_implementation(
    domain: &DomainDescriptor,
    provider: &ProviderContract,
) -> ZurResult<()> {

    validate_domain_descriptor(domain)?;

    validate_provider_contract(provider)?;

    let implemented =
        provider
            .interfaces()
            .iter()
            .map(|interface| interface.id())
            .collect::<std::collections::BTreeSet<_>>();

    if !domain
        .interfaces()
        .iter()
        .any(|interface| implemented.contains(interface))
    {
        return Err(
            ZurError::compatibility(
                format!(
                    "provider {} does not implement an interface \
                     declared by domain {}",
                    provider.provider_id(),
                    domain.domain().qualified_name(),
                ),
            )
            .with_component(COMPONENT)
            .with_operation(
                "validate_domain_interface_implementation",
            )
            .with_provider(
                provider.provider_id().to_string(),
            ),
        );
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::frontend::ast::node::domains::domain::Domain;

    use crate::zur::contracts::provider::{
        ProviderInterfaceId,
    };

    use crate::zur::version::{
        ServiceContractVersion,
        Version,
    };

    fn domain_descriptor(
        namespace: &str,
        name: &str,
        interface_name: &str,
    ) -> DomainDescriptor {

        let mut descriptor = DomainDescriptor::new(
            Domain::new(namespace, name)
                .expect("test domain must be valid"),

            ServiceContractVersion::new(
                Version::new(1, 0, 0),
            ),

            format!("{name} test domain"),
        )
        .expect("test descriptor must be valid");

        descriptor
            .insert_interface(
                ProviderInterfaceId::new(interface_name)
                    .expect("test interface must be valid"),
            )
            .expect("test interface must be unique");

        descriptor
    }

    #[test]
    fn validates_individual_domain_descriptors() {

        let descriptor = domain_descriptor(
            "zamani",
            "classical",
            "zamani.classical.execution",
        );

        assert!(
            validate_domain_descriptor(&descriptor).is_ok()
        );
    }

    #[test]
    fn accepts_open_world_domain_namespaces() {

        let descriptor = domain_descriptor(
            "future",
            "photonics",
            "future.photonics.execution",
        );

        assert_eq!(
            descriptor.domain().qualified_name(),
            "future.photonics",
        );
    }

    #[test]
    fn supports_multiple_independent_domain_descriptors() {

        let domains = vec![
            domain_descriptor(
                "zamani",
                "classical",
                "zamani.classical.execution",
            ),

            domain_descriptor(
                "zamani",
                "quantum",
                "zamani.quantum.execution",
            ),

            domain_descriptor(
                "future",
                "photonics",
                "future.photonics.execution",
            ),
        ];

        assert_eq!(domains.len(), 3);

        assert_ne!(
            domains[0].domain(),
            domains[1].domain(),
        );

        assert_ne!(
            domains[1].domain(),
            domains[2].domain(),
        );
    }

    #[test]
    fn domain_descriptors_preserve_interface_identity() {

        let descriptor = domain_descriptor(
            "zamani",
            "quantum",
            "zamani.quantum.execution",
        );

        let interface = ProviderInterfaceId::new(
            "zamani.quantum.execution",
        )
        .unwrap();

        assert!(
            descriptor.supports_interface(&interface)
        );
    }

    #[test]
    fn domain_interface_lookup_is_exact() {

        let descriptor = domain_descriptor(
            "zamani",
            "classical",
            "zamani.classical.execution",
        );

        let different_interface = ProviderInterfaceId::new(
            "zamani.quantum.execution",
        )
        .unwrap();

        assert!(
            !descriptor.supports_interface(&different_interface)
        );
    }

    #[test]
    fn future_domain_identity_is_not_a_closed_enum() {

        let descriptor = domain_descriptor(
            "future",
            "new_architecture",
            "future.new_architecture.execution",
        );

        assert_eq!(
            descriptor.domain().qualified_name(),
            "future.new_architecture",
        );
    }
}
