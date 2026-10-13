
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Classical-Domain Semantic Validation
//!
//! Path: `src/frontend/semantic/domains/classical.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the classical-domain semantic validation boundary for the native
//! Zamani frontend AST.
//!
//! Classical validation checks the classical interpretation of source
//! constructs while delegating shared language semantics to their authoritative
//! owners: name resolution, type inference, type compatibility, effects,
//! capabilities, contracts, resource requirements, and diagnostics.
//!
//! This module is designed to compose with quantum, HDL, hybrid, AI/data,
//! concurrency, distributed, and future domain validators.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/types/`
//! - `src/frontend/semantic/symbols/`
//! - `src/frontend/semantic/effects/`
//! - `src/frontend/semantic/diagnostics/`
//!
//! The grammar and normative specifications define source meaning.
//! The canonical AST defines source structure.
//! Shared semantic subsystems own their respective analysis rules.
//!
//! ## OWNS
//!
//! - Classical-domain validation contracts.
//! - Classical-specific rule dispatch.
//! - Classical validation issue classification.
//! - Explicit delegation points for shared semantic checks.
//! - Validation completeness reporting.
//!
//! ## DOES_NOT_OWN
//!
//! - AST definitions or AST identity allocation.
//! - Parsing, lexing, or grammar interpretation.
//! - Symbol-table storage or name resolution.
//! - The canonical semantic type system.
//! - Type inference, unification, or generic substitution.
//! - Effect inference or effect authorization.
//! - Capability registration or target capability discovery.
//! - Resource feasibility or hardware allocation.
//! - Quantum operation registries or quantum IR.
//! - HDL elaboration or hardware IR.
//! - Classical IR generation or optimization.
//! - Backend selection, scheduling, placement, or runtime execution.
//!
//! ## SCALABILITY
//!
//! This module defines no universal machine-capacity ceilings, fixed expression
//! counts, fixed declaration counts, fixed integer widths, or fixed domain
//! counts.
//!
//! The caller controls analysis budgets. Exhausting a budget must be reported
//! as incomplete analysis, never as successful semantic validation.
//!
//! Actual computation remains bounded by available memory, execution time,
//! representable values, configured operational policy, and physical resources.
//!
//! ## DETERMINISM
//!
//! This module does not use global mutable state, randomness, wall-clock time,
//! hardware discovery, or hash-map iteration order to decide semantic meaning.
//!
//! Diagnostic ordering is delegated to the shared diagnostics subsystem.
//!
//! ## INTEGRATION
//!
//! - The native AST supplies `Expression`, `ExpressionKind`, `OperatorRef`,
//!   and `NodeId`.
//! - The semantic session supplies a `ClassicalSemanticContext` implementation.
//! - The symbol subsystem resolves identifiers and call targets.
//! - The type subsystem checks operator operands, conditions, calls, casts,
//!   assignments, and return/value compatibility.
//! - The effects subsystem checks observable effects and execution contexts.
//! - The capability and resource subsystems retain their own obligations.
//! - The domain registry selects this validator for classical-domain analysis.
//! - The semantic pipeline merges the returned issues into its canonical
//!   diagnostic collection.
//! - IR generation consumes finalized semantic facts, not this validator's
//!   temporary state.
//!
//! ## FREEZE CRITERIA
//!
//! - The context implementation delegates to the canonical symbol, type,
//!   effect, capability, and diagnostic services.
//! - Every supported native AST expression kind has an explicit rule path.
//! - Unknown extension constructs are never silently certified as valid.
//! - Context-sensitive rules are tested using the normative language rules.
//! - Incomplete analysis cannot be mistaken for a valid program.
//! - Diagnostics retain canonical AST node identities.
//! - The file passes workspace compilation, formatting, lint, conformance,
//!   determinism, fuzz, and stress tests.
//!
//! ## IMPORTANT LIMITATION
//!
//! This module does not claim that every expression is valid merely because
//! its shape is recognized. Shared semantic checks must actually be performed
//! by the supplied context implementation.

use core::fmt;

use crate::frontend::ast::node::expressions::expression::{
    Expression, ExpressionKind, OperatorRef,
};
use crate::frontend::ast::node::node_id::NodeId;

// =============================================================================
// Public schema
// =============================================================================

