
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Operation Contracts
//!
//! Path: `src/zhal/operations/contract.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines the hardware-facing metadata and compatibility contract
//! for operations exposed through the Zamani Universal Hardware
//! Abstraction Layer.
//!
//! An operation contract describes the requirements and lifecycle
//! semantics of an operation independently of its physical hardware
//! realization.
//!
//! Supported domains are open-ended and include:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - AI and tensor acceleration.
//! - Hybrid computation.
//! - Distributed computation.
//! - Embedded computation.
//! - Networking and interconnects.
//! - Neuromorphic computation.
//! - Photonic computation.
//! - Analog computation.
//! - Future computational technologies.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Hardware operation contract composition.
//! - Provider API compatibility requirements.
//! - Required capability references.
//! - Submission semantics.
//! - Completion semantics.
//! - Cancellation semantics.
//! - Retry and idempotency semantics.
//! - Provider contract compatibility validation.
//!
//! This module does NOT own:
//!
//! - Operation identity.
//! - Provider identity.
//! - Request or response contract identity.
//! - Semantic capability identity.
//! - Version representation.
//! - Operation payload types.
//! - Operation submission.
//! - Operation execution.
//! - Operation completion tracking.
//! - Cancellation implementation.
//! - Resource allocation.
//! - Resource reservation.
//! - Scheduling.
//! - Routing.
//! - Hardware discovery.
//! - Device selection.
//! - Domain-specific execution semantics.
//! - Compiler or IR definitions.
//!
//! # Canonical authorities
//!
//! Provider identity and operation descriptors:
//!     crate::stdlib::provider
//!
//! Capability identity and evidence:
//!     crate::stdlib::capability
//!
//! Semantic version representation:
//!     crate::stdlib::version
//!
//! Hardware operation-instance identity:
//!     crate::zhal::identity
//!
//! Hardware operation errors:
//!     crate::zhal::errors
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Submission:
//!     crate::zhal::operations::submission
//!
//! Completion:
//!     crate::zhal::operations::completion
//!
//! Cancellation:
//!     crate::zhal::operations::cancellation
//!
//! Runtime resource management:
//!     crate::zur
//!
//! # Important distinction
//!
//! ProviderOperationId identifies an operation contract.
//!
//! HardwareOperationIdentity identifies an individual operation
//! instance.
//!
//! These identities must never be confused.
//!
//! Similarly, an operation contract version is distinct from:
//!
//! - Zamani language version.
//! - Grammar version.
//! - Cargo package version.
//! - ZHAL API version.
//! - Provider implementation version.
//! - Hardware firmware version.
//!
//! # POCO-REAF
//!
//! Operation contracts are independent of:
//!
//! - Physical processor count.
//! - Memory capacity.
//! - Device count.
//! - Qubit count.
//! - Accelerator count.
//! - Distributed participant count.
//! - Hardware vendor.
//! - Hardware architecture.
//! - Physical location.
//! - Hardware topology.
//!
//! No arbitrary universal hardware ceilings are introduced.
//!
//! Collections grow according to available resources and explicitly
//! configured operational budgets.
//!
//! This does not promise literal infinite computation or storage.
//!
//! # Capability semantics
//!
//! Required capabilities refer to the canonical standard-library
//! CapabilityId.
//!
//! A provider declaration establishes that the provider advertises
//! support for a capability.
//!
//! It does not establish that the capability is currently available.
//!
//! Current availability requires evidence from the provider's
//! CapabilitySnapshot.
//!
//! Resource availability and feasibility remain separate decisions.
//!
//! # Provider compatibility
//!
//! An operation contract may specify:
//!
//! - Any provider API version.
//! - An exact provider API version.
//! - An inclusive or exclusive provider API version range.
//!
//! Version precedence follows the canonical semantic-version
//! implementation.
//!
//! Build metadata does not affect semantic version precedence.
//!
//! # Security
//!
//! Operation contracts are descriptive metadata.
//!
//! They must not contain:
//!
//! - Credentials.
//! - Access tokens.
//! - Private keys.
//! - Authentication headers.
//! - Session secrets.
//! - Physical hardware handles.
//!
//! A compatible operation contract does not establish authorization.
//!
//! Provider registration does not establish trust.
//!
//! # Grammar boundary
//!
//! Relevant source grammar:
//!
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! These grammars describe computational intent.
//!
//! This module describes hardware-facing operation contracts.
//!
//! It does not parse source syntax or reinterpret semantic
//! requirements, capabilities, preferences, constraints or hints.

use core::cmp::Ordering;
use core::fmt;

