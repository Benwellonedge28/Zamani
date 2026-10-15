
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation (ZUIR)
//!
//! Canonical path: `src/zuir/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden throughout this module tree.
//!
//! ## PURPOSE
//!
//! Define the canonical module boundary for Zamani's Universal Intermediate
//! Representation (ZUIR).
//!
//! ZUIR represents established computational meaning independently of a
//! particular execution target. It provides a shared representation and
//! extensible contracts for classical, quantum, HDL, tensor, AI/ML,
//! distributed, networking, cryptographic, and future computational domains.
//!
//! This file composes the implementation. It does not implement another IR,
//! parser, semantic model, optimizer, or runtime.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! The implementation must conform to the repository's established
//! architecture and the following specifications:
//!
//! - `DESIGN.md` — repository-wide architecture, where applicable.
//! - `src/zuir/ZUIR-SPEC.md` — ZUIR representation specification.
//! - `grammar/DESIGN.md` — language architecture and ownership boundaries.
//! - `grammar/Zamani.g4` — source grammar composition.
//! - `grammar/antlr/ZamaniLexer.g4` — lexer boundary.
//! - `grammar/antlr/ZamaniParser.g4` — parser boundary.
//! - `grammar/lexer/tokens.g4` — shared lexical token contracts.
//! - `grammar/specification/semantic-model.md` — source-language meaning.
//! - `grammar/specification/scalability-model.md` — scalability requirements.
//! - `grammar/specification/poco-reaf.md` — portability and POCO-REAF.
//!
//! The grammar defines syntax. The semantic model defines source meaning.
//! ZUIR represents validated computational meaning. Domain IRs and target
//! backends implement specialized lowering and execution.
//!
//! Where specifications conflict, the conflict must be resolved in the
//! authoritative specification. This module must not invent a second
//! authority or silently change the meaning of an existing construct.
//!
//! ## OWNS
//!
//! - The canonical ZUIR module hierarchy.
//! - Stable paths to the existing subsystem implementations.
//! - A small, deliberate root-level public API.
//! - The ZUIR-wide unsafe-code prohibition.
//! - The test-module integration point.
//!
//! ## DOES NOT OWN
//!
//! - Source grammar, tokens, AST construction, parsing, or name resolution.
//! - Semantic type inference or the language's source-level semantic model.
//! - Definitions of IR nodes, identifiers, types, values, or effects.
//! - Dialect operation semantics or extension execution.
//! - Verification, analysis, optimization, transformation, or pass algorithms.
//! - Serialization, migration, canonicalization, or fingerprint algorithms.
//! - Target discovery, physical resource allocation, routing, or scheduling.
//! - Classical IR, quantum IR, HDL IR, or runtime implementations.
//!
//! Those responsibilities belong to the relevant child modules or existing
//! repository subsystems. This file only connects their public boundaries.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! ZUIR must not impose arbitrary universal limits on program size, operation
//! count, logical qubits, processors, accelerators, memory, register width,
//! tensor rank, distributed nodes, topology, or computational domains.
//!
//! This module introduces no such limits. Actual scalability depends on the
//! implementations, address space, available resources, target capabilities,
//! and explicitly configured operational budgets.
//!
//! An absent configured budget does not promise literal infinity. No finite
//! implementation can guarantee execution for every possible computation.
//!
//! The same program meaning must be preserved across supported targets.
//! If a target cannot represent or realize a required semantic property, the
//! responsible verifier or lowering stage must reject the conversion or
//! report a documented, explicitly permitted alternative. It must not
//! silently discard or redefine the property.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! Zamani grammar and frontend
//!           |
//!           v
//!      AST / semantic model
//!           |
//!           v
//!          ZUIR
//!           |
//!           +--> core and classical dialects
//!           +--> quantum dialect and quantum::ir adapter
//!           +--> HDL dialect and hardware IR adapter
//!           +--> tensor / AI / distributed / networking / crypto dialects
//!           |
//!           v
//! verification, analysis, transformation, and lowering
//!           |
//!           v
//!       target IRs / runtimes
//! ```
//!
//! This diagram describes architectural boundaries, not a claim that every
//! adapter or end-to-end compilation path is already implemented.
//!
//! Lower-level foundational modules must not depend on this facade merely
//! to access their sibling implementations. Prefer direct owner-module paths
//! inside ZUIR; use root re-exports for external callers.
//!
//! ## COMPATIBILITY
//!
//! Child module paths are public API where declared `pub`. Renaming or
//! removing a public path requires a deliberate compatibility decision.
//! Root-level re-exports are selective; wildcard re-exports are intentionally
//! avoided to prevent accidental name collisions and ambiguous ownership.
//!
//! ## INTEGRATION REQUIREMENTS
//!
//! - `src/lib.rs` must declare `pub mod zuir;` exactly once.
//! - Each declared child module must have valid Rust source.
//! - `dialects/mod.rs` must declare the standard dialect modules; its current
//!   contents must be repaired before this module tree can compile.
//! - `tests/mod.rs` is included only for test builds.
//! - The conformance test module and all registered test modules must compile.
//! - Existing `quantum::ir` and classical IR implementations remain authoritative
//!   for their own domain models; ZUIR must not duplicate their types.
//!
//! ## FREEZE CRITERIA
//!
//! This file is frozen only when:
//!
//! - Every module declaration resolves to an existing, valid Rust module.
//! - All public re-exports resolve and have deliberate API ownership.
//! - The crate root exposes ZUIR without creating a duplicate module.
//! - All targets compile, all registered tests pass, and Clippy passes.
//! - Grammar/frontend-to-semantic-model-to-ZUIR boundaries are tested.
//! - Domain lowering preserves meaning or explicitly reports unsupported cases.
//! - No unsafe code or universal hardware-capacity ceiling is introduced.

