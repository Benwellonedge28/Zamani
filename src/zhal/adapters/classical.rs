
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani ZHAL — Classical Hardware Adapter
//!
//! Canonical path:
//!     src/zhal/adapters/classical.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the classical-computing integration boundary between
//! Zamani's existing provider and capability contracts and the
//! universal ZHAL hardware abstraction architecture.
//!
//! This module adapts existing provider observations for classical
//! hardware consumers without introducing another provider,
//! capability, semantic, IR, resource-allocation, or runtime authority.
//!
//! ## Architectural authorities
//!
//! Provider identity and provider lifecycle:
//!     crate::stdlib::provider
//!
//! Capability identities, availability and evidence:
//!     crate::stdlib::capability
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Classical computation:
//!     crate::classical
//!
//! Classical intermediate representation:
//!     crate::classical::zcir
//!
//! Classical execution planning:
//!     crate::classical::execution
//!
//! Backend compilation and target realization:
//!     crate::backend
//!
//! Host-platform observations:
//!     crate::zur::platforms
//!
//! Physical resource allocation and execution:
//!     crate::runtime
//!
//! Universal hardware abstraction:
//!     crate::zhal
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The classical-facing provider adapter.
//! - Classical capability observation.
//! - Classical capability assessment results.
//! - The bridge between provider evidence and classical consumers.
//!
//! It does not own:
//!
//! - Provider identity or provider lifecycle.
//! - Capability identity or capability evidence.
//! - Hardware device identity.
//! - Hardware discovery implementation.
//! - CPU architecture definitions.
//! - CPU instruction selection.
//! - Classical IR or ZCIR.
//! - Backend registration or code generation.
//! - Physical resource allocation.
//! - Thread or process scheduling.
//! - Memory management.
//! - Runtime execution.
//! - Host-platform discovery.
//! - Source-language semantic requirements.
//!
//! ## POCO-REAF
//!
//! Classical hardware providers are open-world implementations.
//!
//! The adapter does not enumerate a closed set of:
//!
//! - Processor vendors.
//! - Processor architectures.
//! - CPU cores.
//! - Processing units.
//! - Memory devices.
//! - Cache levels.
//! - Accelerator devices.
//! - Instruction sets.
//! - Hardware extensions.
//! - Classical operations.
//! - Classical capabilities.
//!
//! No arbitrary universal hardware-capacity ceilings are introduced.
//!
//! Hardware capabilities are supplied through provider observations.
//!
//! Actual feasibility depends on the requirements, evidence,
//! target capabilities, available resources and applicable policies.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Capability semantics
//!
//! The adapter preserves the canonical capability states:
//!
//! - Available.
//! - Unavailable.
//! - Conditional.
//! - Unknown.
//!
//! Missing evidence is Unknown, not Unavailable.
//!
//! A declared provider capability is not proof of current availability.
//!
//! An available capability is not proof that physical resources
//! have been reserved.
//!
//! ## Integration
//!
//! This module consumes the canonical standard-library provider.
//!
//! The universal ZHAL provider and device contracts must be integrated
//! through the ZHAL core when those contracts are implemented.
//!
//! This adapter must not independently implement those missing contracts.
//!
//! ## Thread safety
//!
//! The adapter stores its provider through Arc.
//!
//! Provider implementations must satisfy the existing Provider
//! Send + Sync contract.
//!
//! The adapter does not introduce mutable global state.
//!
//! ## Cancellation
//!
//! Cancellation is delegated to OperationContext and the provider.
//!
//! This adapter does not create another cancellation mechanism.
//!
//! ## Freeze criteria
//!
//! - Canonical provider contracts are reused.
//! - Capability states are preserved.
//! - Unknown evidence is never treated as available.
//! - Provider readiness is evaluated independently.
//! - Cancellation is propagated.
//! - No hardware-capacity ceiling is introduced.
//! - No unsafe Rust is used.
//! - Public API and integration tests pass.

use std::sync::Arc;

use crate::stdlib::capability::{
    CapabilityId,
    CapabilitySnapshot,
    CapabilityStatus,
};

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderStatus,
};

// ============================================================================
// CLASSICAL PROVIDER ADAPTER
// ============================================================================

/// Adapts an existing Zamani provider for classical ZHAL consumers.
///
/// This is a non-owning architectural adapter with shared ownership
/// of the underlying provider.
///
/// The provider remains the authority for its identity, lifecycle,
/// capability observations and implementation behavior.
///
/// The adapter does not create a second provider implementation.
///
/// # Type parameters
///
/// `P` is any implementation of the canonical standard-library
/// [`Provider`] contract.
///
/// # Examples
///
/// A concrete provider can be adapted as follows:
///
/// ```ignore
/// let adapter = ClassicalProviderAdapter::new(
///     Arc::clone(&provider),
/// );
/// ```
///
/// The provider may represent a native CPU implementation,
/// an emulated processor, a virtual machine, or another compatible
/// classical computation service.
///
/// # Scalability
///
/// The adapter introduces no fixed provider count, device count,
/// processor count or capability count.
pub struct ClassicalProviderAdapter<P>
where
    P: Provider + ?Sized,
{
    provider: Arc<P>,
}

