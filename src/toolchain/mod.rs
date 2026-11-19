
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Toolchain
//!
//! Path: `src/toolchain/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Unsafe Rust is prohibited.
//!
//! ## Mission
//!
//! Provide the unified, extensible developer-facing toolchain for the
//! Zamani programming language.
//!
//! POCO-REAF:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The toolchain coordinates project management, workspace management,
//! configuration, dependency resolution, build orchestration, packages,
//! registries, targets, external tools, commands, environments,
//! deployment, diagnostics, security and agent integration.
//!
//! It supports the complete Zamani computational ecosystem:
//!
//! - Classical and systems computing.
//! - Parallel and concurrent computing.
//! - Scientific and high-performance computing.
//! - Quantum computing and simulation.
//! - Hardware description and hardware realization.
//! - AI, machine learning and tensor computing.
//! - Hybrid classical/quantum/AI/hardware computation.
//! - Distributed computing.
//! - Networking and cryptography.
//! - Embedded computing and accelerators.
//! - Future registered computational domains.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Toolchain module composition.
//! - Public subsystem declarations.
//! - Root-level public API organization.
//! - Canonical toolchain error and result re-exports.
//! - The architectural boundaries between toolchain subsystems.
//!
//! It does not implement:
//!
//! - Source grammar, lexing or parsing.
//! - AST construction.
//! - Source-language semantic analysis.
//! - Classical, quantum, HDL, AI or other domain IRs.
//! - Compiler passes or optimization.
//! - Target-specific code generation.
//! - External process execution.
//! - External executable discovery.
//! - Physical hardware discovery or allocation.
//! - Runtime scheduling or execution.
//! - A second manifest parser.
//! - A second cryptographic implementation.
//! - A second artifact store.
//! - A second incremental compilation engine.
//!
//! These responsibilities remain with their canonical repository owners.
//!
//! ## Canonical repository authorities
//!
//! | Responsibility | Authority |
//! |---|---|
//! | Language grammar | `grammar/` |
//! | Lexing and parsing | `frontend/` |
//! | AST | `frontend/ast/` |
//! | Semantic analysis | `frontend/semantic/` |
//! | Classical computation | `classical/` |
//! | Quantum computation and IR | `quantum/` |
//! | HDL and hardware description | `hdl/` |
//! | AI computation | `ai/` |
//! | Hybrid computation | `hybrid/` |
//! | Distributed computation | `distributed/` |
//! | Universal IR | `zuir/` |
//! | Compilation | `compiler/` |
//! | Target-specific backends | `backend/` |
//! | External process execution | `backend/process.rs` |
//! | External tool discovery | `backend/toolchain.rs` |
//! | Physical hardware integration | `zhal/` |
//! | Runtime execution | `zur/` |
//! | Project manifest | `zamani_project_config/` |
//! | Toolchain orchestration | `toolchain/` |
//!
//! ## POCO-REAF contract
//!
//! One logical Zamani project may have multiple compatible target
//! realizations.
//!
//! The toolchain preserves:
//!
//! - Logical project identity.
//! - Source identity.
//! - Dependency identity.
//! - Build request identity.
//! - Build configuration.
//! - Target-selection intent.
//! - Requirements and constraints.
//! - Operational budgets.
//! - Compatibility evidence.
//! - Artifact identities.
//! - Provenance.
//! - Structured operation results.
//!
//! A target request is not proof of feasibility.
//!
//! A successful build-plan construction is not proof of compilation.
//!
//! A produced artifact is not automatically verified, portable,
//! executable or deployable.
//!
//! Each of these conditions must be established by its authoritative
//! subsystem.
//!
//! Literal infinite computation, memory, storage or physical resources
//! are not promised.
//!
//! ## Scalability contract
//!
//! This module introduces no arbitrary universal ceilings on:
//!
//! - Projects or workspaces.
//! - Packages or dependencies.
//! - Build tasks or graph nodes.
//! - Computing domains.
//! - Targets or providers.
//! - CPUs, processors or threads.
//! - GPUs, FPGAs or accelerators.
//! - Quantum resources.
//! - Memory or storage.
//! - Tensor dimensions.
//! - Distributed participants.
//! - Artifacts.
//!
//! Actual operations remain subject to:
//!
//! - Available physical resources.
//! - Rust and operating-system representation limits.
//! - Provider capabilities.
//! - Program semantics.
//! - Compatibility and feasibility.
//! - Explicit operational budgets.
//! - Security and authorization policies.
//!
//! Resource exhaustion must be reported honestly.
//!
//! Operational budgets must not become universal language or hardware
//! capacity restrictions.
//!
//! ## Extensibility contract
//!
//! The toolchain uses open-world provider and domain integration.
//!
//! New providers, targets, registries and computational domains must
//! integrate through their appropriate extension contracts.
//!
//! Adding an implementation must not require introducing another
//! compiler, package manager, runtime or hardware abstraction.
//!
//! This composition root must not require modification merely because
//! a third-party provider introduces a new compatible implementation.
//!
//! First-party subsystem additions must be explicitly declared here.
//!
//! ## Security contract
//!
//! - Unsafe Rust is prohibited.
//! - Public operations must validate their inputs.
//! - Authorization must precede privileged operations.
//! - Untrusted provider metadata must not be treated as trusted evidence.
//! - Credentials must not be exposed through diagnostics.
//! - Artifact integrity must be verified by the owning authority.
//! - Failed operations must never become successful results.
//! - Cancellation and resource exhaustion must remain distinguishable.
//! - Unknown capabilities must not establish feasibility.
//!
//! ## Concurrency contract
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No implicit worker pool.
//! - No process-wide service singleton.
//! - No hidden target selection.
//! - No independent scheduler.
//! - No duplicate resource allocator.
//!
//! Concurrency and execution policies remain with their owning services.
//!
//! ## Versioning contract
//!
//! `version.rs` is the canonical toolchain contract-version authority.
//!
//! `api/versioning.rs` owns API version negotiation.
//!
//! `api/compatibility.rs` owns cross-contract compatibility evaluation.
//!
//! This module introduces no competing language, grammar, compiler,
//! IR, ABI, manifest, package or provider version authority.
//!
//! ## Integration lifecycle
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Frontend and semantic validation
//!       |
//!       v
//! Canonical project manifest
//!       |
//!       v
//! Toolchain API
//!       |
//!       v
//! Workspace and configuration
//!       |
//!       v
//! Dependency resolution
//!       |
//!       v
//! Target and compatibility assessment
//!       |
//!       v
//! Build planning
//!       |
//!       v
//! Task orchestration
//!       |
//!       v
//! Compiler
//!       |
//!       v
//! Backend and artifact production
//!       |
//!       v
//! Artifact verification
//!       |
//!       v
//! Build reporting
//!       |
//!       v
//! Package / Run / Deploy
//!       |
//!       v
//! ZUR / ZHAL / authorized providers
//! ```
//!
//! This is the subsystem composition boundary. It does not claim that
//! every lifecycle operation is already implemented or production-ready.

