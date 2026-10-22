
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Cross-Domain Value References
//!
//! Path: `src/hybrid/data/values.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Represent values that participate in, or cross boundaries within, a
//! hybrid computation without replacing the canonical value and type
//! representations owned by the participating domains.
//!
//! A `CrossDomainValue` is a validated hybrid-layer reference to:
//!
//! - a value identity owned by a canonical domain IR;
//! - a semantic type reference owned by the canonical semantic/type system;
//! - the hybrid graph and producing domain instance;
//! - optional references to canonical ownership and effect contracts;
//! - provenance describing the source and derivation of the value.
//!
//! This module does not contain a universal runtime-value enum. In
//! particular, it does not copy or reinterpret quantum states, tensors,
//! streams, distributed objects, hardware buffers, or classical IR values.
//!
//! ## Canonical ownership
//!
//! - `hybrid::ids` owns `CrossDomainValueId` and `DomainInstanceId`.
//! - `hybrid::errors` owns structured hybrid errors.
//! - `hybrid::contract` owns domain interface and port contracts.
//! - The canonical semantic layer owns semantic types, effects, and ownership
//!   rules.
//! - `quantum::ir` owns quantum IR and its canonical value references.
//! - Existing classical IR modules own classical value semantics.
//! - Domain-specific IR modules own their respective values.
//! - `data::exchange`, `data::ownership`, and `data::transfer`, when
//!   implemented, own exchange, ownership enforcement, and transfer policy.
//! - Graph verification owns whole-graph reference and dependency checks.
//! - Compiler, backend, and runtime modules own realization and execution.
//!
//! References represented here are identifiers, not proof that the
//! referenced entity exists, is available, or is executable.
//!
//! ## Scalability
//!
//! This module imposes no fixed maximum number of values, effects,
//! provenance entries, domains, or graph instances. Collections grow
//! according to available resources and explicit operational policies.
//!
//! Rust strings and collections are finite representations. This module
//! makes no claim that physical memory or computation is infinite.
//!
//! ## Serialization
//!
//! Deserialization validates the structure of every reference and value.
//! Structural validation does not resolve references against an IR, semantic
//! model, graph, or registry. Those checks belong to their authoritative
//! owners and integration adapters.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.

use serde::{Deserialize, Serialize};

use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{CrossDomainValueId, DomainInstanceId};

/// A stable reference to an entity owned by another Zamani subsystem.
///
/// `namespace` identifies the canonical owner or namespace, `kind` describes
/// the referenced entity category, and `identifier` preserves the owner's
/// stable identifier in its native textual form.
///
/// The identifier is intentionally opaque. This type does not assume that
/// all domain IRs use the same integer width, ID type, or identity scheme.
///
/// Examples of the *shape* of a reference include:
///
/// - namespace `quantum.ir`, kind `value`, identifier supplied by quantum IR;
/// - namespace `classical.ir`, kind `value`, identifier supplied by classical IR;
/// - namespace `semantic`, kind `type`, identifier supplied by semantic analysis.
///
/// These are examples of reference namespaces, not a closed registry of
/// permitted domains or entity kinds.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(try_from = "CanonicalEntityRefWire")]
pub struct CanonicalEntityRef {
    namespace: String,
    kind: String,
    identifier: String,
}

/// Serde wire representation. Conversion through `TryFrom` prevents
/// deserialization from bypassing constructor validation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CanonicalEntityRefWire {
    namespace: String,
    kind: String,
    identifier: String,
}

impl CanonicalEntityRef {
    /// Construct a reference to an entity owned by a canonical subsystem.
    ///
    /// The constructor validates non-empty, printable identifiers. It does
    /// not impose a fixed length limit or resolve the reference.
    pub fn new(
        namespace: impl Into<String>,
        kind: impl Into<String>,
        identifier: impl Into<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            namespace: namespace.into(),
            kind: kind.into(),
            identifier: identifier.into(),
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the canonical owner or namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the referenced entity category.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the identifier exactly as supplied by its canonical owner.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Validate this reference, including after data has crossed a trust
    /// boundary.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_reference_component("namespace", &self.namespace)?;
        validate_reference_component("entity kind", &self.kind)?;
        validate_reference_component("entity identifier", &self.identifier)
    }
}

impl TryFrom<CanonicalEntityRefWire> for CanonicalEntityRef {
    type Error = HybridError;

    fn try_from(wire: CanonicalEntityRefWire) -> Result<Self, Self::Error> {
        Self::new(wire.namespace, wire.kind, wire.identifier)
    }
}

