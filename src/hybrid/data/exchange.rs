#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Cross-Domain Data Exchange
//!
//! Path: `src/hybrid/data/exchange.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! ## Purpose
//!
//! Define the validated contract for exchanging a canonical value between
//! computational domains in one hybrid computation.
//!
//! This module describes:
//! - the value being exchanged;
//! - its producing and consuming domain instances;
//! - source and destination representations;
//! - representation compatibility;
//! - an explicitly declared conversion, when required;
//! - availability and dependency conditions;
//! - ownership expectations;
//! - transfer semantics;
//! - structured reasons why an exchange cannot proceed.
//!
//! It does not materialize values, execute conversions, copy buffers, submit
//! device work, or implement network communication.
//!
//! ## Canonical ownership
//!
//! - `data::values` owns cross-domain value references.
//! - `hybrid::ids` owns hybrid graph and domain-instance identities.
//! - `hybrid::errors` owns structured hybrid failures.
//! - `hybrid::contract` owns domain interface and port contracts.
//! - The canonical semantic layer owns type, effect, ownership, and lifetime
//!   rules.
//! - Domain-specific IRs own their representations and conversion semantics.
//! - `data::ownership` owns ownership/lifetime enforcement.
//! - `data::layout` owns abstract data-layout constraints.
//! - `data::transfer` owns transfer requirements and completion contracts.
//! - Compiler, backend, runtime, and networking layers realize exchanges.
//!
//! This module introduces no second IR, type system, runtime-value enum,
//! physical transfer engine, or domain registry.
//!
//! ## Scalability
//!
//! No fixed maximum number of exchanges, domains, values, or representation
//! kinds is imposed here. Collections and identifiers are supplied by their
//! canonical owners. Actual execution remains bounded by available memory,
//! compute resources, target capabilities, and explicitly configured policies.
//!
//! ## Semantic safety
//!
//! A reference does not prove that an entity exists. A declaration of
//! compatibility does not prove that a conversion preserves semantics.
//! An availability declaration does not prove that a runtime result exists.
//! Those facts must be established by the relevant authoritative subsystem.
//!
//! Unknown, incompatible, or unresolved required semantics must not silently
//! become successful exchanges.
//!
//! ## Integration
//!
//! `CrossDomainExchange` consumes `CrossDomainValue` without copying or
//! redefining its payload. Graph verification resolves graph-local references;
//! domain adapters resolve canonical entities; ownership validation checks
//! lifetime and access rules; the transfer subsystem determines how a valid
//! exchange can be realized.
//!
//! This file deliberately does not import the compiler, runtime, hardware,
//! quantum IR, classical IR, or networking implementation.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.

use serde::{Deserialize, Serialize};

use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::DomainInstanceId;
use crate::hybrid::data::values::{
    CanonicalEntityRef,
    CrossDomainValue,
};

/// Describes when the source value is available to a consumer.
///
/// These are semantic declarations, not runtime states. The runtime or
/// authoritative dependency tracker must establish actual readiness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ValueAvailability {
    /// The value is declared available before the consuming operation begins.
    ///
    /// This declaration must still be validated against the producer and
    /// dependency graph.
    Immediate,

    /// Availability depends on a canonical dependency becoming satisfied.
    ///
    /// The referenced entity is opaque here. Its meaning and existence must
    /// be resolved by the dependency authority.
    AfterDependency(CanonicalEntityRef),

    /// The value is deferred until a canonical completion/result entity
    /// becomes available.
    ///
    /// This does not prescribe futures, queues, events, or any other runtime
    /// mechanism.
    Deferred(CanonicalEntityRef),
}

impl ValueAvailability {
    /// Validate the structure of this availability declaration.
    ///
    /// This method does not resolve referenced entities or wait for results.
    pub fn validate(&self) -> Result<(), HybridError> {
        match self {
            Self::Immediate => Ok(()),
            Self::AfterDependency(reference) | Self::Deferred(reference) => {
                reference.validate().map_err(|error| {
                    error.in_component("hybrid data exchange availability")
                })
            }
        }
    }
}

