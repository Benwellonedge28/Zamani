
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Workspace Subsystem
//!
//! Path: `src/toolchain/workspace/mod.rs`
//!
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Mission
//!
//! Provide the unified workspace-management boundary for Zamani projects.
//!
//! This subsystem supports:
//!
//! - Single-package projects.
//! - Multi-package workspaces.
//! - Nested project directories.
//! - Multiple source roots.
//! - Custom source layouts.
//! - Generated and provider-supplied inputs.
//! - Classical, quantum, HDL, AI and hybrid projects.
//! - Distributed and heterogeneous projects.
//! - Future registered computational domains.
//!
//! Its architecture supports:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Workspace subsystem composition.
//! - Public workspace API organization.
//! - Public exports of workspace contracts.
//! - Integration boundaries between workspace components.
//! - Workspace API documentation and conformance tests.
//!
//! This module does NOT own:
//!
//! - Manifest parsing or canonical manifest schemas.
//! - Filesystem discovery implementation.
//! - Project source-layout inspection.
//! - Workspace-member resolution implementation.
//! - Persistent project identity allocation.
//! - Dependency resolution.
//! - Compilation.
//! - Intermediate representations.
//! - Target selection.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Program execution.
//!
//! Those responsibilities belong to their existing authoritative
//! repository subsystems.
//!
//! ## Canonical repository authorities
//!
//! Project manifest:
//! `crate::zamani_project_config`
//!
//! Toolchain identifiers:
//! `crate::toolchain::ids`
//!
//! Toolchain errors:
//! `crate::toolchain::errors`
//!
//! Toolchain results:
//! `crate::toolchain::result`
//!
//! Language frontend:
//! `crate::frontend`
//!
//! Compiler:
//! `crate::compiler`
//!
//! Backend:
//! `crate::backend`
//!
//! Hardware abstraction:
//! `crate::zhal`
//!
//! Runtime:
//! `crate::zur`
//!
//! Distributed computing:
//! `crate::distributed`
//!
//! ## Workspace architecture
//!
//! ```text
//!                Caller-supplied project path
//!                           |
//!                           v
//!                  workspace::discovery
//!                           |
//!                           v
//!                 Discovered workspace
//!                           |
//!              +------------+------------+
//!              |                         |
//!              v                         v
//!       workspace::members       workspace::layout
//!              |                         |
//!              +------------+------------+
//!                           |
//!                           v
//!                 workspace::validation
//!                           |
//!                           v
//!                 workspace::metadata
//!                           |
//!                           v
//!                 Validated workspace
//!                           |
//!              +------------+------------+
//!              |            |            |
//!              v            v            v
//!          manifest    dependencies    build
//!              |            |            |
//!              +------------+------------+
//!                           |
//!                           v
//!                     compiler API
//!                           |
//!                           v
//!                Backend / artifact systems
//!                           |
//!                           v
//!                     Runtime handoff
//! ```
//!
//! The diagram describes subsystem integration, not an assertion
//! that every downstream service is already production-ready.
//!
//! ## Module responsibilities
//!
//! `discovery.rs`
//! - Discovers package and workspace roots.
//! - Uses explicitly supplied paths.
//! - Performs ancestor-based discovery.
//! - Does not recursively enumerate workspace members.
//!
//! `layout.rs`
//! - Interprets physical source layouts.
//! - Supports configurable source roots and extensions.
//! - Supports multiple entry points.
//! - Provides iterative source-file traversal.
//!
//! `members.rs`
//! - Resolves explicit workspace member declarations.
//! - Verifies member paths and package manifests.
//! - Uses canonical ProjectId and WorkspaceId identities.
//! - Enforces workspace-root containment.
//!
//! `metadata.rs`
//! - Collects workspace and project metadata.
//! - Computes content fingerprints.
//! - Records source inventory completeness.
//! - Supports reproducibility and incremental-build integration.
//!
//! `validation.rs`
//! - Cross-validates discovery, membership and layouts.
//! - Reports structured validation issues.
//! - Distinguishes structural errors from filesystem failures.
//!
//! ## Manifest ownership
//!
//! `crate::zamani_project_config` remains the sole authority for
//! Zamani project-manifest schemas and interpretation.
//!
//! This module does not introduce another TOML parser or
//! independently interpret package, dependency, feature,
//! target, build or deployment semantics.
//!
//! Workspace member declarations are supplied to the member
//! resolver through the manifest integration boundary.
//!
//! ## Identity ownership
//!
//! `ProjectId` and `WorkspaceId` are defined in:
//!
//! `crate::toolchain::ids`
//!
//! This module does not generate competing identifiers.
//!
//! Persistent or distributed identity uniqueness must be
//! provided by an appropriate identity authority.
//!
//! ## Error ownership
//!
//! All workspace operations use the canonical toolchain
//! error and result contracts.
//!
//! Filesystem errors, invalid input, validation failures,
//! resource exhaustion and other failures must retain
//! their appropriate classifications.
//!
//! ## POCO-REAF and domain independence
//!
//! A workspace represents project organization, not a
//! physical computing target.
//!
//! Workspace management must not assume that a project
//! is exclusively classical, quantum, HDL, AI or distributed.
//!
//! A single workspace may contain multiple computational
//! domains and their explicitly declared relationships.
//!
//! Domain-specific compilation remains with the appropriate
//! frontend, semantic, IR, compiler and backend authorities.
//!
//! ## Scalability
//!
//! This composition root introduces no artificial ceilings on:
//!
//! - Workspace count.
//! - Project count.
//! - Member count.
//! - Source-file count.
//! - Source-directory count.
//! - Directory nesting depth.
//! - Computational-domain count.
//! - Target count.
//! - Provider count.
//! - Hardware-resource count.
//! - Distributed participant count.
//!
//! This does not promise literal infinite memory, storage,
//! computation or execution.
//!
//! Actual operations remain subject to:
//!
//! - Available memory.
//! - Available storage.
//! - Filesystem capabilities.
//! - Operating-system limits.
//! - Configured operational budgets.
//! - Implementation capabilities.
//! - Cancellation and security policies.
//!
//! Resource exhaustion must be reported explicitly.
//!
//! ## Determinism
//!
//! Workspace components must preserve their documented
//! deterministic ordering and fingerprint contracts.
//!
//! This composition root must not introduce nondeterministic
//! iteration, identity allocation or metadata generation.
//!
//! ## Security
//!
//! - Unsafe Rust is prohibited.
//! - No shell execution is performed.
//! - No external processes are launched.
//! - No source code or manifest content is executed.
//! - Filesystem operations remain within the responsibilities
//!   of their owning workspace components.
//! - Untrusted project data must be validated before use.
//! - Workspace discovery must not imply project trust.
//!
//! ## Concurrency
//!
//! This module does not create global mutable workspace state.
//!
//! Workspace snapshots are owned by their callers.
//!
//! Concurrent operations must use independently owned state
//! and the synchronization contracts of their respective
//! implementation components.
//!
//! ## Compatibility
//!
//! Public workspace types and functions are re-exported
//! without introducing replacement representations.
//!
//! New workspace capabilities should be added to their
//! owning implementation files and exposed here deliberately.
//!
//! Breaking public API changes require an explicit
//! compatibility and migration decision.
//!
//! ## Completion contract
//!
//! This file is complete when:
//!
//! - Every existing workspace implementation is registered.
//! - Every intended public workspace API is exported.
//! - No implementation is duplicated.
//! - No competing identity or error types are introduced.
//! - Public documentation describes the ownership boundaries.
//! - API conformance tests pass.
//! - The parent toolchain module exposes this subsystem.
//! - The library feature configuration permits its intended use.

