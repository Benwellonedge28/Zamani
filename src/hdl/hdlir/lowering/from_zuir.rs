
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — ZUIR-to-HDLIR Lowering
//!
//! Path: `src/hdl/hdlir/lowering/from_zuir.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Convert a pre-verified batch of canonical ZUIR operations into HDLIR
//! operations while preserving operation order, operand relationships,
//! result types, explicitly declared result semantics, and source identity
//! mappings.
//!
//! This adapter is deliberately conservative. A ZUIR operation is lowered
//! only when the caller supplies an explicit destination operation mapping
//! and the source operation contains no attributes, effects, nested regions,
//! or opaque extension payloads. Those constructs require dedicated,
//! semantics-preserving adapters and are rejected here.
//!
//! This file lowers an operation batch, not a complete ZUIR program.
//! ZUIR's program, module, function, region, block, and operation containers
//! have separate ownership and references. A complete-program adapter must
//! first receive an authoritative graph snapshot that resolves those
//! references and supplies the relevant type and block-argument information.
//!
//! ## Architectural authority
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/program.rs`
//! - `src/zuir/ir/module.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/operand.rs`
//! - `src/zuir/ir/result.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/verify/`
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/ir/values.rs`
//!
//! The grammar defines syntax. The frontend semantic model defines
//! source-level meaning. ZUIR represents universal computational meaning.
//! HDLIR owns hardware-domain representation. Target feasibility and
//! physical realization remain downstream.
//!
//! ## Preconditions
//!
//! - The source operations have passed the canonical ZUIR verifier.
//! - The supplied batch contains the operations intended for this conversion.
//! - The caller supplies the source-to-HDLIR type mapping.
//! - The caller supplies mappings for every non-result operand, such as
//!   verified block arguments or values produced outside this batch.
//! - Each source operation name has an explicit, reviewed destination
//!   operation mapping.
//! - The caller registers and verifies destination operation definitions
//!   against the actual HDLIR schema registry.
//!
//! This adapter does not claim to establish whole-program SSA dominance,
//! control-flow validity, dialect registration, or hardware feasibility.
//!
//! ## Losslessness contract
//!
//! No source attribute, effect, region, or opaque payload is discarded.
//! The current implementation rejects operations carrying any of those
//! features. Adding support requires an explicit mapping for each feature
//! and corresponding conformance tests.
//!
//! A destination operation name is never guessed from the source name.
//! Destination result semantics are explicitly supplied by the caller.
//! Type identities are mapped only through the caller's checked type map.
//!
//! ## Scalability
//!
//! There are no fixed universal operation, operand, result, width, or
//! hardware-capacity ceilings. Optional operation and result budgets are
//! caller-controlled operational safeguards.
//!
//! All count conversions and result-count additions are checked. ID
//! exhaustion is reported without wrapping or reusing an ID. Collections
//! grow subject to the available memory, address space, and Rust's
//! representational limits. No finite process promises literal infinity.
//!
//! ## Transaction contract
//!
//! The adapter builds its returned operations and value table locally.
//! A failure returns no partial output. Caller-owned ID allocators may
//! have advanced before a later error; allocated IDs must not be recycled.
//!
//! ## Integration
//!
//! Register this module from `src/hdl/hdlir/lowering/mod.rs` and register
//! the lowering module from `src/hdl/hdlir/mod.rs` only after those module
//! declarations have been reconciled with the actual checkout.
//!
//! This adapter uses the existing HDLIR operation and value definitions.
//! It does not create a competing graph, type system, schema registry,
//! parser, or semantic model.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::hdl::hdlir::ids::{
    HdlirIdAllocator,
    HdlirIdSpaceExhausted,
    HdlirOperationId,
    HdlirTypeId,
    HdlirValueId,
};
use crate::hdl::hdlir::ir::operations::{
    HdlirOperation,
    OperationResult,
};
use crate::hdl::hdlir::ir::values::{
    ValueDefinition,
    ValueRecord,
    ValueSemantics,
    ValueTable,
};
use crate::hdl::hdlir::schema::registry::QualifiedName as HdlirQualifiedName;
use crate::hdl::hdlir::version::HdlirVersion;
use crate::zuir::ids::{
    OperationId as ZuirOperationId,
    TypeId as ZuirTypeId,
    ValueId as ZuirValueId,
};
use crate::zuir::ir::operation::ZuirOperation;

/// Caller-controlled operational budgets for ZUIR-to-HDLIR lowering.
///
/// `None` imposes no additional limit for the corresponding count. It does
/// not promise unlimited memory, time, or system resources.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ZuirLoweringOptions {
    /// Maximum number of source operations processed by this call.
    pub max_operations: Option<u64>,

    /// Maximum total number of operation results processed by this call.
    pub max_results: Option<u64>,
}