/// Declares the relationship between source and destination representations.
///
/// Compatibility is a semantic claim that must be established by canonical
/// type information and domain adapters. This enum does not perform type
/// inference or representation conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RepresentationCompatibility {
    /// The source and destination representations are identical under the
    /// authoritative representation contract.
    Exact,

    /// The representations differ but can interoperate without an explicit
    /// value conversion.
    CompatibleWithoutConversion,

    /// A specific, authoritative conversion must be applied.
    RequiresConversion,

    /// The representations cannot interoperate under the declared contract.
    Incompatible,
}

/// Declares the ownership expectation at the exchange boundary.
///
/// This declaration does not replace the canonical language ownership
/// system or prove that a borrow, move, or shared access is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExchangeOwnership {
    /// The consumer borrows the value under an authoritative lifetime
    /// contract. The producer remains responsible for its ownership.
    Borrowed,

    /// Ownership is transferred to the consumer according to the canonical
    /// ownership contract.
    Owned,

    /// Multiple consumers may share immutable access, provided the canonical
    /// ownership and concurrency rules permit it.
    SharedImmutable,

    /// The value is deferred; ownership and lifetime must remain valid until
    /// the deferred result becomes available.
    Deferred,
}

/// Describes the semantic transfer behavior required at a domain boundary.
///
/// This is intentionally independent of physical transport. A runtime may
/// implement a transfer through shared memory, a device interface, a
/// serialized message, a remote call, or another conforming mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransferSemantics {
    /// The consumer accesses the same logical value under the declared
    /// representation and ownership contract.
    Direct,

    /// The consumer receives a logically equivalent representation produced
    /// by an explicitly declared conversion.
    Converted,

    /// The exchange requires an abstract transfer whose physical mechanism
    /// is selected downstream.
    RequiredTransfer,

    /// The consumer receives a deferred result after the required dependency
    /// or completion condition is satisfied.
    Deferred,
}

/// A validated declaration of one cross-domain value exchange.
///
/// The structure references canonical values and representations. It does
/// not contain the payload itself.
///
/// Instances must be created through [`CrossDomainExchange::new`] or
/// validated explicitly after deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "CrossDomainExchangeWire")]
pub struct CrossDomainExchange {
    value: CrossDomainValue,
    consumer: DomainInstanceId,
    source_representation: CanonicalEntityRef,
    destination_representation: CanonicalEntityRef,
    compatibility: RepresentationCompatibility,
    conversion: Option<CanonicalEntityRef>,
    availability: ValueAvailability,
    ownership: ExchangeOwnership,
    transfer: TransferSemantics,
    ownership_contract: Option<CanonicalEntityRef>,
}

/// Private wire structure ensures deserialization passes through validation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CrossDomainExchangeWire {
    value: CrossDomainValue,
    consumer: DomainInstanceId,
    source_representation: CanonicalEntityRef,
    destination_representation: CanonicalEntityRef,
    compatibility: RepresentationCompatibility,
    conversion: Option<CanonicalEntityRef>,
    availability: ValueAvailability,
    ownership: ExchangeOwnership,
    transfer: TransferSemantics,
    ownership_contract: Option<CanonicalEntityRef>,
}