// ============================================================================
// WORKSPACE IMPLEMENTATION MODULES
// ============================================================================

/// Filesystem-based project and workspace discovery.
///
/// This module owns discovery from explicitly supplied paths.
/// It does not parse the complete canonical project manifest.
pub mod discovery;

/// Physical project and source-file layout inspection.
///
/// This module owns source-directory recognition,
/// entry-point recognition and iterative source traversal.
pub mod layout;

/// Workspace-member declaration validation and resolution.
///
/// This module owns the relationship between a workspace
/// and its explicitly declared package projects.
pub mod members;

/// Workspace and project metadata collection.
///
/// This module owns workspace content fingerprints,
/// project metadata and source-inventory information.
pub mod metadata;

/// Cross-component workspace structural validation.
///
/// This module validates discovery, resolved members
/// and project layouts as a consistent workspace snapshot.
pub mod validation;

// ============================================================================
// DISCOVERY PUBLIC API
// ============================================================================

/// Public discovery functions, configuration, result types
/// and conventional manifest filename.
///
/// The discovery implementation remains in `discovery.rs`.
pub use discovery::{
    discover,
    discover_from_manifest,
    discover_from_manifest_with_options,
    discover_with_options,
    DiscoveredManifest,
    DiscoveryOptions,
    DiscoveryResult,
    ManifestKind,
    DEFAULT_MANIFEST_FILE_NAME,
};

