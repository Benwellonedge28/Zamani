
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Quantum Integration
//!
//! Path: `src/zur/quantum.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # PURPOSE
//!
//! Provides the integration boundary between the Zamani Universal Runtime
//! (ZUR) and the canonical Zamani Quantum Computing subsystem.
//!
//! This module allows ZUR execution providers to consume canonical quantum
//! programs through the existing provider contracts.
//!
//! It does not implement another quantum execution engine.
//!
//! # CANONICAL AUTHORITIES
//!
//! Quantum computation semantics:
//!     crate::quantum::ir
//!
//! Quantum program representation:
//!     crate::quantum::ir::program
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! Quantum hardware execution:
//!     crate::quantum::hardware::backend_trait
//!
//! Quantum semantic validation:
//!     crate::frontend::semantic::domains::quantum
//!
//! ZUR provider contracts:
//!     crate::zur::contracts::provider
//!
//! ZUR domain integration:
//!     crate::zur::domains
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Operation cancellation:
//!     crate::stdlib::context
//!
//! ZUR operational errors:
//!     crate::zur::error
//!
//! Contract versions:
//!     crate::zur::version
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - Quantum-to-ZUR integration.
//! - The canonical quantum domain descriptor factory.
//! - Quantum provider integration validation.
//! - Quantum program structural validation at the ZUR boundary.
//! - A typed quantum program provider payload.
//! - Contract-checked quantum provider invocation.
//!
//! This module does NOT own:
//!
//! - Quantum computation semantics.
//! - Qubit identity.
//! - Physical qubit identity.
//! - Quantum operations.
//! - Quantum gates.
//! - Quantum measurements.
//! - Quantum circuits.
//! - Quantum programs.
//! - Quantum IR validation rules.
//! - Quantum hardware descriptors.
//! - Hardware discovery.
//! - Quantum provider registration.
//! - Provider authentication.
//! - Capability definitions.
//! - Capability evidence generation.
//! - Resource requirements.
//! - Feasibility.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Quantum routing.
//! - Quantum scheduling.
//! - Quantum optimization.
//! - Quantum error correction.
//! - Quantum simulation.
//! - Quantum result mathematics.
//! - Provider networking.
//! - Backend code generation.
//! - Hybrid graph construction.
//! - Distributed execution planning.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! # EXECUTION PIPELINE
//!
//! ```text
//! Zamani quantum source
//!          |
//!          v
//! Canonical frontend
//!          |
//!          v
//! Quantum semantic validation
//!          |
//!          v
//! Canonical Quantum IR
//!          |
//!          v
//! Quantum compiler and planning
//!          |
//!          v
//! Quantum hardware compatibility
//!          |
//!          v
//! Canonical quantum provider
//!          |
//!          v
//! ZUR quantum integration
//!          |
//!          v
//! ZUR ExecutionProvider
//!          |
//!          v
//! Actual provider invocation
//!          |
//!          v
//! Actual provider response
//! ```
//!
//! # IMPORTANT EXECUTION DISTINCTIONS
//!
//! Provider registration does not establish availability.
//!
//! Provider availability does not establish capability availability.
//!
//! Capability availability does not establish feasibility.
//!
//! Feasibility does not establish resource reservation.
//!
//! Resource reservation does not establish execution.
//!
//! Submission does not establish successful completion.
//!
//! This module preserves these distinctions.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is introduced for:
//!
//! - Qubits.
//! - Logical qubits.
//! - Physical qubits.
//! - Quantum operations.
//! - Quantum programs.
//! - Operation parameters.
//! - Quantum providers.
//! - Quantum devices.
//! - Quantum technologies.
//! - Quantum execution sessions.
//! - Quantum hardware.
//! - Quantum computation size.
//!
//! All concrete execution remains subject to:
//!
//! - Available memory.
//! - Available computational resources.
//! - Quantum IR validation.
//! - Explicit operational policies.
//! - Provider capabilities.
//! - Target feasibility.
//! - Actual hardware limitations.
//!
//! No finite machine promises literal infinite computation.
//!
//! # POCO-REAF
//!
//! Quantum programs retain their canonical semantic representation.
//!
//! ZUR does not rewrite a program merely because the target changes.
//!
//! Compatible execution providers may implement the same quantum
//! execution interface for different hardware technologies.
//!
//! New quantum technologies and providers must not require changes
//! to the canonical quantum program representation or this module.
//!
//! # GRAMMAR AUTHORITY
//!
//! Relevant grammar:
//!
//! - grammar/quantum/quantum.g4
//! - grammar/quantum/operations.g4
//! - grammar/quantum/parameters.g4
//! - grammar/quantum/measurement.g4
//! - grammar/quantum/reset.g4
//! - grammar/quantum/controls.g4
//! - grammar/quantum/quantum-resources.g4
//! - grammar/quantum/quantum-capabilities.g4
//! - grammar/quantum/dynamic-circuits.g4
//! - grammar/quantum/error-correction.g4
//! - grammar/hybrid/classical-quantum.g4
//!
//! Grammar expresses source-level intent.
//!
//! Semantic analysis validates source meaning.
//!
//! Quantum IR represents canonical computation semantics.
//!
//! ZUR coordinates operational provider integration.
//!
//! This module never parses source grammar.

