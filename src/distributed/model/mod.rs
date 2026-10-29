
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Logical domain models for Zamani distributed computation.
//!
//! # Purpose
//!
//! This module composes the validated, target-independent data models used
//! to describe distributed participants, tasks, task dependencies,
//! communication intent, and distributed state.
//!
//! It is a domain-model boundary, not a distributed execution engine.
//!
//! # Architectural ownership
//!
//! This module owns no competing definitions of:
//!
//! - logical identifiers, which belong to `crate::distributed::ids`;
//! - distributed execution requirements, which belong to
//!   `crate::distributed::requirements`;
//! - source syntax, AST nodes, or semantic analysis;
//! - canonical types, effects, resource semantics, or capability semantics;
//! - ZUIR, classical IR, `quantum::ir`, or HDLIR;
//! - physical topology, resource allocation, or device identity;
//! - placement, partitioning, scheduling, or routing;
//! - network protocols, serialization formats, or transport;
//! - execution, persistence, checkpointing, or recovery.
//!
//! These responsibilities remain with their respective authoritative
//! repository modules.
//!
//! # Submodules
//!
//! - [`participant`] describes logical participants.
//! - [`task`] describes logical distributed tasks.
//! - [`dependency`] describes task dependency relationships and graphs.
//! - [`communication`] describes logical channels and message intent.
//! - [`state`] describes logical distributed-state metadata.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! canonical grammar and AST
//!           |
//!           v
//! semantic analysis and canonical semantic facts
//!           |
//!           v
//! distributed requirements
//!           |
//!           v
//! distributed model
//!           |
//!           v
//! feasibility, planning, and execution adapters
//! ```
//!
//! The model must not import planner, runtime, transport, or backend
//! implementations. Keeping these dependencies downstream prevents circular
//! ownership and allows model validation to be tested without a live cluster.
//!
//! # Identity and validation
//!
//! Each submodule uses the authoritative identifier types in
//! `crate::distributed::ids`. A descriptive label is never a substitute
//! for identity. Constructing a model value does not authenticate a
//! participant, prove a capability, allocate a resource, or establish that
//! a requested operation can execute.
//!
//! The individual model types are responsible for validating their own
//! invariants. This module provides composition and exports; it must not
//! silently bypass those constructors or duplicate their validation rules.
//!
//! # Serialization and trust boundaries
//!
//! Serialization policy belongs to the individual domain types and to the
//! transport or persistence boundary. A type must not be made deserializable
//! here merely for convenience if doing so would bypass its validation
//! invariants.
//!
//! Untrusted remote data must be decoded through an appropriately versioned
//! and resource-bounded boundary, validated, and only then converted into
//! domain values.
//!
//! # Scalability and POCO-REAF
//!
//! These models impose no arbitrary application-defined universal maximum
//! on the number of participants, tasks, dependencies, channels, messages,
//! or logical state descriptors.
//!
//! Actual memory, address-space, numeric-representation, and configured
//! operational-budget limits remain finite. A caller that cannot represent
//! or allocate a requested workload must report the actual failure rather
//! than treating the workload as semantically invalid solely because of an
//! arbitrary global ceiling.
//!
//! POCO-REAF does not imply that every realization supports every operation.
//! Capability and resource feasibility must be established by the
//! appropriate downstream components.
//!
//! # Completion contract
//!
//! This module is complete when:
//!
//! - all declared submodules exist and compile;
//! - public exports refer to the authoritative types;
//! - required identifier and parent-module declarations are integrated;
//! - no model module depends on a concrete transport or runtime;
//! - model constructors preserve their documented invariants;
//! - the model's tests pass without requiring physical hardware or a cluster.
//!
//! No unsafe Rust is used.

/// Logical communication channels, operations, and message intent.
///
/// This module describes communication requests; it does not implement
/// network transport or guarantee delivery.
pub mod communication;

/// Logical task dependencies and dependency-graph validation.
///
/// This module describes relationships between tasks; it does not schedule
/// or execute those tasks.
pub mod dependency;

/// Logical distributed-state descriptors and state policy metadata.
///
/// This module does not implement storage, consistency algorithms,
/// checkpoint persistence, or state migration.
pub mod state;

/// Logical participants independent of physical machines and endpoints.
///
/// This module does not perform membership discovery, authentication,
/// health monitoring, or placement.
pub mod participant;

/// Target-independent distributed task descriptors.
///
/// This module does not compile, partition, schedule, or execute tasks.
pub mod task;

// Explicit exports provide a deliberate public API rather than exposing
// every implementation detail implicitly through glob re-exports.

pub use communication::{
    ChannelName,
    CommunicationOperation,
    CommunicationChannel,
    CommunicationRecipients,
    CommunicationMessage,
    CommunicationValidationError,
    DeliverySemantics,
    OrderingSemantics,
};

pub use dependency::{
    DependencyKind,
    DependencyRelation,
    TaskDependency,
    DependencyValidationError,
    DependencyGraph,
    DependencyGraphError,
};

pub use participant::{
    Participant,
    ParticipantLabel,
    ParticipantRole,
    ParticipantValidationError,
};

pub use state::{
    CheckpointPolicy,
    ConsistencyModel,
    DistributedStateDescriptor,
    StateLabel,
    StateOwnership,
    StateOwnershipName,
    StateValidationError,
    StateVersion,
};

pub use task::{
    FailurePolicyRef,
    Task,
    TaskLabel,
    TaskLifecycleExpectation,
    TaskPortName,
    TaskPorts,
    TaskValidationError,
};
