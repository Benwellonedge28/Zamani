
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Value Definitions and Use Index
//!
//! Path: `src/hdl/hdlir/ir/values.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define graph-level value records and a deterministic value table for
//! Zamani's hardware-domain intermediate representation (HDLIR).
//!
//! A value record connects an HDLIR value identity to its type, semantic
//! category, and defining origin. The table provides checked insertion,
//! lookup, removal, iteration, and consistency checks against operation
//! results. A derived use index records which operations consume each value.
//!
//! ## Authority
//!
//! This module must conform to:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars.
//! - `grammar/hdl/ports.g4`.
//! - `grammar/specification/semantic-model.md`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/ids.rs`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/ir/operations.rs`.
//! - `src/hdl/hdlir/types/`.
//! - `src/hdl/hdlir/values/`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! Grammar defines source syntax. The canonical frontend owns source-level
//! semantic meaning. ZUIR owns the universal computational representation.
//! HDLIR owns this hardware-domain value graph.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The association between an HDLIR value ID, its type ID, its category,
//!   and its defining origin.
//! - A deterministic table of value definitions.
//! - Local uniqueness and identity consistency checks.
//! - A derived index of operation operand uses.
//! - Cross-checks between value definitions and operation result declarations.
//!
//! This module does not own:
//!
//! - Source parsing, AST nodes, name resolution, or source-level type checking.
//! - The authoritative type definitions or symbolic width evaluator.
//! - Operation descriptors, operation semantics, or operation execution.
//! - Regions, blocks, control flow, or the complete design graph.
//! - Signal connectivity, driver resolution, clock semantics, or memory
//!   behavior.
//! - Physical registers, gates, cells, device allocation, placement, or routing.
//! - Persistent cross-build identity or serialized artifact encoding.
//!
//! ## Critical semantic distinction
//!
//! `ValueSemantics` distinguishes transient computed values from persistent
//! hardware concepts. It does not define their complete behavior:
//!
//! - `Combinational` describes a value produced by combinational computation.
//! - `Interface` describes a value associated with a logical interface.
//! - `Signal` describes a persistent logical signal.
//! - `State` identifies a value representing or referring to stateful hardware.
//! - `MemoryReference` identifies an abstract memory reference or handle.
//! - `Extension` preserves an extension-defined category without claiming
//!   that this core module understands its semantics.
//!
//! A signal is not automatically an SSA temporary. A state value is not
//! automatically a physical flip-flop. A memory reference does not specify
//! physical banks or storage technology. Those meanings belong to the
//! appropriate registered HDLIR contracts and downstream realization.
//!
//! ## Integration contract
//!
//! - `ids.rs` owns `HdlirValueId`, `HdlirTypeId`, `HdlirOperationId`,
//!   `HdlirBlockId`, `HdlirPortId`, and `HdlirSignalId`.
//! - `ir/operations.rs` owns `HdlirOperation` and `OperationResult`.
//! - `ir/blocks.rs` owns block arguments and block structure.
//! - `ir/ports.rs` owns logical port declarations.
//! - `ir/connectivity.rs` owns connections and driver/sink relationships.
//! - `ir/processes.rs`, `ir/clocks.rs`, and `ir/memories.rs` own their
//!   respective behavioral and state-related contracts.
//! - `types/` owns type meaning. This module stores type IDs but does not
//!   interpret them.
//! - `verify/structure.rs` must verify graph-wide definitions, references,
//!   dominance/scope rules where applicable, and use-def consistency.
//! - `verify/types.rs` must verify that stored type IDs exist and satisfy
//!   operation, port, and dialect type constraints.
//! - `builder/` must use checked insertion and preserve the defining-origin
//!   contract.
//! - Serialization must encode these records through an explicit,
//!   versioned format rather than serializing Rust memory layout.
//!
//! The derived use index is not authoritative graph state. It can be rebuilt
//! from operations and must be invalidated or rebuilt after graph mutation.
//!
//! ## Scalability
//!
//! This module sets no universal limit on the number of values or uses.
//! Collection sizes are constrained by available resources and the host's
//! representational limits. No finite implementation can materialize
//! literally infinite input.
//!
//! Value IDs are identities, not vector indices. The use index uses `usize`
//! for an operand's position in its owning in-memory operation because Rust
//! collections are indexed by `usize`. That position is not a persistent ID.
//!
//! The standard library's ordered maps provide deterministic iteration.
//! Their allocation behavior is governed by the standard library and host;
//! this module does not claim to turn allocation failure into a recoverable
//! error on every platform.
//!
//! ## Failure behavior
//!
//! Duplicate value IDs, inconsistent table keys, invalid local definitions,
//! missing operand definitions, and mismatched operation-result definitions
//! return structured HDLIR errors. Repeated operands are valid and produce
//! repeated use records at their distinct operand positions.
//!
//! ## Freeze criteria
//!
//! - Value identities and types have one authoritative definition.
//! - Operation result declarations agree with the value table.
//! - Persistent signal/state concepts are not collapsed into ordinary SSA.
//! - Use indices can be rebuilt deterministically.
//! - Invalid references and duplicate definitions fail explicitly.
//! - Focused and whole-graph tests pass with the actual repository modules.
//! - Formatting, compilation, tests, and Clippy pass.