impl CrossDomainExchange {
    /// Construct a cross-domain exchange declaration and validate all local
    /// structural invariants.
    ///
    /// `value` must be a canonical hybrid reference to a value produced by
    /// `value.producer()`. `consumer` must belong to the same hybrid graph.
    ///
    /// Representation and conversion references must be supplied by their
    /// canonical owners. This constructor does not resolve them or prove that
    /// their conversion is semantics-preserving.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        value: CrossDomainValue,
        consumer: DomainInstanceId,
        source_representation: CanonicalEntityRef,
        destination_representation: CanonicalEntityRef,
        compatibility: RepresentationCompatibility,
        conversion: Option<CanonicalEntityRef>,
        availability: ValueAvailability,
        ownership: ExchangeOwnership,
        transfer: TransferSemantics,
        ownership_contract: Option<CanonicalEntityRef>,
    ) -> Result<Self, HybridError> {
        let exchange = Self {
            value,
            consumer,
            source_representation,
            destination_representation,
            compatibility,
            conversion,
            availability,
            ownership,
            transfer,
            ownership_contract,
        };

        exchange.validate()?;
        Ok(exchange)
    }

    /// Return the canonical cross-domain value reference.
    #[must_use]
    pub fn value(&self) -> &CrossDomainValue {
        &self.value
    }

    /// Return the producing domain-instance identity.
    #[must_use]
    pub const fn producer(&self) -> DomainInstanceId {
        self.value.producer()
    }

    /// Return the consuming domain-instance identity.
    #[must_use]
    pub const fn consumer(&self) -> DomainInstanceId {
        self.consumer
    }

    /// Return the canonical source representation reference.
    #[must_use]
    pub fn source_representation(&self) -> &CanonicalEntityRef {
        &self.source_representation
    }

    /// Return the canonical destination representation reference.
    #[must_use]
    pub fn destination_representation(&self) -> &CanonicalEntityRef {
        &self.destination_representation
    }

    /// Return the declared representation compatibility.
    #[must_use]
    pub const fn compatibility(&self) -> RepresentationCompatibility {
        self.compatibility
    }

    /// Return the explicitly declared conversion, if one is required.
    #[must_use]
    pub fn conversion(&self) -> Option<&CanonicalEntityRef> {
        self.conversion.as_ref()
    }

    /// Return the declared availability condition.
    #[must_use]
    pub fn availability(&self) -> &ValueAvailability {
        &self.availability
    }

    /// Return the exchange-boundary ownership expectation.
    #[must_use]
    pub const fn ownership(&self) -> ExchangeOwnership {
        self.ownership
    }

    /// Return the abstract transfer semantics.
    #[must_use]
    pub const fn transfer(&self) -> TransferSemantics {
        self.transfer
    }

    /// Return the canonical ownership contract, when supplied.
    ///
    /// An absent reference does not imply unrestricted ownership or copying.
    #[must_use]
    pub fn ownership_contract(&self) -> Option<&CanonicalEntityRef> {
        self.ownership_contract.as_ref()
    }

    /// Validate structural consistency and cross-domain invariants.
    ///
    /// This validates only the exchange declaration. It does not establish
    /// that graph nodes, domain instances, canonical representations,
    /// dependencies, types, ownership contracts, or conversion implementations
    /// exist or are executable.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.value.validate()?;

        if self.value.graph_id() != self.consumer.graph() {
            return Err(exchange_error(
                HybridErrorCode::InvalidReference,
                "the exchanged value and consumer belong to different hybrid graphs",
            ));
        }

        self.source_representation
            .validate()
            .map_err(|error| error.in_component("hybrid data exchange source"))?;

        self.destination_representation
            .validate()
            .map_err(|error| {
                error.in_component("hybrid data exchange destination")
            })?;

        self.availability
            .validate()
            .map_err(|error| {
                error.in_component("hybrid data exchange availability")
            })?;

        if let Some(conversion) = &self.conversion {
            conversion.validate().map_err(|error| {
                error.in_component("hybrid data exchange conversion")
            })?;
        }

        if let Some(contract) = &self.ownership_contract {
            contract.validate().map_err(|error| {
                error.in_component("hybrid data exchange ownership")
            })?;
        }

        self.validate_representation_contract()?;
        self.validate_transfer_contract()?;
        self.validate_ownership_contract()
    }

    /// Validate that the conversion declaration agrees with the declared
    /// representation compatibility.
    fn validate_representation_contract(&self) -> Result<(), HybridError> {
        match (self.compatibility, self.conversion.as_ref()) {
            (RepresentationCompatibility::Exact, None)
            | (
                RepresentationCompatibility::CompatibleWithoutConversion,
                None,
            )
            | (
                RepresentationCompatibility::Incompatible,
                None,
            ) => Ok(()),

            (RepresentationCompatibility::RequiresConversion, Some(_)) => {
                if self.transfer != TransferSemantics::Converted {
                    return Err(exchange_error(
                        HybridErrorCode::InvalidContract,
                        "a required representation conversion must use converted transfer semantics",
                    ));
                }

                Ok(())
            }

            (RepresentationCompatibility::RequiresConversion, None) => {
                Err(exchange_error(
                    HybridErrorCode::ValueIncompatibility,
                    "the exchange requires a conversion but no canonical conversion reference was supplied",
                ))
            }

            (_, Some(_)) => Err(exchange_error(
                HybridErrorCode::InvalidContract,
                "a conversion reference was supplied when the compatibility declaration does not require one",
            )),
        }
    }

    /// Validate the consistency of transfer semantics and availability.
    fn validate_transfer_contract(&self) -> Result<(), HybridError> {
        match self.transfer {
            TransferSemantics::Converted => {
                if self.compatibility
                    != RepresentationCompatibility::RequiresConversion
                {
                    return Err(exchange_error(
                        HybridErrorCode::InvalidContract,
                        "converted transfer semantics require an explicit representation conversion",
                    ));
                }
            }

            TransferSemantics::Deferred => {
                if !matches!(
                    self.availability,
                    ValueAvailability::AfterDependency(_)
                        | ValueAvailability::Deferred(_)
                ) {
                    return Err(exchange_error(
                        HybridErrorCode::InvalidContract,
                        "deferred transfer semantics require an explicit dependency or completion reference",
                    ));
                }

                if self.ownership != ExchangeOwnership::Deferred {
                    return Err(exchange_error(
                        HybridErrorCode::InvalidContract,
                        "deferred transfer semantics require deferred ownership semantics",
                    ));
                }
            }

            TransferSemantics::Direct | TransferSemantics::RequiredTransfer => {
                if self.ownership == ExchangeOwnership::Deferred {
                    return Err(exchange_error(
                        HybridErrorCode::InvalidContract,
                        "deferred ownership requires deferred transfer semantics",
                    ));
                }
            }
        }

        Ok(())
    }

    /// Validate ownership-related declarations without duplicating the
    /// canonical language ownership checker.
    fn validate_ownership_contract(&self) -> Result<(), HybridError> {
        if self.ownership == ExchangeOwnership::Deferred
            && !matches!(
                self.availability,
                ValueAvailability::AfterDependency(_)
                    | ValueAvailability::Deferred(_)
            )
        {
            return Err(exchange_error(
                HybridErrorCode::OwnershipViolation,
                "deferred ownership requires an explicit availability dependency",
            ));
        }

        if self.ownership == ExchangeOwnership::Owned
            && self.ownership_contract.is_none()
            && self.value.ownership_contract().is_none()
        {
            return Err(exchange_error(
                HybridErrorCode::OwnershipViolation,
                "owned exchange requires an authoritative ownership contract",
            ));
        }

        Ok(())
    }

    /// Return whether this declaration is structurally capable of proceeding.
    ///
    /// `false` means the declared representations are incompatible. `true`
    /// does not prove that dependencies are satisfied or that the runtime can
    /// perform the exchange.
    #[must_use]
    pub const fn is_representation_compatible(&self) -> bool {
        !matches!(
            self.compatibility,
            RepresentationCompatibility::Incompatible
        )
    }
}

