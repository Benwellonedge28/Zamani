
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Operation Instances
//!
//! Path: `src/hdl/hdlir/ir/operations.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed core IR implementation; requires integration testing.
//!
//! ## Purpose
//!
//! Represent instances of registered hardware-domain operations in HDLIR.
//! An operation references a qualified schema name and schema version,
//! consumes zero or more HDLIR values, defines zero or more typed results,
//! carries typed attribute payloads, and may own references to nested regions.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and the applicable HDL delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! Grammar owns syntax. The frontend owns source-level meaning. ZUIR owns
//! the universal computational representation. HDLIR owns the hardware-domain
//! operation instances and their local structural invariants.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The instance of a registered operation in the HDLIR graph.
//! - Its typed HDLIR identity.
//! - Ordered operand references.
//! - Ordered typed result declarations.
//! - Attribute payload storage.
//! - Ordered nested-region references.
//! - Local structural validation and schema-definition compatibility checks.
//!
//! This module does not own:
//!
//! - Source parsing, name resolution, or source-level semantic validation.
//! - The schema registry or operation schema definitions.
//! - A universal, closed enumeration of hardware operations.
//! - The actual value, type, region, block, or connectivity graph.
//! - Hardware simulation, operation evaluation, synthesis, or target lowering.
//! - Physical resources, vendor primitives, placement, routing, or scheduling.
//!
//! ## Reference and verification contract
//!
//! IDs are references, not collection indices. This module cannot establish
//! that referenced values, types, or regions exist because their owning graph
//! and registries are managed elsewhere.
//!
//! `validate_structure` checks only invariants local to this operation.
//! `validate_against_definition` checks its qualified name, schema version,
//! and operand/result arity against a registered operation definition.
//! Neither function replaces whole-graph verification, type verification,
//! attribute-contract verification, effect checking, or dialect-specific
//! semantic verification.
//!
//! Attribute payloads associated with extension-defined semantics remain
//! opaque until a compatible registered schema interprets them. Unknown
//! payloads must not be evaluated or optimized as if their meaning were known.
//!
//! ## Scalability
//!
//! This module imposes no universal maximum for operations, operands, results,
//! attributes, regions, signals, widths, or hardware resources.
//!
//! Collections grow subject to representable memory, available resources,
//! supported semantics, and caller-enforced operational budgets. This module
//! does not claim to process literally infinite input.
//!
//! Collection lengths are converted to `u64` with checked conversions before
//! they are compared with schema arity contracts. Overflow is reported.
//!
//! ## Determinism
//!
//! Operand, result, and region order is preserved. Attribute names are stored
//! in a `BTreeMap`, giving deterministic key ordering. No pointer identity,
//! host address, or iteration order from an unordered map is used.
//!
//! ## Integration
//!
//! - `ir/mod.rs` must declare this module.
//! - `ir/values.rs` owns the graph definitions for value IDs and their uses.
//! - `ir/regions.rs` owns region contents and region ownership.
//! - `types/` owns HDLIR type semantics.
//! - `schema/registry.rs` owns registered operation definitions.
//! - The verification subsystem must check reference existence, result
//!   definitions, attribute contracts, region ownership, and dialect rules.
//! - Builders must use this type without bypassing mandatory verification.
//! - Lowering and serialization must preserve schema identity, order, values,
//!   result types, attributes, regions, and provenance.
//!
//! This module does not assume that those downstream components are already
//! implemented or integrated.
//!
//! ## Freeze criteria
//!
//! - Local invariants and schema compatibility are tested.
//! - Whole-graph verification checks all external references and ownership.
//! - Attribute encoding has a versioned, deterministic contract.
//! - Schema extension payloads are never interpreted without their schema.
//! - Repository formatting, compilation, tests, and Clippy pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirOperationId, HdlirRegionId, HdlirTypeId, HdlirValueId,
};
use crate::hdl::hdlir::schema::registry::{
    OperationDefinition, QualifiedName,
};
use crate::hdl::hdlir::version::HdlirVersion;

