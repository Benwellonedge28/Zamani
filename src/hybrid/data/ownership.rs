
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Cross-Domain Ownership
//!
//! Path: `src/hybrid/data/ownership.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Define the declarative ownership, borrowing, lifetime, completion,
//! cancellation, and release obligations associated with values that cross
//! computational-domain boundaries.
//!
//! This module supports classical, quantum, tensor, AI/ML, accelerator,
//! HDL, distributed, networking, and future computational domains without
//! defining a closed inventory of domains or copying domain-owned values.
//!
//! ## Canonical ownership
//!
//! This module owns:
//! - Cross-domain ownership contract declarations.
//! - Ownership-boundary operation classification.
//! - Abstract lifetime and access requirements.
//! - Completion and cancellation obligations.
//! - Release responsibility declarations.
//! - Memory visibility and coherence requirements.
//! - Structural validation of these declarations.
//!
//! This module does NOT own:
//! - The language's ownership or borrow checker.
//! - Canonical semantic types, effects, or lifetime regions.
//! - Classical, quantum, HDL, tensor, or other domain IRs.
//! - Runtime job state, physical allocation, or resource destruction.
//! - Physical memory transfer, serialization, or network transport.
//! - Device selection, scheduling, routing, QEC, or backend lowering.
//!
//! ## Integration
//!
//! - `super::values` provides `CrossDomainValue` identities and
//!   `CanonicalEntityRef` references.
//! - `super::super::ids` provides graph-scoped hybrid identities.
//! - `super::super::errors` provides the canonical hybrid error vocabulary.
//! - `super::super::contract::OwnershipExpectation` remains the shared
//!   domain-interface ownership expectation.
//! - The canonical semantic layer proves lifetime and ownership validity.
//! - `data::exchange` checks exchange compatibility against these contracts.
//! - `data::transfer` describes transfer requirements and completion.
//! - `sync` defines graph-level synchronization semantics.
//! - Runtime integration realizes completion, cancellation, and release.
//!
//! The module does not require those downstream modules to exist in order
//! to compile. Their integration must use these public contracts rather
//! than introducing competing ownership definitions.
//!
//! ## Important distinction
//!
//! Structural validation does not prove that a borrow is valid, that a
//! lifetime outlives another lifetime, that a linear value is consumed
//! exactly once, or that a runtime has actually released a resource.
//! Those claims require evidence from their canonical authorities.
//!
//! ## Scalability
//!
//! This module imposes no fixed maximum number of values, domains, lifetime
//! regions, ownership contracts, or graph instances. It does not impose
//! universal hardware-capacity or value-size limits.
//!
//! Operational resource budgets belong to `HybridConfig` and the relevant
//! operation. Rust allocation, address-space, and target limits still apply.
//!
//! ## Serialization
//!
//! Deserialized contracts are structurally revalidated. They are declarations,
//! not proof that referenced entities exist or that obligations have been
//! discharged.

use serde::{Deserialize, Serialize};

use super::super::contract::OwnershipExpectation;
use super::super::errors::{HybridError, HybridErrorCode};
use super::super::ids::{CrossDomainValueId, DomainInstanceId, HybridGraphId};
use super::values::CanonicalEntityRef;

/// The semantic lifetime category associated with a cross-domain value.
///
/// These categories correspond to the lifetime vocabulary in the canonical
/// data specification. They are semantic relationships, not wall-clock
/// durations, physical memory locations, or allocation instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifetimeKind {
    /// A value whose validity is limited to a temporary operation.
    Temporary,

    /// A value whose validity is bounded by a semantic region or scope.
    ScopeBound,

    /// A value whose ownership is held by an owning computation or entity.
    Owned,

    /// A value whose semantic lifetime is persistent.
    Persistent,

    /// A value whose validity is associated with a session.
    Session,

    /// A value governed by stream lifetime semantics.
    Stream,

    /// A value whose lifetime is associated with an application.
    Application,

    /// A value whose lifetime is defined by an external authority.
    External,
}

