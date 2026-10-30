
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Transport
//!
//! Path: `src/distributed/transport/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## 1. PURPOSE
//!
//! This module is the public composition root for distributed
//! communication within the Zamani programming language.
//!
//! It provides a protocol-independent boundary through which
//! distributed execution components communicate with local,
//! remote, heterogeneous, and future execution providers.
//!
//! Its architectural objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! The transport subsystem must support different communication
//! implementations without changing the semantic meaning of
//! a Zamani program.
//!
//! ## 2. AUTHORITATIVE SPECIFICATIONS
//!
//! This module conforms to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/distributed.md`
//! - `grammar/spec/networking.md`
//! - `grammar/spec/hybrid.md`
//! - `grammar/spec/concurrency.md`
//! - `grammar/distributed/messages.g4`
//! - `grammar/distributed/communication.g4`
//! - `grammar/distributed/channels.g4`
//!
//! These specifications have different ownership responsibilities.
//!
//! Grammar defines source syntax.
//! Semantic analysis defines program meaning.
//! Canonical IR defines computational representation.
//! Distributed planning determines feasible communication routes.
//! Transport implements physical communication.
//! Execution coordinates actual work.
//! Runtime providers perform computation.
//!
//! Transport MUST NOT redefine any of those responsibilities.
//!
//! ## 3. MODULE OWNERSHIP
//!
//! This composition root owns:
//!
//! - Transport module composition.
//! - Public transport module declarations.
//! - Deliberate public API re-exports.
//! - Transport API organization.
//! - Transport architectural documentation.
//! - Module-level safety requirements.
//!
//! It does not implement network operations itself.
//!
//! ## 4. CHILD MODULE RESPONSIBILITIES
//!
//! codec.rs
//!
//! Owns:
//! - Message serialization.
//! - Message deserialization.
//! - Protocol version compatibility checks.
//! - Wire representation validation.
//! - Configurable encoding and decoding budgets.
//! - Safe message decoding.
//!
//! errors.rs
//!
//! Owns:
//! - TransportError.
//! - TransportErrorKind.
//! - TransportErrorContext.
//! - TransportFailurePhase.
//! - TransportFailureClassification.
//! - Conversion into canonical DistributedError.
//! - Delivery uncertainty classification.
//! - Preservation of underlying error causes.
//!
//! messages.rs
//!
//! Owns:
//! - ProtocolVersion.
//! - MessageKind.
//! - MessageRole.
//! - MessageEnvelope.
//! - WireMessageEnvelope.
//! - Message correlation.
//! - Reply relationships.
//! - Task and operation associations.
//! - Message extension metadata.
//! - Message structural validation.
//!
//! traits.rs
//!
//! Owns:
//! - TransportProvider.
//! - TransportConnection.
//! - TransportOperationContext.
//! - AuthenticationContext.
//! - AuthenticatedPeer.
//! - DeliveryConfirmation.
//! - SendReceipt.
//! - Connection lifecycle.
//! - Provider shutdown contracts.
//! - Asynchronous transport interfaces.
//!
//! Each child module remains the sole authority for its
//! respective types and implementation.
//!
//! ## 5. ARCHITECTURAL BOUNDARIES
//!
//! This module MUST NOT implement:
//!
//! - A second distributed scheduler.
//! - A second routing algorithm.
//! - A second placement algorithm.
//! - A second partitioning algorithm.
//! - A second message schema.
//! - A second serialization framework.
//! - A second authentication system.
//! - A second authorization system.
//! - A second resource-budget system.
//! - A second execution lifecycle.
//! - A second cancellation system.
//! - A second runtime.
//! - A second compiler or IR.
//! - Hardware allocation.
//! - Device drivers.
//! - Quantum error correction.
//! - Quantum pulse generation.
//!
//! The existing repository authorities remain responsible
//! for those operations.
//!
//! ## 6. DEPENDENCY CONTRACT
//!
//! Transport consumes the following existing authorities:
//!
//! `crate::distributed::config`
//!     Operational resource budgets.
//!
//! `crate::distributed::error`
//!     Canonical distributed errors and delivery state.
//!
//! `crate::distributed::ids`
//!     Logical message, participant, task and operation IDs.
//!
//! `crate::distributed::execution::cancellation`
//!     Cancellation signals and execution deadlines.
//!
//! `crate::distributed::model::communication`
//!     Logical communication requirements.
//!
//! `crate::distributed::planning::routing`
//!     Communication routing decisions.
//!
//! `crate::distributed::execution`
//!     Execution and transport coordination.
//!
//! Existing networking facilities
//!     Physical communication implementations.
//!
//! Existing security facilities
//!     Authentication and authorization.
//!
//! These dependencies must remain acyclic.
//!
//! Foundational types must not depend on this composition root.
//!
//! ## 7. TRANSPORT INDEPENDENCE
//!
//! Transport providers are injected through the interfaces
//! defined in traits.rs.
//!
//! This composition root does not select a specific:
//!
//! - Network protocol.
//! - Socket implementation.
//! - Communication medium.
//! - Operating system.
//! - Network interface.
//! - Port.
//! - Hardware platform.
//! - Cloud provider.
//! - Cluster implementation.
//!
//! New transport technologies must be integrable through
//! the existing provider contracts without modifying
//! distributed program semantics.
//!
//! A provider must explicitly declare the capabilities
//! and guarantees it actually supports.
//!
//! ## 8. MESSAGE AND CODEC INTEGRATION
//!
//! The authoritative message representation is:
//!
//! `messages::MessageEnvelope`
//!
//! The authoritative codec is:
//!
//! `codec::DistributedMessageCodec`
//!
//! Transport providers MUST use these existing contracts.
//!
//! They MUST NOT create private competing message formats
//! for ordinary distributed communication.
//!
//! All received messages must pass through the applicable
//! decoding, structural validation, authentication,
//! authorization and compatibility checks.
//!
//! Successfully decoding a message does not establish:
//!
//! - Sender authenticity.
//! - Authorization.
//! - Semantic validity.
//! - Successful delivery.
//! - Successful execution.
//! - Durable execution.
//! - Exactly-once execution.
//!
//! ## 9. DELIVERY AND EXECUTION SEMANTICS
//!
//! Transport delivery and application execution are
//! separate operations.
//!
//! A successful send receipt MUST NOT be interpreted
//! as proof that a remote computation completed.
//!
//! The transport implementation must preserve the
//! distinction between:
//!
//! - Message not dispatched.
//! - Message dispatched.
//! - Message rejected before delivery.
//! - Message delivered.
//! - Response received.
//! - Response lost.
//! - Delivery outcome unknown.
//! - Execution outcome unknown.
//!
//! A timeout after dispatch MUST NOT automatically
//! be interpreted as proof that the remote operation
//! failed to execute.
//!
//! Exactly-once execution MUST NOT be promised solely
//! because messages contain unique identifiers.
//!
//! Idempotency and durable execution guarantees belong
//! to the existing execution and resilience authorities.
//!
//! ## 10. CANCELLATION AND DEADLINES
//!
//! The authoritative cancellation contract belongs to:
//!
//! `crate::distributed::execution::cancellation`
//!
//! Transport operations consume:
//!
//! `traits::TransportOperationContext`
//!
//! They must respect:
//!
//! - Cancellation requests.
//! - Applicable monotonic deadlines.
//! - Configured resource budgets.
//! - Provider-specific operation capabilities.
//!
//! Cancellation is cooperative unless a provider
//! explicitly implements and documents stronger guarantees.
//!
//! Requesting cancellation does not prove that a
//! remote operation stopped.
//!
//! ## 11. SECURITY
//!
//! Transport integrates with the existing security authority.
//!
//! Authentication establishes peer identity.
//! Authorization determines whether an operation is permitted.
//!
//! These are separate decisions.
//!
//! The transport composition root does not store credentials,
//! implement cryptographic algorithms, or establish trust
//! merely because a participant has been discovered.
//!
//! Secrets MUST NOT appear in:
//!
//! - Message payloads intended as credentials.
//! - Debug representations.
//! - Error messages.
//! - Diagnostic output.
//! - Unprotected logs.
//!
//! ## 12. RESOURCE MANAGEMENT AND SCALABILITY
//!
//! This subsystem introduces no universal maximum for:
//!
//! - Participants.
//! - Nodes.
//! - Connections.
//! - Messages.
//! - Message kinds.
//! - Channels.
//! - Providers.
//! - Protocols.
//! - Workers.
//! - Communication routes.
//! - Concurrent operations.
//!
//! Operational budgets are owned by:
//!
//! `crate::distributed::config::ResourceBudgets`
//!
//! Codec-specific budgets are owned by:
//!
//! `codec::CodecLimits`
//!
//! Transport providers may additionally expose genuine
//! implementation-specific limits.
//!
//! Such limits must be reported explicitly.
//!
//! An unset operational budget does not imply unlimited
//! physical resources.
//!
//! The implementation must scale according to available
//! memory, processing capacity, communication bandwidth,
//! provider capabilities and actual execution resources.
//!
//! No arbitrary universal scalability ceiling is permitted.
//!
//! ## 13. FAILURE PROPAGATION
//!
//! Transport-specific failures originate from:
//!
//! `errors::TransportError`
//!
//! Canonical distributed failures originate from:
//!
//! `crate::distributed::error::DistributedError`
//!
//! Errors must preserve their original causes and
//! delivery-state information wherever available.
//!
//! Transport failures must not silently become successful
//! execution results.
//!
//! Retry decisions remain with the resilience authority.
//!
//! ## 14. SHUTDOWN AND RESOURCE LIFECYCLE
//!
//! Provider and connection lifecycle contracts belong
//! to traits.rs.
//!
//! A transport implementation must support the shutdown
//! behavior defined by its provider contract.
//!
//! Graceful shutdown must account for:
//!
//! - New-operation admission.
//! - Outstanding communication operations.
//! - In-flight requests.
//! - Pending responses.
//! - Cancellation.
//! - Connection closure.
//! - Provider-specific resource cleanup.
//!
//! A shutdown request must not falsely report that
//! outstanding remote operations have completed.
//!
//! ## 15. PUBLIC API POLICY
//!
//! The four child modules remain independently accessible.
//!
//! Their public types are selectively re-exported below.
//!
//! No wildcard re-exports are permitted.
//!
//! Child-module paths remain supported so that consumers
//! can access the owning module explicitly.
//!
//! ## 16. TESTING CONTRACT
//!
//! The child modules own their implementation tests.
//!
//! This composition root must remain testable without
//! requiring a live distributed cluster.
//!
//! Integration tests must eventually establish:
//!
//! - Module composition.
//! - Provider injection.
//! - Message compatibility.
//! - Codec compatibility.
//! - Delivery uncertainty.
//! - Request-response correlation.
//! - Cancellation propagation.
//! - Deadline enforcement.
//! - Authentication integration.
//! - Authorization integration.
//! - Resource-budget enforcement.
//! - Connection shutdown.
//! - Provider shutdown.
//! - Transport-provider replacement.
//! - Cross-domain communication.
//! - Operation failure propagation.
//! - Increasing workloads subject to available resources.
//!
//! ## 17. SAFETY
//!
//! Unsafe Rust is prohibited throughout this module tree.
//!
//! The prohibition is enforced through:
//!
//! `#![forbid(unsafe_code)]`
//!
//! ## 18. COMPLETION CRITERIA
//!
//! This file is complete when:
//!
//! - All four child modules are declared.
//! - Every declared child module exists.
//! - All public re-exports resolve.
//! - No wildcard exports exist.
//! - Existing authoritative types are reused.
//! - Module ownership is unambiguous.
//! - Dependency direction is acyclic.
//! - No transport protocol is hard-coded.
//! - No arbitrary scalability ceiling is introduced.
//! - No unsafe Rust is permitted.
//! - Existing child-module tests remain available.
//! - The parent distributed module exposes this module.
//! - Repository integration checks pass.

/// Safe, resource-aware message serialization and decoding.
pub mod codec;

/// Transport-specific errors and delivery failure classification.
pub mod errors;

/// Versioned transport envelopes and wire-message representations.
pub mod messages;

/// Injectable asynchronous transport provider interfaces.
pub mod traits;

// =============================================================================
// CODEC PUBLIC API
// =============================================================================

pub use self::codec::{
    CodecLimits,
    DistributedMessageCodec,
    ProtocolCompatibility,
};

// =============================================================================
// ERROR PUBLIC API
// =============================================================================

pub use self::errors::{
    is_transport_error,
    normalize_transport_result,
    classify_transport_result,
    TransportError,
    TransportErrorContext,
    TransportErrorKind,
    TransportErrorSource,
    TransportFailureClassification,
    TransportFailurePhase,
    TransportResult,
};

// =============================================================================
// MESSAGE PUBLIC API
// =============================================================================

pub use self::messages::{
    MessageEnvelope,
    MessageEnvelopeParts,
    MessageExtensions,
    MessageKind,
    MessageRole,
    ProtocolVersion,
    WireMessageEnvelope,
};

// =============================================================================
// TRANSPORT INTERFACE PUBLIC API
// =============================================================================

pub use self::traits::{
    control_error,
    is_event,
    is_request,
    validate_outgoing_message,
    validate_response,
    AuthenticatedPeer,
    AuthenticationContext,
    ConnectionCloseReport,
    ConnectionState,
    DeliveryConfirmation,
    SendReceipt,
    ShutdownMode,
    TransportConnection,
    TransportFuture,
    TransportLimits,
    TransportOperationContext,
    TransportProvider,
    TransportShutdownReport,
};