impl ZuirLoweringOptions {
    /// Construct lowering options with explicit operational budgets.
    #[must_use]
    pub const fn new(
        max_operations: Option<u64>,
        max_results: Option<u64>,
    ) -> Self {
        Self {
            max_operations,
            max_results,
        }
    }
}

/// Explicit destination mapping for one source operation name.
///
/// The mapping is a reviewed semantic decision, not an automatic naming
/// convention. `result_semantics` describes the HDLIR category to assign
/// to each result produced by this mapping. If an operation has results
/// requiring different semantic categories, use a future richer mapping
/// contract rather than this uniform-result mapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationMapping {
    target_name: HdlirQualifiedName,
    schema_version: HdlirVersion,
    result_semantics: ValueSemantics,
}

impl OperationMapping {
    /// Create a destination mapping using a validated HDLIR qualified name.
    ///
    /// The caller must ensure that the name and schema version refer to an
    /// operation definition registered in the destination schema registry.
    pub fn new(
        target_name: impl Into<String>,
        schema_version: HdlirVersion,
        result_semantics: ValueSemantics,
    ) -> Result<Self, HdlirLoweringError> {
        let target_name = HdlirQualifiedName::parse(target_name)
            .map_err(|error| HdlirLoweringError::InvalidMapping {
                name: target_name.into(),
                reason: error.to_string(),
            })?;

        result_semantics
            .validate()
            .map_err(|error| HdlirLoweringError::InvalidMapping {
                name: target_name.as_str().to_owned(),
                reason: error.to_string(),
            })?;

        Ok(Self {
            target_name,
            schema_version,
            result_semantics,
        })
    }

    /// Return the destination operation's qualified schema name.
    #[must_use]
    pub fn target_name(&self) -> &HdlirQualifiedName {
        &self.target_name
    }

    /// Return the destination operation schema version.
    #[must_use]
    pub const fn schema_version(&self) -> HdlirVersion {
        self.schema_version
    }

    /// Return the semantic category assigned to each mapped result.
    #[must_use]
    pub const fn result_semantics(&self) -> &ValueSemantics {
        &self.result_semantics
    }
}

/// Failure while converting a ZUIR operation batch into HDLIR.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlirLoweringError {
    /// The source operation count cannot be represented as `u64`.
    OperationCountUnrepresentable,

    /// The caller's operation budget was exceeded.
    OperationBudgetExceeded {
        /// Number of operations requested.
        requested: u64,
        /// Configured maximum.
        limit: u64,
    },

    /// The total result count cannot be represented as `u64`.
    ResultCountUnrepresentable,

    /// Adding a result count would overflow.
    ResultCountOverflow,

    /// The caller's result budget was exceeded.
    ResultBudgetExceeded {
        /// Number of results requested.
        requested: u64,
        /// Configured maximum.
        limit: u64,
    },

    /// A source operation failed local validation.
    InvalidSourceOperation {
        /// Source operation identity.
        id: ZuirOperationId,
        /// Validation explanation.
        reason: String,
    },

    /// No explicit mapping exists for a source operation name.
    MissingOperationMapping {
        /// Canonical source operation name.
        name: String,
    },

    /// A destination mapping is malformed.
    InvalidMapping {
        /// Name supplied to the mapping constructor.
        name: String,
        /// Validation explanation.
        reason: String,
    },

    /// The source operation contains semantics this adapter does not map.
    UnsupportedOperationFeatures {
        /// Source operation identity.
        id: ZuirOperationId,
        /// Features that would otherwise be lost.
        features: Vec<&'static str>,
    },

    /// A source result identity is duplicated or conflicts with an external
    /// source value identity.
    DuplicateSourceValue {
        /// Conflicting source value identity.
        id: ZuirValueId,
    },

    /// Two external source values map to the same HDLIR value identity.
    DuplicateExternalTargetValue {
        /// Conflicting HDLIR value identity.
        id: HdlirValueId,
    },

    /// An operand cannot be resolved to a mapped result or external value.
    UnmappedOperand {
        /// Source operation consuming the operand.
        operation: ZuirOperationId,
        /// Source value identity.
        value: ZuirValueId,
    },

    /// A source result's type has no caller-supplied destination mapping.
    MissingTypeMapping {
        /// Source operation defining the result.
        operation: ZuirOperationId,
        /// Source type identity.
        ty: ZuirTypeId,
    },

    /// The HDLIR operation identity allocator was exhausted.
    OperationIdentityExhausted,

    /// The HDLIR value identity allocator was exhausted.
    ValueIdentityExhausted,

    /// A constructed HDLIR operation or value record failed local validation.
    DestinationConstruction {
        /// Explanation from the canonical HDLIR constructor.
        reason: String,
    },
}

