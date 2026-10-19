
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Expression Evaluation and Analysis Contract
//!
//! Path: `src/hdl/logic/expression.rs`
//! Rust: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Contract-oriented integration implementation. A concrete evaluator must
//! implement `HdlExpressionEngine` using the canonical AST graph and the
//! repository's semantic services before evaluation is operational.
//!
//! ## Purpose
//!
//! Define the HDL-specific boundary for analyzing and evaluating expressions
//! already represented by the canonical Zamani frontend AST.
//!
//! This module supports:
//! - constant-evaluation requests;
//! - symbolic and runtime-dependent results;
//! - explicit deferral and indeterminate outcomes;
//! - effect-aware evaluation policy;
//! - preservation of canonical AST identity and HDL expression role;
//! - integration with existing semantic analysis and value implementations.
//!
//! It does not implement a parser, a second AST, a second type system, or an
//! intermediate representation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its expression-consuming delegate grammars
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/ast/node/expressions/`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/expressions.rs`
//! - `src/hdl/widths.rs`
//! - `src/hdl/values/`
//!
//! The grammar owns syntax. The canonical AST owns expression structure,
//! operator identity, node identity, and source information. The semantic
//! layer owns name resolution, types, overloads, effects, and semantic
//! legality. Existing HDL value modules own their respective representations.
//!
//! ## Owns
//!
//! - The HDL expression evaluation request contract.
//! - Evaluation modes and explicit evaluation outcomes.
//! - Effect-safety requirements for evaluation.
//! - The extension point through which a concrete semantic evaluator operates.
//!
//! ## Does not own
//!
//! - Parsing, operator precedence, or expression AST construction.
//! - Canonical AST nodes, `NodeId`, or `OperatorRef` definitions.
//! - Name resolution, type inference, or overload resolution.
//! - A duplicate width, signedness, constant, or four-state value model.
//! - Assignment execution, process scheduling, or simulation time.
//! - Physical resource allocation, synthesis, placement, or routing.
//! - HDLIR structures, builders, verifiers, passes, serialization, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration contract
//!
//! A caller constructs an `HdlExpressionUse` from a canonical expression
//! `NodeId` and its existing `HdlExpressionRole`. It submits an
//! `HdlEvaluationRequest` to an implementation of `HdlExpressionEngine`.
//!
//! The engine must inspect the canonical AST and invoke the canonical
//! semantic services. It must not infer operator meaning from spelling
//! alone, silently truncate values, or treat an unresolved expression as
//! a successful constant.
//!
//! This file deliberately does not guess constructors, fields, or methods
//! for AST graph APIs that must be verified at their actual integration site.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on expression count, hardware width,
//! operand count, or design size. Requests use canonical node identities;
//! this module does not recursively traverse expression trees.
//!
//! Actual evaluation consumes finite time and memory. Implementations may
//! enforce explicit operational budgets, but must report incomplete work
//! rather than falsely certify a result.
//!
//! ## Freeze criteria
//!
//! - The AST graph adapter resolves the requested `NodeId`.
//! - The semantic evaluator preserves operator, type, width, and effect rules.
//! - Symbolic, runtime-dependent, deferred, and indeterminate results remain
//!   distinguishable from successful constant evaluation.
//! - Four-state values never silently become ordinary Booleans.
//! - HDLIR remains outside this module.
//! - Integration and repository-wide tests pass.

use core::fmt;

use crate::frontend::ast::node::node_id::NodeId;
use crate::hdl::expressions::{HdlExpressionRole, HdlExpressionUse};

/// Version of this module's public evaluation-request contract.
///
/// This is not the Zamani language version or an HDLIR version.
pub const HDL_EXPRESSION_CONTRACT_VERSION: u16 = 1;

/// Requested evaluation behavior.
///
/// The selected mode controls what results the caller is requesting. It
/// does not override source-language semantics or make an illegal operation
/// valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EvaluationMode {
    /// Evaluate only when a compile-time constant can be established.
    ///
    /// Runtime-dependent expressions must not be executed merely to obtain
    /// a result. An expression that is not constant may return an explicit
    /// non-constant or deferred outcome.
    ConstantOnly,

    /// Analyze the expression without requiring a concrete value.
    ///
    /// The implementation may return symbolic dependencies, semantic facts,
    /// or an indeterminate result.
    AnalyzeOnly,

    /// Permit evaluation using a caller-supplied runtime evaluation context.
    ///
    /// This does not grant permission to execute observable effects unless
    /// the effect policy and source-language semantics permit them.
    RuntimeAllowed,
}

