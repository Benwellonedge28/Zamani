
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Unified Runtime — Nano Domain Integration
//!
//! Path: src/zur/nano.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provides the ZUR integration boundary for nano-oriented computation.
//!
//! This module connects canonical nano-domain identities and interfaces
//! to independently implemented ZUR domain providers.
//!
//! It does not implement nano physics, agent semantics, molecular
//! simulation, hardware discovery, resource allocation, scheduling,
//! device control, or a second nano execution engine.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Validated nano-domain provider bindings.
//! - Exact domain/interface integration checks.
//! - Cancellation-aware provider invocation forwarding.
//! - Runtime-facing nano integration documentation.
//!
//! EXISTING AUTHORITIES
//!
//! - frontend AST owns canonical Domain identity.
//! - grammar/nano owns nano source syntax.
//! - frontend semantic analysis owns nano meaning and validation.
//! - src/nano owns nano-domain computation types and algorithms.
//! - zur/domains/descriptor owns runtime-facing domain descriptors.
//! - zur/domains/provider owns DomainProvider.
//! - zur/contracts/provider owns provider invocation contracts.
//! - stdlib/context owns OperationContext and cancellation.
//! - zur/error owns structured operational errors.
//! - quantum::ir owns quantum computation representation.
//! - quantum::hardware owns quantum hardware integration.
//! - hybrid owns cross-domain composition and planning.
//! - distributed owns distributed computation.
//!
//! ARCHITECTURAL INVARIANTS
//!
//! Provider registration is not provider readiness.
//! Provider readiness is not capability availability.
//! Capability availability is not resource reservation.
//! Resource reservation is not execution.
//! Submission is not successful completion.
//!
//! A successful synchronous provider invocation represents the
//! provider's declared completed-operation contract. It does not,
//! by itself, prove physical-world effects beyond the provider's
//! independently established evidence.
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
//! No arbitrary nano-agent, atom, molecule, device, or resource limits.
//!
//! SCALABILITY
//!
//! This module introduces no fixed maximum for:
//!
//! - Nano-domain providers.
//! - Nano-domain identities.
//! - Provider interfaces.
//! - Operations.
//! - Agents.
//! - Atoms.
//! - Molecules.
//! - Materials.
//! - Devices.
//! - Resources.
//! - Execution sessions.
//!
//! Actual limits arise from the implementation, address space,
//! provider capabilities, declared policies and available resources.
//!
//! Literal infinite physical computation is not promised.

// ============================================================================
// IMPORTS
// ============================================================================

use crate::frontend::ast::node::domains::domain::Domain;

use crate::stdlib::context::OperationContext;

use crate::zur::contracts::provider::{
    ProviderInterfaceId,
    ProviderOperationRequest,
    ProviderOperationResponse,
    ProviderSubmission,
    ZurProviderFuture,
};

use crate::zur::domains::provider::DomainProvider;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable diagnostic component identity for this integration module.
pub const COMPONENT: &str = "zur.nano";

// ============================================================================
// VALIDATED PROVIDER BINDING
// ============================================================================

/// A validated association between a canonical nano-domain identity,
/// one provider interface, and an implementation of `DomainProvider`.
///
/// The binding borrows the provider and canonical domain identity.
/// It does not own, initialize, register, select, or shut down the provider.
///
/// The caller must obtain the domain identity from the canonical
/// frontend/semantic authority. This module does not reinterpret source
/// annotations or invent a second domain-identity model.
///
/// A binding proves structural declaration compatibility only.
/// It does not prove provider availability, resource availability,
/// capability availability, feasibility, authorization, or reservation.
pub struct NanoProviderBinding<'a> {
    provider: &'a dyn DomainProvider,
    domain: &'a Domain,
    interface: &'a ProviderInterfaceId,
}

