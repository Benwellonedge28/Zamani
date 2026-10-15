#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Operation
//!
//! Canonical path: `src/zuir/ir/operation.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Represent one target-independent computational operation in ZUIR.
//!
//! An operation records:
//! - Its stable, typed identity.
//! - Its dialect-qualified operation name.
//! - Ordered operands and results.
//! - Namespaced attributes.
//! - Nested-region references.
//! - Declared semantic effects.
//! - An optional opaque payload for lossless interchange.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//!
//! ## OWNERSHIP BOUNDARIES
//!
//! This module owns operation-local representation and validation.
//!
//! It does not own:
//! - Source parsing or AST construction.
//! - Name resolution or semantic type inference.
//! - Block ordering or block terminators.
//! - Region ownership or region contents.
//! - Dialect registration or operation execution.
//! - Whole-program SSA, dominance, or reference verification.
//! - Physical hardware allocation, placement, or scheduling.
//!
//! ## SCALABILITY
//!
//! No universal operation-count, operand-count, result-count, region-count,
//! resource-count, tensor-rank, or hardware-size limit is imposed here.
//! Collection growth remains subject to available resources and allocator
//! behavior. Callers that require fallible reservation should reserve storage
//! before constructing very large collections.
//!
//! ## UNKNOWN OPERATIONS
//!
//! A qualified operation name is not proof that its dialect is registered.
//! Unknown operations must not be executed, optimized, or lowered as if their
//! semantics were known. An optional opaque payload preserves uninterpreted
//! extension data; it does not grant executable meaning.
//!
//! ## INTEGRATION
//!
//! - `block.rs` stores `OperationId` values and owns instruction order.
//! - `region.rs` owns regions referenced by `RegionId`.
//! - `function.rs` owns callable signatures and declared function effects.
//! - The dialect registry defines the meaning and schema of operation names.
//! - Structural, type, effect, ownership, and control-flow verifiers validate
//!   relationships that cannot be established by this module alone.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{OperationId, RegionId, TypeId, ValueId};
use crate::zuir::schema::attributes::{AttributeKey, AttributeValue};
use crate::zuir::schema::types::QualifiedName;

/// Result type for operation-local construction and validation.
pub type OperationResultType<T> = Result<T, OperationError>;

// =============================================================================
// Opaque extension payload
// =============================================================================

/// Opaque, uninterpreted extension data attached to an operation.
///
/// `encoding` identifies the payload representation, not the operation's
/// semantics. Consumers must not execute or interpret the payload unless the
/// relevant extension is registered and its contract is satisfied.
///
/// The byte sequence is preserved as supplied. Its internal format, version,
/// integrity, and compatibility are the responsibility of the extension
/// contract and serialization layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpaqueOperationPayload {
    encoding: QualifiedName,
    bytes: Vec<u8>,
}

impl OpaqueOperationPayload {
    /// Construct an opaque payload with an explicitly qualified encoding name.
    pub fn new(
        encoding: QualifiedName,
        bytes: Vec<u8>,
    ) -> OperationResultType<Self> {
        encoding
            .validate()
            .map_err(|error| OperationError::InvalidPayloadEncoding(
                error.to_string(),
            ))?;

        Ok(Self { encoding, bytes })
    }

    /// Return the qualified encoding name.
    #[must_use]
    pub const fn encoding(&self) -> &QualifiedName {
        &self.encoding
    }

    /// Borrow the uninterpreted payload bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the payload and return its bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Revalidate payload metadata after deserialization or interchange.
    pub fn validate(&self) -> OperationResultType<()> {
        self.encoding
            .validate()
            .map_err(|error| OperationError::InvalidPayloadEncoding(
                error.to_string(),
            ))
    }
}

// =============================================================================
// Operation operands
// =============================================================================

/// One ordered operand of an operation.
///
/// Repeated references to the same value are permitted: an operation may
/// legitimately consume the same value more than once.
///
/// The optional role is a qualified, extension-friendly descriptor. Its
/// presence does not prove that the role is registered or legal for the
/// operation. The dialect verifier owns that decision.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OperationOperand {
    value: ValueId,
    role: Option<QualifiedName>,
}

