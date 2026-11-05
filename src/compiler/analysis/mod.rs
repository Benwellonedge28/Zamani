
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Meta-Compiler — Analysis
//!
//! Path:
//!     src/compiler/analysis/mod.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the public composition boundary for compiler analysis.
//!
//! It exposes the independently implemented compiler analysis subsystems
//! through one stable namespace.
//!
//! Compiler analyses establish facts about a program that are subsequently
//! consumed by verification, optimization, compilation planning, lowering,
//! backend realization, and other compiler stages.
//!
//! This module coordinates their public availability without implementing
//! another analysis engine or duplicating the authoritative models owned
//! by other repository subsystems.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//!                         Zamani source
//!                              |
//!                              v
//!                         Frontend AST
//!                              |
//!                              v
//!                     Semantic validation
//!                              |
//!                              v
//!                    Canonical semantic model
//!                              |
//!                              v
//!                       Canonical ZUIR
//!                              |
//!                              v
//!                  COMPILER ANALYSIS SUBSYSTEM
//!                              |
//!           +------------------+------------------+
//!           |         |        |        |         |
//!           v         v        v        v         v
//!        Program  Dependency Effects Ownership Capabilities
//!           |         |        |        |         |
//!           +---------+--------+--------+---------+
//!                              |
//!                              v
//!                   Analysis results and facts
//!                              |
//!                              v
//!                 Verification / Optimization
//!                              |
//!                              v
//!                   Compilation planning
//!                              |
//!                              v
//!                       Lowering
//!                              |
//!                              v
//!                  Backend / Realization
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Public analysis-module composition.
//! - Stable module-qualified access to compiler analyses.
//! - Analysis subsystem documentation.
//! - The compiler analysis namespace.
//!
//! This module does NOT own:
//!
//! - Source grammar or parsing.
//! - AST construction.
//! - Name or symbol resolution.
//! - Type inference or semantic type definitions.
//! - Semantic ownership declarations.
//! - Canonical effect definitions.
//! - Canonical capability definitions.
//! - Resource requirement semantics.
//! - Dependency graph storage.
//! - Canonical ZUIR entities.
//! - Control-flow graph construction.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Target selection.
//! - Optimization algorithms.
//! - Backend code generation.
//! - Runtime scheduling.
//! - Distributed execution.
//!
//! Those responsibilities remain with their respective authoritative
//! repository subsystems.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! PROGRAM ANALYSIS
//!
//!     src/compiler/analysis/program.rs
//!     src/zuir/ir/program.rs
//!     src/zuir/ir/module.rs
//!
//! DEPENDENCY ANALYSIS
//!
//!     src/compiler/analysis/dependency.rs
//!     src/compiler/context/dependency_context.rs
//!     src/frontend/semantic/
//!     src/compiler/incremental.rs
//!
//! EFFECT ANALYSIS
//!
//!     src/compiler/analysis/effects.rs
//!     src/zuir/ir/effects.rs
//!     src/zuir/analysis/effects.rs
//!
//! OWNERSHIP ANALYSIS
//!
//!     src/compiler/analysis/ownership.rs
//!     src/zuir/ir/ownership.rs
//!     src/zuir/analysis/cfg.rs
//!     src/zuir/verify/ownership.rs
//!     src/zuir/ir/use_def.rs
//!
//! CAPABILITY ANALYSIS
//!
//!     src/compiler/analysis/capabilities.rs
//!     src/frontend/semantic/capabilities/
//!     src/backend/compatibility.rs
//!
//! ============================================================================
//! SEMANTIC AND GRAMMAR AUTHORITY
//! ============================================================================
//!
//! Normative grammar:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/antlr/ZamaniLexer.g4
//!
//! Normative semantic specifications:
//!
//!     grammar/specification/semantic-model.md
//!     grammar/specification/scalability-model.md
//!     grammar/spec/resources.md
//!
//! Semantic implementation:
//!
//!     src/semantic.rs
//!     src/frontend/semantic/
//!
//! Canonical IR:
//!
//!     src/zuir/
//!
//! This module consumes analysis information established by those
//! authorities. It MUST NOT redefine the meaning of source constructs,
//! semantic identities, canonical effects, or IR operations.
//!
//! ============================================================================
//! ANALYSIS RESPONSIBILITIES
//! ============================================================================
//!
//! 1. PROGRAM ANALYSIS
//!
//!    Provides a program-level index of compilation units, analysis
//!    observations, domain participation and externally owned references.
//!
//! 2. DEPENDENCY ANALYSIS
//!
//!    Provides compilation dependency validation, dependency-first ordering,
//!    transitive dependency queries and dependency impact information.
//!
//! 3. EFFECT ANALYSIS
//!
//!    Provides function-level effect aggregation, interprocedural propagation,
//!    effect completeness preservation and declared-effect validation.
//!
//! 4. OWNERSHIP ANALYSIS
//!
//!    Provides CFG-aware ownership dataflow, move and borrow validation,
//!    ownership-flow analysis and structured ownership diagnostics.
//!
//! 5. CAPABILITY ANALYSIS
//!
//!    Evaluates canonical semantic capability requirements against explicitly
//!    scoped capability evidence.
//!
//! ============================================================================
//! SEMANTIC INVARIANTS
//! ============================================================================
//!
//! The following distinctions MUST be preserved throughout compiler analysis.
//!
//! REQUIREMENT:
//!     What the program needs.
//!
//! CAPABILITY:
//!     What an execution environment can provide.
//!
//! CONSTRAINT:
//!     What an implementation must obey.
//!
//! PREFERENCE:
//!     What is desirable but not mandatory.
//!
//! HINT:
//!     Optional implementation guidance.
//!
//! FEASIBILITY:
//!     Whether a particular realization satisfies the applicable conditions.
//!
//! These concepts MUST NOT be collapsed into one representation.
//!
//! An unknown analysis result MUST NOT be silently interpreted as success.
//!
//! An incomplete analysis MUST NOT be promoted to complete.
//!
//! Missing evidence MUST NOT be interpreted as proof of support.
//!
//! Empty effect information MUST NOT automatically be interpreted as purity.
//!
//! Unsupported ownership events MUST NOT be silently treated as unrestricted.
//!
//! ============================================================================
//! SCALABILITY AND POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module imposes no universal maximum on:
//!
//! - Compilation units.
//! - Dependency relationships.
//! - Functions.
//! - Effects.
//! - Ownership events.
//! - Basic blocks.
//! - Control-flow edges.
//! - Values.
//! - Borrow relationships.
//! - Capability requirements.
//! - Capability evidence.
//! - Computing domains.
//! - Hardware devices.
//! - Processing elements.
//! - Distributed participants.
//! - Memory capacity.
//! - Tensor dimensions or rank.
//!
//! No MAX_* constants or arbitrary collection limits are introduced.
//!
//! The child modules use dynamically sized collections and their own
//! documented analysis algorithms.
//!
//! Actual processing is bounded by available memory, address space,
//! processing capacity and caller-configured operational budgets.
//!
//! Literal infinite execution or storage capacity is not promised.
//!
//! Resource exhaustion MUST NOT be converted into a successful,
//! complete, or silently truncated analysis result.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Determinism is established by the individual analysis implementations.
//!
//! This module introduces:
//!
//! - No randomized identifiers.
//! - No global mutable analysis registries.
//! - No implicit hardware discovery.
//! - No hidden execution state.
//! - No order-dependent analysis decisions.
//!
//! The composition boundary preserves the public contracts of the
//! independently implemented analysis modules.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Parent:
//!
//!     src/compiler/mod.rs
//!
//! Required declaration:
//!
//!     pub mod analysis;
//!
//! The existing compiler root must also expose the already-existing
//! configuration and context modules required by dependency analysis:
//!
//!     pub mod configuration;
//!     pub mod context;
//!
//! Existing context facade:
//!
//!     src/compiler/context/mod.rs
//!
//! Dependency analysis consumes its canonical DependencyContext.
//!
//! The compiler analysis namespace does not create another dependency graph.
//!
//! ============================================================================
//! FUTURE EXTENSIBILITY
//! ============================================================================
//!
//! New analyses MAY be introduced as independently owned child modules.
//!
//! Examples of future compiler analysis modules include:
//!
//!     analysis/requirements.rs
//!     analysis/feasibility.rs
//!     analysis/cost.rs
//!     analysis/provenance.rs
//!     analysis/dataflow.rs
//!     analysis/concurrency.rs
//!     analysis/alias.rs
//!     analysis/termination.rs
//!     analysis/complexity.rs
//!
//! These names describe potential compiler-facing analysis integrations.
//!
//! They MUST NOT be declared here until their corresponding implementation
//! files and ownership contracts actually exist.
//!
//! New analyses must reuse canonical semantic, IR, resource and domain
//! authorities rather than introduce competing models.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! Existing child module paths remain unchanged.
//!
//! For example:
//!
//!     crate::compiler::analysis::program
//!     crate::compiler::analysis::dependency
//!     crate::compiler::analysis::effects
//!     crate::compiler::analysis::ownership
//!     crate::compiler::analysis::capabilities
//!
//! No existing public analysis type is renamed or relocated.
//!
//! No existing child implementation is copied into this file.
//!
//! ============================================================================
//! MODULE COMPOSITION
//! ============================================================================

/// Program-level compilation analysis.
///
/// Provides compilation-unit indexing, program analysis observations,
/// domain participation and consistency validation.
pub mod program;

/// Compilation-unit dependency analysis.
///
/// Uses the canonical compiler dependency context rather than introducing
/// another dependency graph.
pub mod dependency;

/// Function-level and interprocedural effect analysis.
///
/// Reuses the canonical ZUIR effect representation and effect policies.
pub mod effects;

/// CFG-aware ownership and borrow-flow analysis.
///
/// Reuses canonical ZUIR ownership contracts, CFG and value identities.
pub mod ownership;

/// Semantic capability compatibility analysis.
///
/// Delegates semantic compatibility evaluation to the canonical frontend.
pub mod capabilities;

// ============================================================================
// PUBLIC ANALYSIS API
// ============================================================================
//
// The module-qualified APIs are intentionally preserved.
//
// Consumers can select the required analysis explicitly:
//
//     crate::compiler::analysis::program
//     crate::compiler::analysis::dependency
//     crate::compiler::analysis::effects
//     crate::compiler::analysis::ownership
//     crate::compiler::analysis::capabilities
//
// Types remain owned by their respective child modules.
// This prevents accidental name collisions and preserves independent
// evolution of the analysis implementations.