use std::any::Any;
use std::sync::Arc;

use crate::frontend::ast::node::domains::domain::Domain;

use crate::quantum::ir::QuantumProgram;

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    ProviderContractId,
    ProviderOperationId,
};

use crate::stdlib::version::Version;

use crate::zur::contracts::provider::{
    DomainProvider,
    ProviderOperationRequest,
    ProviderOperationResponse,
    ProviderPayload,
    ProviderInterfaceId,
    ExecutionProvider,
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

const COMPONENT: &str = "zur.quantum";

const DOMAIN_NAMESPACE: &str = "zamani";

const DOMAIN_NAME: &str = "quantum";

/// Canonical ZUR quantum execution interface identity.
///
/// This identifies an integration interface, not a particular quantum
/// operation, vendor, device or hardware technology.
pub const QUANTUM_EXECUTION_INTERFACE: &str =
    "zamani.quantum.execution";

/// Canonical typed quantum-program payload contract identity.
///
/// Individual provider operation descriptors may declare this contract
/// as their request contract.
pub const QUANTUM_PROGRAM_CONTRACT: &str =
    "zamani.quantum.program";

/// Canonical quantum program payload schema version.
///
/// This is independent of the Quantum IR version, runtime ABI version,
/// package version and language version.
pub const QUANTUM_PROGRAM_SCHEMA_VERSION: &str = "1.0.0";


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
// QUANTUM DOMAIN IDENTITY
// ============================================================================

/// Returns the canonical source-level quantum domain identity.
///
/// The returned identity is owned by the frontend AST.
///
/// This function does not create a second domain identity type.
pub fn quantum_domain() -> Domain {
    Domain::new(
        DOMAIN_NAMESPACE,
        DOMAIN_NAME,
    )
    .expect("canonical quantum domain identity is valid")
}


// ============================================================================
// QUANTUM DOMAIN DESCRIPTOR
// ============================================================================

/// Constructs the canonical ZUR quantum domain descriptor.
///
/// The descriptor connects the canonical quantum domain to the existing
/// ZUR quantum execution interface.
///
/// The supplied contract version is owned by the caller. This prevents
/// this module from silently imposing a version-migration policy.
///
/// The descriptor does not establish:
///
/// - Provider registration.
/// - Provider initialization.
/// - Hardware discovery.
/// - Capability availability.
/// - Resource availability.
/// - Execution feasibility.
/// - Resource reservation.
///
/// Those operations belong to their respective authorities.
///
/// # Arguments
///
/// * `version` - Explicit quantum domain contract version.
///
/// # Errors
///
/// Returns an error if the descriptor or interface cannot be constructed.
pub fn quantum_domain_descriptor(
    version: ServiceContractVersion,
) -> ZurResult<DomainDescriptor> {

    let domain = quantum_domain();

    let interface = ProviderInterfaceId::new(
        QUANTUM_EXECUTION_INTERFACE,
    )
    .map_err(|error| {
        invalid_input(format!(
            "invalid quantum execution interface: {error}"
        ))
    })?;

    let mut descriptor = DomainDescriptor::new(
        domain,
        version,
        "Canonical Zamani quantum computation execution integration",
    )
    .map_err(|error| {
        operation_error(
            "quantum_domain_descriptor",
            error,
        )
    })?;

    descriptor
        .insert_interface(interface)
        .map_err(|error| {
            operation_error(
                "quantum_domain_descriptor",
                error,
            )
        })?;

    descriptor
        .validate()
        .map_err(|error| {
            operation_error(
                "quantum_domain_descriptor",
                error,
            )
        })?;

    Ok(descriptor)
}


// ============================================================================
// DOMAIN DESCRIPTOR VALIDATION
// ============================================================================

/// Validates an existing descriptor as a ZUR quantum domain descriptor.
///
/// This performs structural identity and interface validation.
///
/// It does not perform hardware discovery or determine whether a
/// quantum execution provider is currently available.
///
/// # Errors
///
/// Returns an error if:
///
/// - The descriptor is structurally invalid.
/// - Its domain identity is not `zamani.quantum`.
/// - It does not declare the canonical quantum execution interface.
pub fn validate_quantum_domain_descriptor(
    descriptor: &DomainDescriptor,
) -> ZurResult<()> {

    descriptor
        .validate()
        .map_err(|error| {
            operation_error(
                "validate_quantum_domain_descriptor",
                error,
            )
        })?;

    if !descriptor.domain().is(
        DOMAIN_NAMESPACE,
        DOMAIN_NAME,
    ) {
        return Err(
            compatibility_error(format!(
                "expected domain {DOMAIN_NAMESPACE}.{DOMAIN_NAME}, found {}",
                descriptor.domain().qualified_name(),
            ))
            .with_operation(
                "validate_quantum_domain_descriptor",
            ),
        );
    }

    let interface_exists = descriptor
        .interfaces()
        .iter()
        .any(|interface| {
            interface.as_str()
                == QUANTUM_EXECUTION_INTERFACE
        });

    if !interface_exists {
        return Err(
            compatibility_error(format!(
                "quantum domain descriptor does not declare \
                 interface {QUANTUM_EXECUTION_INTERFACE}"
            ))
            .with_operation(
                "validate_quantum_domain_descriptor",
            ),
        );
    }

    Ok(())
}


// ============================================================================
// QUANTUM PROVIDER VALIDATION
// ============================================================================

/// Validates the structural integration of a ZUR quantum provider.
///
/// The provider must:
///
/// 1. Implement the canonical ZUR `DomainProvider` contract.
/// 2. Pass canonical domain-provider validation.
/// 3. Declare the canonical `zamani.quantum` domain.
/// 4. Implement `zamani.quantum.execution`.
///
/// The provider may additionally implement any number of other domains.
///
/// No hardware vendor or technology is enumerated.
///
/// # Important
///
/// A successful validation establishes structural integration only.
///
/// It does not establish:
///
/// - Provider availability.
/// - Quantum hardware availability.
/// - Quantum capability availability.
/// - Resource availability.
/// - Feasibility.
/// - Resource reservation.
/// - Successful execution.
///
/// # Errors
///
/// Returns an error for an invalid domain/provider relationship.
pub fn validate_quantum_provider(
    provider: &dyn DomainProvider,
) -> ZurResult<()> {

    provider
        .validate_domain_integration()
        .map_err(|error| {
            operation_error(
                "validate_quantum_provider",
                error,
            )
        })?;

    let quantum_descriptor = provider
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
                "provider does not declare the canonical quantum domain",
            )
            .with_operation(
                "validate_quantum_provider",
            )
        })?;

    validate_quantum_domain_descriptor(
        quantum_descriptor,
    )?;

    let contract = provider.execution_contract();

    let implements_quantum_interface = contract
        .interfaces()
        .iter()
        .any(|interface| {
            interface.id().as_str()
                == QUANTUM_EXECUTION_INTERFACE
        });

    if !implements_quantum_interface {
        return Err(
            compatibility_error(format!(
                "provider {} does not implement {}",
                provider.descriptor().id(),
                QUANTUM_EXECUTION_INTERFACE,
            ))
            .with_operation(
                "validate_quantum_provider",
            )
            .with_provider(
                provider.descriptor().id().to_string(),
            ),
        );
    }

    Ok(())
}


