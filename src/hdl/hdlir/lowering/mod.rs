
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Lowering
//!
//! Path: `src/hdl/hdlir/lowering/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: HDLIR lowering composition root.
//!
//! ## Purpose
//!
//! Compose the explicit conversion boundaries between the canonical HDL
//! semantic model, Zamani's Universal Intermediate Representation (ZUIR),
//! and the hardware-domain Intermediate Representation (HDLIR).
//!
//! This module registers the individual lowering implementations and
//! establishes their public namespace. It does not implement conversion
//! algorithms, define another IR, or change the semantics of its inputs.
//!
//! ## Architectural authority
//!
//! The implementation must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/frontend/ast/`
//! - `src/frontend/semantic/`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/schema/`
//! - `src/hdl/hdlir/types/`
//! - `src/hdl/hdlir/values/`
//! - `src/hdl/hdlir/ir/`
//! - `src/hdl/hdlir/verify/`
//!
//! The grammar owns source syntax. The canonical frontend owns source-level
//! semantic analysis and its diagnostic and provenance contracts. ZUIR owns
//! universal computational representation. HDLIR owns hardware-domain
//! representation. Target realization owns physical implementation.
//!
//! These boundaries are authoritative; this module does not replace them.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the implemented lowering submodules.
//! - The public Rust namespace for HDLIR conversion APIs.
//! - The documented dependency direction between conversion boundaries.
//! - The architectural requirement that conversions preserve semantics or
//!   fail explicitly.
//!
//! This module does not own:
//!
//! - Lexing, parsing, AST construction, or source-level name resolution.
//! - A second semantic model or universal intermediate representation.
//! - HDLIR graph structures, types, values, IDs, or operation schemas.
//! - Conversion algorithms or transformation-preservation proofs.
//! - Physical resource allocation, synthesis, placement, routing, or timing
//!   closure.
//! - Hardware discovery, deployment, or runtime device handles.
//!
//! Every conversion implementation remains in its designated child module.
//!
//! ## Conversion boundaries
//!
//! ### `from_semantic`
//!
//! Converts canonical, validated HDL semantic input into the subset of HDLIR
//! that the implemented graph can represent without semantic loss.
//!
//! The adapter must use the existing frontend semantic API. It must not
//! parse source text, reconstruct the AST, redo name resolution, or invent
//! missing semantic facts.
//!
//! Constructs not representable by the available HDLIR graph must be
//! rejected explicitly. A successful conversion does not imply that every
//! HDL construct is supported.
//!
//! ### `from_zuir`
//!
//! Converts supported, verified ZUIR operations into HDLIR using explicit
//! source-to-destination operation and type mappings.
//!
//! This boundary must preserve relevant operand relationships, result types,
//! operation ordering, effects, attributes, regions, and extension payloads.
//! If a construct cannot be preserved by the implemented mapping, conversion
//! must fail rather than silently discard it.
//!
//! An operation-batch adapter must not claim to lower an entire ZUIR program
//! unless it actually handles the enclosing program, module, function,
//! region, block, control-flow, and value-reference contracts.
//!
//! ### `to_zuir`
//!
//! Provides the checked HDLIR-to-ZUIR conversion boundary.
//!
//! Conversion must use the canonical ZUIR API and appropriate registered
//! HDL dialect semantics. HDLIR constructs that cannot be represented
//! faithfully in the destination must produce an explicit failure.
//!
//! Local structural validation alone must not be treated as proof that
//! whole-design verification or semantic equivalence has succeeded.
//!
//! ### `contracts`
//!
//! Defines reusable mapping and preservation contracts for conversion
//! features and obligations.
//!
//! Declaring a mapping does not prove that an implementation preserves
//! semantics. Conversion implementations must provide evidence based on
//! checks they actually perform. They must not manufacture proof or
//! completion claims merely because a conversion returned successfully.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! canonical grammar and specifications
//!                  |
//!                  v
//!       canonical frontend semantics
//!                  |
//!          +-------+-------+
//!          |               |
//!          v               v
//!    semantic input       ZUIR
//!          |               |
//!          v               v
//!      from_semantic   from_zuir
//!          |               |
//!          +-------+-------+
//!                  |
//!                  v
//!                 HDLIR
//!                  |
//!                  v
//!               to_zuir
//!                  |
//!                  v
//!                 ZUIR
//! ```
//!
//! This diagram describes architectural responsibilities, not a guarantee
//! that all conversions or all language constructs are implemented.
//!
//! Child modules may depend on foundational HDLIR, frontend, and ZUIR APIs.
//! They must not depend on this composition root for foundational types.
//! This avoids unnecessary dependency cycles.
//!
//! ## Public API policy
//!
//! The four child modules are exposed through their explicit module paths:
//!
//! - `lowering::from_semantic`
//! - `lowering::from_zuir`
//! - `lowering::to_zuir`
//! - `lowering::contracts`
//!
//! Conversion-specific options, errors, results, mappings, and entry points
//! remain owned by their respective child modules.
//!
//! This composition root deliberately avoids wildcard re-exports and does
//! not create competing aliases for existing frontend, ZUIR, or HDLIR types.
//! Callers should use the documented public APIs of the relevant module.
//!
//! Adding a new conversion direction requires a separately reviewed contract,
//! an implemented adapter, appropriate tests, and deliberate registration
//! here. A proposed adapter or directory is not evidence of support.
//!
//! ## Semantic-preservation contract
//!
//! Every conversion must distinguish:
//!
//! - Invalid or incompletely validated source input.
//! - Unsupported source or destination constructs.
//! - Invalid HDLIR or ZUIR structures.
//! - Missing or incompatible schema and dialect definitions.
//! - Unrepresentable values and checked-arithmetic failures.
//! - Operational-budget exhaustion.
//! - Incomplete verification or analysis.
//! - Proven lossless conversion.
//! - Explicitly supported lossy conversion, where a separate contract
//!   permits it.
//!
//! These outcomes must not be collapsed into a generic successful result.
//!
//! A conversion must not silently:
//!
//! - Change signal, register, clock, reset, or memory semantics.
//! - Discard observable effects or state transitions.
//! - Weaken requirements or constraints.
//! - Replace unknown operations with guessed implementations.
//! - Convert unresolved symbolic quantities into arbitrary concrete values.
//! - Lose source provenance when provenance preservation is required.
//! - Treat incomplete processing as successful validation.
//!
//! A successful lowering result establishes only the guarantees explicitly
//! provided by that adapter. It does not automatically prove synthesis
//! feasibility, target compatibility, physical timing closure, or execution.
//!
//! ## Scalability and POCO-REAF
//!
//! The lowering subsystem must support designs that grow with available
//! memory, representable identifiers, supported semantics, and caller-supplied
//! operational budgets.
//!
//! It must not introduce universal hardware limits for:
//!
//! - Signals, ports, modules, instances, or operations.
//! - Bit widths, aggregate dimensions, or memory capacities.
//! - Clock domains, pipeline stages, or target resource quantities.
//!
//! Symbolic widths, parameters, and resource expressions must remain symbolic
//! until their owning stage can evaluate them correctly.
//!
//! Counts and index conversions must use checked arithmetic where required.
//! Allocation failure, identity exhaustion, and budget exhaustion must not
//! produce wrapped values, reused identities, silently truncated designs, or
//! false success.
//!
//! An absent optional budget does not promise infinite execution resources.
//! No finite implementation can process literally infinite input.
//!
//! The objective is to preserve one program's defined meaning across
//! compatible compilation paths and hardware targets, while reporting
//! unsupported capabilities and infeasible realizations explicitly.
//!
//! ## Determinism and concurrency
//!
//! Where an adapter promises deterministic conversion, equivalent canonical
//! inputs, schema versions, mappings, and configuration must produce
//! equivalent results and stable diagnostics.
//!
//! Implementations must not rely on incidental hash-map iteration order,
//! memory addresses, hardware discovery order, or wall-clock time to determine
//! semantic output.
//!
//! Mutable ID allocators and mutable graph builders must follow their own
//! ownership and synchronization contracts. This module introduces no global
//! mutable state and makes no unsupported thread-safety guarantees.
//!
//! ## Error propagation
//!
//! Each adapter must preserve its documented structured error boundary.
//! Errors must not be converted into success, silently swallowed, or reduced
//! to display strings when callers need machine-readable classifications.
//!
//! If a child adapter returns a structured failure, callers must retain the
//! distinction between invalid input, unsupported conversion, exhausted
//! budgets, and incomplete work.
//!
//! This composition root does not define a second error enum. Shared HDLIR
//! failures remain owned by `crate::hdl::hdlir::errors`, while frontend and
//! ZUIR errors remain owned by their respective subsystems.
//!
//! ## Integration and completion criteria
//!
//! This file is complete for its current scope when:
//!
//! 1. Each declared child module exists and compiles.
//! 2. Each child module uses verified repository APIs rather than assumed
//!    constructors, fields, or conversion semantics.
//! 3. The parent `crate::hdl::hdlir` composition root registers this module.
//! 4. Public API visibility and module paths are intentional and documented.
//! 5. No duplicate IR, semantic model, source-span type, or error system is
//!    introduced.
//! 6. Unsupported and incomplete conversions remain explicit.
//! 7. Focused lowering tests and applicable repository-wide checks pass.
//!
//! Completion of this composition root does not certify the completeness of
//! any child adapter or the production readiness of the entire HDLIR system.

/// Conversion from the canonical, validated HDL semantic model into HDLIR.
///
/// The implementation and its conversion-specific types are owned by
/// `from_semantic`.
pub mod from_semantic;

/// Conversion of explicitly mapped ZUIR operations into HDLIR.
///
/// The implementation and its conversion-specific types are owned by
/// `from_zuir`.
pub mod from_zuir;

/// Reusable mapping, fidelity, and semantic-preservation contracts.
///
/// These contracts describe obligations; they do not themselves prove
/// that a conversion is lossless.
pub mod contracts;

/// Checked conversion from HDLIR into the canonical ZUIR representation.
///
/// The implementation and its conversion-specific types are owned by
/// `to_zuir`.
pub mod to_zuir;
