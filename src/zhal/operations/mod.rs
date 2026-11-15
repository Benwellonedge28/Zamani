
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Operations
//!
//! Path: `src/zhal/operations/mod.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the public composition root for the ZHAL hardware
//! operation subsystem.
//!
//! This module connects the independently implemented operation
//! contracts, submission, completion and cancellation components.
//!
//! It provides a common hardware-operation lifecycle boundary
//! without introducing another execution engine.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Hardware operation subsystem composition.
//! - Public operation API exposure.
//! - Stable module paths.
//! - Cross-component API organization.
//! - Operation subsystem documentation.
//!
//! It does not own:
//!
//! - Provider identity.
//! - Provider registration.
//! - Provider lifecycle.
//! - Capability identity.
//! - Hardware identity.
//! - Operation-instance identity.
//! - Semantic analysis.
//! - Source grammar.
//! - Operation payload semantics.
//! - Hardware discovery.
//! - Device selection.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Scheduling.
//! - Routing.
//! - Backend selection.
//! - Domain-specific execution.
//! - Distributed execution.
//! - Hybrid execution.
//! - Runtime orchestration.
//!
//! These responsibilities remain with their established
//! repository authorities.
//!
//! # Component architecture
//!
//! The operation subsystem consists of four independently
//! implemented components.
//!
//! 1. contract:
//!    Defines operation compatibility and lifecycle guarantees.
//!
//! 2. submission:
//!    Defines operation submission and provider acceptance.
//!
//! 3. completion:
//!    Defines completion records, outcomes and provenance.
//!
//! 4. cancellation:
//!    Defines provider-aware cancellation requests and receipts.
//!
//! These components have separate ownership and must not
//! be merged into a monolithic operation implementation.
//!
//! # Canonical repository authorities
//!
//! Provider contracts:
//!     crate::stdlib::provider
//!
//! Capability contracts:
//!     crate::stdlib::capability
//!
//! Version representation:
//!     crate::stdlib::version
//!
//! Hardware operation identity:
//!     crate::zhal::identity
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Runtime orchestration:
//!     crate::zur
//!
//! Quantum execution:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! # Operation lifecycle
//!
//! The common operation lifecycle is:
//!
//!     Operation Contract
//!           |
//!           v
//!       Submission
//!           |
//!           v
//!       Acceptance
//!           |
//!           v
//!       Execution
//!           |
//!           +----------------------+
//!           |                      |
//!           v                      v
//!       Completion             Cancellation
//!           |                      |
//!           v                      v
//!       Result / Failure       Cancellation Outcome
//!
//! This diagram describes logical relationships, not a mandatory
//! execution sequence.
//!
//! Cancellation may be requested while an operation is executing.
//!
//! An accepted cancellation request does not establish that
//! physical execution has stopped.
//!
//! A provider may report an indeterminate outcome when it cannot
//! establish the final state of an operation.
//!
//! # Critical operation invariants
//!
//! The subsystem must preserve the following invariants:
//!
//! - Operation contracts and operation instances are distinct.
//! - Provider identities use the canonical standard-library type.
//! - Operation identities are validated.
//! - Submission acceptance is not operation completion.
//! - Partial acceptance is not complete acceptance.
//! - Indeterminate acceptance is not rejection.
//! - Unknown completion is not failure.
//! - Partial completion remains explicitly represented.
//! - Cancellation requests are not cancellation confirmation.
//! - Provider receipts must match the requested operation.
//! - Operation payload semantics remain domain-owned.
//! - Retry safety must never be inferred.
//! - Resource availability does not imply resource reservation.
//! - Compatibility does not imply authorization.
//! - Provider registration does not imply provider trust.
//!
//! # POCO-REAF
//!
//! Operation contracts are independent of:
//!
//! - Processor count.
//! - Processor architecture.
//! - Memory capacity.
//! - Device count.
//! - Accelerator count.
//! - Qubit count.
//! - Register width.
//! - Distributed participant count.
//! - Physical topology.
//! - Hardware vendor.
//! - Physical location.
//! - Computational domain.
//!
//! The operation subsystem introduces no arbitrary universal
//! hardware ceilings.
//!
//! Collections and operation workloads grow according to
//! available resources, platform capabilities and explicitly
//! configured operational budgets.
//!
//! This does not promise literal infinite physical computation.
//!
//! # Domain independence
//!
//! The common operation interfaces must accommodate operations
//! originating from:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - AI and tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Embedded computation.
//! - Networking.
//! - Cryptographic computation.
//! - Neuromorphic computation.
//! - Photonic computation.
//! - Analog computation.
//! - Future computational technologies.
//!
//! This module does not define a universal instruction set.
//!
//! Domain-specific request and response payloads retain their
//! existing computational semantics.
//!
//! # Provider integration
//!
//! Hardware providers implement the appropriate operation
//! interfaces defined by the operation components.
//!
//! A provider may implement:
//!
//! - Submission.
//! - Cancellation.
//! - Specialized operation behavior.
//!
//! Completion information is represented through the
//! completion contracts.
//!
//! A provider is not required to implement unsupported
//! operations.
//!
//! Provider discovery, registration, trust and lifecycle
//! remain owned by their respective authorities.
//!
//! # Runtime integration
//!
//! ZUR remains responsible for:
//!
//! - Execution orchestration.
//! - Resource allocation.
//! - Resource management.
//! - Scheduling.
//! - Operation coordination.
//! - Recovery policy.
//! - Runtime lifecycle.
//!
//! ZHAL supplies the hardware-facing operation contracts.
//!
//! This module does not create a second runtime scheduler,
//! resource manager or operation queue.
//!
//! # Distributed integration
//!
//! Remote providers may implement the same operation
//! submission and cancellation interfaces.
//!
//! The operation subsystem preserves:
//!
//! - Provider identity.
//! - Operation identity.
//! - Submission receipts.
//! - Partial acceptance.
//! - Indeterminate outcomes.
//! - Completion provenance.
//! - Cancellation outcomes.
//!
//! Transport, routing, membership and distributed recovery
//! remain owned by the distributed subsystem.
//!
//! # Grammar boundary
//!
//! Relevant grammar:
//!
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! The grammar defines source-level intent.
//!
//! Frontend and semantic authorities validate that intent.
//!
//! Compiler and runtime authorities prepare and orchestrate
//! execution.
//!
//! ZHAL operation contracts describe the physical provider
//! boundary.
//!
//! This module must never depend directly on generated grammar
//! parser types or source-language AST representations.
//!
//! # Public API stability
//!
//! Each component has an independently versioned schema.
//!
//! The public module paths are:
//!
//! - `operations::contract`
//! - `operations::submission`
//! - `operations::completion`
//! - `operations::cancellation`
//!
//! Component-specific constants remain available through their
//! respective module paths.
//!
//! The explicit re-exports below provide convenient access to
//! commonly used public types.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! Provider implementations remain responsible for satisfying
//! the documented Send, Sync, Future and lifecycle contracts.


