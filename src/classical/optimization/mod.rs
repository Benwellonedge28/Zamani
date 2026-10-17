
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Optimization Module Root.
//!
//! Path: `src/classical/optimization/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Register and expose the classical optimization subsystem's existing
//! modules through one stable, documented module boundary.
//!
//! This file composes optimization components. It does not implement a
//! second optimization engine or own the intermediate representation.
//!
//! # Architectural authority
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/resources/`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/statements/loops.g4`
//! - `src/classical/optimization/passes.rs`
//! - `src/classical/optimization/legality.rs`
//! - `src/quantum/ir/classical/`, the existing classical IR boundary
//! - `src/zuir/`, when selected by the compilation pipeline
//!
//! The normative language specifications define source semantics. The
//! selected canonical IR defines the representation consumed by optimization.
//! Grammar files define syntax and do not implement optimization behavior.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Registration of the optimization child modules.
//! - The public module-path boundary for optimization components.
//! - The documented dependency direction between optimization concerns.
//!
//! It does not own:
//!
//! - Source syntax, frontend AST, semantic analysis, or type inference.
//! - A competing classical IR, ZUIR, or runtime.
//! - Legality, alias, dependence, effect, lifetime, or numerical proofs.
//! - Physical hardware discovery, resource allocation, or placement.
//! - Scheduling, target-specific instruction selection, or execution.
//!
//! # Module contracts
//!
//! - `passes`: deterministic pass orchestration and execution accounting.
//! - `legality`: conservative assessment of supplied proof evidence.
//! - `cost_model`: deterministic ranking of eligible candidates using
//!   explicitly supplied estimates, objectives, and policies.
//! - `loop_transform`: target-independent loop transformation planning.
//! - `vectorization`: vectorization opportunity and plan descriptions.
//! - `parallelization`: parallelization opportunity and plan descriptions.
//! - `memory_planning`: logical storage-slot planning from trusted lifetime
//!   evidence, without performing concrete allocations.
//!
//! Each child module owns its own types, errors, validation rules, and tests.
//! This root does not duplicate or redefine those contracts.
//!
//! # Dependency direction
//!
//! ```text
//! Semantic analysis and canonical IR
//!                 |
//!                 v
//!       Legality and analysis evidence
//!                 |
//!                 v
//!  Loop / vector / parallel / memory plans
//!                 |
//!                 v
//!       Cost and feasibility assessment
//!                 |
//!                 v
//!       Pass orchestration and verification
//!                 |
//!                 v
//! Existing canonical IR and downstream lowering
//! ```
//!
//! This diagram describes responsibilities, not a requirement that every
//! optimization pass use every planning module. A pass must use only the
//! analyses and policies relevant to its transformation.
//!
//! # Correctness invariants
//!
//! 1. Cost estimates never establish semantic legality.
//! 2. Missing or inconclusive proof evidence never counts as proof.
//! 3. Target infeasibility must not silently change source-language meaning.
//! 4. Planning is distinct from applying a transformation.
//! 5. A concrete IR adapter must preserve its IR's invariants and must provide
//!    transactional behavior when a transformation can fail.
//! 6. Analyses and proofs must apply to the IR revision being transformed.
//! 7. A transformation must preserve applicable effects, memory behavior,
//!    numerical guarantees, concurrency semantics, and determinism.
//!
//! # Scalability and resource policy
//!
//! No universal semantic ceiling is imposed here on pass count, candidates,
//! loop iterations, vector widths, parallel workers, memory objects, or
//! computational dimensions.
//!
//! Finite host resources, representable values, target capabilities, and
//! explicit operational budgets still apply. Resource exhaustion must be
//! reported through the responsible component rather than disguised as an
//! invalid source program or silent semantic degradation.
//!
//! # Integration contract
//!
//! `src/classical/mod.rs` must declare the classical subsystem and expose
//! this module through `pub mod optimization;`.
//!
//! That parent file must first be valid Rust. In the inspected repository,
//! its current content is placeholder text rather than a Rust module root.
//!
//! The existing optimization files are registered below. They remain
//! independently responsible for their own implementation and tests.
//! Concrete passes must connect their plans to the selected canonical IR
//! through explicit adapters; this module root does not invent such adapters.
//!
//! # Compatibility policy
//!
//! Public paths such as `classical::optimization::cost_model` and
//! `classical::optimization::passes` are the primary API.
//!
//! New components should be added as separately owned modules when their
//! responsibilities warrant it. Existing module paths should not be renamed
//! or removed without an explicit compatibility review.
//!
//! # Completion status
//!
//! This module root is designed against the optimization files found in the
//! inspected repository. Its integration is not verified until the parent
//! module is repaired and the crate builds and tests successfully.

/// Candidate cost estimates, optimization objectives, budget assessment,
/// deterministic ranking, and selection reports.
///
/// Cost estimates do not prove legality or target feasibility.
pub mod cost_model;

/// Conservative transformation-legality assessments based on supplied
/// semantic proof evidence.
///
/// This module does not independently perform alias or dependence analysis.
pub mod legality;

/// Target-independent loop transformation requests, plans, and adapter
/// contracts.
///
/// Source syntax remains owned by the grammar and frontend.
pub mod loop_transform;

/// Logical memory layout planning from object descriptions and trusted
/// lifetime-disjointness evidence.
///
/// This module does not allocate memory or assign physical addresses.
pub mod memory_planning;

/// Parallelization opportunity assessment and target-independent planning.
///
/// Physical placement and worker scheduling remain downstream concerns.
pub mod parallelization;

/// Optimization-pass interfaces, deterministic registration, pass execution,
/// verification boundaries, and operational accounting.
///
/// Concrete passes must preserve their canonical IR's invariants.
pub mod passes;

/// Vectorization opportunity assessment and target-independent planning.
///
/// Physical SIMD instruction selection remains a target-lowering concern.
pub mod vectorization;