impl TryFrom<CrossDomainExchangeWire> for CrossDomainExchange {
    type Error = HybridError;

    fn try_from(wire: CrossDomainExchangeWire) -> Result<Self, Self::Error> {
        Self::new(
            wire.value,
            wire.consumer,
            wire.source_representation,
            wire.destination_representation,
            wire.compatibility,
            wire.conversion,
            wire.availability,
            wire.ownership,
            wire.transfer,
            wire.ownership_contract,
        )
    }
}

/// Construct a structured exchange error with consistent component context.
fn exchange_error(
    code: HybridErrorCode,
    message: impl Into<String>,
) -> HybridError {
    HybridError::new(code, message).in_component("hybrid data exchange")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::data::values::{
        CanonicalTypeRef,
        ValueProvenance,
    };
    use crate::hybrid::ids::{
        CrossDomainValueId,
        GraphScopedId,
        HybridGraphId,
        IdComponent,
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

    fn value_id(graph: HybridGraphId, local: u128) -> CrossDomainValueId {
        CrossDomainValueId::new(GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("non-zero local ID"),
        ))
    }

    fn reference(
        namespace: &str,
        kind: &str,
        identifier: &str,
    ) -> CanonicalEntityRef {
        CanonicalEntityRef::new(namespace, kind, identifier)
            .expect("valid canonical reference")
    }

    fn value(graph: HybridGraphId) -> CrossDomainValue {
        CrossDomainValue::new(
            value_id(graph, 1),
            domain(graph, 2),
            CanonicalTypeRef::new("semantic", "i32")
                .expect("valid type reference"),
            reference("classical.ir", "value", "v1"),
            None,
            Vec::new(),
            ValueProvenance::new(None, Vec::new())
                .expect("valid provenance"),
        )
        .expect("valid cross-domain value")
    }

    fn direct_exchange() -> CrossDomainExchange {
        let graph = graph(1);

        CrossDomainExchange::new(
            value(graph),
            domain(graph, 3),
            reference("classical.ir", "representation", "native"),
            reference("classical.ir", "representation", "native"),
            RepresentationCompatibility::Exact,
            None,
            ValueAvailability::Immediate,
            ExchangeOwnership::Borrowed,
            TransferSemantics::Direct,
            None,
        )
        .expect("valid exchange")
    }

    #[test]
    fn accepts_valid_direct_exchange() {
        let exchange = direct_exchange();

        assert_eq!(exchange.producer(), domain(graph(1), 2));
        assert_eq!(exchange.consumer(), domain(graph(1), 3));
        assert!(exchange.is_representation_compatible());
        assert!(exchange.validate().is_ok());
    }

    #[test]
    fn rejects_consumer_from_another_graph() {
        let graph = graph(1);

        let result = CrossDomainExchange::new(
            value(graph),
            domain(graph(2), 3),
            reference("classical.ir", "representation", "native"),
            reference("classical.ir", "representation", "native"),
            RepresentationCompatibility::Exact,
            None,
            ValueAvailability::Immediate,
            ExchangeOwnership::Borrowed,
            TransferSemantics::Direct,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_conversion_without_conversion_reference() {
        let graph = graph(1);

        let result = CrossDomainExchange::new(
            value(graph),
            domain(graph, 3),
            reference("classical.ir", "representation", "source"),
            reference("accelerator.ir", "representation", "destination"),
            RepresentationCompatibility::RequiresConversion,
            None,
            ValueAvailability::Immediate,
            ExchangeOwnership::Borrowed,
            TransferSemantics::Converted,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_incompatible_representation_as_a_direct_transfer() {
        let graph = graph(1);

        let result = CrossDomainExchange::new(
            value(graph),
            domain(graph, 3),
            reference("classical.ir", "representation", "source"),
            reference("quantum.ir", "representation", "destination"),
            RepresentationCompatibility::Incompatible,
            None,
            ValueAvailability::Immediate,
            ExchangeOwnership::Borrowed,
            TransferSemantics::Direct,
            None,
        )
        .expect("declaration is structurally valid");

        assert!(!result.is_representation_compatible());
    }

    #[test]
    fn rejects_deferred_transfer_without_dependency() {
        let graph = graph(1);

        let result = CrossDomainExchange::new(
            value(graph),
            domain(graph, 3),
            reference("classical.ir", "representation", "native"),
            reference("classical.ir", "representation", "native"),
            RepresentationCompatibility::Exact,
            None,
            ValueAvailability::Immediate,
            ExchangeOwnership::Deferred,
            TransferSemantics::Deferred,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_owned_exchange_without_ownership_contract() {
        let graph = graph(1);

        let result = CrossDomainExchange::new(
            value(graph),
            domain(graph, 3),
            reference("classical.ir", "representation", "native"),
            reference("classical.ir", "representation", "native"),
            RepresentationCompatibility::Exact,
            None,
            ValueAvailability::Immediate,
            ExchangeOwnership::Owned,
            TransferSemantics::Direct,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_malformed_canonical_reference() {
        let result = CanonicalEntityRef::new(
            "classical.ir",
            "representation",
            " ",
        );

        assert!(result.is_err());
    }

    #[test]
    fn exchange_round_trip_preserves_contract() {
        let original = direct_exchange();
        let encoded =
            serde_json::to_string(&original).expect("serialize exchange");

        let decoded: CrossDomainExchange =
            serde_json::from_str(&encoded).expect("deserialize exchange");

        assert_eq!(decoded, original);
        assert!(decoded.validate().is_ok());
    }
}