use std::collections::BTreeMap;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirBlockId, HdlirOperationId, HdlirPortId, HdlirSignalId, HdlirTypeId,
    HdlirValueId,
};
use crate::hdl::hdlir::ir::operations::HdlirOperation;

/// The semantic category of a value stored in HDLIR.
///
/// This classification provides a stable distinction between transient
/// computation and persistent hardware concepts. It does not replace the
/// registered type, operation, process, connectivity, clock, or memory
/// contracts that determine a value's exact behavior.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ValueSemantics {
    /// A transient value produced by combinational computation.
    ///
    /// This does not, by itself, prove that the value is represented in SSA
    /// form or that it is eligible for a particular optimization.
    Combinational,

    /// A value associated with a logical module or interface boundary.
    ///
    /// Physical pins and package connections are outside this contract.
    Interface,

    /// A persistent logical hardware signal.
    ///
    /// Driver/sink and temporal behavior are defined by other HDLIR modules.
    Signal,

    /// A value representing or referring to stateful hardware.
    ///
    /// This does not specify a physical register or a particular technology.
    State,

    /// A reference to abstract memory or a memory-related resource.
    ///
    /// Memory access and collision semantics belong to the memory contract.
    MemoryReference,

    /// An extension-defined value category.
    ///
    /// The extension name is preserved. Consumers must not infer semantics
    /// from the name alone without a compatible registered definition.
    Extension {
        /// Canonical name of the extension defining this category.
        name: String,
    },
}

impl ValueSemantics {
    /// Construct an extension-defined category.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty or contains only whitespace.
    pub fn extension(name: impl Into<String>) -> HdlirResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "value.semantics.extension",
                "extension name must not be empty or whitespace-only",
            ));
        }

        Ok(Self::Extension { name })
    }

    /// Validate local invariants for this category.
    ///
    /// # Errors
    ///
    /// Returns an error if an extension name is invalid.
    pub fn validate(&self) -> HdlirResult<()> {
        if let Self::Extension { name } = self {
            if name.trim().is_empty() {
                return Err(HdlirError::invalid_invariant(
                    "value.semantics.extension",
                    "extension name is empty or whitespace-only",
                ));
            }
        }

        Ok(())
    }
}