use std::collections::BTreeSet;

use crate::stdlib::capability::CapabilityId;

use crate::stdlib::provider::{
    ProviderDescriptor,
    ProviderOperationDescriptor,
    ProviderOperationId,
};

use crate::stdlib::version::Version;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Component identity used in structured ZHAL diagnostics.
pub const COMPONENT: &str = "zhal.operations.contract";

/// Operation-contract schema identifier.
///
/// This is independent of the Cargo package version and
/// provider API version.
pub const CONTRACT_SCHEMA_VERSION: &str = "1";

// ============================================================================
// PROVIDER API COMPATIBILITY
// ============================================================================

/// Provider API compatibility requirement.
///
/// This type describes which provider API versions may implement
/// an operation contract.
///
/// Compatibility is evaluated using semantic version precedence.
///
/// Build metadata does not participate in precedence comparison.
///
/// This type does not establish provider trust, readiness,
/// authorization or current capability availability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderApiCompatibility {
    /// Any provider API version is acceptable.
    ///
    /// The caller remains responsible for validating the
    /// operation-specific contract.
    Any,

    /// Only the specified semantic version precedence is acceptable.
    ///
    /// Build metadata is ignored during comparison.
    Exact(Version),

    /// An explicitly bounded semantic version range.
    ///
    /// Either bound may be absent.
    ///
    /// A range with both bounds absent is equivalent to `Any`.
    Range {
        /// Optional minimum provider API version.
        minimum: Option<Version>,

        /// Optional maximum provider API version.
        maximum: Option<Version>,

        /// Whether the minimum boundary is inclusive.
        include_minimum: bool,

        /// Whether the maximum boundary is inclusive.
        include_maximum: bool,
    },
}

impl ProviderApiCompatibility {
    /// Constructs an unrestricted provider API compatibility requirement.
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }

    /// Constructs an exact provider API version requirement.
    #[must_use]
    pub fn exact(version: Version) -> Self {
        Self::Exact(version)
    }

    /// Constructs a bounded provider API compatibility range.
    ///
    /// Bounds are compared using semantic version precedence.
    ///
    /// At least one bound must be supplied.
    ///
    /// When both bounds are supplied, the minimum must not exceed
    /// the maximum.
    ///
    /// An empty range is rejected.
    ///
    /// # Errors
    ///
    /// Returns an InvalidInput error when:
    ///
    /// - Both bounds are absent.
    /// - The minimum exceeds the maximum.
    /// - Both bounds are equal but either endpoint is exclusive.
    pub fn range(
        minimum: Option<Version>,
        maximum: Option<Version>,
        include_minimum: bool,
        include_maximum: bool,
    ) -> ZhalResult<Self> {
        if minimum.is_none() && maximum.is_none() {
            return Err(contract_error(
                ZhalErrorKind::InvalidInput,
                "provider API compatibility range requires at least one bound",
            ));
        }

        if let (Some(minimum), Some(maximum)) =
            (&minimum, &maximum)
        {
            match minimum.precedence_cmp(maximum) {
                Ordering::Greater => {
                    return Err(contract_error(
                        ZhalErrorKind::InvalidInput,
                        "provider API compatibility minimum exceeds maximum",
                    ));
                }

                Ordering::Equal
                    if !include_minimum || !include_maximum =>
                {
                    return Err(contract_error(
                        ZhalErrorKind::InvalidInput,
                        "provider API compatibility range is empty",
                    ));
                }

                Ordering::Less | Ordering::Equal => {}
            }
        }

        Ok(Self::Range {
            minimum,
            maximum,
            include_minimum,
            include_maximum,
        })
    }

    /// Determines whether a provider API version satisfies this
    /// compatibility requirement.
    ///
    /// This is a version check only.
    ///
    /// It does not establish:
    ///
    /// - Provider readiness.
    /// - Provider trust.
    /// - Operation support.
    /// - Capability availability.
    /// - Hardware feasibility.
    /// - Resource availability.
    #[must_use]
    pub fn accepts(
        &self,
        candidate: &Version,
    ) -> bool {
        match self {
            Self::Any => true,

            Self::Exact(expected) => {
                candidate.precedence_cmp(expected)
                    == Ordering::Equal
            }

            Self::Range {
                minimum,
                maximum,
                include_minimum,
                include_maximum,
            } => {
                if let Some(minimum) = minimum {
                    match candidate.precedence_cmp(minimum) {
                        Ordering::Less => return false,

                        Ordering::Equal if !include_minimum => {
                            return false;
                        }

                        Ordering::Greater | Ordering::Equal => {}
                    }
                }

                if let Some(maximum) = maximum {
                    match candidate.precedence_cmp(maximum) {
                        Ordering::Greater => return false,

                        Ordering::Equal if !include_maximum => {
                            return false;
                        }

                        Ordering::Less | Ordering::Equal => {}
                    }
                }

                true
            }
        }
    }

    /// Returns whether the requirement accepts every provider API version.
    #[must_use]
    pub const fn is_unrestricted(&self) -> bool {
        matches!(self, Self::Any)
    }
}

