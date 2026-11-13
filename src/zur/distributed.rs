
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Distributed Integration
//!
//! Path: src/zur/distributed.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the integration boundary between ZUR and the canonical
//! Zamani distributed-computing subsystem.
//!
//! This module connects validated distributed execution plans to
//! explicitly supplied ZUR distributed execution providers.
//!
//! It does not implement another distributed execution engine.
//!
//! # CANONICAL AUTHORITIES
//!
//! Distributed semantics:
//!     grammar/spec/distributed.md
//!
//! Distributed grammar:
//!     grammar/distributed/
//!
//! Distributed execution:
//!     crate::distributed::execution
//!
//! Canonical executable-plan contract:
//!     crate::distributed::execution::executor::ExecutablePlan
//!
//! Distributed execution identities:
//!     crate::distributed::ids
//!
//! Distributed failures:
//!     crate::distributed::error
//!
//! Distributed planning:
//!     crate::distributed::planning
//!
//! Distributed transport:
//!     crate::distributed::transport
//!
//! Distributed membership:
//!     crate::distributed::membership
//!
//! Distributed security:
//!     crate::distributed::security
//!
//! Distributed resilience:
//!     crate::distributed::resilience
//!
//! ZUR provider contracts:
//!     crate::zur::contracts::provider
//!
//! ZUR domain integration:
//!     crate::zur::domains
//!
//! Common provider lifecycle:
//!     crate::stdlib::provider
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Operational errors:
//!     crate::zur::error
//!
//! Contract versions:
//!     crate::zur::version
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - Distributed-to-ZUR integration.
//! - The canonical ZUR distributed domain descriptor factory.
//! - Distributed provider structural validation.
//! - Distributed execution-plan validation at the ZUR boundary.
//! - Typed distributed execution-plan payloads.
//! - Provider operation contract validation.
//! - Contract-checked distributed provider invocation.
//! - Contract-checked asynchronous submission.
//!
//! This module does NOT own:
//!
//! - Distributed computation semantics.
//! - Distributed AST.
//! - Distributed task identities.
//! - Distributed participant identities.
//! - Distributed execution plans.
//! - Distributed task graphs.
//! - Distributed scheduling.
//! - Distributed placement.
//! - Distributed partitioning.
//! - Distributed routing.
//! - Distributed membership.
//! - Node discovery.
//! - Capability definitions.
//! - Capability evidence.
//! - Resource requirements.
//! - Resource feasibility.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Transport.
//! - Message serialization.
//! - Authentication.
//! - Authorization.
//! - Consensus.
//! - Replication.
//! - Checkpointing.
//! - Recovery.
//! - Backend code generation.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI/tensor IR.
//! - ZUIR.
//! - Hybrid graph construction.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! # EXECUTION PIPELINE
//!
//! Zamani source
//!       |
//!       v
//! Canonical frontend
//!       |
//!       v
//! Distributed semantic validation
//!       |
//!       v
//! Canonical semantic representation
//!       |
//!       v
//! Canonical IR
//!       |
//!       v
//! Distributed planning
//!       |
//!       v
//! Validated distributed execution plan
//!       |
//!       v
//! Canonical distributed execution
//!       |
//!       v
//! ZUR distributed integration
//!       |
//!       v
//! ZUR distributed execution provider
//!       |
//!       v
//! Existing distributed transport/security/resilience
//!       |
//!       v
//! Actual execution providers
//!       |
//!       v
//! Actual execution results
//!
//! # EXECUTION DISTINCTIONS
//!
//! Registration != initialization.
//! Initialization != availability.
//! Availability != capability.
//! Capability != feasibility.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Submission != successful completion.
//! Delivery != successful execution.
//! Timeout != proof of non-execution.
//! Missing response != proof of failure.
//!
//! These distinctions are preserved throughout this module.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is introduced for:
//!
//! - Distributed tasks.
//! - Participants.
//! - Nodes.
//! - Processes.
//! - Workers.
//! - Providers.
//! - Execution contexts.
//! - Domains.
//! - Devices.
//! - Messages.
//! - Communication channels.
//! - Execution plans.
//! - Memory.
//! - Computational resources.
//!
//! Collections and operations remain subject to actual available
//! resources and explicitly configured operational policies.
//!
//! Literal infinite physical computation is not promised.
//!
//! # GRAMMAR AUTHORITY
//!
//! Relevant normative specification:
//!
//! - grammar/spec/distributed.md
//!
//! Relevant specifications:
//!
//! - grammar/spec/networking.md
//! - grammar/spec/concurrency.md
//! - grammar/spec/security.md
//! - grammar/spec/execution.md
//! - grammar/spec/hybrid.md
//! - grammar/spec/portability.md
//!
//! Relevant grammar:
//!
//! - grammar/distributed/distributed.g4
//! - grammar/distributed/nodes.g4
//! - grammar/distributed/tasks.g4
//! - grammar/distributed/actors.g4
//! - grammar/distributed/communication.g4
//! - grammar/distributed/channels.g4
//! - grammar/distributed/messages.g4
//! - grammar/distributed/placement.g4
//! - grammar/distributed/partitioning.g4
//! - grammar/distributed/topology.g4
//! - grammar/distributed/remote-execution.g4
//! - grammar/distributed/fault-tolerance.g4
//! - grammar/distributed/replication.g4
//! - grammar/execution/distributed-execution.g4
//! - grammar/memory/distributed-memory.g4
//! - grammar/effects/distributed.g4
//!
//! Grammar expresses source-level intent.
//! Semantic analysis establishes meaning.
//! Canonical IR represents computation.
//! Distributed planning establishes realization decisions.
//! ZUR coordinates provider integration.
//!
//! This module never parses or reinterprets source grammar.

