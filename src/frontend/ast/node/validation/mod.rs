//! # Zamani Frontend AST — Validation Boundary
//!
//! `src/frontend/ast/node/validation/mod.rs`
//!
//! Canonical public integration boundary for structural validation of the
//! native Zamani frontend AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer / parser
//!      │
//!      ▼
//! ┌──────────────────────────────┐
//! │ Native Zamani AST            │
//! │                              │
//! │ NodeId                       │
//! │ NodeKind                     │
//! │ Node                         │
//! │ Span                         │
//! │ metadata                     │
//! │ source-level relationships   │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//! ┌─────────────────────────────────────────┐
//! │ validation/                             │
//! │                                         │
//! │ invariants.rs                           │
//! │ source_ranges.rs                        │
//! │ recursion.rs                            │
//! │ structural.rs                           │
//! │ validation_error.rs                     │
//! └──────────────────┬──────────────────────┘
//!                    │
//!                    ▼
//!             semantic analysis
//!                    │
//!                    ▼
//!             Semantic Model
//!                    │
//!                    ▼
//!                   ZUIR
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!      classical   quantum     HDL
//!         IR         IR        IR
//!          │         │         │
//!          └─────────┼─────────┘
//!                    ▼
//!              target lowering
//!                    │
//!                    ▼
//!                 hardware
//! ```
//!
//! ## Purpose
//!
//! This module is the **single public aggregation boundary** for native AST
//! validation.
//!
//! It owns:
//!
//! - validation-module composition;
//! - stable public re-exports;
//! - the public validation API surface;
//! - documentation of validation-layer boundaries;
//! - compatibility aliases when necessary;
//! - compile-time enforcement that this module contains no unsafe Rust.
//!
//! It does **not** implement:
//!
//! - AST traversal;
//! - recursion;
//! - cycle detection;
//! - revisit detection;
//! - source-range checking;
//! - node invariant checking;
//! - semantic analysis;
//! - type checking;
//! - resource allocation;
//! - capability resolution;
//! - quantum semantics;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - resilience;
//! - hardware validation;
//! - backend selection;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR lowering.
//!
//! Those responsibilities remain in their authoritative layers.
//!
//! ## Critical architectural rule
//!
//! This file must remain a **facade**, not a second validation engine.
//!
//! ```text
//! validation/mod.rs
//!       │
//!       ├── invariants.rs
//!       ├── source_ranges.rs
//!       ├── recursion.rs
//!       ├── structural.rs
//!       └── validation_error.rs
//! ```
//!
//! Each implementation module owns one responsibility.
//!
//! There must be no competing validation implementation in this file.
//!
//! ## POCO-REAF
//!
//! Validation is independent of computational scale.
//!
//! Nothing in this module assumes:
//!
//! - a maximum AST node count;
//! - a maximum source-file size;
//! - a maximum module count;
//! - a maximum expression count;
//! - a maximum statement count;
//! - a maximum declaration count;
//! - a maximum qubit count;
//! - a maximum register size;
//! - a maximum operation count;
//! - a maximum machine size;
//! - a maximum number of processors;
//! - a maximum number of devices;
//! - a maximum topology size;
//! - a maximum number of computational domains.
//!
//! The absence of a limit in this module is intentional.
//!
//! Operational safety limits may be supplied by callers through the validation
//! and traversal configuration APIs. Such limits are compiler/service policies,
//! not Zamani language semantics.
//!
//! Therefore:
//!
//! ```text
//! tiny AST
//!    │
//!    ├── same validation boundary
//!    │
//! very large AST
//!    │
//!    ├── same validation boundary
//!    │
//! future AST scale
//!    │
//!    └── same validation boundary
//! ```
//!
//! "Infinity" in the POCO-REAF architecture means that this layer introduces no
//! artificial finite language or computational-resource bound. Actual
//! validation remains bounded by representable values and resources available
//! to the executing compiler process.
//!
//! ## Domain neutrality
//!
//! This module deliberately contains no knowledge of:
//!
//! - quantum gates;
//! - qubits;
//! - quantum hardware;
//! - quantum topology;
//! - QEC;
//! - noise models;
//! - calibration;
//! - routing;
//! - scheduling;
//! - CPU instructions;
//! - GPU instructions;
//! - FPGA instructions;
//! - ASIC instructions;
//! - vendor SDKs;
//! - backend APIs;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR.
//!
//! The same validation facade therefore serves:
//!
//! - classical programs;
//! - quantum programs;
//! - hybrid programs;
//! - HDL programs;
//! - accelerator programs;
//! - distributed programs;
//! - AI programs;
//! - future computational domains.
//!
//! A new computational domain must not require this module to be modified
//! merely because that domain exists.
//!
//! ## Validation layering
//!
//! Validation has intentionally separate responsibilities:
//!
//! ```text
//! Layer 0 — AST construction
//!       │
//!       ▼
//! Layer 1 — local invariants
//!       │
//!       ├── node identity
//!       ├── node kind
//!       └── direct references
//!       │
//!       ▼
//! Layer 2 — source coordinates
//!       │
//!       ├── range ordering
//!       ├── source bounds
//!       ├── UTF-8 boundaries
//!       └── optional containment
//!       │
//!       ▼
//! Layer 3 — recursion / graph structure
//!       │
//!       ├── cycles
//!       ├── revisits
//!       ├── traversal limits
//!       └── cancellation
//!       │
//!       ▼
//! Layer 4 — complete structural validation
//!       │
//!       ▼
//! Layer 5 — semantic analysis
//!       │
//!       ▼
//! Semantic Model
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! `mod.rs` exposes these layers without collapsing them.
//!
//! ## Canonical traversal
//!
//! The canonical iterative AST walker is owned by:
//!
//! ```text
//! src/frontend/ast/node/traversal/walk.rs
//! ```
//!
//! Validation must reuse that walker.
//!
//! This module must never introduce a second traversal algorithm.
//!
//! The canonical walker owns:
//!
//! - iterative traversal;
//! - child ordering;
//! - active-path cycle detection;
//! - completed-node revisit detection;
//! - cancellation;
//! - node budgets;
//! - edge budgets;
//! - depth budgets;
//! - traversal statistics;
//! - provider errors;
//! - visitor errors.
//!
//! The validation facade merely exposes validators which consume that
//! infrastructure.
//!
//! ## Source-range boundary
//!
//! Source-range validation is owned by:
//!
//! ```text
//! source_ranges.rs
//! ```
//!
//! That module consumes the canonical `Span` representation supplied by the
//! source subsystem. It does not define a second span abstraction.
//!
//! This is important because the repository's source infrastructure already
//! establishes the canonical coordinate model:
//!
//! ```text
//! SourceId + UTF-8 byte offset
//! ```
//!
//! and half-open ranges:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Source-range validation therefore remains independent from semantic and
//! hardware layers.
//!
//! ## Invariant boundary
//!
//! Local structural invariants are owned by:
//!
//! ```text
//! invariants.rs
//! ```
//!
//! This includes:
//!
//! - valid node identity;
//! - node identity consistency;
//! - node-kind structural validity;
//! - extension-kind structural validity;
//! - self-reference rejection;
//! - direct child reference validity.
//!
//! It does not perform recursive traversal.
//!
//! ## Recursion boundary
//!
//! Recursion and structural-depth policy are owned by:
//!
//! ```text
//! recursion.rs
//! ```
//!
//! The implementation delegates traversal to the canonical iterative walker.
//!
//! There is deliberately no language-level recursion constant.
//!
//! ```text
//! MAX_RECURSION
//! MAX_AST_DEPTH
//! MAX_NESTING
//! ```
//!
//! must never appear as semantic restrictions in this subsystem.
//!
//! ## Complete structural validation
//!
//! Complete graph-level validation is owned by:
//!
//! ```text
//! structural.rs
//! ```
//!
//! That module combines:
//!
//! - canonical node resolution;
//! - local validation;
//! - graph traversal;
//! - source relationship validation;
//! - configurable operational policies;
//! - structural statistics.
//!
//! It must not be duplicated here.
//!
//! ## Error boundary
//!
//! The format-independent validation error model is owned by:
//!
//! ```text
//! validation_error.rs
//! ```
//!
//! Provider errors remain provider errors.
//!
//! Traversal errors remain traversal errors.
//!
//! Validation-specific errors remain validation-specific errors.
//!
//! This prevents the AST layer from forcing every downstream compiler service
//! into one global diagnostic representation.
//!
//! ## Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! source infrastructure
//!       │
//!       ▼
//! AST foundational types
//!       │
//!       ├── NodeId
//!       ├── NodeKind
//!       ├── Node
//!       └── Span
//!       │
//!       ▼
//! AST traversal
//!       │
//!       ├── children
//!       ├── walk
//!       └── visitors
//!       │
//!       ▼
//! AST validation
//!       │
//!       ├── invariants
//!       ├── source_ranges
//!       ├── recursion
//!       ├── structural
//!       └── validation_error
//!       │
//!       ▼
//! semantic analysis
//!       │
//!       ▼
//! semantic model
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! No validation module may depend upward on semantic analysis or downstream
//! execution infrastructure.
//!
//! ## Forbidden dependencies
//!
//! This module and its direct validation children must not depend on:
//!
//! - `src/semantic`;
//! - `src/compiler` implementation details;
//! - `src/quantum/ir`;
//! - `src/quantum/hardware`;
//! - `src/quantum/error_correction`;
//! - `src/quantum/zqn`;
//! - quantum routing;
//! - quantum scheduling;
//! - calibration;
//! - resilience;
//! - runtime execution;
//! - backend implementations;
//! - vendor SDKs;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - operating-system handles.
//!
//! This keeps the native AST validation boundary source-oriented and reusable.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! ```text
//! lexer
//!   │
//!   ▼
//! parser
//!   │
//!   ▼
//! native AST
//!   │
//!   ▼
//! validation::validate(...)
//! ```
//!
//! The parser may use local invariant helpers during construction, but complete
//! structural validation occurs after the AST graph is available.
//!
//! ### AST storage
//!
//! AST storage implements the canonical:
//!
//! ```text
//! AstWalkProvider
//! ```
//!
//! and therefore supplies:
//!
//! ```text
//! NodeId → Node
//! NodeId → direct children
//! ```
//!
//! Validation consumes those interfaces and does not dictate storage.
//!
//! ### Semantic analysis
//!
//! ```text
//! validated AST
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! Semantic Model
//! ```
//!
//! Semantic analysis must not be implemented in this module.
//!
//! ### ZUIR
//!
//! ```text
//! validated AST
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//! ```
//!
//! Validation must not directly construct ZUIR.
//!
//! ### Quantum compilation
//!
//! Quantum validation beyond source/AST structure belongs downstream.
//!
//! For example, this layer does not determine:
//!
//! - whether an operation is mathematically valid;
//! - whether a target supports an operation;
//! - whether enough physical resources exist;
//! - whether a topology permits an interaction;
//! - whether routing is possible;
//! - whether scheduling is possible;
//! - whether a QEC strategy is feasible;
//! - whether calibration exists.
//!
//! Those are semantic/domain/target concerns.
//!
//! ## Public API policy
//!
//! Consumers should normally import validation functionality from this module:
//!
//! ```text
//! frontend::ast::node::validation::...
//! ```
//!
//! rather than depending on implementation-file paths when possible.
//!
//! This makes the internal organization replaceable while retaining a stable
//! facade.
//!
//! ## Module ownership
//!
//! The following files are authoritative:
//!
//! ```text
//! invariants.rs
//!     Local node/reference invariants.
//!
//! source_ranges.rs
//!     Source-coordinate and source-range validation.
//!
//! recursion.rs
//!     Recursion/depth policy using canonical traversal.
//!
//! structural.rs
//!     Complete graph-level structural validation.
//!
//! validation_error.rs
//!     Shared validation-error data model.
//! ```
//!
//! No two files should independently own the same validation rule.
//!
//! ## Determinism
//!
//! Importing this module has no runtime side effects.
//!
//! It does not:
//!
//! - allocate node IDs;
//! - access the filesystem;
//! - read environment variables;
//! - inspect hardware;
//! - inspect runtime state;
//! - use randomness;
//! - use wall-clock time;
//! - create global mutable state;
//! - reorder AST children.
//!
//! Deterministic behavior is inherited from the canonical AST provider and
//! traversal contracts.
//!
//! ## Thread safety
//!
//! This facade contains no mutable global state.
//!
//! The validation policies and validators are value-based and can be created
//! independently for concurrent read-only validation operations, subject to the
//! thread-safety guarantees of the supplied AST provider.
//!
//! ## Security
//!
//! The AST is untrusted compiler input.
//!
//! Validation infrastructure must therefore fail closed on malformed structure
//! while avoiding unchecked memory operations.
//!
//! This facade introduces no unsafe operations and delegates resource exhaustion
//! protection to explicitly configured traversal policies.
//!
//! In particular, no hidden AST-size limit is introduced here.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! ## File completion contract
//!
//! This file is complete when:
//!
//! 1. every validation implementation module has one declaration;
//! 2. the public validation API is re-exported from one stable boundary;
//! 3. no validation algorithm is duplicated here;
//! 4. validation remains domain-neutral;
//! 5. traversal remains delegated to `traversal/walk.rs`;
//! 6. source-range logic remains delegated to `source_ranges.rs`;
//! 7. invariant logic remains delegated to `invariants.rs`;
//! 8. recursion policy remains delegated to `recursion.rs`;
//! 9. complete structural orchestration remains delegated to `structural.rs`;
//! 10. validation-error representation remains delegated to
//!     `validation_error.rs`;
//! 11. no hardware or backend dependency exists;
//! 12. no machine-size or qubit-size limit exists here;
//! 13. Rust 1.97/1.97.1 compatibility is preserved;
//! 14. unsafe Rust is compiler-forbidden.
//!
//! Once those conditions are satisfied, changes to an individual validation
//! implementation should not require reopening this file unless its public
//! contract changes.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Canonical validation implementation modules
// -----------------------------------------------------------------------------
//
// Keep this list explicit. It is the authoritative composition of the native
// AST validation subsystem.
//
// The order reflects dependency responsibility rather than runtime execution
// order:
//
//   validation_error
//       ↓
//   invariants / source_ranges / recursion
//       ↓
//   structural
//
// Rust does not require declarations to follow dependency order, but keeping
// the conceptual order documented here makes the architecture easier to audit.