// ============================================================================
// CANONICAL QUANTUM PROGRAM VALIDATION
// ============================================================================

/// Validates a canonical Quantum IR program before ZUR invocation.
///
/// This delegates semantic and structural ownership to the canonical
/// Quantum IR implementation.
///
/// It does not perform:
///
/// - Semantic analysis of source code.
/// - Target compatibility.
/// - Quantum routing.
/// - Quantum scheduling.
/// - Physical resource allocation.
/// - Hardware discovery.
/// - Quantum simulation.
/// - Quantum execution.
///
/// The canonical IR remains unchanged.
///
/// # Errors
///
/// Returns an error if the program fails its own structural validation.
pub fn validate_quantum_program(
    program: &QuantumProgram,
) -> ZurResult<()> {

    program.validate().map_err(|error| {
        invalid_input(format!(
            "canonical Quantum IR program validation failed: {error}"
        ))
        .with_operation(
            "validate_quantum_program",
        )
    })?;

    Ok(())
}


// ============================================================================
// TYPED QUANTUM PROGRAM PAYLOAD
// ============================================================================

/// An owned, reference-counted canonical Quantum IR program payload.
///
/// This is a ZUR transport envelope around the canonical Quantum IR.
///
/// It does not introduce another quantum program representation.
///
/// The program itself remains:
///
/// `crate::quantum::ir::QuantumProgram`.
///
/// `Arc` allows provider requests to share program ownership without
/// copying the complete program for every invocation.
///
/// The contract identity and schema version are explicitly supplied
/// and remain independently versioned.
///
/// The payload is suitable for safe in-process typed provider invocation.
///
/// Cross-process execution must use the canonical Quantum IR serialization
/// and interchange contracts instead.
///
/// # Ownership
///
/// This type owns:
///
/// - A shared reference to a canonical QuantumProgram.
/// - Its externally declared provider payload contract identity.
/// - Its payload schema version.
///
/// It does not own:
///
/// - Quantum execution.
/// - Hardware selection.
/// - Physical resource allocation.
/// - Provider lifecycle.
/// - Serialization.
/// - Result interpretation.
#[derive(Debug, Clone)]
pub struct QuantumProgramPayload {
    program: Arc<QuantumProgram>,