/// A reference to a type in the canonical semantic type system.
///
/// This is a reference only. It is not a new type system, a type-inference
/// result, or proof that two domain representations are interchangeable.
///
/// Type equivalence and compatibility must be established by the canonical
/// semantic/type authority and the appropriate domain adapters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(try_from = "CanonicalTypeRefWire")]
pub struct CanonicalTypeRef {
    namespace: String,
    identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CanonicalTypeRefWire {
    namespace: String,
    identifier: String,
}

impl CanonicalTypeRef {
    /// Construct a reference to a canonical semantic type.
    pub fn new(
        namespace: impl Into<String>,
        identifier: impl Into<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            namespace: namespace.into(),
            identifier: identifier.into(),
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the canonical semantic namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the canonical type identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Return a reference suitable for diagnostics and logs.
    ///
    /// This is a display representation, not a serialization format or
    /// canonical identity encoding.
    #[must_use]
    pub fn display_name(&self) -> String {
        format!("{}::{}", self.namespace, self.identifier)
    }

    /// Validate the structure of the reference.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_reference_component("type namespace", &self.namespace)?;
        validate_reference_component("type identifier", &self.identifier)
    }
}

impl TryFrom<CanonicalTypeRefWire> for CanonicalTypeRef {
    type Error = HybridError;

    fn try_from(wire: CanonicalTypeRefWire) -> Result<Self, Self::Error> {
        Self::new(wire.namespace, wire.identifier)
    }
}

/// Provenance for a cross-domain value.
///
/// Provenance records references to authoritative source entities; it does
/// not reproduce source spans, AST nodes, semantic expressions, or IR nodes.
///
/// `source` can identify the original source-level or upstream entity when
/// the integration adapter has that information. `derivation` records
/// canonical references associated with the value's derivation, in declared
/// order. This type does not claim that those transformations are valid;
/// that must be established by the subsystem that owns their semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "ValueProvenanceWire")]
pub struct ValueProvenance {
    source: Option<CanonicalEntityRef>,
    derivation: Vec<CanonicalEntityRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ValueProvenanceWire {
    source: Option<CanonicalEntityRef>,
    derivation: Vec<CanonicalEntityRef>,
}

impl ValueProvenance {
    /// Construct provenance from canonical references.
    pub fn new(
        source: Option<CanonicalEntityRef>,
        derivation: Vec<CanonicalEntityRef>,
    ) -> Result<Self, HybridError> {
        let value = Self { source, derivation };
        value.validate()?;
        Ok(value)
    }

    /// Return the optional original source reference.
    #[must_use]
    pub fn source(&self) -> Option<&CanonicalEntityRef> {
        self.source.as_ref()
    }

    /// Return canonical references describing the derivation, in order.
    #[must_use]
    pub fn derivation(&self) -> &[CanonicalEntityRef] {
        &self.derivation
    }

    /// Validate every reference in the provenance chain.
    pub fn validate(&self) -> Result<(), HybridError> {
        if let Some(source) = &self.source {
            source.validate()?;
        }

        for reference in &self.derivation {
            reference.validate()?;
        }

        Ok(())
    }
}

impl TryFrom<ValueProvenanceWire> for ValueProvenance {
    type Error = HybridError;