/// Version of the classical validation contract.
///
/// This version is independent of the Zamani language version, AST schema,
/// semantic artifact schema, and downstream IR versions.
pub const CLASSICAL_VALIDATION_SCHEMA_VERSION: u16 = 1;

/// Result returned by a classical semantic validation pass.
pub type ClassicalResult<T> = Result<T, ClassicalAnalysisFailure>;

/// Stable categories of classical-domain validation issues.
///
/// These categories describe the kind of rule involved. The repository's
/// diagnostic-code registry remains authoritative for final diagnostic codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ClassicalIssueKind {
    /// A source expression refers to an unknown or inaccessible declaration.
    NameResolution,

    /// An operator is not defined for its operands.
    OperatorSemantics,

    /// An expression has an invalid type or type relationship.
    TypeSemantics,

    /// A conditional or logical operation violates its condition rules.
    ConditionSemantics,

    /// A call violates the callable's declared signature.
    CallSemantics,

    /// An assignment violates the language's binding or mutability rules.
    AssignmentSemantics,

    /// An asynchronous operation is used in an invalid context.
    AsyncSemantics,

    /// An operation's effect is not permitted in the current context.
    EffectSemantics,

    /// A required capability or policy obligation is not established.
    CapabilitySemantics,

    /// A domain-specific extension has no applicable semantic handler.
    UnsupportedExtension,

    /// The expression graph or one of its references is invalid.
    StructuralSemantics,

    /// Analysis stopped before all required semantic rules were checked.
    IncompleteAnalysis,
}

impl fmt::Display for ClassicalIssueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Self::NameResolution => "name resolution",
            Self::OperatorSemantics => "operator semantics",
            Self::TypeSemantics => "type semantics",
            Self::ConditionSemantics => "condition semantics",
            Self::CallSemantics => "call semantics",
            Self::AssignmentSemantics => "assignment semantics",
            Self::AsyncSemantics => "asynchronous semantics",
            Self::EffectSemantics => "effect semantics",
            Self::CapabilitySemantics => "capability semantics",
            Self::UnsupportedExtension => "unsupported extension",
            Self::StructuralSemantics => "structural semantics",
            Self::IncompleteAnalysis => "incomplete analysis",
        };

        f.write_str(description)
    }
}

/// One classical semantic issue.
///
/// This is an internal domain-validation record. The semantic pipeline should
/// convert it into the repository's canonical diagnostic representation,
/// preserving the node identity and issue category.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassicalIssue {
    /// Category of the semantic rule that failed.
    pub kind: ClassicalIssueKind,

    /// AST node primarily associated with the issue.
    pub node: NodeId,

    /// Optional related AST node, such as an operand or assignment target.
    pub related_node: Option<NodeId>,

    /// Stable machine-readable rule identifier.
    ///
    /// This identifier is local to this domain contract. The canonical
    /// diagnostics registry may map it to a repository-wide diagnostic code.
    pub rule: &'static str,

    /// Human-readable explanation.
    ///
    /// Consumers must use `kind` and `rule` for programmatic classification,
    /// rather than parsing this text.
    pub message: String,
}

impl ClassicalIssue {
    /// Creates an issue associated with one AST node.
    #[must_use]
    pub fn new(
        kind: ClassicalIssueKind,
        node: NodeId,
        rule: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            node,
            related_node: None,
            rule,
            message: message.into(),
        }
    }

    /// Attaches a related AST node.
    #[must_use]
    pub fn with_related_node(mut self, related_node: NodeId) -> Self {
        self.related_node = Some(related_node);
        self
    }
}

/// Completion status of a classical semantic analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassicalCompletion {
    /// All required rules were run and no issue was reported.
    Complete,

    /// All scheduled rules ran, but at least one issue was reported.
    CompleteWithIssues,

    /// Analysis could not establish the complete semantic result.
    Incomplete,
}

/// Result of classical-domain validation.
///
/// `Complete` is necessary but not sufficient for whole-program validity:
/// the shared semantic pipeline must also finalize names, types, effects,
/// capabilities, resources, contracts, and all applicable domain obligations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassicalValidation {
    /// Completion state of the classical pass.
    pub completion: ClassicalCompletion,

    /// Classical issues in deterministic production order.
    pub issues: Vec<ClassicalIssue>,

    /// Number of expressions submitted to this pass.
    pub expressions_checked: usize,
}