/// Policy for expressions whose effects are relevant to evaluation.
///
/// Effects must be determined by canonical semantic analysis, not guessed
/// from the source spelling of an operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EffectPolicy {
    /// Evaluate only if the semantic layer establishes that evaluation is
    /// free of observable effects.
    RequirePure,

    /// Permit effects when they are explicitly allowed by the language
    /// semantics and the supplied evaluation context.
    AllowPermittedEffects,

    /// Do not execute an expression whose effects cannot be established.
    RejectUnknownEffects,
}

/// Caller-selected operational limits for an evaluation request.
///
/// `None` means this request does not set that particular budget. It does
/// not promise unlimited memory, CPU time, or successful allocation.
///
/// These limits are operational policy, not hardware-width or language
/// semantic limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EvaluationBudget {
    /// Maximum abstract work units the evaluator may consume, if configured.
    pub max_work_units: Option<u64>,

    /// Maximum number of dependency edges the evaluator may inspect, if
    /// configured.
    pub max_dependency_edges: Option<u64>,
}

impl EvaluationBudget {
    /// Create a budget without caller-imposed work limits.
    ///
    /// The evaluator remains subject to available system resources.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_work_units: None,
            max_dependency_edges: None,
        }
    }

    /// Create an explicitly bounded evaluation budget.
    #[must_use]
    pub const fn bounded(
        max_work_units: u64,
        max_dependency_edges: u64,
    ) -> Self {
        Self {
            max_work_units: Some(max_work_units),
            max_dependency_edges: Some(max_dependency_edges),
        }
    }
}

/// Complete request for analyzing or evaluating one canonical expression.
///
/// This structure references the existing AST through `HdlExpressionUse`.
/// It does not copy or recreate the expression tree, source span, type,
/// operator identity, or symbolic-width expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HdlEvaluationRequest {
    expression_use: HdlExpressionUse,
    mode: EvaluationMode,
    effect_policy: EffectPolicy,
    budget: EvaluationBudget,
}

impl HdlEvaluationRequest {
    /// Create an evaluation request.
    ///
    /// The caller must supply an expression reference produced from the
    /// canonical AST. Node existence and node kind must be checked by the
    /// canonical AST graph adapter.
    #[must_use]
    pub const fn new(
        expression_use: HdlExpressionUse,
        mode: EvaluationMode,
        effect_policy: EffectPolicy,
        budget: EvaluationBudget,
    ) -> Self {
        Self {
            expression_use,
            mode,
            effect_policy,
            budget,
        }
    }

    /// Return the canonical expression reference and its HDL usage role.
    #[must_use]
    pub const fn expression_use(&self) -> &HdlExpressionUse {
        &self.expression_use
    }

    /// Return the requested evaluation mode.
    #[must_use]
    pub const fn mode(&self) -> EvaluationMode {
        self.mode
    }

    /// Return the effect policy.
    #[must_use]
    pub const fn effect_policy(&self) -> EffectPolicy {
        self.effect_policy
    }

    /// Return the operational budget.
    #[must_use]
    pub const fn budget(&self) -> EvaluationBudget {
        self.budget
    }

    /// Return the canonical expression node identity.
    #[must_use]
    pub fn expression_id(&self) -> &NodeId {
        self.expression_use.expression()
    }

    /// Return the semantic role of the expression.
    #[must_use]
    pub fn role(&self) -> &HdlExpressionRole {
        self.expression_use.role()
    }
}

/// Effect knowledge established by semantic analysis.
///
/// This is a statement about what the semantic evaluator knows. It is not
/// itself an effect system and must not replace canonical semantic facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EffectKnowledge {
    /// The semantic layer established that evaluation is pure.
    Pure,

    /// Evaluation may perform observable effects.
    Observable,

    /// The available semantic information cannot establish the effects.
    Unknown,
}

/// Why an expression could not be reduced to a constant value.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NonConstantReason {
    /// The expression depends on a runtime value.
    RuntimeDependency,

    /// The operator or operation has no permitted constant interpretation.
    NotConstantEvaluable,

    /// Evaluation would perform observable effects.
    ObservableEffects,

    /// The semantic layer has not established the expression's effects.
    UnknownEffects,

    /// The required type or overload has not been resolved.
    SemanticInformationUnavailable,

    /// The construct is owned by a language or domain extension.
    ExtensionOwned,
}