impl Default for ProviderApiCompatibility {
    fn default() -> Self {
        Self::Any
    }
}

// ============================================================================
// OPERATION SUBMISSION SEMANTICS
// ============================================================================

/// Describes the acceptance semantics of operation submission.
///
/// This metadata does not implement submission.
///
/// Actual submission receipts and outcomes belong to
/// `operations::submission`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SubmissionSemantics {
    /// The provider guarantees an atomic acceptance decision.
    ///
    /// A rejected submission has not been accepted for execution
    /// according to the provider's declared contract.
    AtomicAcceptance,

    /// The provider may accept only part of a submitted operation.
    ///
    /// This is relevant to operations containing independently
    /// accepted work units or segments.
    PartialAcceptance,

    /// The final acceptance outcome may be indeterminate.
    ///
    /// This can occur when a remote acknowledgement is lost.
    ///
    /// Callers must not assume that retrying is safe.
    IndeterminateAcceptance,

    /// The provider defines acceptance behavior through its
    /// domain-specific operation contract.
    ProviderDefined,
}

// ============================================================================
// OPERATION COMPLETION SEMANTICS
// ============================================================================

/// Describes how an operation reports completion.
///
/// Actual result delivery and completion tracking belong to
/// `operations::completion`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CompletionSemantics {
    /// One terminal result represents the completed operation.
    SingleTerminalResult,

    /// The operation may produce incremental or streaming results.
    IncrementalResults,

    /// The provider defines completion behavior through a
    /// specialized domain-specific contract.
    ProviderDefined,
}

// ============================================================================
// OPERATION CANCELLATION SEMANTICS
// ============================================================================

/// Describes the cancellation guarantees of an operation.
///
/// This enum describes guarantees only.
///
/// Actual cancellation requests and provider interaction belong
/// to `operations::cancellation`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CancellationSemantics {
    /// Cancellation is not supported.
    Unsupported,

    /// Cancellation is cooperative.
    ///
    /// The provider attempts to stop work after receiving a
    /// cancellation request.
    ///
    /// Physical termination is not guaranteed.
    Cooperative,

    /// Cancellation is best-effort.
    ///
    /// The provider may be unable to stop already committed work.
    BestEffort,

    /// Cancellation is defined by a domain-specific provider contract.
    ProviderDefined,
}

// ============================================================================
// OPERATION RETRY AND IDEMPOTENCY SEMANTICS
// ============================================================================

/// Describes whether repeating an operation is safe.
///
/// Retry safety must never be inferred from an operation name,
/// transport protocol or provider availability.
///
/// The actual provider implementation must satisfy its declared
/// semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RetrySemantics {
    /// Repeating the operation is not guaranteed to be safe.
    ///
    /// Automatic retries must not be assumed safe.
    NotGuaranteed,

    /// The operation is idempotent according to its contract.
    ///
    /// Repeating the same logical operation has the same
    /// externally observable effect as performing it once.
    Idempotent,

    /// Safe retry requires an externally managed idempotency key.
    ///
    /// The key's lifecycle and uniqueness requirements belong
    /// to the submission authority.
    RequiresIdempotencyKey,

    /// Retry behavior is defined by a domain-specific contract.
    ProviderDefined,
}

// ============================================================================
// COMPLETE OPERATION LIFECYCLE
// ============================================================================

/// Immutable lifecycle semantics associated with an operation.
///
/// These values describe the guarantees an implementation advertises.
///
/// They do not execute, submit, cancel, retry or monitor operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct OperationLifecycle {
    submission: SubmissionSemantics,
    completion: CompletionSemantics,
    cancellation: CancellationSemantics,
    retry: RetrySemantics,
}

impl OperationLifecycle {
    /// Constructs explicit operation lifecycle semantics.
    #[must_use]
    pub const fn new(
        submission: SubmissionSemantics,
        completion: CompletionSemantics,
        cancellation: CancellationSemantics,
        retry: RetrySemantics,
    ) -> Self {
        Self {
            submission,
            completion,
            cancellation,
            retry,
        }
    }

