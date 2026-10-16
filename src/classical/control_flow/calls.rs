
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Call-Site Planning
//!
//! Path: `src/classical/control_flow/calls.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # File contract
//!
//! Status: Proposed implementation; workspace verification pending.
//!
//! ## Purpose
//!
//! Represent a semantically resolved call site using canonical ZUIR
//! identities. Preserve its caller, call-operation identity, target,
//! resolved argument values, and result values.
//!
//! ## Authority
//!
//! - `grammar/expressions/calls.g4` owns invocation syntax.
//! - `grammar/functions/functions.g4` owns function declaration syntax.
//! - The frontend AST owns source-level call expressions and declarations.
//! - Semantic analysis owns callable resolution, overload resolution,
//!   generic inference, argument matching, default arguments, type checking,
//!   effects, capabilities, ownership, and borrowing.
//! - `crate::zuir::ids` owns canonical function, operation, and value IDs.
//! - Canonical ZUIR owns call operations, function signatures, and execution
//!   semantics.
//! - `crate::classical::error` owns classical errors.
//!
//! ## Owns
//!
//! - `CallTarget`, which distinguishes direct function targets from
//!   indirect callable-value targets.
//! - `CallSitePlan`, a target-independent set of references for a resolved
//!   call site.
//! - Local construction invariants, accessors, and decomposition.
//!
//! ## Does not own
//!
//! - Parsing, AST definitions, or source spans.
//! - Function declarations, signatures, or callable types.
//! - Name or overload resolution.
//! - Generic substitution or inference.
//! - Source argument evaluation order.
//! - Named-argument matching or default-argument evaluation.
//! - Type/effect/capability/resource validation.
//! - Canonical IR operations, CFG construction, or identity allocation.
//! - ABI selection, execution, scheduling, or runtime dispatch.
//!
//! ## Input and output contract
//!
//! Inputs are canonical IDs supplied by a caller after the applicable
//! semantic resolution stage. Argument values must already be mapped to
//! the callee's resolved parameter order by semantic analysis or lowering.
//! Result IDs identify the values produced by the call in canonical IR.
//!
//! The plan does not prove that the IDs exist or that the call is valid
//! in a particular IR. The canonical verifier must establish those facts.
//!
//! ## Scalability and POCO-REAF
//!
//! There is no fixed semantic limit on arguments, results, functions,
//! devices, or call sites. Argument and result storage grows with supplied
//! input and is bounded by available memory and addressable representation.
//! No claim of infinite physical resources is made.
//!
//! ## Determinism
//!
//! Argument and result vectors preserve the supplied order. This is
//! important: argument order is semantically significant after argument
//! matching, and result order follows the resolved call signature.
//!
//! ## Integration
//!
//! - `src/classical/control_flow/mod.rs` declares and exports this module.
//! - `src/classical/ids.rs` re-exports canonical `FunctionId`, `OperationId`,
//!   and `ValueId`.
//! - `src/classical/error.rs` provides `ClassicalError` and `ClassicalResult`.
//! - `src/classical/control_flow/functions.rs` describes function-level
//!   entry and return-block references; it does not own call sites.
//! - `src/classical/validation/` validates resolved calls against semantic
//!   and canonical IR information.
//! - `src/classical/lowering/control_flow.rs` or the appropriate canonical
//!   call-lowering module consumes validated plans.
//! - `src/zuir/ir/function.rs` remains the canonical function owner.
//! - The canonical ZUIR call operation and verifier remain authoritative.
//! - `grammar/expressions/calls.g4` remains the call-syntax authority.
//!
//! ## Freeze rule
//!
//! Changes to public fields, target forms, or argument/result semantics
//! require compatibility review of semantic analysis, lowering, and the
//! canonical call-operation representation.

use std::collections::BTreeSet;

use crate::classical::error::{ClassicalError, ClassicalResult};
use crate::classical::ids::{FunctionId, OperationId, ValueId};

/// The resolved target of a call.
///
/// This type intentionally distinguishes a statically resolved function
/// from a callable value whose concrete target is determined by the
/// language's indirect-call semantics.
///
/// It does not represent a function signature, overload set, ABI, or runtime
/// dispatch table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CallTarget {
    /// A function whose canonical identity has already been resolved.
    Direct(FunctionId),

    /// A canonical value representing an indirect callable target.
    ///
    /// Semantic analysis must establish that this value has a callable type
    /// and that its invocation is valid.
    Indirect(ValueId),
}