/// The origin that defines a value in the HDLIR graph.
///
/// An origin describes where the value is defined, not the value's temporal
/// semantics. The corresponding owning subsystem remains responsible for
/// checking that the referenced operation, block, port, or signal exists.
///
/// An operation result is also declared in `OperationResult` in
/// `ir/operations.rs`. The verifier must keep that declaration consistent
/// with this graph-level record.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ValueDefinition {
    /// A result defined by an operation.
    OperationResult {
        /// The operation that defines the result.
        operation: HdlirOperationId,
    },

    /// A block argument defined at a block boundary.
    BlockArgument {
        /// The block that owns the argument.
        block: HdlirBlockId,
        /// Zero-based argument position within that block.
        ///
        /// This is a local sequence position, not an identity or a hardware
        /// resource number.
        position: u64,
    },

    /// A value exposed by a logical module port.
    ModulePort {
        /// The logical port that defines the interface value.
        port: HdlirPortId,
    },

    /// A value associated with a persistent logical signal declaration.
    Signal {
        /// The logical signal declaration.
        signal: HdlirSignalId,
    },

    /// An externally supplied or imported value.
    ///
    /// This origin is appropriate only when the enclosing graph contract
    /// defines how the external value enters the design. It does not imply
    /// a physical pin or a runtime handle.
    External {
        /// Stable logical source description within the enclosing artifact.
        source: String,
    },

    /// An extension-defined origin.
    ///
    /// The extension owns the meaning and any additional consistency rules.
    Extension {
        /// Canonical extension name.
        name: String,
        /// Opaque extension-defined identity or provenance payload.
        payload: Vec<u8>,
    },
}

impl ValueDefinition {
    /// Construct an externally supplied value origin.
    ///
    /// # Errors
    ///
    /// Returns an error if `source` is empty or contains only whitespace.
    pub fn external(source: impl Into<String>) -> HdlirResult<Self> {
        let source = source.into();

        if source.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "value.definition.external",
                "external source must not be empty or whitespace-only",
            ));
        }

        Ok(Self::External { source })
    }

    /// Construct an extension-defined origin.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is empty or contains only whitespace.
    pub fn extension(
        name: impl Into<String>,
        payload: Vec<u8>,
    ) -> HdlirResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "value.definition.extension",
                "extension name must not be empty or whitespace-only",
            ));
        }

        Ok(Self::Extension { name, payload })
    }

    /// Validate this origin's local invariants.
    ///
    /// This does not check referenced entities against a design graph.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid external source or extension name.
    pub fn validate(&self) -> HdlirResult<()> {
        match self {
            Self::External { source } if source.trim().is_empty() => {
                Err(HdlirError::invalid_invariant(
                    "value.definition.external",
                    "external source is empty or whitespace-only",
                ))
            }
            Self::Extension { name, .. } if name.trim().is_empty() => {
                Err(HdlirError::invalid_invariant(
                    "value.definition.extension",
                    "extension name is empty or whitespace-only",
                ))
            }
            _ => Ok(()),
        }
    }
}

/// A graph-level record for one HDLIR value.
///
/// The record stores typed identity and definition metadata. It deliberately
/// does not copy the referenced type's structure: `HdlirTypeId` and the
/// canonical type registry remain authoritative for type meaning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueRecord {
    id: HdlirValueId,
    type_id: HdlirTypeId,
    semantics: ValueSemantics,
    definition: ValueDefinition,
}

impl ValueRecord {
    /// Construct a value record with locally validated metadata.
    ///
    /// This constructor does not prove that the type or defining entity exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the semantic category or definition is malformed.
    pub fn new(
        id: HdlirValueId,
        type_id: HdlirTypeId,
        semantics: ValueSemantics,
        definition: ValueDefinition,
    ) -> HdlirResult<Self> {
        semantics.validate()?;
        definition.validate()?;

        Ok(Self {
            id,
            type_id,
            semantics,
            definition,
        })
    }

    /// Return the value's typed identity.
    #[must_use]
    pub const fn id(&self) -> HdlirValueId {
        self.id
    }

    /// Return the type identity associated with this value.
    #[must_use]
    pub const fn type_id(&self) -> HdlirTypeId {
        self.type_id
    }

    /// Return the semantic category.
    #[must_use]
    pub const fn semantics(&self) -> &ValueSemantics {
        &self.semantics
    }

    /// Return the value's defining origin.
    #[must_use]
    pub const fn definition(&self) -> &ValueDefinition {
        &self.definition
    }

    /// Validate the record's local invariants.
    ///
    /// This method deliberately does not look up types or definitions.
    ///
    /// # Errors
    ///
    /// Returns an error if the semantic category or origin is malformed.
    pub fn validate(&self) -> HdlirResult<()> {
        self.semantics.validate()?;
        self.definition.validate()
    }
}