    fn try_from(wire: ValueProvenanceWire) -> Result<Self, Self::Error> {
        Self::new(wire.source, wire.derivation)
    }
}

/// A hybrid-layer reference to a value produced by a canonical domain.
///
/// The hybrid ID is the identity of the cross-domain reference, not the
/// identity of the underlying canonical value. Multiple hybrid references
/// may refer to one canonical value when their graph and contract semantics
/// permit it.
///
/// This structure deliberately contains no payload field. Payload storage,
/// copying, borrowing, streaming, quantum-state handling, tensor layout, and
/// distributed transfer belong to canonical domain facilities and the
/// dedicated data exchange/ownership/transfer contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(try_from = "CrossDomainValueWire")]
pub struct CrossDomainValue {
    id: CrossDomainValueId,
    producer: DomainInstanceId,
    semantic_type: CanonicalTypeRef,
    canonical_value: CanonicalEntityRef,
    ownership_contract: Option<CanonicalEntityRef>,
    effect_contracts: Vec<CanonicalEntityRef>,
    provenance: ValueProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CrossDomainValueWire {
    id: CrossDomainValueId,
    producer: DomainInstanceId,
    semantic_type: CanonicalTypeRef,
    canonical_value: CanonicalEntityRef,
    ownership_contract: Option<CanonicalEntityRef>,
    effect_contracts: Vec<CanonicalEntityRef>,
    provenance: ValueProvenance,
}

impl CrossDomainValue {
    /// Construct and structurally validate a cross-domain value reference.
    ///
    /// `canonical_value` must identify a value owned by the producing
    /// domain's canonical representation. `semantic_type`, ownership, and
    /// effect references must come from their authoritative subsystems.
    ///
    /// This constructor does not resolve references, perform type checking,
    /// prove ownership, or execute a data transfer.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: CrossDomainValueId,
        producer: DomainInstanceId,
        semantic_type: CanonicalTypeRef,
        canonical_value: CanonicalEntityRef,
        ownership_contract: Option<CanonicalEntityRef>,
        effect_contracts: Vec<CanonicalEntityRef>,
        provenance: ValueProvenance,
    ) -> Result<Self, HybridError> {
        let value = Self {
            id,
            producer,
            semantic_type,
            canonical_value,
            ownership_contract,
            effect_contracts,
            provenance,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the hybrid-layer value identity.
    #[must_use]
    pub const fn id(&self) -> CrossDomainValueId {
        self.id
    }

    /// Return the producing domain-instance identity.
    #[must_use]
    pub const fn producer(&self) -> DomainInstanceId {
        self.producer
    }

    /// Return the canonical semantic type reference.
    #[must_use]
    pub fn semantic_type(&self) -> &CanonicalTypeRef {
        &self.semantic_type
    }

    /// Return the canonical value reference without materializing its value.
    #[must_use]
    pub fn canonical_value(&self) -> &CanonicalEntityRef {
        &self.canonical_value
    }

    /// Return the ownership contract reference, if one was supplied.
    ///
    /// Absence means this structure does not carry an ownership-contract
    /// reference. It does not mean that ownership is unrestricted or that
    /// the value may be copied.
    #[must_use]
    pub fn ownership_contract(&self) -> Option<&CanonicalEntityRef> {
        self.ownership_contract.as_ref()
    }

    /// Return references to canonical effect declarations.
    ///
    /// The hybrid layer does not interpret these references or infer that
    /// the referenced effects are permitted.
    #[must_use]
    pub fn effect_contracts(&self) -> &[CanonicalEntityRef] {
        &self.effect_contracts
    }

    /// Return the value's provenance.
    #[must_use]
    pub fn provenance(&self) -> &ValueProvenance {
        &self.provenance
    }

    /// Return the graph identity shared by this value and its producer.
    #[must_use]
    pub const fn graph_id(&self) -> crate::hybrid::ids::HybridGraphId {
        self.id.graph()
    }

    /// Validate local structure and graph-scoped identity consistency.
    ///
    /// This method does not prove that referenced graph nodes, domain
    /// instances, canonical values, types, effects, or ownership contracts
    /// exist. Graph verification and canonical subsystem adapters must
    /// perform those authoritative checks.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.id.graph() != self.producer.graph() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                "cross-domain value and producing domain instance belong to different hybrid graphs",
            )
            .in_component("hybrid data values"));
        }

        self.semantic_type.validate()?;
        self.canonical_value.validate()?;

        if let Some(contract) = &self.ownership_contract {
            contract.validate()?;
        }

        for effect in &self.effect_contracts {
            effect.validate()?;
        }

        self.provenance.validate()
    }
}

impl TryFrom<CrossDomainValueWire> for CrossDomainValue {
    type Error = HybridError;

    fn try_from(wire: CrossDomainValueWire) -> Result<Self, Self::Error> {
        Self::new(
            wire.id,
            wire.producer,
            wire.semantic_type,
            wire.canonical_value,
            wire.ownership_contract,
            wire.effect_contracts,
            wire.provenance,
        )
    }
}

