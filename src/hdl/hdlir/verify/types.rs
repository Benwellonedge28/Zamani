
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Type Verification
//!
//! Path: `src/hdl/hdlir/verify/types.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed verification boundary; requires integration and tests
//! against the checked-out repository before being declared production-ready.
//!
//! ## Purpose
//!
//! Verify that HDLIR values reference valid types, operation operands refer
//! to existing values, operation results agree with their value records,
//! and operation-specific type rules are satisfied.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/ir/values.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/hdl/hdlir/types/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! Source syntax and source-level semantic truth belong to the canonical
//! frontend. ZUIR owns the universal computational representation. HDLIR
//! owns hardware-domain representation and verification.
//!
//! ## Ownership
//!
//! This module owns graph-level type-verification orchestration and its
//! verification report. The supplied type context owns concrete type
//! resolution and operation-specific type semantics.
//!
//! This module does not own type definitions, source name resolution,
//! physical resource allocation, synthesis, placement, routing, or timing
//! closure.
//!
//! ## Scalability
//!
//! No universal maximum width, operand count, result count, number of types,
//! number of operations, or hardware-resource count is introduced.
//!
//! An optional work budget limits verifier effort, not legal HDL semantics.
//! Counter arithmetic is checked. Budget exhaustion is an error and never
//! produces a successful verification report.
//!
//! ## Determinism
//!
//! Operations and their operands/results are processed in their supplied
//! order. Type identities are deduplicated using an ordered set. This module
//! does not depend on pointer addresses, hash iteration order, or target
//! hardware.
//!
//! ## Integration
//!
//! `verify/mod.rs` must register this module. The HDLIR composition root must
//! register `verify` once the parent module is valid Rust.
//!
//! The type context adapter must resolve `HdlirTypeId` values using the
//! repository's canonical HDLIR type/schema ownership and enforce each
//! operation's registered type constraints. It must not invent permissive
//! defaults for unknown operations or types.
//!
//! ## Completion criteria
//!
//! The file is complete only when its public contracts are wired to the
//! actual type registry, all required operation rules are enforced, tests
//! cover valid and invalid graphs, and repository-wide checks pass.

use std::collections::BTreeSet;

use crate::hdl::hdlir::errors::{
    HdlirBudgetKind,
    HdlirError,
    HdlirResult,
};
use crate::hdl::hdlir::ids::{
    HdlirOperationId,
    HdlirTypeId,
    HdlirValueId,
};
use crate::hdl::hdlir::ir::operations::HdlirOperation;
use crate::hdl::hdlir::ir::values::{
    ValueDefinition,
    ValueRecord,
    ValueTable,
};

/// Explicit operational budget for HDLIR type verification.
///
/// `max_items: None` means that this API imposes no configured item-count
/// budget. It does not guarantee unlimited memory, execution time, or
/// successful processing of arbitrarily large inputs.
///
/// Each visited value record, operation, operand, and result consumes one
/// unit of work. Type-context implementations may perform additional work;
/// they remain responsible for applying any budgets they require internally.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeVerificationBudget {
    /// Maximum number of verifier work items, or `None` for no configured
    /// item-count limit.
    pub max_items: Option<u128>,
}

impl TypeVerificationBudget {
    /// Construct a budget with an explicit maximum work-item count.
    #[must_use]
    pub const fn new(max_items: u128) -> Self {
        Self {
            max_items: Some(max_items),
        }
    }

    /// Construct a budget without a configured work-item limit.
    ///
    /// Resource availability and representational limits still apply.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self { max_items: None }
    }
}

