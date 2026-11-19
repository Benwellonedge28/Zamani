
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Toolchain
//!
//! Path: `src/toolchain/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Mission
//!
//! Provide the unified, extensible, developer-facing toolchain for Zamani.
//!
//! POCO-REAF:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`.
//!
//! The toolchain coordinates the development and distribution lifecycle:
//!
//! - Projects and workspaces.
//! - Configuration and operational policies.
//! - Manifest integration.
//! - Dependency resolution.
//! - Build planning and orchestration.
//! - Task orchestration.
//! - Target intent and compatibility.
//! - External toolchain integration.
//! - Package creation and installation.
//! - Registry integration.
//! - Developer commands.
//! - Environment validation.
//! - Deployment planning.
//! - Security and provenance.
//! - Diagnostics.
//! - Human and AI-agent integration.
//!
//! It supports the Zamani computational ecosystem, including classical,
//! quantum, HDL, AI, tensor, hybrid, distributed, networking, cryptographic,
//! embedded, accelerator and future computational domains.
//!
//! These domains share one toolchain architecture. Their implementations
//! remain owned by their respective repository subsystems.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Toolchain module composition.
//! - Public subsystem declarations.
//! - Root-level API organization.
//! - Canonical toolchain error and result exports.
//! - Toolchain architectural boundaries.
//! - Stable toolchain subsystem identity.
//!
//! It does NOT own:
//!
//! - Source grammar, token definitions or lexical analysis.
//! - Parsing or AST construction.
//! - Source-language semantic analysis.
//! - Semantic names, types, effects or capability definitions.
//! - Classical, quantum, HDL, AI or other domain IRs.
//! - Compiler passes or optimization.
//! - Target-specific code generation.
//! - External process execution.
//! - External executable discovery.
//! - Physical hardware discovery or allocation.
//! - Physical placement, routing or scheduling.
//! - Runtime execution.
//! - A second project-manifest parser.
//! - A second cryptographic implementation.
//! - A second artifact store.
//! - A second incremental compilation engine.
//!
//! ## Canonical repository authorities
//!
//! | Responsibility | Canonical authority |
//! |---|---|
//! | Source grammar | `grammar/` |
//! | Language semantics | `frontend/semantic/` |
//! | Source AST | `frontend/ast/` |
//! | Classical computation | `classical/` |
//! | Quantum computation and IR | `quantum/` |
//! | HDL and hardware description | `hdl/` |
//! | AI computation | `ai/` |
//! | Hybrid computation | `hybrid/` |
//! | Distributed computation | `distributed/` |
//! | Universal IR | `zuir/` |
//! | Compilation | `compiler/` |
//! | Target-specific code generation | `backend/` |
//! | External process execution | `backend/process.rs` |
//! | External tool discovery | `backend/toolchain.rs` |
//! | Backend security | `backend/security/` |
//! | Physical hardware integration | `zhal/` |
//! | Runtime and execution | `zur/` |
//! | Project manifest | `zamani_project_config/` |
//! | Toolchain orchestration | `toolchain/` |
//!
//! The toolchain consumes the public contracts of these authorities.
//! It must not create competing implementations or redefine their types.
//!
//! ## Grammar integration
//!
//! The toolchain respects the grammar authority hierarchy.
//!
//! Important existing specifications include:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/`
//! - `grammar/spec/`
//! - `grammar/modules/packages.g4`
//! - `grammar/compile/target.g4`
//! - `grammar/compile/target-selection.g4`
//! - `grammar/compile/deterministic-builds.g4`
//! - `grammar/resources/`
//! - `grammar/spec/resources.md`
//! - `grammar/execution/`
//! - `grammar/distributed/`
//!
//! Grammar files define source syntax.
//!
//! Semantic analysis defines the meaning of source constructs.
//!
//! The toolchain consumes validated project, semantic, compilation and
//! resource contracts. It must not parse grammar files to implement another
//! language frontend or reinterpret source-language semantics.
//!
//! ## POCO-REAF contract
//!
//! One logical Zamani project may have multiple compatible target
//! realizations.
//!
//! The toolchain preserves the identity and traceability of:
//!
//! - Projects and workspaces.
//! - Source inputs.
//! - Dependency resolutions.
//! - Build requests.
//! - Build configurations.
//! - Target-selection intent.
//! - Requirements.
//! - Capabilities.
//! - Preferences.
//! - Constraints.
//! - Hints.
//! - Operational budgets.
//! - Compatibility assessments.
//! - Build tasks.
//! - Produced artifacts.
//! - Verification evidence.
//! - Provenance.
//!
//! The following distinctions are mandatory:
//!
//! ```text
//! Target request
//!     !=
//! Target feasibility
//!
//! Build plan
//!     !=
//! Successful compilation
//!
//! Compilation success
//!     !=
//! Artifact verification
//!
//! Artifact verification
//!     !=
//! Execution readiness
//!
//! Execution readiness
//!     !=
//! Successful execution
//!
//! Deployment planning
//!     !=
//! Deployment completion
//! ```
//!
//! A logical build request may produce multiple target-specific artifacts.
//! Different realizations must preserve the applicable source semantics.
//!
//! The toolchain must not promise that every program can execute on every
//! physical machine or that every computational domain is universally
//! available.
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
//! - CPUs, processors, cores or threads.
//! - GPUs, FPGAs, QPUs or accelerators.
//! - Physical or logical qubits.
//! - Memory or storage.
//! - Tensor dimensions or rank.
//! - Distributed participants.
//! - Artifacts.
//! - Registries.
//! - Package sources.
//!
//! No fixed universal hardware capacity is established by the toolchain.
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
//! - Provider and implementation limitations.
//!
//! Operational budgets are permitted and necessary.
//!
//! Examples include:
//!
//! - Build-time budgets.
//! - Memory budgets.
//! - Storage budgets.
//! - Network-transfer budgets.
//! - Download-size budgets.
//! - Archive-extraction budgets.
//! - Configurable task-concurrency budgets.
//! - Retry budgets.
//! - Log-retention budgets.
//!
//! These budgets must be configurable and must not become universal
//! language-level or hardware-capacity restrictions.
//!
//! Resource exhaustion must be reported explicitly.
//!
//! ## Open-world extensibility
//!
//! The toolchain must support extensible providers and computational domains.
//!
//! It must not require a closed enumeration of:
//!
//! - Computing domains.
//! - Processor architectures.
//! - Hardware vendors.
//! - Accelerator models.
//! - Quantum providers.
//! - HDL synthesis providers.
//! - Registry implementations.
//! - Deployment environments.
//! - Operating systems.
//! - Future computational technologies.
//!
//! New compatible providers integrate through their canonical extension
//! contracts.
//!
//! A new provider must not require a second compiler, package manager,
//! process runner, runtime or hardware abstraction.
//!
//! First-party Rust modules are explicitly composed below. Third-party
//! implementations must use the existing registration and provider APIs.
//!
//! ## Feasibility and evidence
//!
//! The toolchain must distinguish:
//!
//! - Passed.
//! - Blocked.
//! - Unresolved.
//! - Unsupported.
//! - Not applicable.
//!
//! Missing evidence must never be interpreted as success.
//!
//! An unknown capability must not be interpreted as an available capability.
//!
//! An unverified artifact must not be presented as a verified artifact.
//!
//! A requested target must not be presented as a feasible target without
//! evidence from the appropriate authority.
//!
//! ## Security contract
//!
//! - Unsafe Rust is prohibited.
//! - External inputs must be validated at their owning boundaries.
//! - Authorization must precede privileged operations.
//! - Provider metadata must not automatically become trusted evidence.
//! - Credentials must not appear in diagnostics or logs.
//! - Artifact integrity must be verified by the appropriate authority.
//! - Failed operations must never become successful results.
//! - Cancellation must remain distinguishable from failure.
//! - Resource exhaustion must remain distinguishable from unsupported
//!   operations.
//! - Untrusted archives and registry responses must be validated.
//! - Human and AI clients must use the same authorization boundaries.
//!
//! This composition root does not implement the security mechanisms.
//!
//! ## Concurrency contract
//!
//! This composition root introduces:
//!
//! - No global mutable state.
//! - No implicit worker pool.
//! - No process-wide service singleton.
//! - No hidden target selection.
//! - No duplicate scheduler.
//! - No duplicate resource allocator.
//! - No hidden concurrency ceiling.
//!
//! Task concurrency, cancellation and resource budgeting belong to their
//! respective services.
//!
//! ## Versioning contract
//!
//! `version.rs` owns toolchain contract-version representations.
//!
//! `api/versioning.rs` owns API version negotiation.
//!
//! `api/compatibility.rs` owns compatibility evaluation.
//!
//! These version authorities remain distinct from:
//!
//! - Zamani language version.
//! - Grammar version.
//! - Compiler version.
//! - IR schema version.
//! - Backend API version.
//! - ABI version.
//! - Manifest version.
//! - Package archive version.
//! - Registry protocol version.
//! - Provider version.
//! - Hardware firmware version.
//!
//! This module must not introduce a competing version authority.
//!
//! ## Integration lifecycle
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Canonical frontend and semantic validation
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
//! Build-plan construction
//!       |
//!       v
//! Task orchestration
//!       |
//!       v
//! Canonical compiler API
//!       |
//!       v
//! Backend and artifact production
//!       |
//!       v
//! Artifact verification
//!       |
//!       v
//! Structured build reporting
//!       |
//!       +------------------+------------------+
//!       |                  |                  |
//!       v                  v                  v
//!    Package             Run               Deploy
//!       |                  |                  |
//!       v                  v                  v
//!  Registry/Store         ZUR          Deployment providers
//!                          |
//!                          v
//!                         ZHAL
//! ```
//!
//! This describes the intended subsystem integration.
//! Module registration alone does not certify that every lifecycle stage
//! is implemented or production-ready.
//!
//! ## Composition requirements
//!
//! Every declared Rust module below corresponds to an existing component
//! in the inspected repository tree.
//!
//! Do not declare speculative modules.
//!
//! Do not remove existing modules without first migrating their consumers.
//!
//! Do not create duplicate subsystem authorities.
//!
//! The crate root must expose this module under the `buildsystem` feature.
//!
//! The Cargo feature must not be used to bypass safety, validation,
//! compatibility or authorization requirements.

