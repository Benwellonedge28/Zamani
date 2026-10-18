
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Requirements
//!
//! Path: `src/hdl/requirements.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97+
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the HDL-domain integration boundary for portable realization
//! requirements, constraints, preferences, hints, and named properties.
//!
//! This module records what an HDL design asks of a possible realization.
//! It does not decide whether a particular target can satisfy those requests.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/synthesis.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//!
//! The grammar owns syntax; the canonical AST owns source structure; the
//! shared semantic models own resource and capability meanings.
//!
//! ## Ownership
//!
//! This module owns:
//! - the HDL requirements aggregation API;
//! - HDL-specific named property requirements;
//! - source-associated requirement records;
//! - deterministic collection and duplicate-source handling;
//! - the boundary between requirement declarations and later feasibility.
//!
//! It does not own:
//! - parsing, AST nodes, source IDs, or source spans;
//! - resource quantities, units, or resource constraints;
//! - capability identity, versions, or capability requirements;
//! - expression evaluation, name resolution, or type inference;
//! - target discovery, resource allocation, or feasibility decisions;
//! - synthesis, scheduling, placement, routing, or timing closure;
//! - any HDL intermediate representation.
//!
//! ## Semantic rules
//!
//! A requirement is not proof that a target supports it.
//! An unresolved symbolic value is not a satisfied requirement.
//! A preference or hint is not silently promoted to a mandatory requirement.
//! Resource and capability meanings are delegated to their canonical models.
//! Physical realization is a downstream responsibility.
//!
//! ## Scalability
//!
//! There are no language-level limits on requirement counts, property counts,
//! widths, capacities, hierarchy sizes, or hardware counts in this module.
//! Numeric resource quantities use the canonical resource model, which can
//! preserve exact decimal quantities and symbolic AST expressions.
//!
//! Rust collections and compilation are necessarily bounded by available
//! memory, processing time, and any explicit caller-supplied operational
//! budget. This module introduces no hidden universal capacity ceiling.
//!
//! ## Determinism and mutation
//!
//! Records are indexed by canonical AST `NodeId` in a `BTreeMap`.
//! Iteration order is deterministic. Re-inserting an identical record is
//! idempotent; reusing a source identity for different data is an error.
//! Batch insertion is transactional: failure leaves the original set intact.
//!
//! ## Integration
//!
//! Frontend semantic adapters construct canonical resource and capability
//! requirements first, then wrap them in `HdlRequirementRecord` with the
//! original AST node identity and span.
//!
//! Target feasibility consumes these records later. No target-specific
//! decision is made while constructing or storing them.
//!
//! The module must be registered by `src/hdl/mod.rs` only after the frontend
//! and semantic module paths are publicly available and build-verified.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::span::Span;
use crate::frontend::semantic::capabilities::model::CapabilityRequirement;
use crate::frontend::semantic::resources::model::{
    ResourceConstraint,
    ResourceQuantity,
    ResourceRequirement,
    ResourceUnit,
};

/// Schema version of the in-memory HDL requirements collection.
///
/// This is not the Zamani language version, AST schema version, or HDLIR
/// version.
pub const HDL_REQUIREMENTS_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name for this module.
pub const HDL_REQUIREMENTS_MODULE_NAME: &str = "zamani.hdl.requirements";

/// A comparison relation for a named HDL property.
///
/// The relation is recorded, not evaluated. Evaluation requires the property's
/// declared semantics and any necessary semantic or target evidence.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlPropertyRelation {
    /// The property must equal the requested value.
    Equal,
    /// The property must not equal the requested value.
    NotEqual,
    /// The property must be strictly less than the requested value.
    LessThan,
    /// The property must be less than or equal to the requested value.
    LessThanOrEqual,
    /// The property must be strictly greater than the requested value.
    GreaterThan,
    /// The property must be greater than or equal to the requested value.
    GreaterThanOrEqual,
}

/// Binding strength for a named HDL property.
///
/// These variants deliberately distinguish semantic requirements from
/// optimization preferences and non-binding implementation hints.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlPropertyIntent {
    /// The property condition is mandatory for a valid realization.
    Required,
    /// The property restricts acceptable realizations.
    Constraint,
    /// The property guides optimization but does not change program meaning.
    Preference,
    /// The property provides optional implementation guidance.
    Hint,
}

