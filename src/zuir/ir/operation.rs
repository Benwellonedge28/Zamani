
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Operation
//!
//! Canonical path: `src/zuir/ir/operation.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Represent one target-independent operation in Zamani's Universal
//! Intermediate Representation. An operation contains a qualified name,
//! ordered operand and result references, attributes, nested-region
//! references, declared effects, and optional uninterpreted extension data.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/operand.rs`
//! - `src/zuir/ir/result.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/effects.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/spec/quantum.md`
//!
//! OWNS
//!
//! - Operation-local representation.
//! - Operation-local validation and mutation.
//! - Opaque operation payload storage.
//! - Operation-specific errors.
//!
//! DOES NOT OWN
//!
//! - Operand or result type definitions.
//! - Identifier allocation.
//! - Block instruction ordering or terminators.
//! - Region storage or ownership.
//! - Dialect registration or operation execution.
//! - Source syntax, AST construction, name resolution, or type inference.
//! - Global SSA, dominance, use-def, effect, ownership, or CFG verification.
//! - Target-specific layout, placement, routing, scheduling, or execution.
//!
//! SCALABILITY AND POCO-REAF
//!
//! No universal operation, operand, result, region, hardware, qubit, register,
//! thread, tensor-rank, device, or node limit is defined here. Vectors grow
//! dynamically, and explicit try_reserve methods let callers detect capacity
//! reservation failures before appending. Ordinary allocation can still fail
//! according to the platform's allocator behavior.
//!
//! No finite process can promise literal infinity. POCO-REAF means that this
//! representation imposes no artificial universal machine-size ceiling.
//!
//! UNKNOWN OPERATIONS
//!
//! A valid QualifiedName does not prove that a dialect or operation is
//! registered. Unknown operations and payloads must remain uninterpreted
//! until a trusted dialect contract defines their meaning. They must not be
//! silently executed, optimized, or lowered.
//!
//! VALIDITY BOUNDARY
//!
//! validate() checks local invariants only. The whole-IR verifier must resolve
//! references, establish global identity uniqueness, verify types and SSA,
//! validate region ownership and control flow, and consult dialect schemas.
//!
//! Deserialization validates operation-local structure. It does not establish
//! whole-program validity or authorize execution.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{OperationId, RegionId};
use crate::zuir::ir::operand::OperationOperand;
use crate::zuir::ir::result::OperationValueResult;
use crate::zuir::schema::attributes::{AttributeKey, AttributeValue};
use crate::zuir::schema::types::QualifiedName;

/// Result type for operation-local construction, mutation, and validation.
pub type OperationResultType<T> = Result<T, OperationError>;

// =============================================================================
// Opaque extension payload
// =============================================================================

/// Uninterpreted extension data attached to an operation.
///
/// `encoding` identifies the payload encoding, not the operation's semantics.
/// Only a registered and trusted extension may interpret this payload.
///
/// Payload bytes are preserved as supplied. Their internal schema, version,
/// integrity, and semantic compatibility belong to the extension and
/// serialization contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpaqueOperationPayload {
    encoding: QualifiedName,
    bytes: Vec<u8>,
}

impl OpaqueOperationPayload {
    /// Construct a payload using a validated qualified encoding name.
    pub fn new(
        encoding: QualifiedName,
        bytes: Vec<u8>,
    ) -> OperationResultType<Self> {
        encoding
            .validate()
            .map_err(|error| {
                OperationError::InvalidPayloadEncoding(error.to_string())
            })?;

        Ok(Self { encoding, bytes })
    }