/// The operation declared at a computational-domain boundary.
///
/// This classifies the semantic operation requested at the boundary. It
/// does not execute the operation or establish that the operation is legal
/// under the canonical language ownership rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipOperation {
    /// The consumer borrows the value without receiving ownership.
    Borrow,

    /// The value is shared under an explicit sharing contract.
    Share,

    /// Ownership is transferred to another owner.
    Move,

    /// A logical copy is requested.
    ///
    /// Whether a physical copy is needed is implementation-defined, provided
    /// identity, isolation, lifetime, and observable semantics are preserved.
    Copy,

    /// A representation is serialized under a canonical data contract.
    Serialize,

    /// The value is transferred across a domain boundary.
    Transfer,

    /// A domain-specific value is materialized into another representation.
    Materialize,

    /// A value or linear resource is consumed.
    Consume,

    /// A domain produces a value under its canonical value semantics.
    Produce,
}

/// The access guarantees required while a value is used across domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessRequirement {
    /// Read-only access; mutation through this contract is not permitted.
    ReadOnly,

    /// Exclusive mutable access under canonical ownership rules.
    ExclusiveMutable,

    /// Concurrent mutable access is allowed only when the canonical
    /// semantic and synchronization authorities establish its validity.
    SynchronizedMutable,
}

/// The completion guarantee required before the value may be observed.
///
/// This describes a semantic obligation. It is not a runtime job-state
/// machine and does not imply that a submitted operation has completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionRequirement {
    /// The operation must complete before the boundary operation returns.
    Synchronous,

    /// The consumer must wait for the required completion event before use.
    AwaitBeforeUse,

    /// The value is delivered according to a stream's completion semantics.
    Stream,

    /// Completion is governed by an external canonical contract.
    External,
}

/// The cancellation guarantee associated with an outstanding operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationRequirement {
    /// No cancellable asynchronous operation is declared by this contract.
    NotApplicable,

    /// Cancellation may be attempted but is not guaranteed to succeed.
    BestEffort,

    /// Cancellation obligations must propagate according to the canonical
    /// operation and runtime contracts.
    Propagate,

    /// Cancellation behavior is governed by an external authority.
    External,
}

/// The authority responsible for releasing or retaining the resource.
///
/// This declares responsibility. It does not perform deallocation, destroy
/// hardware resources, or cancel a physical job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseResponsibility {
    /// The producer retains responsibility for the underlying resource.
    Producer,

    /// The consumer is responsible after ownership has been transferred.
    Consumer,

    /// A shared protocol defines release responsibility.
    SharedProtocol,

    /// An external canonical owner controls release.
    ExternalOwner,

    /// This contract does not assign local release responsibility.
    ///
    /// This is not permission to leak a resource. The canonical owner must
    /// define and enforce the actual resource lifecycle.
    NotOwnedHere,
}

/// The required memory-visibility and coherence guarantee.
///
/// These are abstract semantic requirements. They do not specify cache
/// protocols, memory banks, network transports, or device mechanisms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoherenceRequirement {
    /// No additional cross-domain coherence guarantee is declared.
    Unspecified,

    /// Consumers observe an immutable shared value.
    ImmutableShared,

    /// A logical copy must provide the required mutation isolation.
    CopyIsolated,

    /// Consumers require a coherent view under the canonical memory model.
    Coherent,

    /// Eventual consistency is explicitly acceptable.
    EventuallyConsistent,

    /// Visibility requires the declared synchronization obligations.
    Synchronized,
}