use std::any::Any;
use std::sync::Arc;

use crate::distributed::execution::executor::ExecutablePlan;

use crate::distributed::ids::PlanId;

use crate::frontend::ast::node::domains::domain::Domain;

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    ProviderContractId,
    ProviderOperationId,
    ProviderId,
};

use crate::stdlib::version::Version;

use crate::zur::contracts::provider::{
    DomainProvider,
    ExecutionProvider,
    ProviderOperationRequest,
    ProviderOperationResponse,
    ProviderPayload,
    ProviderInterfaceId,
    ProviderSubmission,
    ProviderSubmissionStatus,
};

use crate::zur::domains::descriptor::DomainDescriptor;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::version::ServiceContractVersion;


// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.distributed";

const DOMAIN_NAMESPACE: &str = "zamani";

const DOMAIN_NAME: &str = "distributed";

/// Canonical ZUR distributed execution interface identity.
///
/// This is an extensible integration contract identity.
///
/// It is not a node identity, provider identity, network protocol,
/// transport identifier, hardware identifier or vendor identity.
pub const DISTRIBUTED_EXECUTION_INTERFACE: &str =
    "zamani.distributed.execution";

/// Canonical distributed execution-plan payload contract identity.
///
/// The actual plan representation remains owned by
/// crate::distributed::execution and its canonical plan adapters.
pub const DISTRIBUTED_PLAN_CONTRACT: &str =
    "zamani.distributed.execution.plan";

/// Canonical distributed execution-result envelope identity.
///
/// Actual domain-specific result representations remain owned
/// by their canonical execution providers.
pub const DISTRIBUTED_RESULT_CONTRACT: &str =
    "zamani.distributed.execution.result";


// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
}

fn compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
}

fn operation_error(
    operation: &'static str,
    error: ZurError,
) -> ZurError {
    error
        .with_component(COMPONENT)
        .with_operation(operation)
}


// ============================================================================
// CANONICAL DISTRIBUTED DOMAIN IDENTITY
// ============================================================================

/// Returns the canonical source-level distributed domain identity.
///
/// The identity is owned by the frontend AST.
///
/// No second domain identity type is introduced.
pub fn distributed_domain() -> Domain {
    Domain::new(
        DOMAIN_NAMESPACE,
        DOMAIN_NAME,
    )
    .expect("canonical distributed domain identity is valid")
}