impl<P> ClassicalProviderAdapter<P>
where
    P: Provider + ?Sized,
{
    /// Constructs a classical-facing adapter around an existing provider.
    ///
    /// The supplied provider retains ownership of its implementation
    /// and all provider-specific resources.
    ///
    /// This constructor does not initialize the provider, register it,
    /// authenticate it, or establish hardware availability.
    #[must_use]
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Returns the underlying provider.
    ///
    /// This is a borrowed reference. Ownership remains shared through
    /// the adapter's Arc.
    #[must_use]
    pub fn provider(&self) -> &P {
        self.provider.as_ref()
    }

    /// Returns the provider's immutable descriptor.
    ///
    /// The descriptor contains declared metadata, not live evidence
    /// of hardware availability.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        self.provider.descriptor()
    }

    /// Returns the provider's current observed status.
    ///
    /// This is an observation and may become stale immediately.
    #[must_use]
    pub fn status(&self) -> ProviderStatus {
        self.provider.status()
    }

    /// Returns whether the provider may currently accept operations.
    ///
    /// This checks the canonical provider lifecycle status only.
    ///
    /// It does not establish that a particular capability exists
    /// or that sufficient resources are available.
    #[must_use]
    pub fn may_accept_operations(&self) -> bool {
        self.status().may_accept_operations()
    }

    /// Returns the provider's shared ownership handle.
    ///
    /// This is useful when another authorized subsystem needs to
    /// retain the same provider implementation.
    #[must_use]
    pub fn provider_handle(&self) -> Arc<P> {
        Arc::clone(&self.provider)
    }

    /// Obtains a fresh capability snapshot from the provider.
    ///
    /// The snapshot is an observation, not a resource reservation.
    ///
    /// The provider is responsible for obtaining and reporting
    /// evidence according to its implementation contract.
    ///
    /// # Cancellation
    ///
    /// Cancellation is checked before requesting the observation
    /// and is propagated through the supplied OperationContext.
    ///
    /// # Errors
    ///
    /// Returns the canonical standard-library error when:
    ///
    /// - The operation has already been cancelled.
    /// - The provider cannot obtain its capability snapshot.
    ///
    /// Provider failures are propagated without being converted
    /// into successful observations.
    pub fn observe_capabilities<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        Box::pin(async move {
            context.check_cancelled()?;

            self.provider
                .capability_snapshot(context)
                .await
        })
    }

    /// Assesses a collection of required classical capabilities.
    ///
    /// This method obtains a fresh provider snapshot and evaluates
    /// each supplied capability identity against that snapshot.
    ///
    /// The identities must originate from the appropriate upstream
    /// semantic or execution requirement authority.
    ///
    /// This method does not independently interpret source-language
    /// requirements, constraints, preferences or hints.
    ///
    /// # Important
    ///
    /// A successful capability assessment does not guarantee:
    ///
    /// - Physical resource reservation.
    /// - Memory availability at execution time.
    /// - Successful backend compilation.
    /// - Successful operation submission.
    /// - Successful execution.
    ///
    /// Those decisions belong to their respective authorities.
    ///
    /// # Cancellation
    ///
    /// The operation observes the supplied cancellation context.
    pub fn assess_capabilities<'a>(
        &'a self,
        context: &'a OperationContext,
        requirements: &'a [CapabilityId],
    ) -> ProviderFuture<'a, ClassicalCapabilityReport> {
        Box::pin(async move {
            context.check_cancelled()?;

            let snapshot = self
                .provider
                .capability_snapshot(context)
                .await?;

            let provider_status = self.provider.status();

            Ok(ClassicalCapabilityReport::from_snapshot(
                provider_status,
                &snapshot,
                requirements.iter().cloned(),
            ))
        })
    }
}

// ============================================================================
// INDIVIDUAL CAPABILITY ASSESSMENT
// ============================================================================

/// Result of assessing one required classical capability.
///
/// This structure preserves the canonical capability identity
/// and its exact reported availability state.
///
/// It does not reinterpret conditional or unknown states as success.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassicalCapabilityAssessment {
    capability: CapabilityId,
    status: CapabilityStatus,
}

impl ClassicalCapabilityAssessment {
    /// Creates a capability assessment.
    #[must_use]
    pub fn new(
        capability: CapabilityId,
        status: CapabilityStatus,
    ) -> Self {
        Self {
            capability,
            status,
        }
    }

    /// Returns the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the provider-reported capability status.
    #[must_use]
    pub const fn status(&self) -> CapabilityStatus {
        self.status
    }