impl Default for TypeVerificationBudget {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Canonical type facts and operation rules required by this verifier.
///
/// Implement this adapter using the actual HDLIR type/schema definitions.
/// This trait deliberately avoids defining another type system or assuming
/// that every registered type is a logic vector.
///
/// `validate_type` must fail when the identity cannot be resolved or its
/// definition is invalid. It must not silently accept unknown types.
///
/// `verify_operation_types` must apply the registered operation's complete
/// type contract, including operand/result constraints, signedness, widths,
/// aggregate relationships, and any other type properties relevant to that
/// operation. Symbolic uncertainty must be reported or retained according
/// to the relevant contract; it must not be treated as proven compatibility.
///
/// Implementations must not claim successful checking when a required
/// operation rule is missing. Return an explicit HDLIR error instead.
pub trait HdlirTypeContext {
    /// Resolve and validate an HDLIR type identity.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown, incompatible, malformed, or otherwise
    /// invalid type definition.
    fn validate_type(&self, type_id: HdlirTypeId) -> HdlirResult<()>;

    /// Verify an operation's type-dependent contract.
    ///
    /// `operand_types` follows the operation's operand order.
    /// `result_types` follows its declared result order.
    ///
    /// The caller guarantees that every listed value exists and that every
    /// type identity has passed `validate_type`.
    ///
    /// # Errors
    ///
    /// Returns an error when the operation's type contract is violated,
    /// cannot be resolved, or is unsupported by the supplied context.
    fn verify_operation_types(
        &self,
        operation: &HdlirOperation,
        operand_types: &[HdlirTypeId],
        result_types: &[HdlirTypeId],
    ) -> HdlirResult<()>;
}

/// Counts of entities successfully checked by type verification.
///
/// Counts describe completed work, not hardware capacity or the number of
/// resources available on a target.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TypeVerificationReport {
    /// Number of value records visited and validated.
    pub values_checked: u128,

    /// Number of operation instances visited.
    pub operations_checked: u128,

    /// Number of operand references checked.
    pub operands_checked: u128,

    /// Number of operation results checked.
    pub results_checked: u128,

    /// Number of distinct type identities validated.
    pub distinct_types_checked: u128,

    /// Number of work items consumed by this verifier.
    pub work_items: u128,
}

impl TypeVerificationReport {
    /// Construct an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values_checked: 0,
            operations_checked: 0,
            operands_checked: 0,
            results_checked: 0,
            distinct_types_checked: 0,
            work_items: 0,
        }
    }
}

/// Verify type integrity across an HDLIR value table and operation sequence.
///
/// This function verifies:
///
/// - Every value record's local invariants.
/// - Every value's type identity.
/// - Every operation operand's value reference and type identity.
/// - Every operation result's value record, type identity, and defining
///   operation relationship.
/// - That result values do not claim a missing defining operation.
/// - The registered type-dependent contract of every operation.
///
/// This is type verification, not complete HDLIR verification. Structural
/// ownership, control flow, connectivity, clocks, memory behavior, and other
/// invariants must be checked by their corresponding verifier stages.
///
/// # Errors
///
/// Returns the first detected error. An error means verification did not
/// complete successfully; callers must not interpret it as a successful
/// report or as proof that every other invariant is invalid.
///
/// A work-budget exhaustion is reported explicitly. It never returns a
/// successful partial report.
///
/// # Determinism
///
/// Given the same ordered operations, value table, type context, and budget,
/// the verifier follows the same traversal order. Deterministic diagnostics
/// additionally require the supplied type context to be deterministic.
pub fn verify_types<C: HdlirTypeContext>(
    operations: &[HdlirOperation],
    values: &ValueTable,
    type_context: &C,
    budget: TypeVerificationBudget,
) -> HdlirResult<TypeVerificationReport> {
    let mut verifier = TypeVerifier {
        type_context,
        budget,
        report: TypeVerificationReport::new(),
        checked_types: BTreeSet::new(),
    };

    verifier.verify_values(values)?;
    verifier.verify_operations(operations, values)?;
    verifier.verify_value_definitions(operations, values)?;

    Ok(verifier.report)
}

/// Internal state for one verification run.
///
/// This object is local to a call. It contains no global mutable state and
/// does not alter the input graph.
struct TypeVerifier<'a, C> {
    type_context: &'a C,
    budget: TypeVerificationBudget,
    report: TypeVerificationReport,
    checked_types: BTreeSet<HdlirTypeId>,
}

