
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Abstract Cross-Domain Transfers
//!
//! Path: `src/hybrid/data/transfer.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Define validated, target-independent requirements for transferring or
//! exposing a value across computational-domain boundaries.
//!
//! A transfer declaration describes:
//!
//! - the canonical value and exchange involved;
//! - the producing and consuming domain instances;
//! - source and destination representation references;
//! - the abstract transfer mode;
//! - required completion and visibility guarantees;
//! - cancellation and failure obligations;
//! - integrity and ordering requirements;
//! - optional references to canonical transfer and ownership contracts.
//!
//! It does not contain a value payload, copy a buffer, select a device,
//! allocate memory, serialize a domain payload, perform network I/O, submit
//! a hardware job, or execute a runtime lifecycle.
//!
//! ## Canonical ownership
//!
//! - `data::values` owns `CrossDomainValue` and canonical entity references.
//! - `data::exchange` owns `CrossDomainExchange` and representation compatibility.
//! - `data::ownership` owns cross-domain ownership and lifetime obligations.
//! - `hybrid::ids` owns graph-scoped hybrid identities.
//! - `hybrid::errors` owns structured hybrid errors.
//! - The semantic layer owns type, effect, and language ownership semantics.
//! - Domain-specific IRs own their canonical values and representations.
//! - Compiler and backend layers select and lower a realization.
//! - Runtime, transport, and hardware layers perform actual transfers and
//!   enforce completion, cancellation, integrity, and resource-release duties.
//!
//! ## Portability and scalability
//!
//! This module defines no closed list of computational domains, no fixed
//! maximum number of transfers, and no universal hardware-size limits.
//! Transfer collections and identifiers are supplied by their canonical
//! owners. Explicit operational budgets may be enforced by callers.
//!
//! No finite implementation has infinite memory, address space, time, or
//! target capacity. Resource exhaustion must be reported explicitly.
//!
//! ## Semantic safety
//!
//! A valid transfer declaration is not proof that a referenced value,
//! representation, ownership contract, dependency, or completion event exists.
//! Nor does it prove that a conversion preserves semantics or that a runtime
//! has completed the transfer. Those facts must be established by the
//! authoritative graph, semantic, domain, compiler, and runtime layers.
//!
//! An implementation must not silently discard required transfer guarantees
//! to accommodate a target.
//!
//! ## Serialization
//!
//! Deserialization passes through structural validation. Structural validation
//! does not resolve external references or establish runtime completion.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.

use serde::{Deserialize, Serialize};

use crate::hybrid::contract::OwnershipExpectation;
use crate::hybrid::data::exchange::{
    CrossDomainExchange,
    ExchangeOwnership,
    RepresentationCompatibility,
    TransferSemantics,
};
use crate::hybrid::data::ownership::{
    CancellationRequirement,
    CompletionRequirement,
    OwnershipContract,
};
use crate::hybrid::data::values::CanonicalEntityRef;
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{CrossDomainValueId, DomainInstanceId};

/// The abstract transfer behavior required by an exchange.
///
/// These variants describe semantic requirements, not implementation
/// mechanisms. A backend or runtime determines how to satisfy them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransferMode {
    /// The consumer can access the same logical value under the declared
    /// representation and ownership rules.
    ///
    /// This does not require or promise shared physical memory.
    DirectAccess,

    /// The consumer requires a semantically compatible representation
    /// established by an authoritative conversion contract.
    ConvertedRepresentation,

    /// A transfer across the domain boundary is required, but the physical
    /// mechanism remains a downstream implementation decision.
    AbstractTransfer,

    /// The result is not consumable until the declared dependency or
    /// completion condition is satisfied.
    DeferredResult,
}

/// The minimum completion guarantee required by a transfer.
///
/// These are declarative requirements. They are not runtime states and do
/// not themselves wait for an asynchronous operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransferCompletion {
    /// The transfer must be complete before the boundary operation returns.
    BeforeReturn,

    /// The consumer must not use the result until the required completion
    /// condition has been established.
    BeforeUse,

    /// Availability follows the canonical stream contract.
    StreamContract,

    /// Completion semantics are defined by an external canonical contract.
    ExternalContract,
}