/// Value associated with a named HDL property.
///
/// Quantity values reuse the shared resource quantity model so exact and
/// symbolic resource values are not reimplemented here.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum HdlPropertyValue {
    /// A quantity, possibly symbolic or runtime-derived.
    Quantity(ResourceQuantity),
    /// A Boolean property value.
    Boolean(bool),
    /// A textual property value.
    Text(String),
}

/// A target-independent property requirement.
///
/// `key` is an open, namespaced identity such as `timing.period` or
/// `interconnect.bandwidth`. The key is not restricted to a built-in list.
/// A property registry or domain specification defines its meaning, type,
/// units, and comparison rules.
///
/// This structure does not claim that a target supports the property.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HdlPropertyRequirement {
    key: String,
    relation: HdlPropertyRelation,
    value: HdlPropertyValue,
    unit: Option<ResourceUnit>,
    intent: HdlPropertyIntent,
}

impl HdlPropertyRequirement {
    /// Construct a named property requirement after validating its identity
    /// and textual value representation.
    ///
    /// This constructor validates only representation-level invariants.
    /// Property registration, dimensional compatibility, expression typing,
    /// and target feasibility belong to their respective services.
    pub fn new(
        key: impl Into<String>,
        relation: HdlPropertyRelation,
        value: HdlPropertyValue,
        unit: Option<ResourceUnit>,
        intent: HdlPropertyIntent,
    ) -> Result<Self, HdlRequirementError> {
        let key = key.into();
        validate_property_key(&key)?;

        if let HdlPropertyValue::Text(value) = &value {
            if value.chars().any(char::is_control) {
                return Err(HdlRequirementError::ControlCharacter {
                    field: "property value",
                });
            }
        }

        Ok(Self {
            key,
            relation,
            value,
            unit,
            intent,
        })
    }

    /// Return the exact validated property identity.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return the requested comparison relation.
    #[must_use]
    pub const fn relation(&self) -> HdlPropertyRelation {
        self.relation
    }

    /// Return the requested value without resolving symbolic expressions.
    #[must_use]
    pub fn value(&self) -> &HdlPropertyValue {
        &self.value
    }

    /// Return the optional semantic unit.
    #[must_use]
    pub fn unit(&self) -> Option<&ResourceUnit> {
        self.unit.as_ref()
    }

    /// Return the property's binding strength.
    #[must_use]
    pub const fn intent(&self) -> HdlPropertyIntent {
        self.intent
    }
}

/// A single HDL requirement declaration backed by a canonical semantic model.
///
/// The payload determines which shared semantic concept owns the declaration.
/// Source identity and span are retained independently of the payload.
///
/// A record's source identity and span must originate from the same canonical
/// AST node. This association is the caller's responsibility because checking
/// it requires access to the complete AST graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlRequirementRecord {
    source_node: NodeId,
    source_span: Span,
    statement: HdlRequirementStatement,
}

/// Canonical semantic statement stored in an HDL requirement record.
///
/// Resource and capability variants reuse existing semantic models rather
/// than defining competing representations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlRequirementStatement {
    /// A resource requirement with its canonical quantity, unit, and intent.
    Resource(ResourceRequirement),
    /// A resource constraint with its canonical relation and intent.
    ResourceConstraint(ResourceConstraint),
    /// A required capability with its canonical identity and properties.
    Capability(CapabilityRequirement),
    /// An HDL-specific named property condition.
    Property(HdlPropertyRequirement),
}

impl HdlRequirementRecord {
    /// Construct a source-associated record.
    ///
    /// The caller must supply a canonical AST node ID and that node's span.
    /// This function does not independently validate the association against
    /// the AST graph.
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        source_span: Span,
        statement: HdlRequirementStatement,
    ) -> Self {
        Self {
            source_node,
            source_span,
            statement,
        }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the original source span.
    #[must_use]
    pub const fn source_span(&self) -> &Span {
        &self.source_span
    }

    /// Return the canonical semantic statement.
    #[must_use]
    pub const fn statement(&self) -> &HdlRequirementStatement {
        &self.statement
    }

    /// Consume the record and return its parts.
    #[must_use]
    pub fn into_parts(self) -> (NodeId, Span, HdlRequirementStatement) {
        (self.source_node, self.source_span, self.statement)
    }
}