impl CallTarget {
    /// Returns the direct function ID, if this is a direct call.
    #[must_use]
    pub const fn as_direct(self) -> Option<FunctionId> {
        match self {
            Self::Direct(function) => Some(function),
            Self::Indirect(_) => None,
        }
    }

    /// Returns the indirect callable value, if this is an indirect call.
    #[must_use]
    pub const fn as_indirect(self) -> Option<ValueId> {
        match self {
            Self::Direct(_) => None,
            Self::Indirect(value) => Some(value),
        }
    }

    /// Returns whether this target is a direct function reference.
    #[must_use]
    pub const fn is_direct(self) -> bool {
        matches!(self, Self::Direct(_))
    }

    /// Returns whether this target is an indirect callable value.
    #[must_use]
    pub const fn is_indirect(self) -> bool {
        matches!(self, Self::Indirect(_))
    }
}

/// Target-independent references for one semantically resolved call site.
///
/// The plan is not itself a canonical IR call instruction. It records
/// canonical identities that the canonical IR builder or adapter can use.
///
/// `arguments` are in the resolved parameter order expected by the call
/// operation, not necessarily their original textual order. Source-order
/// evaluation and the semantics of named, spread, or default arguments
/// must already have been handled by the appropriate semantic/lowering
/// stages.
///
/// `results` preserve the canonical call's result ordering. A call may
/// produce no result, one result, or multiple results according to the
/// resolved callable and canonical IR contract.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallSitePlan {
    operation: OperationId,
    caller: FunctionId,
    target: CallTarget,
    arguments: Vec<ValueId>,
    results: Vec<ValueId>,
}

impl CallSitePlan {
    /// Creates a call-site plan after target and argument resolution.
    ///
    /// The constructor validates that result IDs are distinct within this
    /// plan. Reusing an argument value is allowed: the same SSA value may
    /// legally be passed to more than one parameter.
    ///
    /// This function does not validate function signatures, parameter types,
    /// callable-value types, effects, or the existence of referenced IDs.
    ///
    /// # Errors
    ///
    /// Returns `ClassicalError::InvalidInput` if a result ID appears more
    /// than once.
    pub fn new(
        operation: OperationId,
        caller: FunctionId,
        target: CallTarget,
        arguments: Vec<ValueId>,
        results: Vec<ValueId>,
    ) -> ClassicalResult<Self> {
        let mut seen_results = BTreeSet::new();

        for result in &results {
            if !seen_results.insert(*result) {
                return Err(ClassicalError::invalid_input(format!(
                    "call operation {operation} declares result value {result} more than once"
                )));
            }
        }

        Ok(Self {
            operation,
            caller,
            target,
            arguments,
            results,
        })
    }

    /// Returns the canonical operation identity for this call site.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the canonical identity of the function containing this call.
    #[must_use]
    pub const fn caller(&self) -> FunctionId {
        self.caller
    }

    /// Returns the resolved call target.
    #[must_use]
    pub const fn target(&self) -> CallTarget {
        self.target
    }

    /// Returns argument value identities in resolved parameter order.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Returns result value identities in canonical result order.
    #[must_use]
    pub fn results(&self) -> &[ValueId] {
        &self.results
    }

    /// Returns the number of arguments in the resolved call.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns the number of results produced by the resolved call.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Returns whether the call has no arguments.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns whether the call has no results.
    #[must_use]
    pub fn has_no_results(&self) -> bool {
        self.results.is_empty()
    }