    /// Returns whether unconditional capability availability
    /// has been established.
    ///
    /// Conditional, unavailable and unknown states return false.
    #[must_use]
    pub const fn is_established_available(&self) -> bool {
        self.status.is_available()
    }

    /// Returns whether the capability is explicitly unavailable.
    #[must_use]
    pub const fn is_unavailable(&self) -> bool {
        self.status.is_unavailable()
    }

    /// Returns whether capability availability is conditional.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        self.status.is_conditional()
    }

    /// Returns whether capability availability is unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        self.status.is_unknown()
    }
}

// ============================================================================
// CLASSICAL CAPABILITY REPORT
// ============================================================================

/// Evidence-backed assessment of classical provider capabilities.
///
/// The report contains:
///
/// - The provider's observed status.
/// - The original canonical capability snapshot.
/// - Individual assessments of requested capabilities.
///
/// The original snapshot is retained so that callers do not lose
/// the evidence supplied by the canonical capability authority.
///
/// This report is observational. It is not a feasibility certificate,
/// resource reservation or execution guarantee.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassicalCapabilityReport {
    provider_status: ProviderStatus,
    snapshot: CapabilitySnapshot,
    assessments: Vec<ClassicalCapabilityAssessment>,
}

impl ClassicalCapabilityReport {
    /// Constructs a report from an existing provider snapshot.
    ///
    /// This is useful for deterministic testing and for integrating
    /// capability observations obtained through another authorized
    /// observation path.
    ///
    /// The supplied requirements are preserved in iteration order.
    ///
    /// No arbitrary requirement-count ceiling is imposed.
    #[must_use]
    pub fn from_snapshot(
        provider_status: ProviderStatus,
        snapshot: &CapabilitySnapshot,
        requirements: impl IntoIterator<Item = CapabilityId>,
    ) -> Self {
        let assessments = requirements
            .into_iter()
            .map(|capability| {
                let status = snapshot.status(&capability);

                ClassicalCapabilityAssessment::new(
                    capability,
                    status,
                )
            })
            .collect();

        Self {
            provider_status,
            snapshot: snapshot.clone(),
            assessments,
        }
    }

    /// Returns the observed provider status.
    #[must_use]
    pub const fn provider_status(&self) -> ProviderStatus {
        self.provider_status
    }

    /// Returns the original capability snapshot.
    ///
    /// The snapshot remains the canonical evidence representation.
    #[must_use]
    pub fn snapshot(&self) -> &CapabilitySnapshot {
        &self.snapshot
    }

    /// Returns the individual capability assessments.
    ///
    /// The original requirement order is preserved.
    #[must_use]
    pub fn assessments(&self) -> &[ClassicalCapabilityAssessment] {
        &self.assessments
    }