/// Validate an opaque reference component.
///
/// The function intentionally does not impose a fixed length or a closed
/// namespace registry. Canonical owners may use different identifier
/// formats. Whitespace-only strings and control characters are rejected to
/// prevent malformed references and ambiguous diagnostics.
fn validate_reference_component(
    label: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.trim().is_empty() {
        return Err(HybridError::new(
            HybridErrorCode::InvalidReference,
            format!("{label} must not be empty"),
        )
        .in_component("hybrid data values"));
    }

    if value.chars().any(char::is_control) {
        return Err(HybridError::new(
            HybridErrorCode::InvalidReference,
            format!("{label} must not contain control characters"),
        )
        .in_component("hybrid data values"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::{
        CrossDomainValueId, DomainInstanceId, GraphScopedId, HybridGraphId,
        IdComponent,
    };

    fn graph_id(value: u128) -> HybridGraphId {
        HybridGraphId::new(value).expect("non-zero graph identity")
    }

    fn scoped_id(graph: HybridGraphId, local: u128) -> GraphScopedId {
        GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("non-zero local identity"),
        )
    }

    fn value_id(graph: HybridGraphId, local: u128) -> CrossDomainValueId {
        CrossDomainValueId::new(scoped_id(graph, local))
    }

    fn domain_id(graph: HybridGraphId, local: u128) -> DomainInstanceId {
        DomainInstanceId::new(scoped_id(graph, local))
    }

    fn sample_value() -> CrossDomainValue {
        CrossDomainValue::new(
            value_id(graph_id(1), 2),
            domain_id(graph_id(1), 3),
            CanonicalTypeRef::new("semantic", "core.integer")
                .expect("valid type reference"),
            CanonicalEntityRef::new("classical.ir", "value", "value:17")
                .expect("valid canonical value reference"),
            Some(
                CanonicalEntityRef::new(
                    "semantic",
                    "ownership_contract",
                    "ownership:4",
                )
                .expect("valid ownership reference"),
            ),
            vec![
                CanonicalEntityRef::new(
                    "semantic",
                    "effect",
                    "effect:io",
                )
                .expect("valid effect reference"),
            ],
            ValueProvenance::new(
                Some(
                    CanonicalEntityRef::new(
                        "frontend.ast",
                        "expression",
                        "node:42",
                    )
                    .expect("valid source reference"),
                ),
                Vec::new(),
            )
            .expect("valid provenance"),
        )
        .expect("valid cross-domain value")
    }

    #[test]
    fn valid_cross_domain_value_preserves_canonical_references() {
        let value = sample_value();

        assert_eq!(value.graph_id(), graph_id(1));
        assert_eq!(value.producer(), domain_id(graph_id(1), 3));
        assert_eq!(value.semantic_type().namespace(), "semantic");
        assert_eq!(value.semantic_type().identifier(), "core.integer");
        assert_eq!(value.canonical_value().namespace(), "classical.ir");
        assert_eq!(value.canonical_value().identifier(), "value:17");
        assert_eq!(value.effect_contracts().len(), 1);
        assert!(value.ownership_contract().is_some());
        assert!(value.provenance().source().is_some());
        assert!(value.validate().is_ok());
    }

    #[test]
    fn rejects_value_and_producer_from_different_graphs() {
        let result = CrossDomainValue::new(
            value_id(graph_id(1), 2),
            domain_id(graph_id(9), 3),
            CanonicalTypeRef::new("semantic", "core.integer")
                .expect("valid type"),
            CanonicalEntityRef::new("classical.ir", "value", "value:17")
                .expect("valid value reference"),
            None,
            Vec::new(),
            ValueProvenance::new(None, Vec::new())
                .expect("valid provenance"),
        );

        let error = result.expect_err("graph mismatch must be rejected");
        assert_eq!(error.code(), HybridErrorCode::InvalidReference);
    }

    #[test]
    fn rejects_empty_canonical_reference_components() {
        let error = CanonicalEntityRef::new("", "value", "17")
            .expect_err("empty namespace must be rejected");

        assert_eq!(error.code(), HybridErrorCode::InvalidReference);
    }

    #[test]
    fn rejects_control_characters_in_references() {
        let error = CanonicalTypeRef::new("semantic", "integer\nforged")
            .expect_err("control characters must be rejected");

        assert_eq!(error.code(), HybridErrorCode::InvalidReference);
    }

    #[test]
    fn no_ownership_reference_does_not_imply_copyability() {
        let value = CrossDomainValue::new(
            value_id(graph_id(1), 2),
            domain_id(graph_id(1), 3),
            CanonicalTypeRef::new("semantic", "quantum.state")
                .expect("valid type"),
            CanonicalEntityRef::new("quantum.ir", "value", "state:17")
                .expect("valid canonical reference"),
            None,
            Vec::new(),
            ValueProvenance::new(None, Vec::new())
                .expect("valid provenance"),
        )
        .expect("valid structural reference");

        assert!(value.ownership_contract().is_none());
        // This module intentionally offers no copy/materialize/transfer API.
    }

    #[test]
    fn arbitrary_length_identifiers_are_not_rejected_by_a_fixed_ceiling() {
        let long_identifier = "x".repeat(16_384);
        let reference = CanonicalEntityRef::new(
            "custom.domain",
            "value",
            long_identifier.clone(),
        )
        .expect("no module-defined identifier length ceiling");

        assert_eq!(reference.identifier(), long_identifier);
    }

    #[test]
    fn provenance_retains_derivation_order() {
        let first = CanonicalEntityRef::new("classical.ir", "operation", "op:1")
            .expect("valid reference");
        let second = CanonicalEntityRef::new("quantum.ir", "operation", "op:2")
            .expect("valid reference");

        let provenance = ValueProvenance::new(
            None,
            vec![first.clone(), second.clone()],
        )
        .expect("valid provenance");

        assert_eq!(provenance.derivation(), &[first, second]);
    }
}
