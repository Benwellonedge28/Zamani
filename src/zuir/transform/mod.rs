//! # Zamani Universal Intermediate Representation (ZUIR) Transformations
//!
//! Canonical path: `src/zuir/transform/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97.1 or later
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Module-composition boundary. Freeze only after the parent module and
//! repository integration checks pass.
//!
//! PURPOSE
//!
//! Expose the canonical collection of ZUIR transformation modules:
//!
//! - `canonicalize`: semantics-preserving canonical representation.
//! - `simplify`: semantics-preserving simplification.
//! - `inline`: contract-checked function inlining.
//! - `control_flow`: semantics-preserving control-flow transformations.
//! - `legalize`: conversion to an explicitly supported operation subset.
//!
//! This module owns module composition and the public namespace boundary.
//! It does not implement transformations or define another IR.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/pass/manager.rs`
//! - `src/zuir/pass/contracts.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/dialect/`
//! - `src/zuir/verify/`
//! - `src/zuir/lowering/`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//!
//! OWNERSHIP
//!
//! THIS MODULE OWNS:
//!
//! - The public transformation-module hierarchy.
//! - Explicit exposure of transformation implementations.
//! - Module-level integration documentation.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - IR entities, types, values, operations, or identifiers.
//! - Operation semantics or dialect-specific rewrite rules.
//! - IR verification or semantic analysis.
//! - Pass execution, global budgets, cancellation, or analysis invalidation.
//! - Physical resource allocation, routing, scheduling, or target execution.
//! - Source grammar, AST definitions, or source-language semantics.
//!
//! DEPENDENCY DIRECTION
//!
//! Frontend and semantic model
//!     -> canonical ZUIR
//!     -> verifier
//!     -> transformation/pass infrastructure
//!     -> verified downstream lowering
//!
//! Individual transformations may depend on the canonical IR, dialect
//! contracts, verifier, and pass infrastructure. Core IR and foundational
//! pass contracts must not depend on this module.
//!
//! GRAMMAR AND SEMANTIC BOUNDARY
//!
//! Grammar files define source syntax; they do not authorize IR rewrites.
//!
//! In particular:
//!
//! - `grammar/statements/conditionals.g4` owns statement-level conditional
//!   syntax.
//! - `grammar/expressions/conditionals.g4` owns value-producing conditional
//!   expression syntax.
//! - `grammar/statements/loops.g4` owns the canonical structured loop forms.
//! - `grammar/statements/breaks.g4` and
//!   `grammar/statements/continues.g4` own the corresponding statement syntax.
//! - `grammar/antlr/ZamaniParser.g4` owns parser composition.
//! - `grammar/DESIGN.md` governs language-layer boundaries.
//!
//! The semantic model and canonical IR define the meaning that transformations
//! must preserve. No transformation may infer semantics solely from grammar
//! rule names or operation-name strings.
//!
//! SEMANTIC PRESERVATION
//!
//! Every transformation is responsible for honoring its own adapter and
//! pass contracts. This module does not imply that all transformations are
//! applicable to every IR or target.
//!
//! Implementations must preserve, or explicitly reject transformations that
//! cannot preserve:
//!
//! - Types, value identity, SSA/use-def constraints, and control-flow edges.
//! - Observable effects, ordering, synchronization, and concurrency.
//! - Ownership, borrowing, linear and affine resource constraints.
//! - Quantum measurement, reset, and classical feed-forward semantics.
//! - HDL timing, signal relationships, clocking, and sequential behavior.
//! - Numerical behavior, precision requirements, and applicable contracts.
//! - Capabilities, resource requirements, provenance, and extension metadata.
//!
//! Unknown operations must not be silently deleted, reinterpreted, reordered,
//! or treated as pure. A dialect may authorize a rewrite only through its
//! registered, trusted semantic contract.
//!
//! TRANSACTION AND FAILURE POLICY
//!
//! A module declaration does not make a transformation atomic.
//!
//! Each transformation must document whether an individual rewrite is atomic,
//! whether a failed pass can leave earlier rewrites committed, and how callers
//! must re-verify the IR and invalidate analyses after failure.
//!
//! A caller must not treat an IR as verified merely because a transformation
//! returned an error or stopped due to cancellation or budget exhaustion.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This module introduces no fixed limits on:
//!
//! - Operations, blocks, regions, functions, or modules.
//! - Qubits, processors, threads, devices, or distributed nodes.
//! - Tensor rank, register width, memory capacity, or topology size.
//! - The number of registered dialects or supported computational domains.
//!
//! Transformations must use the canonical IR and dynamically sized structures
//! appropriate to their algorithms. Arithmetic and counters must be checked
//! where overflow is possible. Resource exhaustion, cancellation, and
//! configured work-budget exhaustion must be reported explicitly.
//!
//! Literal infinity is not possible on finite computing resources. An
//! implementation must not turn an operational limit into a hidden semantic
//! or universal hardware limit.
//!
//! EXTENSIBILITY
//!
//! New transformations must be added as independently owned modules with
//! documented contracts, tests, deterministic behavior where required, and
//! explicit pass-manager integration.
//!
//! Do not add a closed universal list of operations, devices, quantum gates,
//! or computational domains here. Domain-specific transformations belong in
//! the appropriate domain module and must follow the canonical pass contract.
//!
//! PUBLIC API POLICY
//!
//! The child modules are exposed by name. Their public APIs remain namespaced:
//!
//!     zuir::transform::canonicalize::...
//!     zuir::transform::simplify::...
//!     zuir::transform::inline::...
//!     zuir::transform::control_flow::...
//!     zuir::transform::legalize::...
//!
//! Deliberately avoid wildcard re-exports. The child modules use distinct
//! configuration, adapter, report, and error types; flattening those APIs
//! would create collisions and obscure ownership.
//!
//! Adding a module here does not automatically register its pass with the
//! pass manager. Registration belongs to the compilation pipeline and must
//! use the pass's declared identity, version, requirements, determinism,
//! verification policy, and invalidation contract.
//!
//! COMPATIBILITY
//!
//! Renaming or removing a public child module is a public API change.
//! Breaking changes must follow `src/zuir/ZUIR-SPEC.md` and the repository's
//! compatibility policy. A module must not be declared until its source file
//! exists and is included in the build.
//!
//! FREEZE CRITERIA
//!
//! - Every declared child module exists and compiles.
//! - Each child transformation has a defined semantic contract.
//! - Public pass identities are unique and namespaced.
//! - The parent `src/zuir/mod.rs` correctly declares `transform`.
//! - The crate's canonical pass and verification APIs are reconciled.
//! - Grammar and semantic boundaries remain consistent with ZUIR.
//! - Unit, integration, conformance, and documentation tests pass.
//! - Formatting, linting, and the repository's supported build checks pass.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Canonical transformation modules
// -----------------------------------------------------------------------------