impl OperationOperand {
    /// Construct an operand without an explicit role.
    #[must_use]
    pub const fn new(value: ValueId) -> Self {
        Self { value, role: None }
    }

    /// Construct an operand with a qualified role.
    pub fn with_role(
        value: ValueId,
        role: QualifiedName,
    ) -> OperationResultType<Self> {
        role.validate()
            .map_err(|error| OperationError::InvalidOperandRole(
                error.to_string(),
            ))?;

        Ok(Self {
            value,
            role: Some(role),
        })
    }

    /// Return the referenced value identity.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the optional qualified operand role.
    #[must_use]
    pub const fn role(&self) -> Option<&QualifiedName> {
        self.role.as_ref()
    }

    /// Revalidate the operand after deserialization or interchange.
    pub fn validate(&self) -> OperationResultType<()> {
        if let Some(role) = &self.role {
            role.validate()
                .map_err(|error| OperationError::InvalidOperandRole(
                    error.to_string(),
                ))?;
        }

        Ok(())
    }
}

// =============================================================================
// Operation results
// =============================================================================

/// One ordered result produced by an operation.
///
/// The result identity is globally typed as a `ValueId`; its semantic type is
/// referenced by `TypeId`. The result's position in the operation's result
/// vector defines its ordinal association.
///
/// The operation-local validator ensures result identities are unique within
/// the operation. The whole-program verifier must also ensure that a value
/// has exactly one legal definition across the entire IR.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OperationValueResult {
    value: ValueId,
    ty: TypeId,
}

impl OperationValueResult {
    /// Construct a result from its value and semantic type identities.
    #[must_use]
    pub const fn new(value: ValueId, ty: TypeId) -> Self {
        Self { value, ty }
    }

    /// Return the result value identity.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the semantic type identity.
    #[must_use]
    pub const fn ty(&self) -> TypeId {
        self.ty
    }
}

// =============================================================================
// Operation errors
// =============================================================================

/// Errors produced by operation-local construction and validation.
///
/// These errors deliberately do not depend on verifier internals or a
/// particular diagnostics frontend. The public ZUIR API can translate them
/// into the repository-wide structured error and diagnostic types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationError {
    /// The qualified operation name is malformed.
    InvalidOperationName(String),

    /// A result identity occurs more than once in this operation.
    DuplicateResult(ValueId),

    /// A nested region identity occurs more than once in this operation.
    DuplicateRegion(RegionId),

    /// An operand role is malformed.
    InvalidOperandRole(String),

    /// A declared effect name is malformed.
    InvalidEffectName(String),

    /// An attribute key is malformed.
    InvalidAttributeKey(String),

    /// An attribute value is structurally invalid.
    InvalidAttributeValue {
        key: String,
        reason: String,
    },

    /// An opaque payload's encoding name is malformed.
    InvalidPayloadEncoding(String),
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperationName(reason) => {
                write!(f, "invalid qualified operation name: {reason}")
            }
            Self::DuplicateResult(value) => {
                write!(f, "duplicate operation result value identity: {value}")
            }
            Self::DuplicateRegion(region) => {
                write!(f, "duplicate nested region reference: {region}")
            }
            Self::InvalidOperandRole(reason) => {
                write!(f, "invalid operation operand role: {reason}")
            }
            Self::InvalidEffectName(reason) => {
                write!(f, "invalid operation effect name: {reason}")
            }
            Self::InvalidAttributeKey(reason) => {
                write!(f, "invalid operation attribute key: {reason}")
            }
            Self::InvalidAttributeValue { key, reason } => {
                write!(f, "invalid value for operation attribute {key}: {reason}")
            }
            Self::InvalidPayloadEncoding(reason) => {
                write!(f, "invalid opaque payload encoding: {reason}")
            }
        }
    }
}

impl std::error::Error for OperationError {}

// =============================================================================
// Canonical operation
// =============================================================================