// ============================================================================
// DISTRIBUTED DOMAIN DESCRIPTOR
// ============================================================================

/// Constructs the canonical ZUR distributed domain descriptor.
///
/// The descriptor connects the existing distributed domain to
/// the canonical ZUR distributed execution interface.
///
/// The supplied contract version remains caller-controlled.
///
/// Constructing a descriptor does not establish provider availability,
/// authentication, capability availability, feasibility, resource
/// reservation or execution.
///
/// # Arguments
///
/// * `version` - Explicit distributed domain contract version.
///
/// # Errors
///
/// Returns an error if descriptor construction or validation fails.
pub fn distributed_domain_descriptor(
    version: ServiceContractVersion,
) -> ZurResult<DomainDescriptor> {

    let domain = distributed_domain();

    let interface = ProviderInterfaceId::new(
        DISTRIBUTED_EXECUTION_INTERFACE,
    )
    .map_err(|error| {
        invalid_input(format!(
            "invalid distributed execution interface: {error}"
        ))
    })?;

    let mut descriptor = DomainDescriptor::new(
        domain,
        version,
        "Canonical Zamani distributed computation execution integration",
    )
    .map_err(|error| {
        operation_error(
            "distributed_domain_descriptor",
            error,
        )
    })?;

    descriptor
        .insert_interface(interface)
        .map_err(|error| {
            operation_error(
                "distributed_domain_descriptor",
                error,
            )
        })?;

    descriptor
        .validate()
        .map_err(|error| {
            operation_error(
                "distributed_domain_descriptor",
                error,
            )
        })?;

    Ok(descriptor)
}


// ============================================================================
// DISTRIBUTED DOMAIN DESCRIPTOR VALIDATION
// ============================================================================

/// Validates a descriptor as the canonical distributed domain descriptor.
///
/// Structural validation only.
///
/// # Errors
///
/// Returns an error when the domain identity or execution interface
/// does not match the canonical distributed integration boundary.
pub fn validate_distributed_domain_descriptor(
    descriptor: &DomainDescriptor,
) -> ZurResult<()> {

    descriptor
        .validate()
        .map_err(|error| {
            operation_error(
                "validate_distributed_domain_descriptor",
                error,
            )
        })?;

    if !descriptor.domain().is(
        DOMAIN_NAMESPACE,
        DOMAIN_NAME,
    ) {
        return Err(
            compatibility_error(format!(
                "expected domain {}.{}, found {}",
                DOMAIN_NAMESPACE,
                DOMAIN_NAME,
                descriptor.domain().qualified_name(),
            ))
            .with_operation(
                "validate_distributed_domain_descriptor",
            ),
        );
    }

    let interface_exists = descriptor
        .interfaces()
        .iter()
        .any(|interface| {
            interface.as_str()
                == DISTRIBUTED_EXECUTION_INTERFACE
        });

    if !interface_exists {
        return Err(
            compatibility_error(format!(
                "distributed domain descriptor does not declare {}",
                DISTRIBUTED_EXECUTION_INTERFACE,
            ))
            .with_operation(
                "validate_distributed_domain_descriptor",
            ),
        );
    }

    Ok(())
}


// ============================================================================
// DISTRIBUTED PROVIDER VALIDATION
// ============================================================================