/// Why a result must be deferred to another compiler phase.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeferredReason {
    /// Name resolution must finish first.
    RequiresNameResolution,

    /// Type checking or overload resolution must finish first.
    RequiresTypeResolution,

    /// One or more symbolic parameters must be resolved.
    RequiresSymbolicResolution,

    /// Another compiler phase owns the required operation.
    LaterCompilerPhase,

    /// A language extension owns the operation.
    ExtensionOwned,
}

/// Why the evaluator cannot currently establish a definitive result.
///
/// An indeterminate result must never be interpreted as proof of validity,
/// constantness, or target feasibility.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IndeterminateReason {
    /// One or more referenced nodes or dependencies are unavailable.
    MissingSemanticDependency,

    /// The operation depends on an unknown or high-impedance logic value.
    UnknownLogicValue,

    /// Symbolic information is insufficient to decide the result.
    UnresolvedSymbolicValue,

    /// The evaluator cannot establish whether the operation is valid.
    UnprovenSemantics,

    /// The evaluator reached an explicitly configured operational budget.
    BudgetExhausted,
}

/// Result of an expression evaluation or analysis request.
///
/// The value type `V` belongs to the concrete semantic evaluator. For HDL
/// logic operations, that evaluator may use the existing types in
/// `src/hdl/values/`; this contract deliberately does not introduce a
/// competing universal value enum.
///
/// `Constant(V)` is the only outcome that asserts a concrete evaluated
/// value has been established. All other variants preserve uncertainty or
/// explain why a concrete value is unavailable.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EvaluationOutcome<V> {
    /// A concrete value was successfully established.
    Constant(V),

    /// Evaluation remains dependent on symbolic inputs.
    ///
    /// The evaluator is responsible for retaining the canonical dependency
    /// relationships in its semantic context.
    Symbolic,

    /// The expression depends on runtime state or inputs.
    RuntimeDependent,

    /// A constant result is not available for the stated reason.
    NotConstant(NonConstantReason),

    /// The expression requires another semantic or compiler phase.
    Deferred(DeferredReason),

    /// The evaluator cannot establish a definitive result.
    Indeterminate(IndeterminateReason),
}

impl<V> EvaluationOutcome<V> {
    /// Return whether this outcome contains an established constant value.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Return the established constant value, if one exists.
    #[must_use]
    pub const fn as_constant(&self) -> Option<&V> {
        match self {
            Self::Constant(value) => Some(value),
            _ => None,
        }
    }

    /// Transform a constant value while preserving every non-constant outcome.
    pub fn map_constant<U>(
        self,
        transform: impl FnOnce(V) -> U,
    ) -> EvaluationOutcome<U> {
        match self {
            Self::Constant(value) => EvaluationOutcome::Constant(transform(value)),
            Self::Symbolic => EvaluationOutcome::Symbolic,
            Self::RuntimeDependent => EvaluationOutcome::RuntimeDependent,
            Self::NotConstant(reason) => EvaluationOutcome::NotConstant(reason),
            Self::Deferred(reason) => EvaluationOutcome::Deferred(reason),
            Self::Indeterminate(reason) => EvaluationOutcome::Indeterminate(reason),
        }
    }
}

/// Successful result envelope returned by a semantic evaluator.
///
/// `expression` identifies the canonical AST node whose evaluation was
/// requested. `effects` records the evaluator's semantic finding. The
/// envelope does not independently certify type correctness or target
/// feasibility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HdlEvaluationResult<V> {
    expression: NodeId,
    outcome: EvaluationOutcome<V>,
    effects: EffectKnowledge,
}

impl<V> HdlEvaluationResult<V> {
    /// Construct an evaluation result for the requested canonical node.
    ///
    /// Concrete engines should use the node ID from the original request.
    #[must_use]
    pub const fn new(
        expression: NodeId,
        outcome: EvaluationOutcome<V>,
        effects: EffectKnowledge,
    ) -> Self {
        Self {
            expression,
            outcome,
            effects,
        }
    }