/// A value carried by an operation attribute.
///
/// The enum provides a schema-neutral storage model. The operation's
/// registered attribute contract determines which variants are permitted
/// and what they mean. It does not itself define evaluation semantics.
///
/// Integer values use explicit signed or unsigned representations. Floating
/// point payloads are deliberately not included here because NaN handling,
/// bit-exact encoding, rounding, and cross-target equivalence require an
/// explicit type and serialization contract.
///
/// `ConstantPayload` and `SymbolicExpressionPayload` retain a qualified
/// schema identifier and opaque bytes. Consumers must not infer semantics
/// from the bytes without a compatible schema-specific decoder.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationAttributeValue {
    /// A Boolean attribute.
    Boolean(bool),

    /// A signed integer attribute.
    SignedInteger(i128),

    /// An unsigned integer attribute.
    UnsignedInteger(u128),

    /// A string attribute.
    String(String),

    /// A qualified schema name.
    QualifiedName(QualifiedName),

    /// A reference to an HDLIR type.
    ///
    /// The verifier must establish that the referenced type exists and is
    /// permitted by the operation's attribute contract.
    Type(HdlirTypeId),

    /// A reference to an HDLIR value.
    ///
    /// The verifier must establish that the referenced value exists and is
    /// permitted by the operation's attribute contract.
    Value(HdlirValueId),

    /// An ordered array of attribute values.
    Array(Vec<OperationAttributeValue>),

    /// A dictionary of attribute values with deterministic key ordering.
    Dictionary(BTreeMap<String, OperationAttributeValue>),

    /// A schema-defined constant payload.
    ///
    /// The payload is opaque here; the named schema owns its interpretation.
    ConstantPayload {
        /// Schema defining the payload's representation and meaning.
        schema: QualifiedName,
        /// Schema-specific encoded bytes.
        payload: Vec<u8>,
    },

    /// A schema-defined symbolic expression payload.
    ///
    /// The payload must not be evaluated without a compatible decoder and
    /// evaluator for the declared schema.
    SymbolicExpressionPayload {
        /// Schema defining the expression representation and meaning.
        schema: QualifiedName,
        /// Schema-specific encoded bytes.
        payload: Vec<u8>,
    },

    /// An extension-defined attribute payload.
    ///
    /// Unknown extensions may be retained when the enclosing artifact
    /// compatibility policy permits it, but they must not be interpreted
    /// as understood semantics.
    ExtensionPayload {
        /// Schema defining the extension payload.
        schema: QualifiedName,
        /// Opaque extension data.
        payload: Vec<u8>,
    },
}

/// A typed result declared by an operation.
///
/// Result IDs are HDLIR-owned identities. A result's type is referenced
/// rather than copied into this structure, so the type registry remains
/// the single authority for type meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OperationResult {
    id: HdlirValueId,
    type_id: HdlirTypeId,
}

impl OperationResult {
    /// Construct a result declaration.
    ///
    /// This constructor establishes that both IDs are typed, non-zero HDLIR
    /// identities. It cannot establish that the type is registered or that
    /// the result ID is unique in the enclosing graph.
    #[must_use]
    pub const fn new(id: HdlirValueId, type_id: HdlirTypeId) -> Self {
        Self { id, type_id }
    }

    /// Return the result value identity.
    #[must_use]
    pub const fn id(self) -> HdlirValueId {
        self.id
    }

    /// Return the referenced type identity.
    #[must_use]
    pub const fn type_id(self) -> HdlirTypeId {
        self.type_id
    }
}

/// One operation instance in the hardware-domain IR.
///
/// This type stores operation structure, not the implementation of the
/// operation. Its meaning comes from the registered schema and the applicable
/// dialect contract.
///
/// An operation may represent combinational logic, a state transition,
/// a memory access, a logical connection, a process-related construct, or
/// another registered hardware-domain concept. No fixed operation enum is
/// required to extend HDLIR with a new qualified operation name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirOperation {
    id: HdlirOperationId,
    name: QualifiedName,
    schema_version: HdlirVersion,
    operands: Vec<HdlirValueId>,
    results: Vec<OperationResult>,
    attributes: BTreeMap<String, OperationAttributeValue>,
    regions: Vec<HdlirRegionId>,
}