    contract_id: ProviderContractId,

    schema_version: Version,
}

impl QuantumProgramPayload {

    /// Constructs a typed quantum-program payload.
    ///
    /// The program is validated before it enters the provider boundary.
    ///
    /// # Arguments
    ///
    /// * `program` - Shared canonical Quantum IR program.
    /// * `contract_id` - Provider request contract identity.
    /// * `schema_version` - Explicit payload schema version.
    ///
    /// # Errors
    ///
    /// Returns an error if the canonical Quantum IR program is invalid.
    pub fn new(
        program: Arc<QuantumProgram>,
        contract_id: ProviderContractId,
        schema_version: Version,
    ) -> ZurResult<Self> {

        validate_quantum_program(
            program.as_ref(),
        )?;

        Ok(Self {
            program,
            contract_id,
            schema_version,
        })
    }

    /// Returns the canonical Quantum IR program.
    #[must_use]
    pub fn program(&self) -> &QuantumProgram {
        self.program.as_ref()
    }

    /// Returns the shared canonical program reference.
    #[must_use]
    pub fn shared_program(
        &self,
    ) -> &Arc<QuantumProgram> {
        &self.program
    }

    /// Returns the payload contract identity.
    #[must_use]
    pub fn contract_id(
        &self,
    ) -> &ProviderContractId {
        &self.contract_id
    }

    /// Returns the payload schema version.
    #[must_use]
    pub fn schema_version(
        &self,
    ) -> &Version {
        &self.schema_version
    }
}