    /// Return the identity of the evaluated expression.
    #[must_use]
    pub const fn expression(&self) -> &NodeId {
        &self.expression
    }

    /// Borrow the evaluation outcome.
    #[must_use]
    pub const fn outcome(&self) -> &EvaluationOutcome<V> {
        &self.outcome
    }

    /// Consume the result and return its outcome.
    #[must_use]
    pub fn into_outcome(self) -> EvaluationOutcome<V> {
        self.outcome
    }

    /// Return the evaluator's effect finding.
    #[must_use]
    pub const fn effects(&self) -> EffectKnowledge {
        self.effects
    }

    /// Return whether a constant value was established.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        self.outcome.is_constant()
    }
}

/// Errors that a concrete expression engine may report.
///
/// This enum covers the common boundary failures. Implementations may wrap
/// their detailed AST, semantic, value, and diagnostic errors in their own
/// error types rather than converting them to strings.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HdlExpressionError {
    /// The requested canonical node does not exist.
    UnknownExpression(NodeId),

    /// The canonical node exists but is not an expression node.
    NotAnExpression(NodeId),

    /// The returned result belongs to a different expression.
    ResultIdentityMismatch {
        /// Requested canonical expression.
        expected: NodeId,

        /// Expression reported by the evaluator.
        actual: NodeId,
    },

    /// The evaluation policy prohibits the requested operation.
    PolicyDenied,

    /// The operation could not be completed within the configured budget.
    BudgetExhausted,

    /// A semantic or value operation failed.
    EvaluationFailed(String),
}

impl fmt::Display for HdlExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownExpression(id) => {
                write!(formatter, "unknown HDL expression node {id:?}")
            }
            Self::NotAnExpression(id) => {
                write!(formatter, "AST node {id:?} is not an expression")
            }
            Self::ResultIdentityMismatch { expected, actual } => {
                write!(
                    formatter,
                    "HDL expression result identity mismatch: \
                     expected {expected:?}, received {actual:?}"
                )
            }
            Self::PolicyDenied => {
                formatter.write_str("HDL expression evaluation denied by policy")
            }
            Self::BudgetExhausted => {
                formatter.write_str("HDL expression evaluation budget exhausted")
            }
            Self::EvaluationFailed(message) => {
                write!(formatter, "HDL expression evaluation failed: {message}")
            }
        }
    }
}

impl std::error::Error for HdlExpressionError {}

/// Adapter implemented by the concrete HDL semantic evaluation service.
///
/// The implementation must use the canonical AST graph and semantic model.
/// It is responsible for operator lookup, operand resolution, type checking,
/// overload resolution, effect analysis, width rules, and operation-specific
/// value semantics.
///
/// The trait intentionally does not assume an unverified AST graph API or
/// invent a second expression representation.
///
/// A concrete implementation should use an explicit work stack for deep
/// expression graphs where recursive evaluation could exhaust the call stack.
pub trait HdlExpressionEngine {
    /// Concrete value type produced by this engine.
    ///
    /// For example, an implementation may use the existing constant and
    /// four-state representations without requiring this contract to know
    /// their internal layouts.
    type Value;

    /// Engine-specific error type.
    type Error;

    /// Analyze or evaluate the requested canonical expression.
    ///
    /// Required invariants:
    ///
    /// - Validate that the node exists and is an expression.
    /// - Preserve the role supplied by the caller.
    /// - Follow canonical operator and type semantics.
    /// - Respect the requested mode and effect policy.
    /// - Never report an unresolved or partial result as a constant.
    /// - Return an explicit error when the operation itself fails.
    /// - Return `Indeterminate(BudgetExhausted)` or a documented engine
    ///   error when the configured budget prevents completion.
    /// - Do not lower to HDLIR or select a physical implementation.
    fn evaluate(
        &mut self,
        request: &HdlEvaluationRequest,
    ) -> Result<HdlEvaluationResult<Self::Value>, Self::Error>;
}