impl ClassicalValidation {
    /// Creates a completed result without issues.
    #[must_use]
    pub fn success(expressions_checked: usize) -> Self {
        Self {
            completion: ClassicalCompletion::Complete,
            issues: Vec::new(),
            expressions_checked,
        }
    }

    /// Creates a completed result containing issues.
    #[must_use]
    pub fn with_issues(
        expressions_checked: usize,
        issues: Vec<ClassicalIssue>,
    ) -> Self {
        let completion = if issues.is_empty() {
            ClassicalCompletion::Complete
        } else {
            ClassicalCompletion::CompleteWithIssues
        };

        Self {
            completion,
            issues,
            expressions_checked,
        }
    }

    /// Creates an incomplete result.
    ///
    /// An incomplete result must never be treated as proof that the program is
    /// semantically valid.
    #[must_use]
    pub fn incomplete(
        expressions_checked: usize,
        mut issues: Vec<ClassicalIssue>,
        node: NodeId,
        reason: impl Into<String>,
    ) -> Self {
        issues.push(ClassicalIssue::new(
            ClassicalIssueKind::IncompleteAnalysis,
            node,
            "classical.analysis.incomplete",
            reason,
        ));

        Self {
            completion: ClassicalCompletion::Incomplete,
            issues,
            expressions_checked,
        }
    }

    /// Returns true only if this pass completed and reported no issues.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.completion == ClassicalCompletion::Complete && self.issues.is_empty()
    }
}

/// Failures that prevent the pass from producing a complete result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClassicalAnalysisFailure {
    /// The configured analysis budget was exhausted.
    BudgetExceeded {
        /// Name of the configured budget.
        budget: String,

        /// Number of units processed before stopping.
        processed: usize,
    },

    /// The caller supplied an invalid validation policy.
    InvalidPolicy {
        /// Explanation of the invalid policy.
        message: String,
    },

    /// A semantic service failed before its check could complete.
    ServiceFailure {
        /// Name of the service.
        service: String,

        /// Explanation supplied by the service.
        message: String,
    },
}

impl fmt::Display for ClassicalAnalysisFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BudgetExceeded { budget, processed } => {
                write!(
                    f,
                    "classical analysis budget `{budget}` exceeded after processing {processed} items"
                )
            }
            Self::InvalidPolicy { message } => {
                write!(f, "invalid classical analysis policy: {message}")
            }
            Self::ServiceFailure { service, message } => {
                write!(f, "classical semantic service `{service}` failed: {message}")
            }
        }
    }
}

impl std::error::Error for ClassicalAnalysisFailure {}

// =============================================================================
// Configurable operational policy
// =============================================================================

/// Optional operational budgets for classical semantic validation.
///
/// These limits are compiler safety policies, not language-level limits.
/// `None` means this policy does not impose that particular budget.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClassicalValidationPolicy {
    /// Maximum number of expressions accepted by this pass.
    pub max_expressions: Option<usize>,

    /// Maximum number of issues emitted by this pass.
    ///
    /// Reaching the issue limit marks the result incomplete because additional
    /// errors may remain undiscovered.
    pub max_issues: Option<usize>,
}

