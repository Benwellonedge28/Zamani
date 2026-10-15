
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation — Analysis
//!
//! Canonical path: `src/zuir/analysis/mod.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: Rust 1.97 or later.
//! Safety policy: unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Production-oriented module registry. Freeze after parent-module
//! integration, workspace compilation, and analysis conformance tests pass.
//!
//! PURPOSE
//!
//! Provide the canonical module boundary for analyses over ZUIR.
//!
//! Analyses derive facts from canonical IR entities. They must not create a
//! competing IR, replace the semantic model, redefine source syntax, or
//! silently change program meaning.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/verify/`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/data/provenance.g4`
//! - `grammar/expressions/provenance.g4`
//! - `grammar/compile/provenance.g4`
//! - `grammar/security/provenance.g4`
//!
//! OWNERSHIP
//!
//! This module owns only the public module registry for ZUIR analyses.
//! Each child module owns its own analysis algorithms, result types,
//! validation rules, and analysis-specific errors.
//!
//! DOES NOT OWN
//!
//! - Canonical IR types, values, identifiers, or source maps.
//! - Source parsing, AST construction, or semantic name resolution.
//! - Authoritative whole-program verification.
//! - Domain-specific operation semantics.
//! - Physical hardware discovery, allocation, routing, or scheduling.
//! - Runtime execution, persistence, or backend code generation.
//!
//! ANALYSIS MODULE CONTRACTS
//!
//! `cfg`
//!     Builds a region-local control-flow graph from canonical control flow.
//!
//! `def_use`
//!     Builds and queries canonical definition/use information.
//!
//! `dominance`
//!     Computes dominance facts over a validated control-flow graph.
//!
//! `effects`
//!     Summarizes declared effects while preserving uncertainty about
//!     whether the analyzed scope is complete.
//!
//! `liveness`
//!     Computes region-local value liveness over the CFG and its block and
//!     edge arguments.
//!
//! `provenance`
//!     Analyzes source-origin associations from the canonical SourceMap.
//!     Source locations alone do not prove complete computational lineage,
//!     semantic preservation, trust, or authenticity.
//!
//! `resources`
//!     Analyzes canonical symbolic resource intents. It does not perform
//!     physical allocation or guarantee that a target can satisfy them.
//!
//! DEPENDENCY DIRECTION
//!
//! Child analyses may depend on canonical IDs, canonical IR, and other
//! lower-level analysis interfaces where explicitly required.
//!
//! Canonical IR and source-map modules must not depend on this registry.
//! The registry must not implement algorithms or duplicate child-module
//! data structures.
//!
//! PUBLIC API
//!
//! The child modules are exposed through their namespaces. This avoids
//! ambiguous names and lets each analysis evolve its own API deliberately.
//!
//! Examples:
//!
//!     crate::zuir::analysis::cfg::ControlFlowGraph
//!     crate::zuir::analysis::dominance::DominanceInfo
//!     crate::zuir::analysis::effects::EffectAnalysis
//!     crate::zuir::analysis::liveness::LivenessAnalysis
//!     crate::zuir::analysis::provenance::ProvenanceAnalysis
//!     crate::zuir::analysis::resources::ResourceAnalysis
//!
//! The exact public types and function signatures remain owned by the
//! corresponding child modules.
//!
//! DETERMINISM
//!
//! Deterministic behavior is defined by each analysis contract. This module
//! does not imply that every analysis is complete, deterministic under every
//! possible input, or independent of caller-provided traversal order.
//!
//! COMPLETENESS AND TRUST
//!
//! An analysis result proves only what its own contract states about the
//! supplied scope. An empty or partial result must not be treated as proof
//! of purity, complete provenance, resource feasibility, whole-program
//! validity, or the absence of unmodeled behavior.
//!
//! Run the authoritative verifier before relying on facts that require a
//! structurally and semantically valid IR.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This module imposes no fixed limit on operations, values, blocks, regions,
//! functions, modules, source origins, resources, qubits, processors,
//! accelerators, tensor dimensions, or distributed nodes.
//!
//! Capacity is bounded by available memory, address space, time, and
//! explicitly configured compiler safety budgets. Literal infinite capacity
//! is not possible; exhaustion must be handled by the relevant operation
//! or analysis rather than by a hidden universal machine-size assumption.
//!
//! SAFETY
//!
//! Unsafe Rust is prohibited. Each child module is also required to preserve
//! this policy independently.
//!
//! INTEGRATION
//!
//! `src/zuir/mod.rs` must declare `pub mod analysis;` after it is converted
//! into valid Rust source. The crate root must declare `zuir` exactly once.
//!
//! New analysis modules must be added here only after their implementation,
//! ownership contract, dependencies, and tests exist. Do not register
//! placeholder modules or declare files that have not been created.
//!
//! FREEZE CRITERIA
//!
//! - Every declared child module exists.
//! - Every child module has a documented responsibility and stable API.
//! - There are no circular dependencies introduced by this registry.
//! - The parent ZUIR module and crate root compile successfully.
//! - Relevant unit, integration, determinism, and stress tests pass.
//! - No unsafe code or fixed hardware-capacity assumptions are introduced.

/// Control-flow graph construction and region-local graph analysis.
pub mod cfg;

/// Definition/use indexing and queries for canonical IR entities.
pub mod def_use;

/// Dominance and related control-flow facts.
pub mod dominance;

/// Analysis of declared operation effects and effect contracts.
pub mod effects;

/// Region-local liveness analysis.
pub mod liveness;

/// Source-origin provenance analysis.
pub mod provenance;

/// Symbolic resource-intent analysis.
pub mod resources;
