 
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani HDL — Hardware Description and Hardware/Software Co-Design
//!
//! Path: `src/hdl/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for Zamani's HDL computing domain.
//!
//! It establishes the public architectural boundary through which the HDL
//! domain's structural descriptions, behavioral semantics, verification,
//! simulation, interoperability, target-feasibility analysis, and eventual
//! realization interfaces will be integrated as their implementations are
//! introduced.
//!
//! This file coordinates module ownership. It does not implement a parser,
//! AST, semantic analyzer, hardware execution engine, or intermediate
//! representation.
//!
//! ## Normative authority
//!
//! HDL implementation and integration MUST conform to the repository's
//! actual language authorities, including:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - The applicable delegate grammars under `grammar/hdl/`
//! - The canonical frontend AST and semantic contracts.
//!
//! The repository's normative specifications define required semantics.
//! The canonical grammar defines syntax and language composition.
//! Existing frontend modules own parsing and source-level semantic analysis.
//! This composition root MUST NOT override those authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The top-level Rust module boundary for HDL computing.
//! - Registration of implemented HDL child modules.
//! - The architectural dependency and integration rules documented here.
//! - The separation between portable HDL intent and physical realization.
//! - The HDL subsystem's safe-Rust invariant.
//!
//! It does NOT own:
//!
//! - Lexer rules, token definitions, or token scanning.
//! - Grammar productions, parsing, or parser recovery.
//! - AST nodes, source spans, or source-file identity.
//! - General name resolution, type inference, or semantic-model ownership.
//! - Hardware-specific semantic facts already owned by the frontend.
//! - Physical hardware allocation or target discovery.
//! - Placement, routing, synthesis, or timing closure.
//! - A competing universal or domain-specific intermediate representation.
//! - Backend execution, deployment, or runtime device handles.
//!
//! Each responsibility MUST have one authoritative implementation.
//!
//! ## Existing repository integration
//!
//! The existing HDL parser is:
//!
//! `crate::frontend::parser::domains::hdl`
//!
//! It owns HDL construct parsing and its parser-specific configuration,
//! loss-preserving syntax representation, and parse errors.
//!
//! The existing HDL semantic validator is:
//!
//! `crate::frontend::semantic::domains::hdl`
//!
//! It owns the existing HDL-specific semantic validation boundary, including
//! its symbolic-width and semantic-input contracts.
//!
//! These modules MUST be reused through their actual public APIs. This
//! composition root MUST NOT copy their implementations or invent adapters
//! whose constructors, types, or invariants have not been verified.
//!
//! The canonical AST remains owned by:
//!
//! `crate::frontend::ast::node`
//!
//! The shared semantic model remains owned by:
//!
//! `crate::frontend::semantic::model`
//!
//! Source locations MUST use the canonical frontend span contract where
//! spans are needed. HDL MUST NOT introduce a competing source-span system.
//!
//! The legacy compiler modules remain separate until an explicitly tested
//! migration changes their integration.
//!
//! ## Intermediate representation boundary
//!
//! All HDL intermediate-representation implementation belongs under:
//!
//! `src/hdl/hdlir/`
//!
//! This includes, when implemented, HDLIR data structures, builders,
//! verification, transformations, serialization, and lowering owned by
//! that representation.
//!
//! HDLIR implementation is explicitly outside the scope of this composition
//! root's present implementation milestone.
//!
//! The `hdlir` child module is registered only to preserve its existing
//! module boundary. Its implementation MUST NOT be moved into this file or
//! duplicated in another HDL child module.
//!
//! The HDL subsystem MUST NOT silently create a second IR elsewhere under
//! `src/hdl/`.
//!
//! ## Intended dependency direction
//!
//! The architectural dependency direction is:
//!
//! ```text
//! grammar and language specifications
//!                 |
//!                 v
//! canonical lexer and parser
//!                 |
//!                 v
//! canonical domain-neutral AST
//!                 |
//!                 v
//! shared semantic model and HDL validation
//!                 |
//!                 v
//!       src/hdl/ domain integration
//!                 |
//!                 v
//!        src/hdl/hdlir/ (deferred)
//!                 |
//!                 v
//! optimization and implementation planning
//!                 |
//!                 v
//! target realization and deployment
//! ```
//!
//! This diagram describes responsibility and intended data flow. It does
//! not claim that every stage is already implemented or operational.
//!
//! Lower-level parsing and semantic validation MUST NOT depend on target
//! realization, physical allocation, placement, or routing.
//!
//! The HDL composition root MUST NOT force the existing frontend to depend
//! on this module merely to parse or semantically validate HDL.
//!
//! ## POCO-REAF and portability
//!
//! The architectural objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! HDL source describes hardware computation, behavior, requirements,
//! constraints, and contracts independently of unnecessary physical-target
//! assumptions.
//!
//! Target-specific realization is permitted when explicitly requested or
//! required by the implementation contract. Such specificity MUST remain
//! distinguishable from portable HDL semantics.
//!
//! The architecture MUST NOT introduce universal language-level ceilings
//! for signal widths, ports, modules, instances, memories, clock domains,
//! pipeline stages, devices, or hardware resource quantities.
//!
//! Symbolic quantities MUST remain representable until their owning stage
//! is authorized to resolve or evaluate them.
//!
//! A target that cannot meet a requirement MUST produce an explicit
//! infeasibility or unsupported-capability result. It MUST NOT silently
//! weaken requirements or change program meaning.
//!
//! No finite implementation can process literally infinite input. Scaling
//! therefore means avoiding arbitrary universal hardware ceilings and
//! scaling with available resources, representable values, supported
//! semantics, and explicit operational budgets.
//!
//! Operational budgets are not language-semantic capacity limits. Where
//! required, the owning subsystem MUST report exhaustion or incomplete
//! analysis explicitly rather than silently truncating work or claiming
//! that an incomplete result is valid.
//!
//! ## Module registration policy
//!
//! Each HDL child module MUST be registered here only after:
//!
//! 1. Its ownership and authority have been established.
//! 2. Its public types and API contracts have been agreed.
//! 3. Its dependencies on existing repository APIs have been verified.
//! 4. Its error and incomplete-result behavior have been defined.
//! 5. Its tests and integration requirements have been specified.
//! 6. Its source file and any required child module files exist.
//!
//! A directory proposed in an architecture plan is not evidence that an
//! implementation exists. This file MUST NOT declare speculative modules
//! just to make the intended architecture appear complete.
//!
//! New responsibilities SHOULD be placed in separate child modules when
//! that separation establishes a genuine ownership boundary. Empty,
//! speculative directories and duplicate implementations SHOULD NOT be
//! introduced.
//!
//! Child modules MUST NOT depend on this composition root for foundational
//! data types if doing so would introduce a dependency cycle. Shared
//! contracts belong in their designated owning modules.
//!
//! ## Error and diagnostic boundaries
//!
//! Parser errors, semantic diagnostics, HDL-domain analysis failures,
//! target infeasibility, unsupported features, and operational-budget
//! exhaustion are distinct outcomes.
//!
//! This composition root MUST NOT collapse them into a new string-only
//! error interface or introduce a competing diagnostic representation.
//!
//! New HDL modules MUST integrate with the actual canonical error and
//! diagnostic APIs available to them. Any adapter MUST preserve the
//! distinction between invalid source semantics and a valid program that
//! cannot be realized on a requested target.
//!
//! Incomplete analysis MUST NOT be reported as successful validation.
//!
//! ## Determinism and compatibility
//!
//! Public HDL contracts SHOULD specify deterministic behavior for equivalent
//! inputs, configuration, and provider information wherever determinism is
//! required.
//!
//! Module registration MUST NOT depend on filesystem traversal order,
//! hardware discovery order, or unspecified iteration order.
//!
//! New public APIs MUST be documented and deliberately exported. Broad
//! wildcard re-exports SHOULD NOT be used to conceal ownership boundaries
//! or create name collisions.
//!
//! Compatibility changes MUST be driven by verified consumers, tests,
//! and explicit API or specification changes rather than speculative
//! future requirements.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden throughout this HDL module and its descendants.
//!
//! External HDL input, imported designs, target descriptions, and provider
//! results MUST be treated as potentially malformed. Individual modules
//! are responsible for validating their inputs, checking arithmetic,
//! handling resource exhaustion, and returning structured failures.
//!
//! ## Production-readiness boundary
//!
//! This file is the production-oriented composition boundary, not proof that
//! the HDL computing domain is complete.
//!
//! The subsystem MUST NOT be declared production-ready until its implemented
//! modules, frontend integration, semantic contracts, verification coverage,
//! interoperability behavior, target-feasibility reporting, and supported
//! toolchain checks have been independently validated.
//!
//! Required repository checks include:
//!
//! - `cargo fmt --all -- --check`
//! - `cargo check --all-targets`
//! - `cargo test --all-targets`
//! - `cargo clippy --all-targets -- -D warnings`
//!
//! These checks must be run against the actual repository checkout. This
//! source file does not claim that they have passed.
//!
//! ## Freeze criteria
//!
//! This composition root can be frozen for its current milestone when:
//!
//! 1. It registers only existing child modules.
//! 2. HDLIR remains under `src/hdl/hdlir/` and its implementation remains
//!    deferred in this milestone.
//! 3. Existing parser, AST, and semantic ownership is preserved.
//! 4. No duplicate IR, AST, semantic model, or source-span type is added.
//! 5. Future module registration follows the documented integration policy.
//! 6. The module and its existing child modules compile under the supported
//!    Rust toolchain.
//! 7. The repository's applicable formatting, tests, and lint checks pass.
//!
//! A frozen composition contract does not prohibit a justified correction
//! for a verified defect or a deliberate, reviewed API evolution.

// -----------------------------------------------------------------------------
// Existing HDL intermediate-representation boundary
// -----------------------------------------------------------------------------
//
// This declaration preserves the existing module path. No HDLIR types,
// builders, verifiers, passes, serialization, or lowering are implemented
// or re-exported here.
//
// Keep the implementation of every HDLIR feature inside `hdlir/`.

/// HDL intermediate-representation boundary.
///
/// The implementation of this subsystem is intentionally deferred.
/// All future HDLIR-owned functionality belongs under `src/hdl/hdlir/`.
pub mod hdlir;

#[cfg(test)]
mod tests {
    //! Composition-root contract tests.
    //!
    //! These tests intentionally avoid assuming that future HDL modules or
    //! APIs already exist. Integration tests for the parser, semantic
    //! validator, target feasibility, and HDLIR must live with their owning
    //! implementations when those components are ready.

    #[test]
    fn hdlir_module_boundary_is_registered() {
        // A compile-time module-path check. The HDLIR implementation itself
        // remains outside this milestone.
        let _: fn() = || {
            let _ = core::any::type_name::<super::hdlir::Placeholder>();
        };
    }
}