impl HdlirOperation {
    /// Construct an operation instance.
    ///
    /// The constructor checks local invariants and preserves caller-supplied
    /// operand, result, and region order. It does not look up the operation
    /// definition or verify references against an enclosing graph.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute map contains an empty or
    /// whitespace-only name, if result IDs are duplicated, or if region IDs
    /// are duplicated.
    pub fn new(
        id: HdlirOperationId,
        name: QualifiedName,
        schema_version: HdlirVersion,
        operands: Vec<HdlirValueId>,
        results: Vec<OperationResult>,
        attributes: BTreeMap<String, OperationAttributeValue>,
        regions: Vec<HdlirRegionId>,
    ) -> HdlirResult<Self> {
        for name in attributes.keys() {
            if name.trim().is_empty() {
                return Err(HdlirError::invalid_input(
                    "operation.attributes",
                    "attribute names must not be empty or whitespace-only",
                ));
            }
        }

        let mut result_ids = BTreeSet::new();
        for result in &results {
            if !result_ids.insert(result.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR operation result",
                    identity: result.id().to_string(),
                    scope: Some(id.to_string()),
                });
            }
        }

        let mut region_ids = BTreeSet::new();
        for region_id in &regions {
            if !region_ids.insert(*region_id) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR operation region reference",
                    identity: region_id.to_string(),
                    scope: Some(id.to_string()),
                });
            }
        }

        Ok(Self {
            id,
            name,
            schema_version,
            operands,
            results,
            attributes,
            regions,
        })
    }

    /// Return this operation's identity.
    #[must_use]
    pub const fn id(&self) -> HdlirOperationId {
        self.id
    }

    /// Return the registered qualified operation name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the schema version declared by this instance.
    #[must_use]
    pub const fn schema_version(&self) -> HdlirVersion {
        self.schema_version
    }

    /// Return ordered operand references.
    ///
    /// Repeated operands are permitted: an operation may consume the same
    /// value more than once. Operand validity and use-def consistency belong
    /// to whole-graph verification.
    #[must_use]
    pub fn operands(&self) -> &[HdlirValueId] {
        &self.operands
    }

    /// Return ordered typed result declarations.
    #[must_use]
    pub fn results(&self) -> &[OperationResult] {
        &self.results
    }

    /// Return operation attributes in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, OperationAttributeValue> {
        &self.attributes
    }

    /// Return a named attribute, if present.
    #[must_use]
    pub fn attribute(&self, name: &str) -> Option<&OperationAttributeValue> {
        self.attributes.get(name)
    }

    /// Return ordered nested-region references.
    #[must_use]
    pub fn regions(&self) -> &[HdlirRegionId] {
        &self.regions
    }

    /// Return the operand count as a host collection length.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Return the result count as a host collection length.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Return the attribute count as a host collection length.
    #[must_use]
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    /// Return the nested-region reference count as a host collection length.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Check this operation's local invariants.
    ///
    /// This method does not validate references against an enclosing graph.
    /// In particular, it does not prove that operands, result types, or
    /// regions exist, or that the operation's attributes satisfy a schema.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR error if a local invariant is violated.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        for name in self.attributes.keys() {
            if name.trim().is_empty() {
                return Err(HdlirError::invalid_invariant(
                    "operation.attribute_name",
                    "an attribute name is empty or whitespace-only",
                ));
            }
        }

        let mut result_ids = BTreeSet::new();
        for result in &self.results {
            if !result_ids.insert(result.id()) {
                return Err(HdlirError::invalid_invariant(
                    "operation.unique_results",
                    format!(
                        "result identity {} occurs more than once in operation {}",
                        result.id(),
                        self.id
                    ),
                ));
            }
        }

        let mut region_ids = BTreeSet::new();
        for region_id in &self.regions {
            if !region_ids.insert(*region_id) {
                return Err(HdlirError::invalid_invariant(
                    "operation.unique_regions",
                    format!(
                        "region identity {region_id} occurs more than once in operation {}",
                        self.id
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validate identity, schema version, and arity against a registered
    /// operation definition.
    ///
    /// This check does not replace detailed operation-contract verification.
    /// The current registry's `OperationDefinition` describes operation
    /// identity and operand/result arity; attribute kinds, type constraints,
    /// region shapes, effects, and verifier requirements must be checked by
    /// the corresponding schema-contract and verification components.
    ///
    /// # Errors
    ///
    /// Returns an error when the registered name or version differs from the
    /// instance, when operand/result arity is invalid, or when a collection
    /// length cannot be represented by the schema's `u64` arity contract.
    pub fn validate_against_definition(
        &self,
        definition: &OperationDefinition,
    ) -> HdlirResult<()> {
        self.validate_structure()?;

        if self.name != *definition.name() {
            return Err(HdlirError::OperationContractViolation {
                schema: definition.name().to_string(),
                operation: self.name.to_string(),
                detail: format!(
                    "operation name does not match registered definition `{}`",
                    definition.name()
                ),
            });
        }

        if self.schema_version != definition.version() {
            return Err(HdlirError::SchemaVersionMismatch {
                schema: definition.name().to_string(),
                expected: definition.version().to_string(),
                found: self.schema_version.to_string(),
            });
        }

        let operand_count = u64::try_from(self.operands.len()).map_err(|_| {
            HdlirError::ConversionOutOfRange {
                value: self.operands.len().to_string(),
                destination: "u64 operand arity",
            }
        })?;

        let result_count = u64::try_from(self.results.len()).map_err(|_| {
            HdlirError::ConversionOutOfRange {
                value: self.results.len().to_string(),
                destination: "u64 result arity",
            }
        })?;

        let signature = definition.signature();

        if !signature.operands().accepts(operand_count) {
            return Err(HdlirError::OperationContractViolation {
                schema: definition.name().to_string(),
                operation: self.id.to_string(),
                detail: format!(
                    "operand count {operand_count} violates declared arity {}..{:?}",
                    signature.operands().min(),
                    signature.operands().max()
                ),
            });
        }

        if !signature.results().accepts(result_count) {
            return Err(HdlirError::OperationContractViolation {
                schema: definition.name().to_string(),
                operation: self.id.to_string(),
                detail: format!(
                    "result count {result_count} violates declared arity {}..{:?}",
                    signature.results().min(),
                    signature.results().max()
                ),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::schema::registry::{
        Arity, DefinitionStability, OperationDefinition, OperationSignature,
    };

    fn qualified_name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("valid qualified name")
    }

    fn operation_definition(
        name: &str,
        operands: Arity,
        results: Arity,
    ) -> OperationDefinition {
        let name = qualified_name(name);
        let dialect =
            QualifiedName::parse(name.namespace()).expect("valid dialect name");

        OperationDefinition::new(
            name,
            dialect,
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Core,
            OperationSignature::new(operands, results),
            BTreeSet::new(),
            "Test operation definition",
        )
        .expect("valid operation definition")
    }

    fn operation_id(raw: u128) -> HdlirOperationId {
        HdlirOperationId::new(raw).expect("non-zero operation ID")
    }

    fn value_id(raw: u128) -> HdlirValueId {
        HdlirValueId::new(raw).expect("non-zero value ID")
    }

    fn type_id(raw: u128) -> HdlirTypeId {
        HdlirTypeId::new(raw).expect("non-zero type ID")
    }

    fn region_id(raw: u128) -> HdlirRegionId {
        HdlirRegionId::new(raw).expect("non-zero region ID")
    }

    fn simple_operation(
        operands: Vec<HdlirValueId>,
        results: Vec<OperationResult>,
        attributes: BTreeMap<String, OperationAttributeValue>,
        regions: Vec<HdlirRegionId>,
    ) -> HdlirOperation {
        HdlirOperation::new(
            operation_id(1),
            qualified_name("structural.example"),
            HdlirVersion::new(1, 0, 0),
            operands,
            results,
            attributes,
            regions,
        )
        .expect("valid operation")
    }

    #[test]
    fn preserves_operand_result_and_region_order() {
        let operation = simple_operation(
            vec![value_id(8), value_id(3), value_id(8)],
            vec![
                OperationResult::new(value_id(10), type_id(1)),
                OperationResult::new(value_id(11), type_id(2)),
            ],
            BTreeMap::new(),
            vec![region_id(4), region_id(2)],
        );

        assert_eq!(
            operation.operands(),
            &[value_id(8), value_id(3), value_id(8)]
        );
        assert_eq!(operation.results()[0].id(), value_id(10));
        assert_eq!(operation.results()[1].id(), value_id(11));
        assert_eq!(operation.regions(), &[region_id(4), region_id(2)]);
    }

    #[test]
    fn repeated_operand_is_permitted() {
        let operation = simple_operation(
            vec![value_id(7), value_id(7)],
            Vec::new(),
            BTreeMap::new(),
            Vec::new(),
        );

        assert_eq!(operation.operand_count(), 2);
        assert!(operation.validate_structure().is_ok());
    }

    #[test]
    fn duplicate_result_identity_is_rejected() {
        let result = OperationResult::new(value_id(9), type_id(1));

        let result = HdlirOperation::new(
            operation_id(1),
            qualified_name("structural.example"),
            HdlirVersion::new(1, 0, 0),
            Vec::new(),
            vec![result, result],
            BTreeMap::new(),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR operation result",
                ..
            })
        ));
    }

    #[test]
    fn duplicate_region_reference_is_rejected() {
        let result = HdlirOperation::new(
            operation_id(1),
            qualified_name("structural.example"),
            HdlirVersion::new(1, 0, 0),
            Vec::new(),
            Vec::new(),
            BTreeMap::new(),
            vec![region_id(2), region_id(2)],
        );

        assert!(matches!(
            result,
            Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR operation region reference",
                ..
            })
        ));
    }

    #[test]
    fn rejects_empty_attribute_names() {
        let mut attributes = BTreeMap::new();
        attributes.insert(
            "   ".to_owned(),
            OperationAttributeValue::Boolean(true),
        );

        assert!(matches!(
            HdlirOperation::new(
                operation_id(1),
                qualified_name("structural.example"),
                HdlirVersion::new(1, 0, 0),
                Vec::new(),
                Vec::new(),
                attributes,
                Vec::new(),
            ),
            Err(HdlirError::InvalidInput {
                field: "operation.attributes",
                ..
            })
        ));
    }

    #[test]
    fn attribute_lookup_is_exact() {
        let mut attributes = BTreeMap::new();
        attributes.insert(
            "signed".to_owned(),
            OperationAttributeValue::Boolean(true),
        );

        let operation =
            simple_operation(Vec::new(), Vec::new(), attributes, Vec::new());

        assert_eq!(
            operation.attribute("signed"),
            Some(&OperationAttributeValue::Boolean(true))
        );
        assert_eq!(operation.attribute("Signed"), None);
    }

    #[test]
    fn validates_registered_name_version_and_arity() {
        let definition = operation_definition(
            "structural.example",
            Arity::new(2, Some(2)).expect("valid arity"),
            Arity::new(1, Some(1)).expect("valid arity"),
        );

        let operation = simple_operation(
            vec![value_id(1), value_id(2)],
            vec![OperationResult::new(value_id(3), type_id(1))],
            BTreeMap::new(),
            Vec::new(),
        );

        assert!(operation.validate_against_definition(&definition).is_ok());
    }

    #[test]
    fn rejects_operand_arity_mismatch() {
        let definition = operation_definition(
            "structural.example",
            Arity::new(2, Some(2)).expect("valid arity"),
            Arity::new(0, Some(0)).expect("valid arity"),
        );

        let operation = simple_operation(
            vec![value_id(1)],
            Vec::new(),
            BTreeMap::new(),
            Vec::new(),
        );

        assert!(matches!(
            operation.validate_against_definition(&definition),
            Err(HdlirError::OperationContractViolation { .. })
        ));
    }

    #[test]
    fn rejects_schema_version_mismatch() {
        let definition = operation_definition(
            "structural.example",
            Arity::new(0, Some(0)).expect("valid arity"),
            Arity::new(0, Some(0)).expect("valid arity"),
        );

        let operation = HdlirOperation::new(
            operation_id(1),
            qualified_name("structural.example"),
            HdlirVersion::new(2, 0, 0),
            Vec::new(),
            Vec::new(),
            BTreeMap::new(),
            Vec::new(),
        )
        .expect("valid operation");

        assert!(matches!(
            operation.validate_against_definition(&definition),
            Err(HdlirError::SchemaVersionMismatch { .. })
        ));
    }

    #[test]
    fn attributes_have_deterministic_key_order() {
        let mut attributes = BTreeMap::new();
        attributes.insert(
            "zeta".to_owned(),
            OperationAttributeValue::Boolean(false),
        );
        attributes.insert(
            "alpha".to_owned(),
            OperationAttributeValue::Boolean(true),
        );

        let operation =
            simple_operation(Vec::new(), Vec::new(), attributes, Vec::new());

        let keys: Vec<_> = operation.attributes().keys().map(String::as_str).collect();
        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn nested_attribute_values_are_representable() {
        let mut dictionary = BTreeMap::new();
        dictionary.insert(
            "width".to_owned(),
            OperationAttributeValue::UnsignedInteger(u128::MAX),
        );

        let mut attributes = BTreeMap::new();
        attributes.insert(
            "metadata".to_owned(),
            OperationAttributeValue::Dictionary(dictionary),
        );

        let operation =
            simple_operation(Vec::new(), Vec::new(), attributes, Vec::new());

        assert!(matches!(
            operation.attribute("metadata"),
            Some(OperationAttributeValue::Dictionary(_))
        ));
    }
}