impl<C: HdlirTypeContext> TypeVerifier<'_, C> {
    /// Consume one work item, returning an explicit error on exhaustion.
    fn consume_work(&mut self, operation: &'static str) -> HdlirResult<()> {
        if let Some(limit) = self.budget.max_items {
            if self.report.work_items >= limit {
                return Err(HdlirError::budget_exceeded(
                    HdlirBudgetKind::Verification,
                    Some(limit),
                    Some(self.report.work_items),
                    operation,
                ));
            }
        }

        self.report.work_items = self
            .report
            .work_items
            .checked_add(1)
            .ok_or_else(|| HdlirError::ArithmeticOverflow {
                operation: "counting HDLIR type-verification work",
                context: Some(operation.to_owned()),
            })?;

        Ok(())
    }

    /// Validate a type identity once per verification run.
    fn check_type(&mut self, type_id: HdlirTypeId) -> HdlirResult<()> {
        if self.checked_types.contains(&type_id) {
            return Ok(());
        }

        self.consume_work("validating an HDLIR type identity")?;
        self.type_context.validate_type(type_id)?;

        self.checked_types.insert(type_id);
        self.report.distinct_types_checked = self
            .report
            .distinct_types_checked
            .checked_add(1)
            .ok_or_else(|| HdlirError::ArithmeticOverflow {
                operation: "counting distinct HDLIR types",
                context: None,
            })?;

        Ok(())
    }

    /// Validate all records in the canonical value table.
    fn verify_values(&mut self, values: &ValueTable) -> HdlirResult<()> {
        for record in values.values() {
            self.consume_work("validating an HDLIR value record")?;
            record.validate()?;
            self.check_type(record.type_id())?;

            self.report.values_checked = self
                .report
                .values_checked
                .checked_add(1)
                .ok_or_else(|| HdlirError::ArithmeticOverflow {
                    operation: "counting HDLIR value records",
                    context: None,
                })?;
        }

        Ok(())
    }

    /// Validate all operation references and apply operation-specific rules.
    fn verify_operations(
        &mut self,
        operations: &[HdlirOperation],
        values: &ValueTable,
    ) -> HdlirResult<()> {
        let mut operation_ids = BTreeSet::new();
        let mut result_ids = BTreeSet::new();

        for operation in operations {
            self.consume_work("validating an HDLIR operation")?;
            operation.validate_structure()?;

            if !operation_ids.insert(operation.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR operation",
                    identity: operation.id().to_string(),
                    scope: Some("type verification input".to_owned()),
                });
            }

            let mut operand_types = Vec::new();
            for operand_id in operation.operands() {
                self.consume_work("validating an HDLIR operation operand")?;

                let record = values.get(*operand_id).ok_or_else(|| {
                    HdlirError::MissingEntity {
                        entity_kind: "HDLIR operand value",
                        identity: operand_id.to_string(),
                        scope: Some(operation.id().to_string()),
                    }
                })?;

                self.check_type(record.type_id())?;
                operand_types.push(record.type_id());

                self.report.operands_checked = self
                    .report
                    .operands_checked
                    .checked_add(1)
                    .ok_or_else(|| HdlirError::ArithmeticOverflow {
                        operation: "counting HDLIR operation operands",
                        context: Some(operation.id().to_string()),
                    })?;
            }

            let mut result_types = Vec::new();
            for result in operation.results() {
                self.consume_work("validating an HDLIR operation result")?;

                if !result_ids.insert(result.id()) {
                    return Err(HdlirError::DuplicateEntity {
                        entity_kind: "HDLIR operation result",
                        identity: result.id().to_string(),
                        scope: Some(operation.id().to_string()),
                    });
                }

                let record = values.get(result.id()).ok_or_else(|| {
                    HdlirError::MissingEntity {
                        entity_kind: "HDLIR operation result value record",
                        identity: result.id().to_string(),
                        scope: Some(operation.id().to_string()),
                    }
                })?;

                if record.type_id() != result.type_id() {
                    return Err(HdlirError::invalid_invariant(
                        "type_verification.result_type_agreement",
                        format!(
                            "operation {} declares result {} with type {}, \
                             but its value record has type {}",
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
                            "type_verification.result_definition",
                            format!(
                                "result {} of operation {} has incompatible \
                                 definition {other:?}",
                                result.id(),
                                operation.id()
                            ),
                        ));
                    }
                }

                self.check_type(result.type_id())?;
                result_types.push(result.type_id());

                self.report.results_checked = self
                    .report
                    .results_checked
                    .checked_add(1)
                    .ok_or_else(|| HdlirError::ArithmeticOverflow {
                        operation: "counting HDLIR operation results",
                        context: Some(operation.id().to_string()),
                    })?;
            }

            // This is the semantic type-checking boundary. The context must
            // reject missing operation contracts rather than silently
            // accepting an operation whose type semantics are unknown.
            self.type_context.verify_operation_types(
                operation,
                &operand_types,
                &result_types,
            )?;

            self.report.operations_checked = self
                .report
                .operations_checked
                .checked_add(1)
                .ok_or_else(|| HdlirError::ArithmeticOverflow {
                    operation: "counting HDLIR operations",
                    context: None,
                })?;
        }

        Ok(())
    }

    /// Ensure every operation-result value refers to an operation in this
    /// verification input and is declared by that operation.
    fn verify_value_definitions(
        &mut self,
        operations: &[HdlirOperation],
        values: &ValueTable,
    ) -> HdlirResult<()> {
        let operation_ids: BTreeSet<HdlirOperationId> =
            operations.iter().map(HdlirOperation::id).collect();

        for record in values.values() {
            let ValueDefinition::OperationResult { operation } = record.definition()
            else {
                continue;
            };

            self.consume_work("validating an HDLIR value definition")?;

            if !operation_ids.contains(operation) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "HDLIR defining operation",
                    identity: operation.to_string(),
                    scope: Some(record.id().to_string()),
                });
            }

            let defining_operation = operations
                .iter()
                .find(|candidate| candidate.id() == *operation)
                .ok_or_else(|| HdlirError::MissingEntity {
                    entity_kind: "HDLIR defining operation",
                    identity: operation.to_string(),
                    scope: Some(record.id().to_string()),
                })?;

            if !defining_operation
                .results()
                .iter()
                .any(|result| result.id() == record.id())
            {
                return Err(HdlirError::invalid_invariant(
                    "type_verification.value_definition_is_declared",
                    format!(
                        "value {} claims operation {} as its definition, \
                         but that operation does not declare it as a result",
                        record.id(),
                        operation
                    ),
                ));
            }
        }

        Ok(())
    }
}