// ============================================================================
// OPERATION COMPONENT MODULES
// ============================================================================

/// Operation compatibility and lifecycle contracts.
///
/// Defines provider API compatibility, required capabilities,
/// submission semantics, completion semantics, cancellation
/// semantics and retry guarantees.
pub mod contract;

/// Hardware operation submission.
///
/// Defines submission requests, dispatch modes, acceptance
/// outcomes, provider submission interfaces and receipts.
pub mod submission;

/// Hardware operation completion.
///
/// Defines completion status, completion outcomes,
/// partial completion evidence, failure information
/// and completion provenance.
pub mod completion;

/// Hardware operation cancellation.
///
/// Defines cancellation requests, cancellation outcomes,
/// cancellation receipts and provider cancellation interfaces.
pub mod cancellation;


// ============================================================================
// CONTRACT API
// ============================================================================

/// Common hardware operation contract.
///
/// Describes the operation supported by a provider without
/// embedding domain-specific payload implementations.
pub use contract::HardwareOperationContract;

/// Provider API compatibility requirements.
pub use contract::ProviderApiCompatibility;

/// Operation lifecycle guarantees.
pub use contract::OperationLifecycle;

/// Submission acceptance guarantees.
pub use contract::SubmissionSemantics;

/// Completion guarantees.
pub use contract::CompletionSemantics;

/// Cancellation guarantees.
pub use contract::CancellationSemantics;

/// Retry and idempotency guarantees.
pub use contract::RetrySemantics;


// ============================================================================
// SUBMISSION API
// ============================================================================

/// Operation submission request.
pub use submission::SubmissionRequest;

/// Operation submission receipt.
pub use submission::SubmissionReceipt;

/// Submission receipt identity.
pub use submission::SubmissionReceiptId;

/// Provider operation idempotency key.
pub use submission::IdempotencyKey;

/// Provider submission acceptance outcome.
pub use submission::Acceptance;

/// Operation dispatch mode.
pub use submission::DispatchMode;

/// Provider hardware-operation submission interface.
pub use submission::HardwareOperationSubmitter;

/// Submission future.
pub use submission::SubmissionFuture;

/// Validates and dispatches a hardware operation submission.
pub use submission::submit_operation;


// ============================================================================
// COMPLETION API
// ============================================================================

/// Operation completion reference.
pub use completion::CompletionReference;

/// Operation completion status.
pub use completion::CompletionStatus;

/// Completion provenance.
pub use completion::CompletionProvenance;

/// Partial completion evidence.
pub use completion::PartialCompletionEvidence;

/// Structured operation failure.
pub use completion::CompletionFailure;

/// Operation completion outcome.
pub use completion::CompletionOutcome;

/// Validated operation completion record.
pub use completion::CompletionRecord;


// ============================================================================
// CANCELLATION API
// ============================================================================

/// Cancellation reference.
pub use cancellation::CancellationReference;

/// Hardware operation cancellation request.
pub use cancellation::CancellationRequest;

/// Cancellation outcome.
pub use cancellation::CancellationOutcome;

/// Cancellation receipt.
pub use cancellation::CancellationReceipt;

/// Provider hardware-operation cancellation interface.
pub use cancellation::HardwareOperationCanceller;

/// Cancellation future.
pub use cancellation::CancellationFuture;

/// Dispatches and validates a hardware operation cancellation request.
pub use cancellation::request_cancellation;