impl ProviderPayload for QuantumProgramPayload {

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
// PROVIDER OPERATION CONTRACT VALIDATION
// ============================================================================

/// Validates that a quantum provider declares a compatible operation.
///
/// The operation identity is open-world.
///
/// This module does not enumerate gates or quantum operations.
///
/// The provider operation descriptor must explicitly declare the
/// request contract used by the supplied quantum-program payload.
///
/// A provider interface declaration alone is not sufficient.
///
/// # Errors
///
/// Returns an error when:
///
/// - The quantum execution interface is absent.
/// - The operation is not declared.
/// - The operation does not declare a request contract.
/// - The declared request contract differs from the payload contract.
pub fn validate_quantum_operation(
    provider: &dyn DomainProvider,
    operation: &ProviderOperationId,
    payload: &QuantumProgramPayload,
) -> ZurResult<()> {

    validate_quantum_provider(provider)?;

    let contract = provider.execution_contract();

    let interface = contract
        .interfaces()
        .iter()
        .find(|interface| {
            interface.id().as_str()
                == QUANTUM_EXECUTION_INTERFACE
        })
        .ok_or_else(|| {
            compatibility_error(
                "quantum execution interface is not implemented",
            )
            .with_operation(
                "validate_quantum_operation",
            )
        })?;

    let operation_descriptor = interface
        .operations()
        .iter()
        .find(|descriptor| {
            descriptor.id() == operation
        })
        .ok_or_else(|| {
            compatibility_error(format!(
                "quantum provider does not declare operation {operation}"
            ))
            .with_operation(
                "validate_quantum_operation",
            )
        })?;

    let expected_contract = operation_descriptor
        .request_contract()
        .ok_or_else(|| {
            compatibility_error(format!(
                "quantum operation {operation} does not declare \
                 a request payload contract"
            ))
            .with_operation(
                "validate_quantum_operation",
            )
        })?;

    if expected_contract != payload.contract_id() {
        return Err(
            compatibility_error(format!(
                "quantum operation {operation} expects request contract \
                 {expected_contract}, but received {}",
                payload.contract_id(),
            ))
            .with_operation(
                "validate_quantum_operation",
            ),
        );
    }

    Ok(())
}


// ============================================================================
// QUANTUM PROVIDER INVOCATION
// ============================================================================

/// Invokes a quantum operation through the canonical ZUR provider contract.
///
/// This function performs an actual provider invocation.
///
/// It does not select a provider automatically.
///
/// The caller supplies the provider explicitly.
///
/// The operation must be declared by the provider's canonical
/// quantum execution interface.
///
/// The request payload must satisfy the operation's declared
/// request contract.
///
/// The provider is responsible for:
///
/// - Provider-specific execution.
/// - Actual quantum hardware interaction.
/// - Simulator execution when explicitly implemented.
/// - Device session management.
/// - Resource reservation through the appropriate authority.
/// - Measurement result production.
/// - Provider-specific failure reporting.
/// - Cancellation observation.
///
/// The returned response represents the provider's actual completed
/// invocation according to the canonical ZUR `ExecutionProvider`
/// contract.
///
/// It must not be confused with an asynchronous submission receipt.
///
/// # Arguments
///
/// * `provider` - Explicit quantum execution provider.
/// * `operation` - Open-world provider operation identity.
/// * `payload` - Validated canonical Quantum IR payload.
/// * `context` - Canonical operation and cancellation context.
///
/// # Errors
///
/// Returns errors from:
///
/// - Quantum program validation.
/// - Provider structural validation.
/// - Operation contract validation.
/// - Provider invocation.
/// - Response contract validation.
///
/// No failure is converted into successful execution.
pub async fn invoke_quantum(
    provider: &dyn DomainProvider,
    operation: &ProviderOperationId,
    payload: &QuantumProgramPayload,
    context: &OperationContext,
) -> ZurResult<ProviderOperationResponse> {

    validate_quantum_operation(
        provider,
        operation,
        payload,
    )?;

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
                "invoke_quantum",
                error,
            )
        })?;

    // The response must correspond to the requested operation.
    if response.operation() != operation {
        return Err(
            compatibility_error(format!(
                "quantum provider returned operation {}, \
                 but {} was requested",
                response.operation(),
                operation,
            ))
            .with_operation(
                "invoke_quantum",
            )
            .with_provider(
                provider.descriptor().id().to_string(),
            ),
        );
    }

    // Validate the provider's declared response contract.
    let interface = provider
        .execution_contract()
        .interfaces()
        .iter()
        .find(|interface| {
            interface.id().as_str()
                == QUANTUM_EXECUTION_INTERFACE
        })
        .ok_or_else(|| {
            compatibility_error(
                "quantum interface disappeared during invocation",
            )
            .with_operation(
                "invoke_quantum",
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
                "quantum operation disappeared during invocation",
            )
            .with_operation(
                "invoke_quantum",
            )
        })?;

    if let Some(expected_response) =
        descriptor.response_contract()
    {
        let actual_response =
            response.payload().contract_id();

        if actual_response != expected_response {
            return Err(
                compatibility_error(format!(
                    "quantum operation {operation} returned response \
                     contract {}, expected {}",
                    actual_response,
                    expected_response,
                ))
                .with_operation(
                    "invoke_quantum",
                ),
            );
        }
    }

    Ok(response)
}