    /// Return the payload encoding name.
    #[must_use]
    pub fn encoding(&self) -> &QualifiedName {
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

    /// Validate the payload metadata.
    pub fn validate(&self) -> OperationResultType<()> {
        self.encoding
            .validate()
            .map_err(|error| {
                OperationError::InvalidPayloadEncoding(error.to_string())
            })
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors detectable by operation-local construction or validation.
///
/// Cross-entity errors belong to the relevant whole-IR verifier. These
/// errors deliberately do not depend on diagnostics rendering or backends.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperationError {
    /// The operation's qualified name is malformed.
    InvalidOperationName(String),

    /// A result value identity occurs more than once in this operation.
    DuplicateResult(crate::zuir::ids::ValueId),

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

    /// An opaque payload encoding name is malformed.
    InvalidPayloadEncoding(String),

    /// Reservation for a dynamic collection failed.
    AllocationFailure {
        collection: &'static str,
    },
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
            Self::AllocationFailure { collection } => {
                write!(
                    f,
                    "unable to reserve memory for operation {collection}"
                )
            }
        }
    }
}

impl std::error::Error for OperationError {}

// =============================================================================
// Canonical operation
// =============================================================================

/// Canonical, target-independent operation node.
///
/// This type deliberately has no closed enum of computational instructions.
/// Classical, quantum, HDL, tensor, distributed, networking, cryptographic,
/// and future operations are identified by qualified names and interpreted
/// through their registered dialect contracts.
///
/// Fields remain private so callers use the validated public mutation API.
/// Deserialized data is also checked by the operation-local validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ZuirOperationWire")]
pub struct ZuirOperation {
    id: OperationId,
    name: QualifiedName,

    /// Ordered input references. Repeated operands are permitted.
    operands: Vec<OperationOperand>,

    /// Ordered typed results produced by this operation.
    results: Vec<OperationValueResult>,

    /// Qualified attribute keys in deterministic key order.
    attributes: BTreeMap<AttributeKey, AttributeValue>,

    /// Ordered references to regions nested under this operation.
    regions: Vec<RegionId>,

    /// Declared semantic effects, in deterministic set order.
    effects: BTreeSet<QualifiedName>,

    /// Optional opaque data for a registered extension or interchange layer.
    opaque_payload: Option<OpaqueOperationPayload>,
}

/// Deserialization representation.
///
/// It is private to this module so the canonical operation API remains the
/// only public construction boundary. `TryFrom` validates local invariants
/// before deserialized values become a `ZuirOperation`.
#[derive(Deserialize)]
struct ZuirOperationWire {
    id: OperationId,
    name: QualifiedName,
    operands: Vec<OperationOperand>,
    results: Vec<OperationValueResult>,
    attributes: BTreeMap<AttributeKey, AttributeValue>,
    regions: Vec<RegionId>,
    effects: BTreeSet<QualifiedName>,
    opaque_payload: Option<OpaqueOperationPayload>,
}

impl TryFrom<ZuirOperationWire> for ZuirOperation {
    type Error = OperationError;

    fn try_from(wire: ZuirOperationWire) -> Result<Self, Self::Error> {
        let operation = Self {
            id: wire.id,
            name: wire.name,
            operands: wire.operands,
            results: wire.results,
            attributes: wire.attributes,
            regions: wire.regions,
            effects: wire.effects,
            opaque_payload: wire.opaque_payload,
        };

        operation.validate()?;
        Ok(operation)
    }
}