/// Invoke an expression engine and verify that the result identifies the
/// expression that was requested.
///
/// This wrapper enforces the cross-node identity invariant. The engine
/// remains responsible for validating AST node existence, expression kind,
/// semantic correctness, evaluation policy, and result completeness.
///
/// # Errors
///
/// Returns the engine's error unchanged if evaluation fails. Returns
/// [`HdlExpressionError::ResultIdentityMismatch`] if the engine reports
/// a different expression identity.
pub fn evaluate_expression<E>(
    engine: &mut E,
    request: &HdlEvaluationRequest,
) -> Result<HdlEvaluationResult<E::Value>, EvaluationBoundaryError<E::Error>>
where
    E: HdlExpressionEngine,
{
    let result = engine
        .evaluate(request)
        .map_err(EvaluationBoundaryError::Engine)?;

    if result.expression() != request.expression_id() {
        return Err(EvaluationBoundaryError::IdentityMismatch {
            expected: request.expression_id().clone(),
            actual: result.expression().clone(),
        });
    }

    Ok(result)
}

/// Error returned by the expression evaluation boundary.
#[derive(Debug, PartialEq, Eq)]
pub enum EvaluationBoundaryError<E> {
    /// The concrete semantic engine failed.
    Engine(E),

    /// The engine returned a result for a different AST node.
    IdentityMismatch {
        /// Requested canonical node.
        expected: NodeId,

        /// Actual canonical node in the returned result.
        actual: NodeId,
    },
}

impl<E: fmt::Display> fmt::Display for EvaluationBoundaryError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engine(error) => {
                write!(formatter, "HDL expression engine error: {error}")
            }
            Self::IdentityMismatch { expected, actual } => {
                write!(
                    formatter,
                    "HDL expression result identity mismatch: \
                     expected {expected:?}, received {actual:?}"
                )
            }
        }
    }
}

impl<E> std::error::Error for EvaluationBoundaryError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Engine(error) => Some(error),
            Self::IdentityMismatch { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DeferredReason, EffectKnowledge, EvaluationBudget, EvaluationMode,
        EvaluationOutcome, EffectPolicy, HdlEvaluationRequest,
        HdlEvaluationResult, HdlExpressionEngine, NonConstantReason,
        evaluate_expression,
    };
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::hdl::expressions::{HdlExpressionRole, HdlExpressionUse};

    // The canonical NodeId constructor is intentionally not assumed here.
    // Integration tests should construct IDs using the actual repository
    // allocator. These unit tests focus on generic outcome behavior below.

    #[test]
    fn non_constant_outcomes_are_not_constants() {
        let outcome: EvaluationOutcome<u32> =
            EvaluationOutcome::NotConstant(NonConstantReason::RuntimeDependency);

        assert!(!outcome.is_constant());
        assert!(outcome.as_constant().is_none());
    }

    #[test]
    fn deferred_outcomes_are_preserved_when_mapping_constants() {
        let outcome: EvaluationOutcome<u32> =
            EvaluationOutcome::Deferred(DeferredReason::RequiresTypeResolution);

        let mapped = outcome.map_constant(|value| value.to_string());

        assert_eq!(
            mapped,
            EvaluationOutcome::<String>::Deferred(
                DeferredReason::RequiresTypeResolution
            )
        );
    }

    #[test]
    fn symbolic_outcomes_do_not_become_constants() {
        let outcome: EvaluationOutcome<u32> = EvaluationOutcome::Symbolic;

        assert!(!outcome.is_constant());
        assert!(outcome.as_constant().is_none());
    }

    #[test]
    fn budget_is_explicit_and_has_no_implicit_language_ceiling() {
        let budget = EvaluationBudget::unrestricted();

        assert_eq!(budget.max_work_units, None);
        assert_eq!(budget.max_dependency_edges, None);
    }

    #[test]
    fn effect_policy_is_explicit() {
        assert_ne!(EffectPolicy::RequirePure, EffectPolicy::AllowPermittedEffects);
        assert_ne!(EffectPolicy::RequirePure, EffectPolicy::RejectUnknownEffects);
    }

    // Full engine integration tests belong beside the canonical AST graph
    // adapter, where valid NodeIds can be allocated by the repository's
    // real NodeId allocator. They must cover:
    //
    // - unknown node IDs and non-expression nodes;
    // - identity mismatch between request and result;
    // - pure versus effectful expressions;
    // - symbolic and unresolved widths;
    // - signed and unsigned arithmetic;
    // - four-state X/Z propagation;
    // - invalid operators and type mismatches;
    // - work-budget exhaustion;
    // - deeply nested expression graphs without stack overflow;
    // - deterministic diagnostics and evaluation outcomes.
    //
    // Do not fabricate a NodeId constructor here: use the canonical allocator.
}