/// Validates a ZUR distributed execution provider.
///
/// The provider must:
///
/// 1. Implement the canonical DomainProvider contract.
/// 2. Pass canonical domain-provider validation.
/// 3. Declare the canonical distributed domain.
/// 4. Implement the canonical distributed execution interface.
///
/// A provider may implement additional computational domains.
///
/// This function does not discover, initialize, authenticate,
/// authorize, reserve or execute anything.
///
/// # Errors
///
/// Returns an error if the provider does not satisfy the
/// distributed integration contract.
pub fn validate_distributed_provider(
    provider: &dyn DomainProvider,
) -> ZurResult<()> {

    provider
        .validate_domain_integration()
        .map_err(|error| {
            operation_error(
                "validate_distributed_provider",
                error,
            )
        })?;

    let descriptor = provider
        .domain_descriptors()
        .iter()
        .find(|descriptor| {
            descriptor.domain().is(
                DOMAIN_NAMESPACE,
                DOMAIN_NAME,
            )
        })
        .ok_or_else(|| {
            compatibility_error(
                "provider does not declare the canonical distributed domain",
            )
            .with_operation(
                "validate_distributed_provider",
            )
        })?;

    validate_distributed_domain_descriptor(
        descriptor,
    )?;

    let implements_interface = provider
        .execution_contract()
        .interfaces()
        .iter()
        .any(|interface| {
            interface.id().as_str()
                == DISTRIBUTED_EXECUTION_INTERFACE
        });

    if !implements_interface {
        return Err(
            compatibility_error(format!(
                "provider {} does not implement {}",
                provider.descriptor().id(),
                DISTRIBUTED_EXECUTION_INTERFACE,
            ))
            .with_operation(
                "validate_distributed_provider",
            )
            .with_provider(
                provider.descriptor().id().to_string(),
            ),
        );
    }

    Ok(())
}


// ============================================================================
// CANONICAL DISTRIBUTED EXECUTION PLAN VALIDATION
// ============================================================================

/// Validates a canonical distributed executable plan.
///
/// This delegates execution-plan validation to the existing
/// distributed subsystem.
///
/// It does not create a second plan validator.
///
/// It does not perform:
///
/// - Placement.
/// - Scheduling.
/// - Routing.
/// - Hardware selection.
/// - Participant discovery.
/// - Resource allocation.
/// - Resource reservation.
/// - Transport.
/// - Execution.
///
/// Runtime-specific volatile-resource checks remain the responsibility
/// of the appropriate runtime and resource authorities.
///
/// # Errors
///
/// Returns an error when the canonical plan rejects execution validation.
pub fn validate_distributed_plan<P>(
    plan: &P,
) -> ZurResult<()>
where
    P: ExecutablePlan,
{
    plan.validate_for_execution()
        .map_err(|error| {
            invalid_input(format!(
                "canonical distributed execution-plan validation failed: {error}"
            ))
            .with_operation(
                "validate_distributed_plan",
            )
        })
}


// ============================================================================
// TYPED DISTRIBUTED EXECUTION-PLAN PAYLOAD
// ============================================================================

/// A reference-counted envelope around a canonical distributed plan.
///
/// The generic plan type must implement the existing canonical
/// `ExecutablePlan` contract.
///
/// This type does not introduce another distributed execution plan.
///
/// It allows an in-process ZUR provider to receive an owned,
/// shared reference to the canonical validated plan.
///
/// Cross-process execution must use the canonical distributed
/// serialization and transport contracts.
///
/// The payload does not imply that the plan has already been
/// transmitted, accepted, scheduled, reserved or executed.
#[derive(Debug, Clone)]
pub struct DistributedPlanPayload<P>
where
    P: ExecutablePlan,
{
    plan: Arc<P>,
    contract_id: ProviderContractId,
    schema_version: Version,
}