/// The cancellation guarantee required for outstanding transfer work.
///
/// Cancellation behavior depends on the underlying operation and runtime.
/// A declaration here does not cancel any physical operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransferCancellation {
    /// No asynchronous cancellable transfer is declared.
    NotApplicable,

    /// Cancellation may be attempted, but success is not guaranteed.
    BestEffort,

    /// Cancellation obligations must propagate according to the canonical
    /// operation, ownership, synchronization, and runtime contracts.
    Propagate,

    /// An external canonical contract defines cancellation behavior.
    ExternalContract,
}

/// The integrity guarantee required at the transfer boundary.
///
/// This describes the observable guarantee, not a particular checksum,
/// cryptographic primitive, wire format, or hardware mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransferIntegrity {
    /// No additional transfer-integrity guarantee is declared here.
    ///
    /// This does not waive guarantees imposed by the language, domain,
    /// security, or canonical value contract.
    Unspecified,

    /// The transferred result must satisfy the canonical representation and
    /// value contract.
    RepresentationPreserved,

    /// The receiver must be able to establish integrity using the
    /// authoritative integrity contract.
    Verifiable,

    /// An external canonical contract defines the required integrity.
    ExternalContract,
}

/// The ordering guarantee required for a transfer.
///
/// This does not prescribe a scheduler, queue, barrier implementation, or
/// distributed-consensus protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransferOrdering {
    /// Only the ordering implied by the canonical dependency graph is required.
    DependencyOrdered,

    /// The transfer must complete before the consumer operation may proceed.
    ConsumerWaitsForCompletion,

    /// Ordering is governed by the canonical stream or synchronization contract.
    StreamOrdered,

    /// An external canonical contract defines ordering.
    ExternalContract,
}

/// The required outcome if a transfer cannot satisfy its contract.
///
/// The runtime or compiler must implement this policy. This enum does not
/// recover from errors or invent a fallback representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransferFailurePolicy {
    /// Report failure; do not present the transfer as successful.
    Fail,

    /// Propagate the failure through the dependent computation.
    Propagate,

    /// Use an explicitly declared alternative from a canonical downstream
    /// plan, but only if that alternative preserves required semantics.
    UseDeclaredAlternative,

    /// Failure handling is governed by an external canonical contract.
    ExternalContract,
}

/// A validated declaration of the transfer requirements for one exchange.
///
/// All identifiers and representations remain references to canonical
/// entities. No domain payload or physical transport handle is stored here.
///
/// Create instances with [`TransferContract::new`] or
/// [`TransferContract::from_exchange`]. Deserialization also validates the
/// local structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "TransferContractWire")]
pub struct TransferContract {
    contract_ref: CanonicalEntityRef,
    value: CrossDomainValueId,
    producer: DomainInstanceId,
    consumer: DomainInstanceId,
    source_representation: CanonicalEntityRef,
    destination_representation: CanonicalEntityRef,
    mode: TransferMode,
    completion: TransferCompletion,
    cancellation: TransferCancellation,
    integrity: TransferIntegrity,
    ordering: TransferOrdering,
    failure_policy: TransferFailurePolicy,
    ownership_contract: Option<CanonicalEntityRef>,
    completion_contract: Option<CanonicalEntityRef>,
    integrity_contract: Option<CanonicalEntityRef>,
}

/// Private deserialization representation.
///
/// Keeping this separate prevents serde from constructing an unchecked
/// public `TransferContract`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TransferContractWire {
    contract_ref: CanonicalEntityRef,
    value: CrossDomainValueId,
    producer: DomainInstanceId,
    consumer: DomainInstanceId,
    source_representation: CanonicalEntityRef,
    destination_representation: CanonicalEntityRef,
    mode: TransferMode,
    completion: TransferCompletion,
    cancellation: TransferCancellation,
    integrity: TransferIntegrity,
    ordering: TransferOrdering,
    failure_policy: TransferFailurePolicy,
    ownership_contract: Option<CanonicalEntityRef>,
    completion_contract: Option<CanonicalEntityRef>,
    integrity_contract: Option<CanonicalEntityRef>,
}

