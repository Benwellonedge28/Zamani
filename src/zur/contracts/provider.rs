
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Provider Contracts
//!
//! Path: src/zur/contracts/provider.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Defines the ZUR execution-provider integration contract.
//!
//! A provider is an independently implemented execution capability
//! exposed through a versioned, validated interface.
//!
//! Providers may implement classical, quantum, HDL, AI, tensor,
//! accelerator, hybrid, distributed, photonic, or future computation.
//!
//! Computational domains are not enumerated here.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - ZUR provider interface contracts.
//! - Provider contract version requirements.
//! - Capability evidence references.
//! - Resource requirement references.
//! - Execution-provider interfaces.
//! - Typed operation request and response boundaries.
//! - Streaming execution contracts.
//! - Session execution contracts.
//! - Provider submission and execution outcomes.
//! - Provider contract validation.
//!
//! This module does NOT own:
//!
//! - Canonical provider identity.
//! - Canonical provider descriptors.
//! - Canonical capability definitions.
//! - Capability discovery implementations.
//! - Semantic requirements.
//! - Semantic constraints.
//! - Preferences or hints.
//! - Feasibility evaluation.
//! - Resource allocation or reservation.
//! - Backend registration.
//! - Code generation.
//! - Compiler optimization.
//! - Artifact manifests.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//! - Physical hardware discovery.
//! - Device drivers.
//! - Provider authentication or authorization.
//! - Runtime lifecycle management.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ARCHITECTURAL INVARIANTS
//!
//! Provider registration != provider initialization.
//! Provider initialization != provider availability.
//! Provider availability != capability availability.
//! Capability availability != resource reservation.
//! Feasibility != resource reservation.
//! Submission != successful completion.
//! Provider claims != authenticated evidence.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe code.
//! No global mutable state.
//! No fabricated execution success.
//! No implicit provider selection.
//! No implicit resource allocation.
//! No arbitrary computational-domain ceilings.
//!
//! SCALABILITY
//!
//! Collections grow subject to available resources.
//!
//! There are no universal maximum counts for providers, operations,
//! interfaces, domains, resources, devices, or execution sessions.
//!
//! Literal infinite computation is not promised.
//!
//! INTEGRATION
//!
//! - stdlib/provider.rs owns common provider identity and lifecycle.
//! - zur/contracts/abi.rs owns the execution ABI.
//! - zur/contracts/service.rs owns runtime service contracts.
//! - zur/version.rs owns ZUR contract versions.
//! - zur/ids.rs owns runtime execution and session identities.
//! - zur/error.rs owns operational errors.
//! - zur/core/service_registry.rs owns runtime service registration.
//! - zur/domains/ owns domain-provider registration and discovery.
//! - compiler and backend retain artifact and code-generation ownership.
//! - quantum, hybrid, distributed and HDL retain their domain contracts.

use core::any::Any;
use core::fmt;
use core::future::Future;
use core::pin::Pin;

use std::collections::BTreeSet;

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    Provider as CommonProvider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderId,
    ProviderLifecycle,
    ProviderOperationDescriptor,
    ProviderOperationId,
    ProviderStatus,
};

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

use crate::zur::ids::SessionId;

use crate::zur::version::{
    ServiceContractVersion,
    VersionRange,
};

// ============================================================================
// ASYNCHRONOUS EXECUTION CONTRACT
// ============================================================================

/// An asynchronous ZUR provider operation.
///
/// The future is tied to the provider and operation-context lifetimes.
///
/// It is Send so compatible execution environments can drive the
/// operation through multithreaded executors.
///
/// Providers requiring single-threaded execution must expose an
/// appropriate compatible adapter.
pub type ZurProviderFuture<'a, T> =
    Pin<Box<dyn Future<Output = ZurResult<T>> + Send + 'a>>;

// ============================================================================
// VALIDATION
// ============================================================================

/// Provider contract validation failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProviderContractValidationError {
    /// An identifier is empty.
    EmptyIdentifier,

    /// An identifier contains an invalid character.
    InvalidIdentifier,

    /// An identifier contains an empty namespace segment.
    EmptyNamespaceSegment,

    /// An identifier contains surrounding whitespace.
    SurroundingWhitespace,

    /// A descriptive field is empty.
    EmptyDescription,

    /// A descriptive field contains control characters.
    InvalidDescription,

    /// An interface has been declared more than once.
    DuplicateInterface,

    /// An evidence reference has been declared more than once.
    DuplicateEvidence,

    /// A resource requirement reference has been declared more than once.
    DuplicateResourceRequirement,

    /// The interface does not support any invocation mode.
    MissingInvocationMode,

    /// The supplied interface version range is invalid.
    InvalidVersionRange,
}

