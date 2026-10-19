
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Memory Subsystem
//!
//! Path: `src/hdl/memory/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Public composition root for the target-independent HDL memory subsystem.
//! The module registers the existing memory model, access analysis, and
//! observable memory-semantics contracts. It does not implement a second
//! memory representation.
//!
//! This API must be considered integration-ready, not production-certified,
//! until the complete workspace checks and HDL conformance tests pass.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/memories.g4`
//! - `grammar/hdl/parameters.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/`
//! - `src/zuir/types/type_def.rs`
//! - `src/zuir/types/layout.rs`
//!
//! The grammar owns source syntax. The canonical frontend owns AST nodes,
//! source identities, name resolution, and source-level semantic validation.
//! ZUIR owns canonical semantic type and dimension representations.
//! This module owns only the composition and public API of the HDL logical
//! memory subsystem.
//!
//! ## Module ownership
//!
//! - `model`: logical memory declarations, symbolic dimensions, element-type
//!   references, logical ports, initialization intent, and source-linked
//!   metadata.
//! - `access`: access-request contracts, logical port permissions, address
//!   rank and statically decidable bounds checks, and conservative address-
//!   overlap analysis.
//! - `semantics`: explicit contracts for initialization, access latency,
//!   read-during-write behavior, and write-collision behavior.
//!
//! These are separate responsibilities. In particular, access analysis does
//! not decide observable collision behavior, and memory semantics do not
//! choose a physical implementation.
//!
//! ## Does not own
//!
//! - Parsing, lexing, AST construction, source spans, or name resolution.
//! - A replacement type system, expression AST, or symbolic-expression engine.
//! - Memory contents, runtime storage allocation, or access execution.
//! - Physical addresses, banks, controllers, caches, or memory technologies.
//! - Arbitration, simulation scheduling, synthesis, placement, or routing.
//! - Target selection, hardware discovery, resource allocation, or feasibility.
//! - HDLIR data structures, builders, verifiers, passes, serialization, or
//!   lowering. All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration contract
//!
//! The frontend adapter maps validated source declarations into `HdlMemory`
//! using canonical `TypeRef`, `Dimension`, and `NodeId` values. It supplies
//! source identities and preserves dimension and declaration order.
//!
//! Access analysis consumes the declaration and source-derived access
//! requests. It must preserve unresolved symbolic and dynamic addresses as
//! indeterminate instead of treating them as valid or invalid by guesswork.
//!
//! Memory semantics consumes the declaration and access assessments. Any
//! behavioral policy not specified by the language or an explicit source
//! contract must remain unspecified; it must not silently become a physical
//! default.
//!
//! Resource analysis consumes the logical declaration without forcing
//! symbolic dimensions into host-sized integers. Target feasibility and
//! physical realization belong to downstream target-specific components.
//!
//! ## Scalability and portability
//!
//! This module imposes no universal maximum on memory dimensions, logical
//! ports, or access requests. It introduces no hardware-capacity constants
//! and makes no assumptions about FPGA block RAM, ASIC macros, or any
//! particular memory technology.
//!
//! Symbolic dimensions and canonical type references remain owned by ZUIR.
//! The subsystem does not calculate physical capacity or allocate storage.
//! Rust collections and the host process are finite; operations that exhaust
//! available resources must return an explicit error or incomplete result
//! through their owning API rather than silently changing program meaning.
//!
//! ## Determinism
//!
//! Public exports are explicit. The subsystem introduces no global mutable
//! state, pointer-derived identity, target discovery, or implicit physical
//! implementation policy.
//!
//! ## Compatibility and freeze criteria
//!
//! This module is complete when:
//!
//! - Every declared child module exists and compiles.
//! - The public re-exports resolve to the canonical implementations.
//! - The frontend adapter preserves memory declarations and source identities.
//! - Unresolved addresses and symbolic dimensions remain explicit.
//! - Behavioral defaults are not invented where the specification is silent.
//! - No duplicate type system, AST, source-map system, or IR is introduced.
//! - Formatting, workspace checks, tests, Clippy, and HDL conformance checks pass.

// ---------------------------------------------------------------------------
// Canonical memory subsystem modules
// ---------------------------------------------------------------------------

/// Target-independent access validation and address-overlap analysis.
///
/// This module does not execute accesses or determine observable memory
/// behavior. Those responsibilities belong to `semantics`.
pub mod access;

/// Logical memory declarations and their structural invariants.
///
/// This module does not allocate memory or select physical resources.
pub mod model;

/// Observable logical memory behavior and explicit behavioral policies.
///
/// This module does not schedule simulation events or implement physical
/// memory primitives.
pub mod semantics;

// ---------------------------------------------------------------------------
// Deliberate public API
//
// The child modules remain public for callers that prefer explicit paths.
// These re-exports provide a convenient, stable entry point without wildcard
// exports or a second implementation of any memory concept.
// ---------------------------------------------------------------------------

pub use access::{
    analyze_collision, validate_access, AccessCertainty, AddressOverlap,
    DimensionAddressCheck, IndeterminateReason, MemoryAccessAssessment,
    MemoryAccessError, MemoryAccessOperation, MemoryAccessRequest,
    MemoryAddressTerm, MemoryCollisionAssessment, MemoryCollisionKind,
};

pub use model::{
    HdlMemory, MemoryId, MemoryInitialization, MemoryMetadata,
    MemoryMetadataKind, MemoryModelError, MemoryPort, MemoryPortAccess,
    MemoryPortDirection, MemoryPortId, HDL_MEMORY_MODEL_SCHEMA_VERSION,
};

pub use semantics::{
    CollisionEffect, HDL_MEMORY_SEMANTICS_SCHEMA_VERSION, InitializationContract,
    MemoryLatency, MemorySemantics, MemorySemanticsError,
    ReadDuringWritePolicy, SimultaneousAccessAssessment, WriteCollisionPolicy,
};
