#![forbid(unsafe_code)]

//! # Zamani Toolchain — Dependency Management
//!
//! Path: `src/toolchain/dependencies/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root and public module boundary for
//! Zamani toolchain dependency management.
//!
//! It exposes the existing dependency authorities through one coherent
//! subsystem without duplicating their implementations.
//!
//! Dependency management supports projects across classical, quantum,
//! HDL, AI, tensor, hybrid, distributed, networking, cryptographic,
//! embedded, accelerator and future computational domains.
//!
//! The dependency subsystem is domain-neutral. It manages package
//! relationships and package requirements; it does not implement
//! computational-domain semantics.
//!
//! ## Architectural ownership
//!
//! Each child module has one authoritative responsibility.
//!
//! - `model` owns canonical dependency identities, declarations,
//!   source declarations, package coordinates and dependency metadata.
//!
//! - `constraints` owns dependency constraint representations,
//!   candidate evidence, version evaluator registration and
//!   constraint evaluation.
//!
//! - `features` owns package-feature activation assessment and
//!   feature-dependent dependency expansion contracts.
//!
//! - `sources` owns dependency source discovery and source-specific
//!   candidate-provider implementations.
//!
//! - `compatibility` owns dependency compatibility requirements,
//!   compatibility evidence and compatibility evaluation.
//!
//! - `graph` owns package dependency topology, reachability,
//!   cycle analysis and dependency-first ordering.
//!
//! - `resolver` owns candidate discovery coordination, dependency
//!   solving, candidate selection, feature unification and
//!   resolution graph construction.
//!
//! - `lockfile` owns deterministic dependency-resolution snapshots,
//!   lockfile serialization, structural verification and persistence.
//!
//! - `validation` owns post-resolution validation before a resolved
//!   dependency graph is accepted by build or installation workflows.
//!
//! ## Ownership boundaries
//!
//! This module does not own:
//!
//! - Source-language grammar or parsing.
//! - Frontend AST construction.
//! - Semantic analysis.
//! - Canonical project manifest parsing.
//! - Package archive creation or extraction.
//! - Package-content integrity verification.
//! - Cryptographic signatures or trust decisions.
//! - Registry protocols or authentication.
//! - Compiler dependency graphs.
//! - Build-task graphs.
//! - Target selection or hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling or execution.
//!
//! Those responsibilities remain with their existing repository
//! authorities.
//!
//! ## Dependency direction
//!
//! The intended conceptual flow is:
//!
//! ```text
//! Canonical project manifest / frontend declarations
//!                     |
//!                     v
//!                  model
//!                     |
//!                     v
//!                constraints
//!                     |
//!          +----------+----------+
//!          |                     |
//!          v                     v
//!       sources              features
//!          |                     |
//!          +----------+----------+
//!                     |
//!                     v
//!                  resolver
//!                     |
//!          +----------+----------+
//!          |                     |
//!          v                     v
//!        graph             compatibility
//!          |                     |
//!          +----------+----------+
//!                     |
//!                     v
//!                 validation
//!                     |
//!                     v
//!                  lockfile
//!                     |
//!                     v
//!          Toolchain build / package services
//! ```
//!
//! This diagram describes the conceptual workflow, not a requirement
//! that Rust modules depend on one another in this exact sequence.
//! The resolver may use graph, feature and constraint APIs during
//! resolution, while validation independently checks the completed
//! result.
//!
//! ## Determinism
//!
//! The subsystem preserves the deterministic ordering and identity
//! guarantees established by its authoritative child modules.
//!
//! This composition root introduces no global mutable state,
//! implicit provider registration, hidden initialization or
//! nondeterministic module selection.
//!
//! ## Scalability
//!
//! This module introduces no fixed limits on:
//!
//! - Dependency count.
//! - Package count.
//! - Dependency graph size.
//! - Dependency-tree depth.
//! - Feature count.
//! - Version-string length.
//! - Source count.
//! - Provider count.
//! - Computational domains.
//! - Hardware types.
//! - Distributed participants.
//!
//! Actual processing and storage requirements depend on supplied
//! dependency data, available resources and explicit operational
//! budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## POCO-REAF
//!
//! Dependency identity and requirements remain independent of the
//! machine on which a project is eventually compiled or executed.
//!
//! The dependency subsystem must not encode assumptions about a
//! particular CPU, GPU, FPGA, QPU, operating system, cloud provider
//! or runtime.
//!
//! Target-specific compatibility is established through the
//! appropriate registered compatibility and provider authorities.
//!
//! ## Integration
//!
//! The toolchain composition root must declare this module through:
//!
//! `pub mod dependencies;`
//!
//! The crate root must expose the toolchain under the existing
//! `buildsystem` feature.
//!
//! The manifest integration boundary must expose its existing
//! `adapter` module because the local dependency source provider
//! consumes `ManifestAdapter`.
//!
//! ## Public API policy
//!
//! Child modules are exposed by their explicit module paths.
//!
//! This file intentionally does not wildcard-re-export their contents.
//! Consumers should import the specific authoritative types they need,
//! for example:
//!
//! - `dependencies::model::DependencyDeclaration`
//! - `dependencies::constraints::DependencyConstraintSet`
//! - `dependencies::resolver::DependencyResolver`
//! - `dependencies::graph::PackageDependencyGraph`
//! - `dependencies::lockfile::Lockfile`
//! - `dependencies::validation::DependencyValidator`
//!
//! This avoids ambiguous names, accidental API expansion and
//! competing public type aliases.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden in this module and its descendants.

// ============================================================================
// CANONICAL DEPENDENCY MODEL
// ============================================================================

/// Canonical dependency identities, declarations, sources and metadata.
///
/// All dependency consumers must use these shared representations rather
/// than creating competing package or dependency identity types.
pub mod model;

// ============================================================================
// CONSTRAINTS AND EVIDENCE
// ============================================================================

/// Dependency requirements, preferences, hints, candidate evidence
/// and extensible constraint evaluators.
pub mod constraints;

// ============================================================================
// PACKAGE FEATURE MANAGEMENT
// ============================================================================

/// Package-feature activation assessment and feature-dependent
/// dependency expansion contracts.
///
/// Package features are distinct from Zamani language/compiler feature gates.
pub mod features;

// ============================================================================
// PACKAGE DEPENDENCY GRAPH
// ============================================================================

/// Package dependency topology, reachability, cycle analysis
/// and dependency-first ordering.
///
/// This graph is distinct from compiler and build-task graphs.
pub mod graph;

// ============================================================================
// DEPENDENCY RESOLUTION
// ============================================================================

/// Candidate discovery coordination, deterministic dependency solving,
/// candidate selection, feature unification and resolution graphs.
pub mod resolver;

// ============================================================================
// DEPENDENCY SOURCE DISCOVERY
// ============================================================================

/// Local-path source discovery and the integration boundary for
/// registry, Git, URL and future candidate providers.
pub mod sources;

// ============================================================================
// DEPENDENCY COMPATIBILITY
// ============================================================================

/// Evidence-based compatibility requirements and evaluation.
///
/// Compatibility dimensions remain extensible rather than being
/// restricted to a closed list of computational domains or vendors.
pub mod compatibility;

// ============================================================================
// DEPENDENCY LOCKFILES
// ============================================================================

/// Deterministic resolution snapshots, lockfile serialization,
/// verification and persistence.
///
/// Lockfiles record resolver decisions; they do not independently
/// resolve or download packages.
pub mod lockfile;

// ============================================================================
// POST-RESOLUTION VALIDATION
// ============================================================================

/// Validation of completed dependency resolutions before build
/// or installation orchestration accepts them.
pub mod validation;