impl fmt::Display for HdlirLoweringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperationCountUnrepresentable => {
                f.write_str("ZUIR operation count cannot be represented as u64")
            }
            Self::OperationBudgetExceeded { requested, limit } => {
                write!(
                    f,
                    "ZUIR-to-HDLIR operation budget exceeded: \
                     requested {requested}, limit {limit}"
                )
            }
            Self::ResultCountUnrepresentable => {
                f.write_str("ZUIR result count cannot be represented as u64")
            }
            Self::ResultCountOverflow => {
                f.write_str("ZUIR result count overflowed u64")
            }
            Self::ResultBudgetExceeded { requested, limit } => {
                write!(
                    f,
                    "ZUIR-to-HDLIR result budget exceeded: \
                     requested {requested}, limit {limit}"
                )
            }
            Self::InvalidSourceOperation { id, reason } => {
                write!(f, "invalid ZUIR operation {id}: {reason}")
            }
            Self::MissingOperationMapping { name } => {
                write!(f, "no explicit HDLIR mapping exists for ZUIR operation `{name}`")
            }
            Self::InvalidMapping { name, reason } => {
                write!(f, "invalid HDLIR mapping for `{name}`: {reason}")
            }
            Self::UnsupportedOperationFeatures { id, features } => {
                write!(
                    f,
                    "ZUIR operation {id} has unsupported features: {}",
                    features.join(", ")
                )
            }
            Self::DuplicateSourceValue { id } => {
                write!(f, "ZUIR value identity {id} is defined or supplied more than once")
            }
            Self::DuplicateExternalTargetValue { id } => {
                write!(
                    f,
                    "multiple external ZUIR values map to HDLIR value identity {id}"
                )
            }
            Self::UnmappedOperand { operation, value } => {
                write!(
                    f,
                    "operand value {value} of ZUIR operation {operation} \
                     has no HDLIR value mapping"
                )
            }
            Self::MissingTypeMapping { operation, ty } => {
                write!(
                    f,
                    "result of ZUIR operation {operation} references \
                     unmapped type {ty}"
                )
            }
            Self::OperationIdentityExhausted => {
                f.write_str("HDLIR operation identity space exhausted")
            }
            Self::ValueIdentityExhausted => {
                f.write_str("HDLIR value identity space exhausted")
            }
            Self::DestinationConstruction { reason } => {
                write!(f, "HDLIR destination construction failed: {reason}")
            }
        }
    }
}

impl Error for HdlirLoweringError {}

impl From<HdlirIdSpaceExhausted> for HdlirLoweringError {
    fn from(_: HdlirIdSpaceExhausted) -> Self {
        Self::ValueIdentityExhausted
    }
}

/// Successfully lowered operation batch and its source-to-destination maps.
///
/// `operations` preserves the input batch's order. `values` contains the
/// definitions created by this batch; it does not contain external value
/// records owned by the caller's enclosing graph.
///
/// `source_to_value` includes both caller-supplied external mappings and
/// mappings for results created by this batch. The caller must merge the
/// returned operation and value records into the authoritative enclosing
/// HDLIR graph and run its full verifier before trusting the result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZuirLoweringOutput {
    /// HDLIR operations in the same order as the input batch.
    pub operations: Vec<HdlirOperation>,

    /// HDLIR value definitions created by operation results in this batch.
    pub values: ValueTable,

    /// Source operation identity to newly allocated HDLIR operation identity.
    pub source_to_operation: BTreeMap<ZuirOperationId, HdlirOperationId>,

    /// Source value identity to HDLIR value identity, including external
    /// mappings supplied by the caller.
    pub source_to_value: BTreeMap<ZuirValueId, HdlirValueId>,
}

