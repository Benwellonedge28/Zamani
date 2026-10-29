
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Typed logical identities for Zamani distributed computation.
//!
//! Path: `src/distributed/ids.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Ownership
//!
//! This module owns identifiers for entities whose identity belongs to the
//! distributed-computing subsystem. It does not own source identifiers,
//! semantic symbols, IR identities, physical device identifiers, network
//! addresses, operating-system process IDs, or runtime lifecycle state.
//!
//! # Identity representation
//!
//! Each ID consists of:
//!
//! - a non-zero namespace;
//! - a non-zero sequence number within that namespace.
//!
//! The namespace separates independently created allocators. The sequence
//! provides unique allocations within one allocator namespace.
//!
//! Randomly generated namespaces provide probabilistic cross-allocator
//! uniqueness, not a mathematical guarantee. Applications requiring
//! collision-free identity across administrative or security boundaries
//! must use coordinated namespace assignment or an authoritative registry.
//!
//! # Allocation and scalability
//!
//! Allocation is synchronized, uses checked arithmetic, and never wraps or
//! silently reuses an identifier. Exhaustion and synchronization poisoning
//! are reported explicitly.
//!
//! The representation is finite because it uses fixed-width integer fields.
//! It does not impose an arbitrary limit on the number of machines, tasks,
//! participants, or hardware devices that Zamani can describe. An allocator
//! exhausts its representable sequence only after using its available ID
//! space.
//!
//! # Serialization
//!
//! IDs have typed textual representations suitable for logs, diagnostics,
//! storage keys, and message correlation. Serde serialization preserves the
//! structured namespace and sequence representation.
//!
//! Deserializing an ID proves only that its representation is valid. It
//! does not prove that the corresponding task, participant, or other entity
//! exists, is authorized, or is still alive.
//!
//! # Integration
//!
//! - `model/participant.rs` uses `ParticipantId`.
//! - `model/task.rs` uses `TaskId`.
//! - `execution/` uses `ExecutionContextId` and `OperationId`.
//! - `planning/` uses `PlanId`.
//! - `transport/` uses `MessageId` and `CorrelationId`.
//! - `membership/` uses `ParticipantId`.
//! - `resilience/` uses `CheckpointId`.
//! - `model/communication.rs` uses `ChannelId`.
//! - `model/state.rs` uses `DistributedStateId`.
//! - `runtime/distributed.rs` requires an explicit compatibility migration
//!   from its current String-based `NodeId` and `ServiceId` aliases.
//!
//! These are integration contracts; this file does not claim that all those
//! modules already exist.
//!
//! # Safety
//!
//! No unsafe Rust is used.

use core::fmt;
use core::num::NonZeroU128;
use core::str::FromStr;
use std::sync::{Mutex, PoisonError};

use rand::random;
use serde::{Deserialize, Serialize};

/// Errors returned by distributed identity construction and allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IdError {
    /// The namespace component must be non-zero.
    ZeroNamespace,

    /// The sequence component must be non-zero.
    ZeroSequence,

    /// The allocator has exhausted its representable sequence space.
    Exhausted,

    /// The allocator lock was poisoned after a panic in a critical section.
    AllocatorPoisoned,

    /// The textual ID does not match the expected type-specific format.
    InvalidFormat,

    /// A numeric component is not a valid hexadecimal `u128`.
    InvalidComponent,

    /// The supplied textual ID belongs to a different ID category.
    WrongKind,
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroNamespace => f.write_str("distributed ID namespace must be non-zero"),
            Self::ZeroSequence => f.write_str("distributed ID sequence must be non-zero"),
            Self::Exhausted => f.write_str("distributed ID allocator exhausted"),
            Self::AllocatorPoisoned => {
                f.write_str("distributed ID allocator synchronization was poisoned")
            }
            Self::InvalidFormat => {
                f.write_str("invalid distributed ID format")
            }
            Self::InvalidComponent => {
                f.write_str("invalid distributed ID numeric component")
            }
            Self::WrongKind => {
                f.write_str("distributed ID has the wrong entity kind")
            }
        }
    }
}