/// Shared, format-independent validation error data model.
///
/// Owns error classification, stable error codes, locations, related context,
/// and validation-result aggregation.
pub mod validation_error;

/// Reusable local AST invariants.
///
/// Owns node identity, node-kind, direct-reference and local structural checks.
pub mod invariants;

/// Source-coordinate and source-range validation.
///
/// Owns bounds, UTF-8 boundary, and optional parent/child span relationship
/// checks.
pub mod source_ranges;

/// Recursion/depth validation policy.
///
/// Delegates graph traversal to the canonical iterative AST walker.
pub mod recursion;

/// Complete graph-level structural AST validation.
///
/// Owns orchestration of local invariants, canonical traversal, graph
/// relationships, and configurable structural policy.
pub mod structural;

// -----------------------------------------------------------------------------
// Stable public re-exports
// -----------------------------------------------------------------------------
//
// These exports form the intended facade.
//
// Consumers should not need to know the physical file organization beneath
// this module.
//
// Do not re-export implementation-private helpers merely for convenience.
// Only stable public contracts belong here.

// Validation error model.
pub use validation_error::{
    ValidationError,
    ValidationErrorCode,
    ValidationErrorContext,
    ValidationErrorKind,
    ValidationResult,
};

// Local invariants.
pub use invariants::{
    is_valid_node_id,
    validate_child_identities,
    validate_child_identity,
    validate_child_reference,
    validate_node,
    validate_node_id,
    validate_node_identity,
    validate_node_kind,
    validate_resolved_child,
    validate_resolved_children,
    InvariantError,
};