// ============================================================================
// ROOT-LEVEL TOOLCHAIN CONTRACTS
// ============================================================================

/// Canonical structured toolchain errors and result aliases.
///
/// Owns toolchain error classifications, stable error codes, contextual
/// information and causal error preservation.
///
/// All toolchain subsystems must use this error authority.
pub mod errors;

/// Strongly typed identities owned by the toolchain.
///
/// Owns project, workspace, package, dependency, task, build, registry,
/// provider, environment and deployment identities.
///
/// Compiler artifact identities, semantic symbols and physical hardware
/// identities remain with their respective authorities.
pub mod ids;

/// Independently versioned toolchain contracts.
///
/// Owns toolchain contract-version representations and compatibility
/// information.
///
/// API negotiation is implemented by `api::versioning`.
pub mod version;

/// Canonical toolchain result-facing API.
///
/// Reuses the error definitions in `errors`.
///
/// Does not introduce another error taxonomy.
pub mod result;

// ============================================================================
// PUBLIC TOOLCHAIN API
// ============================================================================

/// Public toolchain API.
///
/// Defines requests, responses, capabilities, compatibility and API
/// version negotiation.
///
/// Shared by CLI applications, IDEs, automation, enterprises and
/// authorized AI agents.
pub mod api;

// ============================================================================
// CONFIGURATION AND PROJECT MANAGEMENT
// ============================================================================