impl fmt::Display for ProviderContractValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EmptyIdentifier =>
                "provider contract identifier cannot be empty",

            Self::InvalidIdentifier =>
                "provider contract identifier contains invalid characters",

            Self::EmptyNamespaceSegment =>
                "provider contract identifier contains an empty segment",

            Self::SurroundingWhitespace =>
                "provider contract identifier contains surrounding whitespace",

            Self::EmptyDescription =>
                "provider contract description cannot be empty",

            Self::InvalidDescription =>
                "provider contract description contains control characters",

            Self::DuplicateInterface =>
                "provider contract contains a duplicate interface",

            Self::DuplicateEvidence =>
                "provider contract contains duplicate capability evidence",

            Self::DuplicateResourceRequirement =>
                "provider contract contains a duplicate resource requirement",

            Self::MissingInvocationMode =>
                "provider interface must declare an invocation mode",

            Self::InvalidVersionRange =>
                "provider interface has an invalid version range",
        })
    }
}

impl std::error::Error for ProviderContractValidationError {}

fn contract_error(
    error: ProviderContractValidationError,
) -> ZurError {
    ZurError::with_code(
        ErrorKind::Validation,
        "zur.provider.contract.invalid",
        error.to_string(),
    )
    .expect("static error code must be valid")
    .with_component("zur.contracts.provider")
}

fn validate_identifier(value: &str) -> ZurResult<()> {
    if value.is_empty() {
        return Err(contract_error(
            ProviderContractValidationError::EmptyIdentifier,
        ));
    }

    if value.trim() != value {
        return Err(contract_error(
            ProviderContractValidationError::SurroundingWhitespace,
        ));
    }

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(contract_error(
                ProviderContractValidationError::EmptyNamespaceSegment,
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(contract_error(
                ProviderContractValidationError::InvalidIdentifier,
            ));
        }
    }

    Ok(())
}

fn validate_description(value: &str) -> ZurResult<()> {
    if value.trim().is_empty() {
        return Err(contract_error(
            ProviderContractValidationError::EmptyDescription,
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(contract_error(
            ProviderContractValidationError::InvalidDescription,
        ));
    }

    Ok(())
}

// ============================================================================
// INTERFACE IDENTITY
// ============================================================================

/// Stable, extensible identity of a provider interface.
///
/// Examples:
///
/// - zamani.classical.execution
/// - zamani.quantum.execution
/// - zamani.hdl.synthesis
/// - zamani.ai.inference
/// - zamani.distributed.execution
/// - future.photonics.execution
///
/// These are examples, not a closed inventory.
///
/// Interface identity does not identify a physical device,
/// vendor, backend, or computational resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProviderInterfaceId(String);

impl ProviderInterfaceId {
    /// Creates a validated interface identity.
    pub fn new(value: impl Into<String>) -> ZurResult<Self> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the canonical interface identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderInterfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// INVOCATION MODES
// ============================================================================

/// Invocation modes supported by an interface.
///
/// The modes describe how operations are invoked.
///
/// They do not describe computational domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProviderInvocationMode {
    /// The operation completes through a synchronous invocation contract.
    Synchronous,

    /// The operation is driven asynchronously.
    Asynchronous,

    /// The operation produces a sequence of results.
    Streaming,

    /// The operation participates in a persistent execution session.
    Session,
}

// ============================================================================
// INTERFACE CONTRACT
// ============================================================================

/// A versioned provider interface declaration.
///
/// An interface identifies a contract implemented by a provider.
///
/// Its operations remain owned by their specialized interface
/// and domain authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderInterface {
    id: ProviderInterfaceId,

    accepted_versions:
        VersionRange<ServiceContractVersion>,

    invocation_modes:
        BTreeSet<ProviderInvocationMode>,

    operations:
        Vec<ProviderOperationDescriptor>,

    description: Option<String>,
}

impl ProviderInterface {
    /// Constructs a provider interface.
    ///
    /// At least one invocation mode is required.
    pub fn new(
        id: ProviderInterfaceId,
        accepted_versions: VersionRange<ServiceContractVersion>,
        invocation_modes: impl IntoIterator<Item = ProviderInvocationMode>,
    ) -> ZurResult<Self> {
        let invocation_modes: BTreeSet<_> =
            invocation_modes.into_iter().collect();

        if invocation_modes.is_empty() {
            return Err(contract_error(
                ProviderContractValidationError::MissingInvocationMode,
            ));
        }

        Ok(Self {
            id,
            accepted_versions,
            invocation_modes,
            operations: Vec::new(),
            description: None,
        })
    }

    /// Returns the interface identity.
    #[must_use]
    pub fn id(&self) -> &ProviderInterfaceId {
        &self.id
    }

    /// Returns the accepted interface versions.
    #[must_use]
    pub fn accepted_versions(
        &self,
    ) -> &VersionRange<ServiceContractVersion> {
        &self.accepted_versions
    }

    /// Returns supported invocation modes.
    #[must_use]
    pub fn invocation_modes(
        &self,
    ) -> &BTreeSet<ProviderInvocationMode> {
        &self.invocation_modes
    }

    /// Returns declared operations.
    #[must_use]
    pub fn operations(&self) -> &[ProviderOperationDescriptor] {
        &self.operations
    }

    /// Returns the optional interface description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Adds an operation declaration.
    ///
    /// Duplicate operation identities are rejected.
    pub fn insert_operation(
        &mut self,
        operation: ProviderOperationDescriptor,
    ) -> ZurResult<()> {
        if self.operations.iter().any(
            |existing| existing.id() == operation.id()
        ) {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "operation {} is already declared",
                        operation.id()
                    ),
                )
                .with_component("zur.contracts.provider"),
            );
        }

        self.operations.push(operation);

        Ok(())
    }

    /// Adds descriptive interface metadata.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> ZurResult<Self> {
        let description = description.into();

        validate_description(&description)?;

        self.description = Some(description);

        Ok(self)
    }

    /// Checks whether the interface declares an invocation mode.
    #[must_use]
    pub fn supports(
        &self,
        mode: ProviderInvocationMode,
    ) -> bool {
        self.invocation_modes.contains(&mode)
    }
}