// ============================================================================
// PROJECT LAYOUT PUBLIC API
// ============================================================================

/// Public source-layout configuration, descriptions,
/// classification, inspection functions and conventions.
///
/// The layout implementation remains in `layout.rs`.
pub use layout::{
    inspect,
    inspect_discovered,
    inspect_discovered_with_options,
    inspect_with_options,
    LayoutKind,
    ProjectLayout,
    SourceLayoutOptions,
    SourceRoot,
    DEFAULT_EXECUTABLE_ENTRY_POINT,
    DEFAULT_LIBRARY_ENTRY_POINT,
    DEFAULT_SOURCE_DIRECTORY,
    DEFAULT_SOURCE_EXTENSION,
};

// ============================================================================
// WORKSPACE MEMBER PUBLIC API
// ============================================================================

/// Public workspace-member declarations, resolution options,
/// resolved-member collections and manifest convention.
///
/// Member identity allocation remains with the caller's
/// canonical identity authority.
///
/// The member implementation remains in `members.rs`.
pub use members::{
    MemberResolutionOptions,
    WorkspaceMember,
    WorkspaceMemberDeclaration,
    WorkspaceMembers,
    DEFAULT_MEMBER_MANIFEST_FILE_NAME,
};

// ============================================================================
// WORKSPACE METADATA PUBLIC API
// ============================================================================

/// Public workspace metadata, project metadata,
/// metadata file descriptions and fingerprint contracts.
///
/// The metadata implementation remains in `metadata.rs`.
pub use metadata::{
    MetadataFile,
    MetadataFileRole,
    WorkspaceMetadata,
    WorkspaceProjectMetadata,
    FINGERPRINT_ALGORITHM,
    FINGERPRINT_FORMAT_VERSION,
};

// ============================================================================
// WORKSPACE VALIDATION PUBLIC API
// ============================================================================

/// Public validation configuration, issue types,
/// validation reports and validation operations.
///
/// The validation implementation remains in `validation.rs`.
pub use validation::{
    validate_workspace,
    validate_workspace_with_options,
    ValidationSeverity,
    WorkspaceValidationCode,
    WorkspaceValidationIssue,
    WorkspaceValidationOptions,
    WorkspaceValidationReport,
};