/// Toolchain configuration, policies, profiles and operational budgets.
///
/// Integrates with canonical compiler configuration and resource policies.
pub mod configuration;

/// Workspace discovery, layout, membership, metadata and validation.
///
/// Consumes the canonical project-manifest authority.
pub mod workspace;

/// Toolchain-facing project-manifest integration.
///
/// Adapts `crate::zamani_project_config`.
///
/// Does not introduce another manifest parser or manifest schema authority.
pub mod manifest;

// ============================================================================
// DEPENDENCY AND PACKAGE MANAGEMENT
// ============================================================================

/// Domain-neutral dependency management.
///
/// Owns dependency models, constraints, sources, feature resolution,
/// compatibility, graphs, lockfiles and resolution.
pub mod dependencies;

/// Package creation, archive handling, extraction, verification,
/// installation and package metadata.
///
/// Reuses canonical cryptographic, artifact and registry authorities.
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
/// Does not independently discover or allocate physical hardware.
pub mod targets;

/// External-tool requirements, resolution, validation and readiness.
///
/// Reuses `crate::backend::toolchain` for external executable discovery
/// and version validation.
///
/// Reuses `crate::backend::process` for process execution.
pub mod external_tools;

/// Environment discovery, configuration, fingerprints and validation.
///
/// Reuses canonical compiler, backend and platform authorities.
pub mod environments;