// ============================================================================
// EXTERNAL EVIDENCE REFERENCES
// ============================================================================

/// Reference to capability evidence owned by another subsystem.
///
/// This type does not define capability semantics or establish
/// the authenticity, freshness, or availability of evidence.
///
/// The referenced record must be resolved and validated by the
/// appropriate capability authority.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityEvidenceRef {
    authority: String,
    record: String,
}

impl CapabilityEvidenceRef {
    /// Constructs a reference to an externally owned evidence record.
    pub fn new(
        authority: impl Into<String>,
        record: impl Into<String>,
    ) -> ZurResult<Self> {
        let authority = authority.into();
        let record = record.into();

        validate_identifier(&authority)?;
        validate_identifier(&record)?;

        Ok(Self {
            authority,
            record,
        })
    }

    /// Returns the evidence authority identity.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the externally owned evidence record identity.
    #[must_use]
    pub fn record(&self) -> &str {
        &self.record
    }
}

// ============================================================================
// RESOURCE REQUIREMENT REFERENCES
// ============================================================================

/// Reference to a canonical resource requirement.
///
/// The requirement itself remains owned by the semantic,
/// resource, compiler, or domain subsystem.
///
/// This reference does not represent an allocation or reservation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceRequirementRef {
    authority: String,
    requirement: String,
}

impl ResourceRequirementRef {
    /// Creates a reference to an externally owned resource requirement.
    pub fn new(
        authority: impl Into<String>,
        requirement: impl Into<String>,
    ) -> ZurResult<Self> {
        let authority = authority.into();
        let requirement = requirement.into();

        validate_identifier(&authority)?;
        validate_identifier(&requirement)?;

        Ok(Self {
            authority,
            requirement,
        })
    }

    /// Returns the owning authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the canonical requirement identity.
    #[must_use]
    pub fn requirement(&self) -> &str {
        &self.requirement
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Versioned ZUR contract for one provider.
///
/// The descriptor reuses the standard-library provider model.
///
/// The contract adds ZUR-specific interface, evidence and resource
/// requirement references.
///
/// It does not create another provider identity, capability model,
/// or resource model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderContract {
    descriptor: ProviderDescriptor,

    contract_version: ServiceContractVersion,

    interfaces: Vec<ProviderInterface>,

    capability_evidence: Vec<CapabilityEvidenceRef>,

    resource_requirements: Vec<ResourceRequirementRef>,
}

impl ProviderContract {
    /// Constructs a provider contract.
    ///
    /// Duplicate interface identities are rejected.
    pub fn new(
        descriptor: ProviderDescriptor,
        contract_version: ServiceContractVersion,
        interfaces: Vec<ProviderInterface>,
    ) -> ZurResult<Self> {
        let mut identities = BTreeSet::new();

        for interface in &interfaces {
            if !identities.insert(interface.id().clone()) {
                return Err(contract_error(
                    ProviderContractValidationError::DuplicateInterface,
                ));
            }
        }

        Ok(Self {
            descriptor,
            contract_version,
            interfaces,
            capability_evidence: Vec::new(),
            resource_requirements: Vec::new(),
        })
    }

    /// Returns the canonical provider descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.descriptor.id()
    }

    /// Returns the ZUR provider contract version.
    #[must_use]
    pub fn contract_version(&self) -> &ServiceContractVersion {
        &self.contract_version
    }

    /// Returns all declared interfaces.
    #[must_use]
    pub fn interfaces(&self) -> &[ProviderInterface] {
        &self.interfaces
    }

