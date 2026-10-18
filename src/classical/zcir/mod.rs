
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Classical Intermediate Representation (ZCIR)
//!
//! Canonical path: `src/classical/zcir/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the public module boundary for Zamani's classical-computation
//! integration with the canonical Zamani Universal Intermediate
//! Representation (ZUIR).
//!
//! ZCIR exposes classical-facing contracts, adapters, verification,
//! analysis, construction, lowering, and serialization facilities through
//! the existing repository architecture. It must not create a competing
//! IR graph, identifier system, type authority, value-definition system,
//! or canonical serialization implementation.
//!
//! ## Architectural authority
//!
//! Source syntax and language semantics are governed by:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Canonical computational representation and shared infrastructure are
//! governed by `src/zuir/`, including its IDs, IR entities, types, values,
//! verification, analysis, dialects, passes, lowering, and serialization.
//!
//! The existing `src/quantum/ir/` and
//! `src/quantum/ir/classical/` boundaries must retain their established
//! ownership. ZCIR integration must not introduce duplicate canonical
//! classical representations or dependency cycles.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the ZCIR public submodules.
//! - The composition boundary for classical ZUIR adapters.
//! - The test-only module registry.
//!
//! This module does not own:
//!
//! - Source parsing, AST construction, name resolution, or semantic analysis.
//! - Canonical IR storage, entity identities, or SSA identity allocation.
//! - A competing classical type system or constant representation.
//! - Hardware selection, physical allocation, scheduling, or execution.
//! - A second serializer, schema-version authority, or canonical IR format.
//!
//! ## Dependency direction
//!
//! The intended direction is:
//!
//! ```text
//! Source grammar and frontend
//!           |
//!           v
//! Resolved semantic model
//!           |
//!           v
//! ZCIR classical integration boundary
//!           |
//!           v
//! Canonical ZUIR entities and verified contracts
//!           |
//!           v
//! Domain lowering and target-specific compilation
//!           |
//!           v
//! Runtime and execution environment
//! ```
//!
//! This diagram expresses responsibility boundaries, not a claim that
//! every pipeline stage or target backend is already implemented.
//!
//! ZCIR adapters may depend on ZUIR. ZUIR must not depend on ZCIR merely
//! to implement its canonical representation or shared infrastructure.
//! Cross-domain integrations must follow the repository's established
//! dependency direction and avoid cycles.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no universal maximum for operations, blocks,
//! functions, modules, values, memory, threads, processors, accelerators,
//! devices, or collection lengths.
//!
//! A program's meaning must not depend on arbitrary hardware ceilings.
//! Actual compilation and execution remain subject to available resources,
//! representable identities, operational budgets, implementation support,
//! program requirements, and target capabilities.
//!
//! Resource budgets must be explicit and must not silently change program
//! semantics. A target that cannot satisfy required capabilities or
//! resources must produce an explicit feasibility or compilation result.
//!
//! ## Public module contracts
//!
//! - `api`: public ZCIR-facing access to canonical APIs.
//! - `config`: configuration and operational-budget integration.
//! - `version`: version and compatibility metadata.
//! - `error`: classical-facing error contracts.
//! - `diagnostics`: structured diagnostic integration.
//! - `provenance`: compilation provenance and lineage contracts.
//! - `compatibility`: compatibility assessment and policy contracts.
//! - `capabilities`: target-independent capability requirements.
//! - `resources`: resource requirements and feasibility integration.
//! - `ids`: canonical identity integration.
//! - `source_map`: source-map adapter over canonical source locations.
//! - `types`: classical-facing type contracts and adapters.
//! - `values`: classical-facing value contracts and adapters.
//! - `ir`: canonical ZUIR entity re-exports for classical consumers.
//! - `builder`: validated construction interfaces.
//! - `verify`: verification entry points and reports.
//! - `analysis`: derived analysis and data-flow integration.
//! - `passes`: transformation orchestration and legality contracts.
//! - `lowering`: conversions and compatibility adapters.
//! - `serialization`: canonical ZUIR serialization integration.
//!
//! Each submodule owns its documented responsibilities. This root only
//! registers modules; it does not duplicate their implementation.
//!
//! ## Integration requirements
//!
//! 1. `src/classical/mod.rs` must expose this module using `pub mod zcir;`
//!    after this root and its declared children are available.
//! 2. Each declared module must exist at its corresponding Rust module path.
//! 3. Each child module must compile against the actual upstream APIs.
//! 4. ZCIR-facing IDs and IR entities must resolve to canonical ZUIR types.
//! 5. Lowering and serialization must preserve canonical identity and their
//!    owning implementations' documented errors and invariants.
//! 6. The frontend and semantic model remain upstream; this module must
//!    not parse source text or independently infer source-language meaning.
//! 7. The test registry must be present before test builds are expected to
//!    compile.
//!
//! ## Freeze criteria
//!
//! - Every registered module exists and compiles.
//! - Public paths and dependency directions are verified.
//! - No competing IR core or identity system is introduced.
//! - Unsafe Rust remains forbidden.
//! - Test, compilation, formatting, and lint gates pass.
//!
//! Registering a module does not prove that its functionality is complete,
//! that all classical operations are supported, or that POCO-REAF works
//! for every possible target.

/// Public ZCIR-facing API over canonical Zamani infrastructure.
pub mod api;

/// Target-independent configuration and operational-budget integration.
pub mod config;

/// Versioning and compatibility metadata.
pub mod version;

/// Classical-facing error contracts.
pub mod error;

/// Structured diagnostic integration.
pub mod diagnostics;

/// Compilation provenance and lineage contracts.
pub mod provenance;

/// Artifact and consumer compatibility contracts.
pub mod compatibility;

/// Capability requirements and target-capability integration.
pub mod capabilities;

/// Resource requirements and feasibility integration.
pub mod resources;

/// Canonical identity integration.
pub mod ids;

/// Source-map adapter over canonical source locations.
pub mod source_map;

/// Classical-facing type contracts and adapters.
pub mod types;

/// Classical-facing value contracts and adapters.
pub mod values;

/// Classical-facing boundary over canonical ZUIR entities.
pub mod ir;

/// Validated IR construction interfaces.
pub mod builder;

/// Structural and semantic verification integration.
pub mod verify;

/// Derived analysis and data-flow integration.
pub mod analysis;

/// Transformation orchestration and legality contracts.
pub mod passes;

/// ZCIR, ZUIR, and legacy compatibility adapters.
pub mod lowering;

/// Canonical serialization integration.
pub mod serialization;

#[cfg(test)]
mod tests;