/// A validated declaration of cross-domain ownership obligations.
///
/// The `contract_ref` identifies this contract in the canonical ownership
/// namespace. `value` identifies the hybrid-layer reference, not a payload.
/// `producer` and `consumer` identify participating domain instances.
///
/// `lifetime_region`, when present, refers to a region owned by the canonical
/// semantic/type system. This module never creates or resolves that region.
///
/// All fields are private so ordinary callers must construct contracts
/// through `new` and satisfy structural validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "OwnershipContractWire")]
pub struct OwnershipContract {
    contract_ref: CanonicalEntityRef,
    value: CrossDomainValueId,
    producer: DomainInstanceId,
    consumer: Option<DomainInstanceId>,
    expectation: OwnershipExpectation,
    operation: OwnershipOperation,
    lifetime: LifetimeKind,
    lifetime_region: Option<CanonicalEntityRef>,
    access: AccessRequirement,
    completion: CompletionRequirement,
    cancellation: CancellationRequirement,
    release: ReleaseResponsibility,
    coherence: CoherenceRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct OwnershipContractWire {
    contract_ref: CanonicalEntityRef,
    value: CrossDomainValueId,
    producer: DomainInstanceId,
    consumer: Option<DomainInstanceId>,
    expectation: OwnershipExpectation,
    operation: OwnershipOperation,
    lifetime: LifetimeKind,
    lifetime_region: Option<CanonicalEntityRef>,
    access: AccessRequirement,
    completion: CompletionRequirement,
    cancellation: CancellationRequirement,
    release: ReleaseResponsibility,
    coherence: CoherenceRequirement,
}

impl OwnershipContract {
    /// Construct and validate a cross-domain ownership contract.
    ///
    /// This constructor verifies structural relationships and conservative
    /// consistency rules. It does not perform language type checking, prove
    /// lifetime subtyping, or execute ownership transitions.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract_ref: CanonicalEntityRef,
        value: CrossDomainValueId,
        producer: DomainInstanceId,
        consumer: Option<DomainInstanceId>,
        expectation: OwnershipExpectation,
        operation: OwnershipOperation,
        lifetime: LifetimeKind,
        lifetime_region: Option<CanonicalEntityRef>,
        access: AccessRequirement,
        completion: CompletionRequirement,
        cancellation: CancellationRequirement,
        release: ReleaseResponsibility,
        coherence: CoherenceRequirement,
    ) -> Result<Self, HybridError> {
        let contract = Self {
            contract_ref,
            value,
            producer,
            consumer,
            expectation,
            operation,
            lifetime,
            lifetime_region,
            access,
            completion,
            cancellation,
            release,
            coherence,
        };

        contract.validate()?;
        Ok(contract)
    }

    /// Return the canonical reference identifying this contract.
    #[must_use]
    pub fn contract_ref(&self) -> &CanonicalEntityRef {
        &self.contract_ref
    }

    /// Return the hybrid-layer value identity.
    #[must_use]
    pub const fn value(&self) -> CrossDomainValueId {
        self.value
    }

    /// Return the producing domain instance.
    #[must_use]
    pub const fn producer(&self) -> DomainInstanceId {
        self.producer
    }

    /// Return the consuming domain instance, if one is declared.
    #[must_use]
    pub const fn consumer(&self) -> Option<DomainInstanceId> {
        self.consumer
    }

    /// Return the shared domain-interface ownership expectation.
    #[must_use]
    pub const fn expectation(&self) -> OwnershipExpectation {
        self.expectation
    }

    /// Return the declared boundary operation.
    #[must_use]
    pub const fn operation(&self) -> OwnershipOperation {
        self.operation
    }

    /// Return the declared lifetime category.
    #[must_use]
    pub const fn lifetime(&self) -> LifetimeKind {
        self.lifetime
    }

    /// Return the canonical lifetime-region reference, if provided.
    #[must_use]
    pub fn lifetime_region(&self) -> Option<&CanonicalEntityRef> {
        self.lifetime_region.as_ref()
    }

    /// Return the required access guarantee.
    #[must_use]
    pub const fn access(&self) -> AccessRequirement {
        self.access
    }

    /// Return the completion obligation.
    #[must_use]
    pub const fn completion(&self) -> CompletionRequirement {
        self.completion
    }