// ---------------------------------------------------------------------------
// Foundational contracts
// ---------------------------------------------------------------------------

/// Stable typed identities for ZUIR entities.
pub mod ids;

/// ZUIR and serialized-artifact version contracts.
pub mod version;

/// Structured errors and result types.
pub mod errors;

/// Configuration, policies, and explicit operational work budgets.
pub mod config;

/// Structured diagnostics and diagnostic reporting contracts.
pub mod diagnostics;

/// Source and transformation provenance mappings.
pub mod source_map;

// ---------------------------------------------------------------------------
// Normative contracts and schema
// ---------------------------------------------------------------------------

/// Executable and documentary ZUIR specification contracts.
pub mod spec;

/// Schema-level descriptors, attributes, metadata, and extension manifests.
pub mod schema;

// ---------------------------------------------------------------------------
// Types, values, and canonical IR
// ---------------------------------------------------------------------------

/// Canonical semantic type subsystem.
pub mod types;

/// Portable values, literals, constants, symbols, and expressions.
pub mod values;

/// Canonical program, module, function, region, block, operation, and
/// associated semantic data structures.
pub mod ir;

// ---------------------------------------------------------------------------
// Extensibility and validated construction
// ---------------------------------------------------------------------------

/// Dialect definitions, operation schemas, interfaces, and registry.
pub mod dialect;

/// Standard dialect implementations.
///
/// The `dialects/mod.rs` file must declare each existing standard dialect
/// module explicitly. See the required integration changes below.
pub mod dialects;

/// Validated APIs for constructing ZUIR entities and programs.
pub mod builder;

// ---------------------------------------------------------------------------
// Verification and read-only analysis
// ---------------------------------------------------------------------------

/// Structural and semantic verification of ZUIR.
pub mod verify;

/// Derived control-flow, data-flow, effects, resources, and provenance facts.
pub mod analysis;

// ---------------------------------------------------------------------------
// Passes, transformations, and lowering
// ---------------------------------------------------------------------------

/// Pass contracts, registration, invalidation, and orchestration.
pub mod pass;

/// Semantics-preserving and legality-driven IR transformations.
pub mod transform;

/// Explicitly validated lowering interfaces and adapters.
pub mod lowering;

// ---------------------------------------------------------------------------
// Persistence, identity, and incremental compilation
// ---------------------------------------------------------------------------

/// Versioned ZUIR encoding, decoding, validation, and canonical serialization.
pub mod serialization;

/// Versioned content fingerprints.
pub mod fingerprint;

/// Dependency tracking, invalidation, and incremental cache identity.
pub mod incremental;

/// Registered import/export formats and representation interchange.
pub mod interchange;

// ---------------------------------------------------------------------------
// Public API facade
// ---------------------------------------------------------------------------

/// Deliberate public entry points for ZUIR clients.
///
/// Prefer this facade for supported high-level API use. Subsystem modules
/// remain public for callers that need their documented specialized APIs.
pub mod api;

/// Convenient access to the public API facade type.
pub use api::ZuirApi;

/// Convenient access to the immutable configuration contract.
pub use config::ZuirConfig;

/// Convenient access to structured error and result contracts.
pub use errors::{ZuirError, ZuirErrorCode, ZuirErrorKind, ZuirResult};

// ---------------------------------------------------------------------------
// Test integration
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