impl<'a> NanoProviderBinding<'a> {
    /// Creates a validated binding to a nano-domain provider interface.
    ///
    /// The supplied `domain` must be the canonical nano-domain identity
    /// resolved by semantic analysis.
    ///
    /// Structural validation checks:
    ///
    /// - The canonical domain identity is well formed.
    /// - The provider's domain integration is structurally valid.
    /// - The provider declares the exact supplied domain.
    /// - The provider implements the exact supplied interface for
    ///   that domain.
    ///
    /// This method does not initialize the provider, discover hardware,
    /// evaluate resource feasibility, reserve resources, or execute work.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the domain or provider integration
    /// is invalid or the requested interface is not implemented.
    pub fn new(
        provider: &'a dyn DomainProvider,
        domain: &'a Domain,
        interface: &'a ProviderInterfaceId,
    ) -> ZurResult<Self> {
        domain.validate().map_err(|error| {
            ZurError::invalid_input(
                format!("invalid nano-domain identity: {error}"),
            )
            .with_component(COMPONENT)
            .with_operation("create_provider_binding")
            .with_source(error)
        })?;

        provider
            .validate_domain_integration()
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation("validate_provider_integration")
            })?;

        if !provider.declares_domain(domain) {
            return Err(
                ZurError::compatibility(
                    format!(
                        "provider does not declare domain {}",
                        domain.qualified_name(),
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("create_provider_binding")
                .with_provider(
                    provider.descriptor().id().to_string(),
                ),
            );
        }

        if !provider.implements_domain_interface(
            domain,
            interface,
        ) {
            return Err(
                ZurError::compatibility(
                    format!(
                        "provider does not implement interface {} \
                         for domain {}",
                        interface,
                        domain.qualified_name(),
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("create_provider_binding")
                .with_provider(
                    provider.descriptor().id().to_string(),
                ),
            );
        }

        Ok(Self {
            provider,
            domain,
            interface,
        })
    }

    /// Returns the canonical nano-domain identity associated with
    /// this binding.
    #[must_use]
    pub fn domain(&self) -> &Domain {
        self.domain
    }

    /// Returns the provider interface associated with this binding.
    #[must_use]
    pub fn interface(&self) -> &ProviderInterfaceId {
        self.interface
    }

    /// Returns the bound domain provider.
    ///
    /// The returned provider remains owned and lifecycle-managed
    /// by its original provider authority.
    #[must_use]
    pub fn provider(&self) -> &'a dyn DomainProvider {
        self.provider
    }

    /// Returns the provider's stable execution contract.
    ///
    /// The contract describes declared operations and interfaces.
    /// It does not prove current physical or operational availability.
    #[must_use]
    pub fn execution_contract(
        &self,
    ) -> &crate::zur::contracts::provider::ProviderContract {
        self.provider.execution_contract()
    }

    /// Invokes a typed provider operation.
    ///
    /// The provider remains responsible for validating the operation
    /// identifier, payload contract, authorization, resource requirements,
    /// and operation-specific execution semantics.
    ///
    /// The operation context is passed unchanged to the provider.
    ///
    /// Cancellation is checked before forwarding the request.
    ///
    /// A successful response follows the provider's synchronous
    /// completed-operation contract. It is not a submission receipt.
    ///
    /// # Errors
    ///
    /// Returns a cancellation error when the operation context has
    /// already been cancelled.
    ///
    /// Propagates the provider's structured execution error.
    pub fn invoke<'b>(
        &'b self,
        request: ProviderOperationRequest<'b>,
        context: &'b OperationContext,
    ) -> ZurProviderFuture<'b, ProviderOperationResponse> {
        if context.is_cancelled() {
            return Box::pin(async {
                Err(
                    ZurError::cancelled(
                        "nano provider invocation was cancelled before dispatch",
                    )
                    .with_component(COMPONENT)
                    .with_operation("invoke"),
                )
            });
        }

        self.provider.invoke(request, context)
    }

    /// Submits a provider-managed asynchronous operation.
    ///
    /// A successful result is a submission receipt only.
    /// It does not establish that the operation has started or completed.
    ///
    /// The caller must use the provider's canonical observation,
    /// session, or completion contract to determine the actual outcome.
    ///
    /// # Errors
    ///
    /// Returns a cancellation error when the operation context has
    /// already been cancelled.
    ///
    /// Propagates the provider's structured submission error.
    pub fn submit<'b>(
        &'b self,
        request: ProviderOperationRequest<'b>,
        context: &'b OperationContext,
    ) -> ZurProviderFuture<'b, ProviderSubmission> {
        if context.is_cancelled() {
            return Box::pin(async {
                Err(
                    ZurError::cancelled(
                        "nano provider submission was cancelled before dispatch",
                    )
                    .with_component(COMPONENT)
                    .with_operation("submit"),
                )
            });
        }

        self.provider.submit(request, context)
    }
}

// ============================================================================
// INTEGRATION VALIDATION
// ============================================================================

/// Validates that a provider declares and implements a supplied
/// canonical nano-domain interface.
///
/// This is a structural contract check only.
///
/// It does not establish:
///
/// - Provider initialization.
/// - Provider readiness.
/// - Physical device availability.
/// - Capability availability.
/// - Resource feasibility.
/// - Resource reservation.
/// - Security authorization.
/// - Successful execution.
///
/// The domain identity must already have been resolved by the
/// canonical semantic authority.
///
/// # Errors
///
/// Returns a structured compatibility or validation error when
/// the provider does not satisfy the declared integration contract.
pub fn validate_nano_provider(
    provider: &dyn DomainProvider,
    domain: &Domain,
    interface: &ProviderInterfaceId,
) -> ZurResult<()> {
    let _binding = NanoProviderBinding::new(
        provider,
        domain,
        interface,
    )?;

    Ok(())
}

// ============================================================================
// ARCHITECTURAL RE-EXPORTS
// ============================================================================

/// Canonical provider binding type used by ZUR nano integrations.
pub type NanoExecutionBinding<'a> = NanoProviderBinding<'a>;

// ============================================================================
// TESTING AND CONFORMANCE NOTES
// ============================================================================
//
// Provider conformance tests belong in:
//     zur/testing/provider_harness.rs
//     zur/testing/conformance.rs
//
// Nano-domain execution tests belong with the canonical nano-domain
// implementation and its provider adapter.
//
// Required integration tests:
//
// 1. A valid nano domain and implemented interface can be bound.
// 2. An undeclared domain is rejected.
// 3. An undeclared interface is rejected.
// 4. Invalid domain identity is rejected.
// 5. An invalid provider domain contract is rejected.
// 6. A pre-cancelled operation is not forwarded.
// 7. Provider invocation errors are propagated unchanged.
// 8. Provider submission is not reported as completion.
// 9. Provider unavailability is not reported as success.
// 10. Resource reservation is not inferred from provider binding.
// 11. Future nano-domain extensions use the same provider contract.
// 12. No runtime global state is created.
// 13. No unsafe code is introduced.
// 14. No arbitrary domain or resource ceilings are introduced.
//
// This module intentionally contains no fake nano-agent simulator,
// no physical actuation implementation, no molecular physics model,
// no second IR, no resource allocator, and no execution scheduler.