// ============================================================================
// WORKSPACE API CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that the discovery API is available through
    /// the workspace composition root.
    #[test]
    fn discovery_api_is_publicly_composed() {
        let options = DiscoveryOptions::default();

        assert_eq!(
            options.manifest_file_name(),
            DEFAULT_MANIFEST_FILE_NAME
        );

        assert!(ManifestKind::Package.has_package());
        assert!(!ManifestKind::Package.has_workspace());

        assert!(ManifestKind::Workspace.has_workspace());
        assert!(!ManifestKind::Workspace.has_package());

        assert!(
            ManifestKind::PackageAndWorkspace.has_package()
        );

        assert!(
            ManifestKind::PackageAndWorkspace.has_workspace()
        );

        // Compile-time API availability check.
        let _: Option<DiscoveredManifest> = None;
        let _: Option<DiscoveryResult> = None;

        let _ = discover;
        let _ = discover_with_options;
        let _ = discover_from_manifest;
        let _ = discover_from_manifest_with_options;
    }

    /// Verify that the layout API is available through
    /// the workspace composition root.
    #[test]
    fn layout_api_is_publicly_composed() {
        let options = SourceLayoutOptions::default();

        assert_eq!(
            options.source_extension(),
            std::ffi::OsStr::new(DEFAULT_SOURCE_EXTENSION)
        );

        assert_eq!(
            DEFAULT_SOURCE_DIRECTORY,
            "src"
        );

        assert_eq!(
            DEFAULT_EXECUTABLE_ENTRY_POINT,
            "main.zm"
        );

        assert_eq!(
            DEFAULT_LIBRARY_ENTRY_POINT,
            "lib.zm"
        );

        let _: Option<ProjectLayout> = None;
        let _: Option<SourceRoot> = None;

        assert!(matches!(
            LayoutKind::Empty,
            LayoutKind::Empty
        ));

        let _ = inspect;
        let _ = inspect_with_options;
        let _ = inspect_discovered;
        let _ = inspect_discovered_with_options;
    }

    /// Verify that workspace-member contracts are publicly
    /// accessible without creating another identity system.
    #[test]
    fn member_api_is_publicly_composed() {
        let options = MemberResolutionOptions::default();

        assert_eq!(
            options.manifest_file_name(),
            DEFAULT_MEMBER_MANIFEST_FILE_NAME
        );

        let _: Option<WorkspaceMemberDeclaration> = None;
        let _: Option<WorkspaceMember> = None;
        let _: Option<WorkspaceMembers> = None;
    }

    /// Verify that workspace metadata contracts are publicly
    /// accessible through the composition root.
    #[test]
    fn metadata_api_is_publicly_composed() {
        assert_eq!(
            FINGERPRINT_ALGORITHM,
            "sha256"
        );

        assert_ne!(
            FINGERPRINT_FORMAT_VERSION,
            0
        );

        let _: Option<MetadataFile> = None;
        let _: Option<MetadataFileRole> = None;
        let _: Option<WorkspaceProjectMetadata> = None;
        let _: Option<WorkspaceMetadata> = None;
    }

    /// Verify that validation contracts are publicly
    /// accessible through the composition root.
    #[test]
    fn validation_api_is_publicly_composed() {
        let options = WorkspaceValidationOptions::new();

        assert!(!options.require_members());
        assert!(!options.require_source_files());

        assert!(
            ValidationSeverity::Error.prevents_success()
        );

        assert!(
            !ValidationSeverity::Warning.prevents_success()
        );

        let _: Option<WorkspaceValidationCode> = None;
        let _: Option<WorkspaceValidationIssue> = None;
        let _: Option<WorkspaceValidationReport> = None;

        let _ = validate_workspace;
        let _ = validate_workspace_with_options;
    }

    /// Verify that the workspace composition root introduces
    /// no replacement public identity or error contract.
    #[test]
    fn canonical_toolchain_contracts_remain_authoritative() {
        use crate::toolchain::errors::ToolchainError;
        use crate::toolchain::ids::{
            ProjectId,
            WorkspaceId,
        };
        use crate::toolchain::result::Result;

        let _: Option<ProjectId> = None;
        let _: Option<WorkspaceId> = None;
        let _: Option<ToolchainError> = None;
        let _: Option<Result<()>> = None;
    }
}
