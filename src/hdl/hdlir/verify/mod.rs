
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Verification
//!
//! Path: `src/hdl/hdlir/verify/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for verification of Zamani's
//! hardware-domain intermediate representation (HDLIR).
//!
//! It exposes the independently implemented verification subsystems through
//! one stable module boundary. It does not implement a second IR, duplicate
//! individual verification algorithms, or replace the canonical frontend's
//! semantic validation.
//!
//! ## Architectural authority
//!
//! Verification MUST remain consistent with:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/`
//! - `src/hdl/hdlir/schema/`
//! - `src/hdl/hdlir/types/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns source syntax. The canonical frontend owns source-level
//! semantic analysis. ZUIR owns the universal computational representation.
//! HDLIR owns the hardware-domain representation and its verification.
//!
//! This module MUST NOT introduce another parser, AST, semantic model,
//! source-span system, type registry, identity allocator, or hardware graph.
//!
//! ## Verification responsibilities
//!
//! The child modules provide the following independent verification services:
//!
//! - `structure`: graph structure, references, identity consistency, and
//!   exclusive ownership.
//! - `types`: value definitions, type identities, operation operands/results,
//!   and type-context-dependent constraints.
//! - `connectivity`: logical endpoint resolution, driver/sink rules,
//!   connection consistency, and endpoint type compatibility.
//! - `processes`: process declarations, process-body relationships, events,
//!   and process-kind-specific contracts.
//! - `clocks`: clock/reset source relationships, clock domains, and
//!   clocking contracts.
//! - `resources`: logical resource requirements, symbolic constraints,
//!   and resource-contract consistency.
//!
//! Each subsystem remains authoritative for its own checks. A successful
//! result from one subsystem MUST NOT be interpreted as proof that the
//! other subsystems have passed.
//!
//! In particular, structural validity does not prove type correctness;
//! type correctness does not prove connectivity correctness; and completed
//! HDLIR verification does not prove target feasibility, synthesis success,
//! physical timing closure, or successful hardware execution.
//!
//! ## Ownership and dependency direction
//!
//! This module owns:
//!
//! - Registration of the verification modules.
//! - The deliberate public verification API.
//! - Documentation of verification boundaries and integration requirements.
//!
//! This module does NOT own:
//!
//! - IR node definitions or graph mutation.
//! - Individual verification algorithms.
//! - Canonical frontend semantic validation.
//! - Schema or dialect registration.
//! - Physical resource allocation, scheduling, placement, or routing.
//! - Synthesis, timing closure, or target-specific realization.
//! - A second diagnostics, source-location, or identity system.
//!
//! Verification modules MUST consume the existing HDLIR and canonical
//! repository contracts through their declared interfaces.
//!
//! ## Completion and partial results
//!
//! Each verification entry point defines its own error, report, and
//! operational-budget contract. Callers MUST preserve those contracts.
//!
//! A caller MUST NOT:
//!
//! - Treat an exhausted work budget as successful verification.
//! - Ignore a failed required verification stage.
//! - Interpret unresolved symbolic constraints as proven satisfied.
//! - Silently accept an unknown operation or unsupported semantic contract.
//! - Claim that checks ran when their context or inputs were unavailable.
//!
//! When a check returns an incomplete report or an error, the enclosing
//! compiler stage MUST preserve that outcome.
//!
//! ## Scalability
//!
//! This module imposes no fixed maximum on signals, ports, modules,
//! operations, values, clock domains, processes, memories, connections,
//! widths, or logical hardware resources.
//!
//! Individual verification subsystems may accept explicit work budgets.
//! Those budgets limit operational effort, not the meaning or expressiveness
//! of the HDL language. Exhaustion MUST be reported explicitly.
//!
//! Finite software cannot process literally infinite input. Actual processing
//! is limited by available resources, representable data, supported
//! semantics, and caller-configured operational budgets.
//!
//! ## Determinism
//!
//! This composition root introduces no global mutable state, nondeterministic
//! registry, pointer-derived identity, or hidden verification cache.
//!
//! Each child subsystem is responsible for its documented traversal order,
//! deterministic findings, and checked work accounting. Any future aggregate
//! verifier MUST define deterministic stage ordering and preserve the outcomes
//! of every required stage.
//!
//! ## Integration contract
//!
//! The parent module `crate::hdl::hdlir` MUST register this module only after
//! the HDLIR module tree and its dependencies are valid Rust modules.
//!
//! Existing consumers SHOULD import a specific verification entry point or
//! context contract from this module instead of duplicating verification
//! logic in `src/hdl/mod.rs`, `src/hdl/api.rs`, the frontend, ZUIR, or a
//! backend.
//!
//! A caller integrating multiple verification stages is responsible for
//! supplying consistent graph snapshots and concrete schema/type/context
//! implementations to each applicable stage.
//!
//! The caller MUST NOT mutate the graph during verification unless the
//! relevant API explicitly provides a safe snapshot or synchronization
//! contract.
//!
//! ## Production qualification
//!
//! This file integrates the verification module tree at the Rust API level.
//! Production readiness additionally requires the complete dependency graph,
//! all child modules, their adapters, and the repository's tests to compile
//! and pass together on the supported toolchain.
//!
//! The presence of this file alone MUST NOT be treated as evidence that every
//! verification category is implemented or that the entire compiler pipeline
//! is production-ready.