// Source-range validation.
pub use source_ranges::{
    validate_parent_child,
    validate_parent_child_with_report,
    validate_span,
    validate_span_with_report,
    RangePosition,
    SingleSource,
    SourceRangeError,
    SourceRangePolicy,
    SourceRangeProvider,
    SourceRangeReport,
    SpanContainmentPolicy,
};

// Recursion validation.
pub use recursion::{
    validate as validate_recursion,
    validate_with_policy as validate_recursion_with_policy,
    validate_with_walk_config as validate_recursion_with_walk_config,
    walk_config_for,
    RecursionPolicy,
    RecursionValidation,
    RecursionValidationError,
};

// Complete structural validation.
pub use structural::{
    validate as validate_structural,
    validate_program,
    validate_with_config as validate_structural_with_config,
    SpanValidationPolicy,
    StructuralValidationConfig,
    StructuralValidationError,
    StructuralValidationStatistics,
    StructuralValidator,
};

// -----------------------------------------------------------------------------
// Stable facade helpers
// -----------------------------------------------------------------------------
//
// These helpers deliberately delegate to the authoritative implementation
// modules. They do not create another validation engine.
//
// Their purpose is to provide discoverable, semantically named entry points
// from the validation facade.

// =============================================================================
// Default structural validation
// =============================================================================