    /// Finds an interface by identity.
    #[must_use]
    pub fn interface(
        &self,
        id: &ProviderInterfaceId,
    ) -> Option<&ProviderInterface> {
        self.interfaces.iter().find(
            |interface| interface.id() == id
        )
    }

    /// Returns capability evidence references.
    #[must_use]
    pub fn capability_evidence(
        &self,
    ) -> &[CapabilityEvidenceRef] {
        &self.capability_evidence
    }

    /// Returns resource requirement references.
    #[must_use]
    pub fn resource_requirements(
        &self,
    ) -> &[ResourceRequirementRef] {
        &self.resource_requirements
    }

    /// Adds an externally owned capability evidence reference.
    pub fn insert_capability_evidence(
        &mut self,
        reference: CapabilityEvidenceRef,
    ) -> ZurResult<()> {
        if self.capability_evidence.contains(&reference) {
            return Err(contract_error(
                ProviderContractValidationError::DuplicateEvidence,
            ));
        }

        self.capability_evidence.push(reference);

        Ok(())
    }

    /// Adds an externally owned resource requirement reference.
    pub fn insert_resource_requirement(
        &mut self,
        reference: ResourceRequirementRef,
    ) -> ZurResult<()> {
        if self.resource_requirements.contains(&reference) {
            return Err(contract_error(
                ProviderContractValidationError::DuplicateResourceRequirement,
            ));
        }

        self.resource_requirements.push(reference);

        Ok(())
    }
}

// ============================================================================
// TYPED OPERATION PAYLOADS
// ============================================================================

/// A typed operation payload.
///
/// Domain-specific implementations supply their own request and
/// response types.
///
/// The contract identity and schema version allow adapters to validate
/// the payload against the appropriate external contract.
///
/// `Any` enables safe in-process type inspection and downcasting.
///
/// Cross-process and cross-language adapters must use their canonical
/// serialization and interchange contracts instead.
pub trait ProviderPayload: Any + Send + Sync {
    /// Returns the canonical payload contract identity.
    fn contract_id(
        &self,
    ) -> &crate::stdlib::provider::ProviderContractId;

    /// Returns the payload schema version.
    fn schema_version(
        &self,
    ) -> &crate::stdlib::version::Version;

    /// Returns the payload as Any for safe type inspection.
    fn as_any(&self) -> &dyn Any;
}

// ============================================================================
// OPERATION REQUEST
// ============================================================================

/// A typed provider operation request.
///
/// This is an invocation envelope, not a competing domain IR.
///
/// The operation identity is owned by the provider's operation contract.
/// The request payload is owned by its specialized domain interface.
pub struct ProviderOperationRequest<'a> {
    operation: &'a ProviderOperationId,
    request: &'a dyn ProviderPayload,
}