// ============================================================================
// ROOT-LEVEL TOOLCHAIN CONTRACTS
// ============================================================================

/// Canonical structured toolchain errors and stable error classifications.
///
/// Owns toolchain error categories, codes, causal error preservation
/// and standard result aliases.
///
/// All toolchain subsystems must use this error authority.
pub mod errors;

/// Strongly typed identities owned by the toolchain.
///
/// Owns project, workspace, package, dependency, task, build,
/// registry, provider, environment and deployment identities.
///
/// Backend artifact identities, compiler identities, semantic symbols
/// and physical hardware identities remain with their own authorities.
pub mod ids;

/// Independently versioned toolchain contracts.
///
/// Owns toolchain API, manifest, package-format, registry-protocol,
/// provider and task-protocol version representations and compatibility
/// contracts.
///
/// Language and grammar versions remain independently owned.
pub mod version;

/// Canonical result-facing API.
///
/// Reuses the error definitions in `errors` and provides
/// contextual result composition.
///
/// Does not introduce a competing error taxonomy.
pub mod result;

// ============================================================================
// PUBLIC TOOLCHAIN API
// ============================================================================

/// Public API for toolchain clients.
///
/// Provides request, response, capability, compatibility and
/// API-version negotiation contracts.
///
/// Shared by CLI applications, IDEs, automation, enterprises,
/// external integrations and authorized AI agents.
pub mod api;

// ============================================================================
// CONFIGURATION AND PROJECT MANAGEMENT
// ============================================================================

/// Toolchain configuration, policies, profiles and operational budgets.
///
/// Integrates with canonical compiler configuration and resource-budget
/// infrastructure.
pub mod configuration;

/// Workspace discovery, membership, metadata and validation.
///
/// Consumes the canonical project manifest authority.
pub mod workspace;

/// Toolchain-facing manifest integration.
///
/// Adapts `crate::zamani_project_config`.
///
/// Does not introduce another manifest parser or schema authority.
pub mod manifest;

// ============================================================================
// DEPENDENCY AND PACKAGE MANAGEMENT
// ============================================================================

/// Domain-neutral package dependency management.
///
/// Owns dependency declarations, constraints, source integration,
/// feature resolution, compatibility, graphs, lockfiles and resolution.
pub mod dependencies;