    /// Returns the number of assessed requirements.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.assessments.len()
    }

    /// Returns whether the report contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assessments.is_empty()
    }

    /// Returns whether every requested capability is unconditionally
    /// available according to the supplied snapshot.
    ///
    /// This does not evaluate provider readiness or resource availability.
    #[must_use]
    pub fn all_capabilities_available(&self) -> bool {
        self.assessments
            .iter()
            .all(ClassicalCapabilityAssessment::is_established_available)
    }

    /// Returns whether the provider is currently in a state that
    /// may accept operations.
    #[must_use]
    pub const fn provider_may_accept_operations(&self) -> bool {
        self.provider_status.may_accept_operations()
    }

    /// Returns whether the provider capability gate is satisfied.
    ///
    /// This requires:
    ///
    /// 1. The provider status permits operations.
    /// 2. Every requested capability has established availability.
    ///
    /// This is NOT a complete execution-feasibility assessment.
    ///
    /// Resource constraints, authorization, backend compatibility,
    /// semantic requirements and runtime allocation remain outside
    /// this adapter.
    #[must_use]
    pub fn provider_capability_gate_satisfied(&self) -> bool {
        self.provider_may_accept_operations()
            && self.all_capabilities_available()
    }

    /// Returns whether any required capability is explicitly unavailable.
    #[must_use]
    pub fn has_unavailable_capability(&self) -> bool {
        self.assessments
            .iter()
            .any(ClassicalCapabilityAssessment::is_unavailable)
    }

    /// Returns whether any required capability has conditional availability.
    #[must_use]
    pub fn has_conditional_capability(&self) -> bool {
        self.assessments
            .iter()
            .any(ClassicalCapabilityAssessment::is_conditional)
    }

    /// Returns whether any required capability has unknown availability.
    #[must_use]
    pub fn has_unknown_capability(&self) -> bool {
        self.assessments
            .iter()
            .any(ClassicalCapabilityAssessment::is_unknown)
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
        CapabilityKind,
        CapabilityState,
    };

    fn capability(id: &str) -> Capability {
        Capability::new(
            CapabilityId::new(id)
                .expect("valid capability identity"),
            CapabilityKind::new("computation")
                .expect("valid capability kind"),
        )
    }

    fn snapshot_with(
        id: &str,
        status: CapabilityStatus,
    ) -> CapabilitySnapshot {
        let mut snapshot = CapabilitySnapshot::new();

        snapshot
            .insert(CapabilityState::new(
                capability(id),
                status,
            ))
            .expect("unique capability");

        snapshot
    }

    #[test]
    fn available_classical_capability_is_preserved() {
        let snapshot = snapshot_with(
            "classical.integer",
            CapabilityStatus::Available,
        );

        let requirement = CapabilityId::new("classical.integer")
            .expect("valid requirement");

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(report.all_capabilities_available());
        assert!(report.provider_capability_gate_satisfied());
    }

    #[test]
    fn unavailable_capability_is_not_treated_as_available() {
        let snapshot = snapshot_with(
            "classical.vector",
            CapabilityStatus::Unavailable,
        );

        let requirement = CapabilityId::new("classical.vector")
            .expect("valid requirement");

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(!report.all_capabilities_available());
        assert!(!report.provider_capability_gate_satisfied());
        assert!(report.has_unavailable_capability());
    }

    #[test]
    fn unknown_capability_is_not_treated_as_available() {
        let snapshot = CapabilitySnapshot::new();

        let requirement = CapabilityId::new("classical.future_operation")
            .expect("valid requirement");

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(report.has_unknown_capability());
        assert!(!report.provider_capability_gate_satisfied());
    }

    #[test]
    fn conditional_capability_is_not_silently_accepted() {
        let snapshot = snapshot_with(
            "classical.parallel",
            CapabilityStatus::Conditional,
        );

        let requirement = CapabilityId::new("classical.parallel")
            .expect("valid requirement");

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [requirement],
        );

        assert!(report.has_conditional_capability());
        assert!(!report.provider_capability_gate_satisfied());
    }

    #[test]
    fn unavailable_provider_cannot_pass_capability_gate() {
        let snapshot = snapshot_with(
            "classical.integer",
            CapabilityStatus::Available,
        );

        let requirement = CapabilityId::new("classical.integer")
            .expect("valid requirement");

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Unavailable,
            &snapshot,
            [requirement],
        );

        assert!(report.all_capabilities_available());
        assert!(!report.provider_may_accept_operations());
        assert!(!report.provider_capability_gate_satisfied());
    }

    #[test]
    fn degraded_provider_can_accept_operations_when_capabilities_hold() {
        let snapshot = snapshot_with(
            "classical.integer",
            CapabilityStatus::Available,
        );

        let requirement = CapabilityId::new("classical.integer")
            .expect("valid requirement");

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Degraded,
            &snapshot,
            [requirement],
        );

        assert!(report.provider_capability_gate_satisfied());
    }

    #[test]
    fn empty_requirement_collection_does_not_bypass_provider_status() {
        let snapshot = CapabilitySnapshot::new();

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Failed,
            &snapshot,
            std::iter::empty(),
        );

        assert!(report.is_empty());
        assert!(!report.provider_capability_gate_satisfied());
    }

    #[test]
    fn multiple_requirements_preserve_input_order() {
        let mut snapshot = CapabilitySnapshot::new();

        for id in [
            "classical.integer",
            "classical.vector",
            "classical.parallel",
        ] {
            snapshot
                .insert(CapabilityState::new(
                    capability(id),
                    CapabilityStatus::Available,
                ))
                .expect("unique capability");
        }

        let requirements = [
            CapabilityId::new("classical.parallel")
                .expect("valid ID"),
            CapabilityId::new("classical.integer")
                .expect("valid ID"),
        ];

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            requirements,
        );

        assert_eq!(report.requirement_count(), 2);

        assert_eq!(
            report.assessments()[0].capability().as_str(),
            "classical.parallel",
        );

        assert_eq!(
            report.assessments()[1].capability().as_str(),
            "classical.integer",
        );
    }

    #[test]
    fn future_capability_namespaces_are_supported() {
        let id = CapabilityId::new(
            "future.architecture.specialized_classical_computation",
        )
        .expect("open-world capability identity");

        let snapshot = snapshot_with(
            id.as_str(),
            CapabilityStatus::Available,
        );

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            [id],
        );

        assert!(report.provider_capability_gate_satisfied());
    }

    #[test]
    fn snapshot_is_preserved_in_report() {
        let snapshot = snapshot_with(
            "classical.integer",
            CapabilityStatus::Available,
        );

        let report = ClassicalCapabilityReport::from_snapshot(
            ProviderStatus::Ready,
            &snapshot,
            std::iter::empty(),
        );

        assert_eq!(report.snapshot(), &snapshot);
    }
}