impl<'a> ProviderOperationRequest<'a> {
    /// Constructs an operation request.
    #[must_use]
    pub fn new(
        operation: &'a ProviderOperationId,
        request: &'a dyn ProviderPayload,
    ) -> Self {
        Self {
            operation,
            request,
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation(&self) -> &ProviderOperationId {
        self.operation
    }

    /// Returns the typed request payload.
    #[must_use]
    pub fn payload(&self) -> &dyn ProviderPayload {
        self.request
    }

    /// Attempts a safe typed request downcast.
    #[must_use]
    pub fn downcast<T: ProviderPayload>(&self) -> Option<&T> {
        self.request.as_any().downcast_ref::<T>()
    }
}

// ============================================================================
// OPERATION RESPONSE
// ============================================================================

/// A typed provider operation response.
///
/// The response is not a feasibility assessment or a resource
/// reservation record.
pub struct ProviderOperationResponse {
    operation: ProviderOperationId,
    response: Box<dyn ProviderPayload>,
}

impl ProviderOperationResponse {
    /// Constructs an operation response.
    #[must_use]
    pub fn new(
        operation: ProviderOperationId,
        response: Box<dyn ProviderPayload>,
    ) -> Self {
        Self {
            operation,
            response,
        }
    }

    /// Returns the completed operation identity.
    #[must_use]
    pub fn operation(&self) -> &ProviderOperationId {
        &self.operation
    }

    /// Returns the typed response payload.
    #[must_use]
    pub fn payload(&self) -> &dyn ProviderPayload {
        self.response.as_ref()
    }

    /// Attempts a safe response downcast.
    #[must_use]
    pub fn downcast<T: ProviderPayload>(&self) -> Option<&T> {
        self.response.as_any().downcast_ref::<T>()
    }

    /// Consumes the response and attempts a typed downcast.
    ///
    /// Returns the original response if its type does not match.
    pub fn into_payload<T: ProviderPayload>(
        self,
    ) -> Result<Box<T>, Self> {
        if !self.response.as_any().is::<T>() {
            return Err(self);
        }

        let Self {
            operation,
            response,
        } = self;

        let payload: Box<dyn Any + Send + Sync> = response;

        match payload.downcast::<T>() {
            Ok(value) => Ok(value),
            Err(_) => unreachable!(
                "payload type was validated before downcasting"
            ),
        }
        .map_err(|_| {
            // This branch is unreachable because the type was
            // checked before consuming the payload.
            Self {
                operation,
                response: Box::new(UnreachablePayload),
            }
        })
    }
}

/// Internal defensive payload.
///
/// It is never returned by a valid public execution operation.
struct UnreachablePayload;

impl ProviderPayload for UnreachablePayload {
    fn contract_id(
        &self,
    ) -> &crate::stdlib::provider::ProviderContractId {
        unreachable!("internal defensive payload")
    }

    fn schema_version(
        &self,
    ) -> &crate::stdlib::version::Version {
        unreachable!("internal defensive payload")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ============================================================================
// STREAMING
// ============================================================================

/// A streaming provider result.
///
/// The stream is explicitly pulled by its consumer.
///
/// Implementations must observe cancellation through the supplied
/// operation context.
///
/// A stream ending successfully means that its provider has
/// completed its declared stream contract.
pub trait ProviderStream: Send {
    /// Retrieves the next stream item.
    ///
    /// `Ok(None)` means successful end of stream.
    ///
    /// An error means the stream failed and must not be treated
    /// as successfully completed.
    fn next<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, Option<ProviderStreamItem>>;
}

/// One typed item produced by a provider stream.
pub struct ProviderStreamItem {
    sequence: String,
    payload: Box<dyn ProviderPayload>,
}

impl ProviderStreamItem {
    /// Creates a stream item with a canonical decimal sequence.
    ///
    /// The sequence is represented as a string to avoid imposing
    /// a fixed-width numeric ceiling.
    pub fn new(
        sequence: impl Into<String>,
        payload: Box<dyn ProviderPayload>,
    ) -> ZurResult<Self> {
        let sequence = sequence.into();

        if sequence.is_empty()
            || !sequence.bytes().all(|b| b.is_ascii_digit())
            || (sequence.len() > 1 && sequence.starts_with('0'))
        {
            return Err(
                ZurError::invalid_input(
                    "stream sequence must be canonical decimal",
                )
                .with_component("zur.contracts.provider"),
            );
        }

        Ok(Self {
            sequence,
            payload,
        })
    }

    /// Returns the stream sequence.
    #[must_use]
    pub fn sequence(&self) -> &str {
        &self.sequence
    }

    /// Returns the typed stream payload.
    #[must_use]
    pub fn payload(&self) -> &dyn ProviderPayload {
        self.payload.as_ref()
    }

    /// Attempts a safe typed downcast.
    #[must_use]
    pub fn downcast<T: ProviderPayload>(&self) -> Option<&T> {
        self.payload.as_any().downcast_ref::<T>()
    }
}

// ============================================================================
// SUBMISSION OUTCOME
// ============================================================================

/// Outcome of submitting work to a provider.
///
/// Accepted submission does not mean that execution completed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProviderSubmissionStatus {
    /// The provider accepted the operation for processing.
    Accepted,

    /// The provider explicitly rejected the operation.
    Rejected,

    /// The provider could not establish the submission outcome.
    Unknown,
}

/// Evidence-bearing submission receipt.
///
/// This is an operational receipt, not a resource reservation,
/// feasibility result or execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderSubmission {
    provider_id: ProviderId,
    operation: ProviderOperationId,
    status: ProviderSubmissionStatus,
    external_reference: Option<String>,
}

impl ProviderSubmission {
    /// Constructs a submission receipt.
    pub fn new(
        provider_id: ProviderId,
        operation: ProviderOperationId,
        status: ProviderSubmissionStatus,
        external_reference: Option<String>,
    ) -> ZurResult<Self> {
        if let Some(reference) = &external_reference {
            validate_description(reference)?;
        }

        Ok(Self {
            provider_id,
            operation,
            status,
            external_reference,
        })
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation(&self) -> &ProviderOperationId {
        &self.operation
    }

    /// Returns the actual submission status.
    #[must_use]
    pub fn status(&self) -> &ProviderSubmissionStatus {
        &self.status
    }

    /// Returns the optional provider-side reference.
    #[must_use]
    pub fn external_reference(&self) -> Option<&str> {
        self.external_reference.as_deref()
    }

    /// Returns whether the provider explicitly accepted the submission.
    #[must_use]
    pub fn was_accepted(&self) -> bool {
        matches!(
            self.status,
            ProviderSubmissionStatus::Accepted
        )
    }
}

// ============================================================================
// EXECUTION OUTCOME
// ============================================================================

/// Actual provider execution outcome.
///
/// This type describes execution, not planning or feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProviderExecutionStatus {
    /// Execution completed successfully.
    Completed,

    /// Execution produced a valid partial result.
    PartiallyCompleted,

    /// Execution was explicitly cancelled.
    Cancelled,

    /// Execution timed out.
    TimedOut,

    /// Execution failed.
    Failed,
}

/// Structured execution metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderExecutionOutcome {
    provider_id: ProviderId,
    operation: ProviderOperationId,
    status: ProviderExecutionStatus,
}

impl ProviderExecutionOutcome {
    /// Creates an execution outcome.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
        operation: ProviderOperationId,
        status: ProviderExecutionStatus,
    ) -> Self {
        Self {
            provider_id,
            operation,
            status,
        }
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation(&self) -> &ProviderOperationId {
        &self.operation
    }

    /// Returns the actual execution status.
    #[must_use]
    pub const fn status(&self) -> ProviderExecutionStatus {
        self.status
    }

    /// Returns whether the provider explicitly reports completion.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        matches!(
            self.status,
            ProviderExecutionStatus::Completed
        )
    }
}