/// Validates a native AST root using the canonical default structural policy.
///
/// This is the primary facade entry point for callers that need complete
/// structural AST validation without custom operational policy.
///
/// The default policy:
//!
//! - introduces no artificial node limit;
//! - introduces no artificial edge limit;
//! - introduces no artificial depth limit;
//! - rejects cycles;
//! - rejects completed-node revisits;
//! - permits zero-width source spans;
//! - permits generated/cross-source source provenance;
//! - does not require the root to be a `Program`.
///
/// Semantic, quantum, hardware, routing, scheduling, QEC, calibration, and
/// backend validation remain outside this function.
#[inline]
pub fn validate<P>(
    provider: &P,
    root: crate::frontend::ast::node::node_id::NodeId,
) -> Result<
    StructuralValidationStatistics,
    StructuralValidationError<P::Error>,
>
where
    P: crate::frontend::ast::node::traversal::walk::AstWalkProvider,
    P::Error: core::fmt::Display,
{
    structural::validate(provider, root)
}

// =============================================================================
// Explicit structural validation
// =============================================================================

/// Validates a native AST root with an explicit structural policy.
///
/// This is the primary configurable facade entry point.
///
/// Operational limits are caller-selected safety budgets and do not become
/// Zamani language limits.
#[inline]
pub fn validate_with_config<P>(
    provider: &P,
    root: crate::frontend::ast::node::node_id::NodeId,
    config: StructuralValidationConfig,
) -> Result<
    StructuralValidationStatistics,
    StructuralValidationError<P::Error>,