    /// Return the cancellation obligation.
    #[must_use]
    pub const fn cancellation(&self) -> CancellationRequirement {
        self.cancellation
    }

    /// Return the declared release responsibility.
    #[must_use]
    pub const fn release(&self) -> ReleaseResponsibility {
        self.release
    }

    /// Return the required coherence guarantee.
    #[must_use]
    pub const fn coherence(&self) -> CoherenceRequirement {
        self.coherence
    }

    /// Return the graph identity associated with the value.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.value.graph()
    }

    /// Validate structural consistency and conservative boundary rules.
    ///
    /// This method deliberately does not prove that the referenced entities
    /// exist. Resolution and semantic proof belong to the graph verifier,
    /// domain registry, and canonical semantic authority.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.contract_ref.validate()?;

        if self.contract_ref.kind() != "ownership_contract" {
            return Err(ownership_error(
                "the canonical ownership contract reference must identify an ownership_contract entity",
            ));
        }

        if self.value.graph() != self.producer.graph() {
            return Err(ownership_error(
                "the value and producer must belong to the same hybrid graph",
            ));
        }

        if let Some(consumer) = self.consumer {
            if consumer.graph() != self.value.graph() {
                return Err(ownership_error(
                    "the value, producer, and consumer must belong to the same hybrid graph",
                ));
            }

            if consumer == self.producer
                && matches!(
                    self.operation,
                    OwnershipOperation::Move
                        | OwnershipOperation::Transfer
                        | OwnershipOperation::Copy
                        | OwnershipOperation::Serialize
                        | OwnershipOperation::Materialize
                )
            {
                return Err(ownership_error(
                    "a cross-domain transfer operation cannot declare the producer as its own consumer",
                ));
            }
        }

        if let Some(region) = &self.lifetime_region {
            region.validate()?;
        }

        match self.lifetime {
            LifetimeKind::ScopeBound | LifetimeKind::Temporary => {
                if self.lifetime_region.is_none() {
                    return Err(ownership_error(
                        "temporary and scope-bound lifetimes require a canonical semantic-region reference",
                    ));
                }
            }
            LifetimeKind::External => {
                if self.lifetime_region.is_none() {
                    return Err(ownership_error(
                        "an external lifetime requires a reference to its canonical lifetime authority",
                    ));
                }
            }
            LifetimeKind::Owned
            | LifetimeKind::Persistent
            | LifetimeKind::Session
            | LifetimeKind::Stream
            | LifetimeKind::Application => {}
        }

        match self.operation {
            OwnershipOperation::Borrow => {
                if self.consumer.is_none() {
                    return Err(ownership_error(
                        "borrowing requires an explicitly identified consumer",
                    ));
                }

                if !matches!(
                    self.expectation,
                    OwnershipExpectation::Borrowed | OwnershipExpectation::External
                ) {
                    return Err(ownership_error(
                        "borrowing must not declare that the consumer receives ownership",
                    ));
                }

                if self.release == ReleaseResponsibility::Consumer {
                    return Err(ownership_error(
                        "a borrow does not transfer release responsibility to the consumer",
                    ));
                }
            }

            OwnershipOperation::Share => {
                if self.consumer.is_none() {
                    return Err(ownership_error(
                        "sharing requires an explicitly identified consumer",
                    ));
                }

                if self.access != AccessRequirement::ReadOnly
                    && self.coherence != CoherenceRequirement::Synchronized
                {
                    return Err(ownership_error(
                        "mutable sharing requires an explicit synchronized coherence guarantee",
                    ));
                }

                if self.release == ReleaseResponsibility::Consumer {
                    return Err(ownership_error(
                        "sharing does not by itself transfer ownership or release responsibility",
                    ));
                }
            }

            OwnershipOperation::Move
            | OwnershipOperation::Transfer
            | OwnershipOperation::Materialize => {
                if self.consumer.is_none() {
                    return Err(ownership_error(
                        "ownership transfer requires an explicitly identified consumer",
                    ));
                }

                if !matches!(
                    self.expectation,
                    OwnershipExpectation::Owned | OwnershipExpectation::External
                ) {
                    return Err(ownership_error(
                        "move, transfer, and materialization must declare the applicable ownership contract",
                    ));
                }

                if self.release != ReleaseResponsibility::Consumer
                    && self.expectation == OwnershipExpectation::Owned
                {
                    return Err(ownership_error(
                        "an owned transfer must assign post-transfer release responsibility to the consumer",
                    ));
                }
            }

            OwnershipOperation::Copy => {
                if self.consumer.is_none() {
                    return Err(ownership_error(
                        "copying across domains requires an explicitly identified consumer",
                    ));
                }

                if self.coherence != CoherenceRequirement::CopyIsolated
                    && self.coherence != CoherenceRequirement::ImmutableShared
                {
                    return Err(ownership_error(
                        "a cross-domain copy requires an explicit isolation or immutable-sharing guarantee",
                    ));
                }
            }

            OwnershipOperation::Serialize => {
                if self.consumer.is_none() {
                    return Err(ownership_error(
                        "serialization for cross-domain exchange requires an identified consumer",
                    ));
                }
            }

            OwnershipOperation::Consume => {
                if self.expectation == OwnershipExpectation::Borrowed {
                    return Err(ownership_error(
                        "consuming a value is incompatible with a borrowed-only ownership expectation",
                    ));
                }
            }

            OwnershipOperation::Produce => {
                // Production establishes a value at its canonical owner.
                // It does not itself imply transfer to another domain.
            }
        }

        if self.access == AccessRequirement::SynchronizedMutable
            && self.coherence != CoherenceRequirement::Synchronized
        {
            return Err(ownership_error(
                "synchronized mutable access requires an explicit synchronized coherence guarantee",
            ));
        }

        if self.completion == CompletionRequirement::Stream
            && self.lifetime != LifetimeKind::Stream
        {
            return Err(ownership_error(
                "stream completion requires a stream-compatible semantic lifetime",
            ));
        }

        if self.lifetime == LifetimeKind::External
            && self.release != ReleaseResponsibility::ExternalOwner
            && self.release != ReleaseResponsibility::SharedProtocol
        {
            return Err(ownership_error(
                "an external lifetime must delegate release responsibility to its external owner or shared protocol",
            ));
        }

        if self.expectation == OwnershipExpectation::External
            && self.release != ReleaseResponsibility::ExternalOwner
            && self.release != ReleaseResponsibility::SharedProtocol
        {
            return Err(ownership_error(
                "externally governed ownership must identify its external owner or shared release protocol",
            ));
        }

        Ok(())
    }
}