impl ZuirOperation {
    /// Construct an operation with empty operand, result, region, attribute,
    /// and effect collections.
    ///
    /// Whether an empty signature is legal for a particular operation is
    /// decided by its registered dialect schema, not by this constructor.
    pub fn new(
        id: OperationId,
        name: QualifiedName,
    ) -> OperationResultType<Self> {
        name.validate().map_err(|error| {
            OperationError::InvalidOperationName(error.to_string())
        })?;

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

    /// Construct an operation with pre-existing operands and results.
    ///
    /// Both collections are validated before being installed. On error, no
    /// operation is returned.
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

    /// Return this operation's identity.
    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    /// Return this operation's qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Replace the operation name after validating its syntax.
    ///
    /// This does not check dialect registration or the new operation's schema.
    pub fn set_name(
        &mut self,
        name: QualifiedName,
    ) -> OperationResultType<()> {
        name.validate().map_err(|error| {
            OperationError::InvalidOperationName(error.to_string())
        })?;

        self.name = name;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Operands
    // -------------------------------------------------------------------------

    /// Borrow the ordered operands.
    #[must_use]
    pub fn operands(&self) -> &[OperationOperand] {
        &self.operands
    }

    /// Return the operand count.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Reserve additional operand capacity without changing the operand list.
    ///
    /// Call this before a large batch append when allocation failure must be
    /// reported as an ordinary error.
    pub fn try_reserve_operands(
        &mut self,
        additional: usize,
    ) -> OperationResultType<()> {
        self.operands
            .try_reserve(additional)
            .map_err(|_| OperationError::AllocationFailure {
                collection: "operands",
            })
    }

    /// Append a validated operand.
    pub fn add_operand(
        &mut self,
        operand: OperationOperand,
    ) -> OperationResultType<()> {
        operand.validate().map_err(|error| {
            OperationError::InvalidOperandRole(error.to_string())
        })?;

        self.operands.push(operand);
        Ok(())
    }

    /// Replace all operands after validating every operand.
    ///
    /// The current collection remains unchanged if validation fails.
    pub fn set_operands(
        &mut self,
        operands: Vec<OperationOperand>,
    ) -> OperationResultType<()> {
        for operand in &operands {
            operand.validate().map_err(|error| {
                OperationError::InvalidOperandRole(error.to_string())
            })?;
        }

        self.operands = operands;
        Ok(())
    }

    /// Remove an operand by index.
    ///
    /// Returns `None` when the index is out of range.
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

    // -------------------------------------------------------------------------
    // Results
    // -------------------------------------------------------------------------

    /// Borrow the ordered operation results.
    #[must_use]
    pub fn results(&self) -> &[OperationValueResult] {
        &self.results
    }

    /// Return the result count.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Reserve additional result capacity without changing the result list.
    pub fn try_reserve_results(
        &mut self,
        additional: usize,
    ) -> OperationResultType<()> {
        self.results
            .try_reserve(additional)
            .map_err(|_| OperationError::AllocationFailure {
                collection: "results",
            })
    }

    /// Append a result if its value identity is unique within this operation.
    pub fn add_result(
        &mut self,
        result: OperationValueResult,
    ) -> OperationResultType<()> {
        if self
            .results
            .iter()
            .any(|existing| existing.value() == result.value())
        {
            return Err(OperationError::DuplicateResult(result.value()));
        }

        self.results.push(result);
        Ok(())
    }

    /// Replace all results after checking local value-identity uniqueness.
    ///
    /// The current collection remains unchanged if validation fails.
    pub fn set_results(
        &mut self,
        results: Vec<OperationValueResult>,
    ) -> OperationResultType<()> {
        let mut seen = BTreeSet::new();

        for result in &results {
            if !seen.insert(result.value()) {
                return Err(OperationError::DuplicateResult(result.value()));
            }
        }

        self.results = results;
        Ok(())
    }

    /// Remove a result by its ordinal index.
    ///
    /// Removing a result may invalidate external uses. The caller must update
    /// affected IR and rerun the appropriate verifier.
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

    // -------------------------------------------------------------------------
    // Attributes
    // -------------------------------------------------------------------------

    /// Borrow the operation's deterministically ordered attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<AttributeKey, AttributeValue> {
        &self.attributes
    }

    /// Look up an attribute by its complete qualified key.
    #[must_use]
    pub fn attribute(
        &self,
        key: &AttributeKey,
    ) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Insert or replace an attribute after structural validation.
    ///
    /// Returns the previous value if one existed. Whether the namespace is
    /// authorized and whether the attribute is allowed by the operation
    /// schema are responsibilities of dialect verification.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> OperationResultType<Option<AttributeValue>> {
        key.validate().map_err(|error| {
            OperationError::InvalidAttributeKey(error.to_string())
        })?;

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

    // -------------------------------------------------------------------------
    // Nested regions
    // -------------------------------------------------------------------------

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

    /// Reserve additional nested-region capacity.
    pub fn try_reserve_regions(
        &mut self,
        additional: usize,
    ) -> OperationResultType<()> {
        self.regions
            .try_reserve(additional)
            .map_err(|_| OperationError::AllocationFailure {
                collection: "regions",
            })
    }

    /// Add a region reference if it is not already present.
    ///
    /// This does not create the region or establish its ownership.
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

    /// Replace all region references after checking local uniqueness.
    ///
    /// The input order is preserved. The current list is unchanged if
    /// duplicate references are found.
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

    /// Remove a region reference without deleting the region itself.
    pub fn remove_region(
        &mut self,
        region: RegionId,
    ) -> bool {
        if let Some(index) = self
            .regions
            .iter()
            .position(|candidate| *candidate == region)
        {
            self.regions.remove(index);
            true
        } else {
            false
        }
    }

    // -------------------------------------------------------------------------
    // Effects
    // -------------------------------------------------------------------------

    /// Borrow the declared effect names in deterministic order.
    #[must_use]
    pub fn effects(&self) -> &BTreeSet<QualifiedName> {
        &self.effects
    }

    /// Add a declared effect after validating its qualified name.
    ///
    /// Returns `true` if inserted, or `false` if it was already declared.
    pub fn add_effect(
        &mut self,
        effect: QualifiedName,
    ) -> OperationResultType<bool> {
        effect.validate().map_err(|error| {
            OperationError::InvalidEffectName(error.to_string())
        })?;

        Ok(self.effects.insert(effect))
    }

    /// Replace the effect set after validating every name.
    ///
    /// The existing set is unchanged if validation fails.
    pub fn set_effects(
        &mut self,
        effects: BTreeSet<QualifiedName>,
    ) -> OperationResultType<()> {
        for effect in &effects {
            effect.validate().map_err(|error| {
                OperationError::InvalidEffectName(error.to_string())
            })?;
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

    // -------------------------------------------------------------------------
    // Opaque extension payload
    // -------------------------------------------------------------------------

    /// Borrow the optional opaque payload.
    #[must_use]
    pub fn opaque_payload(&self) -> Option<&OpaqueOperationPayload> {
        self.opaque_payload.as_ref()
    }

    /// Set or clear the opaque payload after validation.
    pub fn set_opaque_payload(
        &mut self,
        payload: Option<OpaqueOperationPayload>,
    ) -> OperationResultType<()> {
        if let Some(candidate) = &payload {
            candidate.validate()?;
        }

        self.opaque_payload = payload;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Local validation
    // -------------------------------------------------------------------------

    /// Validate every invariant that can be checked from this operation
    /// alone, without consulting the dialect registry or enclosing program.
    pub fn validate(&self) -> OperationResultType<()> {
        self.name.validate().map_err(|error| {
            OperationError::InvalidOperationName(error.to_string())
        })?;

        for operand in &self.operands {
            operand.validate().map_err(|error| {
                OperationError::InvalidOperandRole(error.to_string())
            })?;
        }

        let mut result_ids = BTreeSet::new();

        for result in &self.results {
            if !result_ids.insert(result.value()) {
                return Err(OperationError::DuplicateResult(result.value()));
            }
        }

        let mut region_ids = BTreeSet::new();

        for region in &self.regions {
            if !region_ids.insert(*region) {
                return Err(OperationError::DuplicateRegion(*region));
            }
        }

        for (key, value) in &self.attributes {
            key.validate().map_err(|error| {
                OperationError::InvalidAttributeKey(error.to_string())
            })?;

            value.validate().map_err(|error| {
                OperationError::InvalidAttributeValue {
                    key: key.to_string(),
                    reason: error.to_string(),
                }
            })?;
        }

        for effect in &self.effects {
            effect.validate().map_err(|error| {
                OperationError::InvalidEffectName(error.to_string())
            })?;
        }

        if let Some(payload) = &self.opaque_payload {
            payload.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::{TypeId, ValueId};

    fn operation_id(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test ID must be nonzero")
    }

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test ID must be nonzero")
    }

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test name must be valid")
    }

    fn operation() -> ZuirOperation {
        ZuirOperation::new(operation_id(1), name("core::add"))
            .expect("operation construction should succeed")
    }

    #[test]
    fn constructs_a_generic_operation() {
        let op = operation();

        assert_eq!(op.id(), operation_id(1));
        assert_eq!(op.name(), &name("core::add"));
        assert!(op.operands().is_empty());
        assert!(op.results().is_empty());
        assert!(op.regions().is_empty());
        assert!(op.effects().is_empty());
        assert!(op.attributes().is_empty());
        assert!(op.validate().is_ok());
    }

    #[test]
    fn allows_repeated_operand_references() {
        let mut op = operation();
        let operand = OperationOperand::new(value_id(10));

        op.add_operand(operand.clone()).unwrap();
        op.add_operand(operand).unwrap();

        assert_eq!(op.operand_count(), 2);
        assert_eq!(op.operands()[0].value(), value_id(10));
        assert_eq!(op.operands()[1].value(), value_id(10));
    }

    #[test]
    fn rejects_duplicate_result_values() {
        let mut op = operation();

        op.add_result(OperationValueResult::new(value_id(20), type_id(1)))
            .unwrap();

        let error = op
            .add_result(OperationValueResult::new(value_id(20), type_id(2)))
            .unwrap_err();

        assert_eq!(error, OperationError::DuplicateResult(value_id(20)));
        assert_eq!(op.result_count(), 1);
    }

    #[test]
    fn replacing_results_is_atomic_on_validation_error() {
        let mut op = operation();

        op.add_result(OperationValueResult::new(value_id(20), type_id(1)))
            .unwrap();

        let replacement = vec![
            OperationValueResult::new(value_id(30), type_id(1)),
            OperationValueResult::new(value_id(30), type_id(2)),
        ];

        assert!(op.set_results(replacement).is_err());
        assert_eq!(op.result_count(), 1);
        assert_eq!(op.results()[0].value(), value_id(20));
    }

    #[test]
    fn rejects_duplicate_region_references() {
        let mut op = operation();
        let region = RegionId::new(7).expect("test ID must be nonzero");

        op.add_region(region).unwrap();
        assert_eq!(
            op.add_region(region),
            Err(OperationError::DuplicateRegion(region))
        );
        assert_eq!(op.regions(), &[region]);
    }

    #[test]
    fn validates_names_and_effects() {
        let mut op = operation();

        op.add_effect(name("memory::read")).unwrap();

        assert_eq!(op.effects().len(), 1);
        assert!(op.set_name(name("quantum::measure")).is_ok());
        assert!(op.validate().is_ok());
    }

    #[test]
    fn serde_round_trip_preserves_a_valid_operation() {
        let mut op = operation();
        op.add_operand(OperationOperand::new(value_id(10))).unwrap();
        op.add_result(OperationValueResult::new(value_id(11), type_id(2)))
            .unwrap();
        op.add_effect(name("memory::read")).unwrap();

        let encoded = serde_json::to_string(&op).unwrap();
        let decoded: ZuirOperation = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, op);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn explicit_reservation_does_not_change_logical_contents() {
        let mut op = operation();
        op.add_operand(OperationOperand::new(value_id(10))).unwrap();

        let before = op.operands().to_vec();
        op.try_reserve_operands(64).unwrap();

        assert_eq!(op.operands(), before.as_slice());
    }
}