/// Deterministic collection of HDL requirement records.
///
/// The canonical AST node ID is the key because distinct source declarations
/// remain distinct obligations even if their semantic payloads happen to be
/// equal. The collection does not merge, solve, or discard requirements.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlRequirements {
    records: BTreeMap<NodeId, HdlRequirementRecord>,
}

impl HdlRequirements {
    /// Create an empty requirements collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of distinct source records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Return whether the collection contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Look up a record by its canonical source node identity.
    #[must_use]
    pub fn get(&self, source_node: NodeId) -> Option<&HdlRequirementRecord> {
        self.records.get(&source_node)
    }

    /// Iterate over records in ascending source-node identity order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &HdlRequirementRecord> {
        self.records.values()
    }

    /// Insert one record.
    ///
    /// Re-inserting an identical record is idempotent. A different record
    /// using an existing source node is rejected without changing the set.
    pub fn insert(
        &mut self,
        record: HdlRequirementRecord,
    ) -> Result<(), HdlRequirementError> {
        match self.records.get(&record.source_node) {
            Some(existing) if existing == &record => Ok(()),
            Some(_) => Err(HdlRequirementError::ConflictingSourceNode {
                source_node: record.source_node,
            }),
            None => {
                self.records.insert(record.source_node, record);
                Ok(())
            }
        }
    }

    /// Insert a batch of records atomically.
    ///
    /// If any source node conflicts with an existing record or another record
    /// in the batch, the original collection remains unchanged.
    ///
    /// Duplicate identical records are permitted and are idempotent.
    pub fn extend(
        &mut self,
        records: impl IntoIterator<Item = HdlRequirementRecord>,
    ) -> Result<(), HdlRequirementError> {
        let mut staged = self.records.clone();

        for record in records {
            match staged.get(&record.source_node) {
                Some(existing) if existing == &record => {}
                Some(_) => {
                    return Err(HdlRequirementError::ConflictingSourceNode {
                        source_node: record.source_node,
                    });
                }
                None => {
                    staged.insert(record.source_node, record);
                }
            }
        }

        self.records = staged;
        Ok(())
    }

    /// Consume the collection into deterministic source-node order.
    #[must_use]
    pub fn into_records(self) -> Vec<HdlRequirementRecord> {
        self.records.into_values().collect()
    }

    /// Return the stable architectural module name.
    #[must_use]
    pub const fn module_name() -> &'static str {
        HDL_REQUIREMENTS_MODULE_NAME
    }

    /// Return the in-memory contract schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        HDL_REQUIREMENTS_SCHEMA_VERSION
    }
}

/// Errors produced by the HDL requirements boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlRequirementError {
    /// A named property has an empty or whitespace-only identity.
    EmptyPropertyKey,
    /// A named property contains a control character.
    ControlCharacter {
        /// The field containing the prohibited character.
        field: &'static str,
    },
    /// A source node is already associated with different requirement data.
    ConflictingSourceNode {
        /// The source node whose existing record conflicts.
        source_node: NodeId,
    },
}

impl fmt::Display for HdlRequirementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPropertyKey => {
                formatter.write_str("HDL property key must not be empty")
            }
            Self::ControlCharacter { field } => {
                write!(formatter, "HDL {field} must not contain control characters")
            }
            Self::ConflictingSourceNode { source_node } => {
                write!(
                    formatter,
                    "HDL source node {source_node} already has a different requirement record"
                )
            }
        }
    }
}

impl Error for HdlRequirementError {}