    /// Returns conservative lifecycle defaults.
    ///
    /// Unknown behavior is not interpreted as a guarantee.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            submission: SubmissionSemantics::ProviderDefined,
            completion: CompletionSemantics::ProviderDefined,
            cancellation: CancellationSemantics::ProviderDefined,
            retry: RetrySemantics::NotGuaranteed,
        }
    }

    /// Returns submission semantics.
    #[must_use]
    pub const fn submission(&self) -> SubmissionSemantics {
        self.submission
    }

    /// Returns completion semantics.
    #[must_use]
    pub const fn completion(&self) -> CompletionSemantics {
        self.completion
    }

    /// Returns cancellation semantics.
    #[must_use]
    pub const fn cancellation(&self) -> CancellationSemantics {
        self.cancellation
    }

    /// Returns retry semantics.
    #[must_use]
    pub const fn retry(&self) -> RetrySemantics {
        self.retry
    }

    /// Returns whether cancellation is advertised as supported.
    #[must_use]
    pub const fn supports_cancellation(&self) -> bool {
        !matches!(
            self.cancellation,
            CancellationSemantics::Unsupported
        )
    }

    /// Returns whether automatic retry is declared safe without
    /// an external idempotency key.
    #[must_use]
    pub const fn retry_is_intrinsically_safe(&self) -> bool {
        matches!(
            self.retry,
            RetrySemantics::Idempotent
        )
    }
}

impl Default for OperationLifecycle {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// HARDWARE OPERATION CONTRACT
// ============================================================================

/// Complete hardware-facing operation contract.
///
/// This structure composes existing standard-library contracts
/// rather than defining competing operation identities or versions.
///
/// It contains:
///
/// - Canonical operation descriptor.
/// - Provider API compatibility.
/// - Required capability identities.
/// - Operation lifecycle semantics.
///
/// Request and response payloads remain owned by the relevant
/// computational domain.
///
/// The operation identity is the canonical ProviderOperationId.
///
/// An individual submitted operation must use the separate
/// HardwareOperationIdentity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareOperationContract {
    descriptor: ProviderOperationDescriptor,
    provider_compatibility: ProviderApiCompatibility,
    required_capabilities: BTreeSet<CapabilityId>,
    lifecycle: OperationLifecycle,
}

impl HardwareOperationContract {
    /// Creates a hardware operation contract from an existing
    /// standard-library operation descriptor.
    ///
    /// The descriptor is reused without redefining its identity,
    /// version or request/response contract references.
    #[must_use]
    pub fn new(
        descriptor: ProviderOperationDescriptor,
    ) -> Self {
        Self {
            descriptor,
            provider_compatibility:
                ProviderApiCompatibility::default(),
            required_capabilities: BTreeSet::new(),
            lifecycle: OperationLifecycle::default(),
        }
    }

    /// Creates a hardware operation contract using an existing
    /// standard-library provider operation descriptor.
    ///
    /// This is an explicit alias for [`Self::new`].
    #[must_use]
    pub fn from_provider_descriptor(
        descriptor: ProviderOperationDescriptor,
    ) -> Self {
        Self::new(descriptor)
    }

    /// Replaces the provider API compatibility requirement.
    #[must_use]
    pub fn with_provider_compatibility(
        mut self,
        compatibility: ProviderApiCompatibility,
    ) -> Self {
        self.provider_compatibility = compatibility;
        self
    }

    /// Adds a required capability.
    ///
    /// Capability identity is reused from the standard library.
    ///
    /// Duplicate capability requirements are rejected.
    ///
    /// Requirements are stored in deterministic identity order.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the capability is already present.
    pub fn with_required_capability(
        mut self,
        capability: CapabilityId,
    ) -> ZhalResult<Self> {
        if !self.required_capabilities.insert(capability.clone()) {
            return Err(
                contract_error(
                    ZhalErrorKind::InvalidInput,
                    format!(
                        "duplicate required capability: {capability}"
                    ),
                ),
            );
        }

        Ok(self)
    }

    /// Adds several required capabilities.
    ///
    /// This method validates the complete incoming collection
    /// before returning the updated contract.
    ///
    /// If any capability is duplicated either within the incoming
    /// collection or against an existing requirement, the operation
    /// fails without returning a partially updated contract.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when a duplicate is encountered.
    pub fn with_required_capabilities<I>(
        mut self,
        capabilities: I,
    ) -> ZhalResult<Self>
    where
        I: IntoIterator<Item = CapabilityId>,
    {
        let mut incoming = BTreeSet::new();

        for capability in capabilities {
            if self.required_capabilities.contains(&capability)
                || !incoming.insert(capability.clone())
            {
                return Err(
                    contract_error(
                        ZhalErrorKind::InvalidInput,
                        format!(
                            "duplicate required capability: {capability}"
                        ),
                    ),
                );
            }
        }

        self.required_capabilities.extend(incoming);

        Ok(self)
    }