/// Package creation, archive handling, extraction, verification,
/// installation and package metadata.
///
/// Cryptographic and registry authorities remain external integrations.
pub mod packages;

/// Registry discovery, provider integration, authentication contracts,
/// communication, downloading, publishing and federation.
///
/// Supports multiple independently configured registry providers.
pub mod registries;

// ============================================================================
// TARGET AND EXTERNAL TOOL INTEGRATION
// ============================================================================

/// Target profiles, target matrices, compatibility and target selection.
///
/// Consumes canonical compiler, backend and capability contracts.
///
/// Does not discover or allocate physical hardware independently.
pub mod targets;

/// External-tool requirements, resolution, authorization,
/// validation and readiness reporting.
///
/// Reuses `crate::backend::toolchain` for executable discovery
/// and version validation.
///
/// Reuses `crate::backend::process` for process execution.
pub mod external_tools;

/// Environment discovery, configuration, fingerprints
/// and compatibility validation.
///
/// Reuses canonical platform, compiler and backend authorities.
pub mod environments;

// ============================================================================
// TASK AND BUILD ORCHESTRATION
// ============================================================================

/// Domain-independent task definitions, registration,
/// dependencies, execution, cancellation and reporting.
///
/// Uses the canonical build dependency graph rather than
/// introducing a competing build graph.
pub mod tasks;

/// Project build requests, plans, dependency graphs,
/// compiler invocation, incremental-build coordination,
/// output verification and build reporting.
///
/// Coordinates the existing compiler and backend.
pub mod build;

// ============================================================================
// DEVELOPER COMMAND INFRASTRUCTURE
// ============================================================================

/// Unified developer command infrastructure.
///
/// Provides extensible command registration, dispatch,
/// authorization, auditing and structured command outcomes.
///
/// Commands must delegate to the owning toolchain services.
pub mod commands;

// ============================================================================
// DEPLOYMENT
// ============================================================================

/// Deployment requests, profiles, validation, planning,
/// providers and reporting.
///
/// Coordinates deployment without becoming a runtime,
/// scheduler, hardware allocator or deployment execution engine.
pub mod deployment;

// ============================================================================
// SECURITY AND DIAGNOSTICS
// ============================================================================

/// Toolchain security integration.
///
/// Coordinates security policies, trust, signatures,
/// integrity, provenance and validation.
///
/// Reuses canonical cryptographic and security authorities.
pub mod security;

/// Structured toolchain diagnostics, aggregation,
/// rendering and reporting.
///
/// Preserves diagnostic information originating from
/// the frontend, compiler, backend and other subsystems.
pub mod diagnostics;

// ============================================================================
// HUMAN AND AI AGENT INTEGRATION
// ============================================================================

/// Human, automation and AI-agent integration.
///
/// Provides shared context, requests, results,
/// authorization, audit and compliance contracts.
///
/// Agents must use the same underlying toolchain services
/// and security boundaries as human developers.
pub mod agents;

// ============================================================================
// CANONICAL PUBLIC ERROR AND RESULT EXPORTS
// ============================================================================

/// Canonical toolchain error type.
///
/// This is a re-export, not a second error implementation.
pub use errors::{ToolchainError, ToolchainErrorKind};

/// Canonical result aliases.
///
/// These aliases preserve the single error authority in `errors`.
pub use errors::{Result, ToolchainResult};

// ============================================================================
// COMPOSITION INVARIANTS
// ============================================================================

/// Stable architectural identity of the toolchain composition root.
///
/// This identifies the subsystem, not a language or package version.
pub const TOOLCHAIN_MODULE_ID: &str = "zamani.toolchain";

/// Returns the stable architectural identity of this subsystem.
#[must_use]
pub const fn module_id() -> &'static str {
    TOOLCHAIN_MODULE_ID
}

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolchain_module_identity_is_stable() {
        assert_eq!(module_id(), "zamani.toolchain");
    }

    #[test]
    fn toolchain_error_exports_are_canonical() {
        let error = ToolchainError::invalid_input("invalid test input");

        assert_eq!(error.kind(), ToolchainErrorKind::InvalidInput);
        assert!(!error.code().is_empty());
    }

    #[test]
    fn toolchain_result_alias_uses_canonical_error() {
        let result: ToolchainResult<()> =
            Err(ToolchainError::cancelled("test cancellation"));

        assert!(result.is_err());
    }

    #[test]
    fn toolchain_result_and_error_types_are_available() {
        fn accepts_result(_: Result<()>) {}
        fn accepts_toolchain_result(_: ToolchainResult<()>) {}
        fn accepts_error(_: ToolchainError) {}

        let _ = (
            accepts_result,
            accepts_toolchain_result,
            accepts_error,
        );
    }
}