impl std::error::Error for IdError {}

impl<T> From<PoisonError<T>> for IdError {
    fn from(_: PoisonError<T>) -> Self {
        Self::AllocatorPoisoned
    }
}

/// Common representation of a distributed logical identity.
///
/// This representation is public for inspection and explicit integration,
/// but its fields remain private so invalid values cannot be constructed
/// through ordinary struct literals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DistributedId {
    namespace: NonZeroU128,
    sequence: NonZeroU128,
}

impl DistributedId {
    /// Construct an ID from its namespace and sequence.
    pub fn new(namespace: u128, sequence: u128) -> Result<Self, IdError> {
        let namespace =
            NonZeroU128::new(namespace).ok_or(IdError::ZeroNamespace)?;
        let sequence =
            NonZeroU128::new(sequence).ok_or(IdError::ZeroSequence)?;

        Ok(Self {
            namespace,
            sequence,
        })
    }

    /// Return the namespace component.
    #[must_use]
    pub const fn namespace(self) -> u128 {
        self.namespace.get()
    }

    /// Return the sequence component.
    #[must_use]
    pub const fn sequence(self) -> u128 {
        self.sequence.get()
    }

    /// Return the canonical pair of numeric components.
    #[must_use]
    pub const fn components(self) -> (u128, u128) {
        (self.namespace.get(), self.sequence.get())
    }

    fn parse_components(namespace: &str, sequence: &str) -> Result<Self, IdError> {
        let namespace =
            u128::from_str_radix(namespace, 16).map_err(|_| IdError::InvalidComponent)?;
        let sequence =
            u128::from_str_radix(sequence, 16).map_err(|_| IdError::InvalidComponent)?;

        Self::new(namespace, sequence)
    }
}

/// An allocator that issues typed distributed IDs.
///
/// Clones share the same sequence and namespace. Independently constructed
/// allocators have separate namespaces unless a namespace is explicitly
/// supplied through [`IdAllocator::with_namespace`].
///
/// The namespace is an identity partition, not an authentication token.
#[derive(Debug)]
pub struct IdAllocator {
    namespace: NonZeroU128,
    next_sequence: Mutex<Option<NonZeroU128>>,
}

impl IdAllocator {
    /// Construct an allocator with a randomly selected namespace.
    ///
    /// The random namespace makes accidental collisions extremely unlikely,
    /// but does not provide a formal uniqueness guarantee across independent
    /// allocators. Use `with_namespace` when namespace uniqueness is managed
    /// externally.
    pub fn new() -> Self {
        loop {
            let candidate = random::<u128>();
            if let Some(namespace) = NonZeroU128::new(candidate) {
                return Self {
                    namespace,
                    next_sequence: Mutex::new(NonZeroU128::new(1)),
                };
            }
        }
    }

    /// Construct an allocator with an externally managed namespace.
    ///
    /// The caller is responsible for ensuring that the namespace is unique
    /// wherever globally unique IDs are required.
    pub fn with_namespace(namespace: u128) -> Result<Self, IdError> {
        let namespace =
            NonZeroU128::new(namespace).ok_or(IdError::ZeroNamespace)?;

        Ok(Self {
            namespace,
            next_sequence: Mutex::new(NonZeroU128::new(1)),
        })
    }

    /// Return the allocator namespace.
    #[must_use]
    pub const fn namespace(&self) -> u128 {
        self.namespace.get()
    }

    /// Allocate the next identity in this allocator's sequence.
    ///
    /// The final non-zero `u128` sequence value is usable. A subsequent
    /// allocation returns `IdError::Exhausted`.
    fn allocate(&self) -> Result<DistributedId, IdError> {
        let mut next = self.next_sequence.lock()?;
        let current = next.take().ok_or(IdError::Exhausted)?;

        *next = current.get().checked_add(1).and_then(NonZeroU128::new);

        Ok(DistributedId {
            namespace: self.namespace,
            sequence: current,
        })
    }
}