impl TransferContract {
    /// Construct and structurally validate a transfer contract.
    ///
    /// `contract_ref` must identify a canonical entity whose kind is
    /// `transfer_contract`.
    ///
    /// The value, producer, and consumer must belong to the same hybrid
    /// graph. Source and destination representations must be valid canonical
    /// references. Optional contract references must also be structurally
    /// valid.
    ///
    /// This constructor does not resolve those references, prove conversion
    /// correctness, or execute a transfer.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract_ref: CanonicalEntityRef,
        value: CrossDomainValueId,
        producer: DomainInstanceId,
        consumer: DomainInstanceId,
        source_representation: CanonicalEntityRef,
        destination_representation: CanonicalEntityRef,
        mode: TransferMode,
        completion: TransferCompletion,
        cancellation: TransferCancellation,
        integrity: TransferIntegrity,
        ordering: TransferOrdering,
        failure_policy: TransferFailurePolicy,
        ownership_contract: Option<CanonicalEntityRef>,
        completion_contract: Option<CanonicalEntityRef>,
        integrity_contract: Option<CanonicalEntityRef>,
    ) -> Result<Self, HybridError> {
        let contract = Self {
            contract_ref,
            value,
            producer,
            consumer,
            source_representation,
            destination_representation,
            mode,
            completion,
            cancellation,
            integrity,
            ordering,
            failure_policy,
            ownership_contract,
            completion_contract,
            integrity_contract,
        };

        contract.validate()?;
        Ok(contract)
    }

    /// Build a transfer declaration from an already declared exchange.
    ///
    /// The caller supplies a canonical transfer-contract reference and
    /// explicit guarantees. The exchange remains the authority for its value,
    /// domains, representations, ownership expectation, and transfer meaning.
    ///
    /// If an ownership contract is supplied, it must agree with the exchange
    /// and its value/domain identities.
    #[allow(clippy::too_many_arguments)]
    pub fn from_exchange(
        contract_ref: CanonicalEntityRef,
        exchange: &CrossDomainExchange,
        ownership: Option<&OwnershipContract>,
        completion: TransferCompletion,
        cancellation: TransferCancellation,
        integrity: TransferIntegrity,
        ordering: TransferOrdering,
        failure_policy: TransferFailurePolicy,
        completion_contract: Option<CanonicalEntityRef>,
        integrity_contract: Option<CanonicalEntityRef>,
    ) -> Result<Self, HybridError> {
        exchange.validate()?;

        let ownership_ref = exchange.ownership_contract().cloned();

        if let Some(ownership) = ownership {
            ownership.validate()?;

            if ownership.value() != exchange.value().id()
                || ownership.producer() != exchange.producer()
                || ownership.consumer() != Some(exchange.consumer())
            {
                return Err(transfer_error(
                    HybridErrorCode::ContractMismatch,
                    "the ownership contract does not describe this exchange's value, producer, and consumer",
                ));
            }

            if let Some(exchange_ref) = exchange.ownership_contract() {
                if exchange_ref != ownership.contract_ref() {
                    return Err(transfer_error(
                        HybridErrorCode::ContractMismatch,
                        "the exchange and ownership contract refer to different ownership contracts",
                    ));
                }
            }
        }

        let mode = match exchange.transfer() {
            TransferSemantics::Direct => TransferMode::DirectAccess,
            TransferSemantics::Converted => TransferMode::ConvertedRepresentation,
            TransferSemantics::RequiredTransfer => TransferMode::AbstractTransfer,
            TransferSemantics::Deferred => TransferMode::DeferredResult,
        };

        let contract = Self::new(
            contract_ref,
            exchange.value().id(),
            exchange.producer(),
            exchange.consumer(),
            exchange.source_representation().clone(),
            exchange.destination_representation().clone(),
            mode,
            completion,
            cancellation,
            integrity,
            ordering,
            failure_policy,
            ownership_ref,
            completion_contract,
            integrity_contract,
        )?;

        contract.validate_against_exchange(exchange)?;
        if let Some(ownership) = ownership {
            contract.validate_against_ownership(ownership)?;
        }

        Ok(contract)
    }

    /// Return the canonical transfer-contract reference.
    #[must_use]
    pub fn contract_ref(&self) -> &CanonicalEntityRef {
        &self.contract_ref
    }

    /// Return the identity of the cross-domain value.
    #[must_use]
    pub const fn value(&self) -> CrossDomainValueId {
        self.value
    }

    /// Return the producer domain instance.
    #[must_use]
    pub const fn producer(&self) -> DomainInstanceId {
        self.producer
    }

    /// Return the consumer domain instance.
    #[must_use]
    pub const fn consumer(&self) -> DomainInstanceId {
        self.consumer
    }

    /// Return the canonical source representation.
    #[must_use]
    pub fn source_representation(&self) -> &CanonicalEntityRef {
        &self.source_representation
    }

    /// Return the canonical destination representation.
    #[must_use]
    pub fn destination_representation(&self) -> &CanonicalEntityRef {
        &self.destination_representation
    }

    /// Return the abstract transfer mode.
    #[must_use]
    pub const fn mode(&self) -> TransferMode {
        self.mode
    }

    /// Return the required completion guarantee.
    #[must_use]
    pub const fn completion(&self) -> TransferCompletion {
        self.completion
    }

    /// Return the required cancellation guarantee.
    #[must_use]
    pub const fn cancellation(&self) -> TransferCancellation {
        self.cancellation
    }

    /// Return the required integrity guarantee.
    #[must_use]
    pub const fn integrity(&self) -> TransferIntegrity {
        self.integrity
    }

    /// Return the required ordering guarantee.
    #[must_use]
    pub const fn ordering(&self) -> TransferOrdering {
        self.ordering
    }

    /// Return the declared transfer failure policy.
    #[must_use]
    pub const fn failure_policy(&self) -> TransferFailurePolicy {
        self.failure_policy
    }

    /// Return the optional canonical ownership-contract reference.
    #[must_use]
    pub fn ownership_contract(&self) -> Option<&CanonicalEntityRef> {
        self.ownership_contract.as_ref()
    }

    /// Return the optional canonical completion-contract reference.
    #[must_use]
    pub fn completion_contract(&self) -> Option<&CanonicalEntityRef> {
        self.completion_contract.as_ref()
    }

    /// Return the optional canonical integrity-contract reference.
    #[must_use]
    pub fn integrity_contract(&self) -> Option<&CanonicalEntityRef> {
        self.integrity_contract.as_ref()
    }

    /// Validate local structural invariants.
    ///
    /// This does not resolve graph membership, canonical references, runtime
    /// completion, or implementation conformance.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.contract_ref.validate()?;

        if self.contract_ref.kind() != "transfer_contract" {
            return Err(transfer_error(
                HybridErrorCode::InvalidContract,
                "the canonical transfer reference must have kind `transfer_contract`",
            ));
        }

        let graph = self.value.graph();

        if self.producer.graph() != graph || self.consumer.graph() != graph {
            return Err(transfer_error(
                HybridErrorCode::InvalidReference,
                "the transfer value, producer, and consumer must belong to the same hybrid graph",
            ));
        }

        if self.producer == self.consumer {
            return Err(transfer_error(
                HybridErrorCode::InvalidContract,
                "a cross-domain transfer must declare distinct producer and consumer domain instances",
            ));
        }

        self.source_representation
            .validate()
            .map_err(|error| error.in_component("hybrid transfer source representation"))?;

        self.destination_representation
            .validate()
            .map_err(|error| error.in_component("hybrid transfer destination representation"))?;

        validate_optional_reference(
            self.ownership_contract.as_ref(),
            "hybrid transfer ownership contract",
        )?;
        validate_optional_reference(
            self.completion_contract.as_ref(),
            "hybrid transfer completion contract",
        )?;
        validate_optional_reference(
            self.integrity_contract.as_ref(),
            "hybrid transfer integrity contract",
        )?;

        if self.completion_contract.is_none()
            && self.completion == TransferCompletion::ExternalContract
        {
            return Err(transfer_error(
                HybridErrorCode::InvalidContract,
                "external completion semantics require a canonical completion-contract reference",
            ));
        }

        if self.integrity_contract.is_none()
            && self.integrity == TransferIntegrity::ExternalContract
        {
            return Err(transfer_error(
                HybridErrorCode::InvalidContract,
                "external integrity semantics require a canonical integrity-contract reference",
            ));
        }

        if self.cancellation == TransferCancellation::Propagate
            && self.completion == TransferCompletion::BeforeReturn
        {
            return Err(transfer_error(
                HybridErrorCode::InvalidContract,
                "cancellation propagation is inconsistent with a transfer declared strictly complete before return",
            ));
        }

        Ok(())
    }

    /// Check that this transfer contract agrees with the exchange declaration.
    ///
    /// The exchange remains authoritative for the value, producer,
    /// consumer, representations, and abstract transfer semantics.
    pub fn validate_against_exchange(
        &self,
        exchange: &CrossDomainExchange,
    ) -> Result<(), HybridError> {
        self.validate()?;
        exchange.validate()?;

        if self.value != exchange.value().id()
            || self.producer != exchange.producer()
            || self.consumer != exchange.consumer()
        {
            return Err(transfer_error(
                HybridErrorCode::ContractMismatch,
                "the transfer contract does not match the exchange's value or domain identities",
            ));
        }

        if self.source_representation != *exchange.source_representation()
            || self.destination_representation != *exchange.destination_representation()
        {
            return Err(transfer_error(
                HybridErrorCode::ContractMismatch,
                "the transfer contract's representations differ from the exchange representations",
            ));
        }

        let expected_mode = match exchange.transfer() {
            TransferSemantics::Direct => TransferMode::DirectAccess,
            TransferSemantics::Converted => TransferMode::ConvertedRepresentation,
            TransferSemantics::RequiredTransfer => TransferMode::AbstractTransfer,
            TransferSemantics::Deferred => TransferMode::DeferredResult,
        };

        if self.mode != expected_mode {
            return Err(transfer_error(
                HybridErrorCode::ContractMismatch,
                "the declared transfer mode conflicts with the exchange's transfer semantics",
            ));
        }

        if let Some(exchange_ownership) = exchange.ownership_contract() {
            if self.ownership_contract.as_ref() != Some(exchange_ownership) {
                return Err(transfer_error(
                    HybridErrorCode::ContractMismatch,
                    "the transfer and exchange refer to different ownership contracts",
                ));
            }
        }

        if exchange.compatibility() == RepresentationCompatibility::Incompatible {
            return Err(transfer_error(
                HybridErrorCode::ValueIncompatibility,
                "an incompatible exchange cannot be made valid by declaring a transfer contract",
            ));
        }

        if exchange.compatibility() == RepresentationCompatibility::RequiresConversion
            && self.mode != TransferMode::ConvertedRepresentation
        {
            return Err(transfer_error(
                HybridErrorCode::ContractMismatch,
                "the exchange requires a declared conversion but the transfer mode does not represent one",
            ));
        }

        if self.mode == TransferMode::ConvertedRepresentation
            && exchange.conversion().is_none()
        {
            return Err(transfer_error(
                HybridErrorCode::InvalidContract,
                "a converted transfer requires the exchange to reference its canonical conversion",
            ));
        }

        Ok(())
    }

    /// Check that this transfer contract agrees with the canonical ownership
    /// contract for the same exchange.
    ///
    /// This verifies declared identity and conservative consistency only.
    /// The semantic layer must still establish that ownership and lifetime
    /// rules are valid for the program.
    pub fn validate_against_ownership(
        &self,
        ownership: &OwnershipContract,
    ) -> Result<(), HybridError> {
        self.validate()?;
        ownership.validate()?;

        if self.value != ownership.value()
            || self.producer != ownership.producer()
            || ownership.consumer() != Some(self.consumer)
        {
            return Err(transfer_error(
                HybridErrorCode::OwnershipViolation,
                "the transfer and ownership contract disagree about the value or participating domains",
            ));
        }

        if let Some(reference) = &self.ownership_contract {
            if reference != ownership.contract_ref() {
                return Err(transfer_error(
                    HybridErrorCode::OwnershipViolation,
                    "the transfer references a different canonical ownership contract",
                ));
            }
        } else {
            return Err(transfer_error(
                HybridErrorCode::OwnershipViolation,
                "the transfer has no ownership-contract reference for the supplied ownership contract",
            ));
        }

        match ownership.expectation() {
            OwnershipExpectation::Borrowed => {
                if self.mode == TransferMode::AbstractTransfer
                    && ownership.completion() == CompletionRequirement::Synchronous
                    && self.completion != TransferCompletion::BeforeReturn
                {
                    return Err(transfer_error(
                        HybridErrorCode::OwnershipViolation,
                        "the transfer completion requirement weakens the synchronous borrowed-value contract",
                    ));
                }
            }
            OwnershipExpectation::Transferred => {
                if ownership.consumer() != Some(self.consumer) {
                    return Err(transfer_error(
                        HybridErrorCode::OwnershipViolation,
                        "a transferred value must identify the receiving domain in its ownership contract",
                    ));
                }
            }
            OwnershipExpectation::Owned
            | OwnershipExpectation::Shared
            | OwnershipExpectation::External => {}
        }

        if ownership.cancellation() == CancellationRequirement::Propagate
            && self.cancellation != TransferCancellation::Propagate
        {
            return Err(transfer_error(
                HybridErrorCode::OwnershipViolation,
                "the transfer cancellation guarantee weakens the ownership contract",
            ));
        }

        Ok(())
    }
}