>
where
    P: crate::frontend::ast::node::traversal::walk::AstWalkProvider,
    P::Error: core::fmt::Display,
{
    structural::validate_with_config(provider, root, config)
}

// =============================================================================
// Complete-program validation
// =============================================================================

/// Validates a root as a complete native Zamani `Program`.
///
/// This is explicit rather than being the default because validation is also
/// useful for AST fragments, generated subtrees, parser recovery trees and
/// transformation intermediate states.
#[inline]
pub fn validate_program_ast<P>(
    provider: &P,
    root: crate::frontend::ast::node::node_id::NodeId,
) -> Result<
    StructuralValidationStatistics,
    StructuralValidationError<P::Error>,
>
where
    P: crate::frontend::ast::node::traversal::walk::AstWalkProvider,
    P::Error: core::fmt::Display,
{
    structural::validate_program(provider, root)
}

// =============================================================================
// AST fragment validation
// =============================================================================

/// Validates an AST fragment without requiring a `Program` root.
///
/// Fragment validation is useful for:
//!
//! - parser recovery;
//! - generated nodes;
//! - macro expansion;
//! - imported syntax;
//! - source transformations;
//! - tests;
//! - incremental compilation.
#[inline]
pub fn validate_fragment<P>(
    provider: &P,
    root: crate::frontend::ast::node::node_id::NodeId,
) -> Result<
    StructuralValidationStatistics,
    StructuralValidationError<P::Error>,