impl Clone for IdAllocator {
    fn clone(&self) -> Self {
        Self {
            namespace: self.namespace,
            next_sequence: Mutex::new(
                *self
                    .next_sequence
                    .lock()
                    .expect("cannot clone a poisoned distributed ID allocator"),
            ),
        }
    }
}

impl Default for IdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Declare a strongly typed distributed ID.
///
/// Each ID category has its own Rust type and textual prefix. This prevents
/// accidental interchange of task, participant, message, and plan IDs.
macro_rules! define_distributed_id {
    ($name:ident, $prefix:literal) => {
        #[doc = concat!("Typed logical identifier for `", stringify!($name), "`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(DistributedId);

        impl $name {
            /// Construct an ID from its namespace and sequence.
            pub fn new(namespace: u128, sequence: u128) -> Result<Self, IdError> {
                Ok(Self(DistributedId::new(namespace, sequence)?))
            }

            /// Return the namespace component.
            #[must_use]
            pub const fn namespace(self) -> u128 {
                self.0.namespace()
            }

            /// Return the sequence component.
            #[must_use]
            pub const fn sequence(self) -> u128 {
                self.0.sequence()
            }

            /// Return the underlying identity representation.
            #[must_use]
            pub const fn as_distributed_id(self) -> DistributedId {
                self.0
            }

            /// Parse this category's canonical textual representation.
            pub fn parse(value: &str) -> Result<Self, IdError> {
                value.parse()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    concat!($prefix, ":{:032x}:{:032x}"),
                    self.0.namespace(),
                    self.0.sequence()
                )
            }
        }

        impl FromStr for $name {
            type Err = IdError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let mut parts = value.split(':');

                let kind = parts.next().ok_or(IdError::InvalidFormat)?;
                let namespace = parts.next().ok_or(IdError::InvalidFormat)?;
                let sequence = parts.next().ok_or(IdError::InvalidFormat)?;

                if parts.next().is_some() {
                    return Err(IdError::InvalidFormat);
                }

                if kind != $prefix {
                    return Err(IdError::WrongKind);
                }

                let id = DistributedId::parse_components(namespace, sequence)?;
                Ok(Self(id))
            }
        }
    };
}

define_distributed_id!(ParticipantId, "participant");
define_distributed_id!(TaskId, "task");
define_distributed_id!(ExecutionContextId, "context");
define_distributed_id!(MessageId, "message");
define_distributed_id!(PlanId, "plan");
define_distributed_id!(OperationId, "operation");
define_distributed_id!(ServiceId, "service");
define_distributed_id!(CorrelationId, "correlation");
define_distributed_id!(ChannelId, "channel");
define_distributed_id!(DistributedStateId, "state");
define_distributed_id!(CheckpointId, "checkpoint");
define_distributed_id!(DeploymentId, "deployment");

/// Allocate a typed ID using the shared allocator.
macro_rules! allocator_method {
    ($method:ident, $type:ident) => {
        /// Allocate a fresh identity of this category.
        pub fn $method(&self) -> Result<$type, IdError> {
            Ok($type(self.allocate()?))
        }
    };
}

impl IdAllocator {
    allocator_method!(participant_id, ParticipantId);
    allocator_method!(task_id, TaskId);
    allocator_method!(execution_context_id, ExecutionContextId);
    allocator_method!(message_id, MessageId);
    allocator_method!(plan_id, PlanId);
    allocator_method!(operation_id, OperationId);
    allocator_method!(service_id, ServiceId);
    allocator_method!(correlation_id, CorrelationId);
    allocator_method!(channel_id, ChannelId);
    allocator_method!(state_id, DistributedStateId);
    allocator_method!(checkpoint_id, CheckpointId);
    allocator_method!(deployment_id, DeploymentId);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn rejects_zero_components() {
        assert_eq!(DistributedId::new(0, 1), Err(IdError::ZeroNamespace));
        assert_eq!(DistributedId::new(1, 0), Err(IdError::ZeroSequence));
        assert_eq!(IdAllocator::with_namespace(0).err(), Some(IdError::ZeroNamespace));
    }