/// Return the type identity of an existing value.
///
/// This small helper is useful to adapters that need to inspect value types
/// without exposing or duplicating the internals of the value table.
///
/// # Errors
///
/// Returns a structured missing-entity error if the requested value does not
/// exist.
pub fn value_type_id(
    values: &ValueTable,
    value_id: HdlirValueId,
) -> HdlirResult<HdlirTypeId> {
    values
        .get(value_id)
        .map(ValueRecord::type_id)
        .ok_or_else(|| HdlirError::MissingEntity {
            entity_kind: "HDLIR value",
            identity: value_id.to_string(),
            scope: None,
        })
}

#[cfg(test)]
mod tests {
    use super::TypeVerificationBudget;

    #[test]
    fn unbounded_budget_does_not_encode_a_hardware_ceiling() {
        assert_eq!(
            TypeVerificationBudget::unbounded().max_items,
            None
        );
    }

    #[test]
    fn explicit_budget_is_preserved() {
        assert_eq!(
            TypeVerificationBudget::new(0).max_items,
            Some(0)
        );
        assert_eq!(
            TypeVerificationBudget::new(u128::MAX).max_items,
            Some(u128::MAX)
        );
    }

    #[test]
    fn default_budget_is_unbounded() {
        assert_eq!(
            TypeVerificationBudget::default(),
            TypeVerificationBudget::unbounded()
        );
    }
}