>
where
    P: crate::frontend::ast::node::traversal::walk::AstWalkProvider,
    P::Error: core::fmt::Display,
{
    structural::validate_fragment(provider, root)
}

// =============================================================================
// Compile-time API invariants
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::node_id::NodeId;

    // -------------------------------------------------------------------------
    // Module composition
    // -------------------------------------------------------------------------

    #[test]
    fn validation_modules_are_composed_through_one_facade() {
        // The test intentionally uses the public symbols rather than private
        // implementation details. This makes the test an API-contract test.
        let _ = StructuralValidator::new();
        let _ = RecursionPolicy::unrestricted();
        let _ = SourceRangePolicy::default();
    }

    // -------------------------------------------------------------------------
    // POCO-REAF policy
    // -------------------------------------------------------------------------

    #[test]
    fn_default_structural_policy_has_no_artificial_scale_limits() {
        let config = StructuralValidationConfig::default();

        assert_eq!(config.traversal.max_nodes, None);
        assert_eq!(config.traversal.max_edges, None);
        assert_eq!(config.traversal.max_depth, None);
    }

    #[test]
    fn unrestricted_recursion_policy_has_no_artificial_depth_limit() {
        let policy = RecursionPolicy::unrestricted();

        assert_eq!(policy.max_depth(), None);
        assert!(policy.rejects_revisits());

        let walk_config = policy.to_walk_config();

        assert_eq!(walk_config.max_nodes, None);
        assert_eq!(walk_config.max_edges, None);
        assert_eq!(walk_config.max_depth, None);
        assert!(walk_config.reject_revisits);
    }

    #[test]
    fn unrestricted_source_range_policy_does_not_introduce_limits() {
        let policy = SourceRangePolicy::unrestricted();

        assert!(!policy.check_bounds);
        assert!(!policy.check_utf8_boundaries);
        assert_eq!(
            policy.containment,
            SpanContainmentPolicy::Ignore
        );
        assert!(!policy.require_same_source);
    }

    // -------------------------------------------------------------------------
    // Foundational identity boundary
    // -------------------------------------------------------------------------

    #[test]
    fn node_id_zero_is_not_a_valid_ast_identity() {
        // `NodeId::new(0)` is rejected by the canonical identity type.
        assert!(NodeId::new(0).is_none());

        // No second identity representation is created by validation.
        assert!(NodeId::new(1).is_some());
    }

    // -------------------------------------------------------------------------
    // Operational limits remain explicit
    // -------------------------------------------------------------------------

    #[test]
    fn explicit_limits_are_configuration_not_language_semantics() {
        let config = StructuralValidationConfig::default()
            .with_max_nodes(1_000)
            .with_max_edges(4_000)
            .with_max_depth(256);

        assert_eq!(config.traversal.max_nodes, Some(1_000));
        assert_eq!(config.traversal.max_edges, Some(4_000));
        assert_eq!(config.traversal.max_depth, Some(256));
    }

    // -------------------------------------------------------------------------
    // Domain neutrality
    // -------------------------------------------------------------------------

    #[test]
    fn_validation_facade_contains_no_domain_specific_configuration() {
        // This test is intentionally lightweight. Domain neutrality is chiefly
        // enforced by module dependencies and API design rather than runtime
        // behavior.
        //
        // The important contract is that validation configuration contains no:
        //
        // - qubit count;
        // - gate set;
        // - hardware topology;
        // - backend identifier;
        // - vendor identifier;
        // - processor architecture.
        let config = StructuralValidationConfig::default();

        assert_eq!(config.traversal.max_nodes, None);
        assert_eq!(config.traversal.max_edges, None);
        assert_eq!(config.traversal.max_depth, None);
    }
}