    #[test]
    fn allocates_distinct_ids_within_a_namespace() {
        let allocator = IdAllocator::with_namespace(42).unwrap();

        let first = allocator.task_id().unwrap();
        let second = allocator.task_id().unwrap();

        assert_eq!(first.namespace(), 42);
        assert_eq!(first.sequence(), 1);
        assert_eq!(second.sequence(), 2);
        assert_ne!(first, second);
    }

    #[test]
    fn different_id_categories_are_type_distinct() {
        let allocator = IdAllocator::with_namespace(7).unwrap();

        let task = allocator.task_id().unwrap();
        let participant = allocator.participant_id().unwrap();

        assert_eq!(task.namespace(), participant.namespace());
        assert_ne!(task.sequence(), participant.sequence());
    }

    #[test]
    fn display_and_parse_round_trip() {
        let id = TaskId::new(0xabc, 0x123).unwrap();
        let encoded = id.to_string();

        assert_eq!(encoded, "task:00000000000000000000000000000abc:00000000000000000000000000000123");
        assert_eq!(encoded.parse::<TaskId>().unwrap(), id);
    }

    #[test]
    fn parser_rejects_wrong_kind_and_malformed_values() {
        assert_eq!(
            "participant:00000000000000000000000000000001:00000000000000000000000000000001"
                .parse::<TaskId>(),
            Err(IdError::WrongKind)
        );

        assert_eq!("task:xyz:1".parse::<TaskId>(), Err(IdError::InvalidComponent));
        assert_eq!("task:1:0".parse::<TaskId>(), Err(IdError::ZeroSequence));
        assert_eq!("task:1:1:extra".parse::<TaskId>(), Err(IdError::InvalidFormat));
    }

    #[test]
    fn cloned_allocator_shares_sequence() {
        let allocator = IdAllocator::with_namespace(99).unwrap();
        let cloned = allocator.clone();

        assert_eq!(allocator.task_id().unwrap().sequence(), 1);
        assert_eq!(cloned.task_id().unwrap().sequence(), 2);
        assert_eq!(allocator.task_id().unwrap().sequence(), 3);
    }

    #[test]
    fn concurrent_allocations_are_unique_within_allocator() {
        let allocator = Arc::new(IdAllocator::with_namespace(100).unwrap());
        let mut workers = Vec::new();

        for _ in 0..8 {
            let allocator = Arc::clone(&allocator);

            workers.push(thread::spawn(move || {
                (0..128)
                    .map(|_| allocator.task_id().unwrap())
                    .collect::<Vec<_>>()
            }));
        }

        let mut ids = HashSet::new();

        for worker in workers {
            for id in worker.join().unwrap() {
                assert!(ids.insert(id));
            }
        }

        assert_eq!(ids.len(), 1024);
    }

    #[test]
    fn serde_round_trip_preserves_typed_id() {
        let id = MessageId::new(0x1234, 0x5678).unwrap();

        let encoded = serde_json::to_string(&id).unwrap();
        let decoded: MessageId = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, id);
    }

    #[test]
    fn maximum_sequence_is_allocated_without_wrapping() {
        let allocator = IdAllocator {
            namespace: NonZeroU128::new(1).unwrap(),
            next_sequence: Mutex::new(NonZeroU128::new(u128::MAX)),
        };

        assert_eq!(allocator.task_id().unwrap().sequence(), u128::MAX);
        assert_eq!(allocator.task_id(), Err(IdError::Exhausted));
    }
}