    /// Replaces the operation lifecycle semantics.
    #[must_use]
    pub fn with_lifecycle(
        mut self,
        lifecycle: OperationLifecycle,
    ) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    /// Returns the canonical provider operation descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderOperationDescriptor {
        &self.descriptor
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub fn id(&self) -> &ProviderOperationId {
        self.descriptor.id()
    }

    /// Returns the operation contract version.
    #[must_use]
    pub fn version(&self) -> &Version {
        self.descriptor.version()
    }

    /// Returns the optional request contract identity.
    #[must_use]
    pub fn request_contract(
        &self,
    ) -> Option<&crate::stdlib::provider::ProviderContractId> {
        self.descriptor.request_contract()
    }

    /// Returns the optional response contract identity.
    #[must_use]
    pub fn response_contract(
        &self,
    ) -> Option<&crate::stdlib::provider::ProviderContractId> {
        self.descriptor.response_contract()
    }

    /// Returns provider API compatibility requirements.
    #[must_use]
    pub fn provider_compatibility(
        &self,
    ) -> &ProviderApiCompatibility {
        &self.provider_compatibility
    }

    /// Returns required capabilities in deterministic order.
    pub fn required_capabilities(
        &self,
    ) -> impl Iterator<Item = &CapabilityId> {
        self.required_capabilities.iter()
    }

    /// Returns the number of required capabilities.
    #[must_use]
    pub fn required_capability_count(&self) -> usize {
        self.required_capabilities.len()
    }

    /// Returns whether no additional capabilities are required.
    #[must_use]
    pub fn has_no_required_capabilities(&self) -> bool {
        self.required_capabilities.is_empty()
    }

    /// Returns operation lifecycle semantics.
    #[must_use]
    pub const fn lifecycle(&self) -> &OperationLifecycle {
        &self.lifecycle
    }