impl TryFrom<TransferContractWire> for TransferContract {
    type Error = HybridError;

    fn try_from(wire: TransferContractWire) -> Result<Self, Self::Error> {
        Self::new(
            wire.contract_ref,
            wire.value,
            wire.producer,
            wire.consumer,
            wire.source_representation,
            wire.destination_representation,
            wire.mode,
            wire.completion,
            wire.cancellation,
            wire.integrity,
            wire.ordering,
            wire.failure_policy,
            wire.ownership_contract,
            wire.completion_contract,
            wire.integrity_contract,
        )
    }
}

fn validate_optional_reference(
    reference: Option<&CanonicalEntityRef>,
    component: &'static str,
) -> Result<(), HybridError> {
    if let Some(reference) = reference {
        reference
            .validate()
            .map_err(|error| error.in_component(component))?;
    }

    Ok(())
}

fn transfer_error(
    code: HybridErrorCode,
    message: impl Into<String>,
) -> HybridError {
    HybridError::new(code, message).in_component("hybrid data transfer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_mode_preserves_distinct_semantics() {
        assert_ne!(TransferMode::DirectAccess, TransferMode::AbstractTransfer);
        assert_ne!(
            TransferMode::ConvertedRepresentation,
            TransferMode::DeferredResult
        );
    }

    #[test]
    fn completion_guarantees_are_not_interchangeable() {
        assert_ne!(TransferCompletion::BeforeReturn, TransferCompletion::BeforeUse);
        assert_ne!(TransferCompletion::StreamContract, TransferCompletion::ExternalContract);
    }

    #[test]
    fn failure_policies_are_explicit() {
        assert_ne!(TransferFailurePolicy::Fail, TransferFailurePolicy::Propagate);
        assert_ne!(
            TransferFailurePolicy::UseDeclaredAlternative,
            TransferFailurePolicy::ExternalContract
        );
    }

    #[test]
    fn integrity_guarantees_are_explicit() {
        assert_ne!(
            TransferIntegrity::Unspecified,
            TransferIntegrity::RepresentationPreserved
        );
        assert_ne!(
            TransferIntegrity::Verifiable,
            TransferIntegrity::ExternalContract
        );
    }
}