/// Canonicalize representations only when equivalence is established by the
/// applicable IR and dialect contracts.
///
/// This pass must not assume that operations are pure or reorderable based
/// solely on their names or descriptive attributes.
pub mod canonicalize;

/// Transform control flow while preserving terminators, successor edges,
/// edge arguments, dominance, effects, and domain-specific execution meaning.
pub mod control_flow;

/// Inline eligible functions only when the adapter can establish the required
/// call, linkage, ownership, effect, and semantic preconditions.
pub mod inline;

/// Legalize IR for an explicitly specified downstream operation subset.
///
/// Legalization must use registered conversion contracts and must not silently
/// discard unsupported operations or semantic information.
pub mod legalize;

/// Simplify IR through explicitly authorized, semantics-preserving rewrites.
///
/// Unknown operations and observable effects must be preserved unless a
/// trusted registered contract proves that a transformation is valid.
pub mod simplify;

// -----------------------------------------------------------------------------
// Module-composition tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure built-in pass identities remain unique and namespaced.
    ///
    /// These identifiers are part of pass-pipeline diagnostics and may be
    /// persisted by pipeline tooling. They must not depend on registration
    /// order, host hardware, or runtime discovery.
    #[test]
    fn built_in_pass_ids_are_unique_and_namespaced() {
        let ids = [
            canonicalize::CANONICALIZE_PASS_ID,
            control_flow::CONTROL_FLOW_PASS_ID,
            inline::INLINE_PASS_ID,
            legalize::LEGALIZE_PASS_ID,
            simplify::SIMPLIFY_PASS_ID,
        ];

        for id in ids {
            assert!(!id.trim().is_empty(), "pass ID must not be empty");
            assert!(
                id.contains('.'),
                "pass ID must be namespaced: {id}"
            );
            assert!(
                !id.chars().any(char::is_whitespace),
                "pass ID must not contain whitespace: {id}"
            );
        }

        for (index, id) in ids.iter().enumerate() {
            assert!(
                !ids[..index].contains(id),
                "duplicate built-in pass ID: {id}"
            );
        }
    }
}