    /// Validates this operation contract against a provider descriptor.
    ///
    /// Validation checks:
    ///
    /// 1. Provider API compatibility.
    /// 2. Declaration of the operation identity.
    /// 3. Operation contract version compatibility.
    /// 4. Request contract identity.
    /// 5. Response contract identity.
    /// 6. Declaration of all required capabilities.
    ///
    /// This method validates provider declarations only.
    ///
    /// It does not establish:
    ///
    /// - Provider readiness.
    /// - Provider trust.
    /// - Authorization.
    /// - Current capability availability.
    /// - Device availability.
    /// - Resource availability.
    /// - Hardware feasibility.
    /// - Successful execution.
    ///
    /// Current capability availability must be evaluated using
    /// evidence-backed capability snapshots by the appropriate
    /// compatibility authority.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - VersionMismatch for incompatible provider API versions.
    /// - ContractViolation for an undeclared or inconsistent operation.
    /// - CapabilityUnsupported for a missing required declaration.
    pub fn validate_provider(
        &self,
        provider: &ProviderDescriptor,
    ) -> ZhalResult<()> {
        if !self.provider_compatibility.accepts(
            provider.api_version(),
        ) {
            return Err(
                contract_error(
                    ZhalErrorKind::VersionMismatch,
                    format!(
                        "provider {} API version is incompatible with operation {}",
                        provider.id(),
                        self.id(),
                    ),
                )
                .with_provider(provider.id().clone()),
            );
        }

        let Some(declared) = provider.operation(self.id()) else {
            return Err(
                contract_error(
                    ZhalErrorKind::ContractViolation,
                    format!(
                        "provider {} does not declare operation {}",
                        provider.id(),
                        self.id(),
                    ),
                )
                .with_provider(provider.id().clone()),
            );
        };

        if declared.version().precedence_cmp(self.version())
            != Ordering::Equal
        {
            return Err(
                contract_error(
                    ZhalErrorKind::VersionMismatch,
                    format!(
                        "provider {} exposes an incompatible version of operation {}",
                        provider.id(),
                        self.id(),
                    ),
                )
                .with_provider(provider.id().clone()),
            );
        }

        if declared.request_contract()
            != self.request_contract()
        {
            return Err(
                contract_error(
                    ZhalErrorKind::ContractViolation,
                    format!(
                        "provider {} declares a different request contract for operation {}",
                        provider.id(),
                        self.id(),
                    ),
                )
                .with_provider(provider.id().clone()),
            );
        }

        if declared.response_contract()
            != self.response_contract()
        {
            return Err(
                contract_error(
                    ZhalErrorKind::ContractViolation,
                    format!(
                        "provider {} declares a different response contract for operation {}",
                        provider.id(),
                        self.id(),
                    ),
                )
                .with_provider(provider.id().clone()),
            );
        }

        for capability in &self.required_capabilities {
            if provider.capability(capability).is_none() {
                return Err(
                    contract_error(
                        ZhalErrorKind::CapabilityUnsupported,
                        format!(
                            "provider {} does not declare required capability {}",
                            provider.id(),
                            capability,
                        ),
                    )
                    .with_provider(provider.id().clone()),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a structured operation-contract error.
fn contract_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
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
    };

    use crate::stdlib::provider::{
        ProviderContractId,
        ProviderId,
        ProviderKind,
    };

    fn version(value: &str) -> Version {
        Version::parse(value).expect("valid semantic version")
    }

    fn capability_id(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability identity")
    }

    fn operation_descriptor() -> ProviderOperationDescriptor {
        ProviderOperationDescriptor::from_id(
            "classical.compute",
            version("1.0.0"),
        )
        .expect("valid operation descriptor")
        .with_contracts(
            ProviderContractId::new("classical.compute.request")
                .expect("valid request contract"),
            ProviderContractId::new("classical.compute.response")
                .expect("valid response contract"),
        )
    }

    fn provider_descriptor() -> ProviderDescriptor {
        let mut provider = ProviderDescriptor::new(
            ProviderId::new("test.classical.provider")
                .expect("valid provider identity"),
            ProviderKind::new("classical")
                .expect("valid provider kind"),
            "Test classical provider",
            version("1.0.0"),
            version("1.2.0"),
        )
        .expect("valid provider descriptor");

        provider
            .insert_operation(operation_descriptor())
            .expect("operation declaration succeeds");

        provider
            .insert_capability(
                Capability::new(
                    capability_id("classical.compute"),
                    CapabilityKind::new("computation")
                        .expect("valid capability kind"),
                ),
            )
            .expect("capability declaration succeeds");

        provider
    }

    // ------------------------------------------------------------------------
    // Provider API compatibility
    // ------------------------------------------------------------------------

    #[test]
    fn unrestricted_provider_compatibility_accepts_any_version() {
        let compatibility = ProviderApiCompatibility::any();

        assert!(compatibility.accepts(&version("0.0.1")));
        assert!(compatibility.accepts(&version("999.0.0")));
        assert!(compatibility.is_unrestricted());
    }

    #[test]
    fn exact_provider_compatibility_uses_semver_precedence() {
        let compatibility =
            ProviderApiCompatibility::exact(version("1.2.0"));

        assert!(compatibility.accepts(&version("1.2.0")));

        assert!(!compatibility.accepts(&version("1.2.1")));

        assert!(!compatibility.accepts(&version("1.3.0")));
    }

    #[test]
    fn exact_compatibility_ignores_build_metadata() {
        let compatibility =
            ProviderApiCompatibility::exact(
                version("1.2.0+build.1"),
            );

        assert!(
            compatibility.accepts(
                &version("1.2.0+build.2")
            )
        );
    }

    #[test]
    fn bounded_compatibility_accepts_inclusive_boundaries() {
        let compatibility =
            ProviderApiCompatibility::range(
                Some(version("1.0.0")),
                Some(version("2.0.0")),
                true,
                true,
            )
            .expect("valid range");

        assert!(compatibility.accepts(&version("1.0.0")));
        assert!(compatibility.accepts(&version("1.5.0")));
        assert!(compatibility.accepts(&version("2.0.0")));

        assert!(!compatibility.accepts(&version("0.9.9")));
        assert!(!compatibility.accepts(&version("2.0.1")));
    }

    #[test]
    fn bounded_compatibility_respects_exclusive_boundaries() {
        let compatibility =
            ProviderApiCompatibility::range(
                Some(version("1.0.0")),
                Some(version("2.0.0")),
                false,
                false,
            )
            .expect("valid range");

        assert!(!compatibility.accepts(&version("1.0.0")));
        assert!(compatibility.accepts(&version("1.5.0")));
        assert!(!compatibility.accepts(&version("2.0.0")));
    }

    #[test]
    fn compatibility_supports_one_sided_ranges() {
        let minimum =
            ProviderApiCompatibility::range(
                Some(version("2.0.0")),
                None,
                true,
                false,
            )
            .expect("valid minimum range");

        let maximum =
            ProviderApiCompatibility::range(
                None,
                Some(version("2.0.0")),
                false,
                true,
            )
            .expect("valid maximum range");

        assert!(minimum.accepts(&version("999.0.0")));
        assert!(!minimum.accepts(&version("1.9.9")));

        assert!(maximum.accepts(&version("0.1.0")));
        assert!(!maximum.accepts(&version("2.0.1")));
    }

    #[test]
    fn compatibility_rejects_reversed_ranges() {
        assert!(
            ProviderApiCompatibility::range(
                Some(version("2.0.0")),
                Some(version("1.0.0")),
                true,
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn compatibility_rejects_empty_ranges() {
        assert!(
            ProviderApiCompatibility::range(
                Some(version("1.0.0")),
                Some(version("1.0.0")),
                false,
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn compatibility_rejects_unbounded_ranges() {
        assert!(
            ProviderApiCompatibility::range(
                None,
                None,
                true,
                true,
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Operation lifecycle
    // ------------------------------------------------------------------------

    #[test]
    fn lifecycle_defaults_are_conservative() {
        let lifecycle = OperationLifecycle::default();

        assert_eq!(
            lifecycle.submission(),
            SubmissionSemantics::ProviderDefined,
        );

        assert_eq!(
            lifecycle.completion(),
            CompletionSemantics::ProviderDefined,
        );

        assert_eq!(
            lifecycle.cancellation(),
            CancellationSemantics::ProviderDefined,
        );

        assert_eq!(
            lifecycle.retry(),
            RetrySemantics::NotGuaranteed,
        );

        assert!(!lifecycle.retry_is_intrinsically_safe());
    }

    #[test]
    fn cancellation_support_reflects_declared_semantics() {
        let unsupported = OperationLifecycle::new(
            SubmissionSemantics::AtomicAcceptance,
            CompletionSemantics::SingleTerminalResult,
            CancellationSemantics::Unsupported,
            RetrySemantics::NotGuaranteed,
        );

        let cooperative = OperationLifecycle::new(
            SubmissionSemantics::AtomicAcceptance,
            CompletionSemantics::SingleTerminalResult,
            CancellationSemantics::Cooperative,
            RetrySemantics::Idempotent,
        );

        assert!(!unsupported.supports_cancellation());
        assert!(cooperative.supports_cancellation());
    }

    #[test]
    fn idempotency_is_explicit() {
        let lifecycle = OperationLifecycle::new(
            SubmissionSemantics::AtomicAcceptance,
            CompletionSemantics::SingleTerminalResult,
            CancellationSemantics::Unsupported,
            RetrySemantics::Idempotent,
        );

        assert!(lifecycle.retry_is_intrinsically_safe());
    }

    // ------------------------------------------------------------------------
    // Contract construction
    // ------------------------------------------------------------------------

    #[test]
    fn operation_contract_reuses_standard_library_descriptor() {
        let descriptor = operation_descriptor();

        let contract =
            HardwareOperationContract::new(descriptor.clone());

        assert_eq!(contract.id(), descriptor.id());
        assert_eq!(contract.version(), descriptor.version());

        assert_eq!(
            contract.request_contract(),
            descriptor.request_contract(),
        );

        assert_eq!(
            contract.response_contract(),
            descriptor.response_contract(),
        );
    }

    #[test]
    fn duplicate_required_capabilities_are_rejected() {
        let capability = capability_id("classical.compute");

        let contract = HardwareOperationContract::new(
            operation_descriptor(),
        )
        .with_required_capability(capability.clone())
        .expect("first requirement succeeds");

        assert!(
            contract
                .with_required_capability(capability)
                .is_err()
        );
    }

    #[test]
    fn required_capabilities_have_deterministic_order() {
        let contract = HardwareOperationContract::new(
            operation_descriptor(),
        )
        .with_required_capabilities([
            capability_id("quantum.measurement"),
            capability_id("classical.compute"),
            capability_id("hdl.synthesis"),
        ])
        .expect("valid requirements");

        let capabilities: Vec<&str> = contract
            .required_capabilities()
            .map(CapabilityId::as_str)
            .collect();

        assert_eq!(
            capabilities,
            vec![
                "classical.compute",
                "hdl.synthesis",
                "quantum.measurement",
            ],
        );
    }

    #[test]
    fn batch_requirement_failure_does_not_partially_update_contract() {
        let contract = HardwareOperationContract::new(
            operation_descriptor(),
        )
        .with_required_capability(
            capability_id("classical.compute"),
        )
        .expect("initial requirement");

        let result = contract
            .with_required_capabilities([
                capability_id("hdl.synthesis"),
                capability_id("classical.compute"),
            ]);

        assert!(result.is_err());

        assert_eq!(
            contract.required_capability_count(),
            1,
        );
    }

    // ------------------------------------------------------------------------
    // Provider compatibility validation
    // ------------------------------------------------------------------------

    #[test]
    fn provider_validation_accepts_matching_contract() {
        let provider = provider_descriptor();

        let contract = HardwareOperationContract::new(
            operation_descriptor(),
        )
        .with_required_capability(
            capability_id("classical.compute"),
        )
        .expect("valid requirement");

        assert!(
            contract.validate_provider(&provider).is_ok()
        );
    }

    #[test]
    fn provider_validation_rejects_missing_operation() {
        let mut provider = provider_descriptor();

        let other = ProviderDescriptor::new(
            ProviderId::new("test.other.provider")
                .expect("valid identity"),
            ProviderKind::new("classical")
                .expect("valid kind"),
            "Other provider",
            version("1.0.0"),
            version("1.2.0"),
        )
        .expect("valid descriptor");

        provider = other;

        let contract = HardwareOperationContract::new(
            operation_descriptor(),
        );

        assert!(
            contract.validate_provider(&provider).is_err()
        );
    }

    #[test]
    fn provider_validation_rejects_incompatible_api_version() {
        let provider = provider_descriptor();

        let contract = HardwareOperationContract::new(
            operation_descriptor(),
        )
        .with_provider_compatibility(
            ProviderApiCompatibility::exact(
                version("9.0.0"),
            ),
        );

        assert_eq!(
            contract
                .validate_provider(&provider)
                .unwrap_err()
                .kind(),
            ZhalErrorKind::VersionMismatch,
        );
    }

    #[test]
    fn provider_validation_rejects_missing_capability() {
        let provider = provider_descriptor();

        let contract = HardwareOperationContract::new(
            operation_descriptor(),
        )
        .with_required_capability(
            capability_id("quantum.error_correction"),
        )
        .expect("valid requirement");

        assert_eq!(
            contract
                .validate_provider(&provider)
                .unwrap_err()
                .kind(),
            ZhalErrorKind::CapabilityUnsupported,
        );
    }

    #[test]
    fn provider_validation_distinguishes_operation_versions() {
        let provider = provider_descriptor();

        let different_operation =
            ProviderOperationDescriptor::from_id(
                "classical.compute",
                version("2.0.0"),
            )
            .expect("valid descriptor")
            .with_contracts(
                ProviderContractId::new(
                    "classical.compute.request",
                )
                .unwrap(),
                ProviderContractId::new(
                    "classical.compute.response",
                )
                .unwrap(),
            );

        let contract =
            HardwareOperationContract::new(
                different_operation,
            );

        assert_eq!(
            contract
                .validate_provider(&provider)
                .unwrap_err()
                .kind(),
            ZhalErrorKind::VersionMismatch,
        );
    }

    #[test]
    fn provider_validation_rejects_different_request_contract() {
        let provider = provider_descriptor();

        let descriptor =
            ProviderOperationDescriptor::from_id(
                "classical.compute",
                version("1.0.0"),
            )
            .expect("valid descriptor")
            .with_contracts(
                ProviderContractId::new(
                    "different.request.contract",
                )
                .unwrap(),
                ProviderContractId::new(
                    "classical.compute.response",
                )
                .unwrap(),
            );

        let contract = HardwareOperationContract::new(
            descriptor,
        );

        assert_eq!(
            contract
                .validate_provider(&provider)
                .unwrap_err()
                .kind(),
            ZhalErrorKind::ContractViolation,
        );
    }

    #[test]
    fn provider_validation_rejects_different_response_contract() {
        let provider = provider_descriptor();

        let descriptor =
            ProviderOperationDescriptor::from_id(
                "classical.compute",
                version("1.0.0"),
            )
            .expect("valid descriptor")
            .with_contracts(
                ProviderContractId::new(
                    "classical.compute.request",
                )
                .unwrap(),
                ProviderContractId::new(
                    "different.response.contract",
                )
                .unwrap(),
            );

        let contract = HardwareOperationContract::new(
            descriptor,
        );

        assert_eq!(
            contract
                .validate_provider(&provider)
                .unwrap_err()
                .kind(),
            ZhalErrorKind::ContractViolation,
        );
    }
}

// ============================================================================
// END
// ============================================================================