fn validate_property_key(key: &str) -> Result<(), HdlRequirementError> {
    if key.trim().is_empty() {
        return Err(HdlRequirementError::EmptyPropertyKey);
    }

    if key.chars().any(char::is_control) {
        return Err(HdlRequirementError::ControlCharacter {
            field: "property key",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::source_id::SourceId;
    use crate::frontend::semantic::capabilities::model::CapabilityId;

    fn source_node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node ID must be valid")
    }

    fn source_span() -> Span {
        Span::new(SourceId::new(1), 0, 0)
    }

    fn property_record(
        node: u64,
        key: &str,
    ) -> HdlRequirementRecord {
        let property = HdlPropertyRequirement::new(
            key,
            HdlPropertyRelation::GreaterThanOrEqual,
            HdlPropertyValue::Boolean(true),
            None,
            HdlPropertyIntent::Required,
        )
        .expect("valid test property");

        HdlRequirementRecord::new(
            source_node(node),
            source_span(),
            HdlRequirementStatement::Property(property),
        )
    }

    #[test]
    fn module_contract_is_stable() {
        assert_eq!(
            HdlRequirements::module_name(),
            "zamani.hdl.requirements"
        );
        assert_eq!(HdlRequirements::schema_version(), 1);
    }

    #[test]
    fn accepts_open_ended_property_names() {
        let property = HdlPropertyRequirement::new(
            "future.interconnect.collective_bandwidth",
            HdlPropertyRelation::GreaterThanOrEqual,
            HdlPropertyValue::Text("required".to_owned()),
            None,
            HdlPropertyIntent::Required,
        );

        assert!(property.is_ok());
    }

    #[test]
    fn rejects_empty_property_names() {
        assert_eq!(
            HdlPropertyRequirement::new(
                "  ",
                HdlPropertyRelation::Equal,
                HdlPropertyValue::Boolean(true),
                None,
                HdlPropertyIntent::Required,
            )
            .unwrap_err(),
            HdlRequirementError::EmptyPropertyKey
        );
    }

    #[test]
    fn rejects_control_characters_in_property_names() {
        assert!(matches!(
            HdlPropertyRequirement::new(
                "timing\nperiod",
                HdlPropertyRelation::Equal,
                HdlPropertyValue::Boolean(true),
                None,
                HdlPropertyIntent::Required,
            ),
            Err(HdlRequirementError::ControlCharacter {
                field: "property key"
            })
        ));
    }

    #[test]
    fn preserves_symbolic_resource_quantities() {
        let expression = source_node(22);
        let quantity = ResourceQuantity::Symbolic(expression);

        assert_eq!(quantity.expression_id(), Some(expression));
        assert!(!quantity.is_known());
    }

    #[test]
    fn capability_requirements_reuse_the_canonical_model() {
        let capability = CapabilityId::new("zamani.hdl", "parallel.compute")
            .expect("valid capability identity");
        let requirement = CapabilityRequirement::new(capability);

        let record = HdlRequirementRecord::new(
            source_node(3),
            source_span(),
            HdlRequirementStatement::Capability(requirement),
        );

        assert!(matches!(
            record.statement(),
            HdlRequirementStatement::Capability(_)
        ));
    }

    #[test]
    fn identical_insert_is_idempotent() {
        let record = property_record(1, "timing.period");
        let mut requirements = HdlRequirements::new();

        requirements.insert(record.clone()).expect("first insert");
        requirements.insert(record).expect("identical insert");

        assert_eq!(requirements.len(), 1);
    }

    #[test]
    fn conflicting_source_identity_is_rejected() {
        let mut requirements = HdlRequirements::new();
        requirements
            .insert(property_record(1, "timing.period"))
            .expect("first insert");

        let result = requirements.insert(property_record(1, "timing.latency"));

        assert!(matches!(
            result,
            Err(HdlRequirementError::ConflictingSourceNode { .. })
        ));
        assert_eq!(requirements.len(), 1);
    }

    #[test]
    fn batch_insertion_is_transactional() {
        let mut requirements = HdlRequirements::new();
        requirements
            .insert(property_record(1, "timing.period"))
            .expect("initial insert");

        let result = requirements.extend([
            property_record(2, "interconnect.bandwidth"),
            property_record(1, "timing.latency"),
        ]);

        assert!(result.is_err());
        assert_eq!(requirements.len(), 1);
        assert!(requirements.get(source_node(2)).is_none());
    }

    #[test]
    fn iteration_is_deterministic_by_source_identity() {
        let mut requirements = HdlRequirements::new();

        requirements
            .insert(property_record(9, "timing.period"))
            .expect("insert");
        requirements
            .insert(property_record(2, "timing.latency"))
            .expect("insert");
        requirements
            .insert(property_record(5, "interconnect.bandwidth"))
            .expect("insert");

        let ids: Vec<_> = requirements
            .iter()
            .map(HdlRequirementRecord::source_node)
            .collect();

        assert_eq!(ids, vec![source_node(2), source_node(5), source_node(9)]);
    }
}