impl TryFrom<OwnershipContractWire> for OwnershipContract {
    type Error = HybridError;

    fn try_from(wire: OwnershipContractWire) -> Result<Self, Self::Error> {
        Self::new(
            wire.contract_ref,
            wire.value,
            wire.producer,
            wire.consumer,
            wire.expectation,
            wire.operation,
            wire.lifetime,
            wire.lifetime_region,
            wire.access,
            wire.completion,
            wire.cancellation,
            wire.release,
            wire.coherence,
        )
    }
}

/// Construct the canonical structured ownership error.
fn ownership_error(message: impl Into<String>) -> HybridError {
    HybridError::new(HybridErrorCode::OwnershipViolation, message)
        .in_component("hybrid data ownership")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::{
        GraphScopedId, HybridGraphId, IdComponent,
    };

    fn graph(value: u128) -> HybridGraphId {
        HybridGraphId::new(value).expect("non-zero graph ID")
    }

    fn domain(graph: HybridGraphId, local: u128) -> DomainInstanceId {
        DomainInstanceId::new(GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("non-zero local ID"),
        ))
    }

    fn value(graph: HybridGraphId, local: u128) -> CrossDomainValueId {
        CrossDomainValueId::new(GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("non-zero local ID"),
        ))
    }

    fn entity(
        namespace: &str,
        kind: &str,
        identifier: &str,
    ) -> CanonicalEntityRef {
        CanonicalEntityRef::new(namespace, kind, identifier)
            .expect("valid canonical entity reference")
    }

    fn borrowed_contract() -> OwnershipContract {
        let graph = graph(1);

        OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:borrow:1",
            ),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::Borrowed,
            OwnershipOperation::Borrow,
            LifetimeKind::ScopeBound,
            Some(entity("semantic", "lifetime_region", "region:1")),
            AccessRequirement::ReadOnly,
            CompletionRequirement::Synchronous,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::ImmutableShared,
        )
        .expect("valid borrowed ownership contract")
    }

    #[test]
    fn accepts_valid_borrow_without_transferring_ownership() {
        let contract = borrowed_contract();

        assert_eq!(contract.operation(), OwnershipOperation::Borrow);
        assert_eq!(contract.expectation(), OwnershipExpectation::Borrowed);
        assert_eq!(contract.release(), ReleaseResponsibility::Producer);
        assert!(contract.validate().is_ok());
    }

    #[test]
    fn rejects_value_and_producer_from_different_graphs() {
        let graph_a = graph(1);
        let graph_b = graph(2);

        let result = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:invalid:1",
            ),
            value(graph_a, 2),
            domain(graph_b, 3),
            Some(domain(graph_a, 4)),
            OwnershipExpectation::Borrowed,
            OwnershipOperation::Borrow,
            LifetimeKind::ScopeBound,
            Some(entity("semantic", "lifetime_region", "region:1")),
            AccessRequirement::ReadOnly,
            CompletionRequirement::Synchronous,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::ImmutableShared,
        );

        assert_eq!(
            result.expect_err("graph mismatch must fail").code(),
            HybridErrorCode::OwnershipViolation
        );
    }

    #[test]
    fn rejects_borrow_without_a_consumer() {
        let graph = graph(1);

        let result = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:invalid:2",
            ),
            value(graph, 2),
            domain(graph, 3),
            None,
            OwnershipExpectation::Borrowed,
            OwnershipOperation::Borrow,
            LifetimeKind::Owned,
            None,
            AccessRequirement::ReadOnly,
            CompletionRequirement::Synchronous,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::Unspecified,
        );

        assert_eq!(
            result.expect_err("borrow must identify its consumer").code(),
            HybridErrorCode::OwnershipViolation
        );
    }

    #[test]
    fn rejects_scope_lifetime_without_semantic_region() {
        let graph = graph(1);

        let result = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:invalid:3",
            ),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::Borrowed,
            OwnershipOperation::Borrow,
            LifetimeKind::ScopeBound,
            None,
            AccessRequirement::ReadOnly,
            CompletionRequirement::Synchronous,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::ImmutableShared,
        );

        assert_eq!(
            result.expect_err("scope lifetime needs a canonical region").code(),
            HybridErrorCode::OwnershipViolation
        );
    }

    #[test]
    fn rejects_mutable_sharing_without_synchronization() {
        let graph = graph(1);

        let result = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:invalid:4",
            ),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::External,
            OwnershipOperation::Share,
            LifetimeKind::External,
            Some(entity("external", "lifetime_region", "region:external")),
            AccessRequirement::SynchronizedMutable,
            CompletionRequirement::External,
            CancellationRequirement::External,
            ReleaseResponsibility::ExternalOwner,
            CoherenceRequirement::EventuallyConsistent,
        );

        assert_eq!(
            result.expect_err("mutable sharing needs synchronization").code(),
            HybridErrorCode::OwnershipViolation
        );
    }

    #[test]
    fn rejects_copy_without_isolation_or_immutable_sharing() {
        let graph = graph(1);

        let result = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:invalid:5",
            ),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::Owned,
            OwnershipOperation::Copy,
            LifetimeKind::Owned,
            None,
            AccessRequirement::ReadOnly,
            CompletionRequirement::Synchronous,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::Unspecified,
        );

        assert_eq!(
            result.expect_err("copy semantics must be explicit").code(),
            HybridErrorCode::OwnershipViolation
        );
    }

    #[test]
    fn accepts_owned_transfer_with_consumer_release_responsibility() {
        let graph = graph(1);

        let contract = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:move:1",
            ),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::Owned,
            OwnershipOperation::Move,
            LifetimeKind::Owned,
            None,
            AccessRequirement::ExclusiveMutable,
            CompletionRequirement::AwaitBeforeUse,
            CancellationRequirement::Propagate,
            ReleaseResponsibility::Consumer,
            CoherenceRequirement::Unspecified,
        )
        .expect("valid owned transfer");

        assert_eq!(contract.consumer(), Some(domain(graph, 4)));
        assert_eq!(contract.release(), ReleaseResponsibility::Consumer);
        assert!(contract.validate().is_ok());
    }

    #[test]
    fn rejects_stream_completion_for_non_stream_lifetime() {
        let graph = graph(1);

        let result = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                "contract:invalid:6",
            ),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::Borrowed,
            OwnershipOperation::Borrow,
            LifetimeKind::ScopeBound,
            Some(entity("semantic", "lifetime_region", "region:1")),
            AccessRequirement::ReadOnly,
            CompletionRequirement::Stream,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::ImmutableShared,
        );

        assert_eq!(
            result.expect_err("stream completion requires stream lifetime").code(),
            HybridErrorCode::OwnershipViolation
        );
    }

    #[test]
    fn rejects_wrong_contract_reference_kind() {
        let graph = graph(1);

        let result = OwnershipContract::new(
            entity("hybrid.data.ownership", "value", "not-a-contract"),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::Borrowed,
            OwnershipOperation::Borrow,
            LifetimeKind::ScopeBound,
            Some(entity("semantic", "lifetime_region", "region:1")),
            AccessRequirement::ReadOnly,
            CompletionRequirement::Synchronous,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::ImmutableShared,
        );

        assert_eq!(
            result.expect_err("wrong reference kind must fail").code(),
            HybridErrorCode::OwnershipViolation
        );
    }

    #[test]
    fn serializes_and_deserializes_valid_contracts() {
        let original = borrowed_contract();

        let encoded =
            serde_json::to_string(&original).expect("serialization succeeds");
        let decoded: OwnershipContract =
            serde_json::from_str(&encoded).expect("valid contract decodes");

        assert_eq!(original, decoded);
    }

    #[test]
    fn deserialization_cannot_bypass_constructor_validation() {
        let original = borrowed_contract();
        let mut encoded =
            serde_json::to_value(&original).expect("serialization succeeds");

        encoded["lifetime_region"] = serde_json::Value::Null;

        let result = serde_json::from_value::<OwnershipContract>(encoded);

        assert!(
            result.is_err(),
            "invalid scope lifetime must fail during deserialization"
        );
    }

    #[test]
    fn accepts_large_canonical_identifiers_without_local_size_ceilings() {
        let graph = graph(1);
        let large_id = format!("entity:{}", "x".repeat(16_384));

        let contract = OwnershipContract::new(
            entity(
                "hybrid.data.ownership",
                "ownership_contract",
                &large_id,
            ),
            value(graph, 2),
            domain(graph, 3),
            Some(domain(graph, 4)),
            OwnershipExpectation::Borrowed,
            OwnershipOperation::Borrow,
            LifetimeKind::ScopeBound,
            Some(entity("semantic", "lifetime_region", "region:1")),
            AccessRequirement::ReadOnly,
            CompletionRequirement::Synchronous,
            CancellationRequirement::NotApplicable,
            ReleaseResponsibility::Producer,
            CoherenceRequirement::ImmutableShared,
        )
        .expect("no hard-coded identifier-size ceiling");

        assert_eq!(contract.contract_ref().identifier(), large_id);
    }
}