    /// Consumes the plan and returns its component references.
    ///
    /// Argument and result ordering is preserved.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OperationId,
        FunctionId,
        CallTarget,
        Vec<ValueId>,
        Vec<ValueId>,
    ) {
        (
            self.operation,
            self.caller,
            self.target,
            self.arguments,
            self.results,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("test function ID must be nonzero")
    }

    fn operation_id(value: u128) -> OperationId {
        OperationId::new(value).expect("test operation ID must be nonzero")
    }

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("test value ID must be nonzero")
    }

    #[test]
    fn direct_call_preserves_canonical_references_and_order() {
        let operation = operation_id(1);
        let caller = function_id(2);
        let callee = function_id(3);
        let arguments = vec![value_id(20), value_id(10)];
        let results = vec![value_id(30)];

        let plan = CallSitePlan::new(
            operation,
            caller,
            CallTarget::Direct(callee),
            arguments.clone(),
            results.clone(),
        )
        .expect("distinct result IDs should be accepted");

        assert_eq!(plan.operation(), operation);
        assert_eq!(plan.caller(), caller);
        assert_eq!(plan.target().as_direct(), Some(callee));
        assert_eq!(plan.target().as_indirect(), None);
        assert_eq!(plan.arguments(), arguments);
        assert_eq!(plan.results(), results);
        assert_eq!(plan.argument_count(), 2);
        assert_eq!(plan.result_count(), 1);
    }

    #[test]
    fn indirect_call_preserves_callable_value() {
        let callable = value_id(50);

        let plan = CallSitePlan::new(
            operation_id(1),
            function_id(2),
            CallTarget::Indirect(callable),
            vec![value_id(3)],
            vec![value_id(4)],
        )
        .expect("valid indirect call plan");

        assert_eq!(plan.target().as_indirect(), Some(callable));
        assert_eq!(plan.target().as_direct(), None);
        assert!(plan.target().is_indirect());
        assert!(!plan.target().is_direct());
    }

    #[test]
    fn zero_argument_zero_result_call_is_supported() {
        let plan = CallSitePlan::new(
            operation_id(1),
            function_id(2),
            CallTarget::Direct(function_id(3)),
            Vec::new(),
            Vec::new(),
        )
        .expect("calls may have no arguments or results");

        assert!(plan.has_no_arguments());
        assert!(plan.has_no_results());
        assert_eq!(plan.argument_count(), 0);
        assert_eq!(plan.result_count(), 0);
    }

    #[test]
    fn multiple_results_are_supported_and_ordered() {
        let results = vec![value_id(8), value_id(3), value_id(5)];

        let plan = CallSitePlan::new(
            operation_id(1),
            function_id(2),
            CallTarget::Direct(function_id(3)),
            Vec::new(),
            results.clone(),
        )
        .expect("distinct result IDs should be accepted");

        assert_eq!(plan.results(), results);
        assert_eq!(plan.result_count(), 3);
    }

    #[test]
    fn duplicate_result_ids_are_rejected() {
        let duplicate = value_id(8);

        let result = CallSitePlan::new(
            operation_id(1),
            function_id(2),
            CallTarget::Direct(function_id(3)),
            Vec::new(),
            vec![duplicate, value_id(9), duplicate],
        );

        assert!(matches!(
            result,
            Err(ClassicalError::InvalidInput { .. })
        ));
    }

    #[test]
    fn the_same_argument_value_may_be_passed_more_than_once() {
        let repeated = value_id(8);

        let plan = CallSitePlan::new(
            operation_id(1),
            function_id(2),
            CallTarget::Direct(function_id(3)),
            vec![repeated, repeated],
            Vec::new(),
        )
        .expect("reusing a value as an argument is not inherently invalid");

        assert_eq!(plan.arguments(), &[repeated, repeated]);
    }

    #[test]
    fn large_canonical_ids_are_not_truncated() {
        let operation = operation_id(u128::MAX);
        let caller = function_id(u128::MAX - 1);
        let callee = function_id(u128::MAX - 2);
        let argument = value_id(u128::MAX - 3);
        let result = value_id(u128::MAX - 4);

        let plan = CallSitePlan::new(
            operation,
            caller,
            CallTarget::Direct(callee),
            vec![argument],
            vec![result],
        )
        .expect("large nonzero canonical IDs should be preserved");

        assert_eq!(plan.operation(), operation);
        assert_eq!(plan.caller(), caller);
        assert_eq!(plan.target(), CallTarget::Direct(callee));
        assert_eq!(plan.arguments(), &[argument]);
        assert_eq!(plan.results(), &[result]);
    }

    #[test]
    fn decomposition_preserves_every_field_and_order() {
        let operation = operation_id(1);
        let caller = function_id(2);
        let target = CallTarget::Indirect(value_id(3));
        let arguments = vec![value_id(4), value_id(5)];
        let results = vec![value_id(6), value_id(7)];

        let plan = CallSitePlan::new(
            operation,
            caller,
            target,
            arguments.clone(),
            results.clone(),
        )
        .expect("valid call plan");

        let (actual_operation, actual_caller, actual_target, actual_arguments, actual_results) =
            plan.into_parts();

        assert_eq!(actual_operation, operation);
        assert_eq!(actual_caller, caller);
        assert_eq!(actual_target, target);
        assert_eq!(actual_arguments, arguments);
        assert_eq!(actual_results, results);
    }

    #[test]
    fn plan_supports_clone_and_equality() {
        let plan = CallSitePlan::new(
            operation_id(1),
            function_id(2),
            CallTarget::Direct(function_id(3)),
            vec![value_id(4)],
            vec![value_id(5)],
        )
        .expect("valid call plan");

        assert_eq!(plan.clone(), plan);
    }
}