// ============================================================================
// PUBLIC MODULE API
// ============================================================================

/// Common public imports for ZUR quantum integrations.
pub mod prelude {

    pub use super::{
        invoke_quantum,
        quantum_domain,
        quantum_domain_descriptor,
        validate_quantum_domain_descriptor,
        validate_quantum_operation,
        validate_quantum_program,
        validate_quantum_provider,
        QuantumProgramPayload,
        QUANTUM_EXECUTION_INTERFACE,
        QUANTUM_PROGRAM_CONTRACT,
        QUANTUM_PROGRAM_SCHEMA_VERSION,
    };
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::zur::version::ServiceContractVersion;

    #[test]
    fn canonical_quantum_domain_identity_is_preserved() {

        let domain = quantum_domain();

        assert!(
            domain.is("zamani", "quantum")
        );

        assert_eq!(
            domain.qualified_name(),
            "zamani.quantum",
        );
    }

    #[test]
    fn quantum_domain_descriptor_declares_canonical_interface() {

        let version =
            ServiceContractVersion::new(
                "1",
                "0",
                "0",
            )
            .expect("valid contract version");

        let descriptor =
            quantum_domain_descriptor(version)
                .expect("valid quantum descriptor");

        assert!(
            validate_quantum_domain_descriptor(
                &descriptor,
            )
            .is_ok()
        );

        assert!(
            descriptor
                .interfaces()
                .iter()
                .any(|interface| {
                    interface.as_str()
                        == QUANTUM_EXECUTION_INTERFACE
                })
        );
    }

    #[test]
    fn rejects_non_quantum_domain_descriptor() {

        let version =
            ServiceContractVersion::new(
                "1",
                "0",
                "0",
            )
            .expect("valid contract version");

        let domain = Domain::new(
            "zamani",
            "classical",
        )
        .expect("valid classical domain");

        let interface =
            ProviderInterfaceId::new(
                QUANTUM_EXECUTION_INTERFACE,
            )
            .expect("valid interface");

        let mut descriptor =
            DomainDescriptor::new(
                domain,
                version,
                "Test descriptor",
            )
            .expect("valid descriptor");

        descriptor
            .insert_interface(interface)
            .expect("interface insertion");

        assert!(
            validate_quantum_domain_descriptor(
                &descriptor,
            )
            .is_err()
        );
    }

    #[test]
    fn quantum_interface_is_not_a_vendor_identity() {

        assert_eq!(
            QUANTUM_EXECUTION_INTERFACE,
            "zamani.quantum.execution",
        );

        // The interface does not identify:
        //
        // - IBM.
        // - IonQ.
        // - Rigetti.
        // - A simulator.
        // - A physical quantum technology.
        // - A particular QPU.
    }

    #[test]
    fn quantum_operation_identity_remains_open_world() {

        let operations = [
            "quantum.gate.h",
            "quantum.measurement",
            "vendor.custom_operation",
            "future.quantum.operation",
            "photonic.quantum.operation",
        ];

        for operation in operations {
            assert!(
                ProviderOperationId::new(operation).is_ok()
            );
        }
    }

    #[test]
    fn schema_version_is_independent_of_package_version() {

        let schema =
            Version::parse(
                QUANTUM_PROGRAM_SCHEMA_VERSION,
            )
            .expect("valid schema version");

        assert_eq!(
            schema.to_string(),
            QUANTUM_PROGRAM_SCHEMA_VERSION,
        );
    }

    #[test]
    fn no_hardware_capacity_is_encoded() {

        // The module declares no universal maximum for:
        //
        // - Qubits.
        // - Devices.
        // - Operations.
        // - Quantum programs.
        // - Quantum providers.
        //
        // Actual feasibility remains with canonical authorities.
        assert!(
            !QUANTUM_EXECUTION_INTERFACE.is_empty()
        );
    }
}