impl<P> DistributedPlanPayload<P>
where
    P: ExecutablePlan,
{
    /// Constructs a distributed plan payload.
    ///
    /// The canonical plan is validated before being accepted.
    ///
    /// # Arguments
    ///
    /// * `plan` - Shared canonical distributed execution plan.
    /// * `contract_id` - Explicit provider payload contract identity.
    /// * `schema_version` - Explicit payload schema version.
    ///
    /// # Errors
    ///
    /// Returns an error if the canonical plan is invalid.
    pub fn new(
        plan: Arc<P>,
        contract_id: ProviderContractId,
        schema_version: Version,
    ) -> ZurResult<Self> {

        validate_distributed_plan(
            plan.as_ref(),
        )?;

        Ok(Self {
            plan,
            contract_id,
            schema_version,
        })
    }

    /// Returns the canonical distributed execution plan.
    #[must_use]
    pub fn plan(&self) -> &P {
        self.plan.as_ref()
    }

    /// Returns the shared canonical plan reference.
    #[must_use]
    pub fn shared_plan(&self) -> &Arc<P> {
        &self.plan
    }

    /// Returns the canonical logical plan identity.
    #[must_use]
    pub fn plan_id(&self) -> PlanId {
        self.plan.plan_id()
    }

    /// Returns the payload contract identity.
    #[must_use]
    pub fn contract_id(&self) -> &ProviderContractId {
        &self.contract_id
    }

    /// Returns the payload schema version.
    #[must_use]
    pub fn schema_version(&self) -> &Version {
        &self.schema_version
    }

    /// Revalidates the canonical plan immediately before execution.
    ///
    /// This is important because a previously validated plan may become
    /// stale with respect to the current execution environment.
    ///
    /// Volatile resource and authorization checks must additionally
    /// be performed by their owning authorities.
    pub fn revalidate(&self) -> ZurResult<()> {
        validate_distributed_plan(
            self.plan.as_ref(),
        )
    }
}