/// Canonical target-independent operation node.
///
/// `ZuirOperation` is intentionally generic. It does not contain a closed
/// enumeration of classical instructions, quantum gates, hardware primitives,
/// tensor operations, or network instructions.
///
/// A dialect registry supplies operation semantics and the schema that
/// constrains operand roles, result types, attributes, nested regions, effects,
/// and operation-specific verification.
///
/// # Local versus global validity
///
/// `validate()` checks facts that can be checked using this operation alone.
/// It cannot prove that operands are defined, result types exist, regions
/// belong to this operation, effect declarations are complete, or the
/// operation is legal in a particular block. Those facts require the
/// whole-program verifier and the dialect registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZuirOperation {
    id: OperationId,
    name: QualifiedName,

    /// Operands are ordered and may reference the same value more than once.
    operands: Vec<OperationOperand>,

    /// Results are ordered; each result has a distinct value identity locally.
    results: Vec<OperationValueResult>,

    /// Namespaced declarative attributes, ordered by their canonical keys.
    attributes: BTreeMap<AttributeKey, AttributeValue>,

    /// References to regions nested under this operation.
    ///
    /// Region existence, unique global ownership, and reciprocal owner
    /// relationships are checked by the whole-program verifier.
    regions: Vec<RegionId>,

    /// Declared semantic effects.
    ///
    /// This set records declarations only. It does not infer effects or prove
    /// that the declaration is complete. Set ordering is deterministic and
    /// does not imply execution ordering.
    effects: BTreeSet<QualifiedName>,

    /// Optional uninterpreted extension payload.
    opaque_payload: Option<OpaqueOperationPayload>,
}

impl ZuirOperation {
    /// Construct an operation with no operands, results, regions, attributes,
    /// effects, or opaque payload.
    ///
    /// The operation is structurally empty but valid as a construction state.
    /// Whether a particular dialect permits zero operands or zero results is
    /// determined by that dialect's operation schema.
    pub fn new(
        id: OperationId,
        name: QualifiedName,
    ) -> OperationResultType<Self> {
        name.validate()
            .map_err(|error| OperationError::InvalidOperationName(
                error.to_string(),
            ))?;

        Ok(Self {
            id,
            name,
            operands: Vec::new(),
            results: Vec::new(),
            attributes: BTreeMap::new(),
            regions: Vec::new(),
            effects: BTreeSet::new(),
            opaque_payload: None,
        })
    }

    /// Construct an operation with operands and results already supplied.
    ///
    /// The input vectors are validated before the new operation is returned.
    pub fn with_signature(
        id: OperationId,
        name: QualifiedName,
        operands: Vec<OperationOperand>,
        results: Vec<OperationValueResult>,
    ) -> OperationResultType<Self> {
        let mut operation = Self::new(id, name)?;
        operation.set_operands(operands)?;
        operation.set_results(results)?;
        Ok(operation)
    }

    /// Return the operation identity.
    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    /// Return the qualified operation name.
    #[must_use]
    pub const fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Replace the operation name after validating its syntax.
    ///
    /// This does not prove that the name is registered or that the operation's
    /// existing fields satisfy the new operation schema.
    pub fn set_name(
        &mut self,
        name: QualifiedName,
    ) -> OperationResultType<()> {
        name.validate()
            .map_err(|error| OperationError::InvalidOperationName(
                error.to_string(),
            ))?;

        self.name = name;
        Ok(())
    }

    /// Borrow the ordered operand list.
    #[must_use]
    pub fn operands(&self) -> &[OperationOperand] {
        &self.operands
    }

    /// Return the number of operands.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Append an operand.
    pub fn add_operand(
        &mut self,
        operand: OperationOperand,
    ) -> OperationResultType<()> {
        operand.validate()?;
        self.operands.push(operand);
        Ok(())
    }

    /// Replace all operands after validating each operand.
    ///
    /// The existing operands remain unchanged if validation fails.
    pub fn set_operands(
        &mut self,
        operands: Vec<OperationOperand>,
    ) -> OperationResultType<()> {
        for operand in &operands {
            operand.validate()?;
        }

        self.operands = operands;
        Ok(())
    }

    /// Remove and return the operand at `index`.
    ///
    /// Returns `None` when the index is outside the current operand list.
    pub fn remove_operand(
        &mut self,
        index: usize,
    ) -> Option<OperationOperand> {
        if index < self.operands.len() {
            Some(self.operands.remove(index))
        } else {
            None
        }
    }

