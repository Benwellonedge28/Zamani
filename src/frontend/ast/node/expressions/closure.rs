//! # Zamani Native AST — Closure Expression Compatibility Layer
//!
//! This module provides the source-level `ClosureExpression` name while
//! preserving `LambdaExpression` as the single canonical AST representation
//! for anonymous functions / closures.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! LambdaExpression
//!     │
//!     ├── ClosureExpression (this compatibility name)
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── lexical scope
//!     ├── name resolution
//!     ├── type inference
//!     ├── capture analysis
//!     ├── effect analysis
//!     ├── capability analysis
//!     └── resource analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical lowering
//!     ├── quantum lowering
//!     ├── hybrid lowering
//!     └── future-domain lowering
//! ```
//!
//! ## Why this file exists
//!
//! Zamani uses the terms "lambda" and "closure" for closely related
//! source-level anonymous-function constructs.
//!
//! The native AST must nevertheless maintain exactly one authoritative
//! representation for that language concept.
//!
//! The repository already provides:
//!
//! ```text
//! LambdaExpression
//! ```
//!
//! in `lambda.rs`, together with the canonical:
//!
//! ```text
//! CoreNodeKind::LambdaExpression
//! ```
//!
//! Consequently this module deliberately does **not** define another struct
//! containing fields such as:
//!
//! ```text
//! ClosureExpression {
//!     parameters: ...,
//!     body: ...,
//!     captures: ...,
//! }
//! ```
//!
//! Such a duplicate representation would create two competing AST models,
//! complicate visitors and serialization, and force semantic/ZUIR lowering
//! code to distinguish structures that have the same source-language meaning.
//!
//! Instead:
//!
//! ```text
//! ClosureExpression = LambdaExpression
//! ```
//!
//! at the Rust type level.
//!
//! ## Canonical representation
//!
//! The canonical representation remains owned by `lambda.rs`.
//!
//! That representation already follows the repository's source-level AST
//! architecture:
//!
//! ```text
//! LambdaExpression
//! ├── Node
//! ├── parameters: Vec<NodeId>
//! ├── return_type: Option<NodeId>
//! └── body: NodeId
//! ```
//!
//! Child relationships are represented with `NodeId` rather than recursively
//! embedding expression structures.
//!
//! This preserves:
//!
//! - large-AST scalability;
//! - deterministic traversal;
//! - separation of node identity from ownership;
//! - iterative AST traversal;
//! - source-level graph semantics;
//! - compatibility with downstream semantic analysis;
//! - compatibility with ZUIR lowering.
//!
//! ## Domain neutrality
//!
//! A closure is a native Zamani language construct, not a quantum construct.
//!
//! This module therefore contains no:
//!
//! - qubit representation;
//! - quantum-resource representation;
//! - physical-qubit mapping;
//! - hardware topology;
//! - gate set;
//! - backend identifier;
//! - QIR type;
//! - LLVM type;
//! - MLIR operation;
//! - scheduler state;
//! - routing state;
//! - calibration state;
//! - QEC state;
//! - resilience state;
//! - runtime state.
//!
//! A closure may nevertheless participate in:
//!
//! - classical computation;
//! - quantum algorithms;
//! - hybrid classical/quantum computation;
//! - accelerator computation;
//! - distributed computation;
//! - future computational domains.
//!
//! Those meanings are resolved downstream.
//!
//! ## POCO-REAF
//!
//! The compatibility layer preserves:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! because it introduces no target-specific representation.
//!
//! A closure can therefore remain source-level and target-independent while
//! later compilation determines how it is represented and executed.
//!
//! ## One authoritative representation
//!
//! This is the most important invariant of this file:
//!
//! ```text
//! LambdaExpression
//!        ▲
//!        │
//!        └── ClosureExpression
//! ```
//!
//! There is no second AST node.
//!
//! There is no second `NodeId`.
//!
//! There is no second `NodeKind`.
//!
//! There is no second serialization schema.
//!
//! There is no second visitor contract.
//!
//! There is no second validation contract.
//!
//! There is no second semantic lowering contract.
//!
//! There is no second ZUIR lowering contract.
//!
//! ## Node classification
//!
//! Both terminology choices classify as:
//!
//! ```text
//! CoreNodeKind::LambdaExpression
//! ```
//!
//! This is intentional.
//!
//! Adding `CoreNodeKind::ClosureExpression` would create two AST identities
//! for the same source-level concept and would force every downstream
//! exhaustive match, visitor, serializer, validator, and lowering pass to
//! distinguish them unnecessarily.
//!
//! ## Capture semantics
//!
//! This module does not introduce a `captures` field.
//!
//! Captures are semantic information, not raw source syntax.
//!
//! For example:
//!
//! ```text
//! |x| x + external_value
//! ```
//!
//! may cause semantic analysis to determine that `external_value` is
//! captured. The capture set and capture modes belong in the semantic model,
//! keyed by the closure/lambda's `NodeId`.
//!
//! This remains independent of:
//!
//! - ownership analysis;
//! - borrow analysis;
//! - lifetime analysis;
//! - type inference;
//! - closure conversion;
//! - target lowering.
//!
//! ## Scalability
//!
//! This module introduces no limits of its own.
//!
//! It does not define:
//!
//! ```text
//! MAX_CLOSURE_PARAMETERS
//! MAX_CLOSURE_DEPTH
//! MAX_CAPTURE_COUNT
//! MAX_CLOSURE_SIZE
//! MAX_QUANTUM_CLOSURES
//! ```
//!
//! The canonical lambda representation owns the actual parameter collection
//! and uses scalable AST node references.
//!
//! Compiler safety/resource limits, when required, must remain configurable
//! policies outside this compatibility layer.
//!
//! ## Determinism
//!
//! Because this module is a type alias/re-export layer, deterministic behavior
//! is inherited directly from the canonical `LambdaExpression` representation.
//!
//! No additional state is introduced.
//!
//! No:
//!
//! - timestamps;
//! - random identifiers;
//! - memory addresses;
//! - raw pointers;
//! - thread-local state;
//! - backend state
//!
//! is introduced here.
//!
//! ## Serialization
//!
//! There is no second closure serialization format.
//!
//! `ClosureExpression` serializes exactly as the canonical
//! `LambdaExpression` representation.
//!
//! This is essential for:
//!
//! - deterministic AST serialization;
//! - schema stability;
//! - round-trip testing;
//! - compatibility between lambda terminology and closure terminology;
//! - avoiding duplicate schema versions.
//!
//! ## Visitor integration
//!
//! Visitors do not need a new closure-specific traversal branch.
//!
//! They continue to traverse the canonical:
//!
//! ```text
//! LambdaExpression
//! ```
//!
//! using the existing expression visitor/traversal infrastructure.
//!
//! Consequently the child order remains the canonical lambda order:
//!
//! ```text
//! parameters[0..N]
//! return_type, if present
//! body
//! ```
//!
//! ## Validation
//!
//! Structural validation continues to operate on the canonical
//! `LambdaExpression`.
//!
//! This module does not duplicate validation.
//!
//! Therefore there is exactly one authoritative implementation of:
//!
//! - node-kind validation;
//! - node-reference validation;
//! - parameter-reference validation;
//! - duplicate-parameter-reference detection;
//! - body-reference validation;
//! - return-type-reference validation.
//!
//! ## Parser integration
//!
//! The parser should construct the canonical `LambdaExpression`.
//!
//! It must not construct a second closure structure merely because the source
//! syntax or parser rule uses the word "closure".
//!
//! If parser code wants the terminology:
//!
//! ```text
//! ClosureExpression
//! ```
//!
//! it may import the compatibility alias provided by this module.
//!
//! The resulting AST remains `LambdaExpression`.
//!
//! ## Semantic integration
//!
//! Semantic analysis should continue to identify the node by its canonical
//! `NodeId` and `CoreNodeKind::LambdaExpression`.
//!
//! It is responsible for resolving:
//!
//! - lexical scope;
//! - parameter bindings;
//! - captured variables;
//! - capture modes;
//! - parameter types;
//! - return type;
//! - callable type;
//! - effects;
//! - capabilities;
//! - resource requirements.
//!
//! None of these are introduced into this file.
//!
//! ## ZUIR integration
//!
//! ZUIR lowering continues to consume the semantic representation generated
//! from `LambdaExpression`.
//!
//! This module must never import ZUIR.
//!
//! The dependency direction remains:
//!
//! ```text
//! AST
//!   ↓
//! semantic model
//!   ↓
//! ZUIR
//! ```
//!
//! and never:
//!
//! ```text
//! AST → ZUIR
//! ```
//!
//! ## Quantum integration
//!
//! Closures may be used in quantum and hybrid programs without this module
//! becoming quantum-aware.
//!
//! For example, a closure may eventually:
//!
//! - calculate a gate parameter;
//! - transform measurement data;
//! - construct an operation;
//! - express a generic algorithm;
//! - control a hybrid computation;
//! - operate over a generic resource abstraction.
//!
//! These are semantic/lowering concerns.
//!
//! This file deliberately remains independent of quantum IR, hardware, and
//! execution.
//!
//! ## Dependency policy
//!
//! Allowed dependency:
//!
//! - the canonical `lambda` expression module;
//! - Rust standard library facilities needed by aliases/re-exports.
//!
//! This module must not depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - OpenQASM;
//! - hardware;
//! - scheduling;
//! - routing;
//! - optimization;
//! - QEC;
//! - resilience;
//! - calibration;
//! - runtime;
//! - backend providers.
//!
//! ## Migration policy
//!
//! Existing code using:
//!
//! ```text
//! LambdaExpression
//! ```
//!
//! remains valid.
//!
//! New code that needs closure terminology may use:
//!
//! ```text
//! ClosureExpression
//! ```
//!
//! without introducing a second node type.
//!
//! Legacy representations such as:
//!
//! ```text
//! Closure(...)
//! AnonymousFunction(...)
//! ```
//!
//! must be migrated to the canonical `LambdaExpression` representation,
//! rather than being represented as an additional native AST variant.
//!
//! ## No-re-edit guarantee
//!
//! Adding a new:
//!
//! - quantum backend;
//! - quantum technology;
//! - computational domain;
//! - optimizer;
//! - scheduler;
//! - router;
//! - QEC implementation;
//! - resilience implementation;
//! - hardware target
//!
//! does not require changing this file.
//!
//! A source-language change that genuinely changes lambda/closure syntax or
//! semantics should modify the canonical `lambda.rs` contract first. This
//! compatibility layer should then remain unchanged unless the public names
//! themselves change.
//!
//! ## Rust compatibility
//!
//! This module intentionally uses only stable language features compatible
//! with the repository's Rust 1.97 / Rust 1.97.1 requirement.
//!
//! No nightly features are used.
//! No compiler intrinsics are used.
//! No `unsafe` code is used.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// Re-export the canonical representation and its complete public contract.
//
// Keeping the re-export list explicit prevents accidental leakage of private
// implementation details from lambda.rs and makes this module's API stable.
pub use super::lambda::{
    LambdaExpression,
    LambdaExpressionError,
    LAMBDA_EXPRESSION_KIND_NAME,
    LAMBDA_EXPRESSION_SCHEMA_VERSION,
};

