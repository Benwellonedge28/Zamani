
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![warn(missing_docs)]

//! # Zamani Classical Computing — Control-Flow Planning
//!
//! **Path:** `src/classical/control_flow/mod.rs`
//!
//! **Language:** Rust 2021
//!
//! **Minimum Rust version:** 1.97
//!
//! **Safety:** Safe Rust only.
//!
//! **Status:** Proposed integration boundary; full workspace verification
//! is required before production release.
//!
//! ## 1. Purpose
//!
//! This module is the public composition boundary for target-independent
//! classical control-flow planning in Zamani.
//!
//! It exposes the existing, specialized planning modules for:
//!
//! - conditional branches and ordered conditional chains;
//! - condition-controlled and iterator-controlled loops;
//! - function entry and return-flow planning;
//! - direct and indirect call-site planning;
//! - exception and handler-flow planning;
//! - return, break, and continue control transfers.
//!
//! These structures describe semantically resolved relationships using
//! canonical Zamani identities. They are planning adapters, not a second
//! abstract syntax tree, control-flow graph, intermediate representation,
//! or execution engine.
//!
//! ## 2. Architectural authority
//!
//! Responsibilities are deliberately separated.
//!
//! ### Source syntax
//!
//! The grammar subsystem owns the source-language productions:
//!
//! - `grammar/statements/conditionals.g4`;
//! - `grammar/statements/loops.g4`;
//! - `grammar/statements/control-flow.g4`;
//! - `grammar/statements/returns.g4`;
//! - `grammar/statements/breaks.g4`;
//! - `grammar/statements/continues.g4`;
//! - `grammar/statements/exceptions.g4`;
//! - `grammar/functions/functions.g4`;
//! - `grammar/expressions/calls.g4`.
//!
//! These files define syntax and composition, not runtime execution.
//!
//! The actual repository grammar paths and their composition must remain
//! authoritative; this list documents integration responsibilities and
//! must not be interpreted as proof that every grammar file is present or
//! that every grammar production is implemented end to end.
//!
//! ### Frontend and semantic analysis
//!
//! The frontend owns parsed AST nodes and source locations.
//!
//! Semantic analysis owns name resolution, type checking, function and
//! overload resolution, generic inference, ownership and borrowing,
//! effect checking, exception compatibility, loop-context checking,
//! reachability analysis, and the legality of control transfers.
//!
//! A successfully constructed planning value is not proof that these
//! semantic checks have passed.
//!
//! ### Canonical identities
//!
//! `crate::classical::ids` is the classical-facing identity facade.
//!
//! The canonical ZUIR identity subsystem owns identity representation,
//! allocation, and identity invariants. This module must not define
//! replacement `FunctionId`, `BlockId`, `ValueId`, `OperationId`,
//! `SymbolId`, or `TypeId` types.
//!
//! An identifier is an opaque identity, not an array index, source
//! position, memory address, or execution-order guarantee.
//!
//! ### Canonical intermediate representation
//!
//! Canonical ZUIR owns the authoritative representation of functions,
//! operations, basic blocks, edges, terminators, and control-flow graphs.
//!
//! The control-flow planning layer must not construct a parallel IR.
//! The canonical builder and verifier must validate referenced entities,
//! edge consistency, dominance, terminator legality, and other applicable
//! IR invariants.
//!
//! `quantum::ir` remains the canonical quantum semantic boundary. This
//! module must not create a competing quantum IR or introduce a dependency
//! cycle between grammar, classical planning, and canonical IR.
//!
//! ## 3. Ownership contract
//!
//! This module owns:
//!
//! - public composition of the control-flow planning modules;
//! - the stable, documented re-export boundary for their public types;
//! - the module-level integration and compatibility contract.
//!
//! This module does not own:
//!
//! - source grammar or AST node definitions;
//! - expression evaluation or condition typing;
//! - symbol resolution or function overload resolution;
//! - identity allocation or identity ownership;
//! - canonical IR instructions, blocks, edges, or terminators;
//! - control-flow graph construction or verification;
//! - runtime execution, stack management, or exception dispatch;
//! - optimization, scheduling, placement, or hardware discovery;
//! - resource allocation, target selection, or backend lowering.
//!
//! ## 4. Public module structure
//!
//! Each component remains independently responsible for its own
//! structures and construction invariants. This file only composes
//! those modules and exposes their existing public APIs.
//!
//! The module boundaries are:
//!
//! `branch`
//!     BranchPlan, ConditionalArm, BranchChain
//!
//! `loops`
//!     WhileLoopPlan, ForLoopPlan, LoopPlan
//!
//! `functions`
//!     FunctionControlFlowPlan
//!
//! `calls`
//!     CallTarget, CallSitePlan
//!
//! `exceptions`
//!     ThrowPlan, CatchHandlerPlan, TryExceptionPlan
//!
//! `termination`
//!     ReturnPlan, BreakPlan, ContinuePlan, TerminationPlan
//!
//! ## 5. Integration contract
//!
//! Upstream producers:
//!
//! - frontend AST lowering;
//! - name and type resolution;
//! - function and call resolution;
//! - effect, ownership, and capability analysis;
//! - semantic control-flow validation.
//!
//! Downstream consumers:
//!
//! - `crate::classical::validation` for applicable semantic invariants;
//! - `crate::classical::lowering::control_flow` for lowering adapters;
//! - the canonical ZUIR builder and verifier;
//! - later optimization and target-lowering stages.
//!
//! The exact integration APIs must be checked against the repository's
//! actual definitions before consumers are wired together.
//!
//! The public exports below deliberately reuse existing types. They do
//! not presume that every downstream consumer is already implemented.
//!
//! ## 6. Error contract
//!
//! Component constructors may use `crate::classical::error::ClassicalError`
//! and `ClassicalResult<T>` for their documented local validation failures.
//!
//! This module does not define another error type or silently convert
//! semantic failures into successful plans.
//!
//! Errors concerning missing IR entities, invalid references, type
//! mismatches, illegal control transfers, or invalid graph structure must
//! be reported by the subsystem that has enough context to establish the
//! failure.
//!
//! ## 7. POCO-REAF and scalability
//!
//! Control-flow planning must not impose universal fixed limits on:
//!
//! - the number of branches or conditional arms;
//! - loop count, iteration count, or nesting count;
//! - functions, calls, or control-flow blocks;
//! - exception handlers or return sites;
//! - CPU, GPU, FPGA, QPU, accelerator, or distributed-node counts;
//! - memory capacity, register width, or collection size.
//!
//! Collections must be sized according to supplied input and available
//! resources. The absence of a language-level ceiling does not guarantee
//! infinite memory, infinite execution time, or successful allocation.
//!
//! Explicit compiler budgets, cancellation, resource policies, and
//! target-specific constraints belong to their respective owners.
//! They must not silently change the meaning of a valid program.
//!
//! ## 8. Determinism and compatibility
//!
//! Public re-exports must preserve the identity and behavior of the
//! underlying component types.
//!
//! Ordered conditional arms and other semantically ordered inputs must
//! preserve their specified order. No export or adapter may reorder
//! control flow unless a downstream transformation proves that doing so
//! preserves the language's observable semantics.
//!
//! Adding or removing a public export is an API change and requires
//! compatibility review. Changing a component's semantics belongs to
//! that component's owner and requires corresponding tests.
//!
//! ## 9. Completion criteria
//!
//! This module is complete only when:
//!
//! 1. Every declared component exists and compiles.
//! 2. Every re-export names an existing public item.
//! 3. The parent classical module declares this module.
//! 4. Canonical identity and error ownership are preserved.
//! 5. No duplicate AST, IR, CFG, or identity allocator is introduced.
//! 6. Public paths are covered by API and compatibility tests.
//! 7. Component and integration tests pass on the supported toolchain.
//! 8. Workspace formatting, compilation, tests, and lint checks pass.
//!
//! A successful module-level compile alone does not establish that the
//! whole classical subsystem is production-ready.

// -------------------------------------------------------------------------
// Component modules
// -------------------------------------------------------------------------

/// Conditional branch and ordered conditional-chain planning.
pub mod branch;

/// Resolved function call-site planning.
pub mod calls;

/// Function entry and return-flow planning.
pub mod functions;

/// Exception and handler-flow planning.
pub mod exceptions;

/// Structured `while` and `for` loop planning.
pub mod loops;

/// Resolved `return`, `break`, and `continue` planning.
pub mod termination;

// -------------------------------------------------------------------------
// Stable public re-exports
// -------------------------------------------------------------------------

/// Public conditional branch planning types.
pub use branch::{BranchChain, BranchPlan, ConditionalArm};

/// Public call-site planning types.
pub use calls::{CallSitePlan, CallTarget};

/// Public function control-flow planning type.
pub use functions::FunctionControlFlowPlan;

/// Public exception-flow planning types.
pub use exceptions::{CatchHandlerPlan, ThrowPlan, TryExceptionPlan};

/// Public loop planning types.
pub use loops::{ForLoopPlan, LoopPlan, WhileLoopPlan};

/// Public control-transfer planning types.
pub use termination::{BreakPlan, ContinuePlan, ReturnPlan, TerminationPlan};