/// A deterministic table of graph-level HDLIR value definitions.
///
/// Values are keyed by their typed IDs, never by assuming that an ID is a
/// dense integer index. The table does not own operations, blocks, ports,
/// signals, or type definitions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValueTable {
    records: BTreeMap<HdlirValueId, ValueRecord>,
}

impl ValueTable {
    /// Create an empty value table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of value records currently stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Return whether the table contains no value records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Insert a value record.
    ///
    /// The record is inserted only if its identity is not already present.
    /// Duplicate insertion leaves the table unchanged.
    ///
    /// # Errors
    ///
    /// Returns a structured duplicate-entity error if the ID already exists,
    /// or a validation error if the record has invalid local metadata.
    pub fn insert(&mut self, record: ValueRecord) -> HdlirResult<()> {
        record.validate()?;

        if self.records.contains_key(&record.id()) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR value",
                identity: record.id().to_string(),
                scope: None,
            });
        }

        self.records.insert(record.id(), record);
        Ok(())
    }

    /// Look up a value record by its typed identity.
    #[must_use]
    pub fn get(&self, id: HdlirValueId) -> Option<&ValueRecord> {
        self.records.get(&id)
    }

    /// Return whether a value identity is present.
    #[must_use]
    pub fn contains(&self, id: HdlirValueId) -> bool {
        self.records.contains_key(&id)
    }

    /// Remove and return a value record.
    ///
    /// Removing a record does not rewrite operations that use it. The caller
    /// must re-run graph verification before treating the graph as valid.
    pub fn remove(&mut self, id: HdlirValueId) -> Option<ValueRecord> {
        self.records.remove(&id)
    }

    /// Iterate over records in ascending typed-ID order.
    ///
    /// This order is deterministic but is not source declaration order.
    pub fn iter(&self) -> impl Iterator<Item = (&HdlirValueId, &ValueRecord)> {
        self.records.iter()
    }

    /// Iterate over value records in ascending typed-ID order.
    pub fn values(&self) -> impl Iterator<Item = &ValueRecord> {
        self.records.values()
    }

    /// Validate key/record identity agreement and local record invariants.
    ///
    /// # Errors
    ///
    /// Returns an error if a record's map key differs from its embedded ID,
    /// or if a record has invalid local metadata.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        for (key, record) in &self.records {
            if *key != record.id() {
                return Err(HdlirError::invalid_invariant(
                    "value_table.key_identity",
                    format!(
                        "table key {key} does not match record identity {}",
                        record.id()
                    ),
                ));
            }

            record.validate()?;
        }

        Ok(())
    }

    /// Verify that operation operands refer to values in this table and that
    /// every operation result agrees with its graph-level value record.
    ///
    /// This checks value-table/operation consistency only. It does not prove
    /// that types exist, that operations are well typed, or that block scope,
    /// dominance, region ownership, or connectivity rules hold.
    ///
    /// # Errors
    ///
    /// Returns an error for a missing operand definition, duplicate result
    /// definition across operations, missing result record, or mismatched
    /// result origin/type.
    pub fn validate_operations<'a>(
        &self,
        operations: impl IntoIterator<Item = &'a HdlirOperation>,
    ) -> HdlirResult<()> {
        self.validate_structure()?;

        let mut seen_results: BTreeMap<HdlirValueId, HdlirOperationId> =
            BTreeMap::new();

        for operation in operations {
            operation.validate_structure()?;

            for operand in operation.operands() {
                if !self.contains(*operand) {
                    return Err(HdlirError::MissingEntity {
                        entity_kind: "HDLIR operand value",
                        identity: operand.to_string(),
                        scope: Some(operation.id().to_string()),
                    });
                }
            }

            for result in operation.results() {
                if let Some(previous_operation) =
                    seen_results.insert(result.id(), operation.id())
                {
                    return Err(HdlirError::DuplicateEntity {
                        entity_kind: "HDLIR operation result definition",
                        identity: result.id().to_string(),
                        scope: Some(format!(
                            "operations {previous_operation} and {}",
                            operation.id()
                        )),
                    });
                }

                let record = self.get(result.id()).ok_or_else(|| {
                    HdlirError::MissingEntity {
                        entity_kind: "HDLIR operation result record",
                        identity: result.id().to_string(),
                        scope: Some(operation.id().to_string()),
                    }
                })?;

                if record.type_id() != result.type_id() {
                    return Err(HdlirError::invalid_invariant(
                        "value_table.operation_result_type",
                        format!(
                            "operation {} declares result {} with type {}, \
                             but its value record uses type {}",
                            operation.id(),
                            result.id(),
                            result.type_id(),
                            record.type_id()
                        ),
                    ));
                }

                match record.definition() {
                    ValueDefinition::OperationResult {
                        operation: defining_operation,
                    } if *defining_operation == operation.id() => {}
                    other => {
                        return Err(HdlirError::invalid_invariant(
                            "value_table.operation_result_origin",
                            format!(
                                "result {} of operation {} has incompatible \
                                 value origin {other:?}",
                                result.id(),
                                operation.id()
                            ),
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

/// One use of a value as an operation operand.
///
/// The operand position is local to the operation and is not a persistent
/// identity. Repeated operands produce separate use records.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValueUse {
    operation: HdlirOperationId,
    operand_index: usize,
}

impl ValueUse {
    /// Construct a use record.
    #[must_use]
    pub const fn new(operation: HdlirOperationId, operand_index: usize) -> Self {
        Self {
            operation,
            operand_index,
        }
    }

    /// Return the operation containing this use.
    #[must_use]
    pub const fn operation(self) -> HdlirOperationId {
        self.operation
    }

    /// Return the zero-based operand position within the operation.
    #[must_use]
    pub const fn operand_index(self) -> usize {
        self.operand_index
    }
}

/// A derived, deterministic index from value IDs to operation operand uses.
///
/// This index is a cache of graph relationships, not an authoritative
/// definition of the graph. Rebuild it after operation operands change.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValueUseIndex {
    uses: BTreeMap<HdlirValueId, Vec<ValueUse>>,
}

impl ValueUseIndex {
    /// Create an empty use index.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build an index from an iterable of operations.
    ///
    /// The index preserves each operation's operand positions and stores
    /// value keys in ascending typed-ID order. If an operand appears more
    /// than once in an operation, each occurrence is retained.
    ///
    /// This function indexes references as found; it does not check that the
    /// referenced values exist. Call `ValueTable::validate_operations` for
    /// that cross-check.
    #[must_use]
    pub fn from_operations<'a>(
        operations: impl IntoIterator<Item = &'a HdlirOperation>,
    ) -> Self {
        let mut index = Self::new();

        for operation in operations {
            for (operand_index, value_id) in operation.operands().iter().enumerate() {
                index
                    .uses
                    .entry(*value_id)
                    .or_default()
                    .push(ValueUse::new(operation.id(), operand_index));
            }
        }

        // Sort use records so the index is deterministic even if the caller
        // supplied operations in a different iteration order.
        for uses in index.uses.values_mut() {
            uses.sort_unstable();
        }

        index
    }

    /// Return the use records for a value, or an empty slice if it is unused.
    #[must_use]
    pub fn uses_of(&self, value: HdlirValueId) -> &[ValueUse] {
        self.uses.get(&value).map_or(&[], Vec::as_slice)
    }

    /// Return the number of indexed distinct value IDs.
    #[must_use]
    pub fn distinct_value_count(&self) -> usize {
        self.uses.len()
    }

    /// Return the total number of operand uses represented by the index.
    ///
    /// The sum is checked. `None` means the total cannot be represented by
    /// `usize`; it is not silently wrapped.
    #[must_use]
    pub fn total_use_count(&self) -> Option<usize> {
        self.uses.values().try_fold(0_usize, |total, uses| {
            total.checked_add(uses.len())
        })
    }

    /// Iterate over indexed value IDs and their ordered use records.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&HdlirValueId, &[ValueUse])> {
        self.uses
            .iter()
            .map(|(value, uses)| (value, uses.as_slice()))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ValueDefinition, ValueRecord, ValueSemantics, ValueTable, ValueUse,
        ValueUseIndex,
    };
    use crate::hdl::hdlir::ids::{
        HdlirBlockId, HdlirOperationId, HdlirTypeId, HdlirValueId,
    };

    fn value_id(raw: u128) -> HdlirValueId {
        HdlirValueId::new(raw).expect("test IDs must be non-zero")
    }

    fn type_id(raw: u128) -> HdlirTypeId {
        HdlirTypeId::new(raw).expect("test IDs must be non-zero")
    }

    fn operation_id(raw: u128) -> HdlirOperationId {
        HdlirOperationId::new(raw).expect("test IDs must be non-zero")
    }

    fn record(raw: u128) -> ValueRecord {
        ValueRecord::new(
            value_id(raw),
            type_id(1),
            ValueSemantics::Combinational,
            ValueDefinition::BlockArgument {
                block: HdlirBlockId::new(1).expect("non-zero block ID"),
                position: raw
                    .try_into()
                    .expect("test value ID fits a u64 position"),
            },
        )
        .expect("valid test value record")
    }

    #[test]
    fn value_record_retains_identity_type_semantics_and_origin() {
        let value = record(7);

        assert_eq!(value.id(), value_id(7));
        assert_eq!(value.type_id(), type_id(1));
        assert_eq!(*value.semantics(), ValueSemantics::Combinational);
        assert!(matches!(
            value.definition(),
            ValueDefinition::BlockArgument {
                block,
                position: 7
            } if *block == HdlirBlockId::new(1).unwrap()
        ));
    }

    #[test]
    fn table_inserts_and_looks_up_values_deterministically() {
        let mut table = ValueTable::new();

        table.insert(record(9)).unwrap();
        table.insert(record(2)).unwrap();

        let ids: Vec<u128> = table
            .iter()
            .map(|(id, _)| id.get())
            .collect();

        assert_eq!(ids, vec![2, 9]);
        assert_eq!(table.len(), 2);
        assert!(table.contains(value_id(2)));
        assert_eq!(table.get(value_id(9)).unwrap().id(), value_id(9));
        table.validate_structure().unwrap();
    }

    #[test]
    fn duplicate_value_insertion_fails_without_replacing_original() {
        let mut table = ValueTable::new();
        table.insert(record(1)).unwrap();

        let error = table.insert(record(1)).unwrap_err();

        assert!(error.to_string().contains("1"));
        assert_eq!(table.len(), 1);
        assert_eq!(table.get(value_id(1)).unwrap().id(), value_id(1));
    }

    #[test]
    fn removal_returns_record_and_does_not_affect_other_values() {
        let mut table = ValueTable::new();
        table.insert(record(1)).unwrap();
        table.insert(record(2)).unwrap();

        assert_eq!(table.remove(value_id(1)).unwrap().id(), value_id(1));
        assert!(!table.contains(value_id(1)));
        assert!(table.contains(value_id(2)));
    }

    #[test]
    fn invalid_extension_names_are_rejected() {
        assert!(ValueSemantics::extension("   ").is_err());
        assert!(ValueDefinition::extension("", Vec::new()).is_err());
        assert!(ValueDefinition::external(" ").is_err());
    }

    #[test]
    fn extension_payload_is_preserved_without_interpretation() {
        let definition =
            ValueDefinition::extension("vendor.signal", vec![1, 2, 3]).unwrap();

        assert_eq!(
            definition,
            ValueDefinition::Extension {
                name: "vendor.signal".to_owned(),
                payload: vec![1, 2, 3],
            }
        );
    }

    #[test]
    fn value_use_retains_operation_and_operand_position() {
        let usage = ValueUse::new(operation_id(4), 2);

        assert_eq!(usage.operation(), operation_id(4));
        assert_eq!(usage.operand_index(), 2);
    }

    #[test]
    fn empty_table_is_valid_and_empty() {
        let table = ValueTable::new();

        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
        table.validate_structure().unwrap();
    }

    #[test]
    fn use_index_is_empty_without_operations() {
        let index = ValueUseIndex::new();

        assert_eq!(index.distinct_value_count(), 0);
        assert_eq!(index.total_use_count(), Some(0));
        assert!(index.uses_of(value_id(1)).is_empty());
    }
}