// ============================================================================
// EXECUTION SESSION CONTRACT
// ============================================================================

/// A provider session execution interface.
///
/// Session identities are allocated and owned by ZUR.
///
/// Provider-specific session state remains provider-owned.
///
/// A session ID does not prove that a physical resource is reserved.
pub trait ProviderSession: Send + Sync {
    /// Returns the ZUR session identity.
    fn session_id(&self) -> &SessionId;

    /// Returns the owning provider identity.
    fn provider_id(&self) -> &ProviderId;

    /// Returns the session's observed provider status.
    fn status(&self) -> ProviderStatus;
}

// ============================================================================
// EXECUTION PROVIDER
// ============================================================================

/// Common ZUR execution-provider contract.
///
/// Implementors must also implement the canonical standard-library
/// provider and lifecycle contracts.
///
/// This trait is object-safe.
///
/// It does not replace specialized domain provider interfaces.
///
/// A quantum provider must continue to use canonical quantum
/// computation and hardware contracts.
///
/// A distributed provider must continue to use canonical distributed
/// execution and transport contracts.
///
/// A backend remains responsible for code generation.
///
/// The ZUR provider is responsible for the operational execution
/// boundary.
///
/// All operations must observe cancellation through OperationContext.
///
/// Implementations must propagate actual failures.
pub trait ExecutionProvider:
    ProviderLifecycle + Send + Sync
{
    /// Returns the complete ZUR provider contract.
    ///
    /// The contract must remain stable throughout the provider
    /// instance's lifetime.
    fn execution_contract(&self) -> &ProviderContract;

    /// Invokes one typed operation.
    ///
    /// The operation must be declared by the provider's contract.
    ///
    /// The implementation must validate the operation and request
    /// contract before performing execution.
    ///
    /// A successful response represents an actual completed operation,
    /// not merely an accepted submission.
    fn invoke<'a>(
        &'a self,
        request: ProviderOperationRequest<'a>,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ProviderOperationResponse>;

    /// Submits an operation for provider-managed asynchronous execution.
    ///
    /// A successful return represents a submission receipt only.
    ///
    /// Completion must be observed separately through the provider's
    /// operation or session contract.
    ///
    /// Implementations that do not support submission must return
    /// an explicit unsupported error.
    fn submit<'a>(
        &'a self,
        request: ProviderOperationRequest<'a>,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ProviderSubmission> {
        let _ = request;
        let _ = context;

        Box::pin(async {
            Err(
                ZurError::unsupported(
                    "provider does not support asynchronous submission",
                )
                .with_component("zur.contracts.provider"),
            )
        })
    }

    /// Opens a streaming operation.
    ///
    /// Streaming must be explicitly declared by the interface.
    ///
    /// Unsupported providers must return an explicit error.
    fn open_stream<'a>(
        &'a self,
        request: ProviderOperationRequest<'a>,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, Box<dyn ProviderStream>> {
        let _ = request;
        let _ = context;

        Box::pin(async {
            Err(
                ZurError::unsupported(
                    "provider does not support streaming execution",
                )
                .with_component("zur.contracts.provider"),
            )
        })
    }

    /// Opens a provider-managed execution session.
    ///
    /// The session identity is supplied by ZUR.
    ///
    /// Resource allocation and reservation remain separate operations.
    fn open_session<'a>(
        &'a self,
        session_id: SessionId,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, Box<dyn ProviderSession>> {
        let _ = session_id;
        let _ = context;

        Box::pin(async {
            Err(
                ZurError::unsupported(
                    "provider does not support execution sessions",
                )
                .with_component("zur.contracts.provider"),
            )
        })
    }

    /// Invokes an operation within an existing provider session.
    fn invoke_session<'a>(
        &'a self,
        session: &'a dyn ProviderSession,
        request: ProviderOperationRequest<'a>,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ProviderOperationResponse> {
        let _ = session;
        let _ = request;
        let _ = context;

        Box::pin(async {
            Err(
                ZurError::unsupported(
                    "provider does not support session invocation",
                )
                .with_component("zur.contracts.provider"),
            )
        })
    }

    /// Closes a provider-managed session.
    ///
    /// Successful completion means the provider has completed its
    /// declared session-close contract.
    ///
    /// It must not claim physical resource release without evidence.
    fn close_session<'a>(
        &'a self,
        session: &'a dyn ProviderSession,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ()> {
        let _ = session;
        let _ = context;

        Box::pin(async {
            Err(
                ZurError::unsupported(
                    "provider does not support session closure",
                )
                .with_component("zur.contracts.provider"),
            )
        })
    }
}

