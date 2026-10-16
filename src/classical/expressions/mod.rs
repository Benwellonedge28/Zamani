
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Classical Expressions
//!
//! File: `src/classical/expressions/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! Status: Module boundary defined; compilation and integration verification
//! are required before this module can be marked production-ready.
//!
//! ## Purpose
//!
//! Provide the public module boundary for classical processing of Zamani
//! expressions. This module composes the existing expression integration,
//! operator registry, evaluation-order analysis, constant analysis,
//! constant folding, and expression simplification modules.
//!
//! ## Authority and canonical ownership
//!
//! Source syntax:
//!   - grammar/expressions/expressions.g4
//!   - grammar/expressions/compile-time.g4
//!   - grammar/declarations/constants.g4
//!
//! Canonical source AST:
//!   - crate::frontend::ast::node::expressions::expression
//!   - crate::frontend::ast::node::expressions::literal
//!   - crate::frontend::ast::node::node_id
//!
//! Semantic authority:
//!   - the canonical semantic-analysis and type/effect systems;
//!   - explicit operator and extension contracts;
//!   - the language's constant-evaluation and numerical semantics.
//!
//! Canonical IR:
//!   - the repository's existing IR boundaries, including
//!     quantum::ir::classical where applicable.
//!
//! This module does not create a replacement AST, semantic type system,
//! classical IR, quantum IR, or runtime.
//!
//! ## Module ownership
//!
//! - `expression`: canonical AST integration and expression-graph traversal.
//! - `operators`: extensible syntax-to-canonical-operator registry.
//! - `evaluation_order`: structural evaluation-order planning.
//! - `constant`: constant-expression eligibility analysis.
//! - `folding`: semantic-adapter-driven constant folding.
//! - `simplification`: semantic-adapter-driven equivalent-node proposals.
//!
//! Each module remains the authority for its own behavior. This file only
//! composes the modules and exposes their intended APIs.
//!
//! ## Dependency direction
//!
//! ```text
//! grammar
//!   |
//!   v
//! canonical frontend AST
//!   |
//!   v
//! expression
//!   |---------------------|
//!   v                     v
//! operators        evaluation_order
//!   |                     |
//!   +----------+----------+
//!              |
//!              v
//!       semantic analysis
//!              |
//!       +------+-------+
//!       |              |
//!       v              v
//!    constant       simplification
//!       |
//!       v
//!    folding
//!       |
//!       v
//! canonical semantic representation / IR
//! ```
//!
//! The diagram expresses responsibilities, not permission to bypass
//! dependencies. In particular, folding and simplification must only perform
//! operations authorized by semantic analysis.
//!
//! ## POCO-REAF and resource policy
//!
//! This module introduces no fixed language-level limit on expression count,
//! AST size, operand count, integer width, collection size, tensor rank,
//! hardware count, or execution-target count.
//!
//! Finite memory, compilation time, and host resources remain real
//! operational constraints. Any compiler work budget must be explicit and
//! must not be confused with the set of valid Zamani programs.
//!
//! ## Compatibility policy
//!
//! Submodules are public so existing and future consumers can depend on their
//! documented module paths. The facade re-exports selected types and entry
//! points for discoverability. Adding a new submodule must not silently change
//! the semantics or identity of existing public types.
//!
//! ## Safety
//!
//! Safe Rust only. No hardware discovery, host execution, I/O, or mutation of
//! the canonical AST occurs in this module.
//!
//! ## Freeze rule
//!
//! This module may be marked verified only after all declared submodules
//! compile together, the crate exposes the canonical frontend AST, tests pass,
//! and public API compatibility has been reviewed.

// ---------------------------------------------------------------------------
// Canonical expression integration
// ---------------------------------------------------------------------------

pub mod expression;

pub use expression::{
    validate_expression,
    walk_expression_graph,
    Expression,
    ExpressionError,
    ExpressionExtension,
    ExpressionKind,
    ExpressionResult,
    ExpressionValidationPolicy,
    ExpressionWalkError,
    ExpressionWalkResult,
    NodeId,
    OperatorRef,
    EXPRESSION_SCHEMA_VERSION,
};

// ---------------------------------------------------------------------------
// Operator syntax registry
// ---------------------------------------------------------------------------

pub mod operators;

pub use operators::{
    OperatorFixity,
    OperatorRegistry,
    OperatorRegistryError,
    OperatorRegistryResult,
};

// ---------------------------------------------------------------------------
// Evaluation order and control-flow boundaries
// ---------------------------------------------------------------------------

pub mod evaluation_order;

pub use evaluation_order::{
    plan_expression,
    requires_extension_evaluation_contract,
    BinaryEvaluation,
    DeferredKind,
    EvaluationPlan,
    ShortCircuitClassifier,
    ShortCircuitKind,
};

// ---------------------------------------------------------------------------
// Constant-expression eligibility analysis
// ---------------------------------------------------------------------------

pub mod constant;

pub use constant::{
    analyze_constant_expression,
    ConstantAnalysis,
    ConstantAnalysisError,
    ConstantAnalysisResult,
    ConstantIssue,
    ConstantSemantics,
    ResolvedConstantNode,
};

// ---------------------------------------------------------------------------
// Constant folding
// ---------------------------------------------------------------------------

pub mod folding;

pub use folding::{
    fold_one,
    ConstantFoldSemantics,
    DeferredReason,
    FoldBudget,
    FoldError,
    FoldOutcome,
    FoldResult,
    FoldSite,
    FoldValueTable,
    NotConstantReason,
};

// ---------------------------------------------------------------------------
// Semantics-authorized expression simplification
// ---------------------------------------------------------------------------

pub mod simplification;

pub use simplification::{
    simplify_expression_graph,
    ResolvedSimplificationNode,
    SimplificationBudget,
    SimplificationBudgetError,
    SimplificationError,
    SimplificationReason,
    SimplificationRecord,
    SimplificationReport,
    SimplificationResult,
    SimplificationSemantics,
    SimplifiedChild,
};

// ---------------------------------------------------------------------------
// Module-boundary contract tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_expression_types_are_reexported() {
        // Compile-time API contract: these names resolve to the canonical
        // frontend AST types through the classical expression boundary.
        let _: Option<Expression> = None;
        let _: Option<NodeId> = None;
        let _: Option<OperatorRef> = None;
    }

    #[test]
    fn expression_modules_are_exposed() {
        // This verifies module composition, not the correctness of semantic
        // rewrites or the production readiness of downstream implementations.
        let _ = core::any::type_name::<OperatorRegistry>();
        let _ = core::any::type_name::<EvaluationPlan>();
        let _ = core::any::type_name::<ConstantAnalysis>();
        let _ = core::any::type_name::<FoldBudget>();
        let _ = core::any::type_name::<SimplificationBudget>();
    }
}