    /// Borrow the ordered result list.
    #[must_use]
    pub fn results(&self) -> &[OperationValueResult] {
        &self.results
    }

    /// Return the number of results.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Append a result if its value identity is not already used by this
    /// operation.
    pub fn add_result(
        &mut self,
        result: OperationValueResult,
    ) -> OperationResultType<()> {
        if self.results.iter().any(|existing| existing.value == result.value) {
            return Err(OperationError::DuplicateResult(result.value));
        }

        self.results.push(result);
        Ok(())
    }

    /// Replace all results after checking local result-identity uniqueness.
    ///
    /// The existing results remain unchanged if validation fails.
    pub fn set_results(
        &mut self,
        results: Vec<OperationValueResult>,
    ) -> OperationResultType<()> {
        let mut seen = BTreeSet::new();

        for result in &results {
            if !seen.insert(result.value) {
                return Err(OperationError::DuplicateResult(result.value));
            }
        }

        self.results = results;
        Ok(())
    }

    /// Remove and return a result by its ordered index.
    ///
    /// Removing a result may invalidate uses elsewhere in the IR. Callers must
    /// update and reverify those uses before treating the program as valid.
    pub fn remove_result(
        &mut self,
        index: usize,
    ) -> Option<OperationValueResult> {
        if index < self.results.len() {
            Some(self.results.remove(index))
        } else {
            None
        }
    }