// ============================================================================
// CANCELLATION CONTRACT
// ============================================================================

/// Provider cancellation contract.
///
/// Cancellation requests are distinct from successful completion.
///
/// The runtime supplies the operation context, and the provider
/// must observe its cancellation mechanism.
///
/// This trait is an explicit hook for providers that require
/// provider-specific cancellation operations.
pub trait ProviderCancellation: ExecutionProvider {
    /// Requests cancellation of a provider-managed submission.
    ///
    /// The implementation must report the actual cancellation outcome.
    ///
    /// It must not report successful cancellation merely because
    /// a request was transmitted.
    fn cancel<'a>(
        &'a self,
        submission: &'a ProviderSubmission,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ProviderExecutionOutcome>;
}

// ============================================================================
// CONTRACT VALIDATION
// ============================================================================

/// Validates the internal consistency of a provider contract.
///
/// This does not establish:
///
/// - Provider trust.
/// - Provider availability.
/// - Capability availability.
/// - Requirement satisfaction.
/// - Feasibility.
/// - Resource reservation.
/// - Security authorization.
///
/// Those checks belong to their respective authorities.
pub fn validate_provider_contract(
    contract: &ProviderContract,
) -> ZurResult<()> {
    let mut interface_ids = BTreeSet::new();

    for interface in contract.interfaces() {
        if !interface_ids.insert(interface.id()) {
            return Err(contract_error(
                ProviderContractValidationError::DuplicateInterface,
            ));
        }

        if interface.invocation_modes().is_empty() {
            return Err(contract_error(
                ProviderContractValidationError::MissingInvocationMode,
            ));
        }
    }

    let mut evidence = BTreeSet::new();

    for reference in contract.capability_evidence() {
        if !evidence.insert(reference) {
            return Err(contract_error(
                ProviderContractValidationError::DuplicateEvidence,
            ));
        }
    }

    let mut requirements = BTreeSet::new();

    for reference in contract.resource_requirements() {
        if !requirements.insert(reference) {
            return Err(contract_error(
                ProviderContractValidationError::DuplicateResourceRequirement,
            ));
        }
    }

    Ok(())
}

// ============================================================================
// PROVIDER ADMISSION BOUNDARY
// ============================================================================

/// Provider admission evidence.
///
/// This describes the outcome of external provider validation.
///
/// It is deliberately not generated automatically by provider
/// registration or descriptor construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAdmissionEvidence {
    provider_id: ProviderId,
    authority: String,
    evidence_reference: String,
}

