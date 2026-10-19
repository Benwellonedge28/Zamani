
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Timing Subsystem
//!
//! Path: `src/hdl/timing/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Timing subsystem composition root.
//!
//! ## PURPOSE
//!
//! Establish the public architectural boundary for source-level HDL timing
//! semantics and timing analysis integration.
//!
//! This module is responsible for composition and public API governance.
//! It is not itself a clock model, timing engine, simulation scheduler,
//! physical timing-closure engine, or intermediate representation.
//!
//! ## AUTHORITY
//!
//! The timing subsystem must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/timing.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/ast/`
//! - `src/frontend/semantic/model/`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar owns syntax. The canonical AST owns source structure.
//! The shared semantic model owns canonical semantic identities and facts.
//! HDL semantic validation owns source-level HDL legality.
//! This subsystem provides the logical timing-domain integration boundary.
//!
//! If an implementation conflicts with a normative specification, the
//! implementation must not silently redefine the language semantics.
//!
//! ## OWNS
//!
//! This composition root owns:
//!
//! - Timing subsystem module identity.
//! - Timing subsystem API and schema-version metadata.
//! - Explicit registration and re-export of implemented timing modules.
//! - Timing subsystem integration-contract tests.
//!
//! Future child modules have separate responsibilities:
//!
//! - `clock.rs`: logical clock declarations and properties.
//! - `reset.rs`: reset intent and reset relationships.
//! - `domain.rs`: logical clock-domain membership and relationships.
//! - `constraints.rs`: source-level timing constraints and requirements.
//! - `crossing.rs`: clock-domain crossing analysis.
//! - `analysis.rs`: logical timing analysis and structured results.
//!
//! These files are not present in the audited repository revision. They must
//! be implemented against their own frozen contracts before being registered
//! here. This composition root does not declare nonexistent Rust modules.
//!
//! ## DOES NOT OWN
//!
//! This module must not own or implement:
//!
//! - Lexer, parser, grammar productions, or a replacement AST.
//! - A second source-span, symbol-identity, or semantic-fact system.
//! - General expression parsing, name resolution, or type inference.
//! - A competing HDL width or resource model.
//! - Physical clock generation, oscillators, PLLs, DLLs, or clock trees.
//! - Physical reset networks or synchronizer insertion.
//! - Placement, routing, synthesis, or physical timing closure.
//! - Target discovery, physical resource allocation, or vendor APIs.
//! - Simulation event scheduling or runtime execution.
//! - HDLIR data structures, builders, verification, passes, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is outside
//! the scope of this module.
//!
//! ## SEMANTIC BOUNDARIES
//!
//! The timing subsystem must preserve the distinction between:
//!
//! 1. A source-level timing declaration.
//! 2. A validated logical timing relationship.
//! 3. A timing requirement or constraint.
//! 4. A target's ability to satisfy that requirement.
//! 5. A physical implementation's measured or proven timing behavior.
//!
//! These are different facts and must not be represented as interchangeable
//! values or statuses.
//!
//! A source declaration can be syntactically valid while its timing expression
//! remains unresolved. An unresolved expression is not automatically invalid,
//! and it is not evidence that the requirement is satisfied.
//!
//! A valid logical timing model does not establish that a physical target can
//! meet its requirements. Target feasibility and physical timing closure are
//! downstream responsibilities.
//!
//! The implementation must distinguish at least these outcomes wherever
//! timing analysis is performed:
//!
//! - Analysis completed with no detected violations.
//! - Analysis completed with detected violations.
//! - Analysis incomplete or indeterminate.
//! - Analysis rejected invalid input.
//! - Analysis aborted or failed.
//!
//! In particular, an incomplete analysis must never be reported as a
//! successful proof that all timing requirements hold.
//!
//! ## CLOCK, CLOCKING, AND TIMING OWNERSHIP
//!
//! The Rust implementation must mirror the grammar's ownership boundaries.
//!
//! `clocks.g4` defines logical clock declarations and clock properties.
//!
//! `clocking.g4` defines clocking context, relationships, sampling intent,
//! synchronization intent, and crossing-related declarations.
//!
//! `timing.g4` defines general timing declarations, requirements, constraints,
//! windows, paths, exceptions, assertions, and timing budgets.
//!
//! The Rust modules must not redefine one another's primary concepts.
//! Shared concepts should be referenced through stable typed interfaces.
//!
//! Timing expressions must preserve symbolic values and their source meaning.
//! The implementation must not require every period, frequency, latency,
//! phase, uncertainty, or related quantity to be resolved during parsing.
//!
//! Dimensional correctness, expression resolution, and constraint
//! compatibility belong to semantic analysis.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! The timing subsystem supports the architectural objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! It must not impose universal language-level limits on:
//!
//! - Number of clocks or clock domains.
//! - Number of timing constraints or timing paths.
//! - Number of clock relationships or crossings.
//! - Number of processes, modules, or pipeline stages.
//! - Clock frequency, period, phase, latency, or timing-window values.
//!
//! It must not assume a fixed oscillator, clock rate, synchronizer depth,
//! FPGA family, ASIC process, board, physical pin, or vendor timing engine.
//!
//! Symbolic quantities must remain symbolic until the responsible semantic
//! or target-aware stage can resolve them legitimately.
//!
//! Concrete analysis is necessarily bounded by available memory, execution
//! time, representational capacity, and configured operational budgets.
//! Such budgets must be explicit and must not become hidden language limits.
//!
//! If a budget is exhausted, the responsible analysis must expose an
//! incomplete or failed outcome rather than silently claiming success.
//!
//! No finite implementation can process literally infinite input. The
//! scalability objective is the absence of arbitrary universal ceilings,
//! together with explicit resource accounting and compositional design.
//!
//! ## DETERMINISM
//!
//! The composition root must:
//!
//! - Introduce no global mutable state.
//! - Perform no I/O or hardware discovery.
//! - Avoid implicit target selection.
//! - Use explicit public exports rather than wildcard re-exports.
//! - Keep schema metadata independent of the language and artifact versions.
//!
//! Child modules must define their own deterministic identity, ordering,
//! arithmetic, and diagnostic contracts where applicable.
//!
//! ## DEPENDENCY DIRECTION
//!
//! The intended dependency direction is:
//!
//! ```text
//! Canonical HDL grammar
//!         |
//!         v
//! Canonical frontend AST
//!         |
//!         v
//! Shared semantic model and HDL validation
//!         |
//!         v
//! Logical HDL timing contracts
//!         |
//!         +--> Clock declarations
//!         +--> Reset intent
//!         +--> Clock domains
//!         +--> Timing constraints
//!         +--> Crossing analysis
//!         +--> Logical timing analysis
//!         |
//!         v
//! Target feasibility and downstream realization
//!         |
//!         v
//! Physical implementation and timing closure
//! ```
//!
//! The diagram describes architectural ownership, not a claim that all stages
//! have already been implemented.
//!
//! The timing subsystem must not depend on HDLIR internals, physical
//! implementation APIs, or the legacy semantic analyzer.
//!
//! ## INTEGRATION CONTRACT
//!
//! `src/hdl/mod.rs` is the parent composition root. It may expose this module
//! once the timing directory has been created and registered.
//!
//! The existing `src/frontend/semantic/domains/hdl.rs` remains the authority
//! for its current HDL semantic input and validation contract. Integration
//! must use an explicit adapter and must not duplicate the existing validator.
//!
//! The AST adapter must preserve source identity, source locations, symbolic
//! timing expressions, clock/reset references, and declaration relationships.
//! It must not infer missing semantics from the selected target.
//!
//! Future timing child modules must be added only after their files exist and
//! compile. Each child must document its authority, ownership, inputs, outputs,
//! error behavior, scalability policy, tests, and downstream consumers.
//!
//! This root intentionally has no imports from proposed child modules because
//! those implementations do not exist in the audited revision. This allows
//! the root itself to compile without placeholder modules or guessed APIs.
//!
//! ## COMPATIBILITY
//!
//! - Rust 2021 edition.
//! - Rust 1.97 or later.
//! - Stable Rust; no nightly features required.
//! - Safe Rust only.
//! - No new external dependencies.
//! - No duplicate AST, semantic, diagnostic, or IR ownership.
//! - No target-specific implementation dependency.
//!
//! ## FREEZE CRITERIA
//!
//! This composition root can be frozen when:
//!
//! 1. Its module identity and schema version are stable.
//! 2. The public registration policy is documented.
//! 3. Every registered child exists and compiles.
//! 4. Child interfaces respect the grammar and semantic ownership boundaries.
//! 5. No child introduces universal hardware-capacity ceilings.
//! 6. Incomplete analysis cannot be mistaken for proven correctness.
//! 7. The parent HDL module exposes the intended API.
//! 8. Unit tests and repository-level checks pass.
//!
//! A passing composition-root test alone does not establish that the entire
//! HDL timing subsystem is production-ready.

/// Stable architectural identity of the HDL timing subsystem.
pub const TIMING_MODULE_NAME: &str = "zamani.hdl.timing";

/// Version of this composition root's public contract.
///
/// This is not the Zamani language version, grammar version, timing-model
/// version, HDLIR version, or serialized-artifact version.
pub const TIMING_SCHEMA_VERSION: u16 = 1;

/// Return the stable architectural module name.
#[must_use]
pub const fn module_name() -> &'static str {
    TIMING_MODULE_NAME
}

/// Return the composition-root schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    TIMING_SCHEMA_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(module_name(), "zamani.hdl.timing");
    }

    #[test]
    fn schema_version_is_nonzero_and_stable() {
        assert_eq!(schema_version(), 1);
        assert!(schema_version() > 0);
    }

    #[test]
    fn module_identity_and_version_are_independent_values() {
        assert!(!module_name().is_empty());
        assert_ne!(module_name(), schema_version().to_string());
    }
}