/// Lower a pre-verified batch of ZUIR operations into HDLIR.
///
/// `operation_mappings` is keyed by the canonical source operation name
/// returned by `ZuirOperation::name().as_str()`.
///
/// `type_mappings` must contain each source result type used by this batch.
/// `external_values` supplies already-resolved mappings for operands whose
/// definitions are outside the batch, such as block arguments or values
/// produced by another batch.
///
/// The caller owns allocator scope and must ensure the supplied allocators
/// do not collide with IDs already present in the destination graph.
///
/// # Losslessness
///
/// This function rejects any operation with attributes, effects, nested
/// regions, or an opaque payload. It does not silently erase those features.
///
/// # Errors
///
/// Returns a structured error for invalid source operations, missing mappings,
/// unsupported features, unresolved operands, duplicate identities, budget
/// exhaustion, identifier exhaustion, or destination construction failures.
///
/// No partial output is returned on failure. The supplied ID allocators may
/// have advanced and must not recycle allocated identifiers.
///
/// # Verification boundary
///
/// This function is not a replacement for the canonical ZUIR full verifier
/// or the HDLIR full verifier. The source must already be verified, and the
/// enclosing HDLIR graph must be verified after merging this output.
pub fn from_zuir(
    operations: &[ZuirOperation],
    operation_mappings: &BTreeMap<String, OperationMapping>,
    type_mappings: &BTreeMap<ZuirTypeId, HdlirTypeId>,
    external_values: &BTreeMap<ZuirValueId, HdlirValueId>,
    operation_ids: &mut HdlirIdAllocator<HdlirOperationId>,
    value_ids: &mut HdlirIdAllocator<HdlirValueId>,
    options: ZuirLoweringOptions,
) -> Result<ZuirLoweringOutput, HdlirLoweringError> {
    let operation_count = u64::try_from(operations.len())
        .map_err(|_| HdlirLoweringError::OperationCountUnrepresentable)?;

    if let Some(limit) = options.max_operations {
        if operation_count > limit {
            return Err(HdlirLoweringError::OperationBudgetExceeded {
                requested: operation_count,
                limit,
            });
        }
    }

    // Preflight all source-local properties and mapping coverage before
    // allocating any destination IDs.
    let mut seen_operations = BTreeSet::new();
    let mut seen_source_values = BTreeSet::new();
    let mut result_count = 0_u64;

    for operation in operations {
        operation
            .validate()
            .map_err(|error| HdlirLoweringError::InvalidSourceOperation {
                id: operation.id(),
                reason: error.to_string(),
            })?;

        if !seen_operations.insert(operation.id()) {
            return Err(HdlirLoweringError::InvalidSourceOperation {
                id: operation.id(),
                reason: "operation identity occurs more than once in the batch".to_owned(),
            });
        }

        if !operation_mappings.contains_key(operation.name().as_str()) {
            return Err(HdlirLoweringError::MissingOperationMapping {
                name: operation.name().as_str().to_owned(),
            });
        }

        let mut unsupported = Vec::new();

        if !operation.attributes().is_empty() {
            unsupported.push("attributes");
        }
        if !operation.effects().is_empty() {
            unsupported.push("effects");
        }
        if !operation.regions().is_empty() {
            unsupported.push("nested regions");
        }
        if operation.opaque_payload().is_some() {
            unsupported.push("opaque extension payload");
        }

        if !unsupported.is_empty() {
            return Err(HdlirLoweringError::UnsupportedOperationFeatures {
                id: operation.id(),
                features: unsupported,
            });
        }

        let local_result_count = u64::try_from(operation.results().len())
            .map_err(|_| HdlirLoweringError::ResultCountUnrepresentable)?;

        result_count = result_count
            .checked_add(local_result_count)
            .ok_or(HdlirLoweringError::ResultCountOverflow)?;

        if let Some(limit) = options.max_results {
            if result_count > limit {
                return Err(HdlirLoweringError::ResultBudgetExceeded {
                    requested: result_count,
                    limit,
                });
            }
        }

        for result in operation.results() {
            if !seen_source_values.insert(result.value()) {
                return Err(HdlirLoweringError::DuplicateSourceValue {
                    id: result.value(),
                });
            }

            if external_values.contains_key(&result.value()) {
                return Err(HdlirLoweringError::DuplicateSourceValue {
                    id: result.value(),
                });
            }

            if !type_mappings.contains_key(&result.ty()) {
                return Err(HdlirLoweringError::MissingTypeMapping {
                    operation: operation.id(),
                    ty: result.ty(),
                });
            }
        }
    }

    // An external mapping must be injective in the destination value-ID
    // namespace. Otherwise two distinct source values would be conflated.
    let mut seen_external_targets = BTreeSet::new();

    for target_id in external_values.values().copied() {
        if !seen_external_targets.insert(target_id) {
            return Err(HdlirLoweringError::DuplicateExternalTargetValue {
                id: target_id,
            });
        }
    }

    // Allocate destination identities for all operations and results before
    // resolving operands. This permits references between operations in the
    // same batch without assuming that source IDs are dense or indexable.
    let mut source_to_operation = BTreeMap::new();
    let mut source_to_value = external_values.clone();

    for operation in operations {
        let target_id = operation_ids
            .allocate()
            .map_err(|_| HdlirLoweringError::OperationIdentityExhausted)?;

        source_to_operation.insert(operation.id(), target_id);
    }

    for operation in operations {
        for result in operation.results() {
            let target_id = value_ids
                .allocate()
                .map_err(|_| HdlirLoweringError::ValueIdentityExhausted)?;

            source_to_value.insert(result.value(), target_id);
        }
    }

    // Resolve every operand before constructing output objects. Missing
    // mappings are reported rather than replaced with fabricated values.
    for operation in operations {
        for operand in operation.operands() {
            if !source_to_value.contains_key(&operand.value()) {
                return Err(HdlirLoweringError::UnmappedOperand {
                    operation: operation.id(),
                    value: operand.value(),
                });
            }
        }
    }

    let mut output_operations = Vec::with_capacity(operations.len());
    let mut output_values = ValueTable::new();

    for source_operation in operations {
        let target_operation_id = *source_to_operation
            .get(&source_operation.id())
            .expect("destination operation ID was allocated for every source operation");

        let mapping = operation_mappings
            .get(source_operation.name().as_str())
            .expect("operation mapping was checked during preflight");

        let mut target_operands = Vec::with_capacity(source_operation.operands().len());

        for operand in source_operation.operands() {
            let target_value = *source_to_value
                .get(&operand.value())
                .ok_or(HdlirLoweringError::UnmappedOperand {
                    operation: source_operation.id(),
                    value: operand.value(),
                })?;

            target_operands.push(target_value);
        }

        let mut target_results = Vec::with_capacity(source_operation.results().len());

        for source_result in source_operation.results() {
            let target_value_id = *source_to_value
                .get(&source_result.value())
                .expect("destination result ID was allocated during preflight");

            let target_type_id = *type_mappings
                .get(&source_result.ty())
                .ok_or(HdlirLoweringError::MissingTypeMapping {
                    operation: source_operation.id(),
                    ty: source_result.ty(),
                })?;

            target_results.push(OperationResult::new(
                target_value_id,
                target_type_id,
            ));

            let record = ValueRecord::new(
                target_value_id,
                target_type_id,
                mapping.result_semantics.clone(),
                ValueDefinition::OperationResult {
                    operation: target_operation_id,
                },
            )
            .map_err(|error| HdlirLoweringError::DestinationConstruction {
                reason: error.to_string(),
            })?;

            output_values
                .insert(record)
                .map_err(|error| HdlirLoweringError::DestinationConstruction {
                    reason: error.to_string(),
                })?;
        }

        let target_operation = HdlirOperation::new(
            target_operation_id,
            mapping.target_name.clone(),
            mapping.schema_version,
            target_operands,
            target_results,
            BTreeMap::new(),
            Vec::new(),
        )
        .map_err(|error| HdlirLoweringError::DestinationConstruction {
            reason: error.to_string(),
        })?;

        target_operation
            .validate_structure()
            .map_err(|error| HdlirLoweringError::DestinationConstruction {
                reason: error.to_string(),
            })?;

        output_operations.push(target_operation);
    }

    output_values
        .validate_structure()
        .map_err(|error| HdlirLoweringError::DestinationConstruction {
            reason: error.to_string(),
        })?;

    Ok(ZuirLoweringOutput {
        operations: output_operations,
        values: output_values,
        source_to_operation,
        source_to_value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_batch_lowers_without_inventing_operations() {
        let mut operation_ids =
            HdlirIdAllocator::<HdlirOperationId>::with_start(1)
                .expect("one is a valid allocator start");

        let mut value_ids =
            HdlirIdAllocator::<HdlirValueId>::with_start(1)
                .expect("one is a valid allocator start");

        let output = from_zuir(
            &[],
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &mut operation_ids,
            &mut value_ids,
            ZuirLoweringOptions::default(),
        )
        .expect("an empty batch is a valid empty conversion");

        assert!(output.operations.is_empty());
        assert!(output.values.is_empty());
        assert!(output.source_to_operation.is_empty());
        assert!(output.source_to_value.is_empty());
    }

    #[test]
    fn zero_budgets_accept_an_empty_batch() {
        let mut operation_ids =
            HdlirIdAllocator::<HdlirOperationId>::with_start(1)
                .expect("one is a valid allocator start");

        let mut value_ids =
            HdlirIdAllocator::<HdlirValueId>::with_start(1)
                .expect("one is a valid allocator start");

        let options = ZuirLoweringOptions::new(Some(0), Some(0));

        let output = from_zuir(
            &[],
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &mut operation_ids,
            &mut value_ids,
            options,
        )
        .expect("zero operations and results fit zero budgets");

        assert!(output.operations.is_empty());
        assert!(output.values.is_empty());
    }
}