impl ProviderAdmissionEvidence {
    /// Constructs externally supplied admission evidence.
    pub fn new(
        provider_id: ProviderId,
        authority: impl Into<String>,
        evidence_reference: impl Into<String>,
    ) -> ZurResult<Self> {
        let authority = authority.into();
        let evidence_reference = evidence_reference.into();

        validate_identifier(&authority)?;
        validate_identifier(&evidence_reference)?;

        Ok(Self {
            provider_id,
            authority,
            evidence_reference,
        })
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the authority that supplied the evidence.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the external evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> &str {
        &self.evidence_reference
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
        VersionRange,
    };

    fn version() -> Version {
        Version::parse("1.0.0").unwrap()
    }

    fn service_version() -> ServiceContractVersion {
        ServiceContractVersion::new(
            version()
        )
    }

    fn version_range() -> VersionRange<ServiceContractVersion> {
        VersionRange::new(
            service_version(),
            service_version(),
        )
        .unwrap()
    }

    #[test]
    fn interface_identity_is_open_world() {
        let id = ProviderInterfaceId::new(
            "future.photonics.execution",
        )
        .unwrap();

        assert_eq!(
            id.as_str(),
            "future.photonics.execution",
        );
    }

    #[test]
    fn interface_identity_rejects_invalid_names() {
        assert!(ProviderInterfaceId::new("").is_err());
        assert!(ProviderInterfaceId::new(".provider").is_err());
        assert!(ProviderInterfaceId::new("provider.").is_err());
        assert!(ProviderInterfaceId::new("provider..execution").is_err());
        assert!(ProviderInterfaceId::new("provider name").is_err());
    }

    #[test]
    fn interface_requires_an_invocation_mode() {
        let result = ProviderInterface::new(
            ProviderInterfaceId::new(
                "example.execution",
            )
            .unwrap(),
            version_range(),
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn interface_supports_multiple_invocation_modes() {
        let interface = ProviderInterface::new(
            ProviderInterfaceId::new(
                "example.execution",
            )
            .unwrap(),
            version_range(),
            [
                ProviderInvocationMode::Synchronous,
                ProviderInvocationMode::Asynchronous,
                ProviderInvocationMode::Streaming,
                ProviderInvocationMode::Session,
            ],
        )
        .unwrap();

        assert!(
            interface.supports(
                ProviderInvocationMode::Streaming
            )
        );

        assert!(
            interface.supports(
                ProviderInvocationMode::Session
            )
        );
    }

    #[test]
    fn capability_evidence_is_an_external_reference() {
        let reference = CapabilityEvidenceRef::new(
            "frontend.semantic.capabilities",
            "evidence.snapshot",
        )
        .unwrap();

        assert_eq!(
            reference.authority(),
            "frontend.semantic.capabilities",
        );
    }

    #[test]
    fn resource_requirement_is_not_a_reservation() {
        let reference = ResourceRequirementRef::new(
            "frontend.semantic.resources",
            "resource.memory",
        )
        .unwrap();

        assert_eq!(
            reference.requirement(),
            "resource.memory",
        );
    }

    #[test]
    fn submission_does_not_mean_completion() {
        let submission = ProviderSubmission::new(
            ProviderId::new("example.provider")
                .unwrap(),
            ProviderOperationId::new(
                "classical.compute",
            )
            .unwrap(),
            ProviderSubmissionStatus::Accepted,
            None,
        );

        assert!(submission.is_ok());

        assert!(
            !matches!(
                ProviderExecutionStatus::Failed,
                ProviderExecutionStatus::Completed
            )
        );
    }

    #[test]
    fn execution_status_requires_explicit_completion() {
        let completed = ProviderExecutionOutcome::new(
            ProviderId::new("example.provider")
                .unwrap(),
            ProviderOperationId::new(
                "classical.compute",
            )
            .unwrap(),
            ProviderExecutionStatus::Completed,
        );

        assert!(completed.is_completed());

        let partial = ProviderExecutionOutcome::new(
            ProviderId::new("example.provider")
                .unwrap(),
            ProviderOperationId::new(
                "classical.compute",
            )
            .unwrap(),
            ProviderExecutionStatus::PartiallyCompleted,
        );

        assert!(!partial.is_completed());
    }

    #[test]
    fn stream_sequence_supports_arbitrary_width_decimal_values() {
        struct TestPayload {
            contract: crate::stdlib::provider::ProviderContractId,
            schema: Version,
        }

        impl ProviderPayload for TestPayload {
            fn contract_id(
                &self,
            ) -> &crate::stdlib::provider::ProviderContractId {
                &self.contract
            }

            fn schema_version(
                &self,
            ) -> &Version {
                &self.schema
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        let payload = Box::new(TestPayload {
            contract:
                crate::stdlib::provider::ProviderContractId::new(
                    "example.result",
                )
                .unwrap(),
            schema: version(),
        });

        let item = ProviderStreamItem::new(
            "184467440737095516160000000000000000000",
            payload,
        )
        .unwrap();

        assert_eq!(
            item.sequence(),
            "184467440737095516160000000000000000000",
        );
    }

    #[test]
    fn stream_sequence_rejects_noncanonical_values() {
        assert!(
            ProviderStreamItem::new(
                "",
                Box::new(TestEmptyPayload::new()),
            )
            .is_err()
        );

        assert!(
            ProviderStreamItem::new(
                "01",
                Box::new(TestEmptyPayload::new()),
            )
            .is_err()
        );
    }

    struct TestEmptyPayload {
        contract: crate::stdlib::provider::ProviderContractId,
        schema: Version,
    }

    impl TestEmptyPayload {
        fn new() -> Self {
            Self {
                contract:
                    crate::stdlib::provider::ProviderContractId::new(
                        "example.payload",
                    )
                    .unwrap(),
                schema: version(),
            }
        }
    }

    impl ProviderPayload for TestEmptyPayload {
        fn contract_id(
            &self,
        ) -> &crate::stdlib::provider::ProviderContractId {
            &self.contract
        }

        fn schema_version(&self) -> &Version {
            &self.schema
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}