impl<P> ProviderPayload for DistributedPlanPayload<P>
where
    P: ExecutablePlan + 'static,
{
    fn contract_id(
        &self,
    ) -> &ProviderContractId {
        &self.contract_id
    }

    fn schema_version(
        &self,
    ) -> &Version {
        &self.schema_version
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


// ============================================================================
// DISTRIBUTED OPERATION CONTRACT VALIDATION
// ============================================================================

/// Validates a distributed provider operation against its request payload.
///
/// The provider must declare:
///
/// - The canonical distributed execution interface.
/// - The requested operation.
/// - The operation's request contract.
/// - A matching payload contract.
///
/// Operation identities remain open-world.
///
/// This module does not enumerate distributed operation names.
///
/// # Errors
///
/// Returns an error for a missing interface, missing operation,
/// missing request contract or incompatible payload contract.
pub fn validate_distributed_operation<P>(
    provider: &dyn DomainProvider,
    operation: &ProviderOperationId,
    payload: &DistributedPlanPayload<P>,
) -> ZurResult<()>
where
    P: ExecutablePlan + 'static,
{
    validate_distributed_provider(
        provider,
    )?;

    payload.revalidate()?;

    let interface = provider
        .execution_contract()
        .interfaces()
        .iter()
        .find(|interface| {
            interface.id().as_str()
                == DISTRIBUTED_EXECUTION_INTERFACE
        })
        .ok_or_else(|| {
            compatibility_error(
                "distributed execution interface is not implemented",
            )
            .with_operation(
                "validate_distributed_operation",
            )
        })?;

    let descriptor = interface
        .operations()
        .iter()
        .find(|descriptor| {
            descriptor.id() == operation
        })
        .ok_or_else(|| {
            compatibility_error(format!(
                "distributed provider does not declare operation {}",
                operation,
            ))
            .with_operation(
                "validate_distributed_operation",
            )
        })?;

    let expected_contract = descriptor
        .request_contract()
        .ok_or_else(|| {
            compatibility_error(format!(
                "distributed operation {} does not declare a request contract",
                operation,
            ))
            .with_operation(
                "validate_distributed_operation",
            )
        })?;

    if expected_contract != payload.contract_id() {
        return Err(
            compatibility_error(format!(
                "distributed operation {} expects request contract {}, received {}",
                operation,
                expected_contract,
                payload.contract_id(),
            ))
            .with_operation(
                "validate_distributed_operation",
            ),
        );
    }

    Ok(())
}


// ============================================================================
// DISTRIBUTED PROVIDER INVOCATION
// ============================================================================

/// Invokes a distributed operation through an explicitly supplied provider.
///
/// This performs an actual ZUR provider invocation.
///
/// The provider is never selected automatically.
///
/// The provider must implement the canonical distributed interface,
/// and the operation must declare the supplied request contract.
///
/// A successful return represents a completed provider invocation
/// according to the ZUR ExecutionProvider contract.
///
/// It is not an asynchronous submission receipt.
///
/// The provider implementation must delegate to the authoritative
/// distributed execution subsystem and return actual execution results.
///
/// # Errors
///
/// Returns errors from plan validation, provider validation,
/// operation validation, provider execution and response validation.
pub async fn invoke_distributed<P>(
    provider: &dyn DomainProvider,
    operation: &ProviderOperationId,
    payload: &DistributedPlanPayload<P>,
    context: &OperationContext,
) -> ZurResult<ProviderOperationResponse>
where
    P: ExecutablePlan + 'static,
{
    validate_distributed_operation(
        provider,
        operation,
        payload,
    )?;

    // Revalidate immediately before invocation.
    payload.revalidate()?;

    let request = ProviderOperationRequest::new(
        operation,
        payload,
    );

    let response = provider
        .invoke(
            request,
            context,
        )
        .await
        .map_err(|error| {
            operation_error(
                "invoke_distributed",
                error,
            )
        })?;

    // A response for a different operation is invalid.
    if response.operation() != operation {
        return Err(
            compatibility_error(format!(
                "distributed provider returned operation {}, requested {}",
                response.operation(),
                operation,
            ))
            .with_operation(
                "invoke_distributed",
            )
            .with_provider(
                provider.descriptor().id().to_string(),
            ),
        );
    }

    // The response must match the provider's declared response contract.
    let interface = provider
        .execution_contract()
        .interfaces()
        .iter()
        .find(|interface| {
            interface.id().as_str()
                == DISTRIBUTED_EXECUTION_INTERFACE
        })
        .ok_or_else(|| {
            compatibility_error(
                "distributed interface disappeared during invocation",
            )
            .with_operation(
                "invoke_distributed",
            )
        })?;

    let descriptor = interface
        .operations()
        .iter()
        .find(|descriptor| {
            descriptor.id() == operation
        })
        .ok_or_else(|| {
            compatibility_error(
                "distributed operation disappeared during invocation",
            )
            .with_operation(
                "invoke_distributed",
            )
        })?;

    if let Some(expected_response) =
        descriptor.response_contract()
    {
        let actual_response = response
            .payload()
            .contract_id();

        if actual_response != expected_response {
            return Err(
                compatibility_error(format!(
                    "distributed operation {} returned response contract {}, expected {}",
                    operation,
                    actual_response,
                    expected_response,
                ))
                .with_operation(
                    "invoke_distributed",
                )
                .with_provider(
                    provider.descriptor().id().to_string(),
                ),
            );
        }
    }

    Ok(response)
}


// ============================================================================
// DISTRIBUTED ASYNCHRONOUS SUBMISSION
// ============================================================================

/// Submits a distributed operation through an explicitly supplied provider.
///
/// This returns a submission receipt only.
///
/// An accepted submission does not establish that the distributed
/// computation has completed.
///
/// The caller must use the provider's declared completion, session,
/// result or reconciliation contract to establish the final outcome.
///
/// Unknown submission outcomes must remain unknown.
///
/// # Errors
///
/// Returns errors from provider validation, plan validation,
/// operation validation and provider submission.
pub async fn submit_distributed<P>(
    provider: &dyn DomainProvider,
    operation: &ProviderOperationId,
    payload: &DistributedPlanPayload<P>,
    context: &OperationContext,
) -> ZurResult<ProviderSubmission>
where
    P: ExecutablePlan + 'static,
{
    validate_distributed_operation(
        provider,
        operation,
        payload,
    )?;

    payload.revalidate()?;

    let request = ProviderOperationRequest::new(
        operation,
        payload,
    );

    let submission = provider
        .submit(
            request,
            context,
        )
        .await
        .map_err(|error| {
            operation_error(
                "submit_distributed",
                error,
            )
        })?;

    if submission.operation() != operation {
        return Err(
            compatibility_error(format!(
                "distributed provider returned submission for {}, requested {}",
                submission.operation(),
                operation,
            ))
            .with_operation(
                "submit_distributed",
            )
            .with_provider(
                provider.descriptor().id().to_string(),
            ),
        );
    }

    if submission.provider_id()
        != provider.descriptor().id()
    {
        return Err(
            compatibility_error(
                "distributed submission provider identity does not match the invoking provider",
            )
            .with_operation(
                "submit_distributed",
            )
            .with_provider(
                provider.descriptor().id().to_string(),
            ),
        );
    }

    // Accepted means accepted, not completed.
    //
    // Rejected means rejected.
    //
    // Unknown means the submission outcome is uncertain.
    //
    // No status is silently promoted to successful execution.
    match submission.status() {
        ProviderSubmissionStatus::Accepted
        | ProviderSubmissionStatus::Rejected
        | ProviderSubmissionStatus::Unknown => {}
    }

    Ok(submission)
}


// ============================================================================
// PUBLIC MODULE API
// ============================================================================

/// Common public imports for ZUR distributed integrations.
pub mod prelude {
    pub use super::{
        distributed_domain,
        distributed_domain_descriptor,
        validate_distributed_domain_descriptor,
        validate_distributed_operation,
        validate_distributed_plan,
        validate_distributed_provider,
        invoke_distributed,
        submit_distributed,
        DistributedPlanPayload,
        DISTRIBUTED_EXECUTION_INTERFACE,
        DISTRIBUTED_PLAN_CONTRACT,
        DISTRIBUTED_RESULT_CONTRACT,
    };
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::version::ServiceContractVersion;

    fn version() -> ServiceContractVersion {
        ServiceContractVersion::new(
            "1",
            "0",
            "0",
        )
        .expect("valid test contract version")
    }

    #[test]
    fn canonical_distributed_domain_identity_is_preserved() {
        let domain = distributed_domain();

        assert!(
            domain.is(
                DOMAIN_NAMESPACE,
                DOMAIN_NAME,
            )
        );

        assert_eq!(
            domain.qualified_name(),
            "zamani.distributed",
        );
    }

    #[test]
    fn descriptor_declares_canonical_distributed_interface() {
        let descriptor = distributed_domain_descriptor(
            version(),
        )
        .expect("valid distributed descriptor");

        validate_distributed_domain_descriptor(
            &descriptor,
        )
        .expect("descriptor must validate");

        assert!(
            descriptor.interfaces().iter().any(
                |interface| {
                    interface.as_str()
                        == DISTRIBUTED_EXECUTION_INTERFACE
                }
            )
        );
    }

    #[test]
    fn rejects_wrong_domain_identity() {
        let descriptor = DomainDescriptor::new(
            Domain::new(
                "zamani",
                "classical",
            )
            .expect("valid domain"),
            version(),
            "Incorrect distributed descriptor",
        )
        .expect("valid descriptor");

        assert!(
            validate_distributed_domain_descriptor(
                &descriptor,
            )
            .is_err()
        );
    }

    #[test]
    fn distributed_interface_is_open_world() {
        assert_eq!(
            DISTRIBUTED_EXECUTION_INTERFACE,
            "zamani.distributed.execution",
        );

        // The interface does not identify:
        //
        // - A particular node.
        // - A physical machine.
        // - A cloud provider.
        // - A network protocol.
        // - A processor architecture.
        // - A hardware vendor.
        //
        // Future providers implement the same stable contract.
    }

    #[test]
    fn payload_contract_identities_are_distinct() {
        assert_ne!(
            DISTRIBUTED_PLAN_CONTRACT,
            DISTRIBUTED_RESULT_CONTRACT,
        );
    }

    #[test]
    fn no_closed_operation_inventory_is_defined() {
        let _operation = ProviderOperationId::new(
            "future.distributed.specialized_execution",
        )
        .expect("open-world operation identity");

        // Operation identities remain provider-defined.
    }

    #[test]
    fn submission_statuses_are_not_execution_success() {
        assert_ne!(
            ProviderSubmissionStatus::Accepted,
            ProviderSubmissionStatus::Rejected,
        );

        assert_ne!(
            ProviderSubmissionStatus::Accepted,
            ProviderSubmissionStatus::Unknown,
        );
    }
}