impl ClassicalValidationPolicy {
    /// Returns a policy that imposes no local operational budgets.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_expressions: None,
            max_issues: None,
        }
    }

    /// Validates policy consistency.
    ///
    /// A zero budget is rejected rather than silently interpreted as
    /// unrestricted. Call `unrestricted()` to request no local limit.
    pub fn validate(self) -> ClassicalResult<()> {
        if self.max_expressions == Some(0) {
            return Err(ClassicalAnalysisFailure::InvalidPolicy {
                message: "max_expressions must be positive or None".to_owned(),
            });
        }

        if self.max_issues == Some(0) {
            return Err(ClassicalAnalysisFailure::InvalidPolicy {
                message: "max_issues must be positive or None".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Shared semantic-service boundary
// =============================================================================

/// Shared semantic services required by classical-domain analysis.
///
/// Implementations belong to the semantic session or pipeline, not this file.
///
/// Methods default to success only when the corresponding rule is explicitly
/// optional for a particular expression shape. Production implementations must
/// override all checks required by the language specification.
///
/// The `check_*` methods return issues rather than panicking. Service failures
/// that prevent analysis from completing are returned as `ClassicalResult`.
pub trait ClassicalSemanticContext {
    /// Checks the name-resolution and visibility obligations for a reference.
    fn check_name(
        &mut self,
        _expression: &Expression,
        _identifier: NodeId,
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Checks the type and language-defined semantics of an operator.
    fn check_operator(
        &mut self,
        _expression: &Expression,
        _operator: &OperatorRef,
        _operands: &[NodeId],
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Checks a condition's type and permitted conversion rules.
    ///
    /// This method must implement the actual Zamani condition semantics.
    /// It must not assume that arbitrary non-boolean values are truthy.
    fn check_condition(
        &mut self,
        _expression: &Expression,
        _condition: NodeId,
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Checks callable resolution, generic instantiation, argument types,
    /// arity, effects, and return type as applicable.
    fn check_call(
        &mut self,
        _expression: &Expression,
        _callee: NodeId,
        _arguments: &[NodeId],
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Checks assignment target validity, mutability, ownership, and value
    /// compatibility according to the normative language rules.
    fn check_assignment(
        &mut self,
        _expression: &Expression,
        _target: NodeId,
        _value: NodeId,
        _operator: Option<&OperatorRef>,
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Checks whether an asynchronous operation is legal in the current
    /// execution context and whether its effects are permitted.
    fn check_async(
        &mut self,
        _expression: &Expression,
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Checks extension semantics through the registered domain-extension
    /// mechanism.
    ///
    /// Returning no issues means the extension handler successfully validated
    /// the construct. Implementations must not silently accept an unknown
    /// extension merely because it is structurally representable.
    fn check_extension(
        &mut self,
        expression: &Expression,
        extension: &crate::frontend::ast::node::expressions::expression::ExpressionExtension,
    ) -> ClassicalResult<Vec<ClassicalIssue>>;

    /// Checks shared effects and authorization obligations for an expression.
    fn check_effects(
        &mut self,
        _expression: &Expression,
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Checks that the expression's referenced children are structurally
    /// available and consistent with the canonical AST graph.
    ///
    /// The AST graph store is owned by the caller. This hook permits the
    /// structural validator to consult it without duplicating AST storage.
    fn check_child_references(
        &mut self,
        _expression: &Expression,
        _children: &[NodeId],
    ) -> ClassicalResult<Vec<ClassicalIssue>> {
        Ok(Vec::new())
    }

    /// Reports an AST node identity suitable for associating a pass-level
    /// diagnostic with an expression.
    ///
    /// Production contexts may override this if the semantic diagnostic layer
    /// uses a different canonical source identity contract.
    fn fallback_node(&self, expression: &Expression) -> NodeId {
        expression.id()
    }
}

// =============================================================================
// Expression rule dispatch
// =============================================================================

/// Validates one expression's classical semantic obligations.
///
/// This method does not recursively descend the AST. The canonical AST uses
/// node references, so the caller supplies expressions in its established
/// traversal order. This avoids unbounded native call-stack growth for deeply
/// nested programs.
///
/// The caller must include every expression required by the selected traversal
/// policy and ensure that each AST node is submitted according to the pipeline's
/// deduplication rules.
pub fn validate_expression<C: ClassicalSemanticContext>(
    expression: &Expression,
    context: &mut C,
) -> ClassicalResult<Vec<ClassicalIssue>> {
    let mut issues = Vec::new();

    // Every child reference is checked against the owning AST graph. This
    // structural check is deliberately independent of domain type checking.
    let children = expression.child_node_ids();
    issues.extend(context.check_child_references(expression, &children)?);

    match expression.kind() {
        ExpressionKind::Identifier { identifier } => {
            issues.extend(context.check_name(expression, *identifier)?);
        }

        ExpressionKind::Literal { .. } => {
            // Literal parsing and source spelling are owned by the lexer and
            // parser. Literal type and value semantics are handled by the
            // shared type/constant subsystems.
        }

        ExpressionKind::Unary { operator, operand } => {
            issues.extend(context.check_operator(
                expression,
                operator,
                std::slice::from_ref(operand),
            )?);
        }

        ExpressionKind::Binary {
            left,
            operator,
            right,
        } => {
            issues.extend(context.check_operator(
                expression,
                operator,
                &[*left, *right],
            )?);
        }

        ExpressionKind::Conditional {
            condition,
            then_branch,
            else_branch,
        } => {
            issues.extend(context.check_condition(expression, *condition)?);

            // Branch type compatibility belongs to the shared type subsystem.
            // It is intentionally not reimplemented here.
            let branches = match else_branch {
                Some(other) => vec![*then_branch, *other],
                None => vec![*then_branch],
            };

            issues.extend(context.check_operator(
                expression,
                &OperatorRef::native("conditional")
                    .map_err(|error| ClassicalAnalysisFailure::ServiceFailure {
                        service: "AST operator identity".to_owned(),
                        message: error.to_string(),
                    })?,
                &branches,
            )?);
        }

        ExpressionKind::Block { .. } => {
            // Block sequencing, statement completion, local declarations, and
            // scope rules are handled by statement and symbol analysis.
        }

        ExpressionKind::Match { .. } => {
            // Pattern exhaustiveness, arm binding, guard types, and arm result
            // compatibility belong to the pattern/type subsystems.
        }

        ExpressionKind::Loop { .. } => {
            // Loop control-flow validity belongs to statement/control-flow
            // analysis. This pass does not assume that every loop terminates.
        }

        ExpressionKind::Lambda { .. } => {
            // Parameter declarations, captures, inferred callable types,
            // ownership, and effects belong to shared type/effect analysis.
        }

        ExpressionKind::Call { callee, arguments } => {
            issues.extend(context.check_call(expression, *callee, arguments)?);
        }

        ExpressionKind::MemberAccess { object, member } => {
            // Member visibility, field/method lookup, and type-directed
            // resolution belong to the symbol and type subsystems.
            issues.extend(context.check_name(expression, *member)?);

            // `object` is retained in the canonical AST graph and checked by
            // the shared structural/type analysis.
            let _ = object;
        }

        ExpressionKind::Index { object, index } => {
            // Indexability, index type, bounds policy, and any checked runtime
            // obligations belong to shared type/constraint analysis.
            let _ = (object, index);
        }

        ExpressionKind::Range { start, end, .. } => {
            // Range endpoint compatibility and range semantics are defined by
            // the shared type/constant subsystem.
            let _ = (start, end);
        }

        ExpressionKind::Array { .. } | ExpressionKind::Tuple { .. } => {
            // Element inference, homogeneous/heterogeneous collection rules,
            // and symbolic lengths belong to the canonical type subsystem.
        }

        ExpressionKind::Struct { type_name, .. } => {
            issues.extend(context.check_name(expression, *type_name)?);
        }

        ExpressionKind::Cast {
            expression: value,
            target_type,
        } => {
            // Cast legality and target type resolution belong to the shared
            // type subsystem. A cast must not be assumed valid merely because
            // both syntax nodes exist.
            let _ = (value, target_type);
        }

        ExpressionKind::TypeAscription {
            expression: value,
            type_node,
        } => {
            // The shared type subsystem must resolve the ascribed type and
            // check compatibility with the expression.
            let _ = (value, type_node);
        }

        ExpressionKind::Assignment { target, value } => {
            issues.extend(context.check_assignment(
                expression,
                *target,
                *value,
                None,
            )?);
        }

        ExpressionKind::CompoundAssignment {
            target,
            operator,
            value,
        } => {
            issues.extend(context.check_assignment(
                expression,
                *target,
                *value,
                Some(operator),
            )?);

            issues.extend(context.check_operator(
                expression,
                operator,
                &[*target, *value],
            )?);
        }

        ExpressionKind::Try { expression: value } => {
            // The shared type/effect system validates propagation semantics
            // against the enclosing function's declared result contract.
            let _ = value;
        }

        ExpressionKind::Await { .. }
        | ExpressionKind::Async { .. }
        | ExpressionKind::Spawn { .. } => {
            issues.extend(context.check_async(expression)?);
        }

        ExpressionKind::Construction {
            type_name,
            arguments,
        } => {
            issues.extend(context.check_name(expression, *type_name)?);
            issues.extend(context.check_call(
                expression,
                *type_name,
                arguments,
            )?);
        }

        ExpressionKind::Extension(extension) => {
            issues.extend(context.check_extension(expression, extension)?);
        }

        // ExpressionKind is non-exhaustive. A future AST variant must not be
        // silently treated as fully checked by this validator.
        _ => {
            issues.push(ClassicalIssue::new(
                ClassicalIssueKind::UnsupportedExtension,
                context.fallback_node(expression),
                "classical.expression.unhandled",
                "This expression kind has no registered classical semantic rule",
            ));
        }
    }

    // Shared effect checking runs after the expression-specific rule so that
    // effects introduced by a call, assignment, or extension are not skipped.
    issues.extend(context.check_effects(expression)?);

    Ok(issues)
}

// =============================================================================
// Pass orchestration
// =============================================================================

/// Runs classical semantic validation over an already traversed expression
/// sequence.
///
/// The sequence must be supplied by the canonical AST traversal subsystem.
/// This function does not invent a second AST walker or assume that expression
/// storage is flat in the actual AST.
///
/// Issues preserve traversal order. The canonical diagnostic ordering module
/// remains responsible for final repository-wide sorting and deduplication.
///
/// If a configured budget is exhausted, the pass returns an incomplete result
/// rather than reporting semantic success.
pub fn validate_classical<C, I>(
    expressions: I,
    context: &mut C,
    policy: ClassicalValidationPolicy,
) -> ClassicalResult<ClassicalValidation>
where
    C: ClassicalSemanticContext,
    I: IntoIterator<Item = Expression>,
{
    policy.validate()?;

    let mut issues = Vec::new();
    let mut expressions_checked = 0usize;

    for expression in expressions {
        if let Some(maximum) = policy.max_expressions {
            if expressions_checked >= maximum {
                let node = context.fallback_node(&expression);

                return Ok(ClassicalValidation::incomplete(
                    expressions_checked,
                    issues,
                    node,
                    format!(
                        "classical expression budget exhausted at {maximum}; \
                         analysis did not establish complete semantic validity"
                    ),
                ));
            }
        }

        let expression_issues = validate_expression(&expression, context)?;
        expressions_checked = expressions_checked.checked_add(1).ok_or_else(|| {
            ClassicalAnalysisFailure::ServiceFailure {
                service: "classical pass accounting".to_owned(),
                message: "expression count overflow".to_owned(),
            }
        })?;

        issues.extend(expression_issues);

        if let Some(maximum) = policy.max_issues {
            if issues.len() >= maximum {
                return Ok(ClassicalValidation::incomplete(
                    expressions_checked,
                    issues,
                    context.fallback_node(&expression),
                    format!(
                        "classical diagnostic budget exhausted at {maximum}; \
                         additional semantic issues may remain"
                    ),
                ));
            }
        }
    }

    Ok(ClassicalValidation::with_issues(
        expressions_checked,
        issues,
    ))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unrestricted_policy_has_no_machine_capacity_limits() {
        let policy = ClassicalValidationPolicy::unrestricted();

        assert_eq!(policy.max_expressions, None);
        assert_eq!(policy.max_issues, None);
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn zero_expression_budget_is_rejected() {
        let policy = ClassicalValidationPolicy {
            max_expressions: Some(0),
            max_issues: None,
        };

        assert!(matches!(
            policy.validate(),
            Err(ClassicalAnalysisFailure::InvalidPolicy { .. })
        ));
    }

    #[test]
    fn zero_issue_budget_is_rejected() {
        let policy = ClassicalValidationPolicy {
            max_expressions: None,
            max_issues: Some(0),
        };

        assert!(matches!(
            policy.validate(),
            Err(ClassicalAnalysisFailure::InvalidPolicy { .. })
        ));
    }

    #[test]
    fn clean_result_requires_complete_analysis() {
        let complete = ClassicalValidation::success(0);
        assert!(complete.is_clean());

        let incomplete = ClassicalValidation::incomplete(
            0,
            Vec::new(),
            // A concrete NodeId is intentionally supplied by AST-level
            // integration tests; this test exercises only the result model.
            panic_node_id_not_available(),
            "test analysis stopped",
        );

        assert!(!incomplete.is_clean());
        assert_eq!(
            incomplete.completion,
            ClassicalCompletion::Incomplete
        );
    }

    // This helper is intentionally replaced by the canonical NodeId
    // construction used in the repository's AST test utilities before freeze.
    fn panic_node_id_not_available() -> NodeId {
        panic!(
            "Replace with the canonical NodeId test constructor \
             from src/frontend/ast/node/node_id.rs before enabling this test"
        )
    }
}