// ============================================================================
// TASK AND BUILD ORCHESTRATION
// ============================================================================

/// Domain-independent task definitions, registration, dependencies,
/// execution contracts, cancellation and reporting.
///
/// Does not replace the compiler's internal compilation graph or runtime
/// scheduling infrastructure.
pub mod tasks;

/// Project build requests, plans, dependency graphs, compiler invocation,
/// incremental-build coordination, artifact verification and reporting.
///
/// Coordinates the existing compiler and backend.
pub mod build;

// ============================================================================
// DEVELOPER COMMAND INFRASTRUCTURE
// ============================================================================

/// Unified developer command infrastructure.
///
/// Provides command registration, dispatch, authorization, auditing
/// and structured operation outcomes.
///
/// Commands must delegate to the owning toolchain services.
pub mod commands;

// ============================================================================
// DEPLOYMENT
// ============================================================================

/// Deployment requests, profiles, validation, planning, providers
/// and reporting.
///
/// Coordinates deployment without becoming a runtime, scheduler,
/// physical hardware allocator or deployment execution engine.
pub mod deployment;

// ============================================================================
// SECURITY AND DIAGNOSTICS
// ============================================================================

/// Toolchain security integration.
///
/// Coordinates policy, trust, integrity, provenance and validation.
///
/// Reuses canonical cryptographic and security authorities.
pub mod security;

/// Structured toolchain diagnostics and aggregation.
///
/// Preserves diagnostic information originating from the frontend,
/// compiler, backend and other subsystems.
///
/// Presentation and serialization must use the actual diagnostic
/// and command interfaces implemented by their owning modules.
pub mod diagnostics;

// ============================================================================
// HUMAN AND AI AGENT INTEGRATION
// ============================================================================

/// Human, automation and AI-agent integration.
///
/// Provides shared context, requests, results, permissions, auditing
/// and compliance contracts.
///
/// Agents use the same underlying services and security boundaries
/// as human developers.
pub mod agents;

// ============================================================================
// CANONICAL PUBLIC ERROR AND RESULT EXPORTS
// ============================================================================

/// Canonical toolchain error type.
///
/// This is a re-export, not another error implementation.
pub use errors::{ToolchainError, ToolchainErrorKind};

/// Canonical result aliases.
///
/// Both aliases preserve the single error authority in `errors`.
pub use errors::{Result, ToolchainResult};

/// Error-context extension trait.
///
/// Re-exported from the canonical result module.
pub use result::ToolchainErrorExt;

// ============================================================================
// COMPOSITION IDENTITY
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
    use super::{
        module_id, Result, ToolchainError, ToolchainErrorExt,
        ToolchainErrorKind, ToolchainResult, TOOLCHAIN_MODULE_ID,
    };

    #[test]
    fn toolchain_module_identity_is_stable() {
        assert_eq!(module_id(), TOOLCHAIN_MODULE_ID);
        assert_eq!(module_id(), "zamani.toolchain");
    }

    #[test]
    fn canonical_error_exports_are_available() {
        let error = ToolchainError::invalid_input("invalid test input");

        assert_eq!(error.kind(), ToolchainErrorKind::InvalidInput);
        assert!(!error.code().is_empty());
    }

    #[test]
    fn canonical_result_alias_preserves_errors() {
        let result: ToolchainResult<()> =
            Err(ToolchainError::cancelled("test cancellation"));

        assert!(result.is_err());
    }

    #[test]
    fn result_alias_is_the_canonical_result_type() {
        let result: Result<()> = Err(
            ToolchainError::invalid_input("test input"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn canonical_error_context_extension_is_available() {
        let error = ToolchainError::invalid_input("invalid input")
            .with_context("toolchain composition test");

        assert_eq!(error.kind(), ToolchainErrorKind::InvalidInput);
        assert!(!error.context().is_empty());
    }

    #[test]
    fn canonical_error_type_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ToolchainError>();
    }
}
