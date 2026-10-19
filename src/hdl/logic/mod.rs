
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logic Subsystem
//!
//! Path: `src/hdl/logic/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Public composition boundary for the HDL logic subsystem.
//!
//! This module integrates the existing expression, assignment,
//! combinational, sequential, process, and state-machine contracts.
//! It does not claim that all HDL logic execution, analysis, or
//! end-to-end compilation features are already operational.
//!
//! ## Normative authority
//!
//! This subsystem MUST conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/combinational.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/state_machines.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The exact grammar production determines which source constructs are
//! valid. The normative specification determines their required meaning.
//! Neither this module nor a downstream implementation may override them.
//!
//! ## Canonical ownership
//!
//! The following repository boundaries remain authoritative:
//!
//! - `crate::frontend::parser` owns parsing and syntax recognition.
//! - `crate::frontend::ast::node` owns canonical AST nodes and identities.
//! - `crate::frontend::semantic` owns shared semantic infrastructure.
//! - `crate::frontend::semantic::domains::hdl` owns the existing
//!   HDL semantic-validation input and validation contracts.
//! - `crate::hdl::design` owns HDL structural design declarations.
//! - `crate::hdl::types` owns HDL-specific type contracts.
//! - `crate::hdl::values` owns HDL value representations.
//! - `crate::hdl::timing` owns logical clock, reset, and timing contracts.
//! - `crate::hdl::memory` owns HDL memory semantics.
//! - `crate::hdl::analysis` owns cross-design analyses assigned to it.
//! - `crate::hdl::hdlir` exclusively owns HDL intermediate-representation
//!   implementation.
//!
//! This module composes the logic-specific contracts without duplicating
//! those authorities.
//!
//! ## Responsibilities
//!
//! This module:
//!
//! - Registers the six logic-specific implementation modules.
//! - Provides stable, explicit module paths for their public APIs.
//! - Documents responsibility boundaries and integration direction.
//! - Establishes shared architectural invariants for HDL logic.
//!
//! This module MUST NOT implement logic evaluation, assignment validation,
//! dependency analysis, state transitions, or process scheduling itself.
//!
//! ## Does not own
//!
//! This module MUST NOT introduce:
//!
//! - A replacement parser, AST, source-span system, or symbol resolver.
//! - A competing general-purpose type or expression system.
//! - A second implementation of `validate_hdl`.
//! - A universal or domain-specific intermediate representation.
//! - HDLIR nodes, builders, verifiers, passes, serialization, or lowering.
//! - Physical gates, registers, clock trees, pin assignments, or routing.
//! - Synthesis, placement, physical timing closure, or device allocation.
//! - A closed list of hardware primitives or supported future targets.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Module responsibilities
//!
//! ### `expression`
//!
//! Owns HDL expression-evaluation requests, evaluation modes, explicit
//! outcomes, and the extension boundary for expression evaluation.
//! It consumes canonical AST identities and existing semantic services.
//!
//! ### `assignment`
//!
//! Owns assignment-context contracts, local assignment checks, type-checking
//! integration, and conservative driver-conflict detection.
//! It does not replace canonical name resolution or type checking.
//!
//! ### `combinational`
//!
//! Owns combinational-region analysis contracts, dependency ordering,
//! assignment-coverage analysis, and explicit incomplete outcomes.
//! Its callers must derive coverage facts from actual control flow.
//!
//! ### `sequential`
//!
//! Owns sequential-behavior contracts and their local structural checks.
//! Clock and reset declarations remain owned by their respective modules.
//! Physical register inference is not performed here.
//!
//! ### `process`
//!
//! Owns HDL process contracts, including source-level process kind,
//! sensitivity references, and explicit execution-model information.
//! It does not implement a separate runtime or event scheduler.
//!
//! ### `state_machine`
//!
//! Owns logical states, transitions, guards, actions, and structural
//! state-machine analysis. Physical state encoding is a realization choice.
//!
//! ## Dependency direction
//!
//! The intended direction is:
//!
//! ```text
//! grammar and normative specification
//!                 |
//!                 v
//!       canonical frontend parser
//!                 |
//!                 v
//!         canonical frontend AST
//!                 |
//!                 v
//!       shared semantic analysis
//!                 |
//!                 v
//!          HDL logic contracts
//!          /      |       \
//! expressions  assignments  processes
//!                  |           |
//!                  v           v
//!           combinational   sequential
//!                  \           /
//!                   v         v
//!                  state machines
//!                         |
//!                         v
//!                cross-design analysis
//!                         |
//!                         v
//!              future HDLIR integration
//! ```
//!
//! This diagram describes intended responsibility and data flow. It does
//! not assert that every adapter or pipeline edge has been implemented.
//!
//! Dependencies MUST remain acyclic. In particular, parser and semantic
//! foundations must not depend on target realization, and logic contracts
//! must not require HDLIR implementation merely to express source meaning.
//!
//! ## Cross-module invariants
//!
//! 1. Canonical AST node identities MUST be reused.
//! 2. Source-language meaning MUST remain independent of physical target.
//! 3. Symbolic widths and unresolved semantic facts MUST remain explicit.
//! 4. Incomplete analysis MUST NOT be reported as a successful proof.
//! 5. Assignment and driver checks MUST distinguish proven conflicts from
//!    conditions that remain indeterminate.
//! 6. Combinational dependency analysis MUST NOT silently treat sequential
//!    state feedback as a combinational dependency.
//! 7. Process and sequential semantics MUST preserve the grammar-defined
//!    event, clock, reset, and update relationships.
//! 8. State-machine models MUST NOT commit to physical state encoding.
//! 9. Target-specific infeasibility MUST NOT retroactively change source
//!    semantics.
//! 10. No module may introduce a universal hardware-capacity ceiling.
//!
//! ## POCO-REAF and scalability
//!
//! The architectural objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! No architectural maximum is imposed here on:
//!
//! - Signals, expressions, assignments, or processes.
//! - States, transitions, or clock domains.
//! - Dependency edges, ports, or hierarchy size.
//! - Bit widths, parameter values, or memory dimensions.
//! - Supported hardware families or registered extensions.
//!
//! The subsystem MUST NOT assume a particular CPU count, machine word
//! width, memory capacity, FPGA size, ASIC capacity, or device count.
//!
//! Concrete implementations remain bounded by available resources and
//! representable values. Callers may configure operational budgets where
//! appropriate. Exhaustion MUST be reported explicitly and MUST NOT result
//! in silently truncated analysis or a false success result.
//!
//! The absence of a universal ceiling does not imply physically infinite
//! computation, memory, or successful allocation.
//!
//! ## Determinism and diagnostics
//!
//! Where the public contracts require deterministic behavior, equivalent
//! inputs and equivalent configuration MUST yield equivalent outcomes and
//! deterministic diagnostic ordering.
//!
//! Source-language errors, unsupported analysis, indeterminate results,
//! operational-budget exhaustion, and infrastructure failures MUST remain
//! distinguishable.
//!
//! The owning modules produce their respective structured results.
//! This composition root does not introduce a competing diagnostic format.
//!
//! ## Integration contract
//!
//! The parent `crate::hdl` module MUST register this module exactly once.
//! Every declared child module MUST resolve to its corresponding existing
//! source file.
//!
//! The existing expression, assignment, combinational, sequential, process,
//! and state-machine modules remain the owners of their respective APIs.
//! This file does not duplicate their types or introduce blanket re-exports.
//!
//! Cross-module consumers MUST import APIs from their owning module paths.
//! This avoids ambiguous names and allows each contract to evolve under
//! its documented compatibility policy.
//!
//! Any new logic module MUST have a defined responsibility, normative
//! authority, explicit dependencies, error behavior, scalability policy,
//! tests, and integration contract before being registered here.
//!
//! ## Verification and freeze criteria
//!
//! This composition root may be considered integrated when:
//!
//! - All six declared child modules exist and compile.
//! - The parent HDL module exposes this module.
//! - Every child module's dependencies are registered at their actual
//!   canonical paths.
//! - No duplicate parser, AST, semantic authority, or IR is introduced.
//! - Public API paths are documented and intentional.
//! - The module dependency graph is acyclic.
//! - The logic contracts preserve symbolic and indeterminate outcomes.
//! - Tests cover cross-module integration and error propagation.
//! - The repository's required formatting, build, test, and lint checks pass.

// -----------------------------------------------------------------------------
// Logic subsystem modules
// -----------------------------------------------------------------------------
//
// These declarations register the existing implementations. Each module
// owns its types, functions, error contracts, and integration behavior.
//
// Deliberately avoid wildcard re-exports: consumers should use the module
// that owns the contract they need.

/// HDL expression evaluation and analysis contracts.
pub mod expression;

/// HDL assignment semantics and driver-conflict contracts.
pub mod assignment;

/// Combinational behavior, dependency, and coverage analysis.
pub mod combinational;

/// Sequential behavior and state-update contracts.
pub mod sequential;

/// HDL process and sensitivity contracts.
pub mod process;

/// Logical state-machine contracts and structural analysis.
pub mod state_machine;