    /// Borrow the operation's attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<AttributeKey, AttributeValue> {
        &self.attributes
    }

    /// Return one attribute by its complete qualified key.
    #[must_use]
    pub fn attribute(
        &self,
        key: &AttributeKey,
    ) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Insert or replace an attribute.
    ///
    /// Both key and value are structurally validated. The previous value is
    /// returned if the key already existed. Namespace authorization and
    /// attribute-schema compatibility remain dialect-verifier responsibilities.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> OperationResultType<Option<AttributeValue>> {
        key.validate()
            .map_err(|error| OperationError::InvalidAttributeKey(
                error.to_string(),
            ))?;

        value.validate().map_err(|error| {
            OperationError::InvalidAttributeValue {
                key: key.to_string(),
                reason: error.to_string(),
            }
        })?;

        Ok(self.attributes.insert(key, value))
    }

    /// Remove an attribute by its complete qualified key.
    pub fn remove_attribute(
        &mut self,
        key: &AttributeKey,
    ) -> Option<AttributeValue> {
        self.attributes.remove(key)
    }

    /// Borrow the ordered nested-region references.
    #[must_use]
    pub fn regions(&self) -> &[RegionId] {
        &self.regions
    }

    /// Return the number of nested-region references.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Add a nested-region reference if it is not already present.
    ///
    /// This checks only local uniqueness. Region existence and reciprocal
    /// ownership are checked by the whole-program verifier.
    pub fn add_region(
        &mut self,
        region: RegionId,
    ) -> OperationResultType<()> {
        if self.regions.contains(&region) {
            return Err(OperationError::DuplicateRegion(region));
        }

        self.regions.push(region);
        Ok(())
    }

    /// Replace nested-region references after checking local uniqueness.
    ///
    /// The supplied order is preserved because an operation schema may define
    /// region order as meaningful.
    pub fn set_regions(
        &mut self,
        regions: Vec<RegionId>,
    ) -> OperationResultType<()> {
        let mut seen = BTreeSet::new();

        for region in &regions {
            if !seen.insert(*region) {
                return Err(OperationError::DuplicateRegion(*region));
            }
        }

        self.regions = regions;
        Ok(())
    }

    /// Remove a nested-region reference.
    ///
    /// Removing the reference does not delete the region itself. The owning
    /// program builder or transformation must update ownership consistently.
    pub fn remove_region(
        &mut self,
        region: RegionId,
    ) -> bool {
        if let Some(index) = self.regions.iter().position(|candidate| *candidate == region) {
            self.regions.remove(index);
            true
        } else {
            false
        }
    }

    /// Borrow the declared semantic effect names.
    #[must_use]
    pub fn effects(&self) -> &BTreeSet<QualifiedName> {
        &self.effects
    }

    /// Add a declared effect after validating its qualified name.
    ///
    /// Returns `true` when the effect was newly inserted and `false` if it was
    /// already declared.
    pub fn add_effect(
        &mut self,
        effect: QualifiedName,
    ) -> OperationResultType<bool> {
        effect
            .validate()
            .map_err(|error| OperationError::InvalidEffectName(
                error.to_string(),
            ))?;

        Ok(self.effects.insert(effect))
    }

    /// Replace the effect set after validating every effect name.
    ///
    /// The old set remains unchanged if validation fails.
    pub fn set_effects(
        &mut self,
        effects: BTreeSet<QualifiedName>,
    ) -> OperationResultType<()> {
        for effect in &effects {
            effect
                .validate()
                .map_err(|error| OperationError::InvalidEffectName(
                    error.to_string(),
                ))?;
        }

        self.effects = effects;
        Ok(())
    }

    /// Remove a declared effect.
    pub fn remove_effect(
        &mut self,
        effect: &QualifiedName,
    ) -> bool {
        self.effects.remove(effect)
    }

    /// Borrow the optional uninterpreted extension payload.
    #[must_use]
    pub const fn opaque_payload(&self) -> Option<&OpaqueOperationPayload> {
        self.opaque_payload.as_ref()
    }

    /// Set or replace the opaque extension payload.
    pub fn set_opaque_payload(
        &mut self,
        payload: Option<OpaqueOperationPayload>,
    ) -> OperationResultType<()> {
        if let Some(value) = &payload {
            value.validate()?;
        }

        self.opaque_payload = payload;
        Ok(())
    }

    /// Validate every operation-local invariant.
    ///
    /// This method intentionally does not consult a dialect registry or the
    /// enclosing program. A successful result means that the operation's
    /// local representation is well-formed, not that it is executable.
    pub fn validate(&self) -> OperationResultType<()> {
        self.name
            .validate()
            .map_err(|error| OperationError::InvalidOperationName(
                error.to_string(),
            ))?;

        for operand in &self.operands {
            operand.validate()?;
        }

        let mut result_ids = BTreeSet::new();

        for result in &self.results {
            if !result_ids.insert(result.value) {
                return Err(OperationError::DuplicateResult(result.value));
            }
        }

        let mut region_ids = BTreeSet::new();

        for region in &self.regions {
            if !region_ids.insert(*region) {
                return Err(OperationError::DuplicateRegion(*region));
            }
        }

        for (key, value) in &self.attributes {
            key.validate()
                .map_err(|error| OperationError::InvalidAttributeKey(
                    error.to_string(),
                ))?;

            value.validate().map_err(|error| {
                OperationError::InvalidAttributeValue {
                    key: key.to_string(),
                    reason: error.to_string(),
                }
            })?;
        }

        for effect in &self.effects {
            effect
                .validate()
                .map_err(|error| OperationError::InvalidEffectName(
                    error.to_string(),
                ))?;
        }

        if let Some(payload) = &self.opaque_payload {
            payload.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_id(value: u128) -> OperationId {
        OperationId::new(value).expect("test operation ID must be nonzero")
    }

    fn region_id(value: u128) -> RegionId {
        RegionId::new(value).expect("test region ID must be nonzero")
    }

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("test value ID must be nonzero")
    }

    fn type_id(value: u128) -> TypeId {
        TypeId::new(value).expect("test type ID must be nonzero")
    }

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name must be valid")
    }

    fn operation() -> ZuirOperation {
        ZuirOperation::new(operation_id(1), name("core::add"))
            .expect("test operation should be constructible")
    }

    #[test]
    fn constructs_generic_operation() {
        let operation = operation();

        assert_eq!(operation.id(), operation_id(1));
        assert_eq!(operation.name(), &name("core::add"));
        assert!(operation.operands().is_empty());
        assert!(operation.results().is_empty());
        assert!(operation.regions().is_empty());
        assert!(operation.effects().is_empty());
        assert!(operation.attributes().is_empty());
        assert!(operation.validate().is_ok());
    }

    #[test]
    fn accepts_repeated_operands() {
        let mut operation = operation();
        let operand = OperationOperand::new(value_id(10));

        operation.add_operand(operand.clone()).unwrap();
        operation.add_operand(operand).unwrap();

        assert_eq!(operation.operand_count(), 2);
        assert_eq!(operation.operands()[0].value(), value_id(10));
        assert_eq!(operation.operands()[1].value(), value_id(10));
    }

    #[test]
    fn accepts_role_qualified_operand() {
        let operand = OperationOperand::with_role(
            value_id(10),
            name("quantum::control"),
        )
        .unwrap();

        assert_eq!(operand.value(), value_id(10));
        assert_eq!(operand.role(), Some(&name("quantum::control")));
        assert!(operand.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_result_identity() {
        let mut operation = operation();

        operation
            .add_result(OperationValueResult::new(value_id(20), type_id(1)))
            .unwrap();

        let error = operation
            .add_result(OperationValueResult::new(value_id(20), type_id(2)))
            .unwrap_err();

        assert_eq!(error, OperationError::DuplicateResult(value_id(20)));
        assert_eq!(operation.result_count(), 1);
    }

    #[test]
    fn set_results_is_atomic_on_failure() {
        let mut operation = operation();

        operation
            .add_result(OperationValueResult::new(value_id(20), type_id(1)))
            .unwrap();

        let replacement = vec![
            OperationValueResult::new(value_id(30), type_id(1)),
            OperationValueResult::new(value_id(30), type_id(2)),
        ];

        assert!(operation.set_results(replacement).is_err());
        assert_eq!(operation.result_count(), 1);
        assert_eq!(operation.results()[0].value(), value_id(20));
    }

    #[test]
    fn preserves_region_order() {
        let mut operation = operation();

        operation
            .set_regions(vec![region_id(8), region_id(3), region_id(5)])
            .unwrap();

        assert_eq!(
            operation.regions(),
            &[region_id(8), region_id(3), region_id(5)]
        );
    }

    #[test]
    fn rejects_duplicate_region_reference() {
        let mut operation = operation();

        let error = operation
            .set_regions(vec![region_id(2), region_id(2)])
            .unwrap_err();

        assert_eq!(error, OperationError::DuplicateRegion(region_id(2)));
        assert!(operation.regions().is_empty());
    }

    #[test]
    fn attributes_are_validated_and_retrievable() {
        let mut operation = operation();
        let key = AttributeKey::parse("core::description").unwrap();
        let value = AttributeValue::String("portable addition".to_owned());

        assert!(operation
            .set_attribute(key.clone(), value.clone())
            .unwrap()
            .is_none());

        assert_eq!(operation.attribute(&key), Some(&value));
        assert!(operation.validate().is_ok());
    }

    #[test]
    fn effect_set_is_deterministic() {
        let mut operation = operation();

        operation.add_effect(name("memory::read")).unwrap();
        operation.add_effect(name("io::write")).unwrap();
        operation.add_effect(name("memory::read")).unwrap();

        let effects: Vec<_> = operation.effects().iter().cloned().collect();

        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&name("memory::read")));
        assert!(effects.contains(&name("io::write")));
    }

    #[test]
    fn opaque_payload_does_not_change_operation_identity() {
        let mut operation = operation();
        let payload = OpaqueOperationPayload::new(
            name("vendor_example::binary_payload"),
            vec![0, 1, 2, 255],
        )
        .unwrap();

        operation.set_opaque_payload(Some(payload.clone())).unwrap();

        assert_eq!(operation.opaque_payload(), Some(&payload));
        assert!(operation.validate().is_ok());
    }

    #[test]
    fn removing_missing_items_is_non_destructive() {
        let mut operation = operation();

        assert!(operation.remove_operand(0).is_none());
        assert!(operation.remove_result(0).is_none());
        assert!(!operation.remove_region(region_id(99)));
        assert!(!operation.remove_effect(&name("memory::read")));

        assert!(operation.validate().is_ok());
    }

    #[test]
    fn validates_after_deserialization_boundary() {
        let mut operation = operation();

        operation
            .add_operand(OperationOperand::new(value_id(10)))
            .unwrap();

        operation
            .add_result(OperationValueResult::new(value_id(11), type_id(1)))
            .unwrap();

        assert!(operation.validate().is_ok());

        // Serde-derived deserialization is a trust boundary. Callers must run
        // validate() after decoding before the IR is processed further.
    }
}