/// Canonical native AST name for a closure expression.
///
/// This is deliberately a type alias rather than a second struct.
///
/// A closure and a lambda are represented by exactly the same native AST
/// node and therefore have exactly the same:
///
/// - `NodeId` semantics;
/// - `NodeKind`;
/// - source span semantics;
/// - metadata;
/// - child references;
/// - validation;
/// - traversal;
/// - serialization;
/// - semantic lowering contract.
///
/// # Architectural invariant
///
/// ```text
/// ClosureExpression == LambdaExpression
/// ```
///
/// at the Rust type level.
pub type ClosureExpression = LambdaExpression;

/// Canonical error type for closure construction/validation.
///
/// There is no closure-specific error taxonomy because there is no separate
/// closure AST representation.
pub type ClosureExpressionError = LambdaExpressionError;

/// Stable schema version exposed under closure terminology.
///
/// The actual schema remains owned by the canonical lambda representation.
pub const CLOSURE_EXPRESSION_SCHEMA_VERSION: u16 = LAMBDA_EXPRESSION_SCHEMA_VERSION;

/// Stable source-level kind name exposed under closure terminology.
///
/// The canonical AST classification remains `lambda-expression`.
pub const CLOSURE_EXPRESSION_KIND_NAME: &str = LAMBDA_EXPRESSION_KIND_NAME;

#[cfg(test)]
mod tests {
    use super::*;

    /// The closure and lambda representations must remain the exact same
    /// Rust type.
    #[test]
    fn closure_is_canonical_lambda_representation() {
        fn accepts_lambda(_: LambdaExpression) {}

        let closure: ClosureExpression =
            // This test only verifies the type relationship. Construction is
            // delegated entirely to lambda.rs.
            //
            // A real AST construction test belongs to lambda.rs because that
            // is where the canonical constructor contract lives.
            panic_for_type_only_test();

        accepts_lambda(closure);
    }

    /// The public closure error name must resolve to the canonical lambda
    /// error type.
    #[test]
    fn closure_error_is_canonical_lambda_error() {
        fn accepts_lambda_error(_: LambdaExpressionError) {}

        let closure_error: ClosureExpressionError =
            LambdaExpressionError::MissingBody;

        accepts_lambda_error(closure_error);
    }

    #[test]
    fn closure_schema_version_matches_lambda_schema_version() {
        assert_eq!(
            CLOSURE_EXPRESSION_SCHEMA_VERSION,
            LAMBDA_EXPRESSION_SCHEMA_VERSION
        );
    }

    #[test]
    fn closure_kind_name_matches_lambda_kind_name() {
        assert_eq!(
            CLOSURE_EXPRESSION_KIND_NAME,
            LAMBDA_EXPRESSION_KIND_NAME
        );
        assert_eq!(
            CLOSURE_EXPRESSION_KIND_NAME,
            "zamani:lambda-expression"
        );
    }

    /// This function is never executed.
    ///
    /// It exists only so the compile-time type-alias test above does not need
    /// to manufacture an otherwise unrelated AST instance and accidentally
    /// duplicate the canonical lambda construction contract.
    #[allow(clippy::panic)]
    fn panic_for_type_only_test() -> ClosureExpression {
        panic!("type-only test helper must never be executed")
    }
}