// -----------------------------------------------------------------------------
// Verification subsystem registration
// -----------------------------------------------------------------------------
//
// Keep each subsystem independently owned. Do not copy its algorithms into
// this module. A subsystem is registered here only as a real module under
// `src/hdl/hdlir/verify/`.

/// Whole-graph structural verification.
pub mod structure;

/// Type and value-definition verification.
pub mod types;

/// Logical connectivity verification.
pub mod connectivity;

/// Behavioral process verification.
pub mod processes;

/// Clock-domain and reset verification.
pub mod clocks;

/// Logical resource-contract verification.
pub mod resources;

// -----------------------------------------------------------------------------
// Deliberate public API
// -----------------------------------------------------------------------------
//
// These re-exports provide convenient access to the existing subsystem APIs.
// The subsystem modules remain public so callers can also access additional
// documented types without this composition root having to duplicate them.

// Structural verification.

pub use self::structure::{
    verify_structure,
    StructureIssue,
    StructureIssueKind,
    StructureVerificationOptions,
    StructureVerificationReport,
};

// Type verification.

pub use self::types::{
    verify_types,
    HdlirTypeContext,
    TypeVerificationBudget,
    TypeVerificationReport,
};

// Connectivity verification.

pub use self::connectivity::{
    verify_connectivity,
    ConnectivityContext,
    ConnectivityEndpointInfo,
    ConnectivityTypeCompatibility,
    ConnectivityVerificationBudget,
    ConnectivityVerificationReport,
};

// Process verification.

pub use self::processes::{
    verify_processes,
    ProcessVerificationBudget,
    ProcessVerificationContext,
    ProcessVerificationReport,
};

// Clock and reset verification.

pub use self::clocks::{
    verify_clocks,
    ClockSourceRole,
    ClockVerificationBudget,
    ClockVerificationContext,
    ClockVerificationReport,
};

// Resource-contract verification.

pub use self::resources::{
    verify_resources,
    HdlirResourceVerificationContext,
    ResourceSubjectReport,
    ResourceVerificationBudget,
    ResourceVerificationReport,
    ResourceWorkMeter,
};

// -----------------------------------------------------------------------------
// Integration guidance
// -----------------------------------------------------------------------------
//
// There is intentionally no generic `verify_everything()` function here yet.
//
// The verification subsystems have different input requirements and context
// contracts. In particular, type, connectivity, process, clock, and resource
// checks rely on graph- or schema-aware context implementations. Calling them
// without those implementations would either be impossible or risk skipping
// required semantic checks.
//
// A future aggregate entry point may be added only after its contract defines:
// - which checks are mandatory and which are explicitly optional;
// - how each check receives a consistent graph snapshot;
// - how shared and per-stage budgets are accounted for;
// - how errors, findings, unresolved symbolic facts, and incomplete outcomes
//   are combined without losing information;
// - the deterministic execution order;
// - whether later checks continue after an earlier failure;
// - how verification results are associated with graph/schema versions;
// - how callers distinguish completed verification from target feasibility.
//
// Until those contracts exist, callers should invoke the applicable verified
// subsystem APIs explicitly and preserve